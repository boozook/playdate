use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use crate::Error;
use crate::model::Device;


impl Device {
	pub fn write<S: AsRef<str>>(&self, command: S) -> Result<(), Error> {
		debug!("send command: '{}'", command.as_ref());
		let first_try = {
			debug!("  first try...");
			#[cfg(feature = "usb")]
			let res = crate::usb::write(self, command.as_ref()).map_err(Error::from);
			#[cfg(not(feature = "usb"))]
			let res: Result<(), Error> = Err(Error::Err("`usb` feature disabled"));
			res
		};

		match first_try {
			Ok(_) => Ok(()),
			Err(err) => {
				warn!("{err}, trying fallback");
				// we have fallback solution
				#[cfg(unix)]
				let res = unix::tty_write(
				                 &self.tty.as_deref().ok_or(Error::unable_to_find_tty(self))?,
				                 command,
				);
				// or not
				#[cfg(not(unix))]
				let res = Err(err);
				res
			},
		}
	}

	pub fn read_to_stdout(&self, echo: Option<bool>) -> Result<(), Error> {
		let first_try = {
			#[cfg(feature = "usb")]
			let res = crate::usb::read_output(self, echo).map_err(Error::from);
			#[cfg(not(feature = "usb"))]
			let res: Result<(), Error> = Err(Error::Err("`usb` feature disabled"));
			res
		};

		match first_try {
			Ok(_) => Ok(()),
			Err(err) => {
				warn!("{err}, trying fallback");
				// we have fallback solution
				#[cfg(unix)]
				let res = unix::tty_to_stdout_by_device(self, echo);
				// or not
				#[cfg(not(unix))]
				let res = Err(err);
				res
			},
		}
	}
}


#[cfg(unix)]
mod unix {
	use std::process::Command;

	use super::*;


	pub(super) fn tty_to_stdout_by_device(device: &Device, echo: Option<bool>) -> Result<(), Error> {
		let tty = device.tty.as_ref().ok_or(Error::unable_to_find_tty(device))?;
		tty_to_stdout(tty, echo)
	}


	pub(super) fn tty_to_stdout(tty: &Path, echo: Option<bool>) -> Result<(), Error> {
		debug!("Redirecting {} output to this output", tty.display());

		if let Some(echo) = echo {
			let v = if echo { "on" } else { "off" };
			tty_write(tty, format!("echo {v}")).ok();
		}

		// Cat can everything!
		Command::new("cat").arg(&tty)
		                   .stdout(std::process::Stdio::inherit())
		                   .status()?
		                   .exit_ok()?;

		// Sometimes cat see something like EOF, that impossible, so fallback:
		let mut reader = BufReader::new(File::open(&tty)?);
		println!("");
		loop {
			if reader.has_data_left()? {
				let mut buf = String::new();
				reader.read_to_string(&mut buf)?;
				print!("{buf}");
			}
		}
	}


	pub(super) fn tty_write<S: AsRef<str>>(tty: &Path, command: S) -> Result<(), Error> {
		let command = command.as_ref();
		trace!("Write to {}: '{command}'", tty.display());


		let mut file = File::options().read(false)
		                              .write(true)
		                              .create(false)
		                              .create_new(false)
		                              .open(&tty)?;
		file.write_all(command.trim().as_bytes())?;
		file.write("\n".as_bytes())?;
		file.flush()?;

		trace!("Write to {}: complete.", tty.display());
		Ok(())
	}
}
