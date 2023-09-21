#![cfg_attr(not(test), no_std)]

extern crate alloc;
extern crate sys;

pub extern crate menu;
pub extern crate system;
pub extern crate display;
pub extern crate ctrl as controls;
pub extern crate gfx as graphics;

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
