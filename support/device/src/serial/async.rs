#![cfg(feature = "tokio-serial")]
#![cfg(feature = "tokio")]

use std::ops::DerefMut;

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


impl crate::interface::r#async::In for Interface {}
