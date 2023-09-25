#![cfg_attr(not(test), no_std)]
extern crate alloc;

#[allow(unused_imports)]
#[macro_use]
pub extern crate sys;
pub extern crate menu;
pub extern crate display;
pub extern crate ctrl as controls;
pub extern crate gfx as graphics;


// macro re-export
pub use sys::{println, ll_symbols, api, api_opt, api_ok};


pub mod system {
	pub use system::*;
	pub use menu;
}

pub mod sound {
	pub use sound::*;
	pub use sound::prelude::*;
}

pub mod sprite {
	pub use sprite::*;
	pub use sprite::prelude::*;
}

pub mod fs {
	pub use fs::*;
	pub use fs::prelude::*;
}
