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

		/// Playdate Peripherals API.
		fn peripherals(&self) -> controls::peripherals::Peripherals<controls::api::Cache>;

		/// Playdate File-system API.
		fn file(&self) -> fs::Fs<fs::api::Cache>;

		/// Playdate Graphics API.
		fn graphics(&self) -> graphics::Graphics<graphics::api::Cache>;

		// fn sprite() -> sprite::Sprite;

		/// Playdate Display API.
		fn display(&self) -> display::Display<display::api::Cache>;

		/// Playdate Sound API.
		fn sound(&self) -> sound::Sound<sound::api::Cache>;

		// fn lua() -> lua::Lua;
		// fn json() -> json::Json;

		fn scoreboards(&self) -> scoreboards::Scoreboards<scoreboards::api::Cache>;
	}


	impl PlaydateAPIExt for NonNull<sys::ffi::PlaydateAPI> {
		fn system(&self) -> system::System<system::api::Cache> {
			system::System::new_with(system::api::Cache::from(unsafe { self.as_ref() }.system))
		}

		fn peripherals(&self) -> controls::peripherals::Peripherals<controls::api::Cache> {
			let api = system::api::Cache::from(unsafe { self.as_ref() }.system);
			controls::peripherals::Peripherals::new_with(api.into())
		}

		fn file(&self) -> fs::Fs<fs::api::Cache> {
			fs::Fs::new_with(fs::api::Cache::from(unsafe { self.as_ref() }.file))
		}

		fn graphics(&self) -> graphics::Graphics<graphics::api::Cache> {
			graphics::Graphics::new_with(graphics::api::Cache::from(unsafe { self.as_ref() }.graphics))
		}

		fn display(&self) -> display::Display<display::api::Cache> {
			display::Display::new_with(display::api::Cache::from(unsafe { self.as_ref() }.display))
		}

		fn sound(&self) -> sound::Sound<sound::api::Cache> {
			sound::Sound::new_with(sound::api::Cache::from(unsafe { self.as_ref() }.sound))
		}

		fn scoreboards(&self) -> scoreboards::Scoreboards<scoreboards::api::Cache> {
			scoreboards::Scoreboards::new_with(scoreboards::api::Cache::from(unsafe { self.as_ref() }.scoreboards))
		}
	}

	impl PlaydateAPIExt for *const sys::ffi::PlaydateAPI {
		fn system(&self) -> system::System<system::api::Cache> {
			system::System::new_with(system::api::Cache::from(unsafe { self.as_ref() }.expect("api").system))
		}

		fn peripherals(&self) -> controls::peripherals::Peripherals<controls::api::Cache> {
			let api = system::api::Cache::from(unsafe { self.as_ref() }.expect("api").system);
			controls::peripherals::Peripherals::new_with(api.into())
		}

		fn file(&self) -> fs::Fs<fs::api::Cache> {
			fs::Fs::new_with(fs::api::Cache::from(unsafe { self.as_ref() }.expect("api").file))
		}

		fn graphics(&self) -> graphics::Graphics<graphics::api::Cache> {
			graphics::Graphics::new_with(graphics::api::Cache::from(unsafe { self.as_ref() }.expect("api").graphics))
		}

		fn display(&self) -> display::Display<display::api::Cache> {
			display::Display::new_with(display::api::Cache::from(unsafe { self.as_ref() }.expect("api").display))
		}

		fn sound(&self) -> sound::Sound<sound::api::Cache> {
			sound::Sound::new_with(sound::api::Cache::from(unsafe { self.as_ref() }.expect("api").sound))
		}

		fn scoreboards(&self) -> scoreboards::Scoreboards<scoreboards::api::Cache> {
			let api = scoreboards::api::Cache::from(unsafe { self.as_ref() }.expect("api").scoreboards);
			scoreboards::Scoreboards::new_with(api)
		}
	}
}
