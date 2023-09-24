#![cfg_attr(not(test), no_std)]
extern crate sys;
extern crate alloc;

use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::time::Duration;
use core::pin::Pin;
use alloc::boxed::Box;

pub mod time;
pub mod lang;

pub use time::*;
pub use lang::*;


#[derive(Debug, Clone, Copy)]
pub struct System<Api = api::Default>(Api);

impl System<api::Default> {
	/// Creates default [`System`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for System<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> System<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> System<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}


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
	unsafe extern "C" fn proxy<UD, Fn: FnMut(&mut UD) -> bool>(fn_ud: *mut c_void) -> c_int {
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
		      F: 'u + FnMut(&mut U) -> bool {
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
		      F: 'u + FnMut(&mut U) -> bool {
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
	                                              on_update: Option<fn(userdata: &mut U) -> bool>,
	                                              userdata: U) {
		unsafe extern "C" fn proxy<UD: 'static>(fn_ud: *mut c_void) -> c_int {
			if let Some((callback, userdata)) = (fn_ud as *mut (fn(userdata: &mut UD) -> bool, UD)).as_mut() {
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
}


impl<Api: api::Api> System<Api> {
	/// Equivalent to [`sys::ffi::playdate_sys::getLanguage`]
	#[doc(alias = "sys::ffi::playdate_sys::getLanguage")]
	#[inline(always)]
	pub fn language(&self) -> PDLanguage {
		let f = self.0.get_language();
		unsafe { f() }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getCurrentTimeMilliseconds`]
	#[doc(alias = "sys::ffi::playdate_sys::getCurrentTimeMilliseconds")]
	#[inline(always)]
	pub fn current_time_milliseconds(&self) -> Duration {
		let f = self.0.get_current_time_milliseconds();
		let t = unsafe { f() };
		Duration::from_millis(t.into())
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getSecondsSinceEpoch`]
	#[doc(alias = "sys::ffi::playdate_sys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn seconds_since_epoch(&self) -> Duration {
		let f = self.0.get_seconds_since_epoch();
		let mut millis: c_uint = 0;
		let secs = unsafe { f(&mut millis) };
		Duration::new(secs.into(), 0) + Duration::from_millis(millis.into())
	}

	/// Equivalent to [`sys::ffi::playdate_sys::drawFPS`]
	#[doc(alias = "sys::ffi::playdate_sys::drawFPS")]
	#[inline(always)]
	pub fn draw_fps(&self, x: c_int, y: c_int) {
		let f = self.0.draw_fps();
		unsafe { f(x, y) }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getFlipped`]
	#[doc(alias = "sys::ffi::playdate_sys::getFlipped")]
	#[inline(always)]
	pub fn flipped(&self) -> bool {
		let f = self.0.get_flipped();
		unsafe { f() == 1 }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::setAutoLockDisabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setAutoLockDisabled")]
	#[inline(always)]
	pub fn set_auto_lock_disabled(&self, disable: bool) {
		let f = self.0.set_auto_lock_disabled();
		unsafe { f(disable as _) }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getReduceFlashing`]
	#[doc(alias = "sys::ffi::playdate_sys::getReduceFlashing")]
	#[inline(always)]
	pub fn reduce_flashing(&self) -> bool {
		let f = self.0.get_reduce_flashing();
		unsafe { f() == 1 }
	}

	// TODO: invent analog of `std::time::Instant`

	/// Equivalent to [`sys::ffi::playdate_sys::getElapsedTime`]
	#[doc(alias = "sys::ffi::playdate_sys::getElapsedTime")]
	#[inline(always)]
	pub fn elapsed_time(&self) -> Duration {
		let f = self.0.get_elapsed_time();
		let secs = unsafe { f() };
		Duration::from_secs_f32(secs)
	}

	/// Equivalent to [`sys::ffi::playdate_sys::resetElapsedTime`]
	#[doc(alias = "sys::ffi::playdate_sys::resetElapsedTime")]
	#[inline(always)]
	pub fn reset_elapsed_time(&self) {
		let f = self.0.reset_elapsed_time();
		unsafe { f() }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getBatteryPercentage`]
	#[doc(alias = "sys::ffi::playdate_sys::getBatteryPercentage")]
	#[inline(always)]
	pub fn battery_percentage(&self) -> c_float {
		let f = self.0.get_battery_percentage();
		unsafe { f() }
	}


	/// Equivalent to [`sys::ffi::playdate_sys::getBatteryVoltage`]
	#[doc(alias = "sys::ffi::playdate_sys::getBatteryVoltage")]
	#[inline(always)]
	pub fn battery_voltage(&self) -> c_float {
		let f = self.0.get_battery_voltage();
		unsafe { f() }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getTimezoneOffset`]
	#[doc(alias = "sys::ffi::playdate_sys::getTimezoneOffset")]
	#[inline(always)]
	pub fn timezone_offset(&self) -> i32 {
		let f = self.0.get_timezone_offset();
		unsafe { f() }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::shouldDisplay24HourTime`]
	#[doc(alias = "sys::ffi::playdate_sys::shouldDisplay24HourTime")]
	#[inline(always)]
	pub fn should_display_24_hour_time(&self) -> bool {
		let f = self.0.should_display_24_hour_time();
		unsafe { f() == 1 }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::convertEpochToDateTime`]
	#[doc(alias = "sys::ffi::playdate_sys::convertEpochToDateTime")]
	#[inline(always)]
	pub fn convert_epoch_to_date_time(&self, epoch: u32) -> PDDateTime {
		let mut dt = PDDateTime { year: 0,
		                          month: 0,
		                          day: 0,
		                          weekday: 0,
		                          hour: 0,
		                          minute: 0,
		                          second: 0 };
		self.convert_epoch_to_date_time_to(epoch, &mut dt);
		dt
	}

	/// Equivalent to [`sys::ffi::playdate_sys::convertEpochToDateTime`]
	#[doc(alias = "sys::ffi::playdate_sys::convertEpochToDateTime")]
	#[inline(always)]
	pub fn convert_epoch_to_date_time_to(&self, epoch: u32, dt: &mut PDDateTime) {
		let f = self.0.convert_epoch_to_date_time();
		unsafe { f(epoch, dt) }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::convertDateTimeToEpoch`]
	#[doc(alias = "sys::ffi::playdate_sys::convertDateTimeToEpoch")]
	pub fn convert_date_time_to_epoch(&self, dt: &PDDateTime) -> u32 {
		let f = self.0.convert_date_time_to_epoch();
		let epoch = unsafe { f(dt as *const _ as *mut _) };
		let _ = dt; // this to prevent earlier drop.
		epoch
	}
}


pub mod api {
	use core::ffi::c_float;
	use core::ffi::c_int;
	use core::ffi::c_uint;
	use core::ffi::c_void;

	use sys::ffi::PDCallbackFunction;
	use sys::ffi::PDDateTime;
	use sys::ffi::PDLanguage;


	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;

	impl Api for Default {}


	pub trait Api {
		/// Equivalent to [`sys::ffi::playdate_sys::getLanguage`]
		#[doc(alias = "sys::ffi::playdate_sys::getLanguage")]
		#[inline(always)]
		fn get_language(&self) -> unsafe extern "C" fn() -> PDLanguage { *sys::api!(system.getLanguage) }

		/// Equivalent to [`sys::ffi::playdate_sys::getCurrentTimeMilliseconds`]
		#[doc(alias = "sys::ffi::playdate_sys::getCurrentTimeMilliseconds")]
		#[inline(always)]
		fn get_current_time_milliseconds(&self) -> unsafe extern "C" fn() -> c_uint {
			*sys::api!(system.getCurrentTimeMilliseconds)
		}

		/// Equivalent to [`sys::ffi::playdate_sys::getSecondsSinceEpoch`]
		#[doc(alias = "sys::ffi::playdate_sys::getSecondsSinceEpoch")]
		#[inline(always)]
		fn get_seconds_since_epoch(&self) -> unsafe extern "C" fn(milliseconds: *mut c_uint) -> c_uint {
			*sys::api!(system.getSecondsSinceEpoch)
		}

		/// Equivalent to [`sys::ffi::playdate_sys::drawFPS`]
		#[doc(alias = "sys::ffi::playdate_sys::drawFPS")]
		#[inline(always)]
		fn draw_fps(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { *sys::api!(system.drawFPS) }

		/// Equivalent to [`sys::ffi::playdate_sys::setUpdateCallback`]
		#[doc(alias = "sys::ffi::playdate_sys::setUpdateCallback")]
		#[inline(always)]
		fn set_update_callback(&self) -> unsafe extern "C" fn(update: PDCallbackFunction, userdata: *mut c_void) {
			*sys::api!(system.setUpdateCallback)
		}

		/// Equivalent to [`sys::ffi::playdate_sys::getFlipped`]
		#[doc(alias = "sys::ffi::playdate_sys::getFlipped")]
		#[inline(always)]
		fn get_flipped(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(system.getFlipped) }

		/// Equivalent to [`sys::ffi::playdate_sys::setAutoLockDisabled`]
		#[doc(alias = "sys::ffi::playdate_sys::setAutoLockDisabled")]
		#[inline(always)]
		fn set_auto_lock_disabled(&self) -> unsafe extern "C" fn(disable: c_int) {
			*sys::api!(system.setAutoLockDisabled)
		}

		/// Equivalent to [`sys::ffi::playdate_sys::getReduceFlashing`]
		#[doc(alias = "sys::ffi::playdate_sys::getReduceFlashing")]
		#[inline(always)]
		fn get_reduce_flashing(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(system.getReduceFlashing) }

		/// Equivalent to [`sys::ffi::playdate_sys::getElapsedTime`]
		#[doc(alias = "sys::ffi::playdate_sys::getElapsedTime")]
		#[inline(always)]
		fn get_elapsed_time(&self) -> unsafe extern "C" fn() -> c_float { *sys::api!(system.getElapsedTime) }

		/// Equivalent to [`sys::ffi::playdate_sys::resetElapsedTime`]
		#[doc(alias = "sys::ffi::playdate_sys::resetElapsedTime")]
		#[inline(always)]
		fn reset_elapsed_time(&self) -> unsafe extern "C" fn() { *sys::api!(system.resetElapsedTime) }

		/// Equivalent to [`sys::ffi::playdate_sys::getBatteryPercentage`]
		#[doc(alias = "sys::ffi::playdate_sys::getBatteryPercentage")]
		#[inline(always)]
		fn get_battery_percentage(&self) -> unsafe extern "C" fn() -> c_float {
			*sys::api!(system.getBatteryPercentage)
		}


		/// Equivalent to [`sys::ffi::playdate_sys::getBatteryVoltage`]
		#[doc(alias = "sys::ffi::playdate_sys::getBatteryVoltage")]
		#[inline(always)]
		fn get_battery_voltage(&self) -> unsafe extern "C" fn() -> c_float { *sys::api!(system.getBatteryVoltage) }

		/// Equivalent to [`sys::ffi::playdate_sys::getTimezoneOffset`]
		#[doc(alias = "sys::ffi::playdate_sys::getTimezoneOffset")]
		#[inline(always)]
		fn get_timezone_offset(&self) -> unsafe extern "C" fn() -> i32 { *sys::api!(system.getTimezoneOffset) }

		/// Equivalent to [`sys::ffi::playdate_sys::shouldDisplay24HourTime`]
		#[doc(alias = "sys::ffi::playdate_sys::shouldDisplay24HourTime")]
		#[inline(always)]
		fn should_display_24_hour_time(&self) -> unsafe extern "C" fn() -> c_int {
			*sys::api!(system.shouldDisplay24HourTime)
		}

		/// Equivalent to [`sys::ffi::playdate_sys::convertEpochToDateTime`]
		#[doc(alias = "sys::ffi::playdate_sys::convertEpochToDateTime")]
		#[inline(always)]
		fn convert_epoch_to_date_time(&self) -> unsafe extern "C" fn(epoch: u32, datetime: *mut PDDateTime) {
			*sys::api!(system.convertEpochToDateTime)
		}

		/// Equivalent to [`sys::ffi::playdate_sys::convertDateTimeToEpoch`]
		#[doc(alias = "sys::ffi::playdate_sys::convertDateTimeToEpoch")]
		#[inline(always)]
		fn convert_date_time_to_epoch(&self) -> unsafe extern "C" fn(datetime: *mut PDDateTime) -> u32 {
			*sys::api!(system.convertDateTimeToEpoch)
		}
	}
}
