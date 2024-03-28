use std::future::Future;

use futures::stream::FuturesUnordered as Unordered;
use futures::{FutureExt, Stream, StreamExt, TryFutureExt};

use crate::device::query::Query as Query;
use crate::device::query::Value as QueryValue;
use crate::device::serial::SerialNumber as Sn;
use crate::device::{wait_mode_storage, wait_mode_data, Device};
use crate::error::Error;
use crate::interface::r#async::Out;
use crate::mount::{MountAsync, MountHandle};
use crate::mount::MountedDevice;
use crate::mount::volume::volumes_for_map;
use crate::usb::discover::devices_storage;
use crate::usb;
use crate::serial::{self, dev_with_port};
use crate::interface;


type Result<T = (), E = Error> = std::result::Result<T, E>;

// TODO: make timeout configurable
#[cfg_attr(feature = "tracing", tracing::instrument(fields(dev = mount.info().serial_number(), mount = mount.handle.volume().path().as_ref().display().to_string())))]
pub async fn wait_fs_available(mount: &MountedDevice) -> Result {
	const ITER: u64 = 120; // ms
	const RETRIES: u8 = 255; // ≈30 sec
	let mut counter = RETRIES;
	let every = std::time::Duration::from_millis(ITER);
	let mut interval = tokio::time::interval(every);

	let check = || {
		mount.handle
		     .path()
		     .try_exists()
		     .inspect_err(|err| debug!("{err}"))
		     .ok()
		     .unwrap_or_default()
		     .then(|| {
			     let path = mount.handle.path();
			     match std::fs::read_dir(path).inspect_err(|err| debug!("{err}")) {
				     // then find first dir entry:
			        Ok(entries) => entries.into_iter().flatten().next().is_some(),
			        _ => false,
			     }
		     })
		     .unwrap_or_default()
	};

	if check() {
		trace!("filesystem available at {}", mount.handle.path().display());
		return Ok(());
	}

	let sn = mount.info()
	              .serial_number()
	              .ok_or_else(|| Error::DeviceSerial { source: "unknown".into() })?
	              .to_owned();

	while {
		counter -= 1;
		counter
	} != 0
	{
		interval.tick().await;
		trace!("try: {}/{RETRIES}", RETRIES - counter);

		if check() {
			return Ok(());
		} else {
			trace!("waiting filesystem availability: {sn}");
		}
	}

	Err(Error::timeout(format!(
		"{sn}: filesystem not found at {} after {RETRIES} retries",
		mount.handle.path().display()
	)))
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn mount(query: Query) -> Result<impl Stream<Item = Result<MountedDevice>>> {
	match query.value {
		Some(QueryValue::Path(port)) => {
			let fut = mount_by_port_name(port.display().to_string()).await?
			                                                        .left_stream();
			Ok(fut)
		},
		Some(QueryValue::Com(port)) => {
			let fut = mount_by_port_name(format!("COM{port}")).await?.left_stream();
			Ok(fut)
		},
		Some(QueryValue::Serial(sn)) => Ok(mount_by_sn_mb(Some(sn)).await?.right_stream()),
		_ => Ok(mount_by_sn_mb(None).await?.right_stream()),
	}
}


/// Switch between stream methods `mount` and `mount then wait_fs_available`,
/// depending on `wait` parameter.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn mount_and(query: Query, wait: bool) -> Result<impl Stream<Item = Result<MountedDevice>>> {
	let fut = mount(query).await?.flat_map(move |res| {
		                             async move {
			                             match res {
				                             Ok(drive) => {
				                                if wait {
					                                wait_fs_available(&drive).await?
				                                }
				                                Ok(drive)
			                                },
			                                Err(err) => Err(err),
			                             }
		                             }.into_stream()
	                             });
	Ok(fut)
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn mount_by_sn_mb(sn: Option<Sn>) -> Result<Unordered<impl Future<Output = Result<MountedDevice>>>> {
	let devices = usb::discover::devices_with(sn)?;
	let mounting = devices.map(mount_dev);

	let futures = Unordered::new();
	for dev in mounting {
		futures.push(dev?);
	}

	if futures.is_empty() {
		Err(Error::not_found())
	} else {
		Ok(futures)
	}
}


#[cfg_attr(feature = "tracing", tracing::instrument(fields(port = port.as_ref())))]
pub async fn mount_by_port_name<S: AsRef<str>>(
	port: S)
	-> Result<Unordered<impl Future<Output = Result<MountedDevice>>>> {
	let port = port.as_ref();
	let existing = serial::discover::ports().map(|ports| {
		                                        ports.into_iter()
		                                             .find(|p| p.port_name == port)
		                                             .map(serial::Interface::new)
	                                        });

	let futures = Unordered::new();

	let err_not_found = || futures_lite::future::ready(Err(Error::not_found()));

	match existing {
		Ok(Some(port)) => {
			if let serialport::SerialPortType::UsbPort(serialport::UsbPortInfo { serial_number: Some(ref sn),
			                                                                     .. }) = port.info().port_type
			{
				let dev = Sn::try_from(sn.as_str()).map_err(Error::from)
				                                   .and_then(|sn| usb::discover::device(&sn));
				match dev {
					Ok(mut dev) => {
						dev.set_interface(interface::Interface::Serial(port));
						futures.push(mount_dev(dev)?.left_future());
					},
					Err(err) => {
						let name = port.info().port_name.as_str();
						error!("Unable to map specified port {name} to device: {err}");
						port.mount().await?;
						futures.push(err_not_found().right_future());
					},
				}
			}
		},
		Ok(None) => {
			match dev_with_port(port).await {
				Ok(dev) => futures.push(mount_dev(dev)?.left_future()),
				Err(err) => {
					let name = port;
					error!("Unable to map specified port {name} to device: {err}");
					let port = serial::open(name)?;
					let interface = serial::Interface::new_with(port, Some(name.to_string()));
					interface.send_cmd(crate::device::command::Command::Datadisk)
					         .await?;
					futures.push(err_not_found().right_future());
				},
			}
		},
		Err(err) => {
			error!("{err}");
			match dev_with_port(port).await {
				Ok(dev) => futures.push(mount_dev(dev)?.left_future()),
				Err(err) => {
					let name = port;
					error!("Unable to map specified port {name} to device: {err}");
					let port = serial::open(name)?;
					let interface = serial::Interface::new_with(port, Some(name.to_string()));
					interface.send_cmd(crate::device::command::Command::Datadisk)
					         .await?;
					futures.push(err_not_found().right_future());
				},
			}
		},
	}

	if futures.is_empty() {
		Err(Error::not_found())
	} else {
		Ok(futures)
	}
}


#[cfg_attr(feature = "tracing", tracing::instrument(fields(dev = dev.info().serial_number())))]
fn mount_dev(mut dev: Device) -> Result<impl Future<Output = Result<MountedDevice>>> {
	dev.info().serial_number().map(|s| debug!("mounting {s}"));
	let fut = match dev.mode_cached() {
		usb::mode::Mode::Data => {
			trace!("create sending fut");
			async move {
				use interface::r#async::Out;
				dev.open()?;
				dev.interface()?
				   .send_cmd(crate::device::command::Command::Datadisk)
				   .await?;
				dev.close();
				Ok(dev)
			}.and_then(wait_mode_storage)
			.left_future()
		},
		usb::mode::Mode::Storage => futures_lite::future::ready(Ok(dev)).right_future(),
		mode => return Err(Error::WrongState(mode)),
	};
	Ok(fut.and_then(wait_mount_point))
}


// TODO: make timeout configurable
#[cfg_attr(feature = "tracing", tracing::instrument(fields(dev = dev.info().serial_number())))]
async fn wait_mount_point(dev: Device) -> Result<MountedDevice> {
	const ITER: u64 = 100; // ms
	const RETRIES: u8 = 100; // ≈10 sec
	let mut counter = RETRIES;
	let every = std::time::Duration::from_millis(ITER);
	let mut interval = tokio::time::interval(every);


	let sn = dev.info()
	            .serial_number()
	            .ok_or_else(|| Error::DeviceSerial { source: "unknown".into() })?
	            .to_owned();

	while {
		counter -= 1;
		counter
	} != 0
	{
		interval.tick().await;
		trace!("try: {}/{RETRIES}", RETRIES - counter);

		let mode = dev.mode_cached();
		trace!("waiting mount point availability: {sn}, current: {mode}");

		let vol = crate::mount::volume::volume_for(&dev).await
		                                                .map_err(|err| debug!("ERROR: {err}"))
		                                                .ok();
		if let Some(vol) = vol {
			debug!("{sn} mounted, volume found: '{vol}'");
			let handle = MountHandle::new(vol, false);
			let mounted = MountedDevice::new(dev, handle);
			return Ok(mounted);
		} else {
			debug!("mount point still not found, waiting...")
		}
	}

	Err(Error::usb_timeout(dev))
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn unmount(query: Query) -> Result<Unordered<impl Future<Output = (Device, Result)>>> {
	match query.value {
		Some(QueryValue::Path(path)) => {
			// check is `path` is a a path of existing __volume__,
			// try find device behind the volume,
			// unmount the volume anyway
			todo!("unmount dev by vol path: {}", path.display())
		},
		Some(QueryValue::Com(_)) => todo!("ERROR: not supported (impossible)"),
		Some(QueryValue::Serial(sn)) => unmount_mb_sn(Some(sn)),
		_ => unmount_mb_sn(None),
	}.await
}

/// Unmount device(s), then wait for state change to [`Data`][usb::mode::Mode::Data].
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn unmount_and_wait(query: Query) -> Result<impl Stream<Item = Result<Device>>> {
	let stream = Unordered::new();
	unmount(query).await?
	              .for_each_concurrent(4, |(dev, res)| {
		              if let Some(err) = res.err() {
			              let sn = dev.info().serial_number().unwrap_or("unknown");
			              error!("{sn}: {err}")
		              }
		              stream.push(wait_mode_data(dev));
		              futures_lite::future::ready(())
	              })
	              .await;

	trace!("Waiting state change for {} devices.", stream.len());
	Ok(stream)
}

/// Switch between stream methods `unmount` and `unmount_and_wait`,
/// depending on `wait` parameter.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn unmount_and(query: Query, wait: bool) -> Result<impl Stream<Item = Result<Device>>> {
	let results = if wait {
		unmount_and_wait(query).await?.left_stream()
	} else {
		unmount(query).await?
		              .map(|(dev, res)| res.map(|_| dev))
		              .right_stream()
	};

	Ok(results)
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn unmount_mb_sn(sn: Option<Sn>) -> Result<Unordered<impl Future<Output = (Device, Result)>>> {
	let devs = devices_storage()?.filter(move |dev| {
		                             sn.as_ref()
		                               .filter(|qsn| dev.info().serial_number().filter(|ref s| qsn.eq(s)).is_some())
		                               .is_some() ||
		                             sn.is_none()
	                             })
	                             .inspect(|dev| {
		                             dev.info().serial_number().map(|sn| trace!("Unmounting {sn}"));
	                             });

	let unmounting = volumes_for_map(devs).await?
	                                      .into_iter()
	                                      .filter_map(|(dev, vol)| vol.map(|vol| (dev, vol)))
	                                      .inspect(|(dev, vol)| {
		                                      dev.info()
		                                         .serial_number()
		                                         .map(|sn| trace!("Unmounting {sn}: {vol}"));
	                                      })
	                                      .map(|(dev, vol)| {
		                                      let h = MountHandle::new(vol, false);
		                                      MountedDevice::new(dev, h)
	                                      })
	                                      .map(move |mut dev| {
		                                      use crate::mount::UnmountAsync;
		                                      async move {
			                                      dev.device.close();
			                                      let res = dev.unmount().await;
			                                      (dev.device, res)
		                                      }
	                                      })
	                                      .collect::<Unordered<_>>();

	trace!("Unmounting {} devices.", unmounting.len());
	Ok(unmounting)
}
