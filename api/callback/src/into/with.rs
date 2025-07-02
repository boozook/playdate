use core::marker::Tuple;
use crate::util::marker::FnWith;

use super::sealed;


#[allow(private_bounds)]
pub trait FnIntoCallbackWith<Ctx, Target, Args: Tuple, ROut, Ud> {
	fn into_callback_with(self, ud: Ud) -> Target;
}

impl<T, Ctx, Target, Args, ROut, Ud> FnIntoCallbackWith<Ctx, Target, Args, ROut, Ud> for T
	where Args: Tuple,
	      FnWith<T, Ud>: sealed::FnIntoCallback<Ctx, Target, Args, ROut>
{
	#[inline(always)]
	fn into_callback_with(self, ud: Ud) -> Target {
		use sealed::FnIntoCallback;
		FnWith(self, ud).into_callback()
	}
}

#[allow(private_bounds)]
pub trait FnMutIntoCallbackWith<Ctx, Target, Args: Tuple, ROut, Ud> {
	fn into_callback_mut_with(self, ud: Ud) -> Target;
}

impl<T, Ctx, Target, Args, ROut, Ud> FnMutIntoCallbackWith<Ctx, Target, Args, ROut, Ud> for T
	where Args: Tuple,
	      FnWith<T, Ud>: sealed::FnMutIntoCallback<Ctx, Target, Args, ROut>
{
	#[inline(always)]
	fn into_callback_mut_with(self, ud: Ud) -> Target {
		use sealed::FnMutIntoCallback;
		FnWith(self, ud).into_callback_mut()
	}
}

#[allow(private_bounds)]
pub trait FnOnceIntoCallbackWith<Ctx, Target, Args: Tuple, ROut, Ud> {
	fn into_callback_once_with(self, ud: Ud) -> Target;
}

impl<T, Ctx, Target, Args, ROut, Ud> FnOnceIntoCallbackWith<Ctx, Target, Args, ROut, Ud> for T
	where Args: Tuple,
	      FnWith<T, Ud>: sealed::FnOnceIntoCallback<Ctx, Target, Args, ROut>
{
	#[inline(always)]
	fn into_callback_once_with(self, ud: Ud) -> Target {
		use sealed::FnOnceIntoCallback;
		FnWith(self, ud).into_callback_once()
	}
}


#[cfg(test)]
mod tests {
	use core::marker::Tuple;
	use crate::scope::Deferred as Scope;
	use crate::util::marker::UdFn;
	use crate::util::marker::UdPtr;
	use super::FnOnceIntoCallbackWith;


	#[test]
	fn into_with() {
		let f = |_: Option<&mut ()>| {};
		accept_once::<(), _, (Option<&mut (/* Ext */)>,), (/* RR */), (/* Ext */)>(f, ());
		let f = |_: Option<&()>| {};
		accept_once::<(), _, _, _, _>(f, ());

		let mut v = alloc::vec::Vec::new();
		accept_once::<(), _, _, _, _>(move |_: Option<&()>| v.push(()), ());

		let v = alloc::vec::Vec::<()>::new();
		accept_once::<(), _, _, _, _>(move |_: Option<&mut u8>| drop(v), 42);
	}


	type CFn<R> = extern "C" fn(UdPtr) -> R;
	type UdCFn<R, F, Ext> = UdFn<CFn<R>, (F, Ext), 0>;

	fn accept_once<CR, F, RArgs: Tuple, RR, Ext>(f: F, ud: Ext) -> CR
		where F: FnOnceIntoCallbackWith<Scope, UdCFn<CR, F, Ext>, RArgs, RR, Ext> {
		let UdFn(f, ud) = FnOnceIntoCallbackWith::into_callback_once_with(f, ud);
		f(ud.as_udptr())
	}
}
