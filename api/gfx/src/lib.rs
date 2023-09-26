//! Playdate graphics API
#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]

extern crate sys;
extern crate alloc;
pub extern crate color;

pub mod error;
pub mod text;
pub mod bitmap {
	mod bitmap;
	pub mod api;
	pub mod table;
	pub use bitmap::*;
}
pub mod video;
pub mod api;

use core::ffi::c_float;
use core::ffi::c_int;
use error::ApiError;

pub use sys::ffi::LCDBitmapFlip as BitmapFlip;
pub use sys::ffi::LCDBitmapDrawMode as BitmapDrawMode;
pub use sys::ffi::LCDLineCapStyle as LineCapStyle;

use sys::ffi::LCDColor;
use sys::ffi::LCD_ROWS;
use sys::ffi::LCD_ROWSIZE;
use sys::ffi::LCDPolygonFillRule;
use sys::ffi::LCDSolidColor;

pub use bitmap::debug_bitmap;
pub use bitmap::display_buffer_bitmap;
pub use bitmap::copy_frame_buffer_bitmap;
pub use bitmap::set_stencil;
pub use bitmap::set_stencil_tiled;
pub use bitmap::set_draw_mode;
pub use bitmap::push_context;
pub use bitmap::pop_context;


unsafe fn as_slice_mut(buf: *mut u8) -> Result<&'static mut [u8], ApiError> {
	if !buf.is_null() {
		Ok(core::slice::from_raw_parts_mut(
		                                   buf,
		                                   (LCD_ROWSIZE * LCD_ROWS) as usize,
		))
	} else {
		Err(sys::error::NullPtrError.into())
	}
}


/// Returns the current display frame buffer.
/// Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored.
/// Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.
///
/// This function is shorthand for [`Graphics::get_frame`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::getFrame`].
#[doc(alias = "sys::ffi::playdate_graphics::getFrame")]
#[inline(always)]
pub fn get_frame() -> Result<&'static mut [u8], ApiError> { Graphics::Default().get_frame() }


/// Returns the raw bits in the display buffer,
/// __the last completed frame__.
///
/// This function is shorthand for [`Graphics::get_display_frame`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayFrame`].
#[doc(alias = "sys::ffi::playdate_graphics::getDisplayFrame")]
#[inline(always)]
pub fn get_display_frame() -> Result<&'static mut [u8], ApiError> { Graphics::Default().get_display_frame() }

/// After updating pixels in the buffer returned by [`get_frame`],
/// you must tell the graphics system which rows were updated.
///
/// This function marks a contiguous range of rows as updated
/// (e.g., `markUpdatedRows(0, LCD_ROWS-1)` tells the system to update the entire display).
///
/// Both `start` and `end` are __included__ in the range.
///
/// This function is shorthand for [`Graphics::mark_updated_rows`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::markUpdatedRows`].
#[doc(alias = "sys::ffi::playdate_graphics::markUpdatedRows")]
#[inline(always)]
pub fn mark_updated_rows(start: c_int, end: c_int) { Graphics::Default().mark_updated_rows(start, end) }

/// Manually flushes the current frame buffer out to the display.
/// This function is automatically called after each pass through the run loop,
/// so there shouldn’t be any need to call it yourself.
///
/// This function is shorthand for [`Graphics::display`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::display`].
#[doc(alias = "sys::ffi::playdate_graphics::display")]
#[inline(always)]
pub fn display() { Graphics::Default().display() }

/// Clears the entire display, filling it with `color`.
///
/// This function is shorthand for [`Graphics::always`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::clear`].
#[doc(alias = "sys::ffi::playdate_graphics::clear")]
#[inline(always)]
pub fn clear(color: color::Color) { clear_raw(color.into()) }


/// Clears the entire display, filling it with `color`.
///
/// Same as [`clear`], but without conversion `Color` -> `LCDColor`.
/// That conversion is really cheap,
/// so this function is useful if you're working with `LCDColor` directly.
///
/// This function is shorthand for [`Graphics::clear_raw`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::clear`].
#[doc(alias = "sys::ffi::playdate_graphics::clear")]
#[inline(always)]
pub fn clear_raw(color: LCDColor) { Graphics::Default().clear_raw(color) }

/// Sets the current clip rect in __screen__ coordinates.
///
/// This function is shorthand for [`Graphics::set_screen_clip_rect`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setScreenClipRect`].
#[doc(alias = "sys::ffi::playdate_graphics::setScreenClipRect")]
#[inline(always)]
pub fn set_screen_clip_rect(x: c_int, y: c_int, width: c_int, height: c_int) {
	Graphics::Default().set_screen_clip_rect(x, y, width, height)
}

/// Offsets the origin point for all drawing calls to `x, y` (can be negative).
///
/// This is useful, for example, for centering a "camera"
/// on a sprite that is moving around a world larger than the screen.
///
/// This function is shorthand for [`Graphics::set_draw_offset`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setDrawOffset`].
#[doc(alias = "sys::ffi::playdate_graphics::setDrawOffset")]
#[inline(always)]
pub fn set_draw_offset(dx: c_int, dy: c_int) { Graphics::Default().set_draw_offset(dx, dy) }

/// Sets the current clip rect, using __world__ coordinates that is,
/// the given rectangle will be translated by the current drawing offset.
///
/// The clip rect is cleared at the beginning of each update.
///
/// This function is shorthand for [`Graphics::set_clip_rect`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setClipRect`].
#[doc(alias = "sys::ffi::playdate_graphics::setClipRect")]
#[inline(always)]
pub fn set_clip_rect(x: c_int, y: c_int, width: c_int, height: c_int) {
	Graphics::Default().set_clip_rect(x, y, width, height)
}

/// Clears the current clip rect.
///
/// This function is shorthand for [`Graphics::clear_clip_rect`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::clearClipRect`].
#[doc(alias = "sys::ffi::playdate_graphics::clearClipRect")]
#[inline(always)]
pub fn clear_clip_rect() { Graphics::Default().clear_clip_rect() }

/// Sets the background color shown when the display is offset
/// or for clearing dirty areas in the sprite system.
///
/// This function is shorthand for [`Graphics::set_background_color`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setBackgroundColor`].
#[doc(alias = "sys::ffi::playdate_graphics::setBackgroundColor")]
#[inline(always)]
pub fn set_background_color(color: LCDSolidColor) { Graphics::Default().set_background_color(color) }


//
// Geometry
//

/// Fills the polygon with vertices at the given coordinates
/// (an array of `2 * num_points` ints containing alternating x and y values)
/// using the given `color` and fill, or winding, `rule`.
///
/// See [wikipedia](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule.
///
/// This function is shorthand for [`Graphics::fill_polygon`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::fillPolygon`].
#[doc(alias = "sys::ffi::playdate_graphics::fillPolygon")]
#[inline(always)]
pub fn fill_polygon(num_points: c_int, coords: &mut [c_int], color: LCDColor, rule: LCDPolygonFillRule) {
	Graphics::Default().fill_polygon(num_points, coords, color, rule)
}

/// Draws a line from `x1, y1` to `x2, y2` with a stroke width of `width`.
///
/// This function is shorthand for [`Graphics::draw_line`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::drawLine`].
#[doc(alias = "sys::ffi::playdate_graphics::drawLine")]
#[inline(always)]
pub fn draw_line(x1: c_int, y1: c_int, x2: c_int, y2: c_int, width: c_int, color: LCDColor) {
	Graphics::Default().draw_line(x1, y1, x2, y2, width, color)
}

/// Draws a filled triangle with points at `x1, y1`, `x2, y2`, and `x3, y3`.
///
/// This function is shorthand for [`Graphics::fill_triangle`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::fillTriangle`].
#[doc(alias = "sys::ffi::playdate_graphics::fillTriangle")]
#[inline(always)]
pub fn fill_triangle(x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int, color: LCDColor) {
	Graphics::Default().fill_triangle(x1, y1, x2, y2, x3, y3, color);
}

/// Draws a `width` by `height` rect at `x, y`.
///
/// This function is shorthand for [`Graphics::draw_rect`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::drawRect`].
#[doc(alias = "sys::ffi::playdate_graphics::drawRect")]
#[inline(always)]
pub fn draw_rect(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
	Graphics::Default().draw_rect(x, y, width, height, color)
}

/// Draws a filled `width` by `height` rect at `x, y`.
///
/// This function is shorthand for [`Graphics::fill_rect`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::fillRect`].
#[doc(alias = "sys::ffi::playdate_graphics::fillRect")]
#[inline(always)]
pub fn fill_rect(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
	Graphics::Default().fill_rect(x, y, width, height, color)
}

/// Draw an ellipse stroked inside the rect.
///
/// Draws an ellipse inside the rectangle `x, y, width, height` of width `line_width`
/// (inset from the rectangle bounds).
///
/// If `start_angle != end_angle`, this draws an arc between the given angles.
///
/// Angles are given in degrees, clockwise from due north.
///
/// This function is shorthand for [`Graphics::draw_ellipse`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::drawEllipse`].
#[doc(alias = "sys::ffi::playdate_graphics::drawEllipse")]
#[inline(always)]
pub fn draw_ellipse(x: c_int,
                    y: c_int,
                    width: c_int,
                    height: c_int,
                    line_width: c_int,
                    start_angle: c_float,
                    end_angle: c_float,
                    color: LCDColor) {
	Graphics::Default().draw_ellipse(x, y, width, height, line_width, start_angle, end_angle, color)
}

/// Fills an ellipse inside the rectangle `x, y, width, height`.
///
/// If `start_angle != end_angle`, this draws a wedge/Pacman between the given angles.
///
/// Angles are given in degrees, clockwise from due north.
///
/// This function is shorthand for [`Graphics::fill_ellipse`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::fillEllipse`].
#[doc(alias = "sys::ffi::playdate_graphics::fillEllipse")]
#[inline(always)]
pub fn fill_ellipse(x: c_int,
                    y: c_int,
                    width: c_int,
                    height: c_int,
                    start_angle: c_float,
                    end_angle: c_float,
                    color: LCDColor) {
	Graphics::Default().fill_ellipse(x, y, width, height, start_angle, end_angle, color)
}


/// Sets the end cap style used in the line drawing functions.
///
/// This function is shorthand for [`Graphics::set_line_cap_style`],
/// using default ZST end-point.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setLineCapStyle`].
#[doc(alias = "sys::ffi::playdate_graphics::setLineCapStyle")]
#[inline(always)]
pub fn set_line_cap_style(end_cap_style: LineCapStyle) { Graphics::Default().set_line_cap_style(end_cap_style) }


#[derive(Debug, Clone, Copy)]
pub struct Graphics<Api = api::Default>(Api);

impl Graphics<api::Default> {
	/// Creates default [`Graphics`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Graphics<api::Cache> {
	/// Creates [`Graphics`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Graphics<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Graphics<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Graphics<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}

impl<Api: api::Api> Graphics<Api> {
	/// Returns the current display frame buffer.
	/// Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored.
	/// Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getFrame`].
	#[doc(alias = "sys::ffi::playdate_graphics::getFrame")]
	pub fn get_frame(&self) -> Result<&'static mut [u8], ApiError> {
		let f = self.0.get_frame();
		unsafe { as_slice_mut(f()) }
	}


	/// Returns the raw bits in the display buffer,
	/// __the last completed frame__.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayFrame`].
	#[doc(alias = "sys::ffi::playdate_graphics::getDisplayFrame")]
	pub fn get_display_frame(&self) -> Result<&'static mut [u8], ApiError> {
		let f = self.0.get_display_frame();
		unsafe { as_slice_mut(f()) }
	}

	/// After updating pixels in the buffer returned by [`get_frame`],
	/// you must tell the graphics system which rows were updated.
	///
	/// This function marks a contiguous range of rows as updated
	/// (e.g., `markUpdatedRows(0, LCD_ROWS-1)` tells the system to update the entire display).
	///
	/// Both `start` and `end` are __included__ in the range.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::markUpdatedRows`].
	#[doc(alias = "sys::ffi::playdate_graphics::markUpdatedRows")]
	pub fn mark_updated_rows(&self, start: c_int, end: c_int) {
		let f = self.0.mark_updated_rows();
		unsafe { f(start, end) }
	}

	/// Manually flushes the current frame buffer out to the display.
	/// This function is automatically called after each pass through the run loop,
	/// so there shouldn’t be any need to call it yourself.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::display`].
	#[doc(alias = "sys::ffi::playdate_graphics::display")]
	pub fn display(&self) {
		let f = self.0.display();
		unsafe { f() }
	}

	/// Clears the entire display, filling it with `color`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::clear`].
	#[doc(alias = "sys::ffi::playdate_graphics::clear")]
	#[inline(always)]
	pub fn clear(&self, color: color::Color) { clear_raw(color.into()) }


	/// Clears the entire display, filling it with `color`.
	///
	/// Same as [`clear`], but without conversion `Color` -> `LCDColor`.
	/// That conversion is really cheap,
	/// so this function is useful if you're working with `LCDColor` directly.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::clear`].
	#[doc(alias = "sys::ffi::playdate_graphics::clear")]
	pub fn clear_raw(&self, color: LCDColor) {
		let f = self.0.clear();
		unsafe { f(color) }
	}

	/// Sets the current clip rect in __screen__ coordinates.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setScreenClipRect`].
	#[doc(alias = "sys::ffi::playdate_graphics::setScreenClipRect")]
	pub fn set_screen_clip_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
		let f = self.0.set_screen_clip_rect();
		unsafe { f(x, y, width, height) }
	}

	/// Offsets the origin point for all drawing calls to `x, y` (can be negative).
	///
	/// This is useful, for example, for centering a "camera"
	/// on a sprite that is moving around a world larger than the screen.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setDrawOffset`].
	#[doc(alias = "sys::ffi::playdate_graphics::setDrawOffset")]
	pub fn set_draw_offset(&self, dx: c_int, dy: c_int) {
		let f = self.0.set_draw_offset();
		unsafe { f(dx, dy) }
	}

	/// Sets the current clip rect, using __world__ coordinates that is,
	/// the given rectangle will be translated by the current drawing offset.
	///
	/// The clip rect is cleared at the beginning of each update.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setClipRect`].
	#[doc(alias = "sys::ffi::playdate_graphics::setClipRect")]
	pub fn set_clip_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
		let f = self.0.set_clip_rect();
		unsafe { f(x, y, width, height) }
	}

	/// Clears the current clip rect.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::clearClipRect`].
	#[doc(alias = "sys::ffi::playdate_graphics::clearClipRect")]
	pub fn clear_clip_rect(&self) {
		let f = self.0.clear_clip_rect();
		unsafe { f() }
	}

	/// Sets the background color shown when the display is offset
	/// or for clearing dirty areas in the sprite system.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setBackgroundColor`].
	#[doc(alias = "sys::ffi::playdate_graphics::setBackgroundColor")]
	pub fn set_background_color(&self, color: LCDSolidColor) {
		let f = self.0.set_background_color();
		unsafe { f(color) }
	}


	//
	// Geometry
	//

	/// Fills the polygon with vertices at the given coordinates
	/// (an array of `2 * num_points` ints containing alternating x and y values)
	/// using the given `color` and fill, or winding, `rule`.
	///
	/// See [wikipedia](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::fillPolygon`].
	#[doc(alias = "sys::ffi::playdate_graphics::fillPolygon")]
	pub fn fill_polygon(&self,
	                    num_points: c_int,
	                    coords: &mut [c_int],
	                    color: LCDColor,
	                    rule: LCDPolygonFillRule) {
		let f = self.0.fill_polygon();
		unsafe { f(num_points, coords.as_mut_ptr(), color, rule) }
	}

	/// Draws a line from `x1, y1` to `x2, y2` with a stroke width of `width`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawLine`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawLine")]
	pub fn draw_line(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, width: c_int, color: LCDColor) {
		let f = self.0.draw_line();
		unsafe { f(x1, y1, x2, y2, width, color) }
	}

	/// Draws a filled triangle with points at `x1, y1`, `x2, y2`, and `x3, y3`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::fillTriangle`].
	#[doc(alias = "sys::ffi::playdate_graphics::fillTriangle")]
	pub fn fill_triangle(&self,
	                     x1: c_int,
	                     y1: c_int,
	                     x2: c_int,
	                     y2: c_int,
	                     x3: c_int,
	                     y3: c_int,
	                     color: LCDColor) {
		let f = self.0.fill_triangle();
		unsafe { f(x1, y1, x2, y2, x3, y3, color) }
	}

	/// Draws a `width` by `height` rect at `x, y`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawRect`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawRect")]
	pub fn draw_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		let f = self.0.draw_rect();
		unsafe { f(x, y, width, height, color) }
	}

	/// Draws a filled `width` by `height` rect at `x, y`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::fillRect`].
	#[doc(alias = "sys::ffi::playdate_graphics::fillRect")]
	pub fn fill_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		let f = self.0.fill_rect();
		unsafe { f(x, y, width, height, color) }
	}

	/// Draw an ellipse stroked inside the rect.
	///
	/// Draws an ellipse inside the rectangle `x, y, width, height` of width `line_width`
	/// (inset from the rectangle bounds).
	///
	/// If `start_angle != end_angle`, this draws an arc between the given angles.
	///
	/// Angles are given in degrees, clockwise from due north.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawEllipse`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawEllipse")]
	pub fn draw_ellipse(&self,
	                    x: c_int,
	                    y: c_int,
	                    width: c_int,
	                    height: c_int,
	                    line_width: c_int,
	                    start_angle: c_float,
	                    end_angle: c_float,
	                    color: LCDColor) {
		let f = self.0.draw_ellipse();
		unsafe { f(x, y, width, height, line_width, start_angle, end_angle, color) }
	}

	/// Fills an ellipse inside the rectangle `x, y, width, height`.
	///
	/// If `start_angle != end_angle`, this draws a wedge/Pacman between the given angles.
	///
	/// Angles are given in degrees, clockwise from due north.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::fillEllipse`].
	#[doc(alias = "sys::ffi::playdate_graphics::fillEllipse")]
	pub fn fill_ellipse(&self,
	                    x: c_int,
	                    y: c_int,
	                    width: c_int,
	                    height: c_int,
	                    start_angle: c_float,
	                    end_angle: c_float,
	                    color: LCDColor) {
		let f = self.0.fill_ellipse();
		unsafe { f(x, y, width, height, start_angle, end_angle, color) }
	}


	/// Sets the end cap style used in the line drawing functions.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setLineCapStyle`].
	#[doc(alias = "sys::ffi::playdate_graphics::setLineCapStyle")]
	pub fn set_line_cap_style(&self, end_cap_style: LineCapStyle) {
		let f = self.0.set_line_cap_style();
		unsafe { f(end_cap_style) }
	}
}


pub trait BitmapFlipExt {
	#![allow(non_upper_case_globals)]
	const Unflipped: BitmapFlip = BitmapFlip::kBitmapUnflipped;
	const FlippedX: BitmapFlip = BitmapFlip::kBitmapFlippedX;
	const FlippedY: BitmapFlip = BitmapFlip::kBitmapFlippedY;
	const FlippedXY: BitmapFlip = BitmapFlip::kBitmapFlippedXY;
}

impl BitmapFlipExt for BitmapFlip {}


pub trait BitmapDrawModeExt {
	#![allow(non_upper_case_globals)]
	const Copy: BitmapDrawMode = BitmapDrawMode::kDrawModeCopy;
	const WhiteTransparent: BitmapDrawMode = BitmapDrawMode::kDrawModeWhiteTransparent;
	const BlackTransparent: BitmapDrawMode = BitmapDrawMode::kDrawModeBlackTransparent;
	const FillWhite: BitmapDrawMode = BitmapDrawMode::kDrawModeFillWhite;
	const FillBlack: BitmapDrawMode = BitmapDrawMode::kDrawModeFillBlack;
	const XOR: BitmapDrawMode = BitmapDrawMode::kDrawModeXOR;
	const NXOR: BitmapDrawMode = BitmapDrawMode::kDrawModeNXOR;
	const Inverted: BitmapDrawMode = BitmapDrawMode::kDrawModeInverted;
}

impl BitmapDrawModeExt for BitmapDrawMode {}


pub trait LineCapStyleExt {
	#![allow(non_upper_case_globals)]
	const Butt: LineCapStyle = LineCapStyle::kLineCapStyleButt;
	const Square: LineCapStyle = LineCapStyle::kLineCapStyleSquare;
	const Round: LineCapStyle = LineCapStyle::kLineCapStyleRound;
}
impl LineCapStyleExt for LineCapStyle {}
