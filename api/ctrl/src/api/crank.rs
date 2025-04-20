use core::ffi::c_float;

use crate::Api;


/// Crank
#[derive(Clone, Copy)]
pub struct Crank(pub(crate) Api);

impl Default for Crank {
	fn default() -> Self { Self(api!(system)) }
}

impl Crank {
	pub const fn new(api: Api) -> Self { Self(api) }
}


impl Crank {
	/// Returns boolean indicating whether or not the crank is folded into the unit.
	#[doc(alias = "sys::ffi::PlaydateSys::isCrankDocked")]
	#[inline(always)]
	pub fn docked(&self) -> bool { unsafe { (self.0.isCrankDocked)() == 1 } }

	/// Returns the current position of the crank, in the range 0-360.
	/// Zero is pointing up, and the value increases as the crank moves clockwise,
	///                               as viewed from the right side of the device.
	#[doc(alias = "sys::ffi::PlaydateSys::getCrankAngle")]
	#[inline(always)]
	pub fn angle(&self) -> c_float { unsafe { (self.0.getCrankAngle)() } }

	/// Returns the angle change of the crank since the last time this function was called.
	/// Negative values are anti-clockwise.
	#[doc(alias = "sys::ffi::PlaydateSys::getCrankChange")]
	#[inline(always)]
	pub fn change(&self) -> c_float { unsafe { (self.0.getCrankChange)() } }

	/// Returns the previous value for this setting.
	#[doc(alias = "sys::ffi::PlaydateSys::setCrankSoundsDisabled")]
	#[inline(always)]
	pub fn disable_sounds(&self, disable: bool) -> bool {
		unsafe { (self.0.setCrankSoundsDisabled)(disable as _) == 1 }
	}
}
