//! Entry point / event handler example.
#![no_std]
#![no_main]

extern crate alloc;
use core::ptr::null_mut;

#[macro_use]
extern crate playdate_sys as pd;
use pd::ctrl::EventLoopCtrl;
use pd::ffi::{Playdate, SystemEvent};


/// Entry point / event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, event: SystemEvent, _key: u32) -> EventLoopCtrl {
	if dbg!(event) == SystemEvent::Init {
		// 💡 Note that api endpoint is already set by the caller - `eventHandlerShim` in the crate.
		// Of course we can use the one we got as a parameter `api` - it's the same thing,
		// the only difference is that when the `api` function (or macro) is called, the extra option-match will happen.
		unsafe { api!(system.setUpdateCallback)(None, null_mut()) };
		// So is better:
		unsafe { (api.system.setUpdateCallback)(None, null_mut()) };
	}

	EventLoopCtrl::Continue
}


#[cfg(miri)]
#[no_mangle]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize { pd::mock::executor::minimal() }


// Needed for device target when building with arm-gcc and linking with its stdlib.
// pd::ll_symbols!();
