#![no_std]
#![no_main] // for bin

use core::ffi::*;

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate pd;
use pd::ffi::*;


// for bin:
#[used]
pub static EVENT_HANDLER_SHIM: extern "C" fn(*const pd::ffi::PlaydateAPI, pd::ffi::PDSystemEvent, u32) -> i32 = eventHandlerShim;


#[no_mangle]
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	match event {
		PDSystemEvent::kEventInit => unsafe {
			pd::API = api;
			let f = (*(*api).system).setUpdateCallback.unwrap();
			f(Some(on_update), core::ptr::null_mut());

			println!("Init");
		},

		PDSystemEvent::kEventLock => println!("Lock"),
		PDSystemEvent::kEventUnlock => println!("Pause!"),
		PDSystemEvent::kEventPause => println!("Pause"),
		PDSystemEvent::kEventResume => println!("Resume"),
		PDSystemEvent::kEventKeyPressed => println!("KeyPressed"),
		PDSystemEvent::kEventKeyReleased => println!("KeyReleased"),
		PDSystemEvent::kEventLowPower => println!("LowPower"),
		PDSystemEvent::kEventInitLua => println!("InitLua"),
		PDSystemEvent::kEventTerminate => panic!("It finally works!!!"),
	}

	0
}


unsafe extern "C" fn on_update(_: *mut c_void) -> i32 {
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
