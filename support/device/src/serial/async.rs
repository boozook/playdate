#![cfg(feature = "tokio-serial")]

use std::ops::DerefMut;

// use futures::TryFutureExt;
// use futures::AsyncWriteExt;
// use futures_lite::AsyncWriteExt;
// #[allow(unused_imports)]
// use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

use crate::error::Error;
use super::Interface;


impl crate::interface::r#async::Out for Interface {
	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
	async fn send(&self, data: &[u8]) -> Result<usize, Error> {
		trace!("writing {} bytes to {}", data.len(), self.info.port_name);
		if let Some(ref port) = self.port {
			let mut port = port.try_borrow_mut()?;
			let port = port.deref_mut();
			port.write_all(data).await?;
			port.flush().await?;
			Ok(data.len())
		} else {
			Err(Error::not_ready())
		}
	}
}


// impl<T> crate::interface::blocking::Out for T where T: serialport::SerialPort {}
// impl<T> crate::interface::r#async::In for T where T: DerefMut<Target = tokio_serial::SerialStream> {}

// impl<T> crate::interface::r#async::Out for T where T: DerefMut<Target = tokio_serial::SerialStream> {
// impl<T> crate::interface::r#async::Out for T where T: DerefMut<Target = tokio_serial::SerialStream> {
// 	async fn send_cmd(&self, cmd: crate::device::command::Command) -> Result<usize, Error> {
// 		let port = self.deref_mut();
// 		let cmd = cmd.with_break();
// 		port.write_all(cmd.as_bytes()).await?;
// 		port.flush().await?;
// 		Ok(cmd.as_bytes().len())
// 	}
// }


impl crate::interface::r#async::In for Interface {
	// type Error = crate::error::Error;
}
