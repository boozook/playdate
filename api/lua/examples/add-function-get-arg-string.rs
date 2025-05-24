#![no_std]
#![no_main]
#![allow(unused_must_use)]

extern crate alloc;
extern crate sys;
extern crate playdate_lua as lua;

use core::ffi::c_int;

use lua::Lua;
use sys::ffi::*;
use system::System;
use sys::ffi::{Playdate, SystemEvent};
use sys::ctrl::{EventLoopCtrl, UpdateDisplayCtrl};
use sys::macros::api;


/// Entry point, event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// We need to set our update callback in the InitLua handler instead of Init.
	// https://devforum.play.date/t/lua-c-minimal-example/4354/5
	//
	// Just for this example, ignore all other events.
	if event != SystemEvent::InitLua {
		return EventLoopCtrl::Continue;
	}

	// Set update callback
	System::new(api.system).update().set(|| UpdateDisplayCtrl::Nope);

	// Add a function that we depend on and call in main.lua
	Lua::new(api.lua).add_function(Some(log_to_console_from_main_dot_lua), c"example.logToConsole")
	                 .expect("add_function 'log_to_console_from_main_dot_lua' should succeed");

	// Continue event loop
	EventLoopCtrl::Continue
}


// The function we add to the Lua runtime and call from main.lua
pub unsafe extern "C" fn log_to_console_from_main_dot_lua(_lua_state: *mut LuaState) -> c_int {
	// We know that our function takes a single argument which is a string.
	let arg_string = Lua::default().get_arg_string(1)
	                               .expect("get_arg_string should succeed");

	// Avoid going from CString to str and back with playdate::sys::log::println
	let f = api!(system.logToConsole);
	f(arg_string.as_ptr());

	// A `lua_CFunction` should return the number of return values it has pushed
	// onto the stack.
	0
}
