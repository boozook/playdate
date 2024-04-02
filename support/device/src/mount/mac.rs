use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use futures::Future;
use futures::FutureExt;
use serde::Deserialize;
use crate::device::Device;
use crate::error::Error;


pub const VENDOR_ID_ENC: &str = const_hex::const_encode::<2, true>(&crate::VENDOR_ID.to_be_bytes()).as_str();


#[derive(Debug, Clone)]
pub struct Volume {
	path: PathBuf,
}

impl From<PathBuf> for Volume {
	fn from(path: PathBuf) -> Self { Self { path } }
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.path.display().fmt(f) }
}

impl Volume {
	/// This volume's path.
	pub fn path(&self) -> Cow<'_, Path> { self.path.as_path().into() }
}


mod unmount {
	use super::*;
	use crate::mount::Unmount;
	use crate::mount::UnmountAsync;


	impl Unmount for Volume {
		#[cfg_attr(feature = "tracing", tracing::instrument())]
		fn unmount_blocking(&self) -> Result<(), Error> {
			cmd(self).status()?
			         .exit_ok()
			         .map(|_| trace!("unmounted {self}"))
			         .map_err(Into::into)
		}
	}


	impl UnmountAsync for Volume {
		#[cfg_attr(feature = "tracing", tracing::instrument())]
		async fn unmount(&self) -> Result<(), Error> {
			#[cfg(all(feature = "tokio", not(feature = "async-std")))]
			use tokio::process::Command;
			#[cfg(feature = "async-std")]
			use async_std::process::Command;
			Command::from(cmd(self)).status()
			                        .await?
			                        .exit_ok()
			                        .map(|_| trace!("unmounted {self}"))
			                        .map_err(Into::into)
		}
	}

	fn cmd(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("diskutil");
		cmd.arg("eject");
		cmd.arg(vol.path().as_ref());
		cmd
	}
}


#[derive(Debug)]
pub struct SpusbInfo<Fut>
	where Fut: Future<Output = Result<PathBuf, Error>> {
	pub name: String,
	pub serial: String,
	pub volume: Fut,
}


#[cfg_attr(feature = "tracing", tracing::instrument(fields(dev = dev.as_ref().serial_number())))]
pub async fn volume_for<Info>(dev: Info) -> Result<Volume, Error>
	where Info: AsRef<nusb::DeviceInfo> {
	if let Some(sn) = dev.as_ref().serial_number() {
		let res = spusb(move |ref info| info.serial_num == sn).map(|mut iter| iter.next().map(|info| info.volume));
		match res {
			Ok(None) => Err(Error::not_found()),
			Ok(Some(fut)) => Ok(fut),
			Err(err) => Err(err),
		}
	} else {
		Err(Error::not_found())
	}?.await
	.map(Volume::from)
}

#[cfg_attr(feature = "tracing", tracing::instrument(skip(devs)))]
pub async fn volumes_for_map<I>(devs: I) -> Result<HashMap<Device, Option<Volume>>, Error>
	where I: IntoIterator<Item = Device> {
	let mut devs = devs.into_iter()
	                   .filter_map(|dev| {
		                   if let Some(sn) = dev.info().serial_number().map(ToOwned::to_owned) {
			                   Some((dev, sn))
		                   } else {
			                   None
		                   }
	                   })
	                   .collect::<Vec<_>>();

	let mut results = HashMap::with_capacity(devs.len());

	for info in spusb(|_| true)? {
		let i = devs.iter()
		            .enumerate()
		            .find(|(_, (_, sn))| &info.serial == sn)
		            .map(|(i, _)| i);

		if let Some(i) = i {
			match info.volume.await {
				Ok(vol) => {
					let (dev, _) = devs.remove(i);
					results.insert(dev, Some(Volume { path: vol }));
				},
				Err(err) => error!("{err}"),
			}
		}
	}

	results.extend(devs.into_iter().map(|(dev, _)| (dev, None)));

	Ok(results)
}

#[cfg_attr(feature = "tracing", tracing::instrument(skip(devs)))]
pub fn volumes_for<'i, I: 'i>(
	devs: I)
	-> Result<impl Iterator<Item = (impl Future<Output = Result<PathBuf, Error>>, &'i Device)>, Error>
	where I: IntoIterator<Item = &'i Device> {
	let devs = devs.into_iter()
	               .filter_map(|dev| dev.info().serial_number().map(|sn| (dev, sn)))
	               .collect::<Vec<_>>();

	spusb(|_| true).map(move |iter| {
		               iter.filter_map(move |info| {
			                   devs.iter()
			                       .find(|(_, sn)| info.serial == *sn)
			                       .map(|(dev, _)| (info.volume, *dev))
		                   })
	               })
}


/// Call `system_profiler -json SPUSBDataType`
#[cfg_attr(feature = "tracing", tracing::instrument(skip(filter)))]
fn spusb<F>(filter: F)
            -> Result<impl Iterator<Item = SpusbInfo<impl Future<Output = Result<PathBuf, Error>>>>, Error>
	where F: FnMut(&DeviceInfo) -> bool {
	use std::process::Command;

	let output = Command::new("system_profiler").args(["-json", "SPUSBDataType"])
	                                            .output()?;
	output.status.exit_ok()?;

	let data: SystemProfilerResponse = serde_json::from_reader(&output.stdout[..])?;

	let result = data.data
	                 .into_iter()
	                 .filter_map(|c| c.items)
	                 .flatten()
	                 .filter(|item| item.vendor_id == VENDOR_ID_ENC)
	                 .filter(filter)
	                 .filter_map(|item| {
		                 let DeviceInfo { name,
		                                  serial_num: serial,
		                                  media,
		                                  .. } = item;
		                 let volume = media.map(|media| {
			                                   media.into_iter()
			                                        .flat_map(|root| root.volumes.into_iter())
			                                        .filter_map(|par| {
				                                        if let Some(path) = par.mount_point {
					                                        trace!("found mount-point: {}", path.display());
					                                        Some(futures_lite::future::ready(Ok(path)).left_future())
				                                        } else {
					                                        let path = Path::new("/Volumes").join(&par.name);
					                                        if path.exists() {
						                                        trace!("existing, by name: {}", path.display());
						                                        Some(futures_lite::future::ready(Ok(path)).left_future())
					                                        } else if par.volume_uuid.is_some() {
						                                        trace!("not mounted yet, create resolver fut");
						                                        Some(mount_point_for_partition(par).right_future())
					                                        } else {
						                                        None
					                                        }
				                                        }
			                                        })
			                                        .next()
		                                   })
		                                   .flatten();
		                 volume.map(|volume| SpusbInfo { name, serial, volume })
	                 });
	Ok(result)
}


/// Calls `diskutil info -plist {partition.volume_uuid}`
#[cfg_attr(feature = "tracing", tracing::instrument(skip(par), fields(par.name = par.name.as_str())))]
async fn mount_point_for_partition(par: MediaPartitionInfo) -> Result<PathBuf, Error> {
	use std::process::Command;

	if let Some(volume_uuid) = par.volume_uuid.as_deref() {
		let output = Command::new("diskutil").args(["info", "-plist"])
		                                     .arg(volume_uuid)
		                                     .output()?;
		output.status.exit_ok()?;

		let info: DiskUtilResponse = plist::from_bytes(output.stdout.as_slice())?;
		info.mount_point
		    .ok_or(Error::MountNotFound(format!("{} {}", &par.name, &par.bsd_name)))
		    .map(PathBuf::from)
	} else {
		Err(Error::MountNotFound(format!("{} {}", &par.name, &par.bsd_name)))
	}
}


#[derive(Deserialize, Debug)]
struct DiskUtilResponse {
	#[serde(rename = "MountPoint")]
	mount_point: Option<String>,
}


#[derive(Deserialize, Debug)]
struct SystemProfilerResponse {
	#[serde(rename = "SPUSBDataType")]
	data: Vec<ControllerInfo>,
}


#[derive(Deserialize, Debug)]
struct ControllerInfo {
	#[serde(rename = "_items")]
	items: Option<Vec<DeviceInfo>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DeviceInfo {
	#[serde(rename = "_name")]
	pub name: String,
	pub serial_num: String,
	pub vendor_id: String,

	#[serde(rename = "Media")]
	pub media: Option<Vec<DeviceMediaInfo>>,
}


#[derive(Deserialize, Debug)]
pub struct DeviceMediaInfo {
	volumes: Vec<MediaPartitionInfo>,
}

#[derive(Deserialize, Debug)]
pub struct MediaPartitionInfo {
	#[serde(rename = "_name")]
	name: String,
	bsd_name: String,
	volume_uuid: Option<String>,
	mount_point: Option<PathBuf>,
}
