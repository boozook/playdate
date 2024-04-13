#[cfg(feature = "futures")]
use futures::{Stream, StreamExt};

use crate::device::query;
use crate::error::Error;
use crate::device::serial::SerialNumber as Sn;
use crate::{usb, serial, interface};
use crate::PRODUCT_ID_DATA;
use crate::PRODUCT_ID_STORAGE;
use crate::VENDOR_ID;

use super::Device;

type Result<T, E = Error> = std::result::Result<T, E>;


/// Enumerate all Playdate- devices.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn devices() -> Result<impl Iterator<Item = Device>> {
	Ok(nusb::list_devices()?.filter(|d| d.vendor_id() == VENDOR_ID)
	                        .map(Device::new))
}

/// Search Playdate- devices that in data (serial/modem/telnet) mode.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn devices_data() -> Result<impl Iterator<Item = Device>> {
	devices().map(|iter| iter.filter(|d| d.info.product_id() == PRODUCT_ID_DATA))
}

/// Search Playdate- devices that in storage (data-disk) mode.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn devices_storage() -> Result<impl Iterator<Item = Device>> {
	devices().map(|iter| iter.filter(|d| d.info.product_id() == PRODUCT_ID_STORAGE))
}

/// Search exact one device with same serial number.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn device(sn: &Sn) -> Result<Device> {
	devices()?.find(|d| d.info.serial_number().filter(|s| sn.eq(s)).is_some())
	          .ok_or_else(|| Error::not_found())
}

/// Search devices with same serial number,
/// or __any__ Playdate- device if `sn` is `None`.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn devices_with(sn: Option<Sn>) -> Result<impl Iterator<Item = Device>> {
	Ok(devices()?.filter(move |dev| {
		             if let Some(sn) = sn.as_ref() {
			             dev.info().serial_number().filter(|s| sn.eq(s)).is_some()
		             } else {
			             true
		             }
	             }))
}

/// Search devices with same serial number in data mode,
/// or __any__ Playdate- device if `sn` is `None`.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn devices_data_with(sn: Option<Sn>) -> Result<impl Iterator<Item = Device>> {
	Ok(devices_data()?.filter(move |dev| {
		                  if let Some(sn) = sn.as_ref() {
			                  dev.info().serial_number().filter(|s| sn.eq(s)).is_some()
		                  } else {
			                  true
		                  }
	                  }))
}


#[cfg(feature = "futures")]
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn devices_data_for(query: query::Query) -> Result<Vec<Device>> {
	use query::Value as Query;
	use serial::dev_with_port;


	let try_by_port = |port_pref: String| {
		async {
			let existing = serial::discover::ports().map(|ports| {
				                                        ports.into_iter()
				                                             .find(|p| p.port_name == port_pref)
				                                             .map(serial::Interface::new)
			                                        });
			match existing {
				Ok(Some(port)) => {
					if let serialport::SerialPortType::UsbPort(serialport::UsbPortInfo { serial_number:
					                                                                        Some(ref sn),
					                                                                     .. }) = port.info().port_type
					{
						let name = port.info().port_name.as_str().to_owned();
						Sn::try_from(sn.as_str()).map_err(Error::from)
						                         .and_then(|sn| usb::discover::devices_data_with(Some(sn)))
						                         .map(|mut devs| devs.next())
						                         .map(move |mb| {
							                         mb.map(|mut dev| {
								                           dev.set_interface(interface::Interface::Serial(port));
								                           dev
							                           })
						                         })
						                         .map_err(|err| {
							                         error!("Unable to map specified port {name} to device: {err}");
							                         Error::chain(Error::not_found(), [err])
						                         })
						                         .ok()
						                         .flatten()
						                         .ok_or_else(Error::not_found)
					} else {
						dev_with_port(port_pref).await
					}
				},
				Ok(None) => dev_with_port(port_pref).await,
				Err(err) => {
					dev_with_port(port_pref).await
					                        .map_err(|err2| Error::chain(err2, [err]))
				},
			}
		}
	};


	let devs = match query.value {
		Some(Query::Path(port)) => {
			vec![try_by_port(port.to_string_lossy().to_string()).await?]
		},
		Some(Query::Com(port)) => vec![try_by_port(format!("COM{port}")).await?],
		Some(Query::Serial(sn)) => devices_data_with(Some(sn)).map(|i| i.collect())?,
		None => devices_data_with(None).map(|i| i.collect())?,
	};

	Ok(devs)
}


#[cfg(feature = "futures")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip(f)))]
pub async fn for_each_data_interface<F, Fut, T>(query: query::Query, mut f: F) -> Result<impl Stream<Item = T>>
	where Fut: std::future::Future<Output = T>,
	      F: FnMut(interface::Interface) -> Fut {
	use query::Value as Query;
	use serial::unknown_serial_port_info;


	let devs = match query.value {
		Some(Query::Path(port)) => {
			let name = port.to_string_lossy();
			let mut interface = serial::Interface::new(unknown_serial_port_info(name));
			interface.open()?;
			let interface = interface::Interface::Serial(interface);
			futures_lite::stream::once(f(interface).await).left_stream()
		},
		Some(Query::Com(port)) => {
			let name = format!("COM{port}").into();
			let mut interface = serial::Interface::new(unknown_serial_port_info(name));
			interface.open()?;
			let interface = interface::Interface::Serial(interface);
			futures_lite::stream::once(f(interface).await).left_stream()
		},
		Some(Query::Serial(sn)) => {
			let mut interfaces = Vec::new();
			for mut dev in devices_data_with(Some(sn))? {
				dev.open()?;
				dev.interface()?;
				interfaces.push(f(dev.interface.take().unwrap()).await);
			}
			futures_lite::stream::iter(interfaces).right_stream()
		},
		None => {
			let mut interfaces = Vec::new();
			for mut dev in devices_data_with(None)? {
				dev.open()?;
				dev.interface()?;
				interfaces.push(f(dev.interface.take().unwrap()).await);
			}
			futures_lite::stream::iter(interfaces).right_stream()
		},
	};

	Ok(devs)
}
