use core::fmt;


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
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Fs(err) => err.fmt(f),
			Error::Alloc => write!(f, "Allocation failed"),
			Error::InvalidMask => write!(f, "Mask must be the same size as the target bitmap"),
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
