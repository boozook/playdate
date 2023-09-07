#![cfg(all(unix, not(target_os = "macos")))]
use crate::Error;
use crate::model::Device;


/// Get a list of all available devices by OS
pub fn list_usb_devices() -> Result<Vec<Device>, Error> { Ok(tty::find_tty_devices()?.collect::<Vec<_>>()) }


impl Device {
	pub fn refresh(&mut self) -> Result<(), Error> { self.refresh_tty() }

	pub fn refresh_tty(&mut self) -> Result<(), Error> {
		self.tty = tty::find_tty_for(&self.serial)?;
		Ok(())
	}
}


/// Get a list of all available devices by OS
/// and update the `devices`.
pub fn refresh(devices: &mut [Device]) -> Result<(), Error> {
	list_usb_devices()?.into_iter().for_each(|a| {
		                               devices.iter_mut()
		                                      .find(|b| b.serial == a.serial)
		                                      .map(|b| b.refresh_from(a));
	                               });
	Ok(())
}


mod tty {
	use std::path::PathBuf;

	use regex::Regex;
	use crate::Error;
	use crate::model::Device;
	use crate::model::Mode;
	use crate::model::SerialNumber;


	/// Search for a cu fd that looks like to Playdate.
	pub fn find_tty_devices() -> Result<impl Iterator<Item = Device>, Error> {
		// TODO: fix this, use check like for mac: prefix + `SerialNumber::try_from` instead of this regex:
		let re = Regex::new(r"^usb-Panic_Inc_Playdate_(PDU\d+[_-].+)$").expect("invalid regex");
		let devices =
			std::fs::read_dir("/dev/serial/by-id")?.filter_map(move |entry| {
				                                       let entry = entry.ok()?;
				                                       let name = entry.file_name();
				                                       let name_lossy = name.to_string_lossy();
				                                       let captures = re.captures(name_lossy.as_ref())?;
				                                       let _ = captures.get(1)?.as_str();
				                                       let path = entry.path().canonicalize().ok()?;
				                                       if path.to_string_lossy().contains("tty") {
					                                       Some(Device { serial: name_lossy.as_ref().parse().ok()?,
					                                                     tty: Some(path),
					                                                     mode: Mode::Data,
					                                                     volume: None })
				                                       } else {
					                                       None
				                                       }
			                                       });
		// TODO: warn errors and outfiltered
		Ok(devices)
	}

	pub fn find_tty_for(serial: &SerialNumber) -> Result<Option<PathBuf>, Error> {
		Ok(find_tty_devices()?.filter_map(move |device| {
			                      if &device.serial == serial {
				                      device.tty
			                      } else {
				                      None
			                      }
		                      })
		                      .next())
	}
}
