use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use core::any::Any;

use crate::storage::key::Key;
use crate::util::ptr::FnLoc;
use super::Store;


/// Global storage for static values.
/// Based on a [`BTreeMap`].
///
/// Allocates heap for each value.
///
/// _Accepts only `Fn` and `fn`-ptrs - any functions without context._
///
/// Totally unsafe for multithreading and any async.
///
/// Type-checking for the stored value is only for debug at rt, if `debug_assertions` is turned on.
///
/// ⚠️ Requires unique key for each function, so unique usage of a stored function.
/// Be really careful with coerced functions because of "type as key" is not unique for various functions with same signature.
/// For example this storage can contain only one function with `fn() -> ()` signature.
pub struct Static;

impl Static {
	// ... impl Store<T> + use<'static, T>
	pub const fn for_key<T>(_: &T) -> impl Store<T> + 'static
		where Self: Store<T> {
		Self
	}
}


// TODO: use anymap-like (https://github.com/azriel91/anymap2/blob/master/src/lib.rs)
// with custom hasher: https://github.com/droundy/internment/blob/master/src/container.rs#L53-L79
type Map = BTreeMap<Key, Box<dyn Any>>;

type LocMap = BTreeMap<Key, FnLoc>;


// #[inline(never)]
// #[cfg(not(test))]
// fn store_mut() -> &'static mut Map {
// 	static mut STORE: Map = Map::new();
// 	#[allow(static_mut_refs)]
// 	unsafe {
// 		&mut STORE
// 	}
// }

// #[inline(never)]
// #[cfg(test)]
// fn store_mut() -> &'static mut Map {
// 	#[thread_local]
// 	static mut STORE: Map = Map::new();
// 	// LOCAL.with(|x: &String| unsafe { &*(x as *const String) })

// 	#[allow(static_mut_refs)]
// 	unsafe {
// 		&mut STORE
// 	}
// }

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


mod r#static {

	use super::*;


	impl<T: 'static> Store<T> for Static {
		default fn is_empty() -> bool {
			let s = store();
			s.is_empty() || !s.contains_key(&Key::of::<T>())
		}

		/// Insert value replacing the old one.
		default fn set(v: T) {
			let key = Key::of::<T>();
			crate::util::macros::trace!("stored FnItem: {}", core::any::type_name::<T>());
			store_mut().insert(key, Box::new(v));
		}


		default fn get<'t>() -> Option<&'t T> {
			let key = Key::of::<T>();
			store().get(&key).map(|v: &Box<dyn Any>| {
				                 // Sanity check
				                 debug_assert!(v.as_ref().is::<T>());
				                 let ptr = (v.as_ref() as *const dyn Any).cast::<T>();
				                 unsafe { &*ptr }
			                 })
		}
		default fn get_mut<'t>() -> Option<&'t mut T> {
			let key = Key::of::<T>();
			store_mut().get_mut(&key).map(|v: &mut Box<dyn Any>| {
				                         // Sanity check
				                         debug_assert!(v.as_ref().is::<T>());
				                         let ptr = (v.as_mut() as *mut dyn Any).cast::<T>();
				                         unsafe { &mut *ptr }
			                         })
		}
		default fn take() -> Option<T> {
			let key = Key::of::<T>();
			store_mut().remove(&key).map(|v: Box<dyn Any>| {
				                        crate::util::macros::trace!(
				                                                    "removed FnItem: {}",
				                                                    core::any::type_name::<T>()
				);
				                        // Sanity check
				                        debug_assert!(v.as_ref().is::<T>());
				                        let ptr = Box::into_raw(v).cast::<T>();
				                        unsafe { *Box::from_raw(ptr) }
			                        })
		}
	}


	/// Spec for static fn-ptrs.
	mod coerced {
		use super::*;


		#[inline(never)]
		fn store_loc_mut() -> &'static mut LocMap {
			#[cfg_attr(test, thread_local)]
			static mut STORE: LocMap = LocMap::new();
			#[allow(static_mut_refs)]
			unsafe {
				&mut STORE
			}
		}
		#[inline(always)]
		fn store_loc() -> &'static LocMap { self::store_loc_mut() }


		/// Gen spec for static fn-ptrs, e.g.:
		/// ```text,ignore
		/// impl<R> Store<fn() -> R> for Static where R: 'static {
		/// 	fn is_empty() -> bool {
		/// 		let s = store_loc();
		/// 		s.is_empty() || !s.contains_key(&Key::of::<fn() -> R>())
		/// 	}
		///
		/// 	fn set(v: fn() -> R) {
		/// 		let (key, Some(loc)) = Key::of_ptr(&v).to_any() else {
		/// 			unreachable!()
		/// 		};
		/// 		crate::util::macros::trace!("stored fn: {}", core::any::type_name::<fn() -> R>());
		/// 		store_loc_mut().insert(key, loc);
		/// 	}
		///
		/// 	fn get<'t>() -> Option<&'t fn() -> R> {
		/// 		store_loc().get(&Key::of::<fn() -> R>()).map(|loc| loc.cast_ref())
		/// 	}
		///
		/// 	fn get_mut<'t>() -> Option<&'t mut fn() -> R> {
		/// 		store_loc_mut().get_mut(&Key::of::<fn() -> R>())
		/// 		               .map(|loc| loc.cast_mut())
		/// 	}
		///
		/// 	fn take() -> Option<fn() -> R>
		/// 		where fn() -> R: Sized {
		/// 		store_loc_mut().remove(&Key::of::<fn() -> R>())
		/// 		               .map(|loc| {
		/// 								crate::util::macros::trace!("removed fn: {}", core::any::type_name::<fn() -> R>());
		/// 								*(loc.cast_ref())
		/// 							})
		/// 	}
		/// }
		/// ```
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
				impl<$($T,)* R> Store<fn($($T,)*) -> R> for Static
					where R: 'static,
						$( $T: 'static),*
				{
					fn is_empty() -> bool {
						let s = store_loc();
						s.is_empty() || !s.contains_key(&Key::of::<fn($($T,)*) -> R>())
					}

					fn set(v: fn($($T,)*) -> R) {
						let (key, Some(loc)) = Key::of_ptr(&v).to_any() else {
							unreachable!()
						};
						crate::util::macros::trace!("stored fn: {}", core::any::type_name::<fn($($T,)*) -> R>());
						store_loc_mut().insert(key, loc);
					}

					fn get<'t>() -> Option<&'t fn($($T,)*) -> R> {
						store_loc().get(&Key::of::<fn($($T,)*) -> R>()).map(|loc| loc.cast_ref())
					}

					fn get_mut<'t>() -> Option<&'t mut fn($($T,)*) -> R> {
						store_loc_mut().get_mut(&Key::of::<fn($($T,)*) -> R>())
											.map(|loc| loc.cast_mut())
					}

					fn take() -> Option<fn($($T,)*) -> R>
						where fn($($T,)*) -> R: Sized {
						store_loc_mut().remove(&Key::of::<fn($($T,)*) -> R>())
											.map(|loc| {
												crate::util::macros::trace!("removed fn: {}", core::any::type_name::<fn($($T,)*) -> R>());
												*(loc.cast_ref())
											})
					}
				}


				#[cfg(test)]
				#[allow(non_snake_case)]
				mod [<tests_ $($T)*>] {
					use core::ptr::fn_addr_eq;
					use crate::storage::ext::StoreExt as _;
					use super::Static as S;

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


	#[cfg(test)]
	mod tests {
		use super::Static as S;
		use crate::storage::ext::StoreExt as _;


		#[test]
		fn empty() {
			use super::Store;

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
		#[cfg_attr(miri, ignore = "false-positive leak")]
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
				assert!(sb.is_empty());
				sb.write(b);
				assert!(!sb.is_empty());
				assert_eq!(42, sb.get().unwrap()());
				assert!(!sb.is_empty());
			}

			sa.take();
			sb.take();
		}
	}
}


mod associated {
	use super::*;
	use crate::storage::associate::Associated;


	impl<K: 'static> Associated<K> for Static {
		fn is_empty(k: &K) -> bool {
			let s = store();
			s.is_empty() || !s.contains_key(&Self::key(k))
		}

		fn is_empty_for<V: 'static>(k: &K) -> bool {
			let s = store();
			s.is_empty() ||
			store().get(&Self::key(k))
			       .filter(|v: &&Box<dyn Any>| v.as_ref().is::<V>())
			       .is_none()
		}


		fn set<V: 'static>(k: &K, v: V) {
			let key = Self::key(k);
			store_mut().insert(key, Box::new(v));
		}

		fn get<'t, V: 'static>(k: &K) -> Option<&'t V> {
			let key = Self::key(k);
			store().get(&key).map(|v: &Box<dyn Any>| {
				                 // Sanity check
				                 debug_assert!(v.as_ref().is::<V>());
				                 let ptr = (v.as_ref() as *const dyn Any).cast::<V>();
				                 unsafe { &*ptr }
			                 })
		}

		fn get_mut<'t, V: 'static>(k: &K) -> Option<&'t mut V> {
			let key = Self::key(k);
			store_mut().get_mut(&key).map(|v: &mut Box<dyn Any>| {
				                         // Sanity check
				                         debug_assert!(v.as_ref().is::<V>());
				                         let ptr = (v.as_mut() as *mut dyn Any).cast::<V>();
				                         unsafe { &mut *ptr }
			                         })
		}

		fn take<V>(k: &K) -> Option<V>
			where V: Sized + 'static {
			let key = Self::key(k);
			store_mut().remove(&key).map(|v: Box<dyn Any>| {
				                        // Sanity check
				                        debug_assert!(v.as_ref().is::<V>());
				                        let ptr = Box::into_raw(v).cast::<V>();
				                        unsafe { *Box::from_raw(ptr) }
			                        })
		}
	}


	#[cfg(test)]
	mod tests {
		use crate::storage::key::AsKey;
		use super::{Static, Associated};


		#[test]
		#[cfg_attr(miri, ignore = "false-positive leak")]
		fn empty() {
			// for fn-item
			assert!(Static::is_empty(&empty));
			assert!(matches!(Static::get(&empty), None::<&()>));
			assert!(matches!(Static::get_mut(&empty), None::<&mut ()>));
			assert!(matches!(Static::take(&empty), None::<()>));

			Static::set(&empty, 42);
			assert!(!Static::is_empty(&empty));
			let v = Static::get(&empty);
			assert_eq!(Some(&42), v);


			// for coerced (must be empty now)
			let f: fn() = empty;
			assert!(Static::is_empty(&f));

			Static::set(&f, 101);
			assert!(!Static::is_empty(&f));
			let v = Static::get(&f);
			assert_eq!(Some(&101), v);
		}

		#[test]
		#[cfg_attr(debug_assertions, should_panic)]
		fn wrong() {
			Static::set(&wrong, 42);
			Static::take::<(/* not i32 */)>(&wrong);
		}

		#[test]
		#[cfg_attr(miri, ignore = "false-positive leak")]
		fn take() {
			Static::set(&take, 42);
			assert!(!Static::is_empty(&take));
			Static::take::<i32>(&take);
			assert!(Static::is_empty(&take));
		}


		#[test]
		fn key() {
			let key = key.key();
			let add = |v: u8| v + Static::get(&key).unwrap_or(&0);

			Static::set(&key, 42_u8);
			assert_eq!(43, add(1));

			Static::get_mut::<u8>(&key).map(|v| *v += 1);
			assert_eq!(44, add(1));

			Static::take::<u8>(&key);
			assert_eq!(1, add(1));
		}
	}
}
