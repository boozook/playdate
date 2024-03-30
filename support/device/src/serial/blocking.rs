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

impl crate::interface::blocking::In for Interface {}
