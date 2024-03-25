// #![cfg(feature = "async")]

use std::future::Future;

use crate::device::command::Command;
use crate::error::Error;


pub trait Out: In {
	// type Error: std::error::Error;

	// fn send(&self, data: &[u8]) -> impl std::future::Future<Output = Result<usize, Self::Error>> + Send;
	fn send_cmd(&self, cmd: Command) -> impl Future<Output = Result<usize, Error>>;
}

pub trait In {
	// type Error: std::error::Error = crate::error::Error;
}

pub trait Interface: Out {}
impl<T: In + Out> Interface for T {}
