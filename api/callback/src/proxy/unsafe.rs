use core::marker::Tuple;
use super::Proxy;

// Semi-auto impls "to unsafe C" for "to safe C" for any rust arg-set for each of `CArgs` substitution.

#[duplicate::duplicate_item(
		CArgs; []; [A,]; [A, B]; [A, B, C]; [A, B, C, D];
		[A, B, C, D, E]; [A, B, C, D, E, F]; [A, B, C, D, E, F, G];
		[A, B, C, D, E, F, G, H]; [A, B, C, D, E, F, G, H, I];
		[A, B, C, D, E, F, G, H, I, J];
		[A, B, C, D, E, F, G, H, I, J, K];
	)]
impl<T, CR, RFn, RArgs, RRet, CArgs> Proxy<unsafe extern "C" fn(CArgs) -> CR, (CArgs), CR, RFn, RArgs, RRet> for T
	where T: Proxy<extern "C" fn(CArgs) -> CR, (CArgs), CR, RFn, RArgs, RRet>,
	      RFn: FnOnce<RArgs, Output = RRet>,
	      RArgs: Tuple
{
	#[inline(always)]
	default fn fn_fn() -> unsafe extern "C" fn(CArgs) -> CR
		where RFn: Fn<RArgs> {
		<T as Proxy<extern "C" fn(CArgs) -> CR, (CArgs), CR, RFn, RArgs, RFn::Output>>::fn_fn()
	}

	#[inline(always)]
	default fn fn_mut() -> unsafe extern "C" fn(CArgs) -> CR
		where RFn: FnMut<RArgs> {
		<T as Proxy<extern "C" fn(CArgs) -> CR, (CArgs), CR, RFn, RArgs, RFn::Output>>::fn_mut()
	}

	#[inline(always)]
	default fn fn_once() -> unsafe extern "C" fn(CArgs) -> CR
		where RFn: FnOnce<RArgs> {
		<T as Proxy<extern "C" fn(CArgs) -> CR, (CArgs), CR, RFn, RArgs, RFn::Output>>::fn_once()
	}
}
