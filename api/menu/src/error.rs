use core::fmt;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug)]
pub enum Error {
	/// Causes when allocation failed and/or null-ptr returned.
	Alloc,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Alloc => write!(f, "Menu: Allocation failed"),
		}
	}
}


impl Into<ApiError> for Error {
	fn into(self) -> ApiError { ApiError::Api(self) }
}


impl core::error::Error for Error {}
