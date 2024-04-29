#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::*;
use system::System;
use system::event::SystemEventExt as _;
use system::update::UpdateCtrl;


/// Entry point, event handler
#[no_mangle]
fn event_handler(_api: NonNull<PlaydateAPI>, event: PDSystemEvent, _: u32) -> EventLoopCtrl {
	// Just for this example, ignore all events except init:
	if event != PDSystemEvent::Init {
		return EventLoopCtrl::Continue;
	}


	let mut counter: u32 = 0;

	let callback = move |msg| {
		counter += 1;

		println!("[{counter}/3] serial_message_callback: '{}'", msg);

		if counter == 3 {
			println!("stop receiving serial messages");
			System::Default().set_serial_message_callback(None::<fn(_)>);
		}
	};

	// Register callback to start receiving serial messages:
	System::Default().set_serial_message_callback(Some(callback));


	// Also set update callback:
	System::Default().set_update_callback_static(Some(on_update), ());

	// Continue event-loop:
	EventLoopCtrl::Continue
}


/// Update handler
fn on_update(_: &mut ()) -> UpdateCtrl {
	// Continue updates
	UpdateCtrl::Continue
}


// Needed for debug build
ll_symbols!();
