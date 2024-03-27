use std::borrow::Cow;
use std::fmt::Debug;

use serialport::{SerialPortInfo, SerialPortType, available_ports};

use crate::device::Device;
use crate::error::Error;
use crate::{VENDOR_ID, PRODUCT_ID_DATA};


/// Enumerate all serial ports on the system for Playdate devices.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn ports() -> Result<impl Iterator<Item = SerialPortInfo>, Error> {
	let iter = available_ports()?.into_iter().filter(|port| {
		                                         match port.port_type {
			                                         SerialPortType::UsbPort(ref info) => {
			                                            info.vid == VENDOR_ID && info.pid == PRODUCT_ID_DATA
		                                            },
		                                            _ => false,
		                                         }
	                                         });
	Ok(iter)
}


/// Search exact one serial port for device with same serial number.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn port<SN>(sn: &SN) -> Result<SerialPortInfo, Error>
	where SN: PartialEq<str> + Debug {
	let port = ports()?.find(move |port| {
		                   match port.port_type {
			                   SerialPortType::UsbPort(ref info) => {
			                      info.serial_number.as_ref().filter(|s| sn.eq(s)).is_some()
		                      },
		                      _ => false,
		                   }
	                   });
	port.ok_or_else(|| Error::not_found())
}


/// Search serial ports for device with same serial number,
/// or __any__ Playdate- serial port if `sn` is `None`.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn ports_with<SN>(sn: Option<SN>) -> Result<impl Iterator<Item = SerialPortInfo>, Error>
	where SN: PartialEq<str> + Debug {
	let ports = ports()?.filter(move |port| {
		                    match port.port_type {
			                    SerialPortType::UsbPort(ref info) => {
			                       if let Some(sn) = sn.as_ref() {
				                       trace!("found port: {}, dev sn: {:?}", port.port_name, info.serial_number);
				                       info.serial_number
				                           .as_ref()
				                           .filter(|s| {
					                           let res = sn.eq(s);
					                           trace!("sn is ≈ {res}");
					                           res
				                           })
				                           .is_some()
			                       } else {
				                       true
			                       }
		                       },
		                       _ => false,
		                    }
	                    });
	Ok(ports)
}

/// Search serial ports for device with same serial number,
/// or __any__ Playdate- serial port if `sn` is `None`.
///
/// In case of just one device and just one port found, serial number will not be used for matching, so it returns.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn ports_with_or_single<SN>(sn: Option<SN>) -> Result<impl IntoIterator<Item = SerialPortInfo>, Error>
	where SN: PartialEq<str> + Debug {
	let ports: Vec<_> = ports()?.collect();
	let devs: Vec<_> = crate::usb::discover::devices_data()?.collect();

	if ports.len() == 1 && devs.len() == 1 {
		trace!("Auto-match single found dev with port without sn match.");
		Ok(ports)
	} else {
		let ports =
			ports.into_iter().filter(move |port| {
				                 match port.port_type {
					                 SerialPortType::UsbPort(ref info) => {
					                    if let Some(sn) = sn.as_ref() {
						                    trace!("found port: {}, dev sn: {:?}", port.port_name, info.serial_number);
						                    info.serial_number
						                        .as_ref()
						                        .filter(|s| {
							                        let res = sn.eq(s);
							                        trace!("sn is ≈ {res}");
							                        res
						                        })
						                        .is_some()
					                    } else {
						                    true
					                    }
				                    },
				                    _ => false,
				                 }
			                 });
		Ok(ports.collect())
	}
}


#[cfg_attr(feature = "tracing", tracing::instrument(skip(dev)))]
/// Search serial ports for `device`` with same serial number.
#[cfg(not(target_os = "windows"))]
pub fn ports_for(dev: &Device) -> Result<impl Iterator<Item = SerialPortInfo> + '_, Error> {
	ports_with(dev.info().serial_number().map(Cow::Borrowed))
}
#[cfg(target_os = "windows")]
///
/// _On Windows in some strange cases of serial number of the device that behind founded COM port
/// can't be determined of we get just part of it, so we need to use another method to match devices
/// in case of there is just one device and port._
pub fn ports_for(dev: &Device) -> Result<impl Iterator<Item = SerialPortInfo> + '_, Error> {
	ports_with_or_single(dev.info().serial_number().map(Cow::Borrowed)).map(|v| v.into_iter())
}
