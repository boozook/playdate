use core::fmt;
use sys::ffi::{CString, CStr};
use crate::alloc::borrow::ToOwned;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug)]
pub enum Error {
	/// Causes when loading graphics from path fails.
	/// This occurs when file does not exist or invalid format.
	Fs(fs::error::Error),

	/// Causes when allocation failed and/or null-ptr returned.
	Alloc,

	/// Mask must be the same size as the target bitmap.
	InvalidMask,

	/// Font error.
	/// This occurs when char or page not found.
	Font,

	/// Video error.
	Video(CString),

	/// Unknown error.
	Unknown,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Fs(err) => err.fmt(f),
			Error::Alloc => write!(f, "Allocation failed"),
			Error::Font => write!(f, "Font error"),
			Error::InvalidMask => write!(f, "Mask must be the same size as the target bitmap"),
			Error::Video(cs) => {
				match cs.to_str() {
					Ok(err) => err.fmt(f),
					Err(_) => f.write_fmt(format_args!("Video error: {cs:?}")),
				}
			},
			Error::Unknown => write!(f, "Unknown error"),
		}
	}
}

impl From<fs::error::Error> for Error {
	fn from(err: fs::error::Error) -> Self { Error::Fs(err) }
}


impl Into<ApiError> for Error {
	fn into(self) -> ApiError { ApiError::Api(self) }
}


impl core::error::Error for Error {}


impl Error {
	pub(crate) fn video_from(c: &CStr) -> Self { Self::Video(c.to_owned()) }
}
