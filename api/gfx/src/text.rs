// TODO: text/font
// loadFont
// getFontPage
// getPageGlyph
// getGlyphKerning
// getFontHeight
// setTextLeading
// setTextTracking
// setLineCapStyle
// setFont

use core::ffi::c_int;

use alloc::ffi::NulError;
use sys::ffi::{CString, CStr, LCDFont};
use sys::ffi::PDStringEncoding;


const UTF: PDStringEncoding = PDStringEncoding::kUTF8Encoding;


/// Equivalent to [`sys::ffi::playdate_graphics::drawText`].
pub fn draw_text<S: AsRef<str>>(text: S, x: c_int, y: c_int) -> Result<c_int, NulError> {
	let s = CString::new(text.as_ref())?;
	let f = *sys::api!(graphics.drawText);
	let res = unsafe { f(s.as_ptr().cast(), text.as_ref().len(), UTF, x, y) };
	Ok(res)
}

/// Same as [`draw_text`] but takes a [`sys::ffi::CStr`],
/// but little bit more efficient.
///
/// Equivalent to [`sys::ffi::playdate_graphics::drawText`].
pub fn draw_text_cstr(text: &CStr, x: c_int, y: c_int) -> c_int {
	let f = *sys::api!(graphics.drawText);
	let len = text.to_bytes().len();
	unsafe { f(text.as_ptr().cast(), len, UTF, x, y) }
}


/// Equivalent to [`sys::ffi::playdate_graphics::getTextWidth`].
pub fn get_text_width<S: AsRef<str>>(text: S,
                                     font: Option<&LCDFont>,
                                     tracking: c_int)
                                     -> Result<c_int, NulError> {
	let s = CString::new(text.as_ref())?;
	let f = *sys::api!(graphics.getTextWidth);
	let font = font.map(|font| font as *const LCDFont as *mut LCDFont)
	               .unwrap_or(core::ptr::null_mut());
	let res = unsafe { f(font, s.as_ptr().cast(), text.as_ref().len(), UTF, tracking) };
	Ok(res)
}

/// Same as [`get_text_width`] but takes a [`sys::ffi::CStr`],
/// but little bit more efficient.
///
/// Equivalent to [`sys::ffi::playdate_graphics::getTextWidth`].
pub fn get_text_width_cstr(text: &CStr, font: Option<&LCDFont>, tracking: c_int) -> c_int {
	let f = *sys::api!(graphics.getTextWidth);
	let len = text.to_bytes().len();
	let font = font.map(|font| font as *const LCDFont as *mut LCDFont)
	               .unwrap_or(core::ptr::null_mut());
	unsafe { f(font, text.as_ptr().cast(), len, UTF, tracking) }
}
