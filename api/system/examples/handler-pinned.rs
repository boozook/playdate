#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use core::any::Any;
use core::ptr::NonNull;
use alloc::boxed::Box;

use sys::ffi::*;
use sys::EventLoopCtrl;
use system::System;
use system::update::UpdateCtrl;


/// Entry point
#[no_mangle]
fn event_handler(_api: NonNull<PlaydateAPI>, _event: PDSystemEvent, _: u32) -> EventLoopCtrl {
	println!("Init");

	// Do something good with `_api` here...


	// Registering update-callback with user-data.
	// The user-data is just a number because not needed nothing complex for this example.

	static mut HANDLE: Option<Box<dyn Any>> = None;

	if unsafe { HANDLE.is_none() } {
		let system = System::Default();
		let handle = system.set_update_callback(
		                                        move |v| {
			                                        *v += 1;
			                                        println!("{v}");

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
		unsafe { HANDLE = Some(Box::new(handle)) };
	}


	// Continue event-loop:
	EventLoopCtrl::Continue
}


// Needed for debug build
ll_symbols!();
