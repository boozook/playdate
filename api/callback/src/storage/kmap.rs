use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::any::Any;

use crate::util::macros::trace;
use crate::storage::key::Key;
use crate::util::ptr::FnLoc;
use super::Store;


/// Global storage for __one__ unique type per Key, where Key is a "subscription key".
///
/// Same as [`Static`](super::tmap::Static) but with extra Key.
///
/// Type-checking for the stored value is only for debug at rt, if `debug_assertions` is turned on.
pub struct Storage<Key>(PhantomData<Key>);

impl<K: 'static> Storage<K> {
	pub const fn for_key<T>(_: &T) -> impl Store<T> + 'static
		where Self: Store<T> {
		Self(PhantomData::<K>)
	}
}


mod default {
	use super::*;


	// TODO: use anymap-like (https://github.com/azriel91/anymap2/blob/master/src/lib.rs)
	// with custom hasher: https://github.com/droundy/internment/blob/master/src/container.rs#L53-L79
	type Map = BTreeMap<Key, Box<dyn Any>>;


	#[inline(never)]
	fn store_mut() -> &'static mut Map {
		#[cfg_attr(test, thread_local)]
		static mut STORE: Map = Map::new();
		#[allow(static_mut_refs)]
		unsafe {
			&mut STORE
		}
	}

	#[inline(always)]
	fn store() -> &'static Map { self::store_mut() }


	impl<K: 'static, V: 'static> Store<V> for Storage<K> {
		default fn is_empty() -> bool {
			let s = store();
			s.is_empty() || !s.contains_key(&Key::of::<K>())
		}

		/// Insert value replacing the old one.
		default fn set(v: V) {
			let key = Key::of::<K>();
			let prev = store_mut().insert(key, Box::new(v));
			if prev.is_some() {
				trace!(re-add: (V as FnItem => Self));
			} else {
				trace!(add: (V as FnItem => Self));
			}
		}


		default fn get<'t>() -> Option<&'t V> {
			let key = Key::of::<K>();
			store().get(&key).map(|v: &Box<dyn Any>| {
				                 trace!(get: (&V as FnItem => Self));
				                 // Sanity check
				                 debug_assert!(v.as_ref().is::<V>());
				                 let ptr = (v.as_ref() as *const dyn Any).cast::<V>();
				                 unsafe { &*ptr }
			                 })
		}
		default fn get_mut<'t>() -> Option<&'t mut V> {
			let key = Key::of::<K>();
			store_mut().get_mut(&key).map(|v: &mut Box<dyn Any>| {
				                         trace!(get: (&mut V as FnItem => Self));
				                         // Sanity check
				                         debug_assert!(v.as_ref().is::<V>());
				                         let ptr = (v.as_mut() as *mut dyn Any).cast::<V>();
				                         unsafe { &mut *ptr }
			                         })
		}
		default fn take() -> Option<V> {
			let key = Key::of::<K>();
			store_mut().remove(&key).map(|v: Box<dyn Any>| {
				                        trace!(rem: (V as FnItem => Self));
				                        // Sanity check
				                        debug_assert!(v.as_ref().is::<V>());
				                        let ptr = Box::into_raw(v).cast::<V>();
				                        unsafe { *Box::from_raw(ptr) }
			                        })
		}

		default fn remove() -> bool {
			store_mut().remove(&Key::of::<K>())
			           .inspect(|_| trace!(rem: (V as Any => Self)))
			           .is_some()
		}
	}


	#[cfg(test)]
	mod tests {
		use super::Storage;
		use crate::storage::ext::StoreExt as _;


		struct Key;
		type S = Storage<Key>;


		#[test]
		fn empty() {
			use super::Store;

			struct Key;
			type S = Storage<Key>;

			type F = fn();
			assert!(<S as Store<F>>::is_empty());
			assert!(matches!(<S as Store<F>>::get(), None));
			assert!(matches!(<S as Store<F>>::get_mut(), None));
			assert!(matches!(<S as Store<F>>::take(), None));
		}

		#[test]
		fn r#static() {
			fn add(v: u8) -> u8 { v + 1 }
			let store = S::for_key(&add);

			assert!(store.is_empty());
			store.write(add);
			assert!(!store.is_empty());
			assert_eq!(2, store.get().unwrap()(1));

			assert!(!store.is_empty());
			assert!(store.take().is_some());
			assert!(store.is_empty());
		}

		#[test]
		fn closure() {
			let add = |v: u8| v + 1;
			let store = S::for_key(&add);

			assert!(store.is_empty());
			store.write(add);
			assert!(!store.is_empty());
			assert_eq!(2, store.get().unwrap()(1));

			assert!(!store.is_empty());
			assert!(store.take().is_some());
			assert!(store.is_empty());
		}

		#[test]
		#[cfg_attr(miri, ignore = "leak false-positive?")]
		fn various() {
			let a = |v: u8| v + 1;
			let b = || 42;

			let sa = S::for_key(&a);
			let sb = S::for_key(&b);

			assert!(sa.is_empty());
			sa.write(a);
			assert!(!sa.is_empty());
			assert_eq!(2, sa.get().unwrap()(1));
			assert!(!sa.is_empty());

			{
				assert!(!sb.is_empty()); // yup
				sb.write(b);
				assert!(!sb.is_empty());
				assert_eq!(42, sb.get().unwrap()());
				assert!(!sb.is_empty());
			}

			assert!(sb.take().is_some());
			assert!(sa.take().is_none());
		}
	}
}


/// Spec for static fn-ptrs.
mod coerced {
	use super::*;


	type Map = BTreeMap<Key, FnLoc>;


	#[inline(never)]
	fn store_mut() -> &'static mut Map {
		#[cfg_attr(test, thread_local)]
		static mut STORE: Map = Map::new();
		#[allow(static_mut_refs)]
		unsafe {
			&mut STORE
		}
	}
	#[inline(always)]
	fn store() -> &'static Map { self::store_mut() }


	/// spec for static fn-ptrs
	macro_rules! impl_spec {
		// Stopping criteria (1?-ary tuple)
		($T:ident) => {
			impl_spec!(@impl $T);
			impl_spec!(@impl);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($T:ident $( $U:ident )+) => {
			impl_spec!($( $U )+);
			impl_spec!(@impl $T $( $U )+);
		};

		(@impl $( $T:ident )*) => { ::pastey::paste!{
			impl<K, $($T,)* R> Store<fn($($T,)*) -> R> for Storage<K>
				where R: 'static,
						K: 'static,
					$( $T: 'static),*
			{
				fn is_empty() -> bool {
					let s = store();
					s.is_empty() || !s.contains_key(&Key::of::<K>())
				}

				fn set(v: fn($($T,)*) -> R) {
					let Some(loc) = Key::of_ptr(&v).to_loc() else {
						unreachable!()
					};
					let _prev = store_mut().insert(Key::of::<K>(), loc);

					if _prev.is_some() {
						trace!(re-add: (fn($($T,)*) -> R as FnPtr => Self));
					} else {
						trace!(add: (fn($($T,)*) -> R as FnPtr => Self));
					}
				}

				fn get<'t>() -> Option<&'t fn($($T,)*) -> R> {
					store().get(&Key::of::<K>()).map(|loc| {
						trace!(get: (&fn($($T,)*) -> R as FnPtr => Self));
						loc.cast_ref()
					})
				}

				fn get_mut<'t>() -> Option<&'t mut fn($($T,)*) -> R> {
					store_mut().get_mut(&Key::of::<K>())
						        .map(|loc| {
								       trace!(get: (&mut fn($($T,)*) -> R as FnPtr => Self));
								       loc.cast_mut()
							     })
				}

				fn take() -> Option<fn($($T,)*) -> R>
					where fn($($T,)*) -> R: Sized {
					store_mut().remove(&Key::of::<K>())
						        .map(|loc| {
								       trace!(rem: (fn($($T,)*) -> R as FnPtr => Self));
								       *(loc.cast_ref())
							     })
				}

				fn remove() -> bool {
					store_mut().remove(&Key::of::<K>())
						.inspect(|_| trace!(rem: (fn($($T,)*) -> R as FnPtr => Self)))
						.is_some()
				}
			}


			#[cfg(test)]
			#[allow(non_snake_case)]
			mod [<tests_ $($T)*>] {
				use core::ptr::fn_addr_eq;
				use crate::storage::ext::StoreExt as _;
				use super::Storage;

				struct Key;
				type S = Storage<Key>;

				$(type $T = ();)*
				type TEST = fn($($T),*) -> u8;


				#[test]
				#[cfg_attr(miri, ignore = "false-positive leak")]
				/// Should not be stored as fn-ptr
				fn fn_item() {
					fn test($(_: $T),*) -> u8 { 42 }

					// storage for fn-ptr
					let coerced = S::for_key(&(test as TEST));
					coerced.take();
					assert!(coerced.is_empty());

					// store as fn-item
					let store = S::for_key(&test);
					store.write(test);
					assert!(!store.is_empty());

					// check that is not stored as fn-ptr
					assert!(coerced.is_empty());

					// check that stored what we stored
					let f = store.take().unwrap();
					#[cfg(not(miri))] // false-positive
					assert!(fn_addr_eq::<fn($($T),*) -> _, fn($($T),*) -> _>(f, test as _)); // coercion needed to compate ptrs
					assert_eq!(42, f($(<$T as Default>::default()),*));
				}

				#[test]
				#[cfg_attr(miri, ignore = "false-positive")]
				fn coerced() {
					fn test($(_: $T),*) -> u8 { 101 }

					let store = S::for_key(&(test as TEST));
					store.write(test);
					assert!(!store.is_empty());
					let f = store.take().unwrap();
					assert!(fn_addr_eq(f, test as TEST));
					assert_eq!(101, f($(<$T as Default>::default()),*));
				}
			}
		}};
	}

	impl_spec!(E D C B A Z Y X W V U T);
}
