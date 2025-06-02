#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_system as system;


use core::ffi::{CStr, c_char};
use sys::ffi::{CString, Playdate, SystemEvent};
use sys::ctrl::EventLoopCtrl;
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
	let ntp = system.time().server_time();


	// Examples of various callbacks:
	ntp.set(move |res: Result<&CStr, &CStr>| {
		   println!("res: {res:?}");
		   ntp.set(move |res: Result<CString, &CStr>| {
			      println!("res: {res:?}");
			      ntp.set(move |time: Option<CString>, err: Option<CString>| {
				         println!("err: {err:?}, time: {time:?}");
				         ntp.set(move |time: Option<&CStr>, err: Option<&CStr>| {
					         println!("err: {err:?}, time: {time:?}");
					         ntp.set(|time: *const c_char, err: *const c_char| println!("err: {err:?}, time: {time:?}, done"));
				         });
			         });
		      });
	   });


	system.update().unset();

	EventLoopCtrl::Continue
}
