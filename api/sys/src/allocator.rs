//! Global Allocator implementation. Depends on `allocator` feature.

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use crate::macros::trace::trace_alloc;


/// PlaydateOs system allocator.
///
/// Uses system `realloc` c-fn.
pub struct System;


/// Global Playdate [System] allocator.
#[cfg(not(test))]
#[global_allocator]
#[cfg(feature = "allocator")]
pub static GLOBAL: System = System;


unsafe impl GlobalAlloc for System {
	#[inline]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		trace_alloc!(global::alloc size=layout.size());

		#[cfg(not(miri))]
		{
			(unsafe { realloc(core::ptr::null_mut(), layout.size()) }) as *mut u8
		}
		#[cfg(miri)]
		{
			let ptr = miri_alloc(layout.size(), layout.align().max(size_of::<usize>()));
			if !ptr.is_null() {
				miri_pointer_name(ptr.cast(), 0, c"global".to_bytes());
			}
			ptr
		}
	}


	#[inline]
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		trace_alloc!(global::dealloc ptr=ptr, size=layout.size());

		unsafe { dealloc(ptr, layout) };
	}


	#[inline]
	unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
		trace_alloc!(global::realloc ptr=ptr, size=_layout.size(), size=new_size);

		let res = unsafe { realloc(ptr as *mut c_void, new_size) } as *mut u8;

		// default mem-copy- behavior if new != old:
		if !res.is_null() && ptr != res {
			#[cfg(miri)]
			{
				miri_pointer_name(res.cast(), 0, c"global".to_bytes());
			}

			// NOTE: In this case PdOs's system allocator returns new memory
			// with already copied data from old memory,
			// and tail is not-zeroed.
			// So, we don't need to copy anything, e.g. copy_nonoverlapping(old, new).
			// Also, we don't need to deallocate the old memory.
		} // otherwise if new == old => so this is normal re-allocation, grow.

		res
	}

	// `alloc_zeroed` is default impl because Playdate's system allocator
	// as well as Symulator's returns NOT-zeroed memory.
	// alloc's default impl is fills zeroes after allocation.
}


#[cfg(feature = "allocator-api")]
mod local {
	use core::alloc::AllocError;
	use core::alloc::Allocator;
	use core::alloc::Layout;
	use core::ptr::null_mut;
	use core::ptr::slice_from_raw_parts_mut;
	use core::ptr::NonNull;
	use super::{System, realloc, dealloc, trace_alloc};


	unsafe impl Allocator for System {
		/// Attempts to allocate a block of memory.
		///
		/// Returns a new `NonNull<[u8]>` if the allocation was successful.
		/// The returned block of memory is not zeroed.
		///
		/// # Errors
		///
		/// Returning `Err` indicates that either memory is exhausted or `layout` does
		/// not meet allocator's size or alignment constraints,
		/// as well as [`init`](crate::init) was not called.
		///
		/// See more details on [`Allocator`](core::alloc::Allocator::allocate).
		fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
			trace_alloc!(local::allocate size=layout.size());

			let ptr = unsafe { realloc(null_mut(), layout.size()) };

			if ptr.is_null() {
				Err(AllocError)
			} else {
				#[cfg(miri)]
				unsafe {
					super::miri_pointer_name(ptr.cast(), 0, c"system".to_bytes())
				}
				let ptr = slice_from_raw_parts_mut(ptr.cast(), layout.size());
				Ok(unsafe { NonNull::new_unchecked(ptr) })
			}
		}

		/// Deallocates the memory referenced by `ptr`.
		///
		/// Note: ignores layout, just deallocates region which is internally associated with given ptr.
		///
		/// See more details on [`Allocator`](core::alloc::Allocator::deallocate).
		unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
			trace_alloc!(local::deallocate ptr=ptr.as_ptr(), size=layout.size());

			dealloc(ptr.as_ptr().cast(), layout)
		}


		// `allocate_zeroed` is default impl because Playdate's system allocator
		// as well as Symulator's returns NOT-zeroed memory.
		// alloc's default impl is fills zeroes after allocation.


		unsafe fn grow(&self,
		               ptr: NonNull<u8>,
		               old_layout: Layout,
		               new_layout: Layout)
		               -> Result<NonNull<[u8]>, AllocError> {
			trace_alloc!(local::grow ptr=ptr, size=old_layout.size(), size=new_layout.size());
			debug_assert!(
			              new_layout.size() >= old_layout.size(),
			              "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
			);

			let new_ptr: *mut u8 = realloc(ptr.as_ptr().cast(), new_layout.size()).cast();

			let new_ptr = if new_ptr.is_null() {
				return Err(AllocError);
			} else {
				let new_ptr = slice_from_raw_parts_mut(new_ptr, new_layout.size());
				unsafe { NonNull::new_unchecked(new_ptr) }
			};

			if ptr != new_ptr.as_non_null_ptr() {
				// NOTE: In this case PdOs's system allocator returns new memory
				// with already copied data from old memory,
				// and tail is not-zeroed.
				// So, we don't need to copy anything, e.g. copy_nonoverlapping(old, new).
				// Also, we don't need to deallocate the old memory.
				// unsafe {
				// 	core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), old_layout.size());
				// 	self.deallocate(ptr, old_layout);
				// }
			}

			#[cfg(miri)]
			unsafe {
				super::miri_pointer_name(new_ptr.as_ptr().cast(), 0, c"system".to_bytes())
			}

			Ok(new_ptr)
		}

		unsafe fn grow_zeroed(&self,
		                      ptr: NonNull<u8>,
		                      old_layout: Layout,
		                      new_layout: Layout)
		                      -> Result<NonNull<[u8]>, AllocError> {
			trace_alloc!(local::grow_zeroed ptr=ptr, size=old_layout.size(), size=new_layout.size());
			debug_assert!(
			              new_layout.size() >= old_layout.size(),
			              "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
			);

			let new_ptr = self.grow(ptr, old_layout, new_layout)?;

			// zeroing new mem only:
			let size = new_layout.size() - old_layout.size();
			let ext = ptr.add(old_layout.size());
			unsafe { ext.as_ptr().write_bytes(0, size) }

			Ok(new_ptr)
		}

		unsafe fn shrink(&self,
		                 ptr: NonNull<u8>,
		                 old_layout: Layout,
		                 new_layout: Layout)
		                 -> Result<NonNull<[u8]>, AllocError> {
			trace_alloc!(local::shrink ptr=ptr, size=old_layout.size(), size=new_layout.size());
			debug_assert!(
			              new_layout.size() <= old_layout.size(),
			              "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
			);

			// trying to shrink using realloc
			let new_ptr: *mut u8 = realloc(ptr.as_ptr().cast(), new_layout.size()).cast();

			let new_ptr = if new_ptr.is_null() {
				// re-allocate and copy to new location
				let new_ptr = self.allocate(new_layout)?;
				// SAFETY: because `new_layout.size()` must be lower than or equal to
				// `old_layout.size()`, both the old and new memory allocation are valid for reads and
				// writes for `new_layout.size()` bytes. Also, because the old allocation wasn't yet
				// deallocated, it cannot overlap `new_ptr`. Thus, the call to `copy_nonoverlapping` is
				// safe. The safety contract for `dealloc` must be upheld by the caller.
				unsafe {
					core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), new_layout.size());
					self.deallocate(ptr, old_layout);
				}
				new_ptr
			} else {
				let new_ptr = slice_from_raw_parts_mut(new_ptr, new_layout.size());
				unsafe { NonNull::new_unchecked(new_ptr) }
			};

			#[cfg(miri)]
			unsafe {
				super::miri_pointer_name(new_ptr.as_ptr().cast(), 0, c"system".to_bytes())
			}

			Ok(new_ptr)
		}
	}
}


#[track_caller]
#[inline(always)]
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
	if let Some(api) = crate::api() {
		trace_alloc!(realloc ptr=ptr, size=size);
		unsafe { (api.system.realloc)(ptr, size) }
	} else {
		#[cfg(debug_assertions)]
		{
			panic!("missed api.realloc");
		}
		#[cfg(not(debug_assertions))]
		{
			core::ptr::null_mut()
		}
	}
}


#[track_caller]
#[cfg(not(miri))]
#[inline(always)]
unsafe fn dealloc(ptr: *mut u8, _: Layout) { unsafe { realloc(ptr.cast(), 0) }; }

#[cfg(miri)]
#[track_caller]
#[inline(always)]
unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
	miri_dealloc(ptr, layout.size(), layout.align().max(size_of::<usize>()));
}


/*
	With rust-lang/rust#102318 default_alloc_error_handler has been stabilized,
	and the default error handler is enabled by default.
	Nevertheless, a "panic" without allocation is necessary here.
*/

/// Global handler for an Out Of Memory (OOM) condition
#[track_caller]
#[cfg(not(test))]
#[alloc_error_handler]
#[cfg(feature = "allocator")]
fn alloc_error(layout: Layout) -> ! {
	if let Some(api) = crate::api() {
		use crate::print::fmt::*;

		// rendes size to ascii inplace:
		let mut s = [79, 111, 77, 58, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0]; // "OoM: {u32::MAX}b"
		let mut index = s.len() - 3;
		n2ascii(layout.size(), &mut index, &mut s);
		s[5..].rotate_left(index.wrapping_sub(3));

		// alloc-less panic via pd-err
		unsafe { (api.system.error)(s.as_ptr().cast()) };
	} else {
		use core::intrinsics::is_val_statically_known;
		if is_val_statically_known(layout.size() != 0) {
			// const-known, so alloc-less
			panic!("OoM: {}b", layout.size())
		} else {
			// alloc-less const-known panic
			panic!("OoM")
		}
	}
}


#[cfg(miri)]
extern "Rust" {
	/// Miri-provided extern function to allocate memory from the interpreter.
	///
	/// This is useful when no fundamental way of allocating memory is
	/// available, e.g. when using `no_std` + `alloc`.
	fn miri_alloc(size: usize, align: usize) -> *mut u8;

	/// Miri-provided extern function to deallocate memory.
	fn miri_dealloc(ptr: *mut u8, size: usize, align: usize);

	/// Miri-provided extern function to associate a name to the nth parent of a tag.
	/// Typically the name given would be the name of the program variable that holds the pointer.
	/// Unreachable tags can still be named by using nonzero `nth_parent` and a child tag.
	///
	/// This function does nothing under Stacked Borrows, since Stacked Borrows's implementation
	/// of `miri_print_borrow_state` does not show the names.
	///
	/// Under Tree Borrows, the names also appear in error messages.
	pub fn miri_pointer_name(ptr: *const (), nth_parent: u8, name: &[u8]);
}
