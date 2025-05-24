use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr::NonNull;


#[const_trait]
pub trait AsRaw {
	type Output;
	/// Returns a raw pointer to the underlying data.
	///
	/// Unsafe because the returned pointer is only valid as long as the original
	/// and also could cause double-free in user' code.
	///
	/// The caller must ensure that the returned pointer is never mutated,
	/// and that the lifetime of the returned pointer is no longer than the lifetime of the original.
	unsafe fn as_raw(&self) -> NonNull<Self::Output>;
}

impl<T: ~const AsRaw> const AsRaw for &'_ T {
	type Output = T::Output;
	#[inline(always)]
	unsafe fn as_raw(&self) -> NonNull<Self::Output> { AsRaw::as_raw(*self) }
}

// impl<T: ~const Deref<Target = U>, U: ~const AsRaw> const AsRaw for T {
// 	type Output = <U as AsRaw>::Output;
// 	#[inline(always)]
// 	unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
// }

impl<U: ~const AsRaw> const AsRaw for ManuallyDrop<U> where ManuallyDrop<U>: ~const Deref<Target = U> {
	type Output = <U as AsRaw>::Output;
	#[inline(always)]
	unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
}


#[cfg(feature = "alloc")]
mod alloc {
	use ::alloc::boxed::Box;
	use ::alloc::rc::Rc;
	use super::*;


	impl<U: ~const AsRaw> const AsRaw for Box<U> where Box<U>: ~const Deref<Target = U> {
		type Output = <U as AsRaw>::Output;
		#[inline(always)]
		unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
	}

	impl<U: ~const AsRaw> const AsRaw for Rc<U> where Rc<U>: ~const Deref<Target = U> {
		type Output = <U as AsRaw>::Output;
		#[inline(always)]
		unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
	}
}
