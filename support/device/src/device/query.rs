use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use super::serial::SerialNumber;


pub const DEVICE_SERIAL_ENV: &str = "PLAYDATE_SERIAL_DEVICE";


#[derive(Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "clap", command(author, version, about, long_about = None, name = "device"))]
pub struct DeviceQuery {
	/// Serial number of usb device or absolute path to socket.
	/// Format: 'PDUN-XNNNNNN'
	#[cfg_attr(unix, doc = "or '/dev/cu.usbmodemPDUN_XNNNNNN(N)'.")]
	#[cfg_attr(windows, doc = "or 'COM{X}', where {X} is a number of port, e.g.: COM3.")]
	#[cfg_attr(feature = "clap", arg(env = DEVICE_SERIAL_ENV, name = "device"))]
	pub value: Option<DeviceQueryValue>,
}

impl Default for DeviceQuery {
	fn default() -> Self {
		Self { value: std::env::var(DEVICE_SERIAL_ENV).map(|s| DeviceQueryValue::from_str(&s).ok())
		                                              .ok()
		                                              .flatten() }
	}
}

impl std::fmt::Display for DeviceQuery {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.value {
			Some(ref value) => value.fmt(f),
			None => write!(f, "None"),
		}
	}
}

impl std::fmt::Debug for DeviceQuery {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.value.as_ref() {
			Some(value) => f.debug_tuple("Query").field(&value.to_string()).finish(),
			None => f.debug_tuple("Query").field(&None::<()>).finish(),
		}
	}
}

impl std::fmt::Display for DeviceQueryValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DeviceQueryValue::Serial(sn) => write!(f, "sn:{sn}"),
			DeviceQueryValue::Path(path) => write!(f, "serial:{}", path.display()),
			DeviceQueryValue::Com(port) => write!(f, "serial:COM{port}"),
		}
	}
}


#[derive(Clone, Debug)]
pub enum DeviceQueryValue {
	/// Serial number of usb device.
	Serial(SerialNumber),
	/// Absolute path of serial/modem/telnet-socket.
	///
	/// In case of unmounting or installing it also can by mount-point.
	#[cfg_attr(not(unix), doc = "Use only on Unix.")]
	Path(PathBuf),
	/// COM port.
	#[cfg_attr(not(windows), doc = "Use only on Windows.")]
	Com(u16),
}

type ParseError = <SerialNumber as FromStr>::Err;
impl FromStr for DeviceQueryValue {
	type Err = crate::error::Error;

	fn from_str(dev: &str) -> Result<Self, Self::Err> {
		let name = dev.trim();
		if name.is_empty() {
			return Err(ParseError::from(name).into());
		}

		#[cfg(windows)]
		match name.strip_prefix("COM").map(|s| s.parse::<u16>()) {
			Some(Ok(com)) => return Ok(DeviceQueryValue::Com(com)),
			Some(Err(err)) => todo!("Invalid format, seems to COM port, but {err}."),
			None => { /* nothing there */ },
		}

		let serial = SerialNumber::try_from(name);
		let path = Path::new(name);
		let is_direct = path.is_absolute() && path.exists();

		match serial {
			Ok(serial) => {
				if is_direct {
					Ok(DeviceQueryValue::Path(path.to_owned()))
				} else {
					Ok(DeviceQueryValue::Serial(serial))
				}
			},
			Err(err) => {
				if is_direct {
					Ok(DeviceQueryValue::Path(path.to_owned()))
				} else {
					Err(err.into())
				}
			},
		}
	}
}

impl<'s> TryFrom<&'s str> for DeviceQueryValue {
	type Error = crate::error::Error;
	fn try_from(dev: &'s str) -> Result<Self, Self::Error> { Self::from_str(dev) }
}

impl DeviceQueryValue {
	pub fn to_printable_string(&self) -> String {
		match self {
			Self::Serial(sn) => sn.to_string(),
			Self::Path(p) => p.display().to_string(),
			Self::Com(n) => format!("COM{n}"),
		}
	}
}
