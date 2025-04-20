use core::ffi::c_float;

use sys::ffi::Peripherals;

use crate::Api;


/// Accelerometer
#[derive(Clone, Copy)]
pub struct Accelerometer(pub(crate) Api);

impl Default for Accelerometer {
	fn default() -> Self { Self(api!(system)) }
}

impl Accelerometer {
	pub const fn new(api: Api) -> Self { Self(api) }
}


impl Accelerometer {
	/// Returns the `(x, y, z)` last-read accelerometer data.
	///
	/// See also [`get_to`][`Self::get_to`].
	#[doc(alias = "sys::ffi::PlaydateSys::getAccelerometer")]
	pub fn get(&self) -> (c_float, c_float, c_float) {
		let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
		self.get_to(&mut x, &mut y, &mut z);
		(x, y, z)
	}

	/// Sets the last-read accelerometer data directly to `x`, `y` and `z`.
	///
	/// See also [`get`][`Self::get`].
	#[doc(alias = "sys::ffi::PlaydateSys::getAccelerometer")]
	#[inline(always)]
	pub fn get_to(&self, outx: &mut c_float, outy: &mut c_float, outz: &mut c_float) {
		unsafe { (self.0.getAccelerometer)(outx, outy, outz) }
	}


	/// Enables accelerometer.
	///
	/// By default, the accelerometer is disabled to save (a small amount of) power.
	///
	/// To use a peripheral, it must first be enabled via this function.
	///
	/// Accelerometer data is not available until the next update cycle after it’s enabled.
	#[doc(alias = "sys::ffi::PlaydateSys::setPeripheralsEnabled")]
	pub fn enable(api: Api) { unsafe { (api.setPeripheralsEnabled)(Peripherals::Accelerometer) } }

	/// Disables accelerometer.
	///
	/// _Functionally it disables all peripherals, but
	/// currently there's only one peripheral - accelerometer._
	#[doc(alias = "sys::ffi::PlaydateSys::setPeripheralsEnabled")]
	pub fn disable(api: Api) { unsafe { (api.setPeripheralsEnabled)(Peripherals::None) } }
}
