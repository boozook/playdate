use core::marker::PhantomData;
use core::marker::Tuple;


#[diagnostic::on_unimplemented(label = "convertion for the set of parameters of this function is not implemented",
                               message = "the trait Adapter<Args = (parameters of the function)> is not implemented",
                               note = "this is reqirement for the c-proxy function if used, otherwise it could be mismatch of c-parameters and arguments of given function")]
/// Converts parameters to arguments.
pub trait Adapter {
	/// Parameters of a C-function _usually_.
	type Params: Tuple;

	/// Arguments of a Rust-function _usually_.
	type Args: Tuple;

	fn convert(src: Self::Params) -> Self::Args;
}


pub mod default {
	use super::*;


	pub struct Into<CArgs, RArgs>(PhantomData<(CArgs, RArgs)>);


	// gen impl e.g.:
	// impl<A, A1, B, B1> Adapter for Into<(A, B), (A1, B1)>
	// 	where A: core::convert::Into<A1>,
	// 	      B: core::convert::Into<B1>
	// {
	// 	type Params = (A, B);
	// 	type Args = (A1, B1);
	//
	// 	#[inline(always)]
	// 	default fn convert(src: Self::Params) -> Self::Args { (src.0.into(), src.1.into()) }
	// }
	macro_rules! impl_base {
		// alt symbols: ⁿ ˇ ˆ ˉ

		// Stopping criteria (1?-ary tuple)
		($T:ident) => {
			impl_base!(@impl () ());
			::pastey::paste!{ impl_base!(@impl ($T) ([<# $T ˉ>])); }
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($T:ident $( $U:ident )+) => {
			impl_base!($( $U )+);
			::pastey::paste!{ impl_base!(@impl ($T $( $U )+) ([<# $T ˉ>] $( [<# $U ˉ>] )+)); }
		};

		(@impl ($( $A:ident )*) ($( $B:ident )*)) => { ::pastey::paste!{
			impl<$($A,)* $($B),*> Adapter for Into<($($A,)*), ($($B,)*)>
				where $( $B: core::convert::From<$A> ),*
			{
				type Params = ($($A,)*);
				type Args = ($($B,)*);

				#[inline]
				default fn convert(src: Self::Params) -> Self::Args {
					let ($([<$A:lower>],)* ..) = src;
					( $( [< $A:lower >].into(), )* )
				}
			}

			#[test]
			#[cfg(test)]
			#[allow(non_snake_case)]
			fn [<convert_ $($A)* _ $($B)*>]() {
				$(type $A = u8;)*
				$(type $B = u16;)*

				type From = ($($A,)*);
				type To = ($($B,)*);
				let from: From = {
					$(let [<$A:lower>]: $A = 42;)*
					($([<$A:lower>],)*)
				};
				let to: To = Into::<From, To>::convert(from);
				let expected: To = {
					$(let [<$B:lower>]: $B = 42;)*
					($([<$B:lower>],)*)
				};
				assert_eq!(expected, to);
			}
		}};
	}

	impl_base!(E D C B A Z Y X W V U T);


	// spec: cut-out ud
	macro_rules! impl_ud {
		// Stopping criteria (1?-ary tuple)
		($T:ident) => {
			::pastey::paste!{ impl_ud!(@impl ($T) ([<# $T ˉ>])); }
			// ::pastey::paste!{ impl_ud!(@impl (... $T) ([<# $T ˉ>])); }
			impl_ud!(@impl () ());
		};

		// Running criteria (n-ary tuple, with n >= 2)
		($T:ident $( $U:ident )+) => {
			impl_ud!($( $U )+);
			::pastey::paste!{ impl_ud!(@impl ($T $( $U )+) ([<# $T ˉ>] $( [<# $U ˉ>] )+)); }
			// ::pastey::paste!{ impl_ud!(@impl ($T ... $( $U )+) ([<# $T ˉ>] $( [<# $U ˉ>] )+)); }
			// ::pastey::paste!{ impl_ud!(@impl (... $T $( $U )+) ([<# $T ˉ>] $( [<# $U ˉ>] )+)); }
		};

		(@impl ($( $A:ident )* $( ... $($REST:ident)* )?) ($($B:ident )*)) => { ::pastey::paste!{
			#[allow(non_snake_case)]
			mod [< $($A)* _ud_ $( $($REST)*)? >] {
				use $crate::util::marker::Ud;
				use super::*;


				#[diagnostic::do_not_recommend]
				impl<$($A,)* UD, $( $($REST,)* )? $($B),*> Adapter for Into<($($A,)* Ud<UD>, $($($REST,)*)?), ($($B,)*)>
					where Into<($($A,)* $($($REST,)*)?), ($($B,)*)>: Adapter<Params = ($($A,)* $($($REST,)*)?), Args = ($($B,)*)>
				{
					type Params = ($($A,)* Ud<UD>, $($($REST,)*)?);
					type Args = ($($B,)*);

					#[inline]
					default fn convert(src: Self::Params) -> Self::Args {
						let ($([<$A:lower>],)* _ud_, $($($REST:lower,)*)?) = src;
						Into::<($($A,)* $($($REST,)*)?), ($($B,)*)>::convert((
							$( [< $A:lower >].into(), )*
							$($($REST:lower,)*)?
						))
					}
				}


				// spec for extra ud as `Ud<Extra>` `opt &Extra`:
				impl<'t, $($A,)* UD, $( $($REST,)* )? $($B),*> Adapter for Into<($($A,)* Ud<UD>, $($($REST,)*)?), ($($B,)* Option<&'t UD>,)>
					where
						Into<($($A,)* $($($REST,)*)?), ($($B,)*)>: Adapter<Params = ($($A,)* $($($REST,)*)?), Args = ($($B,)*)>,
						Ud<UD>: core::convert::Into<Option<&'t UD>>,
				{
					type Params = ($($A,)* Ud<UD>, $($($REST,)*)?);
					type Args = ($($B,)* Option<&'t UD>,);

					#[inline]
					default fn convert(src: Self::Params) -> Self::Args {
						let ($([<$A:lower>],)* _ud_, $($($REST:lower,)*)?) = src;
						let ($([<$B:lower>],)*) = Into::<($($A,)* $($($REST,)*)?), ($($B,)*)>::convert((
							$( [< $A:lower >].into(), )*
							$($($REST:lower,)*)?
						));
						($([<$B:lower>],)* _ud_.into(),)
					}
				}

				// spec for extra ud as `Ud<Extra>` `opt &mut Extra`:
				impl<'t, $($A,)* UD, $( $($REST,)* )? $($B),*> Adapter for Into<($($A,)* Ud<UD>, $($($REST,)*)?), ($($B,)* Option<&'t mut UD>,)>
					where
						Into<($($A,)* $($($REST,)*)?), ($($B,)*)>: Adapter<Params = ($($A,)* $($($REST,)*)?), Args = ($($B,)*)>,
						Ud<UD>: core::convert::Into<Option<&'t mut UD>>,
				{
					type Params = ($($A,)* Ud<UD>, $($($REST,)*)?);
					type Args = ($($B,)* Option<&'t mut UD>,);

					#[inline]
					default fn convert(src: Self::Params) -> Self::Args {
						let ($([<$A:lower>],)* _ud_, $($($REST:lower,)*)?) = src;
						let ($([<$B:lower>],)*) = Into::<($($A,)* $($($REST,)*)?), ($($B,)*)>::convert((
							$( [< $A:lower >].into(), )*
							$($($REST:lower,)*)?
						));
						($([<$B:lower>],)* _ud_.into(),)
					}
				}

				// spec for extra ud as `Ud<F + Extra>` `opt &Extra`:
				#[diagnostic::do_not_recommend]
				impl<'t, F, $($A,)* UD, $( $($REST,)* )? $($B),*> Adapter for Into<($($A,)* Ud<(F, UD)>, $($($REST,)*)?), ($($B,)* Option<&'t UD>,)>
					where
						Into<($($A,)* $($($REST,)*)?), ($($B,)*)>: Adapter<Params = ($($A,)* $($($REST,)*)?), Args = ($($B,)*)>,
						Ud<UD>: core::convert::Into<Option<&'t UD>>,
				{
					type Params = ($($A,)* Ud<UD>, $($($REST,)*)?);
					type Args = ($($B,)* Option<&'t UD>,);

					#[inline]
					default fn convert(src: Self::Params) -> Self::Args {
						let ($([<$A:lower>],)* _ud_, $($($REST:lower,)*)?) = src;
						let ($([<$B:lower>],)*) = Into::<($($A,)* $($($REST,)*)?), ($($B,)*)>::convert((
							$( [< $A:lower >].into(), )*
							$($($REST:lower,)*)?
						));
						($([<$B:lower>],)* _ud_.into(),)
					}
				}

				// spec for extra ud as `Ud<F + Extra>` `opt &mut Extra`:
				#[diagnostic::do_not_recommend]
				impl<'t, F, $($A,)* UD, $( $($REST,)* )? $($B),*> Adapter for Into<($($A,)* Ud<(F, UD)>, $($($REST,)*)?), ($($B,)* Option<&'t mut UD>,)>
					where
						Into<($($A,)* $($($REST,)*)?), ($($B,)*)>: Adapter<Params = ($($A,)* $($($REST,)*)?), Args = ($($B,)*)>,
						Ud<UD>: core::convert::Into<Option<&'t mut UD>>,
				{
					type Params = ($($A,)* Ud<UD>, $($($REST,)*)?);
					type Args = ($($B,)* Option<&'t mut UD>,);

					#[inline]
					default fn convert(src: Self::Params) -> Self::Args {
						let ($([<$A:lower>],)* _ud_, $($($REST:lower,)*)?) = src;
						let ($([<$B:lower>],)*) = Into::<($($A,)* $($($REST,)*)?), ($($B,)*)>::convert((
							$( [< $A:lower >].into(), )*
							$($($REST:lower,)*)?
						));
						($([<$B:lower>],)* _ud_.into(),)
					}
				}


				#[test]
				#[cfg(test)]
				#[allow(non_snake_case)]
				fn [<convert_ $($A)* _ud_ $($($REST)*)? _ $($B)*>]() {
					$(type $A = u8;)*
					$(type $($REST:lower)* = u8;)?
					$(type $B = u16;)*

					type From = ($($A,)* $($($REST,)*)?);
					type To = ($($B,)*);
					let from: From = {
						$(let [<$A:lower>]: $A = 42;)*
						$($(let [<$REST:lower>]: $REST = 42;)*)?
						($([<$A:lower>],)* $($([<$REST:lower>],)*)?)
					};
					let to: To = Into::<From, To>::convert(from);
					let expected: To = {
						$(let [<$B:lower>]: $B = 42;)*
						($([<$B:lower>],)*)
					};
					assert_eq!(expected, to);
				}


				#[test]
				#[cfg(test)]
				#[allow(non_snake_case)]
				fn [<convert_ $($A)* _ud_ $($($REST)*)? _ $($B)* _ext_mut>]() {
					$(type $A = u8;)*
					$(type $($REST:lower)* = u8;)?
					$(type $B = u16;)*

					type UDV = [u32; 1];
					type UD = Ud<UDV>;

					type From = ($($A,)* UD, $($($REST,)*)?);
					type To<'t> = ($($B,)* Option<&'t mut UDV>,);

					use alloc::boxed::Box;
					let p = Box::into_raw(Box::new([42u32; 1]));

					let from: From = {
						$(let [<$A:lower>]: $A = 42;)*
						let ud = Ud::from(p);
						$($(let [<$REST:lower>]: $REST = 42;)*)?
						($([<$A:lower>],)* ud, $($([<$REST:lower>],)*)?)
					};
					let to: To = Into::<From, To>::convert(from);

					let mut expected_value = [42u32; 1];
					let expected: To = {
						$(let [<$B:lower>]: $B = 42;)*
						($([<$B:lower>],)* Some(&mut expected_value),)
					};

					assert_eq!(expected, to);

					drop(unsafe { Box::from_raw(p) });
				}
			}
		}};
	}

	// here is missed `E` because with tuple `E` + Ud is not comparable (have no impl in corelib).
	impl_ud!(D C B A Z Y X W V U T);
}
