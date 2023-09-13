#![no_std]
#![no_main]
#[allow(unused_imports)]
extern crate alloc;
extern crate game;
use game::eventHandlerShim;

#[used]
pub static EVENT_HANDLER_SHIM: extern "C" fn(*const pd::ffi::PlaydateAPI, pd::ffi::PDSystemEvent, u32) -> i32 =
	eventHandlerShim;
