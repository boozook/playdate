#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![allow(internal_features)]
// tests, benches:
#![cfg_attr(test, feature(test))]
// until not published:
#![allow(static_mut_refs)]
#![allow(non_snake_case)]


extern crate alloc;

#[macro_use]
#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(test)]
extern crate test;


pub mod ffi;
mod miri;
mod system;
// mod file;
mod graphics;
// mod sprite;
mod display;
// mod sound;
// mod lua;
// mod json;
// mod scoreboards;
// mod network;


#[cfg(feature = "executor")]
pub mod executor {
	use super::*;
	use ffi::Playdate;
	use ffi::SystemEvent;


	/// Minimal run, produces events:
	/// - Init
	/// - update x3
	/// - Pause
	/// - Resume
	/// - update x3 with press A on the first update
	/// - Lock
	/// - Unlock
	/// - update x3
	/// - Terminate
	pub fn minimal() -> isize {
		extern "C" {
			fn eventHandlerShim(api: &'static Playdate, event: SystemEvent, key: u32) -> i32;
		}

		let mock = &MOCK;

		unsafe {
			eventHandlerShim(mock, SystemEvent::Init, 0);

			for _ in 0..3 {
				system::call_update();
			}

			eventHandlerShim(mock, SystemEvent::Pause, 0);
			eventHandlerShim(mock, SystemEvent::Resume, 0);

			{
				let a = ffi::Buttons::A;
				let none = ffi::Buttons(0);
				system::setButtonState(a, a, none);
				system::call_update();
				system::setButtonState(none, none, a);
				system::call_update();
				system::setButtonState(none, none, none);
			}

			for _ in 0..3 {
				system::call_update();
			}

			eventHandlerShim(mock, SystemEvent::Lock, 0);
			eventHandlerShim(mock, SystemEvent::Unlock, 0);

			for _ in 0..3 {
				system::call_update();
			}

			eventHandlerShim(mock, SystemEvent::Terminate, 0);
		}

		0
	}
}


pub static MOCK: ffi::Playdate = ffi::Playdate { system: &system::SYSTEM,
																// file: &file::FILE,
																graphics: &graphics::GRAPHICS,
																// sprite: &sprite::SPRITE,
																display: &display::DISPLAY,
																// sound: &sound::SOUND,
																// lua: &lua::LUA,
																// json: &json::JSON,
																// scoreboards: &scoreboards::SCOREBOARDS,
																// network: &network::NETWORK
															  };
