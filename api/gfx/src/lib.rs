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

pub use sys::ffi::LCDBitmapFlip as BitmapFlip;
pub use sys::ffi::LCDBitmapDrawMode as BitmapDrawMode;

pub use bitmap::get_debug_bitmap;
pub use bitmap::get_display_buffer_bitmap;
pub use bitmap::copy_frame_buffer_bitmap;

pub use bitmap::set_stencil;
pub use bitmap::set_stencil_tiled;
pub use bitmap::set_draw_mode;
pub use bitmap::push_context;
pub use bitmap::pop_context;
use sys::ffi::LCDPolygonFillRule;
use sys::ffi::LCDSolidColor;


use core::ffi::c_float;
use core::ffi::c_int;
use error::ApiError;
use sys::ffi::LCDColor;
use sys::ffi::LCD_ROWS;
use sys::ffi::LCD_ROWSIZE;


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
/// Equivalent to [`sys::ffi::playdate_graphics::getFrame`].
pub fn get_frame() -> Result<&'static mut [u8], ApiError> {
	let f = *sys::api!(graphics.getFrame);
	unsafe { as_slice_mut(f()) }
}


/// Returns the raw bits in the display buffer,
/// __the last completed frame__.
///
/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayFrame`].
pub fn get_display_frame() -> Result<&'static mut [u8], ApiError> {
	let f = *sys::api!(graphics.getDisplayFrame);
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
pub fn mark_updated_rows(start: c_int, end: c_int) {
	let f = *sys::api!(graphics.markUpdatedRows);
	unsafe { f(start, end) }
}

/// Manually flushes the current frame buffer out to the display.
/// This function is automatically called after each pass through the run loop,
/// so there shouldn’t be any need to call it yourself.
///
/// Equivalent to [`sys::ffi::playdate_graphics::display`].
pub fn display() {
	let f = *sys::api!(graphics.display);
	unsafe { f() }
}

/// Clears the entire display, filling it with `color`.
///
/// Equivalent to [`sys::ffi::playdate_graphics::clear`].
#[inline(always)]
pub fn clear(color: color::Color) { clear_raw(color.into()) }


/// Clears the entire display, filling it with `color`.
///
/// Same as [`clear`], but without conversion `Color` -> `LCDColor`.
/// That conversion is really cheap,
/// so this function is useful if you're working with `LCDColor` directly.
///
/// Equivalent to [`sys::ffi::playdate_graphics::clear`].
pub fn clear_raw(color: LCDColor) {
	let f = *sys::api!(graphics.clear);
	unsafe { f(color) }
}

/// Sets the current clip rect in __screen__ coordinates.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setScreenClipRect`].
pub fn set_screen_clip_rect(x: c_int, y: c_int, width: c_int, height: c_int) {
	let f = *sys::api!(graphics.setScreenClipRect);
	unsafe { f(x, y, width, height) }
}

/// Equivalent to [`sys::ffi::playdate_graphics::setDrawOffset`].
pub fn set_draw_offset(dx: c_int, dy: c_int) {
	let f = *sys::api!(graphics.setDrawOffset);
	unsafe { f(dx, dy) }
}

/// Sets the current clip rect, using __world__ coordinates that is,
/// the given rectangle will be translated by the current drawing offset.
///
/// The clip rect is cleared at the beginning of each update.
///
/// Equivalent to [`sys::ffi::playdate_graphics::setClipRect`].
pub fn set_clip_rect(x: c_int, y: c_int, width: c_int, height: c_int) {
	let f = *sys::api!(graphics.setClipRect);
	unsafe { f(x, y, width, height) }
}

/// Equivalent to [`sys::ffi::playdate_graphics::clearClipRect`].
pub fn clear_clip_rect() {
	let f = *sys::api!(graphics.clearClipRect);
	unsafe { f() }
}


/// Equivalent to [`sys::ffi::playdate_graphics::setBackgroundColor`].
pub fn set_background_color(color: LCDSolidColor) {
	let f = *sys::api!(graphics.setBackgroundColor);
	unsafe { f(color) }
}


//
// Geometry
//

/// Fills the polygon with vertices at the given coordinates
/// (an array of `2 * num_points` ints containing alternating x and y values)
/// using the given `color` and fill, or winding, `rule`.
///
/// See [https://en.wikipedia.org/wiki/Nonzero-rule](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule.
///
/// Equivalent to [`sys::ffi::playdate_graphics::fillPolygon`].
pub fn fill_polygon(num_points: c_int, coords: &mut [c_int], color: LCDColor, rule: LCDPolygonFillRule) {
	let f = *sys::api!(graphics.fillPolygon);
	unsafe { f(num_points, coords.as_mut_ptr(), color, rule) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::drawLine`].
pub fn draw_line(x1: c_int, y1: c_int, x2: c_int, y2: c_int, width: c_int, color: LCDColor) {
	let f = *sys::api!(graphics.drawLine);
	unsafe { f(x1, y1, x2, y2, width, color) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::fillTriangle`].
pub fn fill_triangle(x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int, color: LCDColor) {
	let f = *sys::api!(graphics.fillTriangle);
	unsafe { f(x1, y1, x2, y2, x3, y3, color) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::drawRect`].
pub fn draw_rect(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
	let f = *sys::api!(graphics.drawRect);
	unsafe { f(x, y, width, height, color) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::fillRect`].
pub fn fill_rect(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
	let f = *sys::api!(graphics.fillRect);
	unsafe { f(x, y, width, height, color) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::drawEllipse`].
pub fn draw_ellipse(x: c_int,
                    y: c_int,
                    width: c_int,
                    height: c_int,
                    line_width: c_int,
                    start_angle: c_float,
                    end_angle: c_float,
                    color: LCDColor) {
	let f = *sys::api!(graphics.drawEllipse);
	unsafe { f(x, y, width, height, line_width, start_angle, end_angle, color) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::fillEllipse`].
pub fn fill_ellipse(x: c_int,
                    y: c_int,
                    width: c_int,
                    height: c_int,
                    start_angle: c_float,
                    end_angle: c_float,
                    color: LCDColor) {
	let f = *sys::api!(graphics.fillEllipse);
	unsafe { f(x, y, width, height, start_angle, end_angle, color) }
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
