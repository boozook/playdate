use crate::device::command::Command;
use crate::error::Error;

pub trait Out: In {
	fn send_cmd(&self, cmd: Command) -> Result<usize, Error>;
}

pub trait In {
}

pub trait Interface: In + Out {}
// impl<T: In<Error = Err> + Out<Error = Err>, Err> Interface for T {}
impl<T: In + Out> Interface for T {}
