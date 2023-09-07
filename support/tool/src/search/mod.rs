use crate::model::Device;

pub mod fs;
mod mac;
mod unix;
mod other;
mod uni {
	#[cfg(target_os = "macos")]
	pub use super::mac::*;
	#[cfg(all(unix, not(target_os = "macos")))]
	pub use super::unix::*;
	#[cfg(not(unix))]
	pub use super::other::*;
}


pub fn list_connected_devices() -> Result<Vec<Device>, crate::Error> {
	#[cfg(feature = "usb")]
	let devices = {
		let mut devices = crate::usb::list_usb_devices()?;
		uni::refresh(&mut devices[..])?;
		devices
	};
	#[cfg(not(feature = "usb"))]
	let devices = uni::list_usb_devices()?;

	trace!("discovered devices: {devices:#?}");
	Ok(devices)
}


impl Device {
	fn refresh_from(&mut self, other: Self) {
		if self.mode != other.mode {
			debug!("{}: mode changed: {} <- {}", other.serial, other.mode, self.mode);
			self.mode = other.mode;
		}

		self.serial = other.serial;
		if other.volume.is_some() {
			self.volume = other.volume;
		}
		if other.tty.is_some() {
			self.tty = other.tty;
		}
	}
}
