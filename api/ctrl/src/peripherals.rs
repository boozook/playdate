use core::ffi::c_float;
use sys::ffi::PDButtons;
use sys::ffi::PDPeripherals;
use crate::api;


/// Peripherals
#[derive(Debug, Clone, Copy)]
pub struct Peripherals<Api = api::Default>(Api);

impl Peripherals<api::Default> {
	/// Creates default [`Peripherals`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Peripherals<api::Cache> {
	/// Creates [`Peripherals`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Peripherals<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Peripherals<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Peripherals<Api> {
	pub const fn new_with(api: Api) -> Self { Self(api) }
}

impl<Api: api::Api> Peripherals<Api> where Api: Copy {
	pub const fn accelerometer(&self) -> Accelerometer<Api> { Accelerometer(self.0) }
	pub const fn buttons(&self) -> Buttons<Api> { Buttons(self.0) }
	pub const fn crank(&self) -> Crank<Api> { Crank(self.0) }
}

pub use shorthands::*;
#[gen_api_shorthands::gen_shorthands_mod(mod shorthands)]
impl<Api: api::Api> Peripherals<Api> {
	/// Enables specified peripheral.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	pub fn enable(&self, value: PDPeripherals) {
		let f = self.0.set_peripherals_enabled();
		unsafe { f(value) }
	}

	/// By default, the accelerometer is disabled to save (a small amount of) power.
	///
	/// To use a peripheral, it must first be enabled via this function.
	///
	/// Accelerometer data is not available until the next update cycle after it’s enabled.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	#[inline(always)]
	pub fn enable_accelerometer(&self) { self.enable(Peripherals::Accelerometer) }

	/// Enables all peripherals.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	#[inline(always)]
	pub fn enable_all(&self) { self.enable(Peripherals::All) }

	/// Disables all peripherals.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	#[inline(always)]
	pub fn disable_all(&self) { self.enable(Peripherals::None) }
}

impl Peripherals<api::Default> {
	#![allow(non_upper_case_globals)]
	pub const None: PDPeripherals = PDPeripherals::kNone;
	pub const Accelerometer: PDPeripherals = PDPeripherals::kAccelerometer;
	pub const All: PDPeripherals = PDPeripherals::kAllPeripherals;
}


// TODO: SystemExt should be const_trait
pub trait SystemExt<Api: api::Api + Copy> {
	fn peripherals(&self) -> Peripherals<Api>;
	fn accelerometer(&self) -> Accelerometer<Api>;
	fn buttons(&self) -> Buttons<Api>;
	fn crank(&self) -> Crank<Api>;
}

impl<Api: system::api::Api + api::Api + Copy> SystemExt<Api> for system::System<Api> {
	fn peripherals(&self) -> Peripherals<Api> { Peripherals::new_with(self.inner()) }
	fn accelerometer(&self) -> Accelerometer<Api> { Accelerometer::new_with(self.inner()) }
	fn buttons(&self) -> Buttons<Api> { Buttons::new_with(self.inner()) }
	fn crank(&self) -> Crank<Api> { Crank::new_with(self.inner()) }
}


/// Accelerometer
#[derive(Debug, Clone, Copy)]
pub struct Accelerometer<Api = api::Default>(Api);

impl Accelerometer<api::Default> {
	/// Creates default [`Accelerometer`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Accelerometer<api::Cache> {
	/// Creates [`Accelerometer`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Accelerometer<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Accelerometer<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Accelerometer<Api> {
	pub const fn new_with(api: Api) -> Self { Self(api) }
}

#[gen_api_shorthands::gen_shorthands_mod(pub(crate) mod accelerometer_shorthands)]
impl<Api: api::Api> Accelerometer<Api> {
	/// Enables accelerometer only.
	///
	/// By default, the accelerometer is disabled to save (a small amount of) power.
	///
	/// To use a peripheral, it must first be enabled via this function.
	///
	/// Accelerometer data is not available until the next update cycle after it’s enabled.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`].
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	pub fn enable(&self) {
		let f = self.0.set_peripherals_enabled();
		unsafe { f(Peripherals::Accelerometer) }
	}

	/// ⚠️ Disables all peripherals including accelerometer.
	///
	/// Currently it doesn't matter because there's just one peripheral - accelerometer.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setPeripheralsEnabled`].
	#[doc(alias = "sys::ffi::playdate_sys::setPeripheralsEnabled")]
	pub fn disable(&self) {
		let f = self.0.set_peripherals_enabled();
		unsafe { f(Peripherals::None) }
	}

	/// Returns `(x, y, z)` accelerometer data.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getAccelerometer`].
	#[doc(alias = "sys::ffi::playdate_sys::getAccelerometer")]
	pub fn get(&self) -> (c_float, c_float, c_float) {
		let mut outx: c_float = 0.0;
		let mut outy: c_float = 0.0;
		let mut outz: c_float = 0.0;

		let f = self.0.get_accelerometer();
		unsafe { f(&mut outx, &mut outy, &mut outz) };
		(outx, outy, outz)
	}

	/// Gets accelerometer data directly to `x`, `y` and `z`.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getAccelerometer`].
	#[doc(alias = "sys::ffi::playdate_sys::getAccelerometer")]
	pub fn get_to(&self, outx: &mut c_float, outy: &mut c_float, outz: &mut c_float) {
		let f = self.0.get_accelerometer();
		unsafe { f(outx, outy, outz) }
	}
}


/// Buttons
#[derive(Debug, Clone, Copy)]
pub struct Buttons<Api = api::Default>(Api);

impl Buttons<api::Default> {
	/// Creates default [`Buttons`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Buttons<api::Cache> {
	/// Creates [`Buttons`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Buttons<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Buttons<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Buttons<Api> {
	pub const fn new_with(api: Api) -> Self { Self(api) }
}

#[gen_api_shorthands::gen_shorthands_mod(pub(crate) mod buttons_shorthands)]
impl<Api: api::Api> Buttons<Api> {
	/// Returns the current buttons [`State`].
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getButtonState`].
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	pub fn get(&self) -> State {
		let mut current = PDButtons(0);
		let mut pushed = PDButtons(0);
		let mut released = PDButtons(0);

		let f = self.0.get_button_state();
		unsafe { f(&mut current, &mut pushed, &mut released) }

		State { current,
		        pushed,
		        released }
	}

	/// Writes the current buttons state to given [`State`].
	///
	/// Updates all (current, pushed and released).
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getButtonState`].
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	pub fn get_to(&self, state: &mut State) {
		let f = self.0.get_button_state();
		unsafe { f(&mut state.current, &mut state.pushed, &mut state.released) }
	}

	/// Writes the current buttons state to given references.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getButtonState`].
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	pub fn get_to_raw(&self, current: &mut PDButtons, pushed: &mut PDButtons, released: &mut PDButtons) {
		let f = self.0.get_button_state();
		unsafe { f(current, pushed, released) }
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getButtonState`].
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	///
	/// Requests & returns only `current` part of state, see [Self::current]
	pub fn current(&self) -> PDButtons {
		use core::ptr::null_mut;

		let mut current = PDButtons(0);
		let f = self.0.get_button_state();
		unsafe { f(&mut current, null_mut(), null_mut()) }
		current
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getButtonState`].
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	///
	/// Requests & returns only `current` part of state, see [Self::pushed]
	pub fn pushed(&self) -> PDButtons {
		use core::ptr::null_mut;

		let mut pushed = PDButtons(0);

		let f = self.0.get_button_state();
		unsafe { f(null_mut(), &mut pushed, null_mut()) }

		pushed
	}

	/// Equivalent to [`sys::ffi::playdate_sys::getButtonState`].
	#[doc(alias = "sys::ffi::playdate_sys::getButtonState")]
	///
	/// Requests & returns only `current` part of state, see [Self::released]
	pub fn released(&self) -> PDButtons {
		use core::ptr::null_mut;

		let mut released = PDButtons(0);

		let f = self.0.get_button_state();
		unsafe { f(null_mut(), null_mut(), &mut released) }

		released
	}
}

/// Represents buttons state.
///
/// * `current` indicates which buttons are currently down.
/// * `pushed` and `released` reflects which buttons were pushed or released over the previous update cycle.
///
/// Note: at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.
pub struct State {
	/// Indicating which buttons are __currently down__.
	pub current: PDButtons,
	/// Reflect which buttons were pushed over the previous update cycle.
	/// See [struct doc][Self].
	pub pushed: PDButtons,
	/// Reflect which buttons were released over the previous update cycle.
	/// See [struct doc][Self].
	pub released: PDButtons,
}


impl core::fmt::Debug for State {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		use crate::buttons::PDButtonsFmt;

		f.debug_struct("Buttons")
		 .field("current", &self.current.display())
		 .field("pushed", &self.pushed.display())
		 .field("released", &self.released.display())
		 .finish()
	}
}


/// Crank
#[derive(Debug, Clone, Copy)]
pub struct Crank<Api = api::Default>(Api);

impl Crank<api::Default> {
	/// Creates default [`Crank`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Crank<api::Cache> {
	/// Creates [`Crank`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Crank<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Crank<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Crank<Api> {
	pub const fn new_with(api: Api) -> Self { Self(api) }
}

#[gen_api_shorthands::gen_shorthands_mod(pub(crate) mod crank_shorthands)]
impl<Api: api::Api> Crank<Api> {
	/// Returns boolean indicating whether or not the crank is folded into the unit.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::isCrankDocked`].
	#[doc(alias = "sys::ffi::playdate_sys::isCrankDocked")]
	pub fn docked(&self) -> bool {
		let f = self.0.is_crank_docked();
		unsafe { f() == 1 }
	}

	/// Returns the current position of the crank, in the range 0-360.
	/// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getCrankAngle`].
	#[doc(alias = "sys::ffi::playdate_sys::getCrankAngle")]
	pub fn angle(&self) -> c_float {
		let f = self.0.get_crank_angle();
		unsafe { f() }
	}

	/// Returns the angle change of the crank since the last time this function was called.
	/// Negative values are anti-clockwise.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::getCrankChange`].
	#[doc(alias = "sys::ffi::playdate_sys::getCrankChange")]
	pub fn change(&self) -> c_float {
		let f = self.0.get_crank_change();
		unsafe { f() }
	}

	/// Returns the previous value for this setting.
	///
	/// Equivalent to [`sys::ffi::playdate_sys::setCrankSoundsDisabled`].
	#[doc(alias = "sys::ffi::playdate_sys::setCrankSoundsDisabled")]
	pub fn disable_sounds(&self, disable: bool) -> bool {
		let f = self.0.set_crank_sounds_disabled();
		unsafe { f(disable as _) == 1 }
	}
}
