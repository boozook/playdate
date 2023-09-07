#![cfg(not(unix))]
#[cfg(not(feature = "usb"))]
compile_error!("Currently unsupported platform, please try build with feature `usb`.");


use crate::Error;
use crate::model::Mode;
use crate::model::Device;


pub fn list_usb_devices() -> Result<Vec<Device>, Error> { Ok(vec![]) }

/// Get a list of all available devices by OS
/// and update the `devices`.
pub fn refresh(devices: &mut [Device]) -> Result<(), Error> { Ok(()) }

impl Device {
	pub fn refresh(&mut self) -> Result<(), Error> {
		// XXX: I have no idea how it works on windows and other non-unix systems.
		Ok(())
	}

	pub fn refresh_tty(&mut self) -> Result<(), Error> {
		// XXX: I have no idea how it works on windows and other non-unix systems.
		Ok(())
	}
}
