use core::marker::Tuple;


pub(crate) trait FnIntoCallback<Scope, Target, Args: Tuple, Ret>:
	FnMutIntoCallback<Scope, Target, Args, Ret> {
	fn into_callback(self) -> Target;
}

pub(crate) trait FnMutIntoCallback<Ctx, Target, Args: Tuple, Ret>:
	FnOnceIntoCallback<Ctx, Target, Args, Ret> {
	fn into_callback_mut(self) -> Target;
}

pub(crate) trait FnOnceIntoCallback<Ctx, Target, Args: Tuple, Ret> {
	fn into_callback_once(self) -> Target;
}


mod as_is {
	use super::*;


	// Gen impl FnTrait< c-fn( ARG_SET ) > for c-fn( ARG_SET )
	// for each invariant of ARG_SET from 0 to 12 args.


	macro_rules! impl_into {
		// Stopping criteria (1?-ary tuple)
		($trait:ident:$fn:ident $T:ident) => {
			impl_into!(@impl $trait:$fn $T);
			impl_into!(@impl $trait:$fn);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($trait:ident:$fn:ident $T:ident $( $U:ident )+) => {
			impl_into!($trait:$fn $( $U )+);
			impl_into!(@impl $trait:$fn $T $( $U )+);
		};

		(@impl $trait:ident:$fn:ident $( $T:ident )*) => { ::pastey::paste!{
			#[allow(non_snake_case)]

			mod [< $trait _ $($T)* >] {
				use super::[<$trait IntoCallback>];

				#[allow(unused)]
				type CFn<$($T,)* R> = extern "C" fn($($T),*) -> R;

				impl<Ctx, $($T,)* R> [<$trait IntoCallback>]<Ctx, CFn<$($T,)* R>, ($($T,)*), R> for CFn<$($T,)* R> {
					#[inline(always)]
					fn $fn(self) -> CFn<$($T,)* R> {
						{
							use $crate::util::macros::*;
							trace!(
								"C({})::{name}: default (as-is) ... -> {}",
								tlen!($($T),*),
								core::any::type_name::<Self>(),
								name=stringify!($fn),
							);
						}
						self
					}
				}


				#[cfg(test)]
				#[duplicate::duplicate_item( Scope; [Deferred]; [Async]; )] // TODO: [Immediate]; [Unique...]
				mod c {
					use super::*;
					use crate::scope::*;
					use core::marker::Tuple;

					#[test]
					fn test() {
						$(type $T = u8;)*

						extern "C" fn f($(_:$T),*) {}
						let coerced = f as CFn<$($T,)* ()>;
						accept(coerced);

						fn accept<$($T,)* Rargs: Tuple, RR>(f: impl [<$trait IntoCallback>]<Scope, CFn<$($T,)* ()>, Rargs, RR>) {
							let _ = f.$fn();
						}
					}
				}
			}
		}};
	}

	impl_into!(Fn:into_callback E D C B A Z Y X W V U T);
	impl_into!(FnMut:into_callback_mut E D C B A Z Y X W V U T);
	impl_into!(FnOnce:into_callback_once E D C B A Z Y X W V U T);
}


// safe -> unsafe
mod r#unsafe {
	use super::*;

	// Gen impl FnTrait< unsafe c-fn( ARG_SET ) >
	// for any T that implements FnTrait< safe c-fn( ARG_SET ) >
	// for each invariant of ARG_SET from 0 to 12 args,
	// where T if any fn with any arg-set, so it's independent of the C-fn's ARG_SET.


	macro_rules! impl_into {
		// Stopping criteria (1?-ary tuple)
		($trait:ident:$fn:ident $T:ident) => {
			impl_into!(@impl $trait:$fn $T);
			impl_into!(@impl $trait:$fn);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($trait:ident:$fn:ident $T:ident $( $U:ident )+) => {
			impl_into!($trait:$fn $( $U )+);
			impl_into!(@impl $trait:$fn $T $( $U )+);
		};

		(@impl $trait:ident:$fn:ident $( $T:ident )*) => { ::pastey::paste!{
			#[allow(non_snake_case)]

			mod [< $trait _ $($T)* >] {
				use core::marker::Tuple;
				use super::[<$trait IntoCallback>];

				type Safe<$($T,)* R> = extern "C" fn($($T),*) -> R;
				type Unsafe<$($T,)* R> = unsafe extern "C" fn($($T),*) -> R;


				impl<Ctx, $($T,)* R, RFn, RArgs: Tuple, RRet > [<$trait IntoCallback>]<Ctx, Unsafe<$($T,)* R>, RArgs, RRet> for RFn
					where RFn: [<$trait IntoCallback>]<Ctx, Safe<$($T,)* R>, RArgs, RRet>
				{
					#[inline(always)]
					default fn $fn(self) -> Unsafe<$($T,)* R> {
						<RFn as [<$trait IntoCallback>]<Ctx, Safe<$($T,)* R>, RArgs, RRet>>::$fn(self)
					}
				}
			}
		}};
	}

	impl_into!(Fn:into_callback E D C B A Z Y X W V U T);
	impl_into!(FnMut:into_callback_mut E D C B A Z Y X W V U T);
	impl_into!(FnOnce:into_callback_once E D C B A Z Y X W V U T);
}


pub mod base {
	use core::marker::Tuple;

	use crate::storage::Store;
	use crate::proto::proxy::{self, Proxy};
	use crate::scope;
	use super::*;


	macro_rules! proxy_fn_iddent {
		(Fn of $t:ty) => {
			<$t>::fn_fn
		};
		(FnMut of $t:ty) => {
			<$t>::fn_mut
		};
		(FnOnce of $t:ty) => {
			<$t>::fn_once
		};
	}


	macro_rules! impl_base {
		// Stopping criteria (1?-ary tuple)
		($trait:ident:$fn:ident $T:ident) => {
			impl_base!(@impl $trait:$fn $T);
			impl_base!(@impl $trait:$fn);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($trait:ident:$fn:ident $T:ident $( $U:ident )+) => {
			impl_base!($trait:$fn $( $U )+);
			impl_base!(@impl $trait:$fn $T $( $U )+);
		};

		(@impl $trait:ident:$fn:ident $( $T:ident )*) => { ::pastey::paste!{
			#[allow(non_snake_case)]

			mod [< $trait _ $($T)* >] {
				use super::*;

				type CFn<$($T,)* R> = extern "C" fn($($T),*) -> R;


				impl<Scope, $($T,)* CR, RFn, RArgs: Tuple> [<$trait IntoCallback>]<Scope, CFn<$($T,)* CR>, RArgs, RFn::Output> for RFn
					where RFn: $trait<RArgs>,
							Scope: scope::Scope,
							// Scope::Adapter<($($T,)*), RArgs>: arg::Adapter<Params = ($($T,)*), Args = RArgs>,
							Scope::Storage<RFn>: Store<RFn>,
							Scope::Proxy<CFn<$($T,)* CR>, ($($T,)*), CR, RFn, RArgs, RFn::Output>:
								proxy::Proxy<CFn<$($T,)* CR>, ($($T,)*), CR, RFn, RArgs, RFn::Output>
				{
					#[inline]
					default fn $fn(self) -> CFn<$($T,)* CR> {
						#[cfg(debug_assertions)]
						{
							use $crate::util::macros::*;
							#[rustfmt::skip]
							trace!(
									"{trait}::{fn}: [{r}] --> C fn[{c}] (size: {size}, ty: {ty}), using {adapter}",
									trait = stringify!($trait),
									fn = stringify!($fn),
									ty = core::any::type_name::<Self>(),
									size = size_of::<Self>(),
									c = format_args!("{} -> {}", core::any::type_name::<($($T,)*)>(), core::any::type_name::<CR>()),
									r = format_args!("{} -> {}", core::any::type_name::<RArgs>(), core::any::type_name::<RFn::Output>()),
									adapter = core::any::type_name::<<Scope as crate::scope::Scope>::Adapter<($($T,)*), RArgs>>(),
							);
						}

						Scope::Storage::<Self>::set(self);
						proxy_fn_iddent!($trait of Scope::Proxy::<CFn<$($T,)* CR>, ($($T,)*), CR, RFn, RArgs, RFn::Output>)()
					}
				}
			}
		}};
	}

	impl_base!(Fn:into_callback E D C B A Z Y X W V U T);
	impl_base!(FnMut:into_callback_mut E D C B A Z Y X W V U T);
	impl_base!(FnOnce:into_callback_once E D C B A Z Y X W V U T);


	#[cfg(test)]
	mod tests {
		use super::*;


		#[test]
		fn test() {
			// fn:
			let f = |_: (), _: ()| {};
			accept_fn::<_, _, (), _, _>(f, (), ());
			accept_mut::<_, _, (), _, _>(f, (), ());
			accept_once::<_, _, (), _, _>(f, (), ());


			// mut:
			let mut v = alloc::vec::Vec::new();
			let mut f = move |_: (), _: ()| v.push(());
			// accept_mut::<(), (), (), _, _>(&mut f, (), ());
			accept_mut::<(), (), (), _, _>(f, (), ());
			let mut v = alloc::vec::Vec::new();
			// {
			// 	let v = &mut v;
			// 	accept_once::<(), (), (), _, _>(move |_: (), _: ()| v.push(()));
			// }
			accept_once::<(), (), (), _, _>(move |_: (), _: ()| v.push(()), (), ());


			// once:
			let v = alloc::vec::Vec::<()>::new();
			accept_once::<(), (), (), _, _>(move |_: (), _: ()| drop(v), (), ());
		}


		fn accept_once<CA, CB, CR, Rargs: Tuple, RR>(f: impl FnOnceIntoCallback<
		                                                                crate::scope::Deferred,
		                                                                unsafe extern "C" fn(CA, CB) -> CR,
		                                                                Rargs,
		                                                                RR,
		>,
		                                             a: CA,
		                                             b: CB) {
			let f = f.into_callback_once();
			unsafe { f(a, b) };
		}

		fn accept_mut<CA, CB, CR, Rargs: Tuple, RR>(f: impl FnMutIntoCallback<
		                                                              crate::scope::Deferred,
		                                                              unsafe extern "C" fn(CA, CB) -> CR,
		                                                              Rargs,
		                                                              RR,
		>,
		                                            a: CA,
		                                            b: CB) {
			let f = f.into_callback_mut();
			unsafe { f(a, b) };
		}

		fn accept_fn<CA, CB, CR, Rargs: Tuple, RR>(f: impl FnIntoCallback<
		                                                          crate::scope::Deferred,
		                                                          unsafe extern "C" fn(CA, CB) -> CR,
		                                                          Rargs,
		                                                          RR,
		>,
		                                           a: CA,
		                                           b: CB) {
			let f = f.into_callback();
			unsafe { f(a, b) };
		}
	}
}


mod ud {
	use super::*;


	macro_rules! impl_ud {
		// Stopping criteria (1?-ary tuple)
		($trait:ident:$fn:ident $T:ident) => {
			impl_ud!(@impl $trait:$fn $T);
			impl_ud!(@impl $trait:$fn ... $T);
			impl_ud!(@impl $trait:$fn);
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($trait:ident:$fn:ident $T:ident $( $U:ident )+) => {
			impl_ud!($trait:$fn $( $U )+);
			impl_ud!(@impl $trait:$fn $T $( $U )+);
			impl_ud!(@impl $trait:$fn $T ... $( $U )+);
			impl_ud!(@impl $trait:$fn ... $T $( $U )+);
		};

		(@impl $trait:ident:$fn:ident $( $T:ident )* $( ... $($REST:ident)* )?) => { ::pastey::paste!{
			#[allow(non_snake_case)]

			mod [< $trait _ $($T)* $(_ $($REST)*)? >] {
				use alloc::boxed::Box;
				use core::marker::Tuple;

				use $crate::util::macros::tlen;
				use $crate::util::marker::{Ud, UdPtr, UdFn, FnWith};
				use $crate::proto::proxy::{self, Proxy};
				use $crate::scope;
				use super::[<$trait IntoCallback>];


				type Safe<$($T,)* $($($REST,)*)? CR> = extern "C" fn($($T,)* UdPtr, $($($REST),*)?) -> CR;
				type Unsafe<$($T,)* $($($REST,)*)? CR> = unsafe extern "C" fn($($T,)* UdPtr, $($($REST),*)?) -> CR;

				const I: u8 = tlen!($( $T )*);


				// spec for Ud with just F
				impl<Scope, $($T,)* $($($REST,)*)? CR, RFn, RArgs: Tuple>
					[<$trait IntoCallback>]<Scope, UdFn<Safe<$($T,)* $($($REST,)*)? CR>, RFn, I>, RArgs, RFn::Output> for RFn
					where RFn: $trait<RArgs> + 'static, // this 'static is brokes scope::Immediate :(
							Scope: scope::Scope,
							Scope::Proxy<Safe<$($T,)* $($($REST,)*)? CR>, ($($T,)* Ud<RFn>, $($($REST),*)?), CR, RFn, RArgs, RFn::Output>:
								proxy::Proxy<Safe<$($T,)* $($($REST,)*)? CR>, ($($T,)* Ud<RFn>, $($($REST),*)?), CR, RFn, RArgs, RFn::Output>
				{
					default fn $fn(self) -> UdFn<Safe<$($T,)* $($($REST,)*)? CR>, RFn, I> {
						#[cfg(debug_assertions)]
						{
							use $crate::util::macros::*;
							#[rustfmt::skip]
							trace!(
									"UD+{trait}::{fn}: [{r}] --> C fn[{c}] (size: {}, ty: {}), using {adapter}",
									size_of::<Self>(),
									core::any::type_name::<Self>(),
									trait = stringify!($trait),
									fn = stringify!($fn),
									c = format_args!("{} -> {}", core::any::type_name::<($($T,)* UdPtr, $($($REST,)*)?)>(), core::any::type_name::<CR>()),
									r = format_args!("{} -> {}", core::any::type_name::<RArgs>(), core::any::type_name::<RFn::Output>()),
									adapter = core::any::type_name::<<Scope as crate::scope::Scope>::Adapter<($($T,)* Ud<RFn>, $($($REST,)*)?), RArgs>>(),
							);
						}
						// for scope::Immediate here should be raw-ref
						let ud = Ud::from(Box::into_raw(Box::new(self)));
						let f = Scope::Proxy::<Safe<$($T,)* $($($REST,)*)? CR>, ($($T,)* Ud<RFn>, $($($REST),*)?), CR, RFn, RArgs, RFn::Output>::fn_once();
						UdFn(f, ud)
					}
				}


				// spec for Ud with (F, Ext)
				impl<Scope, $($T,)* $($($REST,)*)? CR, RFn, RArgs: Tuple, Ext>
					[<$trait IntoCallback>]<Scope, UdFn<Safe<$($T,)* $($($REST,)*)? CR>, (RFn, Ext), I>, RArgs, RFn::Output> for FnWith<RFn, Ext>
					where RFn: $trait<RArgs> + 'static,
							Scope: scope::Scope,
							Scope::Proxy<Safe<$($T,)* $($($REST,)*)? CR>, ($($T,)* Ud<(RFn, Ext)>, $($($REST),*)?), CR, RFn, RArgs, RFn::Output>:
								proxy::Proxy<Safe<$($T,)* $($($REST,)*)? CR>, ($($T,)* Ud<(RFn, Ext)>, $($($REST),*)?), CR, RFn, RArgs, RFn::Output>,
							Scope::Adapter<($($T,)* Ud<Ext>, $($($REST,)*)?), RArgs>: $crate::arg::Adapter<Params = ($($T,)* Ud<Ext>, $($($REST),*)?), Args = RArgs>,
				{
					default fn $fn(self) -> UdFn<Safe<$($T,)* $($($REST,)*)? CR>, (RFn, Ext), I> {
						#[cfg(debug_assertions)]
						{
							use $crate::util::macros::*;
							#[rustfmt::skip]
							trace!(
									"UD+{trait}::{fn}: [{r}] --> C fn[{c}] (size: {}, ty: {}), using {adapter}",
									size_of::<Self>(),
									core::any::type_name::<Self>(),
									trait = stringify!($trait),
									fn = stringify!($fn),
									c = format_args!("{} -> {}", core::any::type_name::<($($T,)* UdPtr, $($($REST,)*)?)>(), core::any::type_name::<CR>()),
									r = format_args!("{} -> {}", core::any::type_name::<RArgs>(), core::any::type_name::<RFn::Output>()),
									adapter = core::any::type_name::<<Scope as crate::scope::Scope>::Adapter<($($T,)* Ud<(RFn, Ext)>, $($($REST,)*)?), RArgs>>(),
							);
						}

						let FnWith(cb, ud) = self;
						let ud = Ud::from(Box::into_raw(Box::new((cb, ud))));
						let f = Scope::Proxy::<Safe<$($T,)* $($($REST,)*)? CR>, ($($T,)* Ud<(RFn, Ext)>, $($($REST),*)?), CR, RFn, RArgs, RFn::Output>::fn_once();
						UdFn::<_, _, I>(f, ud)
					}
				}


				//
				// safe -> unsafe
				impl<Scope, $($T,)* $($($REST,)*)? CR, RFn, RArgs: Tuple>
					[<$trait IntoCallback>]<Scope, UdFn<Unsafe<$($T,)* $($($REST,)*)? CR>, RFn, I>, RArgs, RFn::Output> for RFn
					where RFn: FnOnce<RArgs> + [<$trait IntoCallback>]<Scope, UdFn<Safe<$($T,)* $($($REST,)*)? CR>, RFn, I>, RArgs, RFn::Output>
				{
					#[inline(always)]
					default fn $fn(self) -> UdFn<Unsafe<$($T,)* $($($REST,)*)? CR>, RFn, I> {
						<Self as [<$trait IntoCallback>]<Scope, UdFn<Safe<$($T,)* $($($REST,)*)? CR>, RFn, I>, RArgs, RFn::Output>>::$fn(self).into()
					}
				}
				// TODO: safe -> unsafe for FnWith
				impl<Scope, $($T,)* $($($REST,)*)? CR, RFn, RArgs: Tuple, Ext>
					[<$trait IntoCallback>]<Scope, UdFn<Unsafe<$($T,)* $($($REST,)*)? CR>, (RFn, Ext), I>, RArgs, RFn::Output> for FnWith<RFn, Ext>
					where FnWith<RFn, Ext>:
						[<$trait IntoCallback>]<Scope, UdFn<Safe<$($T,)* $($($REST,)*)? CR>, (RFn, Ext), I>, RArgs, RFn::Output>,
						RFn:FnOnce<RArgs>
				{
					#[inline(always)]
					default fn $fn(self) -> UdFn<Unsafe<$($T,)* $($($REST,)*)? CR>, (RFn, Ext), I> {
						<Self as [<$trait IntoCallback>]<Scope, UdFn<Safe<$($T,)* $($($REST,)*)? CR>, (RFn, Ext), I>, RArgs, RFn::Output>>::$fn(self).into()
					}
				}
			}
		}};
	}

	impl_ud!(Fn:into_callback E D C B A Z Y X W V U T);
	impl_ud!(FnMut:into_callback_mut E D C B A Z Y X W V U T);
	impl_ud!(FnOnce:into_callback_once E D C B A Z Y X W V U T);


	#[cfg(test)]
	mod tests {
		use super::*;
		use crate::util::marker::{UdPtr, UdFn};


		#[test]
		fn test() {
			// fn:
			let f = |_: ()| {};
			accept_fn::<_, (), _, _, _>(f, ());
			accept_mut::<_, (), _, _, _>(f, ());
			accept_once::<_, (), _, _, _>(f, ());

			// // mut:
			let mut v = alloc::vec::Vec::new();
			let mut f = move |_: ()| v.push(());
			// accept_mut::<_, (), _, _, _>(&mut f, ());
			accept_mut::<_, (), _, _, _>(f, ());
			let mut v = alloc::vec::Vec::new();
			// {
			// 	let v = &mut v;
			// 	accept_once::<_, (), _, _, _>(move |u: ()| v.push(u), ());
			// }
			accept_once::<_, (), _, _, _>(move |u: ()| v.push(u), ());

			// once:
			let v = alloc::vec::Vec::<()>::new();
			accept_once::<_, (), _, _, _>(move |_: ()| drop(v), ());
		}


		fn accept_once<A, CR, Rargs: Tuple, RR, F>(f: F, a: A)
			where F: FnOnceIntoCallback<
			                            crate::scope::Deferred,
			                            UdFn<unsafe extern "C" fn(A, UdPtr) -> CR, F, 1>,
			                            Rargs,
			                            RR,
			> {
			let UdFn(f, ud) = f.into_callback_once();
			unsafe { f(a, ud.as_udptr()) };
		}

		fn accept_mut<A, CR, Rargs: Tuple, RR, F>(f: F, a: A)
			where F: FnMutIntoCallback<
			                           crate::scope::Deferred,
			                           UdFn<unsafe extern "C" fn(A, UdPtr) -> CR, F, 1>,
			                           Rargs,
			                           RR,
			> {
			let UdFn(f, ud) = f.into_callback_mut();
			unsafe { f(a, ud.as_udptr()) };
		}

		fn accept_fn<A, CR, Rargs: Tuple, RR, F>(f: F, a: A)
			where F: FnIntoCallback<
			                        crate::scope::Deferred,
			                        UdFn<unsafe extern "C" fn(A, UdPtr) -> CR, F, 1>,
			                        Rargs,
			                        RR,
			> {
			let UdFn(f, ud) = f.into_callback();
			unsafe { f(a, ud.as_udptr()) };
		}
	}
}
