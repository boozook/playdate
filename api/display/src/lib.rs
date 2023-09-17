#![cfg_attr(not(test), no_std)]
extern crate sys;

use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_uint;


#[derive(Debug, Clone, Copy)]
pub struct Display<Api = api::Default>(Api);

impl<Api: Default + api::Api> Default for Display<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Display<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Display<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}


impl<Api: api::Api> Display<Api> {
	/// Equivalent to [`sys::ffi::playdate_display::getWidth`]
	#[doc(alias = "sys::ffi::playdate_display::getWidth")]
	pub fn width(&self) -> c_int {
		let f = self.0.get_width();
		unsafe { f() }
	}

	/// Equivalent to [`sys::ffi::playdate_display::getHeight`]
	#[doc(alias = "sys::ffi::playdate_display::getHeight")]
	pub fn height(&self) -> c_int {
		let f = self.0.get_height();
		unsafe { f() }
	}

	/// Equivalent to [`sys::ffi::playdate_display::setRefreshRate`]
	#[doc(alias = "sys::ffi::playdate_display::setRefreshRate")]
	pub fn set_refresh_rate(&self, rate: c_float) {
		let f = self.0.set_refresh_rate();
		unsafe { f(rate) }
	}

	/// Equivalent to [`sys::ffi::playdate_display::setInverted`]
	#[doc(alias = "sys::ffi::playdate_display::setInverted")]
	pub fn set_inverted(&self, value: bool) {
		let f = self.0.set_inverted();
		unsafe { f(value as _) }
	}

	/// Equivalent to [`sys::ffi::playdate_display::setScale`]
	#[doc(alias = "sys::ffi::playdate_display::setScale")]
	pub fn set_scale(&self, scale: c_uint) {
		let f = self.0.set_scale();
		unsafe { f(scale) }
	}

	/// Equivalent to [`sys::ffi::playdate_display::setMosaic`]
	#[doc(alias = "sys::ffi::playdate_display::setMosaic")]
	pub fn set_mosaic(&self, x: c_uint, y: c_uint) {
		let f = self.0.set_mosaic();
		unsafe { f(x, y) }
	}

	/// Equivalent to [`sys::ffi::playdate_display::setFlipped`]
	#[doc(alias = "sys::ffi::playdate_display::setFlipped")]
	pub fn set_flipped(&self, x: bool, y: bool) {
		let f = self.0.set_flipped();
		unsafe { f(x as _, y as _) }
	}

	/// Equivalent to [`sys::ffi::playdate_display::setOffset`]
	#[doc(alias = "sys::ffi::playdate_display::setOffset")]
	pub fn set_offset(&self, x: c_int, y: c_int) {
		let f = self.0.set_offset();
		unsafe { f(x, y) }
	}
}


pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_float;
	use core::ffi::c_int;
	use core::ffi::c_uint;


	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;

	impl Api for Default {}


	pub trait Api {
		/// Equivalent to [`sys::ffi::playdate_display::getWidth`]
		#[doc(alias = "sys::ffi::playdate_display::getWidth")]
		fn get_width(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(display.getWidth) }
		/// Equivalent to [`sys::ffi::playdate_display::getHeight`]
		#[doc(alias = "sys::ffi::playdate_display::getHeight")]
		fn get_height(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(display.getHeight) }
		/// Equivalent to [`sys::ffi::playdate_display::setRefreshRate`]
		#[doc(alias = "sys::ffi::playdate_display::setRefreshRate")]
		fn set_refresh_rate(&self) -> unsafe extern "C" fn(rate: c_float) { *sys::api!(display.setRefreshRate) }
		/// Equivalent to [`sys::ffi::playdate_display::setInverted`]
		#[doc(alias = "sys::ffi::playdate_display::setInverted")]
		fn set_inverted(&self) -> unsafe extern "C" fn(flag: c_int) { *sys::api!(display.setInverted) }
		/// Equivalent to [`sys::ffi::playdate_display::setScale`]
		#[doc(alias = "sys::ffi::playdate_display::setScale")]
		fn set_scale(&self) -> unsafe extern "C" fn(s: c_uint) { *sys::api!(display.setScale) }
		/// Equivalent to [`sys::ffi::playdate_display::setMosaic`]
		#[doc(alias = "sys::ffi::playdate_display::setMosaic")]
		fn set_mosaic(&self) -> unsafe extern "C" fn(x: c_uint, y: c_uint) { *sys::api!(display.setMosaic) }
		/// Equivalent to [`sys::ffi::playdate_display::setFlipped`]
		#[doc(alias = "sys::ffi::playdate_display::setFlipped")]
		fn set_flipped(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { *sys::api!(display.setFlipped) }
		/// Equivalent to [`sys::ffi::playdate_display::setOffset`]
		#[doc(alias = "sys::ffi::playdate_display::setOffset")]
		fn set_offset(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { *sys::api!(display.setOffset) }
	}
}
