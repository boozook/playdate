//! `println` function.

use core::ffi::c_char;
use alloc::ffi::CString;


#[track_caller]
pub fn println<S: AsRef<str>>(text: S) {
	unsafe {
		let f = (*(*crate::sys::API).system).logToConsole.unwrap();
		if let Ok(s) = CString::new(text.as_ref()) {
			f(s.as_ptr() as *mut c_char);
		} else {
			f(text.as_ref().as_ptr() as *mut c_char);
		}
	}
}
