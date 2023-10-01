use core::ffi::c_float;
use core::ffi::c_int;
use core::ptr::NonNull;

use sys::ffi::PDButtons;
use sys::ffi::PDPeripherals;
use sys::ffi::playdate_sys;

/// Default system peripherals api end-point, ZST.
///
/// All calls approximately costs ~3 derefs.
#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;
impl Api for Default {}


/// Cached system peripherals api end-point.
///
/// Stores one reference, so size on stack is eq `usize`.
///
/// All calls approximately costs ~1 deref.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Cache(&'static playdate_sys);

impl core::default::Default for Cache {
	fn default() -> Self { Self(api!(system)) }
}

impl From<*const playdate_sys> for Cache {
	#[inline(always)]
	fn from(ptr: *const playdate_sys) -> Self { Self(unsafe { ptr.as_ref() }.expect("system")) }
}

impl From<&'static playdate_sys> for Cache {
	#[inline(always)]
	fn from(r: &'static playdate_sys) -> Self { Self(r) }
}

impl From<NonNull<playdate_sys>> for Cache {
	#[inline(always)]
	fn from(ptr: NonNull<playdate_sys>) -> Self { Self(unsafe { ptr.as_ref() }) }
}

impl From<&'_ NonNull<playdate_sys>> for Cache {
	#[inline(always)]
	fn from(ptr: &NonNull<playdate_sys>) -> Self { Self(unsafe { ptr.as_ref() }) }
}

impl From<system::api::Cache> for Cache {
	#[inline(always)]
	fn from(api: system::api::Cache) -> Self { Self(api.as_inner()) }
}


impl Api for Cache {
	#[inline(always)]
	fn set_peripherals_enabled(&self) -> unsafe extern "C" fn(mask: PDPeripherals) {
		self.0.setPeripheralsEnabled.expect("setPeripheralsEnabled")
	}

	#[inline(always)]
	fn get_button_state(
		&self)
		-> unsafe extern "C" fn(current: *mut PDButtons, pushed: *mut PDButtons, released: *mut PDButtons) {
		self.0.getButtonState.expect("getButtonState")
	}

	#[inline(always)]
	fn get_accelerometer(
		&self)
		-> unsafe extern "C" fn(out_x: *mut c_float, out_y: *mut c_float, out_z: *mut c_float) {
		self.0.getAccelerometer.expect("getAccelerometer")
	}

	#[inline(always)]
	fn get_crank_change(&self) -> unsafe extern "C" fn() -> c_float {
		self.0.getCrankChange.expect("getCrankChange")
	}

	#[inline(always)]
	fn get_crank_angle(&self) -> unsafe extern "C" fn() -> c_float { self.0.getCrankAngle.expect("getCrankAngle") }

	#[inline(always)]
	fn is_crank_docked(&self) -> unsafe extern "C" fn() -> c_int { self.0.isCrankDocked.expect("isCrankDocked") }

	#[inline(always)]
	fn set_crank_sounds_disabled(&self) -> unsafe extern "C" fn(flag: c_int) -> c_int {
		self.0.setCrankSoundsDisabled.expect("setCrankSoundsDisabled")
	}
}


impl Api for system::api::Default {}

impl Api for system::api::Cache {
	#[inline(always)]
	fn set_peripherals_enabled(&self) -> unsafe extern "C" fn(mask: PDPeripherals) {
		self.as_inner()
		    .setPeripheralsEnabled
		    .expect("setPeripheralsEnabled")
	}

	#[inline(always)]
	fn get_button_state(
		&self)
		-> unsafe extern "C" fn(current: *mut PDButtons, pushed: *mut PDButtons, released: *mut PDButtons) {
		self.as_inner().getButtonState.expect("getButtonState")
	}

	#[inline(always)]
	fn get_accelerometer(
		&self)
		-> unsafe extern "C" fn(out_x: *mut c_float, out_y: *mut c_float, out_z: *mut c_float) {
		self.as_inner().getAccelerometer.expect("getAccelerometer")
	}

	#[inline(always)]
	fn get_crank_change(&self) -> unsafe extern "C" fn() -> c_float {
		self.as_inner().getCrankChange.expect("getCrankChange")
	}

	#[inline(always)]
	fn get_crank_angle(&self) -> unsafe extern "C" fn() -> c_float {
		self.as_inner().getCrankAngle.expect("getCrankAngle")
	}

	#[inline(always)]
	fn is_crank_docked(&self) -> unsafe extern "C" fn() -> c_int {
		self.as_inner().isCrankDocked.expect("isCrankDocked")
	}

	#[inline(always)]
	fn set_crank_sounds_disabled(&self) -> unsafe extern "C" fn(flag: c_int) -> c_int {
		self.as_inner()
		    .setCrankSoundsDisabled
		    .expect("setCrankSoundsDisabled")
	}
}

pub trait Api {
	/// Returns [`sys::ffi::playdate_sys::setPeripheralsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	fn set_peripherals_enabled(&self) -> unsafe extern "C" fn(mask: PDPeripherals) {
		*sys::api!(system.setPeripheralsEnabled)
	}

	/// Returns [`sys::ffi::playdate_sys::getButtonState`]
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	fn get_button_state(
		&self)
		-> unsafe extern "C" fn(current: *mut PDButtons, pushed: *mut PDButtons, released: *mut PDButtons) {
		*sys::api!(system.getButtonState)
	}

	/// Returns [`sys::ffi::playdate_sys::getAccelerometer`]
	#[doc(alias = "sys::ffi::playdate_sys::getAccelerometer")]
	fn get_accelerometer(
		&self)
		-> unsafe extern "C" fn(out_x: *mut c_float, out_y: *mut c_float, out_z: *mut c_float) {
		*sys::api!(system.getAccelerometer)
	}

	/// Returns [`sys::ffi::playdate_sys::getCrankChange`]
	#[doc(alias = "sys::ffi::playdate_sys::getCrankChange")]
	fn get_crank_change(&self) -> unsafe extern "C" fn() -> c_float { *sys::api!(system.getCrankChange) }

	/// Returns [`sys::ffi::playdate_sys::getCrankAngle`]
	#[doc(alias = "sys::ffi::playdate_sys::getCrankAngle")]
	fn get_crank_angle(&self) -> unsafe extern "C" fn() -> c_float { *sys::api!(system.getCrankAngle) }

	/// Returns [`sys::ffi::playdate_sys::isCrankDocked`]
	#[doc(alias = "sys::ffi::playdate_sys::isCrankDocked")]
	fn is_crank_docked(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(system.isCrankDocked) }

	/// Returns [`sys::ffi::playdate_sys::setCrankSoundsDisabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setCrankSoundsDisabled")]
	fn set_crank_sounds_disabled(&self) -> unsafe extern "C" fn(flag: c_int) -> c_int {
		*sys::api!(system.setCrankSoundsDisabled)
	}
}
