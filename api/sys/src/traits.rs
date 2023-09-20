//! Helpful traits for API parts unification.


pub trait AsRaw {
	type Type;
	/// This method ia actually safe.
	/// Unsafety is because so we're removing owners lifetime that used by some API parts.
	unsafe fn as_raw(&self) -> *mut Self::Type;
}

impl<T: AsRaw<Type = Ptr>, Ptr> AsRaw for &'_ T {
	type Type = Ptr;
	#[inline(always)]
	unsafe fn as_raw(&self) -> *mut Ptr { (*self).as_raw() }
}

impl<T> AsRaw for *mut T {
	type Type = T;
	#[inline(always)]
	unsafe fn as_raw(&self) -> *mut T { *self }
}
