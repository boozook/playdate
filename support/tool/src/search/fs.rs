use std::path::Path;

use crate::model::Device;
use crate::model::Mode;


pub fn is_tty_fd(path: &Path) -> Result<bool, crate::Error> {
	let res = if path.components().count() > 1 && path.try_exists()? {
		#[cfg(unix)]
		{
			use std::os::unix::fs::FileTypeExt;
			let fty = path.metadata()?.file_type();
			fty.is_char_device() || fty.is_block_device() || fty.is_fifo() || fty.is_socket()
		}
		#[cfg(not(unix))]
		// TODO: check
		true
	} else {
		false
	};

	Ok(res)
}


impl Device {
	// TODO: rewrite to `is_data_available()` where check usb (if feature=usb) and tty availability.
	pub fn is_cu_ok(&self) -> bool {
		self.mode == Mode::Data &&
		self.tty
		    .as_deref()
		    .filter(|p| p.try_exists().ok().unwrap_or_default())
		    .is_some()
	}
}
