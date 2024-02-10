//! Process API. Abort and abort with error message.

/// Executes the undefined instruction (UDF) and causes a CPU-level exception.
/// See [`core::intrinsics::abort()`]
pub fn abort() -> ! { core::intrinsics::abort() }


/// Stops the program execution with custom system-level error.
#[track_caller]
pub fn error<S: AsRef<str>>(text: S) -> ! {
	if let Some(f) = unsafe { (*(*crate::sys::API).system).error } {
		if let Ok(s) = alloc::ffi::CString::new(text.as_ref()) {
			unsafe { f(s.as_ptr() as *mut core::ffi::c_char) }
		} else {
			unsafe { f(text.as_ref().as_ptr() as *mut core::ffi::c_char) }
		}
	}
	// Next line is mostly unreachable,
	// but in some cases such as some versions of simulator doesn't stops on `error`,
	// especially in case of `crate::sys::API` isn't filled.
	abort()
}
