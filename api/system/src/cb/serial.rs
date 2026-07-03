use core::any::Any;
use core::ffi::*;
use core::marker::{PhantomData, Tuple};
use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use callback::{arg, proxy, scope};
use callback::into::FnMutIntoCallback;
use sys::ffi::CString;

use crate::{System, Api};


impl System {
	pub const fn serial(&self) -> Serial { Serial(self.0) }
}


/// Ctrl serial-message callback-subscription.
pub struct Serial(Api);


impl Serial {
	/// Sets `callback` to [`sys::ffi::PlaydateSys::setSerialMessageCallback`].
	///
	/// Acceptable options of signature of the `callback` can be
	/// - `(&CStr) -> Output`
	/// - `(CString) -> Output`
	/// - `(*const c_char) -> Output` \
	///   where `Output` is anything that impls `Into<UpdateDisplayCtrl>`.
	///
	/// Accepts functions:
	/// - any rust function which higher then `FnOnce` (excluding),
	///   such as `FnMut` or `Fn` or `fn` (fn-ptr)
	/// - C function pointer that accepts `*const c_char`.
	#[doc(alias = "sys::ffi::PlaydateSys::setSerialMessageCallback")]
	#[allow(private_bounds)]
	pub fn set<Args: Tuple, F>(&self, callback: F)
		where F: FnMutIntoCallback<Scope, CCb, Args, ()> {
		Self::clean();
		let c = self.0.setSerialMessageCallback;
		unsafe { c(Some(callback.into_callback_mut())) };
	}


	/// Sets `None` to [`sys::ffi::PlaydateSys::setSerialMessageCallback`], drops previously set callback.
	pub fn unset(&self) {
		let c = self.0.setSerialMessageCallback;
		unsafe { c(None) };
		Self::clean();
	}

	fn clean() -> bool {
		use crate::callback::storage::Store;
		<<Scope as scope::Scope>::Storage<Self> as Store<Box<dyn Any>>>::remove() ||
		<<Scope as scope::Scope>::Storage<Self> as Store<fn()>>::remove()
	}
}


struct Scope;

impl scope::Scope for Scope {
	type Adapter<In, Out> = CouldStr<In, Out>;
	type Proxy<C, Params, Res, R, Args, Out> =
		proxy::default::Default<R, Self::Storage<R>, Self::Adapter<Params, Args>>;
	type Storage<Key> = <scope::Unique<Serial> as scope::Scope>::Storage<Key>;
}


type CCb = unsafe extern "C" fn(data: *const c_char);


pub(crate) struct CouldStr<In, Out>(PhantomData<(In, Out)>);

// as-is:
impl<T> arg::Adapter for CouldStr<(T,), (T,)> {
	type Params = (T,);
	type Args = (T,);
	#[inline(always)]
	fn convert(src: Self::Params) -> Self::Args { src }
}

impl<'t> arg::Adapter for CouldStr<(*const c_char,), (&'t CStr,)> {
	type Params = (*const c_char,);
	type Args = (&'t CStr,);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args { (unsafe { CStr::from_ptr(src.0) },) }
}

impl arg::Adapter for CouldStr<(*const c_char,), (CString,)> {
	type Params = (*const c_char,);
	type Args = (CString,);
	#[inline]
	fn convert(src: Self::Params) -> Self::Args { (unsafe { CStr::from_ptr(src.0).to_owned() },) }
}
