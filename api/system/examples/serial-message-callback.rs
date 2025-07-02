#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(internal_features, unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;

use core::intrinsics::caller_location as here;

use sys::ffi::{CStr, CString, Playdate, SystemEvent};
use sys::ctrl::EventLoopCtrl;
use system::System;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	// Just for this example, ignore all events except init:
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	// Api-endpoint:
	let system = System::new(api.system);


	// Examples of various callbacks:
	let mut counter: u32 = 0;
	let on_msg = move |msg: &CStr| {
		counter += 1;
		println!("msg #{counter}/3: {msg:?} at {}", here());
		if counter == 3 {
			system.serial().set(move |msg: &CStr| {
				               counter += 1;
				               println!("msg #{counter}/4: {msg:?} at {}", here());
				               system.serial().set(|msg: CString| {
					                              println!("serial msg: {msg:?} at {}", here());
				                              });
			               });
		}
	};

	fn on_msg_fn(msg: &CStr) {
		println!("serial msg: {msg:?} at {}", here());
	}


	system.serial().set(on_msg_fn);

	system.serial().set(on_msg);

	system.update().unset();

	EventLoopCtrl::Continue
}
