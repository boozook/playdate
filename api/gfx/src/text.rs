//! Playdate text API

use core::ffi::{c_int, c_char};
use core::fmt::Debug;
use core::ptr::{null_mut, NonNull};

use fs::path::Path;
use sys::ffi::Font as LCDFont;
use sys::ffi::FontPage;
use sys::ffi::FontGlyph;
use sys::ffi::Bitmap as SysBitmap;

pub use sys::ffi::StringEncoding;
pub use sys::ffi::TextWrappingMode;
pub use sys::ffi::TextAlignment;

use crate::{Api, Graphics};
use crate::bitmap::Borrowed;


// /// Draws the given `text` using the provided coords `x`, `y`.
// ///
// /// Encoding is always `StringEncoding::UTF8`.
// /// If another encoding is desired, use [`draw_text_cstr`] instead.
// ///
// /// If no `font` has been set with [`set_font`],
// /// the default system font `Asheville Sans 14 Light` is used.
// ///
// /// This function is shorthand for [`Graphics::draw_text`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::drawText`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::drawText")]
// #[inline(always)]
// pub fn draw_text<S: AsRef<str>>(text: S, x: c_int, y: c_int) -> Result<c_int, NulError> {
// 	Graphics::Default().draw_text(text, x, y)
// }

// /// Draws the given `text` using the provided options.
// ///
// /// If no `font` has been set with [`set_font`],
// /// the default system font `Asheville Sans 14 Light` is used.
// ///
// /// Same as [`draw_text`] but takes a [`sys::ffi::CStr`],
// /// but little bit more efficient.
// ///
// /// This function is shorthand for [`Graphics::draw_text_cstr`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::drawText`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::drawText")]
// #[inline(always)]
// pub fn draw_text_cstr(text: &CStr, encoding: StringEncoding, x: c_int, y: c_int) -> c_int {
// 	Graphics::Default().draw_text_cstr(text, encoding, x, y)
// }

// /// Draws the `text` in the given rectangle using the provided options.
// ///
// /// If no `font` has been set with [`set_font`],
// /// the default system font `Asheville Sans 14 Light`` is used.
// ///
// /// This function is shorthand for [`Graphics::draw_text_in_rect`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::drawTextInRect`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::drawText")]
// #[inline(always)]
// pub fn draw_text_in_rect<S: AsRef<str>>(text: S,
//                                         x: c_int,
//                                         y: c_int,
//                                         width: c_int,
//                                         height: c_int,
//                                         wrap: TextWrappingMode,
//                                         align: TextAlignment)
//                                         -> Result<(), NulError> {
// 	Graphics::Default().draw_text_in_rect(text, x, y, width, height, wrap, align)
// }

// /// Returns the width of the given `text` in the given `font`.
// ///
// /// This function is shorthand for [`Graphics::get_text_width`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::getTextWidth`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::getTextWidth")]
// #[inline(always)]
// pub fn get_text_width<S: AsRef<str>>(text: S, font: Option<&Font>, tracking: c_int) -> Result<c_int, NulError> {
// 	Graphics::Default().get_text_width(text, font, tracking)
// }

// /// Returns the width of the given `text` in the given `font`.
// ///
// /// Same as [`get_text_width`] but takes a [`sys::ffi::CStr`],
// /// but little bit more efficient.
// ///
// /// This function is shorthand for [`Graphics::get_text_width_cstr`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::getTextWidth`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::getTextWidth")]
// #[inline(always)]
// pub fn get_text_width_cstr(text: &CStr, encoding: StringEncoding, font: Option<&Font>, tracking: c_int) -> c_int {
// 	Graphics::Default().get_text_width_cstr(text, encoding, font, tracking)
// }


// /// Sets the `font` to use in subsequent [`draw_text`] calls.
// ///
// /// This function is shorthand for [`Graphics::set_font`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::setFont`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::setFont")]
// #[inline(always)]
// pub fn set_font(font: &Font) { Graphics::Default().set_font(font) }

// /// Returns the kerning adjustment between characters `glyph_code` and `next_code` as specified by the font
// ///
// /// This function is shorthand for [`Graphics::get_glyph_kerning`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::getGlyphKerning`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::getGlyphKerning")]
// #[inline(always)]
// pub fn get_glyph_kerning(glyph: &Glyph, glyph_code: u32, next_code: u32) -> c_int {
// 	Graphics::Default().get_glyph_kerning(glyph, glyph_code, next_code)
// }


// /// Returns an [`FontPage`] object for the given character code `c`.
// ///
// /// Each [`FontPage`] contains information for 256 characters;
// /// specifically, if `(c1 & ~0xff) == (c2 & ~0xff)`,
// /// then `c1` and `c2` belong to the same page and the same [`FontPage`]
// /// can be used to fetch the character data for both instead of searching for the page twice.
// ///
// /// This function is shorthand for [`Graphics::get_font_page`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::getFontPage`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::getFontPage")]
// #[inline(always)]
// pub fn get_font_page(font: &Font, c: u32) -> Result<FontPage, Error> {
// 	Graphics::Default().get_font_page(font, c)
// }


// /// Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.
// ///
// /// This function is shorthand for [`Graphics::set_text_leading`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::setTextLeading`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::setTextLeading")]
// #[inline(always)]
// pub fn set_text_leading(line_height_adjustment: c_int) {
// 	Graphics::Default().set_text_leading(line_height_adjustment)
// }

// /// Sets the tracking to use when drawing text.
// ///
// /// This function is shorthand for [`Graphics::set_text_tracking`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::setTextTracking`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::setTextTracking")]
// #[inline(always)]
// pub fn set_text_tracking(tracking: c_int) { Graphics::Default().set_text_tracking(tracking) }


// /// Gets the tracking used when drawing text.
// ///
// /// This function is shorthand for [`Graphics::set_text_tracking`],
// /// using default ZST end-point.
// ///
// /// Equivalent to [`sys::ffi::PlaydateGraphics::getTextTracking`].
// #[doc(alias = "sys::ffi::PlaydateGraphics::getTextTracking")]
// #[inline(always)]
// pub fn get_text_tracking() -> c_int { Graphics::Default().get_text_tracking() }


mod sealed {
	use core::ffi::c_char;
	use sys::ffi::CStr;
	use sys::ffi::StringEncoding;
	use super::AsRawStr;


	trait StrPtr {
		fn as_ptr(&self) -> *const c_char;
	}
	impl<'t, T: StrPtr> StrPtr for &'t T {
		#[inline(always)]
		fn as_ptr(&self) -> *const c_char { StrPtr::as_ptr(*self) }
	}
	impl StrPtr for &str {
		#[inline(always)]
		fn as_ptr(&self) -> *const c_char { str::as_ptr(self).cast() }
	}
	impl StrPtr for &CStr {
		#[inline(always)]
		fn as_ptr(&self) -> *const c_char { CStr::as_ptr(self) }
	}
	impl StrPtr for &[u8] {
		#[inline(always)]
		fn as_ptr(&self) -> *const c_char { <[u8]>::as_ptr(self).cast() }
	}
	impl StrPtr for &[u16] {
		#[inline(always)]
		fn as_ptr(&self) -> *const c_char { <[u16]>::as_ptr(self).cast() }
	}


	trait StrMaxLen {
		fn len_max(&self, enc: StringEncoding) -> usize;
	}
	impl<'t, T: StrMaxLen> StrMaxLen for &'t T {
		#[inline(always)]
		fn len_max(&self, enc: StringEncoding) -> usize { StrMaxLen::len_max(*self, enc) }
	}
	impl StrMaxLen for &str {
		#[inline]
		fn len_max(&self, enc: StringEncoding) -> usize {
			match enc {
				StringEncoding::ASCII => self.len(),
				StringEncoding::UTF8 => self.chars().count(),
				#[cfg(debug_assertions)]
				StringEncoding::UTF16 => unimplemented!("`str` is only valid for UTF8"),
				#[cfg(not(debug_assertions))]
				StringEncoding::UTF16 => 0,
			}
		}
	}
	impl StrMaxLen for &CStr {
		#[inline(always)]
		// For null-terminated c-str it's okay to use byte-count as length,
		// since the parser stops at the NUL terminator it’s safe.
		fn len_max(&self, _: StringEncoding) -> usize { self.to_bytes().len() }
	}
	impl StrMaxLen for &[u8] {
		#[inline(always)]
		fn len_max(&self, enc: StringEncoding) -> usize {
			match enc {
				StringEncoding::ASCII => self.len(),
				StringEncoding::UTF8 => self.len(),
				StringEncoding::UTF16 => self.len() / 2,
			}
		}
	}
	impl StrMaxLen for &[u16] {
		#[inline(always)]
		fn len_max(&self, enc: StringEncoding) -> usize {
			match enc {
				StringEncoding::ASCII => self.len() * 2,
				StringEncoding::UTF8 => self.len() * 2,
				StringEncoding::UTF16 => self.len(),
			}
		}
	}


	// blanked impl
	impl<T> AsRawStr for T where T: StrMaxLen + StrPtr + core::fmt::Debug {
		#[inline(always)]
		fn count(&self, enc: StringEncoding) -> usize { StrMaxLen::len_max(self, enc) }

		#[inline(always)]
		fn as_ptr(&self) -> *const c_char { StrPtr::as_ptr(self) }
	}
}


/// Drawable string with known size and raw pointer to its contents.
pub trait AsRawStr: Debug {
	fn count(&self, enc: StringEncoding) -> usize;
	fn as_ptr(&self) -> *const c_char;
}

trait AsRawStrValidate: AsRawStr {
	#[inline(always)]
	#[allow(unused_variables)]
	fn debug_validate(&self, len: usize, enc: StringEncoding) {
		#[cfg(debug_assertions)]
		{
			let max = self.count(enc);
			debug_assert!(len <= max, "{len} > {max} len of {self:?}");
		}
	}
}
impl<T: AsRawStr> AsRawStrValidate for T {}


impl Graphics {
	/// Draws the given `text` using the provided options.
	///
	/// Note that `len` is the length of the decoded string, the number of codepoints in the string, not the number of bytes; \
	/// however, __for `CStr`__ since the parser stops at the `NUL` terminator it’s safe to pass bytes-count in here when you want to draw the entire string.
	/// Same for `str` __if it only contains ASCII__.
	///
	/// If no `font` has been set with [`set_font`],
	/// the default system font `Asheville Sans 14 Light` is used.
	///
	/// See also [`draw_text_in_rect`](Self::draw_text_in_rect).
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawText`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawText")]
	pub fn draw_text(&self, txt: impl AsRawStr, len: usize, enc: StringEncoding, x: c_int, y: c_int) -> c_int {
		txt.debug_validate(len, enc);
		unsafe { (self.0.drawText)(txt.as_ptr().cast(), len, enc, x, y) }
	}


	/// Draws the `text` in the given rectangle using the provided options.
	///
	/// See [`draw_text`](Self::draw_text) for more information.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawTextInRect`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawTextInRect")]
	pub fn draw_text_in_rect(&self,
	                         txt: impl AsRawStr,
	                         len: usize,
	                         enc: StringEncoding,
	                         x: c_int,
	                         y: c_int,
	                         width: c_int,
	                         height: c_int,
	                         wrap: TextWrappingMode,
	                         align: TextAlignment) {
		txt.debug_validate(len, enc);
		let f = self.0.drawTextInRect;
		unsafe { f(txt.as_ptr().cast(), len, enc, x, y, width, height, wrap, align) };
	}

	/// Calculates and returns the width of the given `text` in this font,
	/// without rendering.
	///
	/// See also [`Font::text_width`].
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getTextWidth`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getTextWidth")]
	pub fn text_width(&self,
	                  txt: impl AsRawStr,
	                  len: usize,
	                  enc: StringEncoding,
	                  font: Option<&Font>,
	                  tracking: c_int)
	                  -> c_int {
		txt.debug_validate(len, enc);
		let f = self.0.getTextWidth;
		let font = font.map(|font| font.0.as_ptr()).unwrap_or(null_mut());
		unsafe { f(font, txt.as_ptr().cast(), len, enc, tracking) }
	}


	/// Sets the `font` to use in subsequent [`draw_text`] calls.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setFont`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setFont")]
	pub fn set_font(&self, font: &Font) { unsafe { (self.0.setFont)(font.0.as_ptr()) } }


	/// Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setTextLeading`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setTextLeading")]
	pub fn set_text_leading(&self, line_height_adjustment: c_int) {
		unsafe { (self.0.setTextLeading)(line_height_adjustment) }
	}

	/// Sets the tracking to use when drawing text.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setTextTracking`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setTextTracking")]
	pub fn set_text_tracking(&self, tracking: c_int) { unsafe { (self.0.setTextTracking)(tracking) } }

	/// Gets the tracking used when drawing text.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getTextTracking`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getTextTracking")]
	pub fn text_tracking(&self) -> c_int { unsafe { (self.0.getTextTracking)() } }
}


/// Playdate Font representation.
///
/// See [official docs][] for more information.
///
/// [official docs]: https://sdk.play.date/Inside%20Playdate.html#C-graphics.font
pub struct Font(NonNull<LCDFont>);

impl Drop for Font {
	fn drop(&mut self) { unsafe { sys::allocator::realloc(self.0.as_ptr().cast(), 0) }; }
}


impl Font {
	/// Returns the [`Font`] object for the font file at `path`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::loadFont`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::loadFont")]
	#[inline(always)]
	pub fn load(api: Api, path: impl AsRef<Path>) -> Result<Self, error::LoadError> {
		let path = path.as_ref();
		let mut err: *const c_char = core::ptr::null();

		let ptr = unsafe { (api.loadFont)(path.as_ptr(), &raw mut err) };

		if ptr.is_null() {
			if let Some(err) = unsafe { fs::error::Owned::from_ptr(err) } {
				Err(error::LoadError::Fs(err))
			} else {
				Err(error::LoadError::Alloc(error::Alloc))
			}
		} else {
			Ok(Self(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Returns an [`Font`] object wrapping the LCDFontData data
	/// comprising the contents (minus 16-byte header) of an uncompressed pft file.
	///
	/// The `wide` corresponds to the flag in the header indicating
	/// whether the font contains glyphs at codepoints above `U+1FFFF`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::makeFontFromData`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::makeFontFromData")]
	pub fn new_from_bytes(&self, api: Api, data: &[u8], wide: c_int) -> Result<Font, error::Alloc> {
		let ptr = unsafe { (api.makeFontFromData)(data.as_ptr() as _, wide) };
		NonNull::new(ptr).map(Self).ok_or(error::Alloc)
	}


	/// Returns an [`Page`] object for the given character code `c`.
	///
	/// Each [`Page`] contains information for 256 characters;
	/// specifically, if `(c1 & ~0xff) == (c2 & ~0xff)`,
	/// then `c1` and `c2` belong to the same page and the same [`Page`]
	/// can be used to fetch the character data for both instead of searching for the page twice.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getFontPage`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getFontPage")]
	pub fn page(&self, api: Api, c: u32) -> Result<Page, error::FontError> {
		let ptr = unsafe { (api.getFontPage)(self.0.as_ptr(), c) };
		NonNull::new(ptr).map(Page).ok_or(error::FontError)
	}


	/// Returns the height of this font.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getFontHeight`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getFontHeight")]
	#[inline(always)]
	pub fn height(&self, api: Api) -> u8 { unsafe { (api.getFontHeight)(self.0.as_ptr()) } }


	/// Calculates and returns the width of the given `text` in this font,
	/// without rendering.
	///
	/// See also [`Graphics::text_width`].
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getTextWidth`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getTextWidth")]
	pub fn text_width(&self,
	                  api: Api,
	                  txt: impl AsRawStr,
	                  len: usize,
	                  enc: StringEncoding,
	                  tracking: c_int)
	                  -> c_int {
		txt.debug_validate(len, enc);
		let f = api.getTextWidth;
		unsafe { f(self.0.as_ptr(), txt.as_ptr().cast(), len, enc, tracking) }
	}
}


/// Playdate Glyph representation.
pub struct Glyph(NonNull<FontGlyph>);

impl Glyph {
	/// Returns the kerning adjustment between characters `glyph_code` and `next_code` as specified by the font
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getGlyphKerning`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getGlyphKerning")]
	pub fn kerning(&self, api: Api, glyph_code: u32, next_code: u32) -> c_int {
		unsafe { (api.getGlyphKerning)(self.0.as_ptr(), glyph_code, next_code) }
	}
}


/// Playdate FontPage representation.
pub struct Page(NonNull<FontPage>);


impl Page {
	/// Returns an [`Glyph`] object for character `c` in [`Page`] page,
	///
	/// To also get the glyph’s bitmap and `advance` value
	/// use [`glyph_with_bitmap`] instead.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getPageGlyph`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getPageGlyph")]
	pub fn glyph(&self, api: Api, c: u32) -> Result<Glyph, error::FontError> {
		let f = api.getPageGlyph;
		let ptr = unsafe { f(self.0.as_ptr(), c, null_mut(), null_mut()) };
		NonNull::new(ptr).map(Glyph).ok_or(error::FontError)
	}

	/// Returns an [`Glyph`] object for character `c` in [`Page`] page,
	/// and optionally returns the glyph’s bitmap and `advance` value.
	///
	/// If bitmap is not needed, use [`glyph`] instead.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getPageGlyph`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getPageGlyph")]
	pub fn glyph_with_bitmap<'p>(&'p self,
	                             api: Api,
	                             c: u32,
	                             advance: &mut c_int)
	                             -> Result<(Glyph, Option<Borrowed<'p>>), error::FontError> {
		let mut bitmap: *mut SysBitmap = null_mut();

		let f = api.getPageGlyph;
		let ptr = unsafe { f(self.0.as_ptr(), c, &raw mut bitmap, advance) };

		NonNull::new(ptr).map(Glyph)
		                 .map(|g| (g, NonNull::new(bitmap).map(Borrowed::from_ptr)))
		                 .ok_or(error::FontError)
	}
}


/* TODO:
	draw_text_in_rect				drawTextInRect
	get_text_width				getTextWidth
	get_font_height				getFontHeight
	set_font				setFont
	set_text_tracking				setTextTracking
	get_text_tracking				getTextTracking
	get_glyph_kerning				getGlyphKerning
	load_font				loadFont
	get_font_page				getFontPage
	get_page_glyph				getPageGlyph
	make_font_from_data				makeFontFromData
	set_text_leading				setTextLeading
*/


pub mod error {
	use core::fmt;
	pub(super) use crate::error::*;


	/// Font error.
	/// This occurs when char or page not found.
	#[derive(Debug)]
	pub struct FontError;
	impl core::error::Error for FontError {}
	impl fmt::Display for FontError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "Mask must be the same size as the target bitmap")
		}
	}
}
