#![feature(exit_status_error)]
#![feature(buf_read_has_data_left)]
#![feature(extract_if)]

#[macro_use]
extern crate log;

use std::path::PathBuf;
use std::time::Duration;

pub mod model;
pub mod search;
pub mod mount;
pub mod usb;

use model::Device;

#[cfg(feature = "cli")]
pub mod cli;
pub mod io;

pub use mount::DEVICE_MOUNT_POINT_DEF;
pub const DEVICE_SERIAL_ENV: &str = "PLAYDATE_SERIAL_DEVICE";
pub const DEVICE_MOUNT_POINT_ENV: &str = "PLAYDATE_MOUNT_POINT";


pub struct OnDevicePath {
	pub device: Device,

	/// Path relative to the device mount point.
	/// __Not absolute path.__
	pub path: PathBuf,
}


pub(crate) fn wait_for<F: Fn() -> bool>(f: F, delay: Duration, max: Duration) -> Result<(), Error> {
	wait_for_mut(f, delay, max)
}

pub(crate) fn wait_for_mut<F: FnMut() -> bool>(mut f: F, delay: Duration, max: Duration) -> Result<(), Error> {
	let start = std::time::Instant::now();
	let mut res = false;
	while !res && start.elapsed() < max {
		res = f();
		if !res {
			std::thread::sleep(delay);
		}
	}
	if !res { Err(Error::timeout()) } else { Ok(()) }
}


#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("{0}")]
	ExitStatus(#[from] std::process::ExitStatusError),

	#[error("{0}")]
	Utf8(#[from] std::str::Utf8Error),

	#[error("{0}")]
	#[cfg(feature = "usb")]
	Usb(#[from] rusb::Error),

	#[error("{0}")]
	Format(#[from] model::DeviceSerialFormatError),

	#[error("{0}")]
	Json(#[from] serde_json::Error),

	#[error("{0}")]
	Plist(#[from] plist::Error),

	#[error("Env var '{1}': {0}.")]
	Env(std::env::VarError, &'static str),

	#[error("{0}")]
	Error(String),

	#[error("{0}")]
	Err(&'static str),
}

impl From<String> for Error {
	fn from(value: String) -> Self { Self::Error(value) }
}

impl From<&'static str> for Error {
	fn from(value: &'static str) -> Self { Self::Err(value) }
}

impl Error {
	pub const fn device_not_found() -> Self { Self::Err("Playdate device not found") }
	pub const fn device_already_mounted() -> Self { Self::Err("Playdate device already mounted") }
	fn unable_to_find_tty<D: std::fmt::Display>(device: D) -> Self {
		Self::Error(format!("Unable to find tty for '{device}'"))
	}
	fn named_device_not_found(name: String) -> Self { Self::Error(format!("Playdate device '{name}' not found")) }
	fn env(name: &'static str, err: std::env::VarError) -> Self { Self::Env(err, name) }

	fn invalid_device_name() -> Self {
		Self::Err("Invalid device name, should be in format PDUN_XNNNNNN or path to file-descriptor e.g.: /dev/cu.usbmodemPDUN_XNNNNNN.")
	}

	pub fn multiple_devices<T: AsRef<[Device]>>(devices: T) -> Self {
		let s = format!(
		                "Many devices are found: [{}].\n Please choose one and pass as argument.",
		                devices.as_ref()
		                       .into_iter()
		                       .map(|d| format!("{d}"))
		                       .collect::<Vec<_>>()
		                       .join(", ")
		);


		Self::Error(format!("Playdate device '{s}' not found"))
	}

	fn timeout() -> Self { Self::Err("Max timeout reached") }
}
