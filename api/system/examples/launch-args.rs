#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;


use sys::ffi::{Playdate, SystemEvent};
use sys::ctrl::EventLoopCtrl;
use system::System;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	dbg!(e);

	// Just for this example, ignore all events except init:
	let SystemEvent::Init = e else {
		return EventLoopCtrl::Continue;
	};

	// Api-endpoints:
	let system = System::new(api.system);


	// Examples of various callbacks:

	system.launch_args(|s| println!("args: {s:?}"));

	system.launch_args_path(|args, path| {
		      println!("path: {path:?}, args: {args:?}");

		      if args.is_none() {
			      system.restart(c"test launch-args");
		      }
	      });


	system.update().unset();

	EventLoopCtrl::Continue
}
