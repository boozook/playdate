#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]
#![cfg_attr(feature = "callback", feature(tuple_trait, min_specialization))]
// for cont- compile-time tests:
#![cfg_attr(all(debug_assertions, feature = "callback"),
            feature(core_intrinsics),
            allow(internal_features))]


#[macro_use]
extern crate alloc;
#[macro_use]
extern crate sys;

#[cfg(feature = "callback")]
extern crate callback;


use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::CStr;


pub mod time;
pub mod ctrl;
// pub mod update;
mod cb;

pub mod prelude {
	pub use crate::System;
	pub use crate::time::*;
	pub use crate::ctrl::buttons::*;
	// pub use crate::update::*;
}


type Api = &'static sys::ffi::PlaydateSys;


#[derive(Clone, Copy)]
pub struct System(Api);

impl Default for System {
	fn default() -> Self { Self(api!(system)) }
}

impl System {
	pub const fn new(api: Api) -> Self { Self(api) }

	pub const fn time(&self) -> time::Time { time::Time::new(self.0) }
	pub const fn input(&self) -> ctrl::api::Ctrl { ctrl::api::Ctrl::new(self.0) }
}


impl System {
	/// Returns the current language of the system.
	#[doc(alias = "sys::ffi::PlaydateSys::getLanguage")]
	#[inline(always)]
	pub fn language(&self) -> sys::ffi::Language { unsafe { (self.0.getLanguage)() } }

	/// Calculates the current frames per second and draws that value at `x, y`.
	#[doc(alias = "sys::ffi::PlaydateSys::drawFPS")]
	#[inline(always)]
	pub fn draw_fps(&self, x: c_int, y: c_int) { unsafe { (self.0.drawFPS)(x, y) } }

	/// Returns `true` if the global "flipped" system setting is set.
	#[doc(alias = "sys::ffi::PlaydateSys::getFlipped")]
	#[inline(always)]
	pub fn flipped(&self) -> bool { unsafe { (self.0.getFlipped)() != 0 } }

	/// Disables or enables the 3 minute auto lock feature.
	/// When called, the timer is reset to 3 minutes.
	#[doc(alias = "sys::ffi::PlaydateSys::setAutoLockDisabled")]
	#[inline(always)]
	pub fn set_auto_lock(&self, disable: bool) { unsafe { (self.0.setAutoLockDisabled)(disable as _) } }

	/// Returns `true` if the global "reduce flashing" system setting is set.
	#[doc(alias = "sys::ffi::PlaydateSys::getReduceFlashing")]
	#[inline(always)]
	pub fn reduce_flashing(&self) -> bool { unsafe { (self.0.getReduceFlashing)() == 1 } }


	/// Returns a value from `0-100` denoting the current level of battery charge.
	/// `0` = empty;
	/// `100` = full.
	#[doc(alias = "sys::ffi::PlaydateSys::getBatteryPercentage")]
	#[inline(always)]
	pub fn battery_percentage(&self) -> c_float { unsafe { (self.0.getBatteryPercentage)() } }


	/// Returns the battery’s current voltage level.
	#[doc(alias = "sys::ffi::PlaydateSys::getBatteryVoltage")]
	#[inline(always)]
	pub fn battery_voltage(&self) -> c_float { unsafe { (self.0.getBatteryVoltage)() } }


	/// Pauses execution for the given number of milliseconds.
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::delay`]
	#[doc(alias = "sys::ffi::PlaydateSys::delay")]
	#[inline(always)]
	pub fn delay(&self, ms: time::Milliseconds) { unsafe { (self.0.delay)(ms.0) } }


	/// Reinitializes the Playdate runtime and restarts the currently running program.
	/// The given `launch_args` string will be available after restart via [`launch_args`](Self::launch_args).
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::restartGame`]
	#[doc(alias = "sys::ffi::PlaydateSys::restartGame")]
	#[cold]
	pub fn restart(&self, launch_args: &CStr) { unsafe { (self.0.restartGame)(launch_args.as_ptr()) } }

	/// Returns the string passed in as an argument at launch time,
	/// either via the command line when launching the simulator,
	/// the device console run command, or the above [`restart`](Self::restart) function.
	///
	/// If outpath is not NULL, the path of the currently loaded game is returned in it.
	///
	/// Calls to [`sys::ffi::PlaydateSys::getLaunchArgs`]
	#[doc(alias = "sys::ffi::PlaydateSys::getLaunchArgs")]
	#[inline]
	// TODO: What's the hell is `outpath`?!
	pub fn launch_args(&self, mut callback: impl FnMut(Option<&CStr>)) {
		let p = unsafe { (self.0.getLaunchArgs)(core::ptr::null_mut()) };
		let mut s = if p.is_null() {
			None
		} else {
			Some(unsafe { CStr::from_ptr(p) })
		};
		callback(s);
		let _ = &mut s;
	}


	// TODO: sendMirrorData
}
