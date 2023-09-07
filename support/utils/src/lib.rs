#[cfg(feature = "log")]
extern crate log;

#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{info, trace, debug, warn, error};

#[cfg(not(feature = "log"))]
#[allow(unused_imports)]
use std::{println as info, println as warn, eprintln as error};

#[cfg(not(feature = "log"))]
#[macro_export(local_inner_macros)]
macro_rules! no_op { ($($arg:tt)+) => ((/* no-op */)); }
#[cfg(not(feature = "log"))]
use self::{no_op as trace, no_op as debug};


pub mod compile;
pub mod consts;
pub mod toolchain {
	pub mod sdk;
	pub mod gcc;
}
