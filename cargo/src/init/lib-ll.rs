#![no_std]
use core::ffi::*;

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate playdate_sys as sys;
use sys::ffi::*;


sys::ll_symbols!();


#[no_mangle]
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	match event {
		PDSystemEvent::kEventInit => unsafe {
			sys::API = api;
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
		let b = (*(*sys::API).system).isCrankDocked.as_ref().unwrap()() != 0;
		if b {
			let f = (*(*sys::API).system).drawFPS.as_ref().unwrap();
			f(40, 40);
		} else {
			let f = (*(*sys::API).graphics).clear.as_ref().unwrap();
			f(0);
		}
	}
}
