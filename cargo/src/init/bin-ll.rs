#![no_std]
#![no_main]
#[allow(unused_imports)]
extern crate alloc;
extern crate playdate_sys as sys;
extern crate {crate_name};

use core::ffi::c_int;
use {crate_name}::eventHandlerShim;


#[used]
/// TODO: describe that it's needed to tell rustc that it should be in the output after DCE, LTO and stripping.
pub static EVENT_HANDLER_SHIM: extern "C" fn(*const sys::ffi::PlaydateAPI, sys::ffi::PDSystemEvent, u32) -> c_int = eventHandlerShim;
