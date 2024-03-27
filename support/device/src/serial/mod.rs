use std::borrow::Cow;
use std::cell::RefCell;

use crate::error::Error;


pub mod discover;
pub mod blocking;
pub mod r#async;

mod methods;
pub use methods::*;


#[cfg(not(feature = "tokio-serial"))]
type Port = Box<dyn serialport::SerialPort>;
#[cfg(feature = "tokio-serial")]
type Port = Box<tokio_serial::SerialStream>;

pub struct Interface {
	info: serialport::SerialPortInfo,
	port: Option<RefCell<Port>>,
}


impl Interface {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn new(info: serialport::SerialPortInfo) -> Self { Self { info, port: None } }

	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn new_with(port: Port, name: Option<String>) -> Self {
		use serialport::{SerialPort, SerialPortType, SerialPortInfo};

		let name = port.name().or(name).map(Cow::from).unwrap_or_default();
		let info = SerialPortInfo { port_name: name.to_string(),
		                            port_type: SerialPortType::Unknown };

		let mut result = Self::new(info);
		result.set_port(port);
		result
	}

	pub fn info(&self) -> &serialport::SerialPortInfo { &self.info }
	pub fn is_open(&self) -> bool { self.port.is_some() }

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(self.port_name = self.info().port_name)))]
	pub fn set_port(&mut self, port: Port) { self.port = Some(RefCell::new(port)); }

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(self.port_name = self.info().port_name)))]
	pub fn open(&mut self) -> Result<(), Error> {
		if self.port.is_some() {
			Ok(())
		} else {
			let port = open(&self.info.port_name).map(RefCell::new)?;
			self.port = Some(port);
			Ok(())
		}
	}


	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(self.port_name = self.info().port_name)))]
	pub fn close(&mut self) { self.port.take(); }
}


#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn open<'a, S: Into<std::borrow::Cow<'a, str>>>(port_name: S) -> Result<Port, Error>
	where S: std::fmt::Debug {
	let builder = port_builder(port_name);

	#[cfg(not(feature = "tokio-serial"))]
	{
		Ok(builder.open()?)
	}
	#[cfg(feature = "tokio-serial")]
	{
		use tokio_serial::SerialPortBuilderExt;
		Ok(builder.open_native_async().map(Box::new)?)
	}
}

fn port_builder<'a>(port_name: impl Into<std::borrow::Cow<'a, str>>) -> serialport::SerialPortBuilder {
	serialport::new(port_name, 115200).data_bits(serialport::DataBits::Eight)
}


/* NOTE: This can be safely sent between thread, but not inner port,
			but that's okay because it's boxen under `RefCell`.
			Probably should be pinned, but not sure yet.
*/
unsafe impl Send for Interface {}
unsafe impl Sync for Interface {}
