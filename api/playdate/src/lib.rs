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


pub mod ext {
	use core::ptr::NonNull;


	/// Main Playdate API entry point.
	pub trait PlaydateAPIExt {
		/// Playdate System API.
		fn system(&self) -> system::System<system::api::Cache>;

		// fn file() -> file::File;

		/// Playdate Graphics API.
		fn graphics(&self) -> graphics::Graphics<graphics::api::Cache>;

		// fn sprite() -> sprite::Sprite;

		/// Playdate Display API.
		fn display(&self) -> display::Display<display::api::Cache>;

		// fn sound() -> sound::Sound;
		// fn lua() -> lua::Lua;
		// fn json() -> json::Json;
		// fn scoreboards() -> scoreboards::Scoreboards;
	}


	impl PlaydateAPIExt for NonNull<sys::ffi::PlaydateAPI> {
		fn system(&self) -> system::System<system::api::Cache> {
			system::System::new_with(system::api::Cache::from(unsafe { self.as_ref() }.system))
		}

		fn graphics(&self) -> graphics::Graphics<graphics::api::Cache> {
			graphics::Graphics::new_with(graphics::api::Cache::from(unsafe { self.as_ref() }.graphics))
		}

		fn display(&self) -> display::Display<display::api::Cache> {
			display::Display::new_with(display::api::Cache::from(unsafe { self.as_ref() }.display))
		}
	}

	impl PlaydateAPIExt for *const sys::ffi::PlaydateAPI {
		fn system(&self) -> system::System<system::api::Cache> {
			system::System::new_with(system::api::Cache::from(unsafe { self.as_ref() }.expect("api").system))
		}

		fn graphics(&self) -> graphics::Graphics<graphics::api::Cache> {
			graphics::Graphics::new_with(graphics::api::Cache::from(unsafe { self.as_ref() }.expect("api").graphics))
		}

		fn display(&self) -> display::Display<display::api::Cache> {
			display::Display::new_with(display::api::Cache::from(unsafe { self.as_ref() }.expect("api").display))
		}
	}
}
