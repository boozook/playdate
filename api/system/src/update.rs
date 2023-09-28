use core::ffi::c_void;
use core::ffi::c_int;
use core::marker::PhantomData;
use core::pin::Pin;
use alloc::boxed::Box;

use crate::api;
use crate::System;


/// Pinned wrapper around a function and user data.
///
/// On drop, automatically resets system registered update handler.
pub struct Handler<'t, F, U>(Option<Pin<Box<(F, U)>>>, PhantomData<&'t ()>);

impl<'t, F, U> Drop for Handler<'t, F, U> {
	fn drop(&mut self) {
		let get_fn = || sys::api_opt!(system.setUpdateCallback);
		if self.0.is_some() {
			if let Some(f) = get_fn() {
				unsafe {
					f(None, core::ptr::null_mut());
				}
			}
		}
	}
}


impl<Api: api::Api> System<Api> {
	/// Internal update callback proxy function.
	unsafe extern "C" fn proxy<UD, Fn: FnMut(&mut UD) -> UpdateCtrl>(fn_ud: *mut c_void) -> c_int {
		if let Some((callback, userdata)) = (fn_ud as *mut (Fn, UD)).as_mut() {
			callback(userdata).into()
		} else {
			panic!("user callback missed");
		}
	}


	/// Takes __any__ function and `userdata`,
	/// registers callback in the system and
	/// returns this function with userdata wrapped into the [`Handler`] with [`Pin`] inside.
	///
	/// For register a fn-ptr you could better use [`set_update_callback_static`].
	///
	/// Safety is ensured by [`Handler`],
	/// that resets the system registered update handler when drop.
	///
	/// Wrapping [`sys::ffi::playdate_sys::setUpdateCallback`]
	#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
	#[must_use = "Update handler will be unregistered when Handler dropped"]
	pub fn set_update_callback<'u, U, F>(&self, on_update: F, userdata: U) -> Handler<'u, F, U>
		where U: 'u,
		      F: 'u + FnMut(&mut U) -> UpdateCtrl {
		let f = self.0.set_update_callback();
		let mut userdata = Box::pin((on_update, userdata));
		let ptr = unsafe { userdata.as_mut().get_unchecked_mut() } as *mut _ as *mut c_void;
		unsafe { f(Some(Self::proxy::<U, F>), ptr) };
		Handler(userdata.into(), PhantomData)
	}

	/// Consumes and __leaks__ an __any__ function with `userdata` into the `Box`,
	/// registers callback in the system.
	///
	/// For register a fn-ptr you could better use [`set_update_callback_static`].
	///
	/// __Safety is guaranteed by the caller.__
	///
	/// See also [`System::set_update_callback`], it prevents leaks and more safe.
	///
	/// Wrapping [`sys::ffi::playdate_sys::setUpdateCallback`]
	#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
	pub fn set_update_callback_boxed<'u, U, F>(&self, on_update: F, userdata: U)
		where U: 'u,
		      F: 'u + FnMut(&mut U) -> UpdateCtrl {
		let f = self.0.set_update_callback();
		let ptr = Box::into_raw(Box::new((on_update, userdata)));
		unsafe { f(Some(Self::proxy::<U, F>), ptr as *mut _) };
	}


	/// Consumes and __leaks__ function `on_update` and `userdata`, wraps it into the `Box`,
	/// then registers callback.
	///
	/// See also [`System::set_update_callback`], it prevents leaks and more safe.
	///
	/// Wrapping [`sys::ffi::playdate_sys::setUpdateCallback`]
	#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
	pub fn set_update_callback_static<U: 'static>(&self,
	                                              on_update: Option<fn(userdata: &mut U) -> UpdateCtrl>,
	                                              userdata: U) {
		unsafe extern "C" fn proxy<UD: 'static>(fn_ud: *mut c_void) -> c_int {
			if let Some((callback, userdata)) = (fn_ud as *mut (fn(userdata: &mut UD) -> UpdateCtrl, UD)).as_mut() {
				callback(userdata).into()
			} else {
				panic!("user callback missed");
			}
		}

		let f = self.0.set_update_callback();
		if let Some(callback) = on_update {
			let ptr = Box::into_raw(Box::new((callback, userdata)));
			unsafe { f(Some(proxy::<U>), ptr as *mut _) };
		} else {
			unsafe { f(None, core::ptr::null_mut()) };
		}
	}

	/// Executes `handler`'s [`Update::set_update_handler_with`] with this inner api.
	///
	/// Wrapping [`sys::ffi::playdate_sys::setUpdateCallback`]
	#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
	#[inline(always)]
	pub fn set_update_handler<'t, U: 'static + Update>(&'t self, handler: Option<&'static mut U>)
		where &'t Api: api::Api {
		if let Some(handler) = handler {
			handler.set_update_handler_with(&self.0)
		} else {
			let f = self.0.set_update_callback();
			unsafe { f(None, core::ptr::null_mut()) };
		}
	}
}


/// Implementable stateful update handler
/// with default implementation for adapter and register functions.
pub trait Update: Sized {
	fn update(&mut self) -> UpdateCtrl;

	/// Register a callback function [`Self::update`] in the system,
	/// using [`Default`](api::Default) `api`.
	///
	/// See also [`Update::set_update_handler_with`] and [`System::set_update_handler`].
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setUpdateCallback`]
	#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
	#[inline(always)]
	fn set_update_handler(&'static mut self) { self.set_update_handler_with(api::Default) }

	/// Register a callback function [`Self::update`] in the system,
	/// using given `api`.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setUpdateCallback`]
	#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
	fn set_update_handler_with<Api: api::Api>(&'static mut self, api: Api) {
		let f = api.set_update_callback();
		unsafe { f(Some(Self::update_proxy), self as *mut Self as *mut _) };
	}


	/// Overridable update callback adapter.
	#[doc(hidden)]
	unsafe extern "C" fn update_proxy(handler: *mut c_void) -> c_int {
		if let Some(handler) = (handler as *mut Self).as_mut() {
			Self::update(handler).into()
		} else {
			panic!("user callback missed");
		}
	}
}


#[repr(i32)]
pub enum UpdateCtrl {
	Stop = 0,
	Continue = 1,
}

impl UpdateCtrl {
	pub const fn into(self) -> c_int { self as _ }
}

impl From<bool> for UpdateCtrl {
	fn from(value: bool) -> Self {
		if value {
			Self::Continue
		} else {
			Self::Stop
		}
	}
}

impl<T, E> From<Result<T, E>> for UpdateCtrl {
	fn from(res: Result<T, E>) -> Self {
		if res.is_ok() {
			Self::Continue
		} else {
			Self::Stop
		}
	}
}

#[cfg(feature = "try-trait-v2")]
mod impl_trait_v2 {
	use super::*;
	use core::convert::Infallible;
	use core::ops::FromResidual;

	impl<E> FromResidual<Result<Infallible, E>> for UpdateCtrl {
		fn from_residual(residual: Result<Infallible, E>) -> Self {
			if residual.is_ok() {
				Self::Continue
			} else {
				Self::Stop
			}
		}
	}
}
