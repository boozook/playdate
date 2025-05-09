#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]


#[macro_use]
extern crate alloc;
#[macro_use]
extern crate sys;

use core::ffi::c_float;
use core::ffi::c_int;
use alloc::string::String;


pub mod time;
pub mod ctrl;
// pub mod update;

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


	/// Equivalent to [`sys::ffi::PlaydateSys::setSerialMessageCallback`]
	#[doc(alias = "sys::ffi::PlaydateSys::setSerialMessageCallback")]
	pub fn set_serial_message_callback<F>(&self, callback: Option<F>)
		where F: 'static + FnMut(String) + Sized {
		use core::ffi::c_char;
		use core::ffi::CStr;
		use alloc::boxed::Box;
		use alloc::string::String;


		type FnSerialMessageCallback = Option<unsafe extern "C" fn(data: *const c_char)>;

		static mut STORE: Option<Box<dyn FnMut(String)>> = None;

		pub unsafe extern "C" fn proxy_serial_message_callback<F: FnMut(String)>(data: *const c_char) {
			let data = CStr::from_ptr(data as _).to_string_lossy().into_owned();
			if let Some(ref mut f) = STORE.as_mut() {
				f(data)
			} else {
				// Highly unlikely, mostly impossible case.
				// Should be unreachable, but still possible in case when
				// 0. new callback is None, we have to register it in the System;
				// 1. write callback to `STORE`
				// 2. interrupt, proxy_serial_message_callback called, BOOM!
				// 3. call API::set_serial_message_callback to set our new (None) callback
				// So, see difference in how to store & reg callback at couple lines below.
				panic!("missed callback")
			}
		}


		let f = self.0.setSerialMessageCallback;

		if let Some(callback) = callback {
			let boxed = Box::new(callback);
			// Store firstly, then register it.
			unsafe { STORE = Some(boxed as _) }
			unsafe { f(Some(proxy_serial_message_callback::<F>)) }
		} else {
			// Set firstly, then clear the `STORE`.
			unsafe { f(None) }
			unsafe { STORE = None }
		}
	}

	/// Pauses execution for the given number of milliseconds.
	///
	/// Equivalent to [`sys::ffi::PlaydateSys::delay`]
	#[doc(alias = "sys::ffi::PlaydateSys::delay")]
	#[inline(always)]
	pub fn delay(&self, ms: time::Milliseconds) { unsafe { (self.0.delay)(ms.0) } }


	// TODO:
	// getServerTime
	// sendMirrorData
	// setButtonCallback - ctrl?
	// setUpdateCallback - uh
}
