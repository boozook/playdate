// #![cfg(target_os = "linux")]

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::future::Future;
use std::future::IntoFuture;

use futures::FutureExt;
use udev::Enumerator;

use crate::device::serial::SerialNumber;
use crate::error::Error;
use crate::device::Device;


#[derive(Debug, Clone)]
pub struct Volume {
	/// FS mount point.
	path: PathBuf,

	/// Partition node path, e.g.: `/dev/sda1`.
	part_node: PathBuf,

	/// Disk node path, e.g.: `/dev/sda`.
	disk_node: PathBuf,

	/// Device sysfs path.
	dev_sysfs: PathBuf,
}

impl Volume {
	fn new(path: PathBuf, part: PathBuf, disk: PathBuf, dev_sysfs: PathBuf) -> Self {
		Self { path,
		       part_node: part,
		       disk_node: disk,
		       dev_sysfs }
	}
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.path.display().fmt(f) }
}

impl Volume {
	/// This volume's path.
	pub fn path(&self) -> Cow<'_, Path> { self.path.as_path().into() }
}


mod unmount {
	use futures::TryFutureExt;

	use super::*;
	use crate::mount::Unmount;
	use crate::mount::UnmountAsync;


	impl Unmount for Volume {
		#[cfg_attr(feature = "tracing", tracing::instrument())]
		fn unmount_blocking(&self) -> Result<(), Error> {
			use std::process::Command;


			let res = eject(self).status()
			                     .map_err(Error::from)
			                     .and_then(|res| res.exit_ok().map_err(Error::from))
			                     .or_else(|err| -> Result<(), Error> {
				                     unmount(self).status()
				                                  .map_err(Error::from)
				                                  .and_then(|res| res.exit_ok().map_err(Error::from))
				                                  .map_err(|err2| Error::chain(err, [err2]))
			                     })
			                     .or_else(move |err| -> Result<(), Error> {
				                     udisksctl_unmount(self).status()
				                                            .map_err(Error::from)
				                                            .and_then(|res| res.exit_ok().map_err(Error::from))
				                                            .map_err(|err2| Error::chain(err, [err2]))
			                     })
			                     .or_else(move |err| -> Result<(), Error> {
				                     udisks_unmount(self).status()
				                                         .map_err(Error::from)
				                                         .and_then(|res| res.exit_ok().map_err(Error::from))
				                                         .map_err(|err2| Error::chain(err, [err2]))
			                     })
			                     .inspect(|_| trace!("unmounted {self}"));

			// TODO: use `udisks_power_off` also as fallback for `udisksctl_power_off`:
			Command::from(udisksctl_power_off(self)).status()
			                                        .map_err(Error::from)
			                                        .and_then(|res| res.exit_ok().map_err(Error::from))
			                                        .map_err(move |err2| {
				                                        if let Some(err) = res.err() {
					                                        Error::chain(err, [err2])
				                                        } else {
					                                        err2
				                                        }
			                                        })
		}
	}

	#[cfg(feature = "tokio")]
	impl UnmountAsync for Volume {
		#[cfg_attr(feature = "tracing", tracing::instrument())]
		async fn unmount(&self) -> Result<(), Error> {
			use tokio::process::Command;
			use futures_lite::future::ready;


			Command::from(eject(self)).status()
			                          .map_err(Error::from)
			                          .and_then(|res| ready(res.exit_ok().map_err(Error::from)))
			                          .or_else(|err| {
				                          Command::from(unmount(self)).status()
				                                                      .map_err(|err2| Error::chain(err, [err2]))
				                                                      .and_then(|res| {
					                                                      ready(res.exit_ok().map_err(Error::from))
				                                                      })
			                          })
			                          .or_else(|err| {
				                          Command::from(udisksctl_unmount(self)).status()
				                                                                .map_err(|err2| {
					                                                                Error::chain(err, [err2])
				                                                                })
				                                                                .and_then(|res| {
					                                                                ready(
					                                                                      res.exit_ok()
					                                                                         .map_err(Error::from),
					)
				                                                                })
			                          })
			                          .or_else(|err| {
				                          Command::from(udisks_unmount(self)).status()
				                                                             .map_err(|err2| {
					                                                             Error::chain(err, [err2])
				                                                             })
				                                                             .and_then(|res| {
					                                                             ready(
					                                                                   res.exit_ok()
					                                                                      .map_err(Error::from),
					)
				                                                             })
			                          })
			                          .inspect_ok(|_| trace!("unmounted {self}"))
			                          .then(|res| {
				                          // TODO: use `udisks_power_off` also as fallback for `udisksctl_power_off`:
				                          Command::from(udisksctl_power_off(self)).status()
				                                                                  .map_err(Error::from)
				                                                                  .and_then(|res| {
					                                                                  ready(
					                                                                        res.exit_ok()
					                                                                           .map_err(Error::from),
					)
				                                                                  })
				                                                                  .map_err(|err2| {
					                                                                  if let Some(err) = res.err() {
						                                                                  Error::chain(err, [err2])
					                                                                  } else {
						                                                                  err2
					                                                                  }
				                                                                  })
			                          })
			                          .await
		}
	}


	fn eject(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("eject");
		cmd.arg(vol.path().as_ref());
		cmd
	}

	fn unmount(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("umount");
		cmd.arg(vol.path().as_ref());
		cmd
	}

	fn udisksctl_unmount(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("udisksctl");
		cmd.args(["unmount", "--no-user-interaction", "-b"]);
		cmd.arg(&vol.part_node);
		cmd
	}

	fn udisksctl_power_off(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("udisksctl");
		cmd.args(["power-off", "--no-user-interaction", "-b"]);
		cmd.arg(&vol.disk_node);
		cmd
	}

	fn udisks_unmount(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("udisks");
		cmd.arg("--unmount");
		cmd.arg(&vol.part_node);
		cmd
	}

	fn udisks_power_off(vol: &Volume) -> std::process::Command {
		let mut cmd = std::process::Command::new("udisks");
		cmd.arg("--detach");
		cmd.arg(&vol.disk_node);
		cmd
	}

	// NOTE: mb. try to use `udisks`, that's existing in Ubuntu.
	// udisksctl unmount -b /dev/sdc1 && udisksctl power-off -b /dev/sdc
	// udisks --unmount /dev/sdb1 && udisks --detach /dev/sdb
}


#[cfg_attr(feature = "tracing", tracing::instrument(fields(dev = dev.as_ref().serial_number())))]
pub async fn volume_for<Info>(dev: Info) -> Result<Volume, Error>
	where Info: AsRef<nusb::DeviceInfo> {
	let sysfs = dev.as_ref().sysfs_path();
	let mut enumerator = enumerator()?;
	enumerator.add_syspath(sysfs)?;

	if let Some(sn) = dev.as_ref().serial_number() {
		enumerator.match_property("ID_SERIAL_SHORT", sn)?;
	}

	let mounts = lfs_core::read_mountinfo()?;
	enumerator.scan_devices()?
	          .filter_map(|udev| {
		          udev.devtype()
		              .filter(|ty| *ty == OsStr::new("partition"))
		              .is_some()
		              .then(move || udev.devnode().map(Path::to_path_buf).map(|node| (udev, node)))
	          })
	          .flatten()
	          .find_map(|(udev, node)| {
		          mounts.iter()
		                .find(|inf| Path::new(inf.fs.as_str()) == node.as_path())
		                .map(|inf| (udev, node, inf))
	          })
	          .and_then(|(udev, node, minf)| {
		          let disk = udev.parent()
		                         .filter(is_disk)
		                         .or_else(|| udev.parent().map(|d| d.parent().filter(is_disk)).flatten())
		                         .and_then(|dev| dev.devnode().map(ToOwned::to_owned));
		          let sysfs = PathBuf::from(sysfs);
		          disk.map(move |disk| Volume::new(minf.mount_point.clone(), node, disk, sysfs))
	          })
	          .ok_or_else(|| Error::not_found())
}


#[cfg_attr(feature = "tracing", tracing::instrument(skip(devs)))]
pub async fn volumes_for_map<I>(devs: I) -> Result<HashMap<Device, Option<Volume>>, Error>
	where I: IntoIterator<Item = Device> {
	let mounts = lfs_core::read_mountinfo()?;

	if mounts.is_empty() {
		return Ok(devs.into_iter().map(|dev| (dev, None)).collect());
	}

	let mut enumerator = enumerator()?;

	let udevs: Vec<_> = enumerator.scan_devices()?
	                              .filter(is_partition)
	                              .filter_map(|dev| {
		                              if let Some(sn) = dev.property_value("ID_SERIAL_SHORT") {
			                              let sn = sn.to_string_lossy().to_string();
			                              Some((dev, sn))
		                              } else {
			                              if let Some(sn) = dev.property_value("ID_SERIAL") {
				                              let sn: Result<SerialNumber, _> =
					                              sn.to_string_lossy().as_ref().try_into();
				                              sn.ok().map(|sn| (dev, sn.to_string()))
			                              } else {
				                              None
			                              }
		                              }
	                              })
	                              .collect();

	if udevs.is_empty() {
		return Ok(devs.into_iter().map(|dev| (dev, None)).collect());
	}

	let mut devs = devs.into_iter().filter_map(|dev| {
		                               if let Some(sn) = dev.info().serial_number().map(ToOwned::to_owned) {
			                               Some((dev, sn))
		                               } else {
			                               None
		                               }
	                               });

	let result =
		devs.map(|(dev, ref sna)| {
			    let node =
				    udevs.iter()
				         .find_map(|(inf, snb)| {
					         (sna == snb).then(|| inf.devnode())
					                     .flatten()
					                     .map(ToOwned::to_owned)
					                     .map(|dn| (inf, dn))
				         })
				         .and_then(|(udev, node)| {
					         mounts.iter()
					               .find(|inf| Path::new(inf.fs.as_str()) == node)
					               .and_then(|inf| {
						               let disk = udev.parent()
						                              .filter(is_disk)
						                              .or_else(|| udev.parent().map(|d| d.parent().filter(is_disk)).flatten())
						                              .and_then(|dev| dev.devnode().map(ToOwned::to_owned));

						               let sysfs = dev.info().sysfs_path().to_owned();
						               disk.map(move |disk| Volume::new(inf.mount_point.clone(), node, disk, sysfs))
					               })
				         });
			    (dev, node)
		    })
		    .collect();
	Ok(result)
}


// TODO: this is needed too:
// pub fn volumes_for<'i, I: 'i>(
// 	devs: I)
// 	-> Result<impl Iterator<Item = (impl Future<Output = Result<PathBuf, Error>>, &'i Device)>, Error>
// 	where I: IntoIterator<Item = &'i Device> {
// 	//
// 	Ok(vec![(futures::future::lazy(|_| todo!()).into_future(), &todo!())].into_iter())
// }


fn enumerator() -> Result<Enumerator, Error> {
	let mut enumerator = udev::Enumerator::new()?;
	// filter only PD devices:
	enumerator.match_property("ID_VENDOR", "Panic")?;
	enumerator.match_property("ID_MODEL", "Playdate")?;
	Ok(enumerator)
}


fn is_partition(dev: &udev::Device) -> bool {
	dev.devtype()
	   .filter(|ty| *ty == OsStr::new("partition"))
	   .is_some()
}

fn is_disk(dev: &udev::Device) -> bool { dev.devtype().filter(|ty| *ty == OsStr::new("disk")).is_some() }
