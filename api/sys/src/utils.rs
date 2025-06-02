use core::ops::Deref;
use core::ops::DerefMut;
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

impl<T: ~const Deref<Target = U>, U: ~const AsRaw> const AsRaw for T {
	type Output = <U as AsRaw>::Output;
	#[inline(always)]
	unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
}


// impl<T: ~const AsRaw> const AsRaw for &'_ T {
// 	type Output = T::Output;
// 	#[inline(always)]
// 	unsafe fn as_raw(&self) -> NonNull<Self::Output> { AsRaw::as_raw(*self) }
// }

// use core::mem::ManuallyDrop;
// impl<U: ~const AsRaw> const AsRaw for ManuallyDrop<U> where ManuallyDrop<U>: ~const Deref<Target = U> {
// 	type Output = <U as AsRaw>::Output;
// 	#[inline(always)]
// 	unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
// }


// #[cfg(feature = "alloc")]
// mod alloc {
// 	use ::alloc::boxed::Box;
// 	use ::alloc::rc::Rc;
// 	use super::*;


// 	impl<U: ~const AsRaw> const AsRaw for Box<U> where Box<U>: ~const Deref<Target = U> {
// 		type Output = <U as AsRaw>::Output;
// 		#[inline(always)]
// 		unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
// 	}

// 	impl<U: ~const AsRaw> const AsRaw for Rc<U> where Rc<U>: ~const Deref<Target = U> {
// 		type Output = <U as AsRaw>::Output;
// 		#[inline(always)]
// 		unsafe fn as_raw(&self) -> NonNull<Self::Output> { Deref::deref(self).as_raw() }
// 	}
// }


/// Call-On-Drop is a wrapper around a function that is called when the wrapper is dropped.
#[must_use]
pub struct Cod<T, F: FnOnce()>(T, Option<F>);

impl<T, F: FnOnce()> Cod<T, F> {
	pub const fn new(inner: T, f: F) -> Self { Self(inner, Some(f)) }
	pub fn cancel_cod(&mut self) { self.1.take(); }
	pub const fn cod_is_canceled(&self) -> bool { self.1.is_none() }
}

impl<T, F: FnOnce()> Drop for Cod<T, F> {
	fn drop(&mut self) { self.1.take().map(|f| f()); }
}


// impl<T: AsRaw, F: FnOnce()> AsRaw for Cod<T, F> {
// 	type Output = T::Output;
// 	unsafe fn as_raw(&self) -> NonNull<Self::Output> { T::as_raw(&self.0) }
// }

impl<T, F: FnOnce()> const Deref for Cod<T, F> {
	type Target = T;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T, F: FnOnce()> const DerefMut for Cod<T, F> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T, F: FnOnce()> AsRef<T> for Cod<T, F> {
	fn as_ref(&self) -> &T { &self.0 }
}
impl<T, F: FnOnce()> AsMut<T> for Cod<T, F> {
	fn as_mut(&mut self) -> &mut T { &mut self.0 }
}


#[const_trait]
pub trait IntoCod: Sized {
	fn into_cod(self, f: impl FnOnce()) -> Cod<Self, impl FnOnce()> { Cod::new(self, f) }
}
