use core::any::Any;
use core::ffi::*;
use core::marker::{PhantomData, Tuple};
use alloc::boxed::Box;
use callback::arg::Adapter;
use callback::{proxy, scope};
use callback::into::FnOnceIntoCallback;

use crate::{time, Api};
use super::serial::CouldStr;


impl time::Time {
	pub const fn server_time(&self) -> ServerTime { ServerTime(self.0) }
}


/// Ctrl server-time callback-subscription.
///
#[derive(Clone, Copy)]
pub struct ServerTime(Api);


impl ServerTime {
	/// Sets `callback` to [`sys::ffi::PlaydateSys::getServerTime`].
	///
	/// Accepts any `FnOnce` functions.
	///
	/// Acceptable options of signature of the `callback` can be
	/// - `(Ok, Err) -> Output`,
	/// - `(Option<Ok>, Option<Err>) -> Output`,
	/// - `(Result<Ok, Err>) -> Output`, \
	///   where `Output` is anything that impls `Into<UpdateDisplayCtrl>`, \
	///   and `Ok` and `Err` can be any combination of `&CStr`, `CString` or `*const c_char`.
	///
	/// __Important:__ \
	/// This "subscription" doesn't support multiple requests running simultaneously.
	/// Any call of [`set`](Self::set) or [`unset`](Self::unset) queues a request setting the callback
	/// (`None` in case of [`unset`](Self::unset)). \
	/// So if you call these functions multiple times,
	/// it will add a network task to the system queue __for each call__,
	/// with the last specified callback being called in the end of each created task.
	#[doc(alias = "sys::ffi::PlaydateSys::getServerTime")]
	#[allow(private_bounds)]
	pub fn set<Args: Tuple, F>(&self, callback: F)
		where F: FnOnceIntoCallback<Scope, CCb, Args, ()> {
		let c = self.0.getServerTime;
		unsafe { c(Some(callback.into_callback_once())) };
	}


	/// Sets `None` to [`sys::ffi::PlaydateSys::getServerTime`], drops previous callback.
	///
	/// __Doesn't cancels a running request,__ just prevents any previously specified callback to be called.
	pub fn unset(&self) {
		let c = self.0.getServerTime;
		unsafe { c(None) };
		Self::clean();
	}

	fn clean() -> bool {
		use crate::callback::storage::Store;

		<<Scope as scope::Scope>::Storage<Self> as Store<Box<dyn Any>>>::remove() ||
		<<Scope as scope::Scope>::Storage<Self> as Store<fn()>>::remove()
	}
}


type CCb = unsafe extern "C" fn(time: *const c_char, err: *const c_char);


struct Scope;

impl scope::Scope for Scope {
	type Adapter<In, Out> = CouldTime<In, Out>;
	type Proxy<C, Params, Res, R, Args, Out> =
		proxy::default::Default<R, Self::Storage<R>, Self::Adapter<Params, Args>>;
	type Storage<Key> = <scope::Unique<ServerTime> as scope::Scope>::Storage<Key>;
}


struct CouldTime<In, Out>(PhantomData<(In, Out)>);

// as-is & inherited by CouldStr
type CParam = *const c_char;
impl<T, E> Adapter for CouldTime<(CParam, CParam), (Result<T, E>,)>
	where CouldStr<(CParam,), (T,)>: Adapter<Params = (CParam,), Args = (T,)>,
	      CouldStr<(CParam,), (E,)>: Adapter<Params = (CParam,), Args = (E,)>
{
	type Params = (CParam, CParam);
	type Args = (Result<T, E>,);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		let res = match (src.0.is_null(), src.1.is_null()) {
			(false, _) => Ok(CouldStr::<(CParam,), (T,)>::convert((src.0,)).0),
			(_, false) => Err(CouldStr::<(CParam,), (E,)>::convert((src.1,)).0),
			_ => Err(CouldStr::<(CParam,), (E,)>::convert((c"unknown err".as_ptr(),)).0),
		};
		(res,)
	}
}


impl<T, E> Adapter for CouldTime<(CParam, CParam), (Option<T>, Option<E>)>
	where CouldStr<(CParam,), (T,)>: Adapter<Params = (CParam,), Args = (T,)>,
	      CouldStr<(CParam,), (E,)>: Adapter<Params = (CParam,), Args = (E,)>
{
	type Params = (CParam, CParam);
	type Args = (Option<T>, Option<E>);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args {
		let t = if src.0.is_null() {
			None
		} else {
			Some(CouldStr::<(CParam,), (T,)>::convert((src.0,)).0)
		};

		let e = if src.1.is_null() {
			None
		} else {
			Some(CouldStr::<(CParam,), (E,)>::convert((src.1,)).0)
		};

		(t, e)
	}
}


// as-is:
impl<T> Adapter for CouldTime<(T, T), (T, T)> {
	type Params = (T, T);
	type Args = (T, T);
	fn convert(src: Self::Params) -> Self::Args { src }
}
