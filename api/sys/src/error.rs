use core::fmt;
pub use core::str::Utf8Error;


/// Represents null-ptr-error relative to API,
/// which means that API is not initialized.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ApiError;

impl core::error::Error for ApiError {
	// Overriden default impl of description.
	// Removed default text to do not store it in output binary.
	fn description(&self) -> &str { NO_API }
}

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { NO_API.fmt(f) }
}


const NO_API: &str = "ApiErr";
