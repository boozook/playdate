use core::ffi::c_float;
use sys::ffi::PDButtons;
use sys::ffi::PDPeripherals;
use sys::api;


macro_rules! sysfn {
	($name:ident) => {{ unsafe { (*($crate::sys::api()?.system)).$name?() } }};
	($name:ident, $($arg:tt)*) => {{ unsafe { (*($crate::sys::api()?.system)).$name?($($arg)*) } }};
}


/// Wrapped Peripherals API.
///
/// Currently there's only one peripheral - accelerometer.
pub struct Peripherals;
impl Peripherals {
	#![allow(non_snake_case)]

	pub fn enable(value: PDPeripherals) -> Option<()> {
		unsafe { (*(api()?.system)).setPeripheralsEnabled?(value) }.into()
	}
	pub fn enable_accelerometer() -> Option<()> {
		unsafe { (*(api()?.system)).setPeripheralsEnabled?(Self::Accelerometer()) }.into()
	}

	pub fn enable_all() -> Option<()> { unsafe { (*(api()?.system)).setPeripheralsEnabled?(Self::All()) }.into() }
	pub fn disable_all() -> Option<()> {
		unsafe { (*(api()?.system)).setPeripheralsEnabled?(Self::None()) }.into()
	}

	pub const fn None() -> PDPeripherals { PDPeripherals::kNone }
	pub const fn Accelerometer() -> PDPeripherals { PDPeripherals::kAccelerometer }
	pub const fn All() -> PDPeripherals { PDPeripherals::kAllPeripherals }
}


/// Wrapped Accelerometer API.
pub struct Accelerometer;
impl Accelerometer {
	/// Enable accelerometer only.
	/// By default, the accelerometer is disabled to save (a small amount of) power.
	/// To use a peripheral, it must first be enabled via this function.
	/// Accelerometer data is not available until the next update cycle after it’s enabled.
	///
	/// Uses [`setPeripheralsEnabled`][crate::sys::ffi::playdate_sys::setPeripheralsEnabled].
	pub fn enable() -> Option<()> {
		unsafe { (*(api()?.system)).setPeripheralsEnabled?(PDPeripherals::kAccelerometer) }.into()
	}

	/// Uses [`setPeripheralsEnabled`][crate::sys::ffi::playdate_sys::setPeripheralsEnabled].
	///
	/// ⚠️ Disables all peripherals.
	/// Currently it doesn't matter because there's just one peripheral - accelerometer.
	pub fn disable() -> Option<()> {
		unsafe { (*(api()?.system)).setPeripheralsEnabled?(PDPeripherals::kNone) }.into()
	}

	/// Uses [`getAccelerometer`][crate::sys::ffi::playdate_sys::getAccelerometer].
	///
	/// Returns `(x, y, z)` accelerometer data.
	pub fn get() -> Option<(c_float, c_float, c_float)> {
		let mut outx: c_float = 0.0;
		let mut outy: c_float = 0.0;
		let mut outz: c_float = 0.0;

		unsafe { (*(api()?.system)).getAccelerometer?(&mut outx, &mut outy, &mut outz) }
		Some((outx, outy, outz))
	}

	/// Gets accelerometer data directly to `x`, `y` and `z`.
	pub fn get_to(outx: &mut c_float, outy: &mut c_float, outz: &mut c_float) -> Option<()> {
		unsafe { (*(api()?.system)).getAccelerometer?(outx, outy, outz) }.into()
	}
}


/// Wrapped Buttons API.
///
/// Represents buttons state.
///
/// * `current` indicates which buttons are currently down.
/// * `pushed` and `released` reflects which buttons were pushed or released over the previous update cycle.
///
/// Note: at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.
pub struct Buttons {
	/// Indicating which buttons are __currently down__.
	pub current: PDButtons,
	/// Reflect which buttons were pushed over the previous update cycle.
	/// See [struct doc][Self].
	pub pushed: PDButtons,
	/// Reflect which buttons were released over the previous update cycle.
	/// See [struct doc][Self].
	pub released: PDButtons,
}

impl Buttons {
	/// Uses [`getButtonState`][crate::sys::ffi::playdate_sys::getButtonState].
	pub fn get() -> Option<Self> {
		let mut current = PDButtons(0);
		let mut pushed = PDButtons(0);
		let mut released = PDButtons(0);

		sysfn!(getButtonState, &mut current, &mut pushed, &mut released);

		Self { current,
		       pushed,
		       released }.into()
	}

	/// Uses [`getButtonState`][crate::sys::ffi::playdate_sys::getButtonState].
	pub fn get_to(current: &mut PDButtons, pushed: &mut PDButtons, released: &mut PDButtons) -> Option<()> {
		sysfn!(getButtonState, current, pushed, released).into()
	}

	/// Uses [`getButtonState`][crate::sys::ffi::playdate_sys::getButtonState].
	///
	/// Requests & returns only `current` part of state, see [Self::current]
	pub fn get_current() -> Option<PDButtons> {
		use core::ptr::null_mut;

		let mut current = PDButtons(0);
		sysfn!(getButtonState, &mut current, null_mut(), null_mut());
		current.into()
	}

	/// Uses [`getButtonState`][crate::sys::ffi::playdate_sys::getButtonState].
	///
	/// Requests & returns only `current` part of state, see [Self::pushed]
	pub fn get_pushed() -> Option<PDButtons> {
		use core::ptr::null_mut;

		let mut pushed = PDButtons(0);
		sysfn!(getButtonState, null_mut(), &mut pushed, null_mut());
		pushed.into()
	}

	/// Uses [`getButtonState`][crate::sys::ffi::playdate_sys::getButtonState].
	///
	/// Requests & returns only `current` part of state, see [Self::released]
	pub fn get_released() -> Option<PDButtons> {
		use core::ptr::null_mut;

		let mut released = PDButtons(0);
		sysfn!(getButtonState, null_mut(), null_mut(), &mut released);
		released.into()
	}
}

impl core::fmt::Debug for Buttons {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		use crate::buttons::PDButtonsFmt;

		f.debug_struct("Buttons")
		 .field("current", &self.current.display())
		 .field("pushed", &self.pushed.display())
		 .field("released", &self.released.display())
		 .finish()
	}
}


/// Wrapped Crank API.
pub struct Crank;
impl Crank {
	/// Wrapped [`isCrankDocked`][crate::sys::ffi::playdate_sys::isCrankDocked].
	pub fn docked() -> Option<bool> { Some(sysfn!(isCrankDocked) == 1) }
	/// Wrapped [`getCrankAngle`][crate::sys::ffi::playdate_sys::getCrankAngle].
	pub fn angle() -> Option<c_float> { Some(sysfn!(getCrankAngle)).into() }
	/// Wrapped [`getCrankChange`][crate::sys::ffi::playdate_sys::getCrankChange].
	pub fn change() -> Option<c_float> { Some(sysfn!(getCrankChange)).into() }

	/// Wrapped [`setCrankSoundsDisabled`][crate::sys::ffi::playdate_sys::setCrankSoundsDisabled].
	/// * Returns the previous value for this setting.
	pub fn disable_sounds(disable: bool) -> Option<bool> {
		Some(sysfn!(setCrankSoundsDisabled, disable as _) == 1)
	}
}
