#![no_std]
extern crate alloc;

#[macro_use]
extern crate sys;
extern crate playdate_lua as lua;

use core::ffi::c_int;
use core::ptr::NonNull;

use lua::Lua;
use sys::EventLoopCtrl;
use sys::ffi::*;
use system::System;
use system::event::SystemEventExt as _;
use system::update::UpdateCtrl;


/// Entry point, event handler
#[no_mangle]
fn event_handler(_api: NonNull<PlaydateAPI>, event: PDSystemEvent, _: u32) -> EventLoopCtrl {
	// We need to set our update callback in the InitLua handler instead of Init.
	// https://devforum.play.date/t/lua-c-minimal-example/4354/5
	//
	// Just for this example, ignore all other events.
	if event != PDSystemEvent::InitLua {
		return EventLoopCtrl::Continue;
	}

	// Set update callback
	System::Default().set_update_callback_static(Some(on_update), ());

	// Add a function that we depend on and call in main.lua
	Lua::Default().add_function(Some(log_to_console_from_main_dot_lua), "example.logToConsole")
	              .expect("add_function 'log_to_console_from_main_dot_lua' should succeed");

	// Continue event loop
	EventLoopCtrl::Continue
}


/// Update handler
fn on_update(_: &mut ()) -> UpdateCtrl {
	// Continue updates
	UpdateCtrl::Continue
}


// The function we add to the Lua runtime and call from main.lua
pub unsafe extern "C" fn log_to_console_from_main_dot_lua(_lua_state: *mut lua_State) -> c_int {
	// We know that our function takes a single argument which is a string.
	let arg_string = Lua::Default().get_arg_string(1)
	                               .expect("get_arg_string should succeed");

	// Avoid going from CString to str and back with playdate::sys::log::println
	let f = (*(*sys::API).system).logToConsole
	                             .expect("get logToConsole to succeed");

	f(arg_string.as_ptr());

	// A `lua_CFunction` should return the number of return values it has pushed
	// onto the stack.
	0
}


// Needed for debug build
ll_symbols!();
