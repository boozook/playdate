use alloc::borrow::ToOwned;
use core::fmt;
use sys::ffi::CStr;
use sys::ffi::CString;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug)]
pub enum Error {
	AddFunction(CString),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::AddFunction(cs) => {
				match cs.to_str() {
					Ok(err) => err.fmt(f),
					Err(_) => f.write_fmt(format_args!("Add function error: {cs:?}")),
				}
			},
		}
	}
}


impl From<Error> for ApiError {
	fn from(err: Error) -> Self { ApiError::Api(err) }
}

impl From<&'_ CStr> for Error {
	fn from(cs: &CStr) -> Self { Self::AddFunction(cs.to_owned()) }
}


impl core::error::Error for Error {}
