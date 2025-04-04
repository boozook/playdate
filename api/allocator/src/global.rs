use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use crate::System;


/// Global Playdate [System] allocator.
#[global_allocator]
#[cfg(feature = "global")]
pub static GLOBAL: System = System;


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


/// Global handler for an Out Of Memory (OOM) condition
#[track_caller]
#[alloc_error_handler]
#[cfg(feature = "global-error-handler")]
fn alloc_error(layout: Layout) -> ! {
	type Error = unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...) -> !;
	unsafe extern "Rust" {
		#[link_name = "PDERR"]
		pub static ERROR: core::mem::MaybeUninit<Error>;
	}
	if unsafe { ERROR.as_ptr() }.is_null() {
		use core::intrinsics::is_val_statically_known;
		if is_val_statically_known(layout.size() != 0) {
			// const-known, so alloc-less
			panic!("OoM: {}b", layout.size())
		} else {
			// alloc-less panic
			panic!("OoM")
		}
	} else {
		use numtoa_like::*;
		// rendes size to ascii inplace:
		let mut s = [79, 111, 77, 58, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0];
		let mut index = s.len() - 3;
		render(layout.size(), &mut index, &mut s);
		s[5..].rotate_left(index.wrapping_sub(3));

		// alloc-less panic via pd-err
		unsafe {
			let f = ERROR.assume_init();
			f(s.as_ptr().cast())
		}
	}
}


#[cfg(feature = "global-error-handler")]
mod numtoa_like {
	// A lookup table to prevent the need for conditional branching
	// The value of the remainder of each step will be used as the index
	const LOOKUP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	// A lookup table optimized for decimal lookups. Each two indices represents one possible number.
	const DEC_LOOKUP: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                    2021222324252627282930313233343536373839\
                                    4041424344454647484950515253545556575859\
                                    6061626364656667686970717273747576777879\
                                    8081828384858687888990919293949596979899";

	/// Render `value` to string `buf`, left-aligning.
	/// Modifies `index` pointing to the start of actual string in `buf`.
	pub fn render(value: usize, index: &mut usize, buf: &mut [u8]) {
		let mut v = value;
		// Decode four characters at the same time
		while v > 9999 {
			let rem = (v % 10000) as u16;
			let (frst, scnd) = ((rem / 100) * 2, (rem % 100) * 2);
			buf[*index - 3..*index - 1].copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
			buf[*index - 1..*index + 1].copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
			*index = index.wrapping_sub(4);
			v /= 10000;
		}
		if v > 999 {
			let (frst, scnd) = ((v / 100) * 2, (v % 100) * 2);
			buf[*index - 3..*index - 1].copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
			buf[*index - 1..*index + 1].copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
			*index = index.wrapping_sub(4);
		} else if v > 99 {
			let section = (v as u16 / 10) * 2;
			buf[*index - 2..*index].copy_from_slice(&DEC_LOOKUP[section as usize..section as usize + 2]);
			buf[*index] = LOOKUP[(v % 10) as usize];
			*index = index.wrapping_sub(3);
		} else if v > 9 {
			v *= 2;
			buf[*index - 1..*index + 1].copy_from_slice(&DEC_LOOKUP[v as usize..v as usize + 2]);
			*index = index.wrapping_sub(2);
		} else {
			buf[*index] = LOOKUP[v as usize];
			*index = index.wrapping_sub(1);
		}
	}
}
