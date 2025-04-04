use core::alloc::Allocator;
use core::alloc::AllocError;
use core::alloc::Layout;
use core::ptr::NonNull;
use core::ptr::null_mut;
use core::ptr::slice_from_raw_parts_mut;

use crate::global::dealloc;
use crate::global::realloc;
use crate::System;


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
		debug_assert!(crate::is_inited());

		let p = unsafe { realloc(null_mut(), layout.size()) };

		if p.is_null() {
			Err(AllocError)
		} else {
			let p = slice_from_raw_parts_mut(p.cast(), layout.size());
			Ok(unsafe { NonNull::new_unchecked(p) })
		}
	}

	/// Deallocates the memory referenced by `ptr`.
	///
	/// Note: ignores layout, just deallocates region which is internally associated with given ptr.
	///
	/// See more details on [`Allocator`](core::alloc::Allocator::deallocate).
	unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
		debug_assert!(crate::is_inited());

		unsafe { dealloc(ptr.as_ptr().cast()) };
	}


	// `allocate_zeroed` is default impl because Playdate's system allocator
	// as well as Symulator's returns NOT-zeroed memory.
	// alloc's default impl is fills zeroes after allocation.


	unsafe fn grow(&self,
	               ptr: NonNull<u8>,
	               old_layout: Layout,
	               new_layout: Layout)
	               -> Result<NonNull<[u8]>, AllocError> {
		debug_assert!(
		              new_layout.size() >= old_layout.size(),
		              "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
		);
		debug_assert!(crate::is_inited());

		let new_ptr: *mut u8 = realloc(ptr.as_ptr().cast(), new_layout.size()).cast();

		let new_ptr = if new_ptr.is_null() {
			return Err(AllocError);
		} else {
			let new_ptr = slice_from_raw_parts_mut(new_ptr, new_layout.size());
			unsafe { NonNull::new_unchecked(new_ptr) }
		};

		if ptr != new_ptr.as_non_null_ptr() {
			// SAFETY: because `new_layout.size()` must be greater than or equal to
			// `old_layout.size()`, both the old and new memory allocation are valid for reads and
			// writes for `old_layout.size()` bytes. Also, because the old allocation wasn't yet
			// deallocated, it cannot overlap `new_ptr`. Thus, the call to `copy_nonoverlapping` is
			// safe. The safety contract for `dealloc` must be upheld by the caller.
			unsafe {
				core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), old_layout.size());
				self.deallocate(ptr, old_layout);
			}
		}

		Ok(new_ptr)
	}

	unsafe fn grow_zeroed(&self,
	                      ptr: NonNull<u8>,
	                      old_layout: Layout,
	                      new_layout: Layout)
	                      -> Result<NonNull<[u8]>, AllocError> {
		debug_assert!(
		              new_layout.size() >= old_layout.size(),
		              "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
		);
		debug_assert!(crate::is_inited());

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
		debug_assert!(
		              new_layout.size() <= old_layout.size(),
		              "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
		);
		debug_assert!(crate::is_inited());

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

		Ok(new_ptr)
	}
}


#[cfg(test)]
#[cfg(not(feature = "global"))]
mod tests {
	#![allow(unexpected_cfgs)] // for `fake_alloc`, see explanation in crate::tests.

	use super::*;


	#[test]
	#[cfg_attr(feature = "static-link", ignore = "for static-mut only")]
	fn not_inited() {
		#[cfg(not(feature = "static-link"))]
		unsafe {
			crate::REALLOC = crate::fake as crate::Realloc;
		};

		assert!(!crate::is_inited());
	}

	#[test]
	#[cfg_attr(debug_assertions, should_panic)]
	#[cfg_attr(feature = "static-link", ignore = "for static-mut only")]
	fn allocate_not_inited() {
		#[cfg(not(feature = "static-link"))]
		unsafe {
			crate::REALLOC = crate::fake as crate::Realloc;
		};

		assert!(!crate::is_inited());

		let l = unsafe { Layout::from_size_align_unchecked(size_of::<usize>(), align_of::<usize>()) };
		assert!(System.allocate(l).is_err())
	}


	#[test]
	#[cfg_attr(not(fake_alloc), ignore = "set RUSTFLAGS='--cfg=fake_alloc' to enable.")]
	fn inited_fake() {
		crate::tests::init_fake();
		assert!(crate::is_inited());
	}

	#[test]
	#[cfg_attr(not(fake_alloc), ignore = "set RUSTFLAGS='--cfg=fake_alloc' to enable.")]
	fn allocate_fake() {
		crate::tests::init_fake();

		let l = unsafe { Layout::from_size_align_unchecked(size_of::<usize>(), align_of::<usize>()) };
		assert!(System.allocate(l).is_err())
	}
}
