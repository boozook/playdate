use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use crate::System;


/// Global Playdate [System] allocator.
#[global_allocator]
#[cfg(feature = "global")]
pub static GLOBAL: System = System;


/// Global handler for an Out Of Memory (OOM) condition
#[alloc_error_handler]
#[cfg(feature = "global-error-handler")]
fn alloc_error(layout: Layout) -> ! {
	// TODO: Here could be alloc-less panic.
	// But it can be implemented to use string-on-heap for formatting.
	// So should here be just call `playdate.sys.error`?
	panic!("OoM: {}b", layout.size()) // very short str is to minimize potential allocation anyway.
}


unsafe impl GlobalAlloc for System {
	#[inline]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 { realloc(core::ptr::null_mut(), layout.size()) as *mut u8 }
	#[inline]
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) { dealloc(ptr as *mut c_void); }
	#[inline]
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
		let res = realloc(ptr as *mut c_void, new_size) as *mut u8;

		// default mem-copy- behavior if new != old:
		if !res.is_null() && ptr != res {
			// SAFETY: the previously allocated block cannot overlap the newly allocated block.
			// The safety contract for `dealloc` must be upheld by the caller.
			unsafe {
				core::ptr::copy_nonoverlapping(ptr, res, core::cmp::min(layout.size(), new_size));
				self.dealloc(ptr, layout);
			}
		}

		res
	}

	// `alloc_zeroed` is default impl because Playdate's system allocator
	// as well as Symulator's returns NOT-zeroed memory.
	// alloc's default impl is fills zeroes after allocation.
}


#[track_caller]
#[inline(always)]
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void { crate::get()(ptr, size) }


#[track_caller]
#[inline(always)]
pub unsafe fn dealloc(ptr: *mut c_void) { realloc(ptr, 0); }
