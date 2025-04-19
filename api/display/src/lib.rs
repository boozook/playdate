#![no_std]
#![cfg_attr(not(test), no_main)]
extern crate sys;

use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_uint;


type Api = &'static sys::ffi::PlaydateDisplay;


/// Playdate Display API.
///
/// Uses inner api end-point for all operations.
///
/// ```no_run
/// # use playdate_display::Display;

/// let display = Display::default();
/// let width = display.width();
/// let height = display.height();
/// display.set_fps(30.0);
/// ```
#[derive(Clone, Copy)]
pub struct Display(Api);

impl Default for Display {
	fn default() -> Self { Self(sys::api!(display)) }
}

impl Display {
	pub const COLUMNS: u32 = sys::ffi::LCD_COLUMNS;
	pub const ROWS: u32 = sys::ffi::LCD_ROWS;
	pub const ROW_SIZE: u32 = sys::ffi::LCD_ROWSIZE;

	pub fn new(api: Api) -> Self { Self(api) }
}


impl Display {
	/// Returns the width of the display, taking the current scale into account;
	///
	/// e.g., if the scale is [`DisplayScale::Double`] (x`2`),
	/// this function returns `200` instead of `400`.
	///
	/// See also [`Display::COLUMNS`].
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::getWidth`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::getWidth")]
	#[inline(always)]
	pub fn width(&self) -> c_int { unsafe { (self.0.getWidth)() } }

	/// Returns the height of the display, taking the current scale into account;
	///
	/// e.g., if the scale is [`DisplayScale::Double`] (x`2`),
	/// this function returns `120` instead of `240`.
	///
	/// See also [`Display::ROWS`] and [`Display::ROW_SIZE`].
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::getHeight`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::getHeight")]
	#[inline(always)]
	pub fn height(&self) -> c_int { unsafe { (self.0.getHeight)() } }

	/// Sets the nominal refresh rate in frames per second.
	///
	/// Default is 20 fps, the maximum rate supported by the hardware for full-frame updates.
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setRefreshRate`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setRefreshRate")]
	#[inline(always)]
	pub fn set_fps(&self, rate: c_float) { unsafe { (self.0.setRefreshRate)(rate) } }

	/// If `value` is `true`, the frame buffer is drawn inverted—black instead of white, and vice versa.
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setInverted`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setInverted")]
	#[inline(always)]
	pub fn set_inverted(&self, value: bool) { unsafe { (self.0.setInverted)(value as _) } }

	/// Sets the display scale factor.
	///
	/// The top-left corner of the frame buffer is scaled up to fill the display;
	///
	/// e.g., if the scale is set to [`DisplayScale::Quad`],
	/// the pixels in rectangle `[0, 100] x [0, 60]` are drawn on the screen as `4 x 4` squares.
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setScale`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setScale")]
	#[inline(always)]
	pub fn set_scale(&self, scale: DisplayScale) { self.set_scale_raw(scale.as_uint()); }

	/// Sets the display scale factor.
	///
	/// Valid values for `scale` are `1`, `2`, `4`, and `8`.
	///
	/// The top-left corner of the frame buffer is scaled up to fill the display;
	/// e.g., if the scale is set to `4`, the pixels in rectangle `[0, 100] x [0, 60]` are drawn on the screen as `4 x 4` squares.
	///
	/// See also [`Display::set_scale`].
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setScale`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setScale")]
	#[inline(always)]
	pub fn set_scale_raw(&self, scale: c_uint) { unsafe { (self.0.setScale)(scale) } }

	/// Adds a mosaic effect to the display.
	///
	/// Valid `x` and `y` values are between `0` and `3`, inclusive.
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setMosaic`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setMosaic")]
	#[inline(always)]
	pub fn set_mosaic(&self, x: c_uint, y: c_uint) { unsafe { (self.0.setMosaic)(x, y) } }

	/// Flips the display on the `x` or `y` axis, or both.
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setFlipped`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setFlipped")]
	#[inline(always)]
	pub fn set_flipped(&self, x: bool, y: bool) { unsafe { (self.0.setFlipped)(x as _, y as _) } }

	/// Offsets the display by the given amount.
	///
	/// Areas outside of the displayed area are filled with the current background color.
	///
	/// See also [`playdate-graphics::set_background_color`].
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::setOffset`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::setOffset")]
	#[inline(always)]
	pub fn set_offset(&self, x: c_int, y: c_int) { unsafe { (self.0.setOffset)(x, y) } }

	/// Returns the current nominal display refresh rate.
	///
	/// This is the frame rate the device is targeting,
	/// and does not account for lag due to (for example) code running too slow.
	///
	/// To get the real time frame rate, use [`fps_actual`](Display::fps_actual()).
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::getRefreshRate`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::getRefreshRate")]
	#[inline(always)]
	pub fn fps_target(&self) -> c_float { unsafe { (self.0.getRefreshRate)() } }

	/// Returns the measured, actual refresh rate in frames per second.
	///
	/// This value may be different from the specified refresh rate via [`set_fps`](Display::set_fps)
	/// by a little or a lot depending upon how much calculation is being done per frame.
	///
	/// See also [`fps_target`](Self::fps_target).
	///
	/// Equivalent to [`sys::ffi::PlaydateDisplay::getFPS`]
	#[doc(alias = "sys::ffi::PlaydateDisplay::getFPS")]
	#[inline(always)]
	pub fn fps_actual(&self) -> c_float { unsafe { (self.0.getFPS)() } }
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
	fn into(self) -> c_uint { self.as_uint() }
}

impl From<c_uint> for DisplayScale {
	#[inline(always)]
	fn from(scale: c_uint) -> Self { Self::from_uint(scale) }
}

impl core::fmt::Display for DisplayScale {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { (*self as u8).fmt(f) }
}

impl DisplayScale {
	#[inline(always)]
	pub const fn as_u8(self) -> u8 { self as u8 }
	#[inline(always)]
	pub const fn as_int(self) -> c_int { self.as_u8() as _ }
	#[inline(always)]
	pub const fn as_uint(self) -> c_uint { self.as_u8() as _ }

	pub const fn from_uint(scale: c_uint) -> Self {
		match scale {
			1 => DisplayScale::Normal,
			2 => DisplayScale::Double,
			4 => DisplayScale::Quad,
			8 => DisplayScale::Eight,
			_ => panic!("invalid scale value"),
		}
	}
}
