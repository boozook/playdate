use core::ffi::c_char;
use alloc::borrow::Cow;
use sys::ffi::CStr;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug, Clone)]
#[must_use = "Error message doesn’t live long enough"]
pub enum Error {
	Response(Cow<'static, str>),

	/// Unknown error.
	/// Usually means invalid input or something not found.
	Unknown,
}

impl Error {
	pub fn from_ptr(ptr: *const c_char) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			let s = unsafe { CStr::from_ptr(ptr as _) }.to_string_lossy();
			Self::Response(s).into()
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::Response(s) => write!(f, "{s}"),
			Error::Unknown => write!(f, "Unknown"),
		}
	}
}

impl core::error::Error for Error {}

impl Into<ApiError> for Error {
	fn into(self) -> ApiError { ApiError::Api(self) }
}
