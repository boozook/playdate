use std::borrow::Cow;
use std::cell::RefCell;


pub mod discover;
mod blocking;
mod r#async;

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


impl std::fmt::Display for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use serialport::SerialPort;

		let port_name = &self.info.port_name;
		let name = self.port.as_ref().and_then(|p| {
			                             p.try_borrow()
			                              .ok()
			                              .and_then(|p| p.name().filter(|s| s != port_name))
		                             });

		write!(f, "serial:{}", name.as_deref().unwrap_or(port_name))
	}
}

impl std::fmt::Debug for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Interface")
		 .field("name", &self.info.port_name)
		 .field("opened", &self.port.is_some())
		 .finish()
	}
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

	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn set_port(&mut self, port: Port) { self.port = Some(RefCell::new(port)); }

	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn open(&mut self) -> Result<(), serialport::Error> {
		if self.port.is_some() {
			Ok(())
		} else {
			let port = open(&self.info.port_name).map(RefCell::new)?;
			self.port = Some(port);
			Ok(())
		}
	}


	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn close(&mut self) { self.port.take(); }
}


#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn open<'a, S>(port_name: S) -> Result<Port, serialport::Error>
	where S: Into<std::borrow::Cow<'a, str>> + std::fmt::Debug {
	trace!("opening port {port_name:?}");
	let builder = port_builder(port_name);

	let port;
	#[cfg(not(feature = "tokio-serial"))]
	{
		port = builder.open()?;
	}
	#[cfg(feature = "tokio-serial")]
	{
		use tokio_serial::SerialPortBuilderExt;
		port = builder.open_native_async().map(Box::new)?;
	}

	{
		use serialport::SerialPort;
		let name = port.as_ref().name();
		let name = name.as_deref().unwrap_or("n/a");
		trace!("opened port: {name}");
	}
	Ok(port)
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
