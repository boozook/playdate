#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use alloc::string::String;
use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::*;
use system::System;
use system::event::SystemEventExt as _;
use system::prelude::*;

struct State {
	initialized: bool,
	latest_message: Option<String>,
}

impl State {
	fn new() -> Self {
		Self { initialized: false,
		       latest_message: None }
	}

	/// System event handler
	fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
		match event {
			SystemEvent::Init => {
				System::Default().set_serial_message_callback(Some(|msg| {
					                 self.latest_message = Some(msg);
				                 }));

				// Verify that `set_serial_message_callback` doesn't prevent us from
				// updating other parts of `State`
				self.initialized = true;

				println!("Game init complete");
			},
			_ => {},
		}
		EventLoopCtrl::Continue
	}
}

impl Update for State {
	fn update(&mut self) -> UpdateCtrl {
		if let Some(latest_message) = self.latest_message.take() {
			println!("Latest message: {:#?}", latest_message);
		}
		UpdateCtrl::Continue
	}
}

/// Entry point
#[no_mangle]
pub fn event_handler(_api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
	pub static mut STATE: Option<State> = None;

	if unsafe { STATE.is_none() } {
		let state = State::new();
		unsafe { STATE = Some(state) }
	}

	// We call `set_update_handler` here because it requires (mutably) borrowing
	// all of `State`.
	//
	// If we were to do this inside of `event` it would prevent us from being able
	// to (mutably) borrow any of `State`, which means we couldn't update any
	// state inside of our `set_serial_message_callback` closure.
	if event == SystemEvent::Init {
		unsafe { STATE.as_mut().expect("impossible").set_update_handler() }
	}

	unsafe { STATE.as_mut().expect("impossible").event(event) }
}

// Needed for debug build
ll_symbols!();
