use std::io::prelude::*;

use crate::error::Error;
use super::Interface;


impl crate::interface::blocking::Out for Interface {
	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	fn send_cmd(&self, cmd: crate::device::command::Command) -> Result<usize, Error> {
		trace!("sending `{cmd}` to {}", self.info.port_name);
		if let Some(ref port) = self.port {
			let s = cmd.with_break();
			let mut port = port.try_borrow_mut()?;
			port.write_all(s.as_bytes())?;
			port.flush()?;
			Ok(s.as_bytes().len())
		} else {
			Err(Error::not_ready())
		}
	}
}

impl crate::interface::blocking::In for Interface {
	// type Error = crate::error::Error;
}


// impl crate::interface::blocking::OutInterface for Interface {
// 	type Error = Error;

// 	fn send(&self, buf: &[u8]) -> Result<usize, Self::Error> {
// 		if let Some(ref port) = self.port {
// 			let mut port = port.try_borrow_mut()?;
// 			Ok(port.write(buf).and_then(|len| port.flush().and(Ok(len)))?)
// 		} else {
// 			Err(Error::not_ready())
// 		}
// 	}

// 	fn send_cmd(&self, cmd: crate::device::command::Command) -> Result<usize, Self::Error> {
// 		self.send(cmd.with_break().as_bytes())
// 	}
// }


// impl Write for Interface {
// 	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
// 		if let Some(ref port) = self.port {
// 			port.try_borrow_mut()?.write(buf)
// 		} else {
// 			Err(std::io::Error::new(std::io::ErrorKind::NotConnected, Error::not_ready()))
// 		}
// 	}

// 	fn flush(&mut self) -> std::io::Result<()> {
// 		if let Some(ref port) = self.port {
// 			port.try_borrow_mut()?.flush()
// 		} else {
// 			Err(std::io::Error::new(std::io::ErrorKind::NotConnected, Error::not_ready()))
// 		}
// 	}
// }


// impl futures::AsyncWrite for Interface {
// 	fn poll_write(self: std::pin::Pin<&mut Self>,
// 	              cx: &mut std::task::Context<'_>,
// 	              buf: &[u8])
// 	              -> std::task::Poll<std::io::Result<usize>> {
// 		todo!()
// 	}

// 	fn poll_flush(self: std::pin::Pin<&mut Self>,
// 	              cx: &mut std::task::Context<'_>)
// 	              -> std::task::Poll<std::io::Result<()>> {
// 		todo!()
// 	}

// 	fn poll_close(self: std::pin::Pin<&mut Self>,
// 	              cx: &mut std::task::Context<'_>)
// 	              -> std::task::Poll<std::io::Result<()>> {
// 		todo!()
// 	}
// }
