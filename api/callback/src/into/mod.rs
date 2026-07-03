use core::marker::Tuple;


mod sealed;
mod with;

pub use with::*;


#[allow(private_bounds)]
pub trait FnIntoCallback<Ctx, Target, Args: Tuple, ROut>:
	sealed::FnIntoCallback<Ctx, Target, Args, ROut> {
	fn into_callback(self) -> Target;
}

impl<T, Ctx, Target, Args, ROut> FnIntoCallback<Ctx, Target, Args, ROut> for T
	where T: sealed::FnIntoCallback<Ctx, Target, Args, ROut>,
	      Args: Tuple
{
	#[inline(always)]
	fn into_callback(self) -> Target {
		<T as sealed::FnIntoCallback<Ctx, Target, Args, ROut>>::into_callback(self)
	}
}

#[allow(private_bounds)]
pub trait FnMutIntoCallback<Ctx, Target, Args: Tuple, ROut>:
	sealed::FnMutIntoCallback<Ctx, Target, Args, ROut> {
	fn into_callback_mut(self) -> Target;
}

impl<T, Ctx, Target, Args, ROut> FnMutIntoCallback<Ctx, Target, Args, ROut> for T
	where T: sealed::FnMutIntoCallback<Ctx, Target, Args, ROut>,
	      Args: Tuple
{
	#[inline(always)]
	fn into_callback_mut(self) -> Target {
		<T as sealed::FnMutIntoCallback<Ctx, Target, Args, ROut>>::into_callback_mut(self)
	}
}

#[allow(private_bounds)]
pub trait FnOnceIntoCallback<Ctx, Target, Args: Tuple, ROut>:
	sealed::FnOnceIntoCallback<Ctx, Target, Args, ROut> {
	fn into_callback_once(self) -> Target;
}

impl<T, Ctx, Target, Args, ROut> FnOnceIntoCallback<Ctx, Target, Args, ROut> for T
	where T: sealed::FnOnceIntoCallback<Ctx, Target, Args, ROut>,
	      Args: Tuple
{
	#[inline(always)]
	fn into_callback_once(self) -> Target {
		<T as sealed::FnOnceIntoCallback<Ctx, Target, Args, ROut>>::into_callback_once(self)
	}
}


#[cfg(test)]
mod tests {
	use core::marker::Tuple;
	use crate::scope::Deferred as Scope;
	use super::FnOnceIntoCallback;


	#[test]
	fn into() {
		let f = |_: (), _: ()| {};
		accept_once::<(), (), (), _, _>(f, (), ());

		let mut v = alloc::vec::Vec::new();
		accept_once::<(), (), (), _, _>(move |_: (), _: ()| v.push(()), (), ());

		let v = alloc::vec::Vec::<()>::new();
		accept_once::<(), (), (), _, _>(move |_: (), _: ()| drop(v), (), ());
	}


	type CFn<A, B, R> = unsafe extern "C" fn(A, B) -> R;
	fn accept_once<A, B, CR, RArgs: Tuple, RR>(f: impl FnOnceIntoCallback<Scope, CFn<A, B, CR>, RArgs, RR>,
	                                           a: A,
	                                           b: B) {
		// here I have to specify concrete trait,
		// but outside of this module there is only "top-level" one and simple dot-call is possible,
		// e.g.: f.into_callback_once()
		let f = FnOnceIntoCallback::into_callback_once(f);
		unsafe { f(a, b) };
	}
}
