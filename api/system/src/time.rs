// TODO: invent analog of `std::time::Instant`

use core::ffi::c_float;
use core::ffi::c_uint;
use core::ptr::null_mut;
use core::time::Duration;

pub use sys::ffi::DateTime;

use crate::Api;


pub struct Secs(pub c_uint);
pub struct Milliseconds(pub c_uint);

pub struct Epoch {
	sec: c_uint,
	ms: c_uint,
}

impl Epoch {
	pub const fn new(sec: c_uint, ms: Option<c_uint>) -> Self {
		Self { sec,
		       ms: ms.unwrap_or_default() }
	}
}


#[derive(Clone, Copy)]
pub struct Time(Api);

impl Default for Time {
	fn default() -> Self { Self(api!(system)) }
}

impl Time {
	pub const fn new(api: Api) -> Self { Self(api) }
}

impl Time {
	/// Equivalent to [`sys::ffi::playdateSys::getCurrentTimeMilliseconds`]
	#[doc(alias = "sys::ffi::playdateSys::getCurrentTimeMilliseconds")]
	#[inline(always)]
	pub fn current_time(&self) -> Duration { Duration::from_millis(self.current_time_ms().into()) }

	/// Equivalent to [`sys::ffi::playdateSys::getCurrentTimeMilliseconds`]
	#[doc(alias = "sys::ffi::playdateSys::getCurrentTimeMilliseconds")]
	pub fn current_time_ms(&self) -> c_uint { unsafe { (self.0.getCurrentTimeMilliseconds)() } }

	/// Returns the number of seconds elapsed since midnight (hour 0), January 1, 2000.
	///
	/// See also [`System::seconds_since_epoch_with_ms`].
	///
	/// Equivalent to [`sys::ffi::playdateSys::getSecondsSinceEpoch`]
	#[doc(alias = "sys::ffi::playdateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn seconds_since_epoch(&self) -> c_uint { unsafe { (self.0.getSecondsSinceEpoch)(null_mut()) } }

	/// Returns current time as `(seconds, milliseconds)`,
	/// elapsed since midnight (hour 0), January 1, 2000.
	///
	/// Equivalent to [`sys::ffi::playdateSys::getSecondsSinceEpoch`]
	#[doc(alias = "sys::ffi::playdateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn seconds_since_epoch_with_ms(&self) -> (c_uint, c_uint) {
		let mut millis: c_uint = 0;
		let secs = unsafe { (self.0.getSecondsSinceEpoch)(&mut millis) };
		(secs, millis)
	}

	/// Equivalent to [`sys::ffi::playdateSys::getSecondsSinceEpoch`]
	#[doc(alias = "sys::ffi::playdateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn time_since_epoch(&self) -> Duration {
		let mut millis: c_uint = 0;
		let secs = unsafe { (self.0.getSecondsSinceEpoch)(&mut millis) };
		Duration::new(secs.into(), 0) + Duration::from_millis(millis.into())
	}


	/// Equivalent to [`sys::ffi::playdateSys::getTimezoneOffset`]
	#[doc(alias = "sys::ffi::playdateSys::getTimezoneOffset")]
	#[inline(always)]
	pub fn timezone_offset(&self) -> i32 { unsafe { (self.0.getTimezoneOffset)() } }


	/// Equivalent to [`sys::ffi::playdateSys::shouldDisplay24HourTime`]
	#[doc(alias = "sys::ffi::playdateSys::shouldDisplay24HourTime")]
	#[inline(always)]
	pub fn should_display_24_hour_time(&self) -> bool { unsafe { (self.0.shouldDisplay24HourTime)() == 1 } }

	/// Equivalent to [`sys::ffi::playdateSys::convertEpochToDateTime`]
	#[doc(alias = "sys::ffi::playdateSys::convertEpochToDateTime")]
	#[inline(always)]
	pub fn convert_epoch_to_date_time(&self, epoch: u32) -> DateTime {
		let mut dt = DateTime { year: 0,
		                        month: 0,
		                        day: 0,
		                        weekday: 0,
		                        hour: 0,
		                        minute: 0,
		                        second: 0 };
		self.convert_epoch_to_date_time_to(epoch, &mut dt);
		dt
	}

	/// Equivalent to [`sys::ffi::playdateSys::convertEpochToDateTime`]
	#[doc(alias = "sys::ffi::playdateSys::convertEpochToDateTime")]
	#[inline(always)]
	pub fn convert_epoch_to_date_time_to(&self, epoch: u32, dt: &mut DateTime) {
		unsafe { (self.0.convertEpochToDateTime)(epoch, dt) }
	}

	/// Equivalent to [`sys::ffi::playdateSys::convertDateTimeToEpoch`]
	#[doc(alias = "sys::ffi::playdateSys::convertDateTimeToEpoch")]
	pub fn convert_date_time_to_epoch(&self, dt: &DateTime) -> u32 {
		let epoch = unsafe { (self.0.convertDateTimeToEpoch)(dt as *const _ as *mut _) };
		let _ = dt; // this to prevent earlier drop.
		epoch
	}


	/// Returns the number of __seconds__ since [`reset_elapsed_time`] was called.
	///
	/// The value is a floating-point number with microsecond accuracy.
	///
	/// Equivalent to [`sys::ffi::playdateSys::getElapsedTime`]
	#[doc(alias = "sys::ffi::playdateSys::getElapsedTime")]
	#[inline(always)]
	pub fn elapsed_time_secs(&self) -> c_float { unsafe { (self.0.getElapsedTime)() } }

	/// Equivalent to [`sys::ffi::playdateSys::getElapsedTime`]
	#[doc(alias = "sys::ffi::playdateSys::getElapsedTime")]
	#[inline(always)]
	pub fn elapsed_time(&self) -> Duration {
		let secs = unsafe { (self.0.getElapsedTime)() };
		Duration::from_secs_f32(secs)
	}

	/// Equivalent to [`sys::ffi::playdateSys::resetElapsedTime`]
	#[doc(alias = "sys::ffi::playdateSys::resetElapsedTime")]
	#[inline(always)]
	pub fn reset_elapsed_time(&self) { unsafe { (self.0.resetElapsedTime)() } }
}


pub trait DateTimeExt {
	fn to_epoch(&self) -> u32;
	fn from_epoch(epoch: u32) -> Self;

	fn from_epoch_to(dt: &mut Self, epoch: u32);
}


// impl DateTimeExt for DateTime {
// 	fn to_epoch(&self) -> u32 {
// 		let f = super::api::Default::default().convert_date_time_to_epoch();
// 		unsafe { f(self as *const _ as *mut _) }
// 	}

// 	fn from_epoch(epoch: u32) -> Self {
// 		let mut dt = DateTime { year: 0,
// 		                        month: 0,
// 		                        day: 0,
// 		                        weekday: 0,
// 		                        hour: 0,
// 		                        minute: 0,
// 		                        second: 0 };
// 		let f = super::api::Default::default().convert_epoch_to_date_time();
// 		unsafe { f(epoch, &mut dt) };
// 		dt
// 	}

// 	fn from_epoch_to(dt: &mut Self, epoch: u32) {
// 		let f = super::api::Default::default().convert_epoch_to_date_time();
// 		unsafe { f(epoch, dt) }
// 	}
// }
