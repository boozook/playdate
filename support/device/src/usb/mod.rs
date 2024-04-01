use std::borrow::Cow;

use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::FutureExt;
use futures::TryFutureExt;
use nusb::transfer::RequestBuffer;
use nusb::transfer::TransferError;
use nusb::DeviceInfo;
use nusb::InterfaceInfo;
use object_pool::Pool;
use object_pool::Reusable;

use crate::device::command::Command;
use crate::device::Device;
use crate::error::Error;

use self::mode::DeviceMode;
use self::mode::Mode;

pub mod mode;
pub mod discover;
pub mod io;


const BULK_IN: u8 = 0x81;
const BULK_OUT: u8 = 0x01;

#[allow(dead_code)]
const INTERRUPT_IN: u8 = 0x82;


pub trait HaveDataInterface {
	fn data_interface_number(&self) -> Option<u8>;
	fn have_data_interface(&self) -> bool { self.data_interface_number().is_some() }
}

impl HaveDataInterface for DeviceInfo {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	fn data_interface_number(&self) -> Option<u8> {
		self.interfaces()
		    .find(|i| i.class() == 0xA | 2)
		    .map(|i| i.interface_number())
	}
}

impl HaveDataInterface for nusb::Device {
	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	fn data_interface_number(&self) -> Option<u8> {
		let cfg = self.active_configuration().ok()?;
		for i in cfg.interfaces() {
			let bulk = i.alt_settings().find(|i| i.class() == 0xA | 2);
			if bulk.is_some() {
				return bulk.map(|i| i.interface_number());
			}
		}
		None
	}
}

impl HaveDataInterface for Device {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	fn data_interface_number(&self) -> Option<u8> {
		self.info
		    .data_interface_number()
		    .or_else(|| self.inner.as_ref()?.data_interface_number())
	}
}

pub trait MassStorageInterface {
	fn storage_interface(&self) -> Option<&InterfaceInfo>;
	fn have_storage_interface(&self) -> bool;
}

impl MassStorageInterface for DeviceInfo {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	fn storage_interface(&self) -> Option<&InterfaceInfo> { self.interfaces().find(|i| i.class() == 8) }
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	fn have_storage_interface(&self) -> bool { self.storage_interface().is_some() }
}


impl Device {
	/// 1. Find this device
	/// 1. Compare `mode` of `this` vs. just found
	/// 1. [if changed] Update state of `this`, drop all pending transfers if needed
	///    to prevent future errors when send to unexisting interface.
	/// 1. Return `true` if `mode` changed.
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn refresh(&mut self) -> Result<bool, Error> {
		let mode = self.info.mode();
		if mode != self.mode {
			self.mode = mode;
			self.interface.take();
			self.inner.take();
			debug!(
			       "{}: refreshed by existing.",
			       self.info.serial_number().unwrap_or("unknown")
			);
			Ok(true)
		} else {
			let updated = crate::usb::discover::devices()?.find(|dev| {
				                                              let serial = dev.info().serial_number();
				                                              serial.is_some() && serial == self.info.serial_number()
			                                              });
			if let Some(dev) = updated {
				let mode = dev.mode_cached();
				let changed = mode != self.mode;
				if changed {
					self.mode = mode;
					self.info = dev.info;
					self.interface.take();
					self.inner.take();
					debug!(
					       "{}: refreshed by existing new.",
					       self.info.serial_number().unwrap_or("unknown")
					);
				}
				Ok(changed)
			} else {
				debug!(
				       "{}: device not found.",
				       self.info.serial_number().unwrap_or("unknown")
				);
				self.interface.take();
				self.inner.take();
				Ok(true)
			}
		}
	}


	/// Open USB interface if available,
	/// otherwise try open serial port if available.
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn open(&mut self) -> Result<(), Error> {
		if !matches!(self.mode, Mode::Data) {
			return Err(Error::WrongState(self.mode));
		}

		trace!("opening device");

		// Special case: if we already have an interface, mostly possible serial:
		if self.interface.is_some() {
			return match self.interface_mut()? {
				crate::interface::Interface::Serial(i) => i.open(),
				_ => Ok(()),
			};
		}

		if self.have_data_interface() {
			let bulk = self.try_bulk().map(|_| {});
			if let Some(err) = bulk.err() {
				self.try_serial().map_err(|err2| Error::chain(err2, [err]))
			} else {
				self.interface()
			}
		} else {
			self.try_serial()
		}?;
		Ok(())
	}

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	fn try_bulk(&mut self) -> Result<&crate::interface::Interface, Error> {
		if let Some(ref io) = self.interface {
			Ok(io)
		} else if let Some(ref dev) = self.inner {
			let id = self.info
			             .data_interface_number()
			             .or_else(|| dev.data_interface_number())
			             .ok_or_else(|| Error::not_found())?;
			// previously used 0x01.
			self.interface = Some(Interface::from(dev.claim_interface(id)?).into());
			Ok(self.interface.as_ref().unwrap())
		} else {
			let dev = self.info.open()?;
			let id = self.info
			             .data_interface_number()
			             .or_else(|| dev.data_interface_number())
			             .ok_or_else(|| Error::not_found())?;
			self.interface = Some(Interface::from(dev.claim_interface(id)?).into());
			self.inner = Some(dev);
			Ok(self.interface.as_ref().unwrap())
		}
	}

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	fn try_serial(&mut self) -> Result<&crate::interface::Interface, Error> {
		use crate::serial::Interface;


		let mut errors = Vec::new();
		let port = {
			crate::serial::discover::ports_for(&self).map(|ports| ports.map(|port| Interface::new(port)))?
			                                         .find_map(|mut port| {
				                                         // try to open port, we could get an permission error
				                                         match port.open() {
					                                         Ok(_) => Some(port),
				                                            Err(err) => {
					                                            errors.push(err);
					                                            None
				                                            },
				                                         }
			                                         })
		};

		if let Some(port) = port {
			self.interface = Some(port.into());
			self.interface()
		} else {
			Err(Error::chain(Error::not_found(), errors))
		}
	}


	/// Async read-write interface.
	pub fn interface(&self) -> Result<&crate::interface::Interface, Error> {
		self.interface.as_ref().ok_or_else(|| Error::not_ready())
	}

	pub fn interface_mut(&mut self) -> Result<&mut crate::interface::Interface, Error> {
		self.interface.as_mut().ok_or_else(|| Error::not_ready())
	}

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	pub fn set_interface(&mut self, interface: crate::interface::Interface) {
		self.close();
		self.interface = Some(interface);
	}

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	pub fn close(&mut self) {
		self.info.serial_number().map(|s| debug!("closing {s}"));
		self.interface.take();
		self.inner.take();
	}

	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	pub fn close_with_reset(&mut self) {
		self.info.serial_number().map(|s| debug!("closing* {s}"));
		self.interface.take();
		if let Some(dev) = self.inner.take() {
			dev.reset().map_err(|err| error!("{err}")).ok();
		}
	}
}

impl crate::interface::blocking::Out for Interface {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	fn send_cmd(&self, cmd: Command) -> Result<usize, Error> {
		use crate::interface::r#async;
		let fut = <Self as r#async::Out>::send_cmd(self, cmd);
		futures_lite::future::block_on(fut).map_err(Into::into)
	}
}

impl crate::interface::blocking::In for Interface {}


impl crate::interface::r#async::Out for Interface {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	async fn send(&self, data: &[u8]) -> Result<usize, Error> { self.write(data).map_err(Into::into).await }
}

impl crate::interface::r#async::In for Interface {}


pub struct Interface {
	inner: nusb::Interface,
}

impl From<nusb::Interface> for Interface {
	fn from(interface: nusb::Interface) -> Self { Self { inner: interface } }
}

impl std::fmt::Display for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "usb") }
}

impl std::fmt::Debug for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "usb") }
}

impl Interface {
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn write(&self, data: &[u8]) -> impl std::future::Future<Output = Result<usize, TransferError>> {
		trace!("writing {} bytes", data.len());
		self.inner.bulk_out(BULK_OUT, data.to_vec()).map(|comp| {
			                                            // TODO: attach data to the pool
			                                            let written = comp.data.actual_length();
			                                            let data = comp.data.reuse();
			                                            let s = std::str::from_utf8(&data).map(Cow::Borrowed)
			                                                                              .unwrap_or_else(|_| {
				                                                                              Cow::Owned(hex::encode_upper(&data))
			                                                                              });
			                                            trace!("sent, resp: ({written}) '{s}'");
			                                            comp.status.map(|_| written)
		                                            })
	}
}


pub struct PoolStream<'pool> {
	pool: &'pool Pool<Vec<u8>>,
	queue: nusb::transfer::Queue<RequestBuffer>,
	buffer_size: usize,
	// inner: futures_lite::stream::PollFn<Option<Result<Vec<u8>, TransferError>>>,
}

impl<'pool> futures::Stream for PoolStream<'pool> {
	type Item = Result<Reusable<'pool, Vec<u8>>, TransferError>;

	fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		self.queue.poll_next(ctx).map(|comp| {
			                         let data = comp.data;
			                         match comp.status {
				                         Ok(_) => {
				                            // prepare next request
				                            let buffer_size = self.buffer_size;
				                            let (_, buf) =
					                            self.pool.pull(|| Vec::with_capacity(buffer_size)).detach();
				                            self.queue.submit(RequestBuffer::reuse(buf, buffer_size));
				                            // make received data reusable
				                            let data = Reusable::new(self.pool, data);
				                            Some(Ok(data))
			                            },
			                            Err(err) => {
				                            self.pool.attach(data);
				                            self.queue.cancel_all();
				                            Some(Err(err))
			                            },
			                         }
		                         })
	}

	fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(self.queue.pending())) }
}
