use alloc::borrow::ToOwned;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::fmt;
use sys::ffi::CStr;
use sys::ffi::CString;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug)]
pub enum Error {
	Fs(CString),
	Unknown,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Fs(cs) => {
				match cs.to_str() {
					Ok(err) => err.fmt(f),
					Err(_) => f.write_fmt(format_args!("Fs: {cs:?}")),
				}
			},
			Error::Unknown => write!(f, "Fs: unknown error"),
		}
	}
}


impl From<Error> for ApiError {
	fn from(err: Error) -> Self { ApiError::Api(err) }
}

impl From<&'_ CStr> for Error {
	fn from(cs: &CStr) -> Self { Self::Fs(cs.to_owned()) }
}


impl core::error::Error for Error {}


impl Error {
	/// Equivalent to [`sys::ffi::playdate_file::geterr`]
	#[doc(alias = "sys::ffi::playdate_file::geterr")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn latest() -> Option<Self> {
		use crate::api::Api;
		let f = crate::api::Default.geterr();
		let ptr = unsafe { f() };
		Self::from_ptr(ptr)
	}

	/// Equivalent to [`sys::ffi::playdate_file::geterr`]
	#[doc(alias = "sys::ffi::playdate_file::geterr")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn latest_with<Api: crate::api::Api>(api: Api) -> Option<Self> {
		let f = api.geterr();
		let ptr = unsafe { f() };
		Self::from_ptr(ptr)
	}

	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn from_ptr(ptr: *const core::ffi::c_char) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Self::from(unsafe { CStr::from_ptr(ptr as _) }).into()
		}
	}

	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	#[inline(always)]
	pub fn ok_from_code(result: c_int) -> Result<c_uint, Self> {
		if result < 0 {
			Err(Self::latest().unwrap_or(Self::Unknown))
		} else {
			Ok(result as c_uint)
		}
	}

	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	#[inline(always)]
	pub fn ok_from_code_with<Api: crate::api::Api>(result: c_int, api: Api) -> Result<c_uint, Self> {
		if result < 0 {
			Err(Self::latest_with(api).unwrap_or(Self::Unknown))
		} else {
			Ok(result as c_uint)
		}
	}
}
