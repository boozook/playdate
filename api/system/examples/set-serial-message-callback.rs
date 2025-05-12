#![no_std]
#![no_main]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;


use sys::ffi::*;
use sys::ctrl::*;
use system::System;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	// Just for this example, ignore all events except init:
	if e != SystemEvent::Init {
		return EventLoopCtrl::Continue;
	}

	// Our api-endpoint:
	let system = System::new(api.system);

	// Just counter to give a size (ctx) for the callback, just for example:
	let mut counter: u32 = 0;

	let on_msg = move |msg: &CStr, ctrl| {
		counter += 1;
		println!("msg #{counter}/3: {msg:?}");
		if counter < 3 {
			core::mem::forget(ctrl);
		} else {
			drop(ctrl);
			println!("serial message subscription canceled");
		}
	};

	// Register callback to start receiving serial messages:
	system.set_serial_message_callback(Some(on_msg));

	// Also set update callback:
	system.set_update_callback(Some(on_update));

	// Continue event-loop:
	EventLoopCtrl::Continue
}


/// Update handler
fn on_update() -> UpdateDisplayCtrl { UpdateDisplayCtrl::Nope }


// Needed for debug build
ll_symbols!();
