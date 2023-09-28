#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use core::ptr::NonNull;

use sys::ffi::*;
use sys::EventLoopCtrl;
use system::System;
use system::update::UpdateCtrl;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: NonNull<PlaydateAPI>, event: PDSystemEvent, arg: u32) -> EventLoopCtrl {
	println!("Init");

	// Do something good with `api` here...


	let system = System::Default();

	// Registering update-callback with user-data.
	// The user-data is just a number because not needed nothing complex for this example.
	system.set_update_callback_boxed(
	                                 |v| {
		                                 *v += 1;
		                                 println!("{v} / 100");

		                                 if *v == 100 {
			                                 println!("Stopping updates...");
			                                 system.set_update_callback_static(None, ());
			                                 println!("See you.");
		                                 }

		                                 // Continue updates:
		                                 UpdateCtrl::Continue
	                                 },
	                                 42,
	);

	// Continue event-loop:
	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
