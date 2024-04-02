use std::future::Future;

use crate::device::command::Command;
use crate::error::Error;


pub trait Out: In {
	// type Error: std::error::Error;

	fn send(&self, data: &[u8]) -> impl Future<Output = Result<usize, Error>>;

	fn send_cmd(&self, cmd: Command) -> impl Future<Output = Result<usize, Error>> {
		async move {
			let mut pre = 0;
			if !matches!(cmd, Command::Echo { .. }) {
				use crate::device::command::Switch;

				trace!("send cmd: echo off");
				let echo = Command::Echo { value: Switch::Off };
				pre = self.send(echo.with_break().as_bytes()).await?;
			}

			trace!("send cmd: {cmd}");
			let sent = self.send(cmd.with_break().as_bytes()).await?;
			Ok(pre + sent)
		}
	}
}

pub trait In {
	// type Error: std::error::Error = crate::error::Error;
}

pub trait Interface: Out {}
impl<T: In + Out> Interface for T {}


// pub trait AsyncSend {
// 	fn send_cmd(&mut self,
// 	            cmd: crate::device::command::Command)
// 	            -> impl std::future::Future<Output = Result<usize, Error>>;
// }


// mod ext {
// 	use super::*;


// 	impl<T> AsyncSend for T
// 		where T: tokio::io::AsyncWriteExt,
// 		      Self: Unpin
// 	{
// 		#[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
// 		async fn send_cmd(&mut self, cmd: crate::device::command::Command) -> Result<usize, Error> {
// 			let cmd = cmd.with_break();
// 			let bytes = cmd.as_bytes();
// 			self.write_all(bytes).await?;
// 			self.flush().await?;
// 			Ok(bytes.len())
// 		}
// 	}
// }
