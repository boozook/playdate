use core::marker::Tuple;

use crate::util::marker::{Ud, UdPtr};
use crate::arg;
use super::default::Default;
use super::Proxy;


/// Impl last marked argument as `UD<F>`
macro_rules! impl_ud {
		// Stopping criteria (1?-ary tuple)
		($T:ident) => {
			impl_ud!(@impl $T);
			impl_ud!(@impl ... $T);
			impl_ud!(@impl);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($T:ident $( $U:ident )+) => {
			impl_ud!($( $U )+);
			impl_ud!(@impl $T $( $U )+);
			impl_ud!(@impl $T ... $( $U )+);
			impl_ud!(@impl ... $T $( $U )+);
		};

		(@impl $( $T:ident )*  $( ... $($REST:ident)* )?) => { ::pastey::paste!{
			#[allow(non_snake_case)]
			mod [< ud _ $($T)* $(_ $($REST)*)?>] {
				use super::*;
				use core::any::type_name;
				use crate::util::macros::trace;


				// spec for Ud with just F
				impl<F, Storage, Adapter, $($T,)* $($($REST,)*)? CR, RArgs, RR>
					Proxy<extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<F>, $($($REST),*)?), CR, F, RArgs, RR> for Default<F, Storage, Adapter>
					where RArgs: Tuple,
							RR: Into<CR>,
							Adapter: arg::Adapter<Params = ($($T,)* Ud<F>, $($($REST),*)?), Args = RArgs>
				{
					#[inline(always)]
					default fn fn_fn() -> extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
						where F: Fn<RArgs, Output = RR> {
						trace!(get: (extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR as ProxyFn => Self));
						proxy_fn::<$($T,)* $($($REST,)*)? CR, Adapter, F, RArgs, RR>
					}

					#[inline(always)]
					default fn fn_mut() -> extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
						where F: FnMut<RArgs, Output = RR> {
						trace!(get: (extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR as ProxyMut => Self));
						proxy_mut::<$($T,)* $($($REST,)*)? CR, Adapter, F, RArgs, RR>
					}

					#[inline(always)]
					default fn fn_once() -> extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
						where F: FnOnce<RArgs, Output = RR> {
						trace!(get: (extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR as ProxyOnce => Self));
						proxy_once::<$($T,)* $($($REST,)*)? CR, Adapter, F, RArgs, RR>
					}
				}

				extern "C" fn proxy_fn<$($T,)* $($($REST,)*)? R, Conv, F, Args, Ret>($([<$T:lower>]: $T,)* ud: UdPtr, $($([<$REST:lower>]: $REST,)*)?) -> R
					where Conv: arg::Adapter<Params = ($($T,)* Ud<F>, $($($REST),*)?), Args = Args>,
							F: Fn<Args, Output = Ret>,
							Args: Tuple,
							Ret: Into<R> {
					let ud: Ud<F> = Ud::from(ud.cast());

					let p: *const F = ud.0.cast();
					trace!(get: (&F as Fn => Ud<F>));
					if let Some(f) = unsafe { p.as_ref() } {
						trace!(call: (F as Fn => Ud<F>));
						let args = Conv::convert(($([<$T:lower>],)* ud, $($([<$REST:lower>]),*)?));
						f.call(args).into()
					} else {
						panic!("missed callback: {}", type_name::<F>())
					}
				}

				extern "C" fn proxy_mut<$($T,)* $($($REST,)*)? R, Conv, F, Args, Ret>($([<$T:lower>]: $T,)* ud: UdPtr, $($([<$REST:lower>]: $REST,)*)?) -> R
					where Conv: arg::Adapter<Params = ($($T,)* Ud<F>, $($($REST),*)?), Args = Args>,
							F: FnMut<Args, Output = Ret>,
							Args: Tuple,
							Ret: Into<R> {
					let ud: Ud<F> = Ud::from(ud.cast());

					let p: *mut F = ud.0.cast();
					trace!(get: (&mut F as FnMut => Ud<F>));
					if let Some(f) = unsafe { p.as_mut() } {
						trace!(call: (F as FnMut => Ud<F>));
						let args = Conv::convert(($([<$T:lower>],)* ud, $($([<$REST:lower>]),*)?));
						f.call_mut(args).into()
					} else {
						panic!("missed callback: {}", type_name::<F>())
					}
				}

				extern "C" fn proxy_once<$($T,)* $($($REST,)*)? R, Conv, F, Args, Ret>($([<$T:lower>]: $T,)* ud: UdPtr, $($([<$REST:lower>]: $REST,)*)?) -> R
					where Conv: arg::Adapter<Params = ($($T,)* Ud<F>, $($($REST),*)?), Args = Args>,
							F: FnOnce<Args, Output = Ret>,
							Args: Tuple,
							Ret: Into<R> {
					if !ud.is_null() {
						use alloc::boxed::Box;

						let ud: Ud<F> = Ud::from(ud.cast());
						trace!(rem: (F as FnOnce => Ud<F>));
						let f = unsafe { Box::from_raw(ud.0) };

						trace!(call: (F as FnOnce => Ud<F>));
						let args = Conv::convert(($([<$T:lower>],)* ud, $($([<$REST:lower>]),*)?));
						f.call_once(args).into()
					} else {
						panic!("missed callback: {}", type_name::<F>())
					}
				}


				mod ext {
					use super::*;


					// spec for Ud with (F, Ext)
					impl<F, Ext, Storage, Adapter, $($T,)* $($($REST,)*)? CR, RArgs, RR>
						Proxy<extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<(F, Ext)>, $($($REST),*)?), CR, F, RArgs, RR> for Default<F, Storage, Adapter>
						where RArgs: Tuple,
								RR: Into<CR>,
								Adapter: arg::Adapter<Params = ($($T,)* Ud<Ext>, $($($REST),*)?), Args = RArgs>,
					{
						#[inline(always)]
						default fn fn_fn() -> extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
							where F: Fn<RArgs, Output = RR> {
							trace!(get: (extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR as ProxyFn => Self));
							proxy_fn::<$($T,)* $($($REST,)*)? CR, Adapter, F, RArgs, RR, Ext>
						}

						#[inline(always)]
						default fn fn_mut() -> extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
							where F: FnMut<RArgs, Output = RR> {
							trace!(get: (extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR as ProxyMut => Self));
							proxy_mut::<$($T,)* $($($REST,)*)? CR, Adapter, F, RArgs, RR, Ext>
						}

						#[inline(always)]
						default fn fn_once() -> extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
							where F: FnOnce<RArgs, Output = RR> {
							trace!(get: (extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR as ProxyOnce => Self));
							proxy_once::<$($T,)* $($($REST,)*)? CR, Adapter, F, RArgs, RR, Ext>
						}
					}


					extern "C" fn proxy_fn<$($T,)* $($($REST,)*)? R, Conv, F, Args, Ret, Ext>($([<$T:lower>]: $T,)* ud: UdPtr, $($([<$REST:lower>]: $REST,)*)?) -> R
						where Conv: arg::Adapter<Params = ($($T,)* Ud<Ext>, $($($REST),*)?), Args = Args>,
								F: Fn<Args, Output = Ret>,
								Args: Tuple,
								Ret: Into<R> {
						let ud: Ud<(F, Ext)> = Ud::from(ud.cast());

						let p: *mut (F, Ext) = ud.0.cast();
						trace!(get: (&F as Fn => Ud<(F, Ext)>));
						if let Some((f, ext)) = unsafe { p.as_mut() } {
							trace!(call: (F as Fn => Ud<(F, Ext)>));
							let ud: Ud<Ext> = Ud::<Ext>::from(ext as *mut Ext);
							let args = Conv::convert(($([<$T:lower>],)* ud, $($([<$REST:lower>]),*)?));
							f.call(args).into()
						} else {
							panic!("missed callback: {}", type_name::<F>())
						}
					}

					extern "C" fn proxy_mut<$($T,)* $($($REST,)*)? R, Conv, F, Args, Ret, Ext>($([<$T:lower>]: $T,)* ud: UdPtr, $($([<$REST:lower>]: $REST,)*)?) -> R
						where Conv: arg::Adapter<Params = ($($T,)* Ud<Ext>, $($($REST),*)?), Args = Args>,
								F: FnMut<Args, Output = Ret>,
								Args: Tuple,
								Ret: Into<R> {
						let ud: Ud<(F, Ext)> = Ud::from(ud.cast());

						let p: *mut (F, Ext) = ud.0.cast();
						trace!(get: (&mut F as Fn => Ud<(F, Ext)>));
						if let Some((f, ext)) = unsafe { p.as_mut() } {
							trace!(call: (F as FnMut => Ud<(F, Ext)>));
							let ud: Ud<Ext> = Ud::<Ext>::from(ext as *mut Ext);
							let args = Conv::convert(($([<$T:lower>],)* ud, $($([<$REST:lower>]),*)?));
							f.call_mut(args).into()
						} else {
							panic!("missed callback: {}", type_name::<F>())
						}
					}

					extern "C" fn proxy_once<$($T,)* $($($REST,)*)? R, Conv, F, Args, Ret, Ext>($([<$T:lower>]: $T,)* ud: UdPtr, $($([<$REST:lower>]: $REST,)*)?) -> R
						where Conv: arg::Adapter<Params = ($($T,)* Ud<Ext>, $($($REST),*)?), Args = Args>,
								F: FnOnce<Args, Output = Ret>,
								Args: Tuple,
								Ret: Into<R> {
						use alloc::boxed::Box;
						use core::ptr::addr_of;

						if !ud.is_null() {
							let ud: Ud<(F, Ext)> = Ud::from(ud.cast());

							trace!(rem: (F as FnOnce => Ud<(F, Ext)>));
							let fud = unsafe { Box::from_raw(ud.0) };

							trace!(call: (F as FnOnce => Ud<(F, Ext)>));

							let ud = Ud::<Ext>::from(addr_of!(fud.1).cast_mut());
							let args = Conv::convert(($([<$T:lower>],)* ud, $($([<$REST:lower>]),*)?));
							let ret = fud.0.call_once(args).into();
							// drop(fud.1);
							ret
						} else {
							panic!("missed callback: {}", type_name::<F>())
						}
					}


					// spec for Ud with (F, Ext): safe -> unsafe
					impl<Prx, F, Ext, $($T,)* $($($REST,)*)? CR, RArgs, RR>
						Proxy<unsafe extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<(F, Ext)>, $($($REST),*)?), CR, F, RArgs, RR> for Prx
						where Prx:
							Proxy<extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<(F, Ext)>, $($($REST),*)?), CR, F, RArgs, RR>,
							RArgs: Tuple
					{
						#[inline(always)]
						default fn fn_fn() -> unsafe extern "C" fn($($T,)* UdPtr $(,$($REST),*)? ) -> CR
							where F: Fn<RArgs, Output = RR> {
							<Prx as Proxy<extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<(F, Ext)>, $($($REST),*)?), CR, F, RArgs, RR>>::fn_fn()
						}

						#[inline(always)]
						default fn fn_mut() -> unsafe extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
							where F: FnMut<RArgs, Output = RR> {
							<Prx as Proxy<extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<(F, Ext)>, $($($REST),*)?), CR, F, RArgs, RR>>::fn_mut()
						}

						#[inline(always)]
						default fn fn_once() -> unsafe extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR
							where F: FnOnce<RArgs, Output = RR> {
							<Prx as Proxy<extern "C" fn($($T,)* UdPtr $(,$($REST),*)?) -> CR, ($($T,)* Ud<(F, Ext)>, $($($REST),*)?), CR, F, RArgs, RR>>::fn_once()
						}
					}
				}
			}
		}};
	}

impl_ud!(E D C B A Z Y X W V U T);


#[cfg(test)]
mod ext {
	use core::marker::PhantomData;
	use alloc::boxed::Box;

	use crate::arg::Adapter;
	use crate::proxy;
	use crate::scope;
	use crate::scope::Scope;
	use super::*;


	#[test]
	fn with_default() {
		let f = |_: Option<&mut u8>| {};
		let ud = 42_u8;


		accept_exact(f, ud);
		accept(f, ud);

		let f = |_: Option<&u8>| {};
		accept(f, ud);


		accept_mut(|_: Option<&u8>| {}, ud);
		accept_once(|_: Option<&mut u8>| {}, ud);


		let v = vec![42];
		let f = move |_: Option<&u8>| drop(v);
		accept_once(f, ud);

		type A<'t, Args, Ext> = <scope::Deferred as Scope>::Adapter<(Ud<Ext>,), Args>;
		type P<'t, F, Args, Ext> = proxy::default::Default<F, (), A<'t, Args, Ext>>;

		fn accept_exact<F, Ext>(f: F, ext: Ext)
			where F: Fn(Option<&mut Ext>) {
			type C = unsafe extern "C" fn(UdPtr) -> ();

			let c = <P<F, (Option<&mut Ext>,), Ext> as Proxy<
			                                                 C,
			                                                 (Ud<(F, Ext)>,),
			                                                 (),
			                                                 F,
			                                                 (Option<&mut Ext>,),
			                                                 F::Output,
			>>::fn_fn();

			let ud = Box::into_raw(Box::new((f, ext)));
			unsafe { c(ud.cast()) };
			unsafe { c(ud.cast()) };
			unsafe { c(ud.cast()) };
			drop(unsafe { Box::from_raw(ud) });
		}


		type CRes = ();
		fn accept<F, Args: Tuple, Ext, Res>(f: F, ext: Ext)
			where F: Fn<Args, Output = Res>,
			      arg::default::Into<(Ud<Ext>,), Args>: arg::Adapter,
			      for<'t> A<'t, Args, Ext>: Adapter<Params = (Ud<Ext>,), Args = Args>,
			      CRes: From<F::Output> {
			type C = unsafe extern "C" fn(UdPtr) -> CRes;

			let c = <P<F, Args, Ext> as Proxy<C, (Ud<(F, Ext)>,), CRes, F, Args, F::Output>>::fn_fn();
			let ud = Box::into_raw(Box::new((f, ext)));
			unsafe { c(ud.cast()) };
			unsafe { c(ud.cast()) };
			drop(unsafe { Box::from_raw(ud) });
		}

		fn accept_mut<F, Args: Tuple, Ext, Res>(f: F, ext: Ext)
			where F: FnMut<Args, Output = Res>,
			      arg::default::Into<(Ud<Ext>,), Args>: arg::Adapter,
			      for<'t> A<'t, Args, Ext>: Adapter<Params = (Ud<Ext>,), Args = Args>,
			      CRes: From<F::Output> {
			type C = unsafe extern "C" fn(UdPtr) -> CRes;

			let c = <P<F, Args, Ext> as Proxy<C, (Ud<(F, Ext)>,), CRes, F, Args, F::Output>>::fn_mut();
			let ud = Box::into_raw(Box::new((f, ext)));
			unsafe { c(ud.cast()) };
			unsafe { c(ud.cast()) };
			drop(unsafe { Box::from_raw(ud) });
		}

		fn accept_once<F, Args: Tuple, Ext, Res>(f: F, ext: Ext)
			where F: FnOnce<Args, Output = Res>,
			      arg::default::Into<(Ud<Ext>,), Args>: arg::Adapter,
			      for<'t> A<'t, Args, Ext>: Adapter<Params = (Ud<Ext>,), Args = Args>,
			      CRes: From<F::Output> {
			type C = unsafe extern "C" fn(UdPtr) -> CRes;

			let c = <P<F, Args, Ext> as Proxy<C, (Ud<(F, Ext)>,), CRes, F, Args, F::Output>>::fn_once();
			let ud = Box::into_raw(Box::new((f, ext)));
			unsafe { c(ud.cast()) }; // Box freed in the `c`
		}
	}


	#[test]
	fn with_custom() {
		let f = |v: &mut u8| *v += 1;
		let ud = 42_u8;

		let res = accept(f, ud);
		assert_eq!(43, res);


		fn accept<F, Ext>(f: F, ext: Ext) -> Ext
			where F: Fn(&mut Ext) {
			type C = unsafe extern "C" fn(UdPtr) -> ();
			let c = <proxy::default::Default<
			                                F,
			                                <scope::Deferred as Scope>::Storage<F>,
			                                Custom<(Ud<Ext>,), (&mut Ext,)>,
			> as Proxy<C, (Ud<(F, Ext)>,), (), F, (&mut Ext,), ()>>::fn_fn();

			let ud = Box::into_raw(Box::new((f, ext)));

			unsafe { c(ud.cast()) };

			let (_, ud) = *unsafe { Box::from_raw(ud) };
			ud
		}
	}

	struct Custom<In, Out>(PhantomData<(In, Out)>);

	impl<'t, F: 't, RUD> Adapter for Custom<(Ud<(F, RUD)>,), (&'t mut RUD,)> {
		type Params = (Ud<(F, RUD)>,);
		type Args = (&'t mut RUD,);

		fn convert(ud: Self::Params) -> Self::Args {
			unsafe {
				(ud.0
				   .as_udptr()
				   .cast::<(F, RUD)>()
				   .as_mut()
				   .map(|(_, v)| v)
				   .unwrap(),)
			}
		}
	}

	impl<'t, F: 't, RUD> Adapter for Custom<(Ud<(F, RUD)>,), (&'t RUD,)> {
		type Params = (Ud<(F, RUD)>,);
		type Args = (&'t RUD,);

		fn convert(ud: Self::Params) -> Self::Args {
			unsafe {
				(ud.0
				   .as_udptr()
				   .cast::<(F, RUD)>()
				   .as_mut()
				   .map(|(_, v)| v)
				   .unwrap(),)
			}
		}
	}

	impl<'t, RUD> Adapter for Custom<(Ud<RUD>,), (&'t RUD,)> {
		type Params = (Ud<RUD>,);
		type Args = (&'t RUD,);

		fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<RUD>().as_mut().unwrap(),) } }
	}
	impl<'t, RUD> Adapter for Custom<(Ud<RUD>,), (&'t mut RUD,)> {
		type Params = (Ud<RUD>,);
		type Args = (&'t mut RUD,);

		fn convert(ud: Self::Params) -> Self::Args { unsafe { (ud.0.as_udptr().cast::<RUD>().as_mut().unwrap(),) } }
	}
}
