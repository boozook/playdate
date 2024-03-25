use nusb::DeviceInfo;

use crate::PRODUCT_ID_DATA;
use crate::PRODUCT_ID_STORAGE;
use super::HaveDataInterface;
use super::MassStorageInterface;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
	/// DATA / COMM
	Data,
	/// MASS_STORAGE
	Storage,
	Unknown,
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Mode::Data => 'D',
			Mode::Storage => 'S',
			Mode::Unknown => '?',
		})
	}
}


pub trait DeviceMode {
	/// USB device mode determined by the product ID.
	fn mode(&self) -> Mode;

	/// Actual USB device mode determined by available interfaces.
	fn mode_actual(&self) -> Mode;
}


impl DeviceMode for DeviceInfo {
	fn mode(&self) -> Mode {
		match self.product_id() {
			PRODUCT_ID_DATA => Mode::Data,
			PRODUCT_ID_STORAGE => Mode::Storage,
			_ => Mode::Unknown,
		}
	}

	fn mode_actual(&self) -> Mode {
		if self.have_data_interface() {
			Mode::Data
		} else {
			if self.have_storage_interface() {
				Mode::Storage
			} else {
				Mode::Unknown
			}
		}
	}
}


impl DeviceMode for super::Device {
	fn mode(&self) -> Mode { self.info().mode() }
	fn mode_actual(&self) -> Mode { self.info().mode_actual() }
}
