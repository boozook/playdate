//! Process API. Abort and abort with error message.

/// Executes the undefined instruction (UDF) and causes a CPU-level exception.
/// See [`core::intrinsics::abort()`].
#[track_caller]
#[inline(always)]
pub fn abort() -> ! { core::intrinsics::abort() }


/// Stops the program execution with custom system-level error.
///
/// In case of missed [`crate::sys::API`] (doesn't set) uses [`abort`].
#[track_caller]
pub fn error<S: AsRef<str>>(text: S) -> ! {
	if let Some(f) = unsafe { (*(*crate::sys::API).system).error } {
		// Casting fn->void to fn->!
		// This ugly cast is safe because of `!` is a magic compile-time marker, not a type.
		let f: unsafe extern "C" fn(*const i8, ...) = f;
		let p = core::ptr::addr_of!(f) as *const _ as *const unsafe extern "C" fn(*const i8, ...) -> !;
		let f = unsafe { p.as_ref() }.unwrap();

		if let Ok(s) = alloc::ffi::CString::new(text.as_ref()) {
			unsafe { f(s.as_ptr() as *mut core::ffi::c_char) }
		} else {
			unsafe { f(text.as_ref().as_ptr() as *mut core::ffi::c_char) }
		}
	} else {
		// In case of `crate::sys::API` is missed (doesn't set) just abort the process.
		abort()
	}
	// This is unreachable or the device,
	// `API.system.error` interrupts the execution.
	// But simulator doesn't stops on `error`.
}
