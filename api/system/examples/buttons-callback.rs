#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;


use sys::ffi::{Buttons, Playdate, SystemEvent};
use sys::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};
use system::ctrl::buttons::Buttons as _;
use system::prelude::ButtonQueueResult;
use system::time::Milliseconds;
use system::System;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, e: SystemEvent, _: u32) -> EventLoopCtrl {
	// Just for this example, ignore all events except init:
	let SystemEvent::Init = dbg!(e) else {
		return EventLoopCtrl::Continue;
	};

	// Api-endpoints:
	let system = System::new(api.system);


	// Examples of various callbacks:

	system.input().buttons().set_callback(
	                                      |btn: Buttons, down: bool, when| {
		                                      println!("{}, {down}, {when:?}", btn.display());
		                                      ButtonQueueResult::Captured
	                                      },
	                                      5,
	);


	struct Userdata(bool);


	system.input().buttons().set_callback_with(
	                                           |btn: Buttons, down: bool, when: Milliseconds, ud: &Userdata| {
		                                           println!("{}, {down}, {when:?}, ud: {}", btn.display(), ud.0);
		                                           ButtonQueueResult::Captured
	                                           },
	                                           Userdata(true),
	                                           5,
	);


	system.input().buttons().set_callback_with(
	                                           |btn: Buttons,
	                                            down: bool,
	                                            when: Milliseconds,
	                                            ud: &mut Userdata| {
		                                           println!("{}, {down}, {when:?}", btn.display());
		                                           if !ud.0 {
			                                           ud.0 = true;
			                                           println!("\t -> keep btn in the queue");
			                                           ButtonQueueResult::Nope
		                                           } else {
			                                           // rem btn from the queue
			                                           ButtonQueueResult::Captured
		                                           }
	                                           },
	                                           Userdata(false),
	                                           5,
	);


	system.update().set(|| UpdateDisplayCtrl::Nope);

	EventLoopCtrl::Continue
}
