use core::ffi::c_int;
use core::ffi::c_uint;
use sys::ffi::CStr;
use sys::error::OkOrNullFnErr;
use core::fmt;
use alloc::string::String;
use alloc::string::ToString;

use crate::Fs;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug)]
pub enum Error {
	Fs(String),
	Unknown,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Fs(s) => write!(f, "Fs: {s}"),
			Error::Unknown => write!(f, "Fs: unknown error"),
		}
	}
}


impl Into<ApiError> for Error {
	fn into(self) -> ApiError { ApiError::Api(self) }
}


impl From<String> for Error {
	fn from(s: String) -> Self { Self::Fs(s) }
}

impl From<&'_ str> for Error {
	fn from(s: &str) -> Self { Self::Fs(s.to_string()) }
}


impl core::error::Error for Error {}


impl Error {
	pub fn latest() -> Result<Option<Self>, ApiError> {
		let f = sys::api_ok!(file.geterr)?;
		let ptr = unsafe { f() };
		if ptr.is_null() {
			Ok(None)
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.to_str()
			                                   .map_err(Into::into)
			                                   .map(Self::from)
			                                   .map(Into::into)
		}
	}

	pub fn latest_using(fs: &Fs) -> Result<Option<Self>, ApiError> {
		let f = fs.0.geterr.ok_or_null()?;
		let ptr = unsafe { f() };
		if ptr.is_null() {
			Ok(None)
		} else {
			unsafe { CStr::from_ptr(ptr as _) }.to_str()
			                                   .map_err(Into::into)
			                                   .map(Self::from)
			                                   .map(Into::into)
		}
	}

	#[inline(always)]
	pub fn ok_from_code(result: c_int) -> Result<c_uint, ApiError> {
		if result < 0 {
			let err = Self::latest()?.unwrap_or(Self::Unknown);
			Err(err.into())
		} else {
			Ok(result as c_uint)
		}
	}

	#[inline(always)]
	pub fn ok_from_code_using(result: c_int, fs: &Fs) -> Result<c_uint, ApiError> {
		if result < 0 {
			let err = Self::latest_using(fs)?.unwrap_or(Self::Unknown);
			Err(err.into())
		} else {
			Ok(result as c_uint)
		}
	}
}
