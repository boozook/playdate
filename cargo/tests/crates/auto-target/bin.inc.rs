use core::ffi::*;

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate pd;
use pd::ffi::*;

#[path = "../../shared.rs"]
mod shared;

pd::ll_symbols!();


#[no_mangle]
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	match event {
		PDSystemEvent::kEventInit => unsafe {
			pd::API = api;
			let f = (*(*api).system).setUpdateCallback.unwrap();
			f(Some(on_update), core::ptr::null_mut());

			println!("Init OK");
		},

		PDSystemEvent::kEventLock => println!("Lock"),
		PDSystemEvent::kEventUnlock => println!("Unlock"),
		PDSystemEvent::kEventPause => println!("Pause"),
		PDSystemEvent::kEventResume => println!("Resume"),
		PDSystemEvent::kEventKeyPressed => println!("KeyPressed"),
		PDSystemEvent::kEventKeyReleased => println!("KeyReleased"),
		PDSystemEvent::kEventLowPower => println!("LowPower"),
		PDSystemEvent::kEventInitLua => println!("InitLua"),
		PDSystemEvent::kEventTerminate => println!("Terminate"),
	}

	0
}


unsafe extern "C" fn on_update(_: *mut c_void) -> i32 {
	// This is used for execution tests:
	if let Some(s) = shared::CARGO_PLAYDATE_TEST_VALUE {
		println!("{}{s}", shared::CARGO_PLAYDATE_TEST_VALUE_PREFIX);
	}

	check_crank_docked();
	1
}

#[inline(never)]
pub fn check_crank_docked() {
	unsafe {
		let b = (*(*pd::API).system).isCrankDocked.as_ref().unwrap()() != 0;
		if b {
			let f = (*(*pd::API).system).drawFPS.as_ref().unwrap();
			f(40, 40);
		} else {
			let f = (*(*pd::API).graphics).clear.as_ref().unwrap();
			f(0);
		}
	}
}
