#![no_std]
#![no_main]
#[allow(unused_imports)]
extern crate alloc;
extern crate playdate as pd;
extern crate {crate_name};

use core::ffi::c_int;
use core::ptr::NonNull;
use {crate_name}::event_handler;
use pd::sys::eventHandlerShim;
use pd::sys::ffi::{{PlaydateAPI, PDSystemEvent}};


// Needed to tell rustc that it should be in the output after DCE, LTO and stripping.
#[used] pub static EVENT_HANDLER: extern "Rust" fn(NonNull<PlaydateAPI>, PDSystemEvent, u32) -> bool = event_handler;
#[used] pub static EVENT_HANDLER_SHIM: extern "C" fn(*const PlaydateAPI, PDSystemEvent, u32) -> c_int = eventHandlerShim;
