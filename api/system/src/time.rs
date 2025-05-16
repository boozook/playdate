use core::ffi::c_float;
use core::ffi::c_uint;
use core::ptr::null_mut;

pub use sys::ffi::DateTime;

use crate::Api;


pub mod primitive {
	use core::ffi::c_uint;


	#[repr(transparent)]
	#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Seconds<T>(pub T);


	impl Into<Seconds<i32>> for Seconds<c_uint> {
		fn into(self) -> Seconds<i32> { Seconds::<i32>(self.0 as _) }
	}

	impl<T> Seconds<T> {
		pub const fn new(v: T) -> Self { Self(v) }
	}
}

/// Unsigned Seconds.
pub type Seconds = primitive::Seconds<c_uint>;
/// Signed Seconds.
pub type SecondsOffset = primitive::Seconds<i32>;
/// Seconds value as floating-point with microsecond accuracy.
pub type FloatingSeconds = primitive::Seconds<c_float>;


#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Milliseconds(pub c_uint);


/// Time since epoch, `00:00, January 1, 2000`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch {
	pub sec: Seconds,
	pub ms: Milliseconds,
}

impl Epoch {
	pub const fn new(sec: Seconds, ms: Milliseconds) -> Self { Self { sec, ms } }

	pub const fn from_sec(sec: Seconds) -> Self { Self::new(sec, Milliseconds(0)) }

	/// Sum of inner seconds and milliseconds.
	pub const fn to_sec(&self) -> Seconds { Seconds::new(self.sec.0.wrapping_add(self.ms.0 / 1000)) }
}


mod duration {
	use core::time::Duration;
	use super::Epoch;


	impl Epoch {
		pub const fn into_duration(self) -> Duration {
			Duration::new(self.sec.0 as _, 0).saturating_add(Duration::from_millis(self.ms.0 as _))
		}
	}

	impl Into<Duration> for Epoch {
		fn into(self) -> Duration { Self::into_duration(self) }
	}


	impl super::Time {
		#[doc(alias = "sys::ffi::PlaydateSys::getCurrentTimeMilliseconds")]
		#[inline(always)]
		pub fn current_duration(&self) -> Duration { Duration::from_millis(self.current_time().0 as _) }


		/// Returns the time elapsed since [`reset_elapsed`](Self::reset_elapsed) was called.
		#[doc(alias = "sys::ffi::PlaydateSys::getElapsedTime")]
		#[inline(always)]
		pub fn elapsed_duration(&self) -> Duration {
			let secs = unsafe { (self.0.getElapsedTime)() };
			Duration::from_secs_f32(secs)
		}
	}
}


#[derive(Clone, Copy)]
pub struct Time(pub(crate) Api);

impl Default for Time {
	fn default() -> Self { Self(api!(system)) }
}

impl Time {
	pub const fn new(api: Api) -> Self { Self(api) }
}

impl Time {
	// In-game

	/// Returns the number of milliseconds since _some arbitrary point in time_.
	/// This should present a consistent timebase while a game is running,
	/// but the counter will be __disabled when the device is sleeping__.
	#[doc(alias = "sys::ffi::PlaydateSys::getCurrentTimeMilliseconds")]
	pub fn current_time(&self) -> Milliseconds { Milliseconds(unsafe { (self.0.getCurrentTimeMilliseconds)() }) }


	// Epoch

	/// Returns the number of seconds elapsed since midnight (hour 0), January 1, 2000.
	///
	/// See also [`since_epoch_hp`](Self::since_epoch_hp) for _high-precision_ with microsecond accuracy.
	#[doc(alias = "sys::ffi::PlaydateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn since_epoch(&self) -> Epoch {
		Epoch::from_sec(Seconds::new(unsafe { (self.0.getSecondsSinceEpoch)(null_mut()) }))
	}

	/// Returns the number of seconds __and milliseconds__ elapsed since midnight (hour 0), January 1, 2000.
	///
	/// See also [`since_epoch`](Self::since_epoch) if only seconds are needed.
	#[doc(alias = "sys::ffi::PlaydateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn since_epoch_hp(&self) -> Epoch {
		let mut ms = Milliseconds(0);
		let sec = self.since_epoch_ms_to(&mut ms);
		Epoch::new(sec, ms)
	}

	/// Writes the time elapsed since epoch to `time`.
	#[doc(alias = "sys::ffi::PlaydateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn since_epoch_to(&self, time: &mut Epoch) { time.sec = self.since_epoch_ms_to(&mut time.ms); }

	/// Writes microseconds to `ms` and returns seconds elapsed since epoch.
	#[doc(alias = "sys::ffi::PlaydateSys::getSecondsSinceEpoch")]
	#[inline(always)]
	pub fn since_epoch_ms_to(&self, ms: &mut Milliseconds) -> Seconds {
		let sec = unsafe { (self.0.getSecondsSinceEpoch)(&mut ms.0) };
		Seconds::new(sec)
	}


	// Regional, TZ

	/// Returns the system timezone offset from GMT, in seconds.
	#[doc(alias = "sys::ffi::PlaydateSys::getTimezoneOffset")]
	#[inline(always)]
	pub fn timezone_offset(&self) -> SecondsOffset { SecondsOffset::new(unsafe { (self.0.getTimezoneOffset)() }) }

	/// Returns `true` if the user has set the 24-Hour Time preference in the Settings program.
	#[doc(alias = "sys::ffi::PlaydateSys::shouldDisplay24HourTime")]
	#[inline(always)]
	pub fn should_display_24_hour_time(&self) -> bool { unsafe { (self.0.shouldDisplay24HourTime)() == 1 } }


	// Local, convert

	/// Converts the given epoch seconds time to a [`DateTime`].
	#[doc(alias = "sys::ffi::PlaydateSys::convertEpochToDateTime")]
	#[inline(always)]
	pub fn dt_from_epoch(&self, epoch: Seconds) -> DateTime {
		let mut dt = DateTime { year: 0,
		                        month: 0,
		                        day: 0,
		                        weekday: 0,
		                        hour: 0,
		                        minute: 0,
		                        second: 0 };
		self.epoch_to_dt(epoch, &mut dt);
		dt
	}

	/// Converts the given epoch seconds time to a [`DateTime`], writing to `dt`.
	#[doc(alias = "sys::ffi::PlaydateSys::convertEpochToDateTime")]
	#[inline(always)]
	pub fn epoch_to_dt(&self, epoch: Seconds, dt: &mut DateTime) {
		unsafe { (self.0.convertEpochToDateTime)(epoch.0, dt) }
	}

	/// Converts the given [`DateTime`] to an epoch time.
	#[doc(alias = "sys::ffi::PlaydateSys::convertDateTimeToEpoch")]
	pub fn dt_to_epoch(&self, dt: &DateTime) -> Seconds {
		let epoch = unsafe { (self.0.convertDateTimeToEpoch)((dt as *const DateTime).cast_mut()) };
		let _ = dt; // this to prevent earlier drop.
		Seconds::new(epoch)
	}


	// Elapsed

	/// High-resolution timer.
	///
	/// Returns the number of seconds since [`reset_elapsed`](Self::reset_elapsed) was called.
	///
	/// The value is a floating-point number with microsecond accuracy.
	#[doc(alias = "sys::ffi::PlaydateSys::getElapsedTime")]
	#[inline(always)]
	pub fn elapsed(&self) -> FloatingSeconds { FloatingSeconds::new(unsafe { (self.0.getElapsedTime)() }) }


	/// Resets the high-resolution timer.
	#[doc(alias = "sys::ffi::PlaydateSys::resetElapsedTime")]
	#[inline(always)]
	pub fn reset_elapsed(&self) { unsafe { (self.0.resetElapsedTime)() } }
}
