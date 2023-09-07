use std::path::PathBuf;

mod serial;
pub use serial::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
	/// DATA / COMM
	Data,
	/// MASS_STORAGE
	Storage,
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Mode::Data => "D",
			Mode::Storage => "S",
		})
	}
}


pub struct Device {
	pub(crate) serial: SerialNumber,

	/// Detected interface
	pub(crate) mode: Mode,

	/// Mount point
	pub(crate) volume: Option<PathBuf>,

	/// Path of fd, cu or tty
	pub(crate) tty: Option<PathBuf>,
}

impl std::fmt::Debug for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut res = f.debug_struct(&self.serial.to_string());
		res.field("mode", &self.mode);
		res.field("dev", &self.tty);
		res.field("path", &self.volume);

		match &self.mode {
			Mode::Storage => res.field("mounted", &self.mounted()),
			Mode::Data => &mut res,
		};

		res.finish()
	}
}

impl std::fmt::Display for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}({})", self.serial, self.mode)
	}
}


struct DeviceFd {
	cu: PathBuf,
}

impl std::fmt::Debug for DeviceFd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut res = f.debug_struct("Fd");
		res.field("cu", &self.cu);
		res.finish()
	}
}

impl std::fmt::Display for DeviceFd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.cu.display()) }
}
