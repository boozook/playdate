use std::future::Future;

use crate::device::command::Command;
use crate::error::Error;


pub trait Out: In {
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

pub trait In {}

pub trait Interface: Out {}
impl<T: In + Out> Interface for T {}
