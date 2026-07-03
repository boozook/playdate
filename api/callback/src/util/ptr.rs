use core::ffi::c_void;
use core::marker::FnPtr;


/// Unsafe pointer protecter. Contains pointer to the fn body.
/// We must not deref it, never.
/// But better way to deal with ptrs is as-is - not to cast to usize and miss meta & provenance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct FnLoc(*const ());

impl FnLoc {
	#[inline(always)]
	pub(crate) const fn into_ptr(self) -> *const () { self.0 }
	#[inline(always)]
	pub(crate) const fn as_ptr(&self) -> *const () { self.0 }
	#[inline(always)]
	pub(crate) const fn as_ptr_ref(&self) -> &*const () { &self.0 }
	#[inline(always)]
	pub(crate) const fn as_ptr_mut(&mut self) -> &mut *const () { &mut self.0 }
	#[inline(always)]
	pub(crate) const fn from_ptr(p: *const ()) -> Self { Self(p) }

	#[inline(always)]
	pub(crate) const fn cast_ref<F>(&self) -> &F { unsafe { core::mem::transmute(self.as_ptr_ref()) } }
	#[inline(always)]
	pub(crate) const fn cast_mut<F>(&mut self) -> &mut F { unsafe { core::mem::transmute(self.as_ptr_mut()) } }
}

impl From<*const ()> for FnLoc {
	fn from(p: *const ()) -> Self { Self(p) }
}
impl From<*mut c_void> for FnLoc {
	fn from(p: *mut c_void) -> Self { Self(p.cast()) }
}


macro_rules! impl_fn_loc {
	() => { impl_fn_loc!(@impl); };

	// Stopping criteria (1-ary tuple)
	($T:ident) => {
		impl_fn_loc!();
		impl_fn_loc!(@impl $T);
	};

	// Running criteria (n-ary tuple, with n >= 2)
	($T:ident $($U:ident)+) => {
		impl_fn_loc!($( $U )+);
		impl_fn_loc!(@impl $T $( $U )+);
	};

	(@impl $($T:ident)*) => {
		impl_fn_loc_ref!(<$($T)*> '_ '_ '_ '_);
		impl_fn_loc_ref!(<$($T)*>);
	};
}


macro_rules! impl_fn_loc_ref {
	(<$($A:ident)*> $($T:lifetime)?) => { impl_fn_loc_ref!(@impl<$($A)*>  $($T)?); };

	(<$($A:ident)*> $T:lifetime $($U:lifetime)+) => {
		impl_fn_loc_ref!(<$($A)*> $($U)+);
		impl_fn_loc_ref!(@impl<$($A)*> $T $($U)+);
	};

	(@impl$(<$($A:ident)*>)? $($T:lifetime)*) => {
		impl<$($($A,)*)? R> From< impl_fn_loc_ref!(@ref fn( $($($A),*)? )->R: $($T)*) > for FnLoc {
			#[inline(always)]
			fn from(f: impl_fn_loc_ref!(@ref fn( $($($A),*)? )->R: $($T)*) ) -> Self { Self(f.addr()) }
		}

		// This probably should be for any Fn(A,B,..)->R as fn(A,B,..)->R.
		impl<$($($A,)*)? R> FnPtrId for impl_fn_loc_ref!(@ref fn( $($($A),*)? )->R: $($T)*) {
			#[inline(always)]
			fn loc(&self) -> FnLoc { FnLoc::from(*self) }
			type Type = fn( $($($A),*)? )->R;
		}
	};

	// Util: make refs with lifetimes, e.g.: &'_[&'_[...]] T
	(@ref $ID:ty: $($L:lifetime)*) => { $(&$L)* $ID };
}

impl_fn_loc!(E D C B A Z Y X W V U T);


#[rustc_specialization_trait]
pub(crate) trait FnPtrId {
	type Type;
	fn loc(&self) -> FnLoc;
}


#[cfg(test)]
mod tests {
	use core::any::TypeId;
	use super::*;


	#[test]
	fn loc() {
		// prepare mock refs:
		let a: fn() = loc;
		let b: &'static _ = to_static(&a);
		let c: &'static &'static _ = to_static(&b);
		let d: &'static &'static &'static _ = to_static(&c);

		let a = fn_loc(&a);
		let b = fn_loc(&b);
		let c = fn_loc(&c);
		let d = fn_loc(&d);
		assert_eq!(a, b);
		assert_eq!(b, c);
		assert_eq!(c, d);
	}

	#[test]
	fn ty() {
		// prepare mock refs:
		let a: fn() = loc;
		let b: &'static _ = to_static(&a);
		let c: &'static &'static _ = to_static(&b);
		let d: &'static &'static &'static _ = to_static(&c);

		let ta = type_id_unref(a);
		let tb = type_id_unref(b);
		let tc = type_id_unref(c);
		let td = type_id_unref(d);

		let solved = [ta, tb, tc, td];
		for i in 0..solved.len() {
			for j in i..solved.len() {
				assert_eq!(solved[i], solved[j]);
			}
		}

		// must be equal
		assert_eq!(ta, type_id(a));

		// cmp solved & unsolved problem:
		assert_ne!(tb, type_id(b));
		assert_ne!(tc, type_id(c));
		assert_ne!(td, type_id(d));
		// ^ needed to trigger if it solved internally in the corelib.
	}


	fn fn_loc<T: FnPtrId>(v: &T) -> FnLoc { T::loc(v) }

	/// Get the `TypeId` of the given `T`.
	fn type_id<T: 'static>(_: T) -> TypeId { TypeId::of::<T>() }

	/// Get the `TypeId` of the `V` behind the reference where `T = &V`.
	fn type_id_unref<T: 'static + FnPtrId>(_: T) -> TypeId { TypeId::of::<T::Type>() }

	fn to_static<T>(v: &'_ T) -> &'static T { unsafe { core::mem::transmute(v) } }
}
