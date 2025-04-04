#![no_std]
#![cfg_attr(feature = "allocator-api", feature(allocator_api, slice_ptr_get))]
#![cfg_attr(feature = "global-error-handler",
            feature(alloc_error_handler, core_intrinsics),
            allow(internal_features))]
#![cfg_attr(any(test, debug_assertions, not(feature = "static-link")),
            feature(fn_ptr_trait))]


extern crate alloc;


#[cfg(feature = "allocator-api")]
pub(crate) mod local;
pub(crate) mod global;


/// PlaydateOs system allocator.
///
/// Allocator uses system `realloc` c-fn,
/// so user have to call [`init`] if `static-link` feature is disabled.
///
/// Otherwise if `static-link` is on, it's statically linked and it's nesessary to call [`init`].
pub struct System;


type Realloc = unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void;


use core::ffi::c_void;

/// Fn-pointer to the OS's realloc function.
#[cfg(not(feature = "static-link"))]
static mut REALLOC: Realloc = fake;


/// Fake no-op realloc function, used as initial value (instead of any kinds of null) of [`REALLOC`].
/// Using this function will cause allocation failures.
#[cold]
#[cfg(not(feature = "static-link"))]
unsafe extern "C" fn fake(_: *mut c_void, _: usize) -> *mut c_void { core::ptr::null_mut() }


unsafe extern "C" {
	/// Statically linked OS's realloc function.
	#[cfg(feature = "static-link")]
	fn pdrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}


#[inline(always)]
#[cfg(debug_assertions)]
pub fn init(realloc: Realloc) {
	use core::marker::FnPtr;

	debug_assert!(!realloc.addr().is_null());
	init_realloc(realloc)
}


#[inline(always)]
#[cfg(not(debug_assertions))]
pub const fn init(realloc: Realloc) { init_realloc(realloc) }


#[inline(always)]
#[cfg_attr(feature = "static-link",
           doc = "\n no-op because [`realloc`] is linked statically")]
const fn init_realloc(#[cfg_attr(feature = "static-link", allow(unused_variables))] realloc: Realloc) {
	#[cfg(not(feature = "static-link"))]
	unsafe {
		REALLOC = realloc
	}
}

#[inline(always)]
#[cfg(feature = "static-link")]
pub const fn is_inited() -> bool { true }


#[cfg(not(feature = "static-link"))]
pub fn is_inited() -> bool {
	use core::ptr::fn_addr_eq;
	unsafe { !fn_addr_eq(REALLOC, fake as Realloc) }
}


#[inline(always)]
#[cfg(debug_assertions)]
fn get() -> Realloc {
	let realloc = get_unchecked();

	#[cfg(not(feature = "static-link"))]
	debug_assert!(!core::marker::FnPtr::addr(realloc).is_null(), "missed realloc");

	realloc
}

#[inline(always)]
#[cfg(not(debug_assertions))]
const fn get() -> Realloc { get_unchecked() }


#[inline(always)]
const fn get_unchecked() -> Realloc {
	#[cfg(feature = "static-link")]
	{
		pdrealloc
	}
	#[cfg(not(feature = "static-link"))]
	{
		unsafe { REALLOC }
	}
}


#[cfg(test)]
#[cfg(not(feature = "global"))]
mod tests {
	#![allow(unexpected_cfgs)] // for `fake_alloc`.
	// It could be properly registered by adding to build-script `println!("cargo::rustc-check-cfg=cfg(fake_alloc)")`,
	// but it's only needed for tests and should not used in production,
	// so could be great if compiler warn about it in places other than tests.

	use core::ptr::null_mut;
	use super::*;


	#[test]
	#[cfg_attr(feature = "static-link", ignore = "for static-mut only")]
	fn not_inited() {
		#[cfg(not(feature = "static-link"))]
		unsafe {
			REALLOC = fake
		}
		assert!(!is_inited());
	}

	#[test]
	fn inited() {
		#[cfg(not(feature = "static-link"))]
		{
			unsafe { REALLOC = fake }
			assert!(!is_inited());
		}

		init_fake();

		assert!(is_inited());

		#[cfg(feature = "static-link")]
		assert!(!core::marker::FnPtr::addr(pdrealloc as Realloc).is_null());
	}


	#[test]
	#[cfg_attr(not(fake_alloc), ignore = "set RUSTFLAGS='--cfg=fake_alloc' to enable.")]
	fn get_alloc_fake() {
		init_fake();

		let realloc = get();
		let p = unsafe { realloc(null_mut(), 64) };

		assert!(p.is_null());
	}


	pub(crate) fn init_fake() {
		#[cfg(not(feature = "static-link"))]
		{
			// another fake, mem-location is different from crate::fake
			unsafe extern "C" fn fake(_: *mut c_void, _: usize) -> *mut c_void { null_mut() }
			unsafe { REALLOC = fake }
		}
	}


	#[no_mangle]
	#[cfg(fake_alloc)]
	#[cfg(feature = "static-link")]
	extern "C" fn pdrealloc(_: *mut c_void, _: usize) -> *mut c_void { null_mut() }
}
