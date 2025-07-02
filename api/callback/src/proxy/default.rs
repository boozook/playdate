use core::marker::PhantomData;
use core::marker::Tuple;

use crate::arg;
use super::Proxy;


pub struct Default<F, Storage, Adapter>(PhantomData<(F, Storage, Adapter)>);


macro_rules! impl_def {
		// Stopping criteria (1?-ary tuple)
		($T:ident) => {
			impl_def!(@impl $T);
			impl_def!(@impl);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($T:ident $( $U:ident )+) => {
			impl_def!($( $U )+);
			impl_def!(@impl $T $( $U )+);
		};

		(@impl $( $T:ident )*) => { ::pastey::paste!{
			#[allow(non_snake_case)]
			mod [<def _ $($T)*>] {
				use super::*;
				use core::any::type_name;
				use crate::util::macros::trace;


				impl<F, Storage, Adapter, $($T,)* CR, RArgs, RR>
					Proxy<extern "C" fn($($T,)*) -> CR, ($($T,)*), CR, F, RArgs, RR> for Default<F, Storage, Adapter>
					where RArgs: Tuple,
							RR: Into<CR>,
							Storage: $crate::storage::Store<F>,
							Adapter: arg::Adapter<Params = ($($T,)*), Args = RArgs>
				{
					#[inline(always)]
					default fn fn_fn() -> extern "C" fn($($T,)*) -> CR
						where F: Fn<RArgs, Output = RR> {
						trace!(get: (extern "C" fn($($T,)*) -> CR as ProxyFn => Self));
						proxy_fn::<$($T,)* CR, Storage, Adapter, F, RArgs, RR>
					}

					#[inline(always)]
					default fn fn_mut() -> extern "C" fn($($T,)*) -> CR
						where F: FnMut<RArgs, Output = RR> {
						trace!(get: (extern "C" fn($($T,)*) -> CR as ProxyMut => Self));
						proxy_mut::<$($T,)* CR, Storage, Adapter, F, RArgs, RR>
					}

					#[inline(always)]
					default fn fn_once() -> extern "C" fn($($T,)*) -> CR
						where F: FnOnce<RArgs, Output = RR> {
						trace!(get: (extern "C" fn($($T,)*) -> CR as ProxyOnce => Self));
						proxy_once::<$($T,)* CR, Storage, Adapter, F, RArgs, RR>
					}
				}


				extern "C" fn proxy_fn<$($T,)* R, S, Conv, F, Args, Ret>($([<$T:lower>]: $T),*) -> R
					where S: crate::storage::Store<F>,
							Conv: arg::Adapter<Params = ($($T,)*), Args = Args>,
							F: Fn<Args, Output = Ret>,
							Args: Tuple,
							Ret: Into<R> {
					if let Some(f) = S::get() {
						trace!(call: (F as Fn => S));
						f.call(Conv::convert(($([<$T:lower>],)*))).into()
					} else {
						panic!("missed callback: {}", type_name::<F>())
					}
				}

				extern "C" fn proxy_mut<$($T,)* R, S, Conv, F, Args, Ret>($([<$T:lower>]: $T),*) -> R
					where S: crate::storage::Store<F>,
							Conv: arg::Adapter<Params = ($($T,)*), Args = Args>,
							F: FnMut<Args, Output = Ret>,
							Args: Tuple,
							Ret: Into<R> {
					if let Some(f) = S::get_mut() {
						trace!(call: (F as FnMut => S));
						f.call_mut(Conv::convert(($([<$T:lower>],)*))).into()
					} else {
						panic!("missed callback: {}", type_name::<F>())
					}
				}

				extern "C" fn proxy_once<$($T,)* R, S, Conv, F, Args, Ret>($([<$T:lower>]: $T),*) -> R
					where S: crate::storage::Store<F>,
							Conv: arg::Adapter<Params = ($($T,)*), Args = Args>,
							F: FnOnce<Args, Output = Ret>,
							Args: Tuple,
							Ret: Into<R> {
					if let Some(f) = S::take() {
						trace!(call: (F as FnOnce => S));
						f.call_once(Conv::convert(($([<$T:lower>],)*))).into()
					} else {
						panic!("missed callback: {}", type_name::<F>())
					}
				}
			}
		}};
	}

impl_def!(E D C B A Z Y X W V U T);
