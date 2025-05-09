use core::ffi::c_void;
use core::marker::PhantomData;


#[rustc_unsafe_specialization_marker]
pub(crate) trait IsFnPtr {}
impl<T: core::marker::FnPtr> IsFnPtr for T {}


#[repr(transparent)]
pub struct Ud<T>(pub(crate) *mut T);
impl<T> From<*mut T> for Ud<T> {
	fn from(p: *mut T) -> Self { Self(p) }
}
impl<T> Ud<T> {
	pub const fn into_ptr(self) -> *mut T { self.0 }
	pub const fn as_udptr(&self) -> UdPtr { self.0.cast() }
}


impl<'t, T> Into<Option<&'t mut T>> for Ud<T> {
	fn into(self) -> Option<&'t mut T> { unsafe { self.as_udptr().cast::<T>().as_mut() } }
}
impl<'t, T> Into<Option<&'t T>> for Ud<T> {
	fn into(self) -> Option<&'t T> { unsafe { self.as_udptr().cast::<T>().as_ref() } }
}

// impl<'t, T, U> Into<Option<&'t mut U>> for Ud<(T, U)> {
// 	fn into(self) -> Option<&'t mut U> { unsafe { self.into_ptr().as_mut().map(|(_, u)| u) } }
// }
// impl<'t, T, U> Into<Option<&'t U>> for Ud<(T, U)> {
// 	fn into(self) -> Option<&'t U> { unsafe { self.into_ptr().as_ref().map(|(_, u)| u) } }
// }


#[repr(transparent)]
pub(crate) struct FnWithUd<F, T>(pub(crate) *mut (F, T), PhantomData<(F, T)>);
impl<F, T> From<*mut (F, T)> for FnWithUd<F, T> {
	fn from(p: *mut (F, T)) -> Self { Self(p, PhantomData) }
}
impl<F, T> FnWithUd<F, T> {
	pub const fn into_ptr(self) -> *mut (F, T) { self.0 }
	pub const fn as_udptr(&self) -> UdPtr { self.0.cast() }
}


pub type UdPtr = *mut c_void;

pub struct UdFn<F, T, const I: u8>(pub F, pub Ud<T>);


pub(crate) struct FnWith<F, T>(pub(crate) F, pub(crate) T);


/// Gererates convertion from `UdFn<safe c-fn>` to `UdFn<unsafe c-fn>`, e.g.:
/// ```text
/// 	type Safe<A, R> = extern "C" fn(A) -> R;
/// 	type Unsafe<A, R> = unsafe extern "C" fn(A) -> R;

/// 	impl<A, R, T, const I: u8> Into<UdFn<Unsafe<A, R>, T, I>> for UdFn<Safe<A, R>, T, I> {
/// 		fn into(self) -> UdFn<Unsafe<A, R>, T, I> {
/// 			let Self(f, ud) = self;
/// 			UdFn(f, ud)
/// 		}
/// 	}
/// ```
macro_rules! impl_ud {
	// Stopping criteria (1?-ary tuple)
	($T:ident) => {
		impl_ud!(@impl $T);
		impl_ud!(@impl);
	};

	// Running criteria (n-ary tuple, with n >= 2)
	($T:ident $( $U:ident )+) => {
		impl_ud!($( $U )+);
		impl_ud!(@impl $T $( $U )+);
	};

	(@impl $( $T:ident )*) => { ::pastey::paste!{
		#[allow(non_snake_case)]
		mod [< ud _ $($T)* >] {
			use super::*;
			type Safe<$($T,)* R> = extern "C" fn($($T),*) -> R;
			type Unsafe<$($T,)* R> = unsafe extern "C" fn($($T),*) -> R;

			impl<$($T,)* R, UD, const I: u8> Into<UdFn<Unsafe<$($T,)* R>, UD, I>> for UdFn<Safe<$($T,)* R>, UD, I> {
				fn into(self) -> UdFn<Unsafe<$($T,)* R>, UD, I> {
					let Self(f, ud) = self;
					UdFn(f, ud)
				}
			}
		}
	}};
}

impl_ud!(K H E D C B A Z Y X W V U T);
