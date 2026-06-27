use core::fmt;
use sys::ffi::CString;


#[derive(Debug)]
pub struct AddFunctionError(pub(crate) CString);

impl core::error::Error for AddFunctionError {}

impl fmt::Display for AddFunctionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("Add function error: {:?}", self.0))
	}
}
