//! Playdate text API

use core::ffi::{c_int, c_char};

use alloc::ffi::NulError;
use alloc::boxed::Box;
use fs::Path;
use sys::ffi::{CString, CStr, LCDFont, LCDFontGlyph, LCDFontPage, LCDBitmap};
use sys::traits::AsRaw;

pub use sys::ffi::PDStringEncoding as StringEncoding;
pub use sys::ffi::PDTextWrappingMode as TextWrappingMode;
pub use sys::ffi::PDTextAlignment as TextAlignment;

use crate::Graphics;
use crate::bitmap::BitmapRef;
use crate::error::{Error, ApiError};

#[gen_api_shorthands::gen_shorthands]
impl<Api: crate::api::Api> Graphics<Api> {
	/// Draws the given `text` using the provided coords `x`, `y`.
	///
	/// Encoding is always `StringEncoding::UTF8`.
	/// If another encoding is desired, use [`draw_text_cstr`] instead.
	///
	/// If no `font` has been set with [`set_font`],
	/// the default system font `Asheville Sans 14 Light` is used.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawText`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawText")]
	pub fn draw_text<S: AsRef<str>>(&self, text: S, x: c_int, y: c_int) -> Result<c_int, NulError> {
		let s = CString::new(text.as_ref())?;
		let f = self.0.draw_text();
		let res = unsafe { f(s.as_ptr().cast(), text.as_ref().len(), StringEncoding::UTF8, x, y) };
		Ok(res)
	}

	/// Draws the given `text` using the provided options.
	///
	/// If no `font` has been set with [`set_font`],
	/// the default system font `Asheville Sans 14 Light` is used.
	///
	/// Same as [`draw_text`] but takes a [`sys::ffi::CStr`],
	/// but little bit more efficient.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawText`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawText")]
	pub fn draw_text_cstr(&self, text: &CStr, encoding: StringEncoding, x: c_int, y: c_int) -> c_int {
		let f = self.0.draw_text();
		let len = text.to_bytes().len();
		unsafe { f(text.as_ptr().cast(), len, encoding, x, y) }
	}

	/// Draws the `text` in the given rectangle using the provided options.
	///
	/// If no `font` has been set with [`set_font`],
	/// the default system font `Asheville Sans 14 Light`` is used.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawTextInRect`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawTextInRect")]
	pub fn draw_text_in_rect<S: AsRef<str>>(&self,
	                                        text: S,
	                                        x: c_int,
	                                        y: c_int,
	                                        width: c_int,
	                                        height: c_int,
	                                        wrap: TextWrappingMode,
	                                        align: TextAlignment) -> Result<(), NulError> {
		let s = CString::new(text.as_ref())?;
		let f = self.0.draw_text_in_rect();
		let res = unsafe { f(s.as_ptr().cast(), text.as_ref().len(), StringEncoding::UTF8, x, y, width, height, wrap, align) };
		Ok(res)
	}

	/// Returns the width of the given `text` in the given `font`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getTextWidth`].
	#[doc(alias = "sys::ffi::playdate_graphics::getTextWidth")]
	pub fn get_text_width<S: AsRef<str>>(&self,
	                                     text: S,
	                                     font: Option<&Font>,
	                                     tracking: c_int)
	                                     -> Result<c_int, NulError> {
		let s = CString::new(text.as_ref())?;
		let f = self.0.get_text_width();
		let font = font.map(|font| unsafe { font.as_raw() })
		               .unwrap_or(core::ptr::null_mut());
		let res = unsafe {
			f(
			  font,
			  s.as_ptr().cast(),
			  text.as_ref().len(),
			  StringEncoding::UTF8,
			  tracking,
			)
		};
		Ok(res)
	}

	/// Returns the width of the given `text` in the given `font`.
	///
	/// Same as [`get_text_width`] but takes a [`sys::ffi::CStr`],
	/// but little bit more efficient.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getTextWidth`].
	#[doc(alias = "sys::ffi::playdate_graphics::getTextWidth")]
	pub fn get_text_width_cstr(&self,
	                           text: &CStr,
	                           encoding: StringEncoding,
	                           font: Option<&Font>,
	                           tracking: c_int)
	                           -> c_int {
		let f = self.0.get_text_width();
		let len = text.to_bytes().len();
		let font = font.map(|font| unsafe { font.as_raw() })
		               .unwrap_or(core::ptr::null_mut());
		unsafe { f(font, text.as_ptr().cast(), len, encoding, tracking) }
	}


	/// Returns the height of the given `font`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getFontHeight`].
	#[doc(alias = "sys::ffi::playdate_graphics::getFontHeight")]
	pub fn get_font_height(&self, font: &Font) -> u8 {
		let f = self.0.get_font_height();
		unsafe { f(font.as_raw()) }
	}

	/// Sets the `font` to use in subsequent [`draw_text`] calls.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setFont`].
	#[doc(alias = "sys::ffi::playdate_graphics::setFont")]
	pub fn set_font(&self, font: &Font) {
		let f = self.0.set_font();
		unsafe { f(font.as_raw()) }
	}

	/// Returns the kerning adjustment between characters `glyph_code` and `next_code` as specified by the font
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getGlyphKerning`].
	#[doc(alias = "sys::ffi::playdate_graphics::getGlyphKerning")]
	pub fn get_glyph_kerning(&self, glyph: &Glyph, glyph_code: u32, next_code: u32) -> c_int {
		let f = self.0.get_glyph_kerning();
		unsafe { f(glyph.as_raw(), glyph_code, next_code) }
	}

	/// Returns an [`Glyph`] object for character `c` in [`FontPage`] page,
	///
	/// To also get the glyph’s bitmap and `advance` value
	/// use [`get_page_glyph_with_bitmap`] instead.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getPageGlyph`].
	#[doc(alias = "sys::ffi::playdate_graphics::getPageGlyph")]
	pub fn get_page_glyph(&self, page: &FontPage, c: u32) -> Result<Glyph, Error> {
		let f = self.0.get_page_glyph();
		let ptr = unsafe { f(page.as_raw(), c, core::ptr::null_mut(), core::ptr::null_mut()) };

		if ptr.is_null() {
			Err(Error::Font)
		} else {
			Ok(Glyph(ptr))
		}
	}

	/// Returns an [`Glyph`] object for character `c` in [`FontPage`] page,
	/// and optionally returns the glyph’s bitmap and `advance` value.
	///
	/// If bitmap is not needed, use [`get_page_glyph`] instead.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getPageGlyph`].
	#[doc(alias = "sys::ffi::playdate_graphics::getPageGlyph")]
	pub fn get_page_glyph_with_bitmap<'p>(&self,
	                                      page: &'p FontPage,
	                                      c: u32,
	                                      advance: &mut c_int)
	                                      -> Result<(Glyph, BitmapRef<'p>), Error> {
		let bitmap = Box::new(core::ptr::null_mut() as *mut LCDBitmap);
		let out_bitmap = Box::into_raw(bitmap);

		let f = self.0.get_page_glyph();
		let ptr = unsafe { f(page.as_raw(), c, out_bitmap, advance) };

		if ptr.is_null() {
			Err(Error::Font)
		} else {
			let bitmap = unsafe { Box::from_raw(out_bitmap) };
			if bitmap.is_null() {
				Err(Error::Font)
			} else {
				Ok((Glyph(ptr), BitmapRef::from(*bitmap)))
			}
		}
	}


	/// Returns an [`FontPage`] object for the given character code `c`.
	///
	/// Each [`FontPage`] contains information for 256 characters;
	/// specifically, if `(c1 & ~0xff) == (c2 & ~0xff)`,
	/// then `c1` and `c2` belong to the same page and the same [`FontPage`]
	/// can be used to fetch the character data for both instead of searching for the page twice.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getFontPage`].
	#[doc(alias = "sys::ffi::playdate_graphics::getFontPage")]
	pub fn get_font_page(&self, font: &Font, c: u32) -> Result<FontPage, Error> {
		let f = self.0.get_font_page();
		let ptr = unsafe { f(font.as_raw(), c) };

		if ptr.is_null() {
			Err(Error::Font)
		} else {
			Ok(FontPage(ptr))
		}
	}


	/// Returns the [`Font`] object for the font file at `path`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::loadFont`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadFont")]
	pub fn load_font<P: AsRef<Path>>(&self, path: P) -> Result<Font, ApiError> {
		let mut err = Box::new(core::ptr::null() as *const c_char);
		let out_err = Box::into_raw(err);

		let path = CString::new(path.as_ref())?;

		let f = self.0.load_font();
		let ptr = unsafe { f(path.as_ptr() as *mut c_char, out_err as _) };

		if ptr.is_null() {
			err = unsafe { Box::from_raw(out_err) };
			if let Some(err) = fs::error::Error::from_ptr(*err) {
				Err(Error::Fs(err).into())
			} else {
				Err(Error::Alloc.into())
			}
		} else {
			Ok(Font(ptr))
		}
	}


	/// ⚠️ Caution: This function is not tested.
	///
	/// Returns an [`Font`] object wrapping the LCDFontData data
	/// comprising the contents (minus 16-byte header) of an uncompressed pft file.
	///
	/// The `wide` corresponds to the flag in the header indicating
	/// whether the font contains glyphs at codepoints above `U+1FFFF`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::makeFontFromData`].
	#[doc(alias = "sys::ffi::playdate_graphics::makeFontFromData")]
	pub fn make_font_from_bytes(&self, data: &[u8], wide: c_int) -> Result<Font, Error> {
		let f = self.0.make_font_from_data();
		let ptr = unsafe { f(data.as_ptr() as _, wide) };

		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Font(ptr))
		}
	}


	/// Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setTextLeading`].
	#[doc(alias = "sys::ffi::playdate_graphics::setTextLeading")]
	pub fn set_text_leading(&self, line_height_adjustment: c_int) {
		let f = self.0.set_text_leading();
		unsafe { f(line_height_adjustment) }
	}

	/// Sets the tracking to use when drawing text.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setTextTracking`].
	#[doc(alias = "sys::ffi::playdate_graphics::setTextTracking")]
	pub fn set_text_tracking(&self, tracking: c_int) {
		let f = self.0.set_text_tracking();
		unsafe { f(tracking) }
	}

	/// Sets the tracking to use when drawing text.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getTextTracking`].
	#[doc(alias = "sys::ffi::playdate_graphics::getTextTracking")]
	pub fn get_text_tracking(&self) -> c_int {
		let f = self.0.get_text_tracking();
		unsafe { f() }
	}
}


/// Playdate Font representation.
///
/// See [official docs][] for more information.
///
/// [official docs]: https://sdk.play.date/Inside%20Playdate.html#C-graphics.font
pub struct Font(*mut LCDFont);

impl AsRaw for Font {
	type Type = LCDFont;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0 }
}

/// Playdate Glyph representation.
pub struct Glyph(*mut LCDFontGlyph);

impl AsRaw for Glyph {
	type Type = LCDFontGlyph;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0 }
}

/// Playdate FontPage representation.
pub struct FontPage(*mut LCDFontPage);

impl AsRaw for FontPage {
	type Type = LCDFontPage;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0 }
}


pub trait StringEncodingExt {
	#![allow(non_upper_case_globals)]
	const ASCII: StringEncoding = StringEncoding::kASCIIEncoding;
	const UTF8: StringEncoding = StringEncoding::kUTF8Encoding;
	const LE16Bit: StringEncoding = StringEncoding::k16BitLEEncoding;
}
impl StringEncodingExt for StringEncoding {}

pub trait TextWrappingModeExt {
	#![allow(non_upper_case_globals)]
	const Clip: TextWrappingMode = TextWrappingMode::kWrapClip;
	const Character: TextWrappingMode = TextWrappingMode::kWrapCharacter;
	const Word: TextWrappingMode = TextWrappingMode::kWrapWord;
}
impl TextWrappingModeExt for TextWrappingMode {}

pub trait TextAlignmentExt {
	#![allow(non_upper_case_globals)]
	const Left: TextAlignment = TextAlignment::kAlignTextLeft;
	const Center: TextAlignment = TextAlignment::kAlignTextCenter;
	const Right: TextAlignment = TextAlignment::kAlignTextRight;
}
impl TextAlignmentExt for TextAlignment {}


pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_int;
	use core::ffi::c_void;

	use sys::ffi::LCDBitmap;
	use sys::ffi::LCDFont;
	use sys::ffi::LCDFontData;
	use sys::ffi::LCDFontGlyph;
	use sys::ffi::LCDFontPage;
	use sys::ffi::PDStringEncoding;
  use sys::ffi::PDTextWrappingMode;
  use sys::ffi::PDTextAlignment;


	/// Default graphics text api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	pub type Default = crate::api::Default;

	/// Cached graphics text api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	pub type Cache = crate::api::Cache;


	/// End-point with methods about ops with text.
	pub trait Api {
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
			*sys::api!(graphics.drawText)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::drawTextInRect`]
		#[doc(alias = "sys::ffi::playdate_graphics::drawTextInRect")]
		#[inline(always)]
		fn draw_text_in_rect(&self)
			-> unsafe extern "C" fn(text: *const c_void,
			                        len: usize,
			                        encoding: PDStringEncoding,
			                        x: c_int,
			                        y: c_int,
			                        width: c_int,
			                        height: c_int,
			                        wrap: PDTextWrappingMode,
			                        align: PDTextAlignment) {
			*sys::api!(graphics.drawTextInRect)
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
			*sys::api!(graphics.getTextWidth)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::getFontHeight`]
		#[doc(alias = "sys::ffi::playdate_graphics::getFontHeight")]
		#[inline(always)]
		fn get_font_height(&self) -> unsafe extern "C" fn(font: *mut LCDFont) -> u8 {
			*sys::api!(graphics.getFontHeight)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::setFont`]
		#[doc(alias = "sys::ffi::playdate_graphics::setFont")]
		#[inline(always)]
		fn set_font(&self) -> unsafe extern "C" fn(font: *mut LCDFont) { *sys::api!(graphics.setFont) }

		/// Equivalent to [`sys::ffi::playdate_graphics::setTextTracking`]
		#[doc(alias = "sys::ffi::playdate_graphics::setTextTracking")]
		#[inline(always)]
		fn set_text_tracking(&self) -> unsafe extern "C" fn(tracking: c_int) {
			*sys::api!(graphics.setTextTracking)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::getTextTracking`]
		#[doc(alias = "sys::ffi::playdate_graphics::getTextTracking")]
		#[inline(always)]
		fn get_text_tracking(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(graphics.getTextTracking) }

		/// Equivalent to [`sys::ffi::playdate_graphics::getGlyphKerning`]
		#[doc(alias = "sys::ffi::playdate_graphics::getGlyphKerning")]
		#[inline(always)]
		fn get_glyph_kerning(
			&self)
			-> unsafe extern "C" fn(glyph: *mut LCDFontGlyph, glyphcode: u32, nextcode: u32) -> c_int {
			*sys::api!(graphics.getGlyphKerning)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::loadFont`]
		#[doc(alias = "sys::ffi::playdate_graphics::loadFont")]
		#[inline(always)]
		fn load_font(&self)
		             -> unsafe extern "C" fn(path: *const c_char, outErr: *mut *const c_char) -> *mut LCDFont {
			*sys::api!(graphics.loadFont)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::getFontPage`]
		#[doc(alias = "sys::ffi::playdate_graphics::getFontPage")]
		#[inline(always)]
		fn get_font_page(&self) -> unsafe extern "C" fn(font: *mut LCDFont, c: u32) -> *mut LCDFontPage {
			*sys::api!(graphics.getFontPage)
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
			*sys::api!(graphics.getPageGlyph)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::makeFontFromData`]
		#[doc(alias = "sys::ffi::playdate_graphics::makeFontFromData")]
		#[inline(always)]
		fn make_font_from_data(&self) -> unsafe extern "C" fn(data: *mut LCDFontData, wide: c_int) -> *mut LCDFont {
			*sys::api!(graphics.makeFontFromData)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::setTextLeading`]
		#[doc(alias = "sys::ffi::playdate_graphics::setTextLeading")]
		#[inline(always)]
		fn set_text_leading(&self) -> unsafe extern "C" fn(lineHeightAdjustment: c_int) {
			*sys::api!(graphics.setTextLeading)
		}
	}
}
