use core::any::Any;
use core::any::TypeId;
use core::marker::FnPtr;

use crate::util::ptr::{FnPtrId, FnLoc};
use crate::util::marker::IsFnPtr;


/// Derivative of the type, the sum of:
/// - Unique type id of fn-item,
/// - Static location of fn-ptr if it is, with type id of it.
///
/// Can be created for/from any safe rust-abi functions uniquely.
///
/// Also see [`FromFn`][] and [`FnKey`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
// TODO: May be better to use only second part of `TypeId` as it is in the std, instead of entire `TypeId`, or some short hash of it?
pub enum Key {
	/// Unique type of fn-item.
	Any(TypeId),
	/// Type of fn-ptr with pointer itself, so hopefully unique.
	/// Actually, it doesn't matter if it has been inlined
	/// - we just need to identify it to be able to associate one function with another.
	#[allow(private_interfaces)]
	Ptr(TypeId, FnLoc),
}

impl<T: AsKey> From<T> for Key {
	default fn from(v: T) -> Self { T::key(v) }
}

impl Key {
	pub(crate) const fn to_any(self) -> (Self, Option<FnLoc>) {
		match self {
			Key::Ptr(id, loc) => (Self::Any(id), Some(loc)),
			key => (key, None),
		}
	}

	pub(crate) const fn to_loc(self) -> Option<FnLoc> {
		match self {
			Key::Ptr(_, loc) => Some(loc),
			_ => None,
		}
	}

	#[inline]
	pub fn of_ptr<F: FnPtr + 'static>(f: &F) -> Self {
		let t = TypeId::of::<F>();
		let p = FnPtr::addr(*f);
		Self::Ptr(t, FnLoc::from_ptr(p))
	}
	unsafe fn of_ptr_cast<F: IsFnPtr + 'static>(f: &F) -> Self {
		let t = TypeId::of::<F>();
		let p = unsafe { *(f as *const F as *const *const ()) };
		Self::Ptr(t, FnLoc::from_ptr(p))
	}

	/// [`Any`](Key::Any) variant of the key for the type `F`.
	pub const fn of<F: 'static>() -> Self { Self::Any(TypeId::of::<F>()) }
	/// [`Any`](Key::Any) variant of the key for the type `F` given by `v`.
	pub const fn of_val<F: 'static>(_: &F) -> Self { Self::of::<F>() }
}


pub trait AsKey {
	// TODO: rename to "to key"
	fn key(self) -> Key;
}

// Spec for Any -> TypeId
impl<T: Any> AsKey for &T {
	#[inline(always)]
	default fn key(self) -> Key { Key::of_val(self) }
}

// Spec for FnPtrId
impl<T: Any> AsKey for &T where T: FnPtrId {
	fn key(self) -> Key {
		let t = TypeId::of::<T::Type>();
		let p = (*self).loc();
		Key::Ptr(t, p)
	}
}

// Spec for itself
impl AsKey for &Key {
	#[inline(always)]
	fn key(self) -> Key { *self }
}


#[cfg(test)]
mod tests {
	use std::collections::BTreeSet;

	use super::Key;
	use super::AsKey;


	struct T;
	impl T {
		fn a() {}
		fn b(&self) {}
		fn c(&self) {} // same sig, but not location
	}


	fn key(v: impl AsKey) -> Key { v.key() }


	/// coerced fn
	#[test]
	fn coerced() {
		let a: fn() = T::a;
		let b: fn(_) = T::b;
		let c: fn(_) = T::c;

		// need to test refs to fn
		let _b: &'static _ = to_static(&b);
		let __b: &'static &'static _ = to_static(&_b);
		fn to_static<T>(v: &'_ T) -> &'static T { unsafe { core::mem::transmute(v) } }


		let kf = key(&item);
		let kf_coerced = key(&(item as fn()));
		let kf_coerced_ = key(&&&(item as fn()));

		let ka = key(&a);
		let kb = key(&b);
		let kc = key(&c);


		assert_ne!(kf, kf_coerced);
		assert_eq!(kf_coerced, kf_coerced_);

		assert_ne!(kf, ka);
		assert_ne!(ka, kb);
		assert_ne!(ka, kc);
		assert!(matches!(ka, Key::Ptr(..)));
		assert!(matches!(kb, Key::Ptr(..)));
		assert!(matches!(kc, Key::Ptr(..)));

		// type behind a ref:
		assert_eq!(key(&b), key(_b));
		assert_eq!(key(_b), key(&_b));
		assert_eq!(key(_b), key(__b));
		assert_eq!(key(_b), key(&__b));
		assert_eq!(key(&_b), key(&__b));
	}


	/// fn-items
	#[test]
	fn item() {
		{
			let f = || {};
			let k = key(&f);
			assert!(matches!(k, Key::Any(..)));
		}
		{
			let mut v = vec![0];
			let f = move || v.push(0);
			let k = key(&f);
			assert!(matches!(k, Key::Any(..)));
		}
		{
			let v = vec![0];
			let f = move || drop(v);
			let k = key(&f);
			assert!(matches!(k, Key::Any(..)));
		}

		{
			let a = T::a.key();
			let b = T::b.key();
			let c = T::c.key();
			let c_ = (&T::c).key();
			assert_ne!(a, b);
			assert_ne!(b, c);
			assert_eq!(c, c_);

			assert!(matches!(a, Key::Any(..)));
			assert!(matches!(b, Key::Any(..)));
			assert!(matches!(c, Key::Any(..)));
			assert!(matches!(c_, Key::Any(..)));

			let mut m = BTreeSet::new();
			m.insert(a);
			m.insert(b);
			m.insert(c);
			m.insert(c_);
			assert_eq!(3, m.len());
		}
	}


	// other various things
	#[test]
	fn generic() {
		struct Foo;
		fn generic<T>() -> T { unreachable!() }

		let a: fn() = generic;
		let b: fn() -> u8 = generic;
		let c: fn() -> Foo = generic;

		{
			let _ = a.key();
			let _ = b.key();
			let _ = c.key();
			let _ = generic::<Foo>.key();
		}

		let a = a.key();
		let b = b.key();
		let c = c.key();

		let a_ = generic::<()>.key();
		let b_ = generic::<u8>.key();
		let c_ = generic::<Foo>.key();

		let t = self::generic.key();


		// unique/equality check:
		let all = [a, b, c, a_, b_, c_, t];
		for i in 0..all.len() {
			for j in i..all.len() {
				if i == j {
					assert_eq!(all[i], all[j]);
				} else {
					assert_ne!(all[i], all[j]);
				}
			}
		}


		assert!(matches!(a, Key::Ptr(..)));
		assert!(matches!(b, Key::Ptr(..)));
		assert!(matches!(c, Key::Ptr(..)));

		assert!(matches!(a_, Key::Any(..)));
		assert!(matches!(b_, Key::Any(..)));
		assert!(matches!(c_, Key::Any(..)));

		assert!(matches!(t, Key::Any(..)));
	}


	#[test]
	fn self_as_key() {
		let a = self_as_key.key();
		let b = a.key();
		let c = (&a).key();
		assert_eq!(a, b);
		assert_eq!(a, c);
		assert!(matches!(a, Key::Any(..)));


		let f: fn() = self_as_key;
		let a = f.key();
		let b = a.key();
		let c = (&a).key();
		let d = key(&c);
		assert_eq!(a, b);
		assert_eq!(a, c);
		assert_eq!(a, d);
		assert!(matches!(a, Key::Ptr(..)));
	}
}
