#![cfg_attr(not(test), no_std)]
extern crate sys;

use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_uint;

#[derive(Debug, Clone, Copy)]
pub struct Display<Api = api::Default>(Api);

impl Display<api::Default> {
	/// Creates default [`Display`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Display<api::Cache> {
	/// Creates [`Display`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Display<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Display<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Display<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}


impl Display<api::Default> {
	pub const COLUMNS: u32 = sys::ffi::LCD_COLUMNS;
	pub const ROWS: u32 = sys::ffi::LCD_ROWS;
	pub const ROW_SIZE: u32 = sys::ffi::LCD_ROWSIZE;
	pub const SCREEN_RECT: sys::ffi::LCDRect = sys::ffi::LCDRect { left: 0,
	                                                               right: 0,
	                                                               top: Self::COLUMNS as _,
	                                                               bottom: Self::ROWS as _ };
}

pub use shorthands::*;
#[gen_api_shorthands::gen_shorthands_mod(mod shorthands)]
impl<Api: api::Api> Display<Api> {
	/// Returns the width of the display, taking the current scale into account;
	///
	/// e.g., if the scale is `2`, this function returns `200` instead of `400`.
	///
	/// See also [`Display::COLUMNS`].
	///
	/// Equivalent to [`sys::ffi::playdate_display::getWidth`]
	#[doc(alias = "sys::ffi::playdate_display::getWidth")]
	pub fn width(&self) -> c_int {
		let f = self.0.get_width();
		unsafe { f() }
	}

	/// Returns the height of the display, taking the current scale into account;
	///
	/// e.g., if the scale is `2`, this function returns `120` instead of `240`.
	///
	/// See also [`Display::ROWS`] and [`Display::ROW_SIZE`].
	///
	/// Equivalent to [`sys::ffi::playdate_display::getHeight`]
	#[doc(alias = "sys::ffi::playdate_display::getHeight")]
	pub fn height(&self) -> c_int {
		let f = self.0.get_height();
		unsafe { f() }
	}

	/// Sets the nominal refresh rate in frames per second.
	///
	/// Default is 20 fps, the maximum rate supported by the hardware for full-frame updates.
	///
	/// Equivalent to [`sys::ffi::playdate_display::setRefreshRate`]
	#[doc(alias = "sys::ffi::playdate_display::setRefreshRate")]
	pub fn set_refresh_rate(&self, rate: c_float) {
		let f = self.0.set_refresh_rate();
		unsafe { f(rate) }
	}

	/// If `value` is `true`, the frame buffer is drawn inverted—black instead of white, and vice versa.
	///
	/// Equivalent to [`sys::ffi::playdate_display::setInverted`]
	#[doc(alias = "sys::ffi::playdate_display::setInverted")]
	pub fn set_inverted(&self, value: bool) {
		let f = self.0.set_inverted();
		unsafe { f(value as _) }
	}

	/// Sets the display scale factor.
	///
	/// The top-left corner of the frame buffer is scaled up to fill the display;
	///
	/// e.g., if the scale is set to [`DisplayScale::Quad`],
	/// the pixels in rectangle `[0, 100] x [0, 60]` are drawn on the screen as `4 x 4` squares.
	///
	/// Equivalent to [`sys::ffi::playdate_display::setScale`]
	#[doc(alias = "sys::ffi::playdate_display::setScale")]
	pub fn set_scale(&self, scale: DisplayScale) { self.set_scale_raw(scale.into()); }

	/// Sets the display scale factor.
	///
	/// Valid values for `scale` are `1`, `2`, `4`, and `8`.
	///
	/// The top-left corner of the frame buffer is scaled up to fill the display;
	/// e.g., if the scale is set to `4`, the pixels in rectangle `[0, 100] x [0, 60]` are drawn on the screen as `4 x 4` squares.
	///
	/// See also [`Display::set_scale`].
	///
	/// Equivalent to [`sys::ffi::playdate_display::setScale`]
	#[doc(alias = "sys::ffi::playdate_display::setScale")]
	pub fn set_scale_raw(&self, scale: c_uint) {
		let f = self.0.set_scale();
		unsafe { f(scale) }
	}

	/// Adds a mosaic effect to the display.
	///
	/// Valid `x` and `y` values are between `0` and `3`, inclusive.
	///
	/// Equivalent to [`sys::ffi::playdate_display::setMosaic`]
	#[doc(alias = "sys::ffi::playdate_display::setMosaic")]
	pub fn set_mosaic(&self, x: c_uint, y: c_uint) {
		let f = self.0.set_mosaic();
		unsafe { f(x, y) }
	}

	/// Flips the display on the `x` or `y` axis, or both.
	///
	/// Equivalent to [`sys::ffi::playdate_display::setFlipped`]
	#[doc(alias = "sys::ffi::playdate_display::setFlipped")]
	pub fn set_flipped(&self, x: bool, y: bool) {
		let f = self.0.set_flipped();
		unsafe { f(x as _, y as _) }
	}

	/// Offsets the display by the given amount.
	///
	/// Areas outside of the displayed area are filled with the current background color.
	///
	/// See also [`playdate-graphics::set_background_color`].
	///
	/// Equivalent to [`sys::ffi::playdate_display::setOffset`]
	#[doc(alias = "sys::ffi::playdate_display::setOffset")]
	pub fn set_offset(&self, x: c_int, y: c_int) {
		let f = self.0.set_offset();
		unsafe { f(x, y) }
	}
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayScale {
	Normal = 1,
	Double = 2,
	Quad = 4,
	Eight = 8,
}

impl Into<c_uint> for DisplayScale {
	#[inline(always)]
	fn into(self) -> c_uint { (self as u8).into() }
}

impl core::fmt::Display for DisplayScale {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{}", *self as u8) }
}

impl DisplayScale {
	#[inline(always)]
	pub const fn as_u8(self) -> u8 { self as u8 }
	#[inline(always)]
	pub const fn as_int(self) -> c_int { self as u8 as _ }
}


pub mod api {
	use core::ffi::c_float;
	use core::ffi::c_int;
	use core::ffi::c_uint;
	use core::ptr::NonNull;
	use sys::ffi::playdate_display;


	/// Default display api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached display api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_display);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(display)) }
	}

	impl From<*const playdate_display> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_display) -> Self { Self(unsafe { ptr.as_ref() }.expect("display")) }
	}

	impl From<&'static playdate_display> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_display) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_display>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_display>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_display>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_display>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}


	impl Api for Cache {
		#[inline(always)]
		fn get_width(&self) -> unsafe extern "C" fn() -> c_int { self.0.getWidth.expect("getWidth") }

		#[inline(always)]
		fn get_height(&self) -> unsafe extern "C" fn() -> c_int { self.0.getHeight.expect("getHeight") }

		#[inline(always)]
		fn set_refresh_rate(&self) -> unsafe extern "C" fn(rate: c_float) {
			self.0.setRefreshRate.expect("setRefreshRate")
		}

		#[inline(always)]
		fn set_inverted(&self) -> unsafe extern "C" fn(flag: c_int) { self.0.setInverted.expect("setInverted") }

		#[inline(always)]
		fn set_scale(&self) -> unsafe extern "C" fn(s: c_uint) { self.0.setScale.expect("setScale") }

		#[inline(always)]
		fn set_mosaic(&self) -> unsafe extern "C" fn(x: c_uint, y: c_uint) { self.0.setMosaic.expect("setMosaic") }

		#[inline(always)]
		fn set_flipped(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { self.0.setFlipped.expect("setFlipped") }

		#[inline(always)]
		fn set_offset(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { self.0.setOffset.expect("setOffset") }
	}


	pub trait Api {
		/// Returns [`sys::ffi::playdate_display::getWidth`]
		#[doc(alias = "sys::ffi::playdate_display::getWidth")]
		fn get_width(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(display.getWidth) }
		/// Returns [`sys::ffi::playdate_display::getHeight`]
		#[doc(alias = "sys::ffi::playdate_display::getHeight")]
		fn get_height(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(display.getHeight) }
		/// Returns [`sys::ffi::playdate_display::setRefreshRate`]
		#[doc(alias = "sys::ffi::playdate_display::setRefreshRate")]
		fn set_refresh_rate(&self) -> unsafe extern "C" fn(rate: c_float) { *sys::api!(display.setRefreshRate) }
		/// Returns [`sys::ffi::playdate_display::setInverted`]
		#[doc(alias = "sys::ffi::playdate_display::setInverted")]
		fn set_inverted(&self) -> unsafe extern "C" fn(flag: c_int) { *sys::api!(display.setInverted) }
		/// Returns [`sys::ffi::playdate_display::setScale`]
		#[doc(alias = "sys::ffi::playdate_display::setScale")]
		fn set_scale(&self) -> unsafe extern "C" fn(s: c_uint) { *sys::api!(display.setScale) }
		/// Returns [`sys::ffi::playdate_display::setMosaic`]
		#[doc(alias = "sys::ffi::playdate_display::setMosaic")]
		fn set_mosaic(&self) -> unsafe extern "C" fn(x: c_uint, y: c_uint) { *sys::api!(display.setMosaic) }
		/// Returns [`sys::ffi::playdate_display::setFlipped`]
		#[doc(alias = "sys::ffi::playdate_display::setFlipped")]
		fn set_flipped(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { *sys::api!(display.setFlipped) }
		/// Returns [`sys::ffi::playdate_display::setOffset`]
		#[doc(alias = "sys::ffi::playdate_display::setOffset")]
		fn set_offset(&self) -> unsafe extern "C" fn(x: c_int, y: c_int) { *sys::api!(display.setOffset) }
	}
}
