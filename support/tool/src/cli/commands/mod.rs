use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use crate::Error;
use crate::model::Device;
use crate::model::Mode;
use crate::model::SerialNumber;
use crate::search::fs::is_tty_fd;
use crate::search::list_connected_devices;

pub mod mount;
pub mod install;
pub mod run;
pub mod read;


#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(feature = "cli", command(author, version, about, long_about = None, name = "device"))]
pub struct DeviceQuery {
	/// Serial number of usb device or absolute path to file-descriptor
	///
	/// Value format example: 'PDUN-XNNNNNNN' or '/dev/cu.usbmodemPDUN_XNNNNNNN'
	#[cfg_attr(feature = "cli", arg(env = crate::DEVICE_SERIAL_ENV, name = "device"))]
	pub value: Option<DeviceQueryValue>,
}

impl Default for DeviceQuery {
	fn default() -> Self {
		Self { value: std::env::var(crate::DEVICE_SERIAL_ENV).map(|s| DeviceQueryValue::from_str(&s).ok())
		                                                     .ok()
		                                                     .flatten() }
	}
}


#[derive(Clone, Debug)]
pub enum DeviceQueryValue {
	Serial(SerialNumber),
	Path(PathBuf),
}

type ParseError = <SerialNumber as FromStr>::Err;
impl FromStr for DeviceQueryValue {
	type Err = crate::Error;

	fn from_str(name: &str) -> Result<Self, Self::Err> {
		let name = name.trim();
		if name.is_empty() {
			return Err(ParseError::from(name).into());
		}

		let serial = SerialNumber::try_from(name)?;
		let path = Path::new(name);
		let is_direct = path.is_absolute() && is_tty_fd(path)?;

		if is_direct {
			Ok(DeviceQueryValue::Path(path.to_owned()))
		} else {
			Ok(DeviceQueryValue::Serial(serial))
		}
	}
}

impl DeviceQueryValue {
	pub fn to_printable_string(&self) -> String {
		match self {
			Self::Serial(sn) => sn.to_string(),
			Self::Path(p) => p.display().to_string(),
		}
	}
}


/// Automatically find ony one device. Error if found many.
pub fn find_one_device(query: DeviceQuery) -> Result<Device, Error> {
	let devices = list_connected_devices()?;
	let mut device = if let Some(query) = query.value {
		use DeviceQueryValue as Query;

		let device = match query {
			Query::Serial(serial) => devices.into_iter().find(|device| device.serial == serial),
			Query::Path(path) => {
				if !is_tty_fd(&path)? {
					return Err(Error::Error(format!("{} is not a cu/tty", path.display())).into());
				}

				devices.into_iter()
				       .map(|mut device| {
					       if device.mode == Mode::Data && device.tty.is_none() {
						       device.refresh_tty().ok();
					       }
					       device
				       })
				       .find(|device| {
					       if matches!(device.tty.as_ref(), Some(ref p) if p.as_path() == path.as_path()) {
						       true
					       } else {
						       false
					       }

					       // TODO: create new device with path
				       })
				       .or_else(|| {
					       let maybe = SerialNumber::try_from(path.as_path()).ok()
					                                                         .unwrap_or_else(SerialNumber::unknown);
					       Some(Device { serial: maybe,
					                     mode: Mode::Data,
					                     tty: Some(path.to_path_buf()),
					                     volume: None })
				       })
			},
		};
		device.ok_or(Error::device_not_found())?
	} else {
		if devices.is_empty() {
			return Err(Error::device_not_found().into());
		} else if devices.len() > 1 {
			return Err(Error::multiple_devices(devices).into());
		}
		devices.into_iter().next().ok_or(Error::device_not_found())?
	};

	if device.tty.is_none() {
		device.refresh_tty().ok();
	}

	Ok(device)
}
