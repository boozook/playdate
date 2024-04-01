use std::borrow::Cow;

use crate::device::serial::SerialNumber;
use crate::device::Device;
use crate::error::Error;
use crate::usb;


type Result<T = (), E = Error> = std::result::Result<T, E>;


/// Create `Device` with serial interface by given `port` name/path.
#[cfg_attr(feature = "tracing", tracing::instrument(fields(port = port.as_ref())))]
pub async fn dev_with_port<S: AsRef<str>>(port: S) -> Result<Device> {
	use serialport::SerialPort;

	let name = port.as_ref();
	let port = super::open(name)?;

	let dev = port.as_ref()
	              .name()
	              .map(Cow::from)
	              .or(Some(name.into()))
	              .and_then(|name| {
		              let mut dev = SerialNumber::try_from(name.as_ref()).map_err(Error::from)
		                                                                 .and_then(|sn| usb::discover::device(&sn))
		                                                                 .ok()?;
		              let mut inter = super::Interface::new(unknown_serial_port_info(name));
		              inter.set_port(port);
		              dev.set_interface(crate::interface::Interface::Serial(inter));
		              Some(dev)
	              });

	// TODO: error: device not found for serial port
	dev.ok_or_else(|| Error::not_found())
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn unknown_serial_port_info(port_name: Cow<'_, str>) -> serialport::SerialPortInfo {
	let unknown = serialport::SerialPortType::Unknown;
	serialport::SerialPortInfo { port_name: port_name.to_string(),
	                             port_type: unknown }
}


/// Open given `interface` and read to stdout infinitely.
#[cfg(feature = "tokio-serial")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip(interface), fields(interface.port_name = interface.info().port_name)))]
pub async fn redirect_interface_to_stdout(interface: &super::Interface) -> Result<(), Error> {
	let mut out = tokio::io::stdout();
	let mut port = interface.port
	                        .as_ref()
	                        .ok_or(Error::not_ready())?
	                        .try_borrow_mut()?;
	tokio::io::copy(port.as_mut(), &mut out).await
	                                        .map_err(Error::from)
	                                        .map(|bytes| debug!("Redirected {bytes} bytes to stdout."))
}

/// Open given `port` and read to stdout infinitely.
#[cfg(feature = "tokio-serial")]
#[cfg_attr(feature = "tracing", tracing::instrument(fields(port.name = serialport::SerialPort::name(port.as_ref()))))]
pub async fn redirect_port_to_stdout(port: &mut super::Port) -> Result<(), Error> {
	let mut out = tokio::io::stdout();
	tokio::io::copy(port, &mut out).await
	                               .map_err(Error::from)
	                               .map(|bytes| debug!("Redirected {bytes} bytes to stdout."))
}


/// Open port by given `port` name/path and read to stdout infinitely.
#[cfg(feature = "tokio-serial")]
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn redirect_to_stdout<S>(port: S) -> Result<(), Error>
	where S: for<'a> Into<Cow<'a, str>> + std::fmt::Debug {
	let mut port = super::open(port)?;
	redirect_port_to_stdout(&mut port).await
}

/// Open port by given `port` name/path and read to stdout infinitely.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn redirect_to_stdout_blocking<S>(port: S) -> Result<(), Error>
	where S: for<'a> Into<Cow<'a, str>> + std::fmt::Debug {
	let mut port = super::open(port)?;

	#[cfg(feature = "tokio-serial")]
	{
		let handle = tokio::runtime::Handle::current();
		std::thread::spawn(move || {
			let fut = redirect_port_to_stdout(&mut port);
			let res = handle.block_on(fut);
			if let Err(err) = res {
				error!("Error redirecting to stdout: {err}");
			}
		}).join()
		.expect("Error when join on the redirecting to stdout thread.");

		Ok(())
	}
	#[cfg(not(feature = "tokio-serial"))]
	{
		let mut port = super::open(port)?;
		let mut out = std::io::stdout();
		std::io::copy(&mut port, &mut out).map_err(Error::from)
		                                  .map(|bytes| debug!("Redirected {bytes} bytes to stdout."))
	}
}
