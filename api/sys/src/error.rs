use core::fmt;
pub use core::str::Utf8Error;


/// Represents null-ptr-error relative to API,
/// which means that API is not initialized.
///
/// Or that API is not available for the target platform, such as Simulator.
#[derive(Debug, Clone, Copy)]
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


/// Represents common null-ptr-error.
#[derive(Debug, Clone, Copy)]
pub struct NullPtrError;

impl core::error::Error for NullPtrError {
	// Overriden default impl of description.
	// Removed default text to do not store it in output binary.
	fn description(&self) -> &str { NULL_PTR }
}

impl fmt::Display for NullPtrError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { NULL_PTR.fmt(f) }
}

const NULL_PTR: &str = "NullPtr";

impl From<ApiError> for NullPtrError {
	#[inline(always)]
	fn from(_: ApiError) -> Self { Self }
}
