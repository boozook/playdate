#![no_std]
extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use core::ffi::*;
use core::ptr::NonNull;
use pd::sys::ffi::PlaydateAPI;

use pd::display::Display;
use pd::graphics::*;
use pd::graphics::bitmap::*;
use pd::system::prelude::*;


/// Game state
struct State {
	// TODO: Fill the state
}


impl State {
	fn new() -> Self {
		// TODO: Init the state

		Self {}
	}


	/// System event handler
	fn event(&'static mut self, event: SystemEvent) -> bool {
		match event {
			// Initial setup
			SystemEvent::Init => {
				// Set FPS to 30
				Display::Default().set_refresh_rate(30.0);

				// Register our update handler that defined below
				self.set_update_handler();

				println!("Game init complete");
			},
			// TODO: React to other events
			_ => {},
		}
		true
	}
}


impl Update for State {
	/// Updates the state
	fn update(&mut self) -> bool {
		clear(Color::WHITE);


		// TODO: update the state of game


		System::Default().draw_fps(0, 0);

		true
	}
}


/// Entry point
#[no_mangle]
fn event_handler(_api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> bool {
	// Unsafe static storage for our state.
	// Usually it's safe because there's only one thread.
	pub static mut STATE: Option<State> = None;
	if unsafe { STATE.is_none() } {
		let state = State::new();
		unsafe { STATE = Some(state) }
	}

	// Call state.event
	unsafe { STATE.as_mut().expect("impossible") }.event(event)
}


// Needed for debug build, absolutely optional
ll_symbols!();
