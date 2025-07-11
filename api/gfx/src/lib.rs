//! Playdate graphics API
#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_trait_impl, const_deref)]
#![feature(allocator_api)]


extern crate alloc;
extern crate sys;
extern crate callback;
pub extern crate color;

pub mod error;
pub mod bitmap;
pub mod text;
pub mod video;

use core::ffi::c_float;
use core::ffi::c_int;
use core::ops::Deref;

use sys::error::NullPtrError;
pub use sys::ffi::BitmapFlip;
pub use sys::ffi::BitmapDrawMode;
pub use sys::ffi::LineCapStyle;

use sys::ffi::Color as LCDColor;
use sys::ffi::LCD_ROWS;
use sys::ffi::LCD_ROWSIZE;
use sys::ffi::PolygonFillRule;
use sys::ffi::SolidColor;


unsafe fn as_slice_mut(buf: *mut u8) -> Result<&'static mut [u8], NullPtrError> {
	if !buf.is_null() {
		Ok(core::slice::from_raw_parts_mut(
		                                   buf,
		                                   (LCD_ROWSIZE * LCD_ROWS) as usize,
		))
	} else {
		Err(NullPtrError)
	}
}


type Api = &'static sys::ffi::PlaydateGraphics;


#[derive(Clone, Copy)]
pub struct Graphics(Api);

impl const Deref for Graphics {
	type Target = Api;
	fn deref(&self) -> &Self::Target { &self.0 }
}


impl Default for Graphics {
	fn default() -> Self { Self(sys::macros::api!(graphics)) }
}

impl Graphics {
	pub const fn new(api: Api) -> Self { Self(api) }
}


impl Graphics {
	/// Sets the pixel at `(x,y)` in the current drawing context (by default the screen) to the given `color`.
	/// Be aware that setting a pixel at a time is not very efficient:
	/// In our testing, more than around 20,000 calls in a tight loop will drop the frame rate below 30 fps.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setPixel`]
	#[doc(alias = "sys::ffi::PlaydateGraphics::setPixel")]
	#[inline(always)]
	pub fn set_pixel<'c>(&self, x: c_int, y: c_int, color: impl color::IntoColor<'c>) {
		self.set_pixel_raw(x, y, color.into_color())
	}

	/// Same as [`set_pixel`][Graphics::set_pixel], but without conversion [`Color`][color::Color] -> [`LCDColor`].
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setPixel`]
	#[doc(alias = "sys::ffi::PlaydateGraphics::setPixel")]
	#[inline(always)]
	pub fn set_pixel_raw(&self, x: c_int, y: c_int, color: color::LcdColor) {
		unsafe { (self.0.setPixel)(x, y, color.into_raw()) }
	}


	/// Returns the current display frame buffer.
	/// Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored.
	/// Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getFrame`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getFrame")]
	pub fn raw_frame_buffer(&self) -> Result<&'static mut [u8], NullPtrError> {
		unsafe { as_slice_mut((self.0.getFrame)()) }
	}


	/// Returns the raw bits in the display buffer,
	/// __the last completed frame__.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getDisplayFrame`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getDisplayFrame")]
	pub fn raw_frame_buffer_last(&self) -> Result<&'static mut [u8], NullPtrError> {
		unsafe { as_slice_mut((self.0.getDisplayFrame)()) }
	}

	/// After updating pixels in the buffer returned by [`get_frame`],
	/// you must tell the graphics system which rows were updated.
	///
	/// This function marks a contiguous range of rows as updated
	/// (e.g., `markUpdatedRows(0, LCD_ROWS-1)` tells the system to update the entire display).
	///
	/// Both `start` and `end` are __included__ in the range.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::markUpdatedRows`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::markUpdatedRows")]
	pub fn mark_updated_rows(&self, start: c_int, end: c_int) { unsafe { (self.0.markUpdatedRows)(start, end) } }

	/// Manually flushes the current frame buffer out to the display.
	/// This function is automatically called after each pass through the run loop,
	/// so there shouldn’t be any need to call it yourself.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::display`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::display")]
	pub fn display(&self) { unsafe { (self.0.display)() } }

	/// Clears the entire display, filling it with `color`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::clear`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::clear")]
	#[inline(always)]
	pub fn clear<'c>(&self, color: impl color::IntoColor<'c>) {
		unsafe { (self.0.clear)(color.into_color().into_raw()) }
	}

	/// Sets the current clip rect in __screen__ coordinates.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setScreenClipRect`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setScreenClipRect")]
	pub fn set_screen_clip_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
		unsafe { (self.0.setScreenClipRect)(x, y, width, height) }
	}

	/// Offsets the origin point for all drawing calls to `x, y` (can be negative).
	///
	/// This is useful, for example, for centering a "camera"
	/// on a sprite that is moving around a world larger than the screen.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setDrawOffset`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setDrawOffset")]
	pub fn set_draw_offset(&self, dx: c_int, dy: c_int) { unsafe { (self.0.setDrawOffset)(dx, dy) } }

	/// Sets the current clip rect, using __world__ coordinates that is,
	/// the given rectangle will be translated by the current drawing offset.
	///
	/// The clip rect is cleared at the beginning of each update.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setClipRect`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setClipRect")]
	pub fn set_clip_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
		unsafe { (self.0.setClipRect)(x, y, width, height) }
	}

	/// Clears the current clip rect.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::clearClipRect`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::clearClipRect")]
	pub fn clear_clip_rect(&self) { unsafe { (self.0.clearClipRect)() } }

	/// Sets the background color shown when the display is offset
	/// or for clearing dirty areas in the sprite system.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setBackgroundColor`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setBackgroundColor")]
	pub fn set_background_color(&self, color: SolidColor) { unsafe { (self.0.setBackgroundColor)(color) } }


	//
	// Geometry
	//

	/// Fills the polygon with vertices at the given coordinates
	/// (an array of `2 * num_points` ints containing alternating x and y values)
	/// using the given `color` and fill, or winding, `rule`.
	///
	/// See [wikipedia](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::fillPolygon`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::fillPolygon")]
	pub fn fill_polygon(&self, num_points: c_int, coords: &mut [c_int], color: LCDColor, rule: PolygonFillRule) {
		unsafe { (self.0.fillPolygon)(num_points, coords.as_mut_ptr(), color, rule) }
	}

	/// Draws a line from `x1, y1` to `x2, y2` with a stroke width of `width`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawLine`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawLine")]
	pub fn draw_line(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, width: c_int, color: LCDColor) {
		unsafe { (self.0.drawLine)(x1, y1, x2, y2, width, color) }
	}

	/// Draws a filled triangle with points at `x1, y1`, `x2, y2`, and `x3, y3`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::fillTriangle`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::fillTriangle")]
	pub fn fill_triangle(&self,
	                     x1: c_int,
	                     y1: c_int,
	                     x2: c_int,
	                     y2: c_int,
	                     x3: c_int,
	                     y3: c_int,
	                     color: LCDColor) {
		unsafe { (self.0.fillTriangle)(x1, y1, x2, y2, x3, y3, color) }
	}

	/// Draws a `width` by `height` rect at `x, y`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawRect`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawRect")]
	pub fn draw_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		unsafe { (self.0.drawRect)(x, y, width, height, color) }
	}

	/// Draws a filled `width` by `height` rect at `x, y`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::fillRect`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::fillRect")]
	pub fn fill_rect(&self, x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		unsafe { (self.0.fillRect)(x, y, width, height, color) }
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
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawEllipse`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawEllipse")]
	pub fn draw_ellipse(&self,
	                    x: c_int,
	                    y: c_int,
	                    width: c_int,
	                    height: c_int,
	                    line_width: c_int,
	                    start_angle: c_float,
	                    end_angle: c_float,
	                    color: LCDColor) {
		let f = self.0.drawEllipse;
		unsafe { f(x, y, width, height, line_width, start_angle, end_angle, color) }
	}

	/// Fills an ellipse inside the rectangle `x, y, width, height`.
	///
	/// If `start_angle != end_angle`, this draws a wedge/Pacman between the given angles.
	///
	/// Angles are given in degrees, clockwise from due north.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::fillEllipse`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::fillEllipse")]
	pub fn fill_ellipse(&self,
	                    x: c_int,
	                    y: c_int,
	                    width: c_int,
	                    height: c_int,
	                    start_angle: c_float,
	                    end_angle: c_float,
	                    color: LCDColor) {
		let f = self.0.fillEllipse;
		unsafe { f(x, y, width, height, start_angle, end_angle, color) }
	}


	/// Sets the end cap style used in the line drawing functions.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setLineCapStyle`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setLineCapStyle")]
	pub fn set_line_cap_style(&self, end_cap_style: LineCapStyle) {
		unsafe { (self.0.setLineCapStyle)(end_cap_style) }
	}
}


#[const_trait]
trait AsRef<'ext, T: ?Sized> {
	fn as_ref<'t>(&'t self) -> &'t T
		where 'ext: 't;
}

#[const_trait]
trait AsMut<'ext, T: ?Sized>: AsRef<'ext, T> {
	fn as_mut<'t>(&'t mut self) -> &'t mut T
		where 'ext: 't;
}


impl<'e, T: core::convert::AsRef<U>, U> AsRef<'e, U> for T {
	#[inline]
	fn as_ref<'t>(&'t self) -> &'t U
		where 'e: 't {
		core::convert::AsRef::as_ref(self)
	}
}

impl<'e, T, U> AsMut<'e, U> for T where T: core::convert::AsMut<U> + core::convert::AsRef<U> {
	#[inline]
	fn as_mut<'t>(&'t mut self) -> &'t mut U
		where 'e: 't {
		core::convert::AsMut::as_mut(self)
	}
}
