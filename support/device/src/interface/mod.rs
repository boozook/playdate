use crate::error::Error;

pub mod blocking;
pub mod r#async;


pub enum Interface {
	Usb(crate::usb::Interface),
	Serial(crate::serial::Interface),
}

impl From<crate::usb::Interface> for Interface {
	fn from(interface: crate::usb::Interface) -> Self { Self::Usb(interface) }
}

impl From<crate::serial::Interface> for Interface {
	fn from(interface: crate::serial::Interface) -> Self { Self::Serial(interface) }
}


impl std::fmt::Debug for Interface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Usb(_) => f.debug_tuple("Usb").finish(),
			Self::Serial(i) => f.debug_tuple("Serial").field(i.info()).finish(),
		}
	}
}


impl r#async::Out for Interface
	where crate::usb::Interface: r#async::Out,
	      crate::serial::Interface: r#async::Out
{
	#[inline(always)]
	async fn send_cmd(&self, cmd: crate::device::command::Command) -> Result<usize, Error> {
		match self {
			Interface::Usb(i) => r#async::Out::send_cmd(i, cmd).await,
			Interface::Serial(i) => r#async::Out::send_cmd(i, cmd).await,
		}
	}
}

impl r#async::In for Interface
	where crate::usb::Interface: r#async::In,
	      crate::serial::Interface: r#async::In
{
	// type Error = Error;
}


impl blocking::Out for Interface {
	#[inline(always)]
	fn send_cmd(&self, cmd: crate::device::command::Command) -> Result<usize, Error> {
		match self {
			Interface::Usb(i) => blocking::Out::send_cmd(i, cmd),
			Interface::Serial(i) => blocking::Out::send_cmd(i, cmd),
		}
	}
}

impl blocking::In for Interface {
	// type Error = crate::error::Error;
}


pub trait AsyncSend {
	fn send_cmd(&mut self,
	            cmd: crate::device::command::Command)
	            -> impl std::future::Future<Output = Result<usize, Error>>;
}


mod ext {
	use super::*;


	impl<T> AsyncSend for T
		where T: tokio::io::AsyncWriteExt,
		      Self: Unpin
	{
		#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
		async fn send_cmd(&mut self, cmd: crate::device::command::Command) -> Result<usize, Error> {
			let cmd = cmd.with_break();
			let bytes = cmd.as_bytes();
			self.write_all(bytes).await?;
			self.flush().await?;
			Ok(bytes.len())
		}
	}
}
