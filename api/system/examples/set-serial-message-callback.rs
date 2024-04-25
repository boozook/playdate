#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use core::ffi::c_char;
use core::ffi::CStr;
use core::ptr::NonNull;

use sys::EventLoopCtrl;
use sys::ffi::*;
use system::System;
use system::event::SystemEventExt as _;
use system::update::UpdateCtrl;


pub unsafe extern "C" fn serial_message_callback(data: *const c_char) {
	let data = CStr::from_ptr(data as _).to_string_lossy().into_owned();
	println!("serial_message_callback: {}", data);
}

/// Entry point, event handler
#[no_mangle]
fn event_handler(_api: NonNull<PlaydateAPI>, event: PDSystemEvent, _: u32) -> EventLoopCtrl {
	if event == PDSystemEvent::Init {
		System::Default().set_serial_message_callback(Some(serial_message_callback));
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
