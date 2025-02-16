#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate sys;

pub mod api;
pub mod buttons;
pub mod peripherals;

pub mod accelerometer {
	use core::ffi::c_float;
	use crate::peripherals::Accelerometer;

	/// Enables accelerometer only.
	///
	/// By default, the accelerometer is disabled to save (a small amount of) power.
	///
	/// To use a peripheral, it must first be enabled via this function.
	///
	/// Accelerometer data is not available until the next update cycle after it’s enabled.
	///
	/// This function is shorthand for [`Accelerometer::enable`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`].
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	#[inline(always)]
	pub fn enable() { Accelerometer::Default().enable() }

	/// ⚠️ Disables all peripherals including accelerometer.
	///
	/// Currently it doesn't matter because there's just one peripheral - accelerometer.
	///
	/// This function is shorthand for [`Accelerometer::disable`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`].
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	#[inline(always)]
	pub fn disable() { Accelerometer::Default().disable() }

	/// Returns `(x, y, z)` accelerometer data.
	///
	/// This function is shorthand for [`Accelerometer::get`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getAccelerometer`].
	#[doc(alias = "sys::ffi::playdate_sys::getAccelerometer")]
	#[inline(always)]
	pub fn get() -> (c_float, c_float, c_float) { Accelerometer::Default().get() }

	/// Gets accelerometer data directly to `x`, `y` and `z`.
	///
	/// This function is shorthand for [`Accelerometer::get_to`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getAccelerometer`].
	#[doc(alias = "sys::ffi::playdate_sys::getAccelerometer")]
	#[inline(always)]
	pub fn get_to(outx: &mut c_float, outy: &mut c_float, outz: &mut c_float) { Accelerometer::Default().get_to(outx, outy, outz) }
}

pub mod crank {
	use core::ffi::c_float;
	use crate::peripherals::Crank;

	/// Returns boolean indicating whether or not the crank is folded into the unit.
	///
	/// This function is shorthand for [`Crank::docked`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::isCrankDocked`].
	#[doc(alias = "sys::ffi::playdate_sys::isCrankDocked")]
	#[inline(always)]
	pub fn docked() -> bool { Crank::Default().docked() }

	/// Returns the current position of the crank, in the range 0-360.
	/// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
	///
	/// This function is shorthand for [`Crank::angle`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getCrankAngle`].
	#[doc(alias = "sys::ffi::playdate_sys::getCrankAngle")]
	#[inline(always)]
	pub fn angle() -> c_float { Crank::Default().angle() }

	/// Returns the angle change of the crank since the last time this function was called.
	/// Negative values are anti-clockwise.
	///
	/// This function is shorthand for [`Crank::change`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getCrankChange`].
	#[doc(alias = "sys::ffi::playdate_sys::getCrankChange")]
	#[inline(always)]
	pub fn change() -> c_float { Crank::Default().change() }

	/// Returns the previous value for this setting.
	///
	/// This function is shorthand for [`Crank::disable_sounds`],
	/// using default ZST end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setCrankSoundsDisabled`].
	#[doc(alias = "sys::ffi::playdate_sys::setCrankSoundsDisabled")]
	#[inline(always)]
	pub fn disable_sounds(disable: bool) -> bool { Crank::Default().disable_sounds(disable) }
}
