use core::fmt;
use core::ffi::*;
use alloc::borrow::ToOwned;
use sys::ffi::{CStr, CString};

use crate::Api;


/// Owned FS Error.
pub type Owned = Error<CString>;
/// Borrowed FS Error, with message borrowed from C part.
pub type Borrowed<'t> = Error<&'t CStr>;


#[derive(Debug)]
#[must_use = "Error message is borrowed from C part, must be used immediately or converted into an owned."]
pub enum Error<T: AsRef<CStr>> {
	Reason(T),
	Unknown,
}

impl From<Borrowed<'_>> for Owned {
	fn from(err: Borrowed) -> Self { err.into_owned() }
}
impl<'t> From<&'t Owned> for Borrowed<'t> {
	fn from(err: &'t Owned) -> Self {
		match err {
			Error::Reason(err) => Error::Reason(err.as_c_str()),
			Error::Unknown => Error::Unknown,
		}
	}
}


impl Borrowed<'_> {
	pub fn into_owned(self) -> Owned {
		match self {
			Self::Reason(err) => Error::Reason(err.to_owned()),
			Self::Unknown => Error::Unknown,
		}
	}
}


impl<T: fmt::Debug + AsRef<CStr>> core::error::Error for Error<T> {}

impl<T: fmt::Debug + AsRef<CStr>> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Reason(cs) => {
				match cs.as_ref().to_str() {
					Ok(err) => err.fmt(f),
					Err(_) => f.write_fmt(format_args!("{cs:?}")),
				}
			},
			Self::Unknown => write!(f, "unknown fs err"),
		}
	}
}

impl<'t: 'op, 'op> From<&'t CStr> for Borrowed<'op> {
	fn from(err: &'t CStr) -> Self { Self::Reason(err) }
}

impl From<&'_ CStr> for Owned {
	fn from(err: &'_ CStr) -> Self { Self::Reason(err.to_owned()) }
}

impl<T: AsRef<CStr>> Error<T> where for<'t> Self: From<&'t CStr> {
	pub unsafe fn from_ptr(ptr: *const core::ffi::c_char) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Some(Self::from(unsafe { CStr::from_ptr(ptr.cast()) }))
		}
	}
}
impl<'t: 'op, 'op, T: AsRef<CStr>> Error<T> where Self: 'op + From<&'t CStr> {
	#[inline(always)]
	pub(crate) fn from_code(code: c_int, api: Api) -> Result<c_uint, Self> {
		if code < 0 {
			let err = latest(api).map(Self::from).unwrap_or(Self::Unknown);
			Err(err)
		} else {
			Ok(code as c_uint)
		}
	}


	/// Reads [`sys::ffi::PlaydateFile::geterr`]
	#[doc(alias = "sys::ffi::PlaydateFile::geterr")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub(crate) fn latest(api: Api) -> Self { latest(api).map(Self::from).unwrap_or(Self::Unknown) }
}


/// Reads [`sys::ffi::PlaydateFile::geterr`]
#[doc(alias = "sys::ffi::PlaydateFile::geterr")]
#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
fn latest<'t>(api: Api) -> Option<&'t CStr> {
	let ptr = unsafe { (api.geterr)() };
	if ptr.is_null() {
		None
	} else {
		Some(unsafe { CStr::from_ptr(ptr.cast()) })
	}
}


/// Breaks program execution with error given by system.
/// Used in drop impls.
///
/// Does not reads error, just checks if code is negative and ptr is not null.
#[track_caller]
pub(crate) fn err_code_on_drop(code: c_int, api: Api) {
	const ERR: &CStr = c"on-drop";
	if code < 0 {
		let ptr = unsafe { (api.geterr)() };
		let error = api!(system.error);
		if ptr.is_null() {
			unsafe { error(ERR.as_ptr().cast()) }
		} else {
			unsafe { error(c"%s: %s".as_ptr().cast(), ERR.as_ptr(), ptr) }
		}
	}
}


/// Owned FS + FromUtf8 errors.
#[derive(Debug)]
pub enum ReadUtf8Error {
	Fs(Owned),
	Utf8(alloc::string::FromUtf8Error),
}

impl core::error::Error for ReadUtf8Error {}


impl fmt::Display for ReadUtf8Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Fs(err) => err.fmt(f),
			Self::Utf8(err) => err.fmt(f),
		}
	}
}


impl From<Owned> for ReadUtf8Error {
	fn from(err: Owned) -> Self { Self::Fs(err) }
}

impl From<Borrowed<'_>> for ReadUtf8Error {
	fn from(err: Borrowed) -> Self { Self::Fs(err.into_owned()) }
}

impl From<alloc::string::FromUtf8Error> for ReadUtf8Error {
	fn from(err: alloc::string::FromUtf8Error) -> Self { Self::Utf8(err) }
}
