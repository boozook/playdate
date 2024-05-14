#![feature(extract_if)]
#![feature(io_error_more)]

#[macro_use]
extern crate log;

pub use utils::*;


pub mod fs;
pub mod value;
pub mod layout;
pub mod config;
pub mod assets;
pub mod metadata;
pub mod manifest;


#[cfg(feature = "cargo")]
#[macro_export(local_inner_macros)]
macro_rules! cargo {
	(env $($arg:tt)+) => (std::println!("cargo:rerun-if-env-changed={}", std::format_args!($($arg)+)));
	(path $($arg:tt)+) => (std::println!("cargo:rerun-if-changed={}", std::format_args!($($arg)+)));
	($($arg:tt)+) => (std::println!("cargo:{}", std::format_args!($($arg)+)))
}

#[cfg(not(feature = "cargo"))]
#[macro_export(local_inner_macros)]
macro_rules! cargo {
	(env $($arg:tt)+) => ((/* no-op */));
	(path $($arg:tt)+) => ((/* no-op */));
	($($arg:tt)+) => ((/* no-op */));
}
