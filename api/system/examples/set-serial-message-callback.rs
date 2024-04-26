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
	if event == PDSystemEvent::Init {
		System::Default().set_serial_message_callback(|data| println!("serial_message_callback: {}", data));
	}

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
