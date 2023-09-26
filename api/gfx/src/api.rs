//! Global Playdate graphics API.

use core::ffi::c_void;
use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ptr::NonNull;

use sys::ffi::LCDBitmap;
use sys::ffi::LCDColor;
use sys::ffi::LCDRect;
use sys::ffi::LCDLineCapStyle;
use sys::ffi::LCDPolygonFillRule;
use sys::ffi::playdate_graphics;
use sys::ffi::LCDBitmapDrawMode;
use sys::ffi::LCDSolidColor;
use sys::ffi::LCDBitmapFlip;
use sys::ffi::LCDBitmapTable;
use sys::ffi::LCDFontPage;
use sys::ffi::LCDFontGlyph;
use sys::ffi::LCDFont;
use sys::ffi::PDStringEncoding;
use sys::ffi::LCDFontData;
use sys::ffi::playdate_video;


/// Default graphics api end-point, ZST.
///
/// All calls approximately costs ~3 derefs.
#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;
impl crate::bitmap::api::Api for Default {}
impl crate::bitmap::table::api::Api for Default {}
impl crate::text::api::Api for Default {}

impl Api for Default {
	#[inline(always)]
	fn video<VApi: crate::video::api::Api + From<*const playdate_video>>(&self) -> VApi {
		VApi::from(sys::api!(graphics.video))
	}
}


/// Cached graphics api end-point.
///
/// Stores one reference, so size on stack is eq `usize`.
///
/// All calls approximately costs ~1 deref.
#[derive(Debug, Clone, Copy)]
pub struct Cache(&'static playdate_graphics);

impl core::default::Default for Cache {
	fn default() -> Self { Self(sys::api!(graphics)) }
}

impl From<*const playdate_graphics> for Cache {
	#[inline(always)]
	fn from(ptr: *const playdate_graphics) -> Self { Self(unsafe { ptr.as_ref() }.expect("system")) }
}

impl From<&'static playdate_graphics> for Cache {
	#[inline(always)]
	fn from(r: &'static playdate_graphics) -> Self { Self(r) }
}

impl From<NonNull<playdate_graphics>> for Cache {
	#[inline(always)]
	fn from(ptr: NonNull<playdate_graphics>) -> Self { Self(unsafe { ptr.as_ref() }) }
}

impl From<&'_ NonNull<playdate_graphics>> for Cache {
	#[inline(always)]
	fn from(ptr: &NonNull<playdate_graphics>) -> Self { Self(unsafe { ptr.as_ref() }) }
}


impl crate::text::api::Api for Cache {
	/// Equivalent to [`sys::ffi::playdate_graphics::drawText`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawText")]
	#[inline(always)]
	fn draw_text(
		&self)
		-> unsafe extern "C" fn(text: *const c_void,
		                        len: usize,
		                        encoding: PDStringEncoding,
		                        x: c_int,
		                        y: c_int) -> c_int {
		self.0.drawText.expect("drawText")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getTextWidth`]
	#[doc(alias = "sys::ffi::playdate_graphics::getTextWidth")]
	#[inline(always)]
	fn get_text_width(
		&self)
		-> unsafe extern "C" fn(font: *mut LCDFont,
		                        text: *const c_void,
		                        len: usize,
		                        encoding: PDStringEncoding,
		                        tracking: c_int) -> c_int {
		self.0.getTextWidth.expect("getTextWidth")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getFontHeight`]
	#[doc(alias = "sys::ffi::playdate_graphics::getFontHeight")]
	#[inline(always)]
	fn get_font_height(&self) -> unsafe extern "C" fn(font: *mut LCDFont) -> u8 {
		self.0.getFontHeight.expect("getFontHeight")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setFont`]
	#[doc(alias = "sys::ffi::playdate_graphics::setFont")]
	#[inline(always)]
	fn set_font(&self) -> unsafe extern "C" fn(font: *mut LCDFont) { self.0.setFont.expect("setFont") }

	/// Equivalent to [`sys::ffi::playdate_graphics::setTextTracking`]
	#[doc(alias = "sys::ffi::playdate_graphics::setTextTracking")]
	#[inline(always)]
	fn set_text_tracking(&self) -> unsafe extern "C" fn(tracking: c_int) {
		self.0.setTextTracking.expect("setTextTracking")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getGlyphKerning`]
	#[doc(alias = "sys::ffi::playdate_graphics::getGlyphKerning")]
	#[inline(always)]
	fn get_glyph_kerning(
		&self)
		-> unsafe extern "C" fn(glyph: *mut LCDFontGlyph, glyphcode: u32, nextcode: u32) -> c_int {
		self.0.getGlyphKerning.expect("getGlyphKerning")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::loadFont`]
	#[doc(alias = "sys::ffi::playdate_graphics::loadFont")]
	#[inline(always)]
	fn load_font(&self) -> unsafe extern "C" fn(path: *const c_char, outErr: *mut *const c_char) -> *mut LCDFont {
		self.0.loadFont.expect("loadFont")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getFontPage`]
	#[doc(alias = "sys::ffi::playdate_graphics::getFontPage")]
	#[inline(always)]
	fn get_font_page(&self) -> unsafe extern "C" fn(font: *mut LCDFont, c: u32) -> *mut LCDFontPage {
		self.0.getFontPage.expect("getFontPage")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getPageGlyph`]
	#[doc(alias = "sys::ffi::playdate_graphics::getPageGlyph")]
	#[inline(always)]
	fn get_page_glyph(
		&self)
		-> unsafe extern "C" fn(page: *mut LCDFontPage,
		                        c: u32,
		                        bitmap: *mut *mut LCDBitmap,
		                        advance: *mut c_int) -> *mut LCDFontGlyph {
		self.0.getPageGlyph.expect("getPageGlyph")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::makeFontFromData`]
	#[doc(alias = "sys::ffi::playdate_graphics::makeFontFromData")]
	#[inline(always)]
	fn make_font_from_data(&self) -> unsafe extern "C" fn(data: *mut LCDFontData, wide: c_int) -> *mut LCDFont {
		self.0.makeFontFromData.expect("makeFontFromData")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setTextLeading`]
	#[doc(alias = "sys::ffi::playdate_graphics::setTextLeading")]
	#[inline(always)]
	fn set_text_leading(&self) -> unsafe extern "C" fn(lineHeightAdustment: c_int) {
		self.0.setTextLeading.expect("setTextLeading")
	}
}


impl crate::bitmap::table::api::Api for Cache {
	#[inline(always)]
	fn new_bitmap_table(
		&self)
		-> unsafe extern "C" fn(count: c_int, width: c_int, height: c_int) -> *mut LCDBitmapTable {
		self.0.newBitmapTable.expect("newBitmapTable")
	}

	#[inline(always)]
	fn free_bitmap_table(&self) -> unsafe extern "C" fn(table: *mut LCDBitmapTable) {
		self.0.freeBitmapTable.expect("freeBitmapTable")
	}

	#[inline(always)]
	fn load_bitmap_table(
		&self)
		-> unsafe extern "C" fn(path: *const c_char, out_err: *mut *const c_char) -> *mut LCDBitmapTable {
		self.0.loadBitmapTable.expect("loadBitmapTable")
	}

	#[inline(always)]
	fn load_into_bitmap_table(
		&self)
		-> unsafe extern "C" fn(path: *const c_char, table: *mut LCDBitmapTable, out_err: *mut *const c_char) {
		self.0.loadIntoBitmapTable.expect("loadIntoBitmapTable")
	}

	#[inline(always)]
	fn get_table_bitmap(&self) -> unsafe extern "C" fn(table: *mut LCDBitmapTable, idx: c_int) -> *mut LCDBitmap {
		self.0.getTableBitmap.expect("getTableBitmap")
	}
}


impl crate::bitmap::api::Api for Cache {
	#[inline(always)]
	fn new_bitmap(&self)
	              -> unsafe extern "C" fn(width: c_int, height: c_int, bgcolor: LCDColor) -> *mut LCDBitmap {
		self.0.newBitmap.expect("newBitmap")
	}

	#[inline(always)]
	fn free_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) {
		self.0.freeBitmap.expect("freeBitmap")
	}

	#[inline(always)]
	fn load_bitmap(&self)
	               -> unsafe extern "C" fn(path: *const c_char, outerr: *mut *const c_char) -> *mut LCDBitmap {
		self.0.loadBitmap.expect("loadBitmap")
	}

	#[inline(always)]
	fn copy_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
		self.0.copyBitmap.expect("copyBitmap")
	}

	#[inline(always)]
	fn load_into_bitmap(
		&self)
		-> unsafe extern "C" fn(path: *const c_char, bitmap: *mut LCDBitmap, out_err: *mut *const c_char) {
		self.0.loadIntoBitmap.expect("loadIntoBitmap")
	}

	#[inline(always)]
	fn get_bitmap_data(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        width: *mut c_int,
		                        height: *mut c_int,
		                        row_bytes: *mut c_int,
		                        mask: *mut *mut u8,
		                        data: *mut *mut u8) {
		self.0.getBitmapData.expect("getBitmapData")
	}

	#[inline(always)]
	fn clear_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, bgcolor: LCDColor) {
		self.0.clearBitmap.expect("clearBitmap")
	}

	#[inline(always)]
	fn rotated_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        rotation: c_float,
		                        x_scale: c_float,
		                        y_scale: c_float,
		                        allocedSize: *mut c_int) -> *mut LCDBitmap {
		self.0.rotatedBitmap.expect("rotatedBitmap")
	}

	#[inline(always)]
	fn set_bitmap_mask(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, mask: *mut LCDBitmap) -> c_int {
		self.0.setBitmapMask.expect("setBitmapMask")
	}

	#[inline(always)]
	fn get_bitmap_mask(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
		self.0.getBitmapMask.expect("getBitmapMask")
	}

	#[inline(always)]
	fn draw_bitmap(&self)
	               -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, x: c_int, y: c_int, flip: LCDBitmapFlip) {
		self.0.drawBitmap.expect("drawBitmap")
	}

	#[inline(always)]
	fn tile_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        x: c_int,
		                        y: c_int,
		                        width: c_int,
		                        height: c_int,
		                        flip: LCDBitmapFlip) {
		self.0.tileBitmap.expect("tileBitmap")
	}

	#[inline(always)]
	fn draw_rotated_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        x: c_int,
		                        y: c_int,
		                        rotation: c_float,
		                        center_x: c_float,
		                        center_y: c_float,
		                        x_scale: c_float,
		                        y_scale: c_float) {
		self.0.drawRotatedBitmap.expect("drawRotatedBitmap")
	}

	#[inline(always)]
	fn draw_scaled_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
		self.0.drawScaledBitmap.expect("drawScaledBitmap")
	}

	#[inline(always)]
	fn check_mask_collision(
		&self)
		-> unsafe extern "C" fn(bitmap1: *mut LCDBitmap,
		                        x1: c_int,
		                        y1: c_int,
		                        flip1: LCDBitmapFlip,
		                        bitmap2: *mut LCDBitmap,
		                        x2: c_int,
		                        y2: c_int,
		                        flip2: LCDBitmapFlip,
		                        rect: LCDRect) -> c_int {
		self.0.checkMaskCollision.expect("checkMaskCollision")
	}

	#[inline(always)]
	fn set_color_to_pattern(
		&self)
		-> unsafe extern "C" fn(color: *mut LCDColor,
		                        bitmap: *mut LCDBitmap,
		                        x: core::ffi::c_int,
		                        y: core::ffi::c_int) {
		self.0.setColorToPattern.expect("setColorToPattern")
	}
}


impl Api for Cache {
	/// Equivalent to [`sys::ffi::playdate_graphics::video`]
	#[doc(alias = "sys::ffi::playdate_graphics::video")]
	#[inline(always)]
	fn video<VApi: crate::video::api::Api + From<*const playdate_video>>(&self) -> VApi {
		VApi::from(self.0.video)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::clear`]
	#[doc(alias = "sys::ffi::playdate_graphics::clear")]
	#[inline(always)]
	fn clear(&self) -> unsafe extern "C" fn(color: LCDColor) { self.0.clear.expect("clear") }

	/// Equivalent to [`sys::ffi::playdate_graphics::setBackgroundColor`]
	#[doc(alias = "sys::ffi::playdate_graphics::setBackgroundColor")]
	#[inline(always)]
	fn set_background_color(&self) -> unsafe extern "C" fn(color: LCDSolidColor) {
		self.0.setBackgroundColor.expect("setBackgroundColor")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setStencil`]
	#[doc(alias = "sys::ffi::playdate_graphics::setStencil")]
	#[inline(always)]
	fn set_stencil(&self) -> unsafe extern "C" fn(stencil: *mut LCDBitmap) {
		self.0.setStencil.expect("setStencil")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setDrawMode`]
	#[doc(alias = "sys::ffi::playdate_graphics::setDrawMode")]
	#[inline(always)]
	fn set_draw_mode(&self) -> unsafe extern "C" fn(mode: LCDBitmapDrawMode) {
		self.0.setDrawMode.expect("setDrawMode")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setDrawOffset`]
	#[doc(alias = "sys::ffi::playdate_graphics::setDrawOffset")]
	#[inline(always)]
	fn set_draw_offset(&self) -> unsafe extern "C" fn(dx: c_int, dy: c_int) {
		self.0.setDrawOffset.expect("setDrawOffset")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setClipRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::setClipRect")]
	#[inline(always)]
	fn set_clip_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int) {
		self.0.setClipRect.expect("setClipRect")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::clearClipRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::clearClipRect")]
	#[inline(always)]
	fn clear_clip_rect(&self) -> unsafe extern "C" fn() { self.0.clearClipRect.expect("clearClipRect") }

	/// Equivalent to [`sys::ffi::playdate_graphics::setLineCapStyle`]
	#[doc(alias = "sys::ffi::playdate_graphics::setLineCapStyle")]
	#[inline(always)]
	fn set_line_cap_style(&self) -> unsafe extern "C" fn(endCapStyle: LCDLineCapStyle) {
		self.0.setLineCapStyle.expect("setLineCapStyle")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::pushContext`]
	#[doc(alias = "sys::ffi::playdate_graphics::pushContext")]
	#[inline(always)]
	fn push_context(&self) -> unsafe extern "C" fn(target: *mut LCDBitmap) {
		self.0.pushContext.expect("pushContext")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::popContext`]
	#[doc(alias = "sys::ffi::playdate_graphics::popContext")]
	#[inline(always)]
	fn pop_context(&self) -> unsafe extern "C" fn() { self.0.popContext.expect("popContext") }

	/// Equivalent to [`sys::ffi::playdate_graphics::drawLine`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawLine")]
	#[inline(always)]
	fn draw_line(
		&self)
		-> unsafe extern "C" fn(x1: c_int, y1: c_int, x2: c_int, y2: c_int, width: c_int, color: LCDColor) {
		self.0.drawLine.expect("drawLine")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillTriangle`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillTriangle")]
	#[inline(always)]
	fn fill_triangle(
		&self)
		-> unsafe extern "C" fn(x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int, color: LCDColor) {
		self.0.fillTriangle.expect("fillTriangle")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawRect")]
	#[inline(always)]
	fn draw_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		self.0.drawRect.expect("drawRect")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillRect")]
	#[inline(always)]
	fn fill_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		self.0.fillRect.expect("fillRect")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawEllipse`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawEllipse")]
	#[inline(always)]
	fn draw_ellipse(
		&self)
		-> unsafe extern "C" fn(x: c_int,
		                        y: c_int,
		                        width: c_int,
		                        height: c_int,
		                        lineWidth: c_int,
		                        startAngle: c_float,
		                        endAngle: c_float,
		                        color: LCDColor) {
		self.0.drawEllipse.expect("drawEllipse")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillEllipse`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillEllipse")]
	#[inline(always)]
	fn fill_ellipse(
		&self)
		-> unsafe extern "C" fn(x: c_int,
		                        y: c_int,
		                        width: c_int,
		                        height: c_int,
		                        startAngle: c_float,
		                        endAngle: c_float,
		                        color: LCDColor) {
		self.0.fillEllipse.expect("fillEllipse")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getFrame`]
	#[doc(alias = "sys::ffi::playdate_graphics::getFrame")]
	#[inline(always)]
	fn get_frame(&self) -> unsafe extern "C" fn() -> *mut u8 { self.0.getFrame.expect("getFrame") }

	/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayFrame`]
	#[doc(alias = "sys::ffi::playdate_graphics::getDisplayFrame")]
	#[inline(always)]
	fn get_display_frame(&self) -> unsafe extern "C" fn() -> *mut u8 {
		self.0.getDisplayFrame.expect("getDisplayFrame")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getDebugBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::getDebugBitmap")]
	#[inline(always)]
	fn get_debug_bitmap(&self) -> Option<unsafe extern "C" fn() -> *mut LCDBitmap> {
		sys::api!(graphics).getDebugBitmap
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::copyFrameBufferBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::copyFrameBufferBitmap")]
	#[inline(always)]
	fn copy_frame_buffer_bitmap(&self) -> unsafe extern "C" fn() -> *mut LCDBitmap {
		self.0.copyFrameBufferBitmap.expect("copyFrameBufferBitmap")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::markUpdatedRows`]
	#[doc(alias = "sys::ffi::playdate_graphics::markUpdatedRows")]
	#[inline(always)]
	fn mark_updated_rows(&self) -> unsafe extern "C" fn(start: c_int, end: c_int) {
		self.0.markUpdatedRows.expect("markUpdatedRows")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::display`]
	#[doc(alias = "sys::ffi::playdate_graphics::display")]
	#[inline(always)]
	fn display(&self) -> unsafe extern "C" fn() { self.0.display.expect("display") }

	/// Equivalent to [`sys::ffi::playdate_graphics::setScreenClipRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::setScreenClipRect")]
	#[inline(always)]
	fn set_screen_clip_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int) {
		self.0.setScreenClipRect.expect("setScreenClipRect")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillPolygon`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillPolygon")]
	#[inline(always)]
	fn fill_polygon(
		&self)
		-> unsafe extern "C" fn(nPoints: c_int, coords: *mut c_int, color: LCDColor, fillrule: LCDPolygonFillRule) {
		self.0.fillPolygon.expect("fillPolygon")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayBufferBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::getDisplayBufferBitmap")]
	#[inline(always)]
	fn get_display_buffer_bitmap(&self) -> unsafe extern "C" fn() -> *mut LCDBitmap {
		self.0.getDisplayBufferBitmap.expect("getDisplayBufferBitmap")
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setStencilImage`]
	#[doc(alias = "sys::ffi::playdate_graphics::setStencilImage")]
	#[inline(always)]
	fn set_stencil_image(&self) -> unsafe extern "C" fn(stencil: *mut LCDBitmap, tile: c_int) {
		self.0.setStencilImage.expect("setStencilImage")
	}
}


pub trait Api: crate::bitmap::api::Api + crate::bitmap::table::api::Api + crate::text::api::Api {
	/// Equivalent to [`sys::ffi::playdate_graphics::video`]
	#[doc(alias = "sys::ffi::playdate_graphics::video")]
	fn video<VApi>(&self) -> VApi
		where VApi: From<*const playdate_video> + crate::video::api::Api;

	/// Equivalent to [`sys::ffi::playdate_graphics::clear`]
	#[doc(alias = "sys::ffi::playdate_graphics::clear")]
	#[inline(always)]
	fn clear(&self) -> unsafe extern "C" fn(color: LCDColor) { *sys::api!(graphics.clear) }

	/// Equivalent to [`sys::ffi::playdate_graphics::setBackgroundColor`]
	#[doc(alias = "sys::ffi::playdate_graphics::setBackgroundColor")]
	#[inline(always)]
	fn set_background_color(&self) -> unsafe extern "C" fn(color: LCDSolidColor) {
		*sys::api!(graphics.setBackgroundColor)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setStencil`]
	#[doc(alias = "sys::ffi::playdate_graphics::setStencil")]
	#[inline(always)]
	fn set_stencil(&self) -> unsafe extern "C" fn(stencil: *mut LCDBitmap) { *sys::api!(graphics.setStencil) }

	/// Equivalent to [`sys::ffi::playdate_graphics::setDrawMode`]
	#[doc(alias = "sys::ffi::playdate_graphics::setDrawMode")]
	#[inline(always)]
	fn set_draw_mode(&self) -> unsafe extern "C" fn(mode: LCDBitmapDrawMode) { *sys::api!(graphics.setDrawMode) }

	/// Equivalent to [`sys::ffi::playdate_graphics::setDrawOffset`]
	#[doc(alias = "sys::ffi::playdate_graphics::setDrawOffset")]
	#[inline(always)]
	fn set_draw_offset(&self) -> unsafe extern "C" fn(dx: c_int, dy: c_int) { *sys::api!(graphics.setDrawOffset) }

	/// Equivalent to [`sys::ffi::playdate_graphics::setClipRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::setClipRect")]
	#[inline(always)]
	fn set_clip_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int) {
		*sys::api!(graphics.setClipRect)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::clearClipRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::clearClipRect")]
	#[inline(always)]
	fn clear_clip_rect(&self) -> unsafe extern "C" fn() { *sys::api!(graphics.clearClipRect) }

	/// Equivalent to [`sys::ffi::playdate_graphics::setLineCapStyle`]
	#[doc(alias = "sys::ffi::playdate_graphics::setLineCapStyle")]
	#[inline(always)]
	fn set_line_cap_style(&self) -> unsafe extern "C" fn(endCapStyle: LCDLineCapStyle) {
		*sys::api!(graphics.setLineCapStyle)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::pushContext`]
	#[doc(alias = "sys::ffi::playdate_graphics::pushContext")]
	#[inline(always)]
	fn push_context(&self) -> unsafe extern "C" fn(target: *mut LCDBitmap) { *sys::api!(graphics.pushContext) }

	/// Equivalent to [`sys::ffi::playdate_graphics::popContext`]
	#[doc(alias = "sys::ffi::playdate_graphics::popContext")]
	#[inline(always)]
	fn pop_context(&self) -> unsafe extern "C" fn() { *sys::api!(graphics.popContext) }

	/// Equivalent to [`sys::ffi::playdate_graphics::drawLine`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawLine")]
	#[inline(always)]
	fn draw_line(
		&self)
		-> unsafe extern "C" fn(x1: c_int, y1: c_int, x2: c_int, y2: c_int, width: c_int, color: LCDColor) {
		*sys::api!(graphics.drawLine)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillTriangle`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillTriangle")]
	#[inline(always)]
	fn fill_triangle(
		&self)
		-> unsafe extern "C" fn(x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int, color: LCDColor) {
		*sys::api!(graphics.fillTriangle)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawRect")]
	#[inline(always)]
	fn draw_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		*sys::api!(graphics.drawRect)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillRect")]
	#[inline(always)]
	fn fill_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int, color: LCDColor) {
		*sys::api!(graphics.fillRect)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawEllipse`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawEllipse")]
	#[inline(always)]
	fn draw_ellipse(
		&self)
		-> unsafe extern "C" fn(x: c_int,
		                        y: c_int,
		                        width: c_int,
		                        height: c_int,
		                        lineWidth: c_int,
		                        startAngle: c_float,
		                        endAngle: c_float,
		                        color: LCDColor) {
		*sys::api!(graphics.drawEllipse)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillEllipse`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillEllipse")]
	#[inline(always)]
	fn fill_ellipse(
		&self)
		-> unsafe extern "C" fn(x: c_int,
		                        y: c_int,
		                        width: c_int,
		                        height: c_int,
		                        startAngle: c_float,
		                        endAngle: c_float,
		                        color: LCDColor) {
		*sys::api!(graphics.fillEllipse)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getFrame`]
	#[doc(alias = "sys::ffi::playdate_graphics::getFrame")]
	#[inline(always)]
	fn get_frame(&self) -> unsafe extern "C" fn() -> *mut u8 { *sys::api!(graphics.getFrame) }

	/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayFrame`]
	#[doc(alias = "sys::ffi::playdate_graphics::getDisplayFrame")]
	#[inline(always)]
	fn get_display_frame(&self) -> unsafe extern "C" fn() -> *mut u8 { *sys::api!(graphics.getDisplayFrame) }

	/// Equivalent to [`sys::ffi::playdate_graphics::getDebugBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::getDebugBitmap")]
	#[inline(always)]
	fn get_debug_bitmap(&self) -> Option<unsafe extern "C" fn() -> *mut LCDBitmap> {
		sys::api!(graphics).getDebugBitmap
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::copyFrameBufferBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::copyFrameBufferBitmap")]
	#[inline(always)]
	fn copy_frame_buffer_bitmap(&self) -> unsafe extern "C" fn() -> *mut LCDBitmap {
		*sys::api!(graphics.copyFrameBufferBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::markUpdatedRows`]
	#[doc(alias = "sys::ffi::playdate_graphics::markUpdatedRows")]
	#[inline(always)]
	fn mark_updated_rows(&self) -> unsafe extern "C" fn(start: c_int, end: c_int) {
		*sys::api!(graphics.markUpdatedRows)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::display`]
	#[doc(alias = "sys::ffi::playdate_graphics::display")]
	#[inline(always)]
	fn display(&self) -> unsafe extern "C" fn() { *sys::api!(graphics.display) }

	/// Equivalent to [`sys::ffi::playdate_graphics::setScreenClipRect`]
	#[doc(alias = "sys::ffi::playdate_graphics::setScreenClipRect")]
	#[inline(always)]
	fn set_screen_clip_rect(&self) -> unsafe extern "C" fn(x: c_int, y: c_int, width: c_int, height: c_int) {
		*sys::api!(graphics.setScreenClipRect)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::fillPolygon`]
	#[doc(alias = "sys::ffi::playdate_graphics::fillPolygon")]
	#[inline(always)]
	fn fill_polygon(
		&self)
		-> unsafe extern "C" fn(nPoints: c_int, coords: *mut c_int, color: LCDColor, fillrule: LCDPolygonFillRule) {
		*sys::api!(graphics.fillPolygon)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayBufferBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::getDisplayBufferBitmap")]
	#[inline(always)]
	fn get_display_buffer_bitmap(&self) -> unsafe extern "C" fn() -> *mut LCDBitmap {
		*sys::api!(graphics.getDisplayBufferBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setStencilImage`]
	#[doc(alias = "sys::ffi::playdate_graphics::setStencilImage")]
	#[inline(always)]
	fn set_stencil_image(&self) -> unsafe extern "C" fn(stencil: *mut LCDBitmap, tile: c_int) {
		*sys::api!(graphics.setStencilImage)
	}
}
