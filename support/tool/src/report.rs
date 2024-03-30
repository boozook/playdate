use std::borrow::Cow;
use std::path::Path;

use pddev::{device, interface, usb, mount};
use pddev::usb::mode::DeviceMode;


pub trait AsReport {
	#[allow(unused)]
	fn as_report(&self) -> DevInfo<'_>;
	fn as_report_short(&self) -> DevInfoShort<'_>;
}


impl AsReport for device::Device {
	fn as_report(&self) -> DevInfo<'_> { DevInfo::new(self, None) }
	fn as_report_short(&self) -> DevInfoShort<'_> { DevInfoShort::new(self, None) }
}

impl AsReport for mount::MountedDevice {
	fn as_report(&self) -> DevInfo<'_> { DevInfo::new(&self.device, Some(self.handle.path())) }
	fn as_report_short(&self) -> DevInfoShort<'_> { DevInfoShort::new(&self.device, Some(self.handle.path())) }
}


#[derive(Debug, serde::Serialize)]
pub struct DevInfoShort<'dev> {
	#[serde(skip_serializing_if = "Option::is_none")]
	serial: Option<&'dev str>,
	#[serde(flatten)]
	state: DevState<'dev>,
}

impl<'dev> DevInfoShort<'dev> {
	fn new(dev: &'dev device::Device, vol: Option<Cow<'dev, Path>>) -> Self {
		Self { serial: dev.info().serial_number(),
		       state: DevState::new(dev, vol) }
	}

	pub fn to_printable_line(&self) -> impl std::fmt::Display {
		let mut s = String::new();
		self.to_printable_line_to(&mut s);
		s
	}

	pub fn to_printable_line_to(&self, buf: &mut String) {
		if let Some(serial) = self.serial {
			buf.push_str(serial);
			buf.push(' ');
		}
		self.state.to_printable_line_to(buf);
	}
}

#[derive(Debug, serde::Serialize)]
pub struct DevInfo<'dev> {
	address: u8,
	#[serde(skip_serializing_if = "Option::is_none")]
	product: Option<&'dev str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	manufacturer: Option<&'dev str>,
	#[serde(flatten)]
	inner: DevInfoShort<'dev>,
}

impl<'dev> DevInfo<'dev> {
	pub fn new(dev: &'dev device::Device, vol: Option<Cow<'dev, Path>>) -> Self {
		Self { address: dev.info().device_address(),
		       product: dev.info().product_string(),
		       manufacturer: dev.info().manufacturer_string(),
		       inner: DevInfoShort::new(dev, vol) }
	}

	pub fn to_printable_line(&self) -> impl std::fmt::Display {
		let mut s = format!("{:02x} ", self.address);
		if let Some(product) = self.product {
			s.push_str(product);
			s.push(' ');
		}
		if let Some(manufacturer) = self.manufacturer {
			s.push_str(manufacturer);
			s.push(' ');
		}
		s.push_str(&self.inner.to_printable_line().to_string());
		s
	}
}


#[derive(Debug, serde::Serialize)]
pub struct DevState<'dev> {
	mode: Cow<'dev, str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	volume: Option<Cow<'dev, Path>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	interface: Option<Cow<'dev, str>>,
}

impl<'dev> DevState<'dev> {
	pub fn new(dev: &'dev device::Device, vol: Option<Cow<'dev, Path>>) -> Self {
		let interface = interface_repr(dev);
		Self { mode: dev.mode().to_string().into(),
		       volume: vol,
		       interface }
	}

	#[allow(unused)]
	pub fn to_printable_line(&self) -> impl std::fmt::Display {
		let mut s = String::new();
		self.to_printable_line_to(&mut s);
		s
	}

	pub fn to_printable_line_to(&self, buf: &mut String) {
		buf.push_str(&format!("({})", self.mode));

		if let Some(ref volume) = self.volume {
			buf.push(' ');
			buf.push_str(&volume.to_string_lossy());
		}
		if let Some(ref interface) = self.interface {
			buf.push(' ');
			buf.push_str(interface);
		}
	}
}


fn interface_repr(dev: &device::Device) -> Option<Cow<'_, str>> {
	let interface = if matches!(dev.mode(), usb::mode::Mode::Data) {
		match dev.interface().ok()? {
			interface::Interface::Usb(_) => Some(Cow::from("bulk")),
			interface::Interface::Serial(dev) => Some(Cow::from(dev.info().port_name.to_owned())),
		}
	} else {
		None
	};

	interface
}
