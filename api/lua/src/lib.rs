#![cfg_attr(not(test), no_std)]

// #[macro_use]
extern crate sys;
extern crate alloc;

use core::ffi::c_char;
use alloc::borrow::ToOwned;

use sys::ffi::CStr;
use sys::ffi::CString;
use sys::ffi::lua_CFunction;


pub mod error;

use error::*;

#[derive(Debug, Clone, Copy)]
pub struct Lua<Api = api::Default>(Api);

impl Lua<api::Default> {
	/// Creates default [`Lua`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Lua<api::Cache> {
	/// Creates [`Lua`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Lua<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Lua<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Lua<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}


impl Lua<api::Default> {}


#[gen_api_shorthands::gen_shorthands]
impl<Api: api::Api> Lua<Api> {
	/// Adds the Lua function *f* to the Lua runtime, with name *name*. (*name*
	/// can be a table path using dots, e.g. if name = “mycode.myDrawingFunction”
	/// adds the function “myDrawingFunction” to the global table “myCode”.)
	///
	/// Equivalent to [`sys::ffi::playdate_lua::addFunction`]
	#[doc(alias = "sys::ffi::playdate_lua::addFunction")]
	pub fn add_function<S: AsRef<str>>(&self, f: lua_CFunction, name: S) -> Result<(), ApiError> {
		let name = CString::new(name.as_ref())?;
		let mut out_err: *const c_char = core::ptr::null_mut();

		let func = self.0.add_function();

		// Returns 1 on success or 0 with an error message in *outErr*.
		let result = unsafe { func(f, name.as_ptr(), &mut out_err) };

		if result == 0 {
			let err_msg = unsafe { CStr::from_ptr(out_err) };
			Err(Error::AddFunction(err_msg.to_owned()).into())
		} else {
			Ok(())
		}
	}

	/// Returns the argument at position *pos* as a string.
	///
	/// Equivalent to [`sys::ffi::playdate_lua::getArgString`]
	#[doc(alias = "sys::ffi::playdate_lua::getArgString")]
	pub fn get_arg_string(&self, pos: i32) -> Option<CString> {
		let f = self.0.get_arg_string();
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

pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_int;
	use core::ptr::NonNull;
	use sys::ffi::lua_CFunction;
	use sys::ffi::playdate_lua;


	/// Default lua api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached lua api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_lua);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(lua)) }
	}

	impl From<*const playdate_lua> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_lua) -> Self { Self(unsafe { ptr.as_ref() }.expect("lua")) }
	}

	impl From<&'static playdate_lua> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_lua) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_lua>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_lua>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_lua>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_lua>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}


	impl Api for Cache {
		#[inline(always)]
		fn add_function(
			&self)
			-> unsafe extern "C" fn(f: lua_CFunction, name: *const c_char, outErr: *mut *const c_char) -> c_int {
			self.0.addFunction.expect("addFunction")
		}

		#[inline(always)]
		fn get_arg_string(&self) -> unsafe extern "C" fn(pos: c_int) -> *const c_char {
			self.0.getArgString.expect("getArgString")
		}
	}


	pub trait Api {
		/// Returns [`sys::ffi::playdate_lua::addFunction`]
		#[doc(alias = "sys::ffi::playdate_lua::addFunction")]
		fn add_function(
			&self)
			-> unsafe extern "C" fn(f: lua_CFunction, name: *const c_char, outErr: *mut *const c_char) -> c_int {
			*sys::api!(lua.addFunction)
		}
		/// Returns [`sys::ffi::playdate_lua::getArgString`]
		#[doc(alias = "sys::ffi::playdate_lua::getArgString")]
		fn get_arg_string(&self) -> unsafe extern "C" fn(pos: c_int) -> *const c_char {
			*sys::api!(lua.getArgString)
		}
	}
}
