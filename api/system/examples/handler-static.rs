#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::*;
use system::System;
use system::update::UpdateCtrl;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: NonNull<PlaydateAPI>, event: PDSystemEvent, arg: u32) -> EventLoopCtrl {
	println!("Init");

	// Do something good with `api` here...


	// Registering update-callback with user-data.
	// The user-data is just a number because not needed nothing complex for this example.
	System::Default().set_update_callback_static(Some(on_update), 42);


	// Continue event-loop:
	EventLoopCtrl::Continue
}


/// Update handler
fn on_update(v: &mut i32) -> UpdateCtrl {
	*v += 1;
	println!("{v} / 100");

	if *v == 100 {
		println!("Stopping updates...");
		System::Default().set_update_callback_static(None, ());
		println!("See you.");
	}

	// Continue updates:
	UpdateCtrl::Continue
}


// Needed for debug build
ll_symbols!();
