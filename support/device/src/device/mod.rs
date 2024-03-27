use std::hash::Hash;
pub use nusb::DeviceInfo;
use crate::usb::mode::DeviceMode;
use crate::usb::mode::Mode;

pub mod serial;
pub mod query;
pub mod command;

mod methods;
pub use methods::*;


/// USB device wrapper
pub struct Device {
	// pub serial: Option<SerialNumber>,
	pub(crate) info: DeviceInfo,
	pub(crate) mode: Mode,

	/// Opened device handle
	pub(crate) inner: Option<nusb::Device>,

	// /// Claimed bulk data interface
	// pub(crate) bulk: Option<crate::usb::Interface>,

	// /// Opened serial fallback interface
	// pub(crate) serial: Option<crate::serial::blocking::Interface>,
	pub(crate) interface: Option<crate::interface::Interface>,
}

impl Eq for Device {}
impl PartialEq for Device {
	fn eq(&self, other: &Self) -> bool {
		self.info.serial_number().is_some() && self.info.serial_number() == other.info.serial_number()
	}
}

impl Hash for Device {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let info = &self.info;
		info.serial_number().hash(state);
		info.bus_number().hash(state);
		info.device_address().hash(state);
		info.vendor_id().hash(state);
		info.product_id().hash(state);
		info.class().hash(state);
		info.subclass().hash(state);
		info.protocol().hash(state);
		info.manufacturer_string().hash(state);
		info.product_string().hash(state);
		self.inner.is_some().hash(state);
		self.interface.is_some().hash(state);
	}
}


impl AsRef<DeviceInfo> for Device {
	fn as_ref(&self) -> &DeviceInfo { &self.info }
}

impl std::fmt::Display for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
		       f,
		       "{}({})",
		       self.info.serial_number().unwrap_or("unknown"),
		       self.info.mode()
		)
	}
}

impl std::fmt::Debug for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Device")
		 .field("sn", &self.info.serial_number())
		 .field("mode", &self.mode)
		 .field("open", &self.is_open())
		 .field("ready", &self.is_ready())
		 .field("interface", &self.interface)
		 .finish()
	}
}


impl Device {
	pub fn new(info: DeviceInfo) -> Self {
		Self { mode: info.mode(),
		       info,
		       inner: None,
		       interface: None }
	}

	pub fn info(&self) -> &DeviceInfo { &self.info }
	pub fn into_info(self) -> DeviceInfo { self.info }


	// USB

	/// Cached mode of this device
	pub fn mode_cached(&self) -> Mode { self.mode }
	pub fn is_open(&self) -> bool { self.inner.is_some() || self.is_ready() }
	pub fn is_ready(&self) -> bool {
		match self.interface.as_ref() {
			Some(crate::interface::Interface::Usb(_)) => true,
			Some(crate::interface::Interface::Serial(inner)) => inner.is_open(),
			None => false,
		}
	}
}
