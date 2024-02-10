#![cfg(not(test))]
//! Global Allocator implementation. Depends on `allocator` feature.


/*
With rust-lang/rust#102318 default_alloc_error_handler has been stabilized,
ie. the default error handler is enabled by default.
Therefore, it's no longer necessary to provide an alloc_error_handler
if the desired error handling is equivalent to a panic.
TODO: think about remove `alloc_error_handler`.
*/


extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;


// #[cfg_attr(feature = "allocator-global", global_allocator)]
#[global_allocator]
#[cfg(feature = "allocator")]
pub static GLOBAL: PlaydateAllocator = PlaydateAllocator;

/// Global handler for an Out Of Memory (OOM) condition
#[alloc_error_handler]
#[cfg(feature = "allocator")]
fn alloc_error(_layout: Layout) -> ! { panic!("Out of Memory") }


pub struct PlaydateAllocator;

unsafe impl GlobalAlloc for PlaydateAllocator {
	#[inline]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 { realloc(core::ptr::null_mut(), layout.size()) as *mut u8 }
	#[inline]
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) { realloc(ptr as *mut c_void, 0); }
	#[inline]
	unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
		realloc(ptr as *mut c_void, new_size) as *mut u8
	}
}


fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
	unsafe {
		if let Some(api) = crate::sys::API.as_ref() {
			if let Some(f) = (*api.system).realloc {
				return f(ptr, size);
			}
		}
	}
	panic!("realloc")
}
