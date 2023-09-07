#![cfg(target_os = "macos")]
use std::process::Command;
use std::path::Path;
use std::path::PathBuf;
use serde::Deserialize;

use crate::Error;
use crate::model::Mode;
use crate::model::Device;


/// Get a list of all available devices by OS
pub fn list_usb_devices() -> Result<Vec<Device>, Error> {
	let mut devices = system_profiler_spusbdata().map_err(|err| debug!("{err}"))
	                                             .unwrap_or_default();
	let mut tty = tty::find_cu_devices()?.collect::<Vec<_>>();

	// merge:
	tty.drain(..).for_each(|tty| {
		             if let Some(device) = devices.iter_mut().find(|d| d.serial == tty.serial) {
			             device.refresh_from(tty);
		             } else {
			             devices.push(tty);
		             }
	             });

	Ok(devices)
}


impl Device {
	pub fn refresh(&mut self) -> Result<(), Error> {
		let devices = system_profiler_spusbdata()?;
		let device = devices.into_iter()
		                    .filter(|d| d.serial == self.serial)
		                    .next()
		                    .ok_or("No device found")?;
		self.refresh_from(device);
		self.refresh_tty()?;
		Ok(())
	}

	pub fn refresh_tty(&mut self) -> Result<(), Error> {
		self.tty = tty::find_cu_for(&self.serial)?;
		Ok(())
	}
}


/// Get a list of all available devices by OS
/// and update the `devices`.
pub fn refresh(devices: &mut [Device]) -> Result<(), Error> {
	list_usb_devices()?.into_iter().for_each(|a| {
		                               devices.iter_mut()
		                                      .find(|b| b.serial == a.serial)
		                                      .map(|b| b.refresh_from(a));
	                               });
	Ok(())
}


mod tty {
	use std::path::PathBuf;
	use crate::Error;
	use crate::model::Device;
	use crate::model::Mode;
	use crate::model::SerialNumber;


	const DIR: &str = "/dev";

	/// Search for a cu fd that looks like to Playdate.
	pub fn find_cu_devices() -> Result<impl Iterator<Item = Device>, Error> {
		let devices = std::fs::read_dir(DIR)?.filter_map(move |entry| {
			                                     let entry = entry.ok()?;
			                                     let name = entry.file_name();
			                                     let name = name.to_string_lossy();
			                                     let serial: SerialNumber = name.parse().ok()?;
			                                     if name.starts_with("cu.usbmodem") {
				                                     let cu = entry.path();
				                                     Some(Device { serial,
				                                                   mode: Mode::Data,
				                                                   tty: Some(cu),
				                                                   volume: None })
			                                     } else {
				                                     None
			                                     }
		                                     });
		Ok(devices)
	}


	pub fn find_cu_for(serial: &SerialNumber) -> Result<Option<PathBuf>, Error> {
		Ok(std::fs::read_dir(DIR)?.filter_map(move |entry| {
			                          let entry = entry.ok()?;
			                          let name = entry.file_name();
			                          let name = name.to_string_lossy();
			                          if name.starts_with("cu.usbmodem") &&
			                             &SerialNumber::try_from(name.as_ref()).ok()? == serial
			                          {
				                          Some(entry.path())
			                          } else {
				                          None
			                          }
		                          })
		                          .next())
	}
}


/// Call `system_profiler -json SPUSBDataType`
fn system_profiler_spusbdata() -> Result<Vec<Device>, Error> {
	let output = Command::new("system_profiler").args(["-json", "SPUSBDataType"])
	                                            .output()?;
	output.status.exit_ok()?;

	let data: SystemProfilerResponse = serde_json::from_reader(&output.stdout[..])?;

	let result = data.data
	                 .into_iter()
	                 .filter_map(|c| c.items)
	                 .flatten()
	                 .filter(|item| item.vendor_id == "0x1331")
	                 .filter_map(|item| {
		                 let name = item.name.to_owned();
		                 let serial = item.serial_num.to_owned();
		                 device_info_to_device(item).map_err(|err| {
			                                            debug!("{} {} {:?}", name, serial, err);
		                                            })
		                                            .ok()
	                 })
	                 .collect::<Vec<_>>();


	Ok(result)
}

fn device_info_to_device(info: DeviceInfo) -> Result<Device, Error> {
	let serial = info.serial_num.parse()?;
	let mode = if info.media.is_some() {
		Mode::Storage
	} else {
		Mode::Data
	};
	let volume = info.media
	                 .map(|media| {
		                 media.into_iter()
		                      .flat_map(|root| root.volumes.into_iter())
		                      .filter_map(|par| {
			                      if let Some(path) = par.mount_point {
				                      Some(path)
			                      } else {
				                      let path = Path::new("/Volumes").join(&par.name);
				                      if path.try_exists().ok()? {
					                      Some(path)
				                      } else {
					                      mount_point_for_partition(&par).map_err(|err| {
						                                                     debug!("{err:?}");
					                                                     })
					                                                     .ok()
				                      }
			                      }
		                      })
		                      .next()
	                 })
	                 .flatten();

	Ok(Device { serial,
	            mode,
	            volume,
	            tty: None })
}

/// Calls `diskutil info -plist {partition.volume_uuid}`
fn mount_point_for_partition(par: &MediaPartitionInfo) -> Result<PathBuf, Error> {
	let output = Command::new("diskutil").args(["info", "-plist"])
	                                     .arg(&par.volume_uuid)
	                                     .output()?;
	output.status.exit_ok()?;
	let info: DiskUtilResponse = plist::from_bytes(output.stdout.as_slice())?;
	info.mount_point
	    .ok_or(format!(
		"Mount point not found for partition {} {}",
		&par.name, &par.bsd_name
	).into())
	    .map(PathBuf::from)
}


#[derive(Deserialize, Debug)]
struct DiskUtilResponse {
	// #[serde(rename = "Ejectable")]
	// ejectable: bool,
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
struct DeviceInfo {
	#[serde(rename = "_name")]
	name: String,
	manufacturer: String,
	product_id: String,
	serial_num: String,
	vendor_id: String,

	#[serde(rename = "Media")]
	media: Option<Vec<DeviceMediaInfo>>,
}


#[derive(Deserialize, Debug)]
struct DeviceMediaInfo {
	volumes: Vec<MediaPartitionInfo>,
}

#[derive(Deserialize, Debug)]
struct MediaPartitionInfo {
	#[serde(rename = "_name")]
	name: String,
	bsd_name: String,
	volume_uuid: String,
	mount_point: Option<PathBuf>,
}
