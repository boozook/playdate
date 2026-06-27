#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate sys;

use core::ffi::c_char;
use core::ops::Deref;
use alloc::borrow::ToOwned;

use sys::ffi::CStr;
use sys::ffi::CString;
use sys::ffi::LuaCFunction;


pub mod error;
use error::AddFunctionError;


type Api = &'static sys::ffi::PlaydateLua;


#[derive(Clone, Copy)]
pub struct Lua(Api);

impl Deref for Lua {
	type Target = Api;
	fn deref(&self) -> &Self::Target { &self.0 }
}


impl Default for Lua {
	fn default() -> Self { Self(sys::macros::api!(lua)) }
}

impl Lua {
	pub const fn new(api: Api) -> Self { Self(api) }
}


impl Lua {
	/// Adds the Lua function *f* to the Lua runtime, with name *name*. (*name*
	/// can be a table path using dots, e.g. if name = “mycode.myDrawingFunction”
	/// adds the function “myDrawingFunction” to the global table “myCode”.)
	///
	/// Equivalent to [`sys::ffi::PlaydateLua::addFunction`]
	#[doc(alias = "sys::ffi::PlaydateLua::addFunction")]
	pub fn add_function<S: AsRef<CStr>>(&self, f: LuaCFunction, name: S) -> Result<(), AddFunctionError> {
		let mut err: *const c_char = core::ptr::null();
		let func = self.0.addFunction;
		let name = name.as_ref();

		// Returns 1 on success or 0 with an error message in `err`.
		let result = unsafe { func(f, name.as_ptr(), &raw mut err) };

		if result == 0 {
			let err_msg = unsafe { CStr::from_ptr(err) };
			Err(AddFunctionError(err_msg.to_owned()))
		} else {
			Ok(())
		}
	}

	/// Returns the argument at position *pos* as a string.
	///
	/// Equivalent to [`sys::ffi::PlaydateLua::getArgString`]
	#[doc(alias = "sys::ffi::PlaydateLua::getArgString")]
	pub fn get_arg_string(&self, pos: i32) -> Option<CString> {
		let f = self.0.getArgString;
		unsafe {
			let ptr = f(pos);
			if ptr.is_null() {
				None
			} else {
				Some(CStr::from_ptr(ptr).to_owned())
			}
		}
	}
}
