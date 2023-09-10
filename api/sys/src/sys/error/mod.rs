use core::any::Any;
use core::convert::Infallible;
use core::fmt;
use core::error::Error as CoreError;

use crate::ffi::Utf8Error;
use alloc::ffi::NulError;
use alloc::string::FromUtf8Error;
pub use null::*;
mod null;


#[derive(Debug)]
pub enum Error<T = ()> {
	Api(T),
	Utf8(Utf8Error),
	FromUtf8(FromUtf8Error),
	CStr(NulError),
	NullPtr(null::NullPtrError),
	#[cfg(feature = "error-ctx")]
	NullPtrCtx(null::ctx::NullPtrError),
}

// impl<T: CoreError> fmt::Display for Error<T> {
impl<T: fmt::Display> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Api(err) => err.fmt(f),
			Error::Utf8(err) => err.fmt(f),
			Error::FromUtf8(err) => err.fmt(f),
			Error::CStr(err) => err.fmt(f),
			Error::NullPtr(err) => err.fmt(f),
			#[cfg(feature = "error-ctx")]
			Error::NullPtrCtx(err) => err.fmt(f),
		}
	}
}

impl<T> Error<T> {
	pub fn from_err<E>(error: Error<E>) -> Self
		where T: From<E> {
		match error {
			Error::Api(err) => Error::Api(err.into()),
			Error::Utf8(err) => Error::Utf8(err),
			Error::FromUtf8(err) => Error::FromUtf8(err),
			Error::CStr(err) => Error::CStr(err),
			Error::NullPtr(err) => Error::NullPtr(err),
			#[cfg(feature = "error-ctx")]
			Error::NullPtrCtx(err) => Error::NullPtrCtx(err),
		}
	}
}


impl<T> From<Utf8Error> for Error<T> {
	fn from(error: Utf8Error) -> Self { Self::Utf8(error) }
}

impl<T> From<FromUtf8Error> for Error<T> {
	fn from(error: FromUtf8Error) -> Self { Self::FromUtf8(error) }
}

impl<T> From<null::NullPtrError> for Error<T> {
	fn from(error: null::NullPtrError) -> Self { Self::NullPtr(error) }
}

impl<T> From<NulError> for Error<T> {
	fn from(error: NulError) -> Self { Self::CStr(error) }
}

impl<T> From<Infallible> for Error<T> {
	fn from(_: Infallible) -> Self { unreachable!() }
}

#[cfg(feature = "error-ctx")]
impl<T> From<null::ctx::NullPtrError> for Error<T> {
	fn from(error: null::ctx::NullPtrError) -> Self { Self::NullPtrCtx(error) }
}


impl<T: Any> CoreError for Error<T> where T: fmt::Display + fmt::Debug {
	fn source(&self) -> Option<&(dyn CoreError + 'static)> {
		match self {
			Error::Utf8(err) => Some(err),
			Error::FromUtf8(err) => Some(err),
			Error::CStr(err) => Some(err),
			Error::NullPtr(err) => Some(err),
			#[cfg(feature = "error-ctx")]
			Error::NullPtrCtx(err) => Some(err),
			Error::Api(err) => (err as &dyn Any).downcast_ref::<&dyn CoreError>().copied(),
		}
	}

	// Removed text from str to do not store this in output binary.
	/// `description()` is deprecated; use `Display`
	fn description(&self) -> &str { "" }
}
