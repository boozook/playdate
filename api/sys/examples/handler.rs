//! Entry point / event handler example.

#![no_std]
extern crate alloc;
use core::ffi::*;
use core::ptr::null_mut;
use core::ptr::NonNull;
use alloc::boxed::Box;

#[macro_use]
extern crate playdate_sys as pd;
use pd::EventLoopCtrl;
use pd::ffi::*;


/// Entry point / event handler
#[no_mangle]
fn event_handler(api: NonNull<PlaydateAPI>, event: PDSystemEvent, arg: u32) -> EventLoopCtrl {
	println!("Init");

	// Do something good with `api` here...


	if event == PDSystemEvent::kEventInit {
		// Registering update-callback with user-data,
		// where user-data is just `u32` because not needed nothing complex for this example.
		let state = Box::into_raw(Box::new(0_u32));
		unsafe { api!(system.setUpdateCallback)(Some(update_handler), state as *mut _) };
	}

	EventLoopCtrl::Continue
}


/// Update handler.
///
/// Just count to a hundred and stop the updates.
unsafe extern "C" fn update_handler(state: *mut c_void) -> c_int {
	let ptr: *mut u32 = state.cast();
	let state = ptr.as_mut().expect("missed state");
	*state += 1;

	println!("Counting, {state}");

	if *state == 100 {
		println!("Stopping updates...");
		api!(system.setUpdateCallback)(None, null_mut());
		println!("See you.");
	}


	// Continue updates:
	true.into()
}


// Needed for debug build, optional.
ll_symbols!();
