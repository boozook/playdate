#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
	storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
	#[inline]
	pub const fn new(storage: Storage) -> Self { Self { storage } }
}
impl<Storage> __BindgenBitfieldUnit<Storage> where Storage: AsRef<[u8]> + AsMut<[u8]> {
	#[inline]
	fn extract_bit(byte: u8, index: usize) -> bool {
		let bit_index = if cfg!(target_endian = "big") {
			7 - (index % 8)
		} else {
			index % 8
		};
		let mask = 1 << bit_index;
		byte & mask == mask
	}
	#[inline]
	pub fn get_bit(&self, index: usize) -> bool {
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = self.storage.as_ref()[byte_index];
		Self::extract_bit(byte, index)
	}
	#[inline]
	pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
		debug_assert!(index / 8 < core::mem::size_of::<Storage>());
		let byte_index = index / 8;
		let byte = *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize);
		Self::extract_bit(byte, index)
	}
	#[inline]
	fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
		let bit_index = if cfg!(target_endian = "big") {
			7 - (index % 8)
		} else {
			index % 8
		};
		let mask = 1 << bit_index;
		if val { byte | mask } else { byte & !mask }
	}
	#[inline]
	pub fn set_bit(&mut self, index: usize, val: bool) {
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = &mut self.storage.as_mut()[byte_index];
		*byte = Self::change_bit(*byte, index, val);
	}
	#[inline]
	pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
		debug_assert!(index / 8 < core::mem::size_of::<Storage>());
		let byte_index = index / 8;
		let byte = (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize);
		*byte = Self::change_bit(*byte, index, val);
	}
	#[inline]
	pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
		let mut val = 0;
		for i in 0..(bit_width as usize) {
			if self.get_bit(i + bit_offset) {
				let index = if cfg!(target_endian = "big") {
					bit_width as usize - 1 - i
				} else {
					i
				};
				val |= 1 << index;
			}
		}
		val
	}
	#[inline]
	pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
		let mut val = 0;
		for i in 0..(bit_width as usize) {
			if Self::raw_get_bit(this, i + bit_offset) {
				let index = if cfg!(target_endian = "big") {
					bit_width as usize - 1 - i
				} else {
					i
				};
				val |= 1 << index;
			}
		}
		val
	}
	#[inline]
	pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
		for i in 0..(bit_width as usize) {
			let mask = 1 << i;
			let val_bit_is_set = val & mask == mask;
			let index = if cfg!(target_endian = "big") {
				bit_width as usize - 1 - i
			} else {
				i
			};
			self.set_bit(index + bit_offset, val_bit_is_set);
		}
	}
	#[inline]
	pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
		for i in 0..(bit_width as usize) {
			let mask = 1 << i;
			let val_bit_is_set = val & mask == mask;
			let index = if cfg!(target_endian = "big") {
				bit_width as usize - 1 - i
			} else {
				i
			};
			Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
		}
	}
}
pub const LCD_COLUMNS: u32 = 400;
pub const LCD_ROWS: u32 = 240;
pub const LCD_ROWSIZE: u32 = 52;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const AUDIO_FRAMES_PER_CYCLE: u32 = 512;
pub const NOTE_C4: u32 = 60;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
#[must_use]
pub struct Aabb {
	pub left: core::ffi::c_int,
	pub right: core::ffi::c_int,
	pub top: core::ffi::c_int,
	pub bottom: core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of Aabb"][::core::mem::size_of::<Aabb>() - 16usize];
	["Alignment of Aabb"][::core::mem::align_of::<Aabb>() - 4usize];
	["Offset of field: Aabb::left"][::core::mem::offset_of!(Aabb, left) - 0usize];
	["Offset of field: Aabb::right"][::core::mem::offset_of!(Aabb, right) - 4usize];
	["Offset of field: Aabb::top"][::core::mem::offset_of!(Aabb, top) - 8usize];
	["Offset of field: Aabb::bottom"][::core::mem::offset_of!(Aabb, bottom) - 12usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum BitmapDrawMode {
	Copy = 0,
	WhiteTransparent = 1,
	BlackTransparent = 2,
	FillWhite = 3,
	FillBlack = 4,
	XOR = 5,
	NXOR = 6,
	Inverted = 7,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum BitmapFlip {
	Unflipped = 0,
	FlippedX = 1,
	FlippedY = 2,
	FlippedXy = 3,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SolidColor {
	Black = 0,
	White = 1,
	Clear = 2,
	XOR = 3,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LineCapStyle {
	Butt = 0,
	Square = 1,
	Round = 2,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum FontLanguage {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum StringEncoding {
	ASCII = 0,
	UTF8 = 1,
	UTF16 = 2,
}
pub type Pattern = [u8; 16usize];
pub type Color = usize;
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PolygonFillRule {
	NonZero = 0,
	EvenOdd = 1,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum TextWrappingMode {
	Clip = 0,
	Character = 1,
	Word = 2,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum TextAlignment {
	Left = 0,
	Center = 1,
	Right = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Bitmap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct BitmapTable {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Font {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct FontData {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct FontPage {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct FontGlyph {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct VideoPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateVideo {
	#[doc = "Opens the *pdv* file at *path* and returns a new video player object for rendering its frames.\n"]
	pub loadVideo: unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut VideoPlayer,
	#[doc = "Frees the given video player.\n"]
	pub freePlayer: unsafe extern "C" fn(p: *mut VideoPlayer),
	#[doc = "Sets the rendering destination for the video player to the given bitmap. If the function fails, it returns 0 and sets an error message that can be read via [getError()](#f-graphics.video.getError).\n"]
	pub setContext: unsafe extern "C" fn(p: *mut VideoPlayer, context: *mut Bitmap) -> core::ffi::c_int,
	#[doc = "Sets the rendering destination for the video player to the screen.\n"]
	pub useScreenContext: unsafe extern "C" fn(p: *mut VideoPlayer),
	#[doc = "Renders frame number *n* into the current context. In case of error, the function returns 0 and sets an error message that can be read via [getError()](#f-graphics.video.getError).\n"]
	pub renderFrame: unsafe extern "C" fn(p: *mut VideoPlayer, n: core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Returns text describing the most recent error.\n"]
	pub getError: unsafe extern "C" fn(p: *mut VideoPlayer) -> *const core::ffi::c_char,
	#[doc = "Retrieves information about the video, by passing in (possibly NULL) value pointers.\n"]
	pub getInfo: unsafe extern "C" fn(p: *mut VideoPlayer,
	                                  outWidth: *mut core::ffi::c_int,
	                                  outHeight: *mut core::ffi::c_int,
	                                  outFrameRate: *mut core::ffi::c_float,
	                                  outFrameCount: *mut core::ffi::c_int,
	                                  outCurrentFrame: *mut core::ffi::c_int),
	#[doc = "Gets the rendering destination for the video player. If no rendering context has been setallocates a context bitmap with the same dimensions as the vieo will be allocated.\n"]
	pub getContext: unsafe extern "C" fn(p: *mut VideoPlayer) -> *mut Bitmap,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateVideo"][::core::mem::size_of::<PlaydateVideo>() - 32usize];
	["Alignment of PlaydateVideo"][::core::mem::align_of::<PlaydateVideo>() - 4usize];
	["Offset of field: PlaydateVideo::loadVideo"][::core::mem::offset_of!(PlaydateVideo, loadVideo) - 0usize];
	["Offset of field: PlaydateVideo::freePlayer"][::core::mem::offset_of!(PlaydateVideo, freePlayer) - 4usize];
	["Offset of field: PlaydateVideo::setContext"][::core::mem::offset_of!(PlaydateVideo, setContext) - 8usize];
	["Offset of field: PlaydateVideo::useScreenContext"]
		[::core::mem::offset_of!(PlaydateVideo, useScreenContext) - 12usize];
	["Offset of field: PlaydateVideo::renderFrame"][::core::mem::offset_of!(PlaydateVideo, renderFrame) - 16usize];
	["Offset of field: PlaydateVideo::getError"][::core::mem::offset_of!(PlaydateVideo, getError) - 20usize];
	["Offset of field: PlaydateVideo::getInfo"][::core::mem::offset_of!(PlaydateVideo, getInfo) - 24usize];
	["Offset of field: PlaydateVideo::getContext"][::core::mem::offset_of!(PlaydateVideo, getContext) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateGraphics {
	pub video: &'static PlaydateVideo,
	#[doc = "Clears the entire display, filling it with *color*.\nEquivalent to [```\nplaydate.graphics.clear()```\n](<./Inside Playdate.html#f-graphics.clear>) in the Lua API.\n"]
	pub clear: unsafe extern "C" fn(color: Color),
	#[doc = "Sets the background color shown when the display is [offset](#f-display.setOffset) or for clearing dirty areas in the sprite system.\nEquivalent to [```\nplaydate.graphics.setBackgroundColor()```\n](<./Inside Playdate.html#f-graphics.setBackgroundColor>) in the Lua API.\n"]
	pub setBackgroundColor: unsafe extern "C" fn(color: SolidColor),
	#[doc = "Sets the stencil used for drawing. For a tiled stencil, use *setStencilImage()* instead. To clear the stencil, set it to *NULL*.\n"]
	pub setStencil: unsafe extern "C" fn(stencil: *mut Bitmap),
	#[doc = "Sets the mode used for drawing bitmaps. Note that text drawing uses bitmaps, so this affects how fonts are displayed as well. Returns the previous draw mode, in case you need to restore it after drawing.\nLCDBitmapDrawModetypedef enum\n{\n\tkDrawModeCopy,\n\tkDrawModeWhiteTransparent,\n\tkDrawModeBlackTransparent,\n\tkDrawModeFillWhite,\n\tkDrawModeFillBlack,\n\tkDrawModeXOR,\n\tkDrawModeNXOR,\n\tkDrawModeInverted\n} LCDBitmapDrawMode;Equivalent to [```\nplaydate.graphics.setImageDrawMode()```\n](<./Inside Playdate.html#f-graphics.setImageDrawMode>) in the Lua API.\n"]
	pub setDrawMode: unsafe extern "C" fn(mode: BitmapDrawMode) -> BitmapDrawMode,
	#[doc = "Offsets the origin point for all drawing calls to *x*, *y* (can be negative).\nThis is useful, for example, for centering a \"camera\" on a sprite that is moving around a world larger than the screen.\nEquivalent to [```\nplaydate.graphics.setDrawOffset()```\n](<./Inside Playdate.html#f-graphics.setDrawOffset>) in the Lua API.\n"]
	pub setDrawOffset: unsafe extern "C" fn(dx: core::ffi::c_int, dy: core::ffi::c_int),
	#[doc = "Sets the current clip rect, using world coordinates—\u{200b}that is, the given rectangle will be translated by the current drawing offset. The clip rect is cleared at the beginning of each update.\nEquivalent to [```\nplaydate.graphics.setClipRect()```\n](<./Inside Playdate.html#f-graphics.setClipRect>) in the Lua API.\n"]
	pub setClipRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                      y: core::ffi::c_int,
	                                      width: core::ffi::c_int,
	                                      height: core::ffi::c_int),
	#[doc = "Clears the current clip rect.\nEquivalent to [```\nplaydate.graphics.clearClipRect()```\n](<./Inside Playdate.html#f-graphics.clearClipRect>) in the Lua API.\n"]
	pub clearClipRect: unsafe extern "C" fn(),
	#[doc = "Sets the end cap style used in the line drawing functions.\nLCDLineCapStyletypedef enum\n{\n\tkLineCapStyleButt,\n\tkLineCapStyleSquare,\n\tkLineCapStyleRound\n} LCDLineCapStyle;Equivalent to [```\nplaydate.graphics.setLineCapStyle()```\n](<./Inside Playdate.html#f-graphics.setLineCapStyle>) in the Lua API.\n"]
	pub setLineCapStyle: unsafe extern "C" fn(endCapStyle: LineCapStyle),
	#[doc = "Sets the font to use in subsequent [drawText](#f-graphics.drawText) calls.\nEquivalent to [```\nplaydate.graphics.setFont()```\n](<./Inside Playdate.html#f-graphics.setFont>) in the Lua API.\n"]
	pub setFont: unsafe extern "C" fn(font: *mut Font),
	#[doc = "Sets the tracking to use when drawing text.\nEquivalent to [```\nplaydate.graphics.font:setTracking()```\n](<./Inside Playdate.html#m-graphics.font.setTracking>) in the Lua API.\n"]
	pub setTextTracking: unsafe extern "C" fn(tracking: core::ffi::c_int),
	#[doc = "Push a new drawing context for drawing into the given bitmap. If *target* is *NULL*, the drawing functions will use the display framebuffer.\nEquivalent to [```\nplaydate.graphics.pushContext()```\n](<./Inside Playdate.html#f-graphics.pushContext>) in the Lua API.\n"]
	pub pushContext: unsafe extern "C" fn(target: *mut Bitmap),
	#[doc = "Pops a context off the stack (if any are left), restoring the drawing settings from before the context was pushed.\nEquivalent to [```\nplaydate.graphics.popContext()```\n](<./Inside Playdate.html#f-graphics.popContext>) in the Lua API.\n"]
	pub popContext: unsafe extern "C" fn(),
	#[doc = "Draws the *bitmap* with its upper-left corner at location *x*, *y*, using the given flip orientation.\n"]
	pub drawBitmap:
		unsafe extern "C" fn(bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int, flip: BitmapFlip),
	#[doc = "Draws the *bitmap* with its upper-left corner at location *x*, *y* tiled inside a *width* by *height* rectangle.\n"]
	pub tileBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                     x: core::ffi::c_int,
	                                     y: core::ffi::c_int,
	                                     width: core::ffi::c_int,
	                                     height: core::ffi::c_int,
	                                     flip: BitmapFlip),
	#[doc = "Draws a line from *x1*, *y1* to *x2*, *y2* with a stroke width of *width*.\nEquivalent to [```\nplaydate.graphics.drawLine()```\n](<./Inside Playdate.html#f-graphics.drawLine>) in the Lua API.\n"]
	pub drawLine: unsafe extern "C" fn(x1: core::ffi::c_int,
	                                   y1: core::ffi::c_int,
	                                   x2: core::ffi::c_int,
	                                   y2: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   color: Color),
	#[doc = "Draws a filled triangle with points at *x1*, *y1*, *x2*, *y2*, and *x3*, *y3*.\nLCDWindingRuletypedef enum\n{\n\tkPolygonFillNonZero,\n\tkPolygonFillEvenOdd\n} LCDPolygonFillRule;Equivalent to [```\nplaydate.graphics.fillTriangle()```\n](<./Inside Playdate.html#f-graphics.fillTriangle>) in the Lua API.\n"]
	pub fillTriangle: unsafe extern "C" fn(x1: core::ffi::c_int,
	                                       y1: core::ffi::c_int,
	                                       x2: core::ffi::c_int,
	                                       y2: core::ffi::c_int,
	                                       x3: core::ffi::c_int,
	                                       y3: core::ffi::c_int,
	                                       color: Color),
	#[doc = "Draws a *width* by *height* rect at *x*, *y*.\nEquivalent to [```\nplaydate.graphics.drawRect()```\n](<./Inside Playdate.html#f-graphics.drawRect>) in the Lua API.\n"]
	pub drawRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                   y: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   height: core::ffi::c_int,
	                                   color: Color),
	#[doc = "Draws a filled *width* by *height* rect at *x*, *y*.\nEquivalent to [```\nplaydate.graphics.fillRect()```\n](<./Inside Playdate.html#f-graphics.fillRect>) in the Lua API.\n"]
	pub fillRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                   y: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   height: core::ffi::c_int,
	                                   color: Color),
	#[doc = "Draws an ellipse inside the rectangle {x, y, width, height} of width *lineWidth* (inset from the rectangle bounds). If *startAngle* != _endAngle, this draws an arc between the given angles. Angles are given in degrees, clockwise from due north.\n"]
	pub drawEllipse: unsafe extern "C" fn(x: core::ffi::c_int,
	                                      y: core::ffi::c_int,
	                                      width: core::ffi::c_int,
	                                      height: core::ffi::c_int,
	                                      lineWidth: core::ffi::c_int,
	                                      startAngle: core::ffi::c_float,
	                                      endAngle: core::ffi::c_float,
	                                      color: Color),
	#[doc = "Fills an ellipse inside the rectangle {x, y, width, height}. If *startAngle* != _endAngle, this draws a wedge/Pacman between the given angles. Angles are given in degrees, clockwise from due north.\n"]
	pub fillEllipse: unsafe extern "C" fn(x: core::ffi::c_int,
	                                      y: core::ffi::c_int,
	                                      width: core::ffi::c_int,
	                                      height: core::ffi::c_int,
	                                      startAngle: core::ffi::c_float,
	                                      endAngle: core::ffi::c_float,
	                                      color: Color),
	#[doc = "Draws the *bitmap* scaled to *xscale* and *yscale* with its upper-left corner at location *x*, *y*. Note that *flip* is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.\n"]
	pub drawScaledBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                           x: core::ffi::c_int,
	                                           y: core::ffi::c_int,
	                                           xscale: core::ffi::c_float,
	                                           yscale: core::ffi::c_float),
	#[doc = "Draws the given text using the provided options. If no font has been set with [setFont](#f-graphics.setFont), the default system font Asheville Sans 14 Light is used. Note that ```\nlen```\n is the length of the **decoded** string—\u{200b}that is, the number of codepoints in the string, not the number of bytes; however, since the parser stops at the NUL terminator it’s safe to pass ```\nstrlen(text)```\n in here when you want to draw the entire string.\nEquivalent to [```\nplaydate.graphics.drawText()```\n](<./Inside Playdate.html#f-graphics.drawText>) in the Lua API.\n"]
	pub drawText: unsafe extern "C" fn(text: *const core::ffi::c_void,
	                                   len: usize,
	                                   encoding: StringEncoding,
	                                   x: core::ffi::c_int,
	                                   y: core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Allocates and returns a new *width* by *height* LCDBitmap filled with *bgcolor*.\n"]
	pub newBitmap:
		unsafe extern "C" fn(width: core::ffi::c_int, height: core::ffi::c_int, bgcolor: Color) -> *mut Bitmap,
	#[doc = "Frees the given *bitmap*.\n"]
	pub freeBitmap: unsafe extern "C" fn(arg1: *mut Bitmap),
	#[doc = "Allocates and returns a new LCDBitmap from the file at *path*. If there is no file at *path*, the function returns null.\n"]
	pub loadBitmap:
		unsafe extern "C" fn(path: *const core::ffi::c_char, outerr: *mut *const core::ffi::c_char) -> *mut Bitmap,
	#[doc = "Returns a new LCDBitmap that is an exact copy of *bitmap*.\n"]
	pub copyBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap) -> *mut Bitmap,
	#[doc = "Loads the image at *path* into the previously allocated *bitmap*.\n"]
	pub loadIntoBitmap: unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                         bitmap: *mut Bitmap,
	                                         outerr: *mut *const core::ffi::c_char),
	#[doc = "Gets various info about *bitmap* including its *width* and *height* and raw pixel data. The data is 1 bit per pixel packed format, in MSB order; in other words, the high bit of the first byte in ```\ndata```\n is the top left pixel of the image. If the bitmap has a mask, a pointer to its data is returned in *mask*, else NULL is returned.\n"]
	pub getBitmapData: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                        width: *mut core::ffi::c_int,
	                                        height: *mut core::ffi::c_int,
	                                        rowbytes: *mut core::ffi::c_int,
	                                        mask: *mut *mut u8,
	                                        data: *mut *mut u8),
	#[doc = "Clears *bitmap*, filling with the given *bgcolor*.\n"]
	pub clearBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap, bgcolor: Color),
	#[doc = "Returns a new, rotated and scaled LCDBitmap based on the given *bitmap*.\n"]
	pub rotatedBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                        rotation: core::ffi::c_float,
	                                        xscale: core::ffi::c_float,
	                                        yscale: core::ffi::c_float,
	                                        allocedSize: *mut core::ffi::c_int)
	                                        -> *mut Bitmap,
	#[doc = "Allocates and returns a new LCDBitmapTable that can hold *count**width* by *height* LCDBitmaps.\n"]
	pub newBitmapTable: unsafe extern "C" fn(count: core::ffi::c_int,
	                                         width: core::ffi::c_int,
	                                         height: core::ffi::c_int)
	                                         -> *mut BitmapTable,
	#[doc = "Frees the given bitmap table. Note that this will invalidate any bitmaps returned by ```\ngetTableBitmap()```\n.\n"]
	pub freeBitmapTable: unsafe extern "C" fn(table: *mut BitmapTable),
	#[doc = "Allocates and returns a new LCDBitmap from the file at *path*. If there is no file at *path*, the function returns null.\n"]
	pub loadBitmapTable: unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                          outerr: *mut *const core::ffi::c_char)
	                                          -> *mut BitmapTable,
	#[doc = "Loads the imagetable at *path* into the previously allocated *table*.\n"]
	pub loadIntoBitmapTable: unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                              table: *mut BitmapTable,
	                                              outerr: *mut *const core::ffi::c_char),
	#[doc = "Returns the *idx* bitmap in *table*, If *idx* is out of bounds, the function returns NULL.\n"]
	pub getTableBitmap: unsafe extern "C" fn(table: *mut BitmapTable, idx: core::ffi::c_int) -> *mut Bitmap,
	#[doc = "Returns the LCDFont object for the font file at *path*. In case of error, *outErr* points to a string describing the error. The returned font can be freed with [playdate→system→realloc(font, 0)](#f-system.realloc) when it is no longer in use.\n"]
	pub loadFont:
		unsafe extern "C" fn(path: *const core::ffi::c_char, outErr: *mut *const core::ffi::c_char) -> *mut Font,
	#[doc = "Returns an LCDFontPage object for the given character code. Each LCDFontPage contains information for 256 characters; specifically, if ```\n(c1 &amp; ~0xff) == (c2 &amp; ~0xff)```\n, then *c1* and *c2* belong to the same page and the same LCDFontPage can be used to fetch the character data for both instead of searching for the page twice.\n"]
	pub getFontPage: unsafe extern "C" fn(font: *mut Font, c: u32) -> *mut FontPage,
	#[doc = "Returns an LCDFontGlyph object for character *c* in LCDFontPage *page*, and optionally returns the glyph’s bitmap and advance value.\n"]
	pub getPageGlyph: unsafe extern "C" fn(page: *mut FontPage,
	                                       c: u32,
	                                       bitmap: *mut *mut Bitmap,
	                                       advance: *mut core::ffi::c_int)
	                                       -> *mut FontGlyph,
	#[doc = "Returns the kerning adjustment between characters *c1* and *c2* as specified by the font.\n"]
	pub getGlyphKerning:
		unsafe extern "C" fn(glyph: *mut FontGlyph, glyphcode: u32, nextcode: u32) -> core::ffi::c_int,
	#[doc = "Returns the width of the given text in the given font. See the [note above](#f-graphics.drawText) about the ```\nlen```\n argument.\nPDStringEncodingtypedef enum\n{\n\tkASCIIEncoding,\n\tkUTF8Encoding,\n\tk16BitLEEncoding\n} PDStringEncoding;"]
	pub getTextWidth: unsafe extern "C" fn(font: *mut Font,
	                                       text: *const core::ffi::c_void,
	                                       len: usize,
	                                       encoding: StringEncoding,
	                                       tracking: core::ffi::c_int)
	                                       -> core::ffi::c_int,
	#[doc = "Returns the current display frame buffer. Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored. Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.\n"]
	pub getFrame: unsafe extern "C" fn() -> *mut u8,
	#[doc = "Returns the raw bits in the display buffer, the last completed frame.\n"]
	pub getDisplayFrame: unsafe extern "C" fn() -> *mut u8,
	#[doc = "Only valid in the Simulator; function is NULL on device. Returns the debug framebuffer as a bitmap. White pixels drawn in the image are overlaid on the display in 50% transparent red.\n"]
	pub getDebugBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut Bitmap>,
	#[doc = "Returns a copy the contents of the working frame buffer as a bitmap. The caller is responsible for freeing the returned bitmap with [playdate-&gt;graphics-&gt;freeBitmap()](#f-graphics.freeBitmap).\n"]
	pub copyFrameBufferBitmap: unsafe extern "C" fn() -> *mut Bitmap,
	#[doc = "After updating pixels in the buffer returned by getFrame(), you must tell the graphics system which rows were updated. This function marks a contiguous range of rows as updated (e.g., markUpdatedRows(0,LCD_ROWS-1) tells the system to update the entire display). Both “start” and “end” are included in the range.\n"]
	pub markUpdatedRows: unsafe extern "C" fn(start: core::ffi::c_int, end: core::ffi::c_int),
	#[doc = "Manually flushes the current frame buffer out to the display. This function is automatically called after each pass through the run loop, so there shouldn’t be any need to call it yourself.\n"]
	pub display: unsafe extern "C" fn(),
	#[doc = "Sets *color* to an 8 x 8 pattern using the given *bitmap*. *x*, *y* indicates the top left corner of the 8 x 8 pattern.\n"]
	pub setColorToPattern:
		unsafe extern "C" fn(color: *mut Color, bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int),
	#[doc = "Returns 1 if any of the opaque pixels in *bitmap1* when positioned at *x1*, *y1* with *flip1* overlap any of the opaque pixels in *bitmap2* at *x2*, *y2* with *flip2* within the non-empty *rect*, or 0 if no pixels overlap or if one or both fall completely outside of *rect*.\n"]
	pub checkMaskCollision: unsafe extern "C" fn(bitmap1: *mut Bitmap,
	                                             x1: core::ffi::c_int,
	                                             y1: core::ffi::c_int,
	                                             flip1: BitmapFlip,
	                                             bitmap2: *mut Bitmap,
	                                             x2: core::ffi::c_int,
	                                             y2: core::ffi::c_int,
	                                             flip2: BitmapFlip,
	                                             rect: Aabb)
	                                             -> core::ffi::c_int,
	#[doc = "Sets the current clip rect in screen coordinates.\nEquivalent to [```\nplaydate.graphics.setScreenClipRect()```\n](<./Inside Playdate.html#f-graphics.setScreenClipRect>) in the Lua API.\n"]
	pub setScreenClipRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                            y: core::ffi::c_int,
	                                            width: core::ffi::c_int,
	                                            height: core::ffi::c_int),
	#[doc = "Fills the polygon with vertices at the given coordinates (an array of 2*```\nnPoints```\n ints containing alternating x and y values) using the given color and fill, or winding, rule. See [https://en.wikipedia.org/wiki/Nonzero-rule](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule. An edge between the last vertex and the first is assumed.\nEquivalent to [```\nplaydate.graphics.fillPolygon()```\n](<./Inside Playdate.html#f-graphics.fillPolygon>) in the Lua API.\n"]
	pub fillPolygon: unsafe extern "C" fn(nPoints: core::ffi::c_int,
	                                      coords: *mut core::ffi::c_int,
	                                      color: Color,
	                                      fillrule: PolygonFillRule),
	#[doc = "Returns the height of the given font.\n"]
	pub getFontHeight: unsafe extern "C" fn(font: *mut Font) -> u8,
	#[doc = "Returns a bitmap containing the contents of the display buffer. The system owns this bitmap—\u{200b}do not free it!\n"]
	pub getDisplayBufferBitmap: unsafe extern "C" fn() -> *mut Bitmap,
	#[doc = "Draws the *bitmap* scaled to *xscale* and *yscale* then rotated by *degrees* with its center as given by proportions *centerx* and *centery* at *x*, *y*; that is: if *centerx* and *centery* are both 0.5 the center of the image is at (*x*,*y*), if *centerx* and *centery* are both 0 the top left corner of the image (before rotation) is at (*x*,*y*), etc.\n"]
	pub drawRotatedBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                            x: core::ffi::c_int,
	                                            y: core::ffi::c_int,
	                                            rotation: core::ffi::c_float,
	                                            centerx: core::ffi::c_float,
	                                            centery: core::ffi::c_float,
	                                            xscale: core::ffi::c_float,
	                                            yscale: core::ffi::c_float),
	#[doc = "Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.\nEquivalent to [```\nplaydate.graphics.font:setLeading()```\n](<./Inside Playdate.html#m-graphics.font.setLeading>) in the Lua API.\n"]
	pub setTextLeading: unsafe extern "C" fn(lineHeightAdustment: core::ffi::c_int),
	#[doc = "Sets a mask image for the given *bitmap*. The set mask must be the same size as the target bitmap.\n"]
	pub setBitmapMask: unsafe extern "C" fn(bitmap: *mut Bitmap, mask: *mut Bitmap) -> core::ffi::c_int,
	#[doc = "Gets a mask image for the given *bitmap*, or returns NULL if the *bitmap* doesn’t have a mask layer. The returned image points to *bitmap*'s data, so drawing into the mask image affects the source bitmap directly. The caller takes ownership of the returned LCDBitmap and is responsible for freeing it when it’s no longer in use.\n"]
	pub getBitmapMask: unsafe extern "C" fn(bitmap: *mut Bitmap) -> *mut Bitmap,
	#[doc = "Sets the stencil used for drawing. If the *tile* flag is set the stencil image will be tiled. Tiled stencils must have width equal to a multiple of 32 pixels. To clear the stencil, call ```\nplaydate→graphics→setStencil(NULL);```\n.\nEquivalent to [```\nplaydate.graphics.setStencilImage()```\n](<./Inside Playdate.html#f-graphics.setStencilImage>) in the Lua API.\n"]
	pub setStencilImage: unsafe extern "C" fn(stencil: *mut Bitmap, tile: core::ffi::c_int),
	#[doc = "Returns an LCDFont object wrapping the LCDFontData *data* comprising the contents (minus 16-byte header) of an uncompressed pft file. *wide* corresponds to the flag in the header indicating whether the font contains glyphs at codepoints above U+1FFFF.\n"]
	pub makeFontFromData: unsafe extern "C" fn(data: *mut FontData, wide: core::ffi::c_int) -> *mut Font,
	#[doc = "Gets the tracking used when drawing text.\nEquivalent to [```\nplaydate.graphics.font:getTracking()```\n](<./Inside Playdate.html#m-graphics.font.getTracking>) in the Lua API.\n"]
	pub getTextTracking: unsafe extern "C" fn() -> core::ffi::c_int,
	#[doc = "Sets the pixel at *(x,y)* in the current drawing context (by default the screen) to the given *color*. Be aware that setting a pixel at a time is not very efficient: In our testing, more than around 20,000 calls in a tight loop will drop the frame rate below 30 fps.\n"]
	pub setPixel: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int, c: Color),
	#[doc = "Gets the color of the pixel at *(x,y)* in the given *bitmap*. If the coordinate is outside the bounds of the bitmap, or if the bitmap has a mask and the pixel is marked transparent, the function returns ```\nkColorClear```\n; otherwise the return value is ```\nkColorWhite```\n or ```\nkColorBlack```\n.\n"]
	pub getBitmapPixel:
		unsafe extern "C" fn(bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int) -> SolidColor,
	#[doc = "Returns the bitmap table’s image count in the *count* pointer (if not NULL) and number of cells across in the *cellswide* pointer (ditto).\n"]
	pub getBitmapTableInfo:
		unsafe extern "C" fn(table: *mut BitmapTable, count: *mut core::ffi::c_int, width: *mut core::ffi::c_int),
	#[doc = "Draws the text in the given rectangle using the provided options. If no font has been set with [setFont](#f-graphics.setFont), the default system font Asheville Sans 14 Light is used. See the [above note](#f-graphics.drawText) about the ```\nlen```\n argument.\nThe *wrap* argument is one of\nPDTextWrappingModetypedef enum\n{\n\tkWrapClip,\n\tkWrapCharacter,\n\tkWrapWord,\n} PDTextWrappingMode;and *align* is one of\nPDTextAlignmenttypedef enum\n{\n\tkAlignTextLeft,\n\tkAlignTextCenter,\n\tkAlignTextRight\n} PDTextAlignment;"]
	pub drawTextInRect: unsafe extern "C" fn(text: *const core::ffi::c_void,
	                                         len: usize,
	                                         encoding: StringEncoding,
	                                         x: core::ffi::c_int,
	                                         y: core::ffi::c_int,
	                                         width: core::ffi::c_int,
	                                         height: core::ffi::c_int,
	                                         wrap: TextWrappingMode,
	                                         align: TextAlignment),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateGraphics"][::core::mem::size_of::<PlaydateGraphics>() - 256usize];
	["Alignment of PlaydateGraphics"][::core::mem::align_of::<PlaydateGraphics>() - 4usize];
	["Offset of field: PlaydateGraphics::video"][::core::mem::offset_of!(PlaydateGraphics, video) - 0usize];
	["Offset of field: PlaydateGraphics::clear"][::core::mem::offset_of!(PlaydateGraphics, clear) - 4usize];
	["Offset of field: PlaydateGraphics::setBackgroundColor"]
		[::core::mem::offset_of!(PlaydateGraphics, setBackgroundColor) - 8usize];
	["Offset of field: PlaydateGraphics::setStencil"]
		[::core::mem::offset_of!(PlaydateGraphics, setStencil) - 12usize];
	["Offset of field: PlaydateGraphics::setDrawMode"]
		[::core::mem::offset_of!(PlaydateGraphics, setDrawMode) - 16usize];
	["Offset of field: PlaydateGraphics::setDrawOffset"]
		[::core::mem::offset_of!(PlaydateGraphics, setDrawOffset) - 20usize];
	["Offset of field: PlaydateGraphics::setClipRect"]
		[::core::mem::offset_of!(PlaydateGraphics, setClipRect) - 24usize];
	["Offset of field: PlaydateGraphics::clearClipRect"]
		[::core::mem::offset_of!(PlaydateGraphics, clearClipRect) - 28usize];
	["Offset of field: PlaydateGraphics::setLineCapStyle"]
		[::core::mem::offset_of!(PlaydateGraphics, setLineCapStyle) - 32usize];
	["Offset of field: PlaydateGraphics::setFont"][::core::mem::offset_of!(PlaydateGraphics, setFont) - 36usize];
	["Offset of field: PlaydateGraphics::setTextTracking"]
		[::core::mem::offset_of!(PlaydateGraphics, setTextTracking) - 40usize];
	["Offset of field: PlaydateGraphics::pushContext"]
		[::core::mem::offset_of!(PlaydateGraphics, pushContext) - 44usize];
	["Offset of field: PlaydateGraphics::popContext"]
		[::core::mem::offset_of!(PlaydateGraphics, popContext) - 48usize];
	["Offset of field: PlaydateGraphics::drawBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, drawBitmap) - 52usize];
	["Offset of field: PlaydateGraphics::tileBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, tileBitmap) - 56usize];
	["Offset of field: PlaydateGraphics::drawLine"][::core::mem::offset_of!(PlaydateGraphics, drawLine) - 60usize];
	["Offset of field: PlaydateGraphics::fillTriangle"]
		[::core::mem::offset_of!(PlaydateGraphics, fillTriangle) - 64usize];
	["Offset of field: PlaydateGraphics::drawRect"][::core::mem::offset_of!(PlaydateGraphics, drawRect) - 68usize];
	["Offset of field: PlaydateGraphics::fillRect"][::core::mem::offset_of!(PlaydateGraphics, fillRect) - 72usize];
	["Offset of field: PlaydateGraphics::drawEllipse"]
		[::core::mem::offset_of!(PlaydateGraphics, drawEllipse) - 76usize];
	["Offset of field: PlaydateGraphics::fillEllipse"]
		[::core::mem::offset_of!(PlaydateGraphics, fillEllipse) - 80usize];
	["Offset of field: PlaydateGraphics::drawScaledBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, drawScaledBitmap) - 84usize];
	["Offset of field: PlaydateGraphics::drawText"][::core::mem::offset_of!(PlaydateGraphics, drawText) - 88usize];
	["Offset of field: PlaydateGraphics::newBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, newBitmap) - 92usize];
	["Offset of field: PlaydateGraphics::freeBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, freeBitmap) - 96usize];
	["Offset of field: PlaydateGraphics::loadBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, loadBitmap) - 100usize];
	["Offset of field: PlaydateGraphics::copyBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, copyBitmap) - 104usize];
	["Offset of field: PlaydateGraphics::loadIntoBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, loadIntoBitmap) - 108usize];
	["Offset of field: PlaydateGraphics::getBitmapData"]
		[::core::mem::offset_of!(PlaydateGraphics, getBitmapData) - 112usize];
	["Offset of field: PlaydateGraphics::clearBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, clearBitmap) - 116usize];
	["Offset of field: PlaydateGraphics::rotatedBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, rotatedBitmap) - 120usize];
	["Offset of field: PlaydateGraphics::newBitmapTable"]
		[::core::mem::offset_of!(PlaydateGraphics, newBitmapTable) - 124usize];
	["Offset of field: PlaydateGraphics::freeBitmapTable"]
		[::core::mem::offset_of!(PlaydateGraphics, freeBitmapTable) - 128usize];
	["Offset of field: PlaydateGraphics::loadBitmapTable"]
		[::core::mem::offset_of!(PlaydateGraphics, loadBitmapTable) - 132usize];
	["Offset of field: PlaydateGraphics::loadIntoBitmapTable"]
		[::core::mem::offset_of!(PlaydateGraphics, loadIntoBitmapTable) - 136usize];
	["Offset of field: PlaydateGraphics::getTableBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, getTableBitmap) - 140usize];
	["Offset of field: PlaydateGraphics::loadFont"]
		[::core::mem::offset_of!(PlaydateGraphics, loadFont) - 144usize];
	["Offset of field: PlaydateGraphics::getFontPage"]
		[::core::mem::offset_of!(PlaydateGraphics, getFontPage) - 148usize];
	["Offset of field: PlaydateGraphics::getPageGlyph"]
		[::core::mem::offset_of!(PlaydateGraphics, getPageGlyph) - 152usize];
	["Offset of field: PlaydateGraphics::getGlyphKerning"]
		[::core::mem::offset_of!(PlaydateGraphics, getGlyphKerning) - 156usize];
	["Offset of field: PlaydateGraphics::getTextWidth"]
		[::core::mem::offset_of!(PlaydateGraphics, getTextWidth) - 160usize];
	["Offset of field: PlaydateGraphics::getFrame"]
		[::core::mem::offset_of!(PlaydateGraphics, getFrame) - 164usize];
	["Offset of field: PlaydateGraphics::getDisplayFrame"]
		[::core::mem::offset_of!(PlaydateGraphics, getDisplayFrame) - 168usize];
	["Offset of field: PlaydateGraphics::getDebugBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, getDebugBitmap) - 172usize];
	["Offset of field: PlaydateGraphics::copyFrameBufferBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, copyFrameBufferBitmap) - 176usize];
	["Offset of field: PlaydateGraphics::markUpdatedRows"]
		[::core::mem::offset_of!(PlaydateGraphics, markUpdatedRows) - 180usize];
	["Offset of field: PlaydateGraphics::display"][::core::mem::offset_of!(PlaydateGraphics, display) - 184usize];
	["Offset of field: PlaydateGraphics::setColorToPattern"]
		[::core::mem::offset_of!(PlaydateGraphics, setColorToPattern) - 188usize];
	["Offset of field: PlaydateGraphics::checkMaskCollision"]
		[::core::mem::offset_of!(PlaydateGraphics, checkMaskCollision) - 192usize];
	["Offset of field: PlaydateGraphics::setScreenClipRect"]
		[::core::mem::offset_of!(PlaydateGraphics, setScreenClipRect) - 196usize];
	["Offset of field: PlaydateGraphics::fillPolygon"]
		[::core::mem::offset_of!(PlaydateGraphics, fillPolygon) - 200usize];
	["Offset of field: PlaydateGraphics::getFontHeight"]
		[::core::mem::offset_of!(PlaydateGraphics, getFontHeight) - 204usize];
	["Offset of field: PlaydateGraphics::getDisplayBufferBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, getDisplayBufferBitmap) - 208usize];
	["Offset of field: PlaydateGraphics::drawRotatedBitmap"]
		[::core::mem::offset_of!(PlaydateGraphics, drawRotatedBitmap) - 212usize];
	["Offset of field: PlaydateGraphics::setTextLeading"]
		[::core::mem::offset_of!(PlaydateGraphics, setTextLeading) - 216usize];
	["Offset of field: PlaydateGraphics::setBitmapMask"]
		[::core::mem::offset_of!(PlaydateGraphics, setBitmapMask) - 220usize];
	["Offset of field: PlaydateGraphics::getBitmapMask"]
		[::core::mem::offset_of!(PlaydateGraphics, getBitmapMask) - 224usize];
	["Offset of field: PlaydateGraphics::setStencilImage"]
		[::core::mem::offset_of!(PlaydateGraphics, setStencilImage) - 228usize];
	["Offset of field: PlaydateGraphics::makeFontFromData"]
		[::core::mem::offset_of!(PlaydateGraphics, makeFontFromData) - 232usize];
	["Offset of field: PlaydateGraphics::getTextTracking"]
		[::core::mem::offset_of!(PlaydateGraphics, getTextTracking) - 236usize];
	["Offset of field: PlaydateGraphics::setPixel"]
		[::core::mem::offset_of!(PlaydateGraphics, setPixel) - 240usize];
	["Offset of field: PlaydateGraphics::getBitmapPixel"]
		[::core::mem::offset_of!(PlaydateGraphics, getBitmapPixel) - 244usize];
	["Offset of field: PlaydateGraphics::getBitmapTableInfo"]
		[::core::mem::offset_of!(PlaydateGraphics, getBitmapTableInfo) - 248usize];
	["Offset of field: PlaydateGraphics::drawTextInRect"]
		[::core::mem::offset_of!(PlaydateGraphics, drawTextInRect) - 252usize];
};
pub type va_list = u32;
impl Buttons {
	pub const Left: Buttons = Buttons(1);
}
impl Buttons {
	pub const Right: Buttons = Buttons(2);
}
impl Buttons {
	pub const Up: Buttons = Buttons(4);
}
impl Buttons {
	pub const Down: Buttons = Buttons(8);
}
impl Buttons {
	pub const B: Buttons = Buttons(16);
}
impl Buttons {
	pub const A: Buttons = Buttons(32);
}
impl ::core::ops::BitOr<Buttons> for Buttons {
	type Output = Self;
	#[inline]
	fn bitor(self, other: Self) -> Self { Buttons(self.0 | other.0) }
}
impl ::core::ops::BitOrAssign for Buttons {
	#[inline]
	fn bitor_assign(&mut self, rhs: Buttons) { self.0 |= rhs.0; }
}
impl ::core::ops::BitAnd<Buttons> for Buttons {
	type Output = Self;
	#[inline]
	fn bitand(self, other: Self) -> Self { Buttons(self.0 & other.0) }
}
impl ::core::ops::BitAndAssign for Buttons {
	#[inline]
	fn bitand_assign(&mut self, rhs: Buttons) { self.0 &= rhs.0; }
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Buttons(pub u8);
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Language {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct DateTime {
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub weekday: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of DateTime"][::core::mem::size_of::<DateTime>() - 8usize];
	["Alignment of DateTime"][::core::mem::align_of::<DateTime>() - 2usize];
	["Offset of field: DateTime::year"][::core::mem::offset_of!(DateTime, year) - 0usize];
	["Offset of field: DateTime::month"][::core::mem::offset_of!(DateTime, month) - 2usize];
	["Offset of field: DateTime::day"][::core::mem::offset_of!(DateTime, day) - 3usize];
	["Offset of field: DateTime::weekday"][::core::mem::offset_of!(DateTime, weekday) - 4usize];
	["Offset of field: DateTime::hour"][::core::mem::offset_of!(DateTime, hour) - 5usize];
	["Offset of field: DateTime::minute"][::core::mem::offset_of!(DateTime, minute) - 6usize];
	["Offset of field: DateTime::second"][::core::mem::offset_of!(DateTime, second) - 7usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct MenuItem {
	_unused: [u8; 0],
}
#[repr(u16)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Peripherals {
	None = 0,
	Accelerometer = 1,
	AllPeripherals = 65535,
}
pub type CallbackFunction =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void) -> core::ffi::c_int>;
pub type MenuItemCallbackFunction =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
pub type ButtonCallbackFunction = ::core::option::Option<unsafe extern "C" fn(button: Buttons,
                                                                              down: core::ffi::c_int,
                                                                              when: u32,
                                                                              userdata: *mut core::ffi::c_void)
                                                                              -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSys { # [doc = "Allocates heap space if *ptr* is NULL, else reallocates the given pointer. If *size* is zero, frees the given pointer.\n"] pub realloc : unsafe extern "C" fn (ptr : * mut core :: ffi :: c_void , size : usize) -> * mut core :: ffi :: c_void , # [doc = "Creates a formatted string and returns it via the *outstring* argument. The arguments and return value match libc’s ```\nasprintf()```\n: the format string is standard ```\nprintf()```\n style, the string returned in *outstring* should be freed by the caller when it’s no longer in use, and the return value is the length of the formatted string.\n"] pub formatString : unsafe extern "C" fn (ret : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int , # [doc = "Calls the log function.\nEquivalent to [```\nprint()```\n](<./Inside Playdate.html#f-print>) in the Lua API.\n"] pub logToConsole : unsafe extern "C" fn (fmt : * const core :: ffi :: c_char , ...) , # [doc = "Calls the log function, outputting an error in red to the console, then pauses execution.\n"] pub error : unsafe extern "C" fn (fmt : * const core :: ffi :: c_char , ...) -> ! , # [doc = "Returns the current language of the system.\n"] pub getLanguage : unsafe extern "C" fn () -> Language , # [doc = "Returns the number of milliseconds since…\u{200b}some arbitrary point in time. This should present a consistent timebase while a game is running, but the counter will be disabled when the device is sleeping.\n"] pub getCurrentTimeMilliseconds : unsafe extern "C" fn () -> core :: ffi :: c_uint , # [doc = "Returns the number of seconds (and sets *milliseconds* if not NULL) elapsed since midnight (hour 0), January 1, 2000.\n"] pub getSecondsSinceEpoch : unsafe extern "C" fn (milliseconds : * mut core :: ffi :: c_uint) -> core :: ffi :: c_uint , # [doc = "Calculates the current frames per second and draws that value at *x, y*.\n"] pub drawFPS : unsafe extern "C" fn (x : core :: ffi :: c_int , y : core :: ffi :: c_int) , # [doc = "PDCallbackFunctionint PDCallbackFunction(void* userdata);Replaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed.\n"] pub setUpdateCallback : unsafe extern "C" fn (update : CallbackFunction , userdata : * mut core :: ffi :: c_void) , # [doc = "Sets the value pointed to by *current* to a bitmask indicating which buttons are currently down. *pushed* and *released* reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.\nPDButtonkButtonLeft\nkButtonRight\nkButtonUp\nkButtonDown\nkButtonB\nkButtonA"] pub getButtonState : unsafe extern "C" fn (current : * mut Buttons , pushed : * mut Buttons , released : * mut Buttons) , # [doc = "By default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.\nPDPeripheralskNone\nkAccelerometer"] pub setPeripheralsEnabled : unsafe extern "C" fn (mask : Peripherals) , # [doc = "Returns the last-read accelerometer data.\n"] pub getAccelerometer : unsafe extern "C" fn (outx : * mut core :: ffi :: c_float , outy : * mut core :: ffi :: c_float , outz : * mut core :: ffi :: c_float) , # [doc = "Returns the angle change of the crank since the last time this function was called. Negative values are anti-clockwise.\n"] pub getCrankChange : unsafe extern "C" fn () -> core :: ffi :: c_float , # [doc = "Returns the current position of the crank, in the range 0-360. Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.\n"] pub getCrankAngle : unsafe extern "C" fn () -> core :: ffi :: c_float , # [doc = "Returns 1 or 0 indicating whether or not the crank is folded into the unit.\n"] pub isCrankDocked : unsafe extern "C" fn () -> core :: ffi :: c_int , # [doc = "The function returns the previous value for this setting.\n"] pub setCrankSoundsDisabled : unsafe extern "C" fn (flag : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Returns 1 if the global \"flipped\" system setting is set, otherwise 0.\n"] pub getFlipped : unsafe extern "C" fn () -> core :: ffi :: c_int , # [doc = "Disables or enables the 3 minute auto lock feature. When called, the timer is reset to 3 minutes.\n"] pub setAutoLockDisabled : unsafe extern "C" fn (disable : core :: ffi :: c_int) , # [doc = "A game can optionally provide an image to be displayed alongside the system menu. *bitmap* must be a 400x240 LCDBitmap. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.\nOptionally, a non-zero *xoffset*, can be provided. This must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by xoffset pixels as the menu is animated in.\nThis function could be called in response to the kEventPause *event* in your implementation of [eventHandler()](#_eventHandler).\n"] pub setMenuImage : unsafe extern "C" fn (bitmap : * mut Bitmap , xOffset : core :: ffi :: c_int) , # [doc = "*title* will be the title displayed by the menu item.\nAdds a new menu item to the System Menu. When invoked by the user, this menu item will:\n1. Invoke your *callback* function.\n2. Hide the System Menu.\n3. Unpause your game and call [eventHandler()](#_eventHandler) with the kEventResume *event*.\nYour game can then present an options interface to the player, or take other action, in whatever manner you choose.\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually.\n"] pub addMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , callback : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem , # [doc = "Adds a new menu item that can be checked or unchecked by the player.\n*title* will be the title displayed by the menu item.\n*value* should be 0 for unchecked, 1 for checked.\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually.\n"] pub addCheckmarkMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , value : core :: ffi :: c_int , callback : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem , # [doc = "Adds a new menu item that allows the player to cycle through a set of options.\n*title* will be the title displayed by the menu item.\n*options* should be an array of strings representing the states this menu item can cycle through. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.\n*optionsCount* should be the number of items contained in *options*.\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually.\n"] pub addOptionsMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , optionTitles : * mut * const core :: ffi :: c_char , optionsCount : core :: ffi :: c_int , f : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem , # [doc = "Removes all custom menu items from the system menu.\n"] pub removeAllMenuItems : unsafe extern "C" fn () , # [doc = "Removes the menu item from the system menu.\n"] pub removeMenuItem : unsafe extern "C" fn (menuItem : * mut MenuItem) , pub getMenuItemValue : unsafe extern "C" fn (menuItem : * mut MenuItem) -> core :: ffi :: c_int , # [doc = "Gets or sets the integer value of the menu item.\nFor checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.\n"] pub setMenuItemValue : unsafe extern "C" fn (menuItem : * mut MenuItem , value : core :: ffi :: c_int) , pub getMenuItemTitle : unsafe extern "C" fn (menuItem : * mut MenuItem) -> * const core :: ffi :: c_char , # [doc = "Gets or sets the display title of the menu item.\n"] pub setMenuItemTitle : unsafe extern "C" fn (menuItem : * mut MenuItem , title : * const core :: ffi :: c_char) , pub getMenuItemUserdata : unsafe extern "C" fn (menuItem : * mut MenuItem) -> * mut core :: ffi :: c_void , # [doc = "Gets or sets the userdata value associated with this menu item.\n"] pub setMenuItemUserdata : unsafe extern "C" fn (menuItem : * mut MenuItem , ud : * mut core :: ffi :: c_void) , # [doc = "Returns 1 if the global \"reduce flashing\" system setting is set, otherwise 0.\n"] pub getReduceFlashing : unsafe extern "C" fn () -> core :: ffi :: c_int , # [doc = "Returns the number of seconds since ```\nplaydate.resetElapsedTime()```\n was called. The value is a floating-point number with microsecond accuracy.\n"] pub getElapsedTime : unsafe extern "C" fn () -> core :: ffi :: c_float , # [doc = "Resets the high-resolution timer.\n"] pub resetElapsedTime : unsafe extern "C" fn () , # [doc = "Returns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full.\n"] pub getBatteryPercentage : unsafe extern "C" fn () -> core :: ffi :: c_float , # [doc = "Returns the battery’s current voltage level.\n"] pub getBatteryVoltage : unsafe extern "C" fn () -> core :: ffi :: c_float , # [doc = "Returns the system timezone offset from GMT, in seconds.\n"] pub getTimezoneOffset : unsafe extern "C" fn () -> i32 , # [doc = "Returns 1 if the user has set the 24-Hour Time preference in the Settings program.\n"] pub shouldDisplay24HourTime : unsafe extern "C" fn () -> core :: ffi :: c_int , # [doc = "Converts the given epoch time to a PDDateTime.\n"] pub convertEpochToDateTime : unsafe extern "C" fn (epoch : u32 , datetime : * mut DateTime) , # [doc = "Converts the given PDDateTime to an epoch time.\n"] pub convertDateTimeToEpoch : unsafe extern "C" fn (datetime : * mut DateTime) -> u32 , # [doc = "Flush the CPU instruction cache, on the very unlikely chance you’re modifying instruction code on the fly. (If you don’t know what I’m talking about, you don’t need this. :smile:)\n"] pub clearICache : unsafe extern "C" fn () , # [doc = "As an alternative to polling for button presses using ```\ngetButtonState()```\n, this function allows a callback function to be set. The function is called for each button up/down event (possibly multiple events on the same button) that occurred during the previous update cycle. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. The function should return 0 on success or a non-zero value to signal an error.\nPDButtonCallbackFunctiontypedef int PDButtonCallbackFunction(PDButtons button, int down, uint32_t when, void* userdata);"] pub setButtonCallback : unsafe extern "C" fn (cb : ButtonCallbackFunction , buttonud : * mut core :: ffi :: c_void , queuesize : core :: ffi :: c_int) , # [doc = "Provides a callback to receive messages sent to the device over the serial port using the ```\nmsg```\n command. If no device is connected, you can send these messages to a game in the simulator by entering ```\n!msg &lt;message&gt;```\n in the Lua console.\n"] pub setSerialMessageCallback : unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (data : * const core :: ffi :: c_char) >) , # [doc = "Allocates and formats a string using a variadic ```\nva_list```\n argument, in the style of ```\nvasprintf()```\n. The string returned via *ret* should be freed by the caller when it is no longer in use. The return value from the function is the length of the formatted string.\n"] pub vaFormatString : unsafe extern "C" fn (outstr : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , args : va_list) -> core :: ffi :: c_int , # [doc = "Like libc ```\nsscanf()```\n, parses a string according to a format string and places the values into pointers passed in after the format. The return value is the number of items matched.\n"] pub parseString : unsafe extern "C" fn (str_ : * const core :: ffi :: c_char , format : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSys"][::core::mem::size_of::<PlaydateSys>() - 176usize];
	["Alignment of PlaydateSys"][::core::mem::align_of::<PlaydateSys>() - 4usize];
	["Offset of field: PlaydateSys::realloc"][::core::mem::offset_of!(PlaydateSys, realloc) - 0usize];
	["Offset of field: PlaydateSys::formatString"][::core::mem::offset_of!(PlaydateSys, formatString) - 4usize];
	["Offset of field: PlaydateSys::logToConsole"][::core::mem::offset_of!(PlaydateSys, logToConsole) - 8usize];
	["Offset of field: PlaydateSys::error"][::core::mem::offset_of!(PlaydateSys, error) - 12usize];
	["Offset of field: PlaydateSys::getLanguage"][::core::mem::offset_of!(PlaydateSys, getLanguage) - 16usize];
	["Offset of field: PlaydateSys::getCurrentTimeMilliseconds"]
		[::core::mem::offset_of!(PlaydateSys, getCurrentTimeMilliseconds) - 20usize];
	["Offset of field: PlaydateSys::getSecondsSinceEpoch"]
		[::core::mem::offset_of!(PlaydateSys, getSecondsSinceEpoch) - 24usize];
	["Offset of field: PlaydateSys::drawFPS"][::core::mem::offset_of!(PlaydateSys, drawFPS) - 28usize];
	["Offset of field: PlaydateSys::setUpdateCallback"]
		[::core::mem::offset_of!(PlaydateSys, setUpdateCallback) - 32usize];
	["Offset of field: PlaydateSys::getButtonState"]
		[::core::mem::offset_of!(PlaydateSys, getButtonState) - 36usize];
	["Offset of field: PlaydateSys::setPeripheralsEnabled"]
		[::core::mem::offset_of!(PlaydateSys, setPeripheralsEnabled) - 40usize];
	["Offset of field: PlaydateSys::getAccelerometer"]
		[::core::mem::offset_of!(PlaydateSys, getAccelerometer) - 44usize];
	["Offset of field: PlaydateSys::getCrankChange"]
		[::core::mem::offset_of!(PlaydateSys, getCrankChange) - 48usize];
	["Offset of field: PlaydateSys::getCrankAngle"][::core::mem::offset_of!(PlaydateSys, getCrankAngle) - 52usize];
	["Offset of field: PlaydateSys::isCrankDocked"][::core::mem::offset_of!(PlaydateSys, isCrankDocked) - 56usize];
	["Offset of field: PlaydateSys::setCrankSoundsDisabled"]
		[::core::mem::offset_of!(PlaydateSys, setCrankSoundsDisabled) - 60usize];
	["Offset of field: PlaydateSys::getFlipped"][::core::mem::offset_of!(PlaydateSys, getFlipped) - 64usize];
	["Offset of field: PlaydateSys::setAutoLockDisabled"]
		[::core::mem::offset_of!(PlaydateSys, setAutoLockDisabled) - 68usize];
	["Offset of field: PlaydateSys::setMenuImage"][::core::mem::offset_of!(PlaydateSys, setMenuImage) - 72usize];
	["Offset of field: PlaydateSys::addMenuItem"][::core::mem::offset_of!(PlaydateSys, addMenuItem) - 76usize];
	["Offset of field: PlaydateSys::addCheckmarkMenuItem"]
		[::core::mem::offset_of!(PlaydateSys, addCheckmarkMenuItem) - 80usize];
	["Offset of field: PlaydateSys::addOptionsMenuItem"]
		[::core::mem::offset_of!(PlaydateSys, addOptionsMenuItem) - 84usize];
	["Offset of field: PlaydateSys::removeAllMenuItems"]
		[::core::mem::offset_of!(PlaydateSys, removeAllMenuItems) - 88usize];
	["Offset of field: PlaydateSys::removeMenuItem"]
		[::core::mem::offset_of!(PlaydateSys, removeMenuItem) - 92usize];
	["Offset of field: PlaydateSys::getMenuItemValue"]
		[::core::mem::offset_of!(PlaydateSys, getMenuItemValue) - 96usize];
	["Offset of field: PlaydateSys::setMenuItemValue"]
		[::core::mem::offset_of!(PlaydateSys, setMenuItemValue) - 100usize];
	["Offset of field: PlaydateSys::getMenuItemTitle"]
		[::core::mem::offset_of!(PlaydateSys, getMenuItemTitle) - 104usize];
	["Offset of field: PlaydateSys::setMenuItemTitle"]
		[::core::mem::offset_of!(PlaydateSys, setMenuItemTitle) - 108usize];
	["Offset of field: PlaydateSys::getMenuItemUserdata"]
		[::core::mem::offset_of!(PlaydateSys, getMenuItemUserdata) - 112usize];
	["Offset of field: PlaydateSys::setMenuItemUserdata"]
		[::core::mem::offset_of!(PlaydateSys, setMenuItemUserdata) - 116usize];
	["Offset of field: PlaydateSys::getReduceFlashing"]
		[::core::mem::offset_of!(PlaydateSys, getReduceFlashing) - 120usize];
	["Offset of field: PlaydateSys::getElapsedTime"]
		[::core::mem::offset_of!(PlaydateSys, getElapsedTime) - 124usize];
	["Offset of field: PlaydateSys::resetElapsedTime"]
		[::core::mem::offset_of!(PlaydateSys, resetElapsedTime) - 128usize];
	["Offset of field: PlaydateSys::getBatteryPercentage"]
		[::core::mem::offset_of!(PlaydateSys, getBatteryPercentage) - 132usize];
	["Offset of field: PlaydateSys::getBatteryVoltage"]
		[::core::mem::offset_of!(PlaydateSys, getBatteryVoltage) - 136usize];
	["Offset of field: PlaydateSys::getTimezoneOffset"]
		[::core::mem::offset_of!(PlaydateSys, getTimezoneOffset) - 140usize];
	["Offset of field: PlaydateSys::shouldDisplay24HourTime"]
		[::core::mem::offset_of!(PlaydateSys, shouldDisplay24HourTime) - 144usize];
	["Offset of field: PlaydateSys::convertEpochToDateTime"]
		[::core::mem::offset_of!(PlaydateSys, convertEpochToDateTime) - 148usize];
	["Offset of field: PlaydateSys::convertDateTimeToEpoch"]
		[::core::mem::offset_of!(PlaydateSys, convertDateTimeToEpoch) - 152usize];
	["Offset of field: PlaydateSys::clearICache"][::core::mem::offset_of!(PlaydateSys, clearICache) - 156usize];
	["Offset of field: PlaydateSys::setButtonCallback"]
		[::core::mem::offset_of!(PlaydateSys, setButtonCallback) - 160usize];
	["Offset of field: PlaydateSys::setSerialMessageCallback"]
		[::core::mem::offset_of!(PlaydateSys, setSerialMessageCallback) - 164usize];
	["Offset of field: PlaydateSys::vaFormatString"]
		[::core::mem::offset_of!(PlaydateSys, vaFormatString) - 168usize];
	["Offset of field: PlaydateSys::parseString"][::core::mem::offset_of!(PlaydateSys, parseString) - 172usize];
};
pub type LuaState = *mut core::ffi::c_void;
pub type LuaCFunction = ::core::option::Option<unsafe extern "C" fn(L: *mut LuaState) -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LuaUdObject {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Sprite {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LuaValueType {
	Int = 0,
	Float = 1,
	Str = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct LuaReg {
	pub name: *const core::ffi::c_char,
	pub func: LuaCFunction,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of LuaReg"][::core::mem::size_of::<LuaReg>() - 8usize];
	["Alignment of LuaReg"][::core::mem::align_of::<LuaReg>() - 4usize];
	["Offset of field: LuaReg::name"][::core::mem::offset_of!(LuaReg, name) - 0usize];
	["Offset of field: LuaReg::func"][::core::mem::offset_of!(LuaReg, func) - 4usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LuaType {
	Nil = 0,
	Bool = 1,
	Int = 2,
	Float = 3,
	String = 4,
	Table = 5,
	Function = 6,
	Thread = 7,
	Object = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub struct LuaVal {
	pub name: *const core::ffi::c_char,
	pub type_: LuaValueType,
	pub v: LuaValBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub union LuaValBindgenTy1 {
	pub intval: core::ffi::c_uint,
	pub floatval: core::ffi::c_float,
	pub strval: *const core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of LuaValBindgenTy1"][::core::mem::size_of::<LuaValBindgenTy1>() - 4usize];
	["Alignment of LuaValBindgenTy1"][::core::mem::align_of::<LuaValBindgenTy1>() - 4usize];
	["Offset of field: LuaValBindgenTy1::intval"][::core::mem::offset_of!(LuaValBindgenTy1, intval) - 0usize];
	["Offset of field: LuaValBindgenTy1::floatval"][::core::mem::offset_of!(LuaValBindgenTy1, floatval) - 0usize];
	["Offset of field: LuaValBindgenTy1::strval"][::core::mem::offset_of!(LuaValBindgenTy1, strval) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of LuaVal"][::core::mem::size_of::<LuaVal>() - 12usize];
	["Alignment of LuaVal"][::core::mem::align_of::<LuaVal>() - 4usize];
	["Offset of field: LuaVal::name"][::core::mem::offset_of!(LuaVal, name) - 0usize];
	["Offset of field: LuaVal::type_"][::core::mem::offset_of!(LuaVal, type_) - 4usize];
	["Offset of field: LuaVal::v"][::core::mem::offset_of!(LuaVal, v) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateLua {
	#[doc = "Adds the Lua function *f* to the Lua runtime, with name *name*. (*name* can be a table path using dots, e.g. if name = “mycode.myDrawingFunction” adds the function “myDrawingFunction” to the global table “myCode”.) Returns 1 on success or 0 with an error message in *outErr*.\n"]
	pub addFunction: unsafe extern "C" fn(f: LuaCFunction,
	                                      name: *const core::ffi::c_char,
	                                      outErr: *mut *const core::ffi::c_char)
	                                      -> core::ffi::c_int,
	#[doc = "Creates a new \"class\" (i.e., a Lua metatable containing functions) with the given name and adds the given functions and constants to it. If the table is simply a list of functions that won’t be used as a metatable, *isstatic* should be set to 1 to create a plain table instead of a metatable. Please see ```\nC_API/Examples/Array```\n for an example of how to use ```\nregisterClass```\n to create a Lua table-like object from C.\n"]
	pub registerClass: unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                        reg: *const LuaReg,
	                                        vals: *const LuaVal,
	                                        isstatic: core::ffi::c_int,
	                                        outErr: *mut *const core::ffi::c_char)
	                                        -> core::ffi::c_int,
	#[doc = "Pushes a [lua_CFunction](#f-lua.cFunction) onto the stack.\n"]
	pub pushFunction: unsafe extern "C" fn(f: LuaCFunction),
	#[doc = "If a class includes an ```\n__index```\n function, it should call this first to check if the indexed variable exists in the metatable. If the indexMetatable() call returns 1, it has located the variable and put it on the stack, and the ```\n__index```\n function should return 1 to indicate a value was found. If indexMetatable() doesn’t find a value, the ```\n__index```\n function can then do its custom getter magic.\n"]
	pub indexMetatable: unsafe extern "C" fn() -> core::ffi::c_int,
	#[doc = "Stops the run loop.\n"]
	pub stop: unsafe extern "C" fn(),
	#[doc = "Starts the run loop back up.\n"]
	pub start: unsafe extern "C" fn(),
	#[doc = "Returns the number of arguments passed to the function.\n"]
	pub getArgCount: unsafe extern "C" fn() -> core::ffi::c_int,
	#[doc = "Returns the type of the variable at stack position *pos*. If the type is *kTypeObject* and *outClass* is non-NULL, it returns the name of the object’s metatable.\n"]
	pub getArgType:
		unsafe extern "C" fn(pos: core::ffi::c_int, outClass: *mut *const core::ffi::c_char) -> LuaType,
	#[doc = "Returns 1 if the argument at the given position *pos* is nil.\n"]
	pub argIsNil: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Returns one if the argument at position *pos* is true, zero if not.\n"]
	pub getArgBool: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Returns the argument at position *pos* as an int.\n"]
	pub getArgInt: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Returns the argument at position *pos* as a float.\n"]
	pub getArgFloat: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_float,
	#[doc = "Returns the argument at position *pos* as a string.\n"]
	pub getArgString: unsafe extern "C" fn(pos: core::ffi::c_int) -> *const core::ffi::c_char,
	#[doc = "Returns the argument at position *pos* as a string and sets *outlen* to its length.\n"]
	pub getArgBytes: unsafe extern "C" fn(pos: core::ffi::c_int, outlen: *mut usize) -> *const core::ffi::c_char,
	#[doc = "Checks the object type of the argument at position *pos* and returns a pointer to it if it’s the correct type. Optionally sets *outud* to a pointer to the opaque LuaUDObject for the given stack.\n"]
	pub getArgObject: unsafe extern "C" fn(pos: core::ffi::c_int,
	                                       type_: *mut core::ffi::c_char,
	                                       outud: *mut *mut LuaUdObject)
	                                       -> *mut core::ffi::c_void,
	#[doc = "Returns the argument at position *pos* as an LCDBitmap.\n"]
	pub getBitmap: unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut Bitmap,
	#[doc = "Returns the argument at position *pos* as an LCDSprite.\n"]
	pub getSprite: unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut Sprite,
	#[doc = "Pushes nil onto the stack.\n"]
	pub pushNil: unsafe extern "C" fn(),
	#[doc = "Pushes the int *val* onto the stack.\n"]
	pub pushBool: unsafe extern "C" fn(val: core::ffi::c_int),
	#[doc = "Pushes the int *val* onto the stack.\n"]
	pub pushInt: unsafe extern "C" fn(val: core::ffi::c_int),
	#[doc = "Pushes the float *val* onto the stack.\n"]
	pub pushFloat: unsafe extern "C" fn(val: core::ffi::c_float),
	#[doc = "Pushes the string *str* onto the stack.\n"]
	pub pushString: unsafe extern "C" fn(str_: *const core::ffi::c_char),
	#[doc = "Like *pushString()*, but pushes an arbitrary byte array to the stack, ignoring \\0 characters.\n"]
	pub pushBytes: unsafe extern "C" fn(str_: *const core::ffi::c_char, len: usize),
	#[doc = "Pushes the LCDBitmap *bitmap* onto the stack.\n"]
	pub pushBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap),
	#[doc = "Pushes the LCDSprite *sprite* onto the stack.\n"]
	pub pushSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Pushes the given custom object *obj* onto the stack and returns a pointer to the opaque LuaUDObject. *type* must match the class name used in [playdate-&gt;lua-&gt;registerClass()](#f-lua.registerClass). *nValues* is the number of slots to allocate for Lua values (see [set/getObjectValue()](#f-lua.setObjectValue)).\n"]
	pub pushObject: unsafe extern "C" fn(obj: *mut core::ffi::c_void,
	                                     type_: *mut core::ffi::c_char,
	                                     nValues: core::ffi::c_int)
	                                     -> *mut LuaUdObject,
	#[doc = "Retains the opaque LuaUDObject *obj* and returns same.\n"]
	pub retainObject: unsafe extern "C" fn(obj: *mut LuaUdObject) -> *mut LuaUdObject,
	#[doc = "Releases the opaque LuaUDObject *obj*.\n"]
	pub releaseObject: unsafe extern "C" fn(obj: *mut LuaUdObject),
	#[doc = "Sets the value of object *obj*'s uservalue slot number *slot* (starting at 1, not zero) to the value at the top of the stack.\n"]
	pub setUserValue: unsafe extern "C" fn(obj: *mut LuaUdObject, slot: core::ffi::c_uint),
	#[doc = "Copies the value at *obj*'s given uservalue *slot* to the top of the stack and returns its stack position.\n"]
	pub getUserValue: unsafe extern "C" fn(obj: *mut LuaUdObject, slot: core::ffi::c_uint) -> core::ffi::c_int,
	pub callFunction_deprecated: unsafe extern "C" fn(name: *const core::ffi::c_char, nargs: core::ffi::c_int),
	#[doc = "Calls the Lua function *name* and and indicates that *nargs* number of arguments have already been pushed to the stack for the function to use. *name* can be a table path using dots, e.g. “playdate.apiVersion”. Returns 1 on success; on failure, returns 0 and puts an error message into the ```\nouterr```\n pointer, if it’s set. Calling Lua from C is slow, so use sparingly.\n"]
	pub callFunction: unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                       nargs: core::ffi::c_int,
	                                       outerr: *mut *const core::ffi::c_char)
	                                       -> core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateLua"][::core::mem::size_of::<PlaydateLua>() - 128usize];
	["Alignment of PlaydateLua"][::core::mem::align_of::<PlaydateLua>() - 4usize];
	["Offset of field: PlaydateLua::addFunction"][::core::mem::offset_of!(PlaydateLua, addFunction) - 0usize];
	["Offset of field: PlaydateLua::registerClass"][::core::mem::offset_of!(PlaydateLua, registerClass) - 4usize];
	["Offset of field: PlaydateLua::pushFunction"][::core::mem::offset_of!(PlaydateLua, pushFunction) - 8usize];
	["Offset of field: PlaydateLua::indexMetatable"]
		[::core::mem::offset_of!(PlaydateLua, indexMetatable) - 12usize];
	["Offset of field: PlaydateLua::stop"][::core::mem::offset_of!(PlaydateLua, stop) - 16usize];
	["Offset of field: PlaydateLua::start"][::core::mem::offset_of!(PlaydateLua, start) - 20usize];
	["Offset of field: PlaydateLua::getArgCount"][::core::mem::offset_of!(PlaydateLua, getArgCount) - 24usize];
	["Offset of field: PlaydateLua::getArgType"][::core::mem::offset_of!(PlaydateLua, getArgType) - 28usize];
	["Offset of field: PlaydateLua::argIsNil"][::core::mem::offset_of!(PlaydateLua, argIsNil) - 32usize];
	["Offset of field: PlaydateLua::getArgBool"][::core::mem::offset_of!(PlaydateLua, getArgBool) - 36usize];
	["Offset of field: PlaydateLua::getArgInt"][::core::mem::offset_of!(PlaydateLua, getArgInt) - 40usize];
	["Offset of field: PlaydateLua::getArgFloat"][::core::mem::offset_of!(PlaydateLua, getArgFloat) - 44usize];
	["Offset of field: PlaydateLua::getArgString"][::core::mem::offset_of!(PlaydateLua, getArgString) - 48usize];
	["Offset of field: PlaydateLua::getArgBytes"][::core::mem::offset_of!(PlaydateLua, getArgBytes) - 52usize];
	["Offset of field: PlaydateLua::getArgObject"][::core::mem::offset_of!(PlaydateLua, getArgObject) - 56usize];
	["Offset of field: PlaydateLua::getBitmap"][::core::mem::offset_of!(PlaydateLua, getBitmap) - 60usize];
	["Offset of field: PlaydateLua::getSprite"][::core::mem::offset_of!(PlaydateLua, getSprite) - 64usize];
	["Offset of field: PlaydateLua::pushNil"][::core::mem::offset_of!(PlaydateLua, pushNil) - 68usize];
	["Offset of field: PlaydateLua::pushBool"][::core::mem::offset_of!(PlaydateLua, pushBool) - 72usize];
	["Offset of field: PlaydateLua::pushInt"][::core::mem::offset_of!(PlaydateLua, pushInt) - 76usize];
	["Offset of field: PlaydateLua::pushFloat"][::core::mem::offset_of!(PlaydateLua, pushFloat) - 80usize];
	["Offset of field: PlaydateLua::pushString"][::core::mem::offset_of!(PlaydateLua, pushString) - 84usize];
	["Offset of field: PlaydateLua::pushBytes"][::core::mem::offset_of!(PlaydateLua, pushBytes) - 88usize];
	["Offset of field: PlaydateLua::pushBitmap"][::core::mem::offset_of!(PlaydateLua, pushBitmap) - 92usize];
	["Offset of field: PlaydateLua::pushSprite"][::core::mem::offset_of!(PlaydateLua, pushSprite) - 96usize];
	["Offset of field: PlaydateLua::pushObject"][::core::mem::offset_of!(PlaydateLua, pushObject) - 100usize];
	["Offset of field: PlaydateLua::retainObject"][::core::mem::offset_of!(PlaydateLua, retainObject) - 104usize];
	["Offset of field: PlaydateLua::releaseObject"]
		[::core::mem::offset_of!(PlaydateLua, releaseObject) - 108usize];
	["Offset of field: PlaydateLua::setUserValue"][::core::mem::offset_of!(PlaydateLua, setUserValue) - 112usize];
	["Offset of field: PlaydateLua::getUserValue"][::core::mem::offset_of!(PlaydateLua, getUserValue) - 116usize];
	["Offset of field: PlaydateLua::callFunction_deprecated"]
		[::core::mem::offset_of!(PlaydateLua, callFunction_deprecated) - 120usize];
	["Offset of field: PlaydateLua::callFunction"][::core::mem::offset_of!(PlaydateLua, callFunction) - 124usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum JsonValueType {
	Null = 0,
	True = 1,
	False = 2,
	Integer = 3,
	Float = 4,
	String = 5,
	Array = 6,
	Table = 7,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub struct JsonValue {
	pub type_: core::ffi::c_char,
	pub data: JsonValueBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub union JsonValueBindgenTy1 {
	pub intval: core::ffi::c_int,
	pub floatval: core::ffi::c_float,
	pub stringval: *mut core::ffi::c_char,
	pub arrayval: *mut core::ffi::c_void,
	pub tableval: *mut core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of JsonValueBindgenTy1"][::core::mem::size_of::<JsonValueBindgenTy1>() - 4usize];
	["Alignment of JsonValueBindgenTy1"][::core::mem::align_of::<JsonValueBindgenTy1>() - 4usize];
	["Offset of field: JsonValueBindgenTy1::intval"]
		[::core::mem::offset_of!(JsonValueBindgenTy1, intval) - 0usize];
	["Offset of field: JsonValueBindgenTy1::floatval"]
		[::core::mem::offset_of!(JsonValueBindgenTy1, floatval) - 0usize];
	["Offset of field: JsonValueBindgenTy1::stringval"]
		[::core::mem::offset_of!(JsonValueBindgenTy1, stringval) - 0usize];
	["Offset of field: JsonValueBindgenTy1::arrayval"]
		[::core::mem::offset_of!(JsonValueBindgenTy1, arrayval) - 0usize];
	["Offset of field: JsonValueBindgenTy1::tableval"]
		[::core::mem::offset_of!(JsonValueBindgenTy1, tableval) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of JsonValue"][::core::mem::size_of::<JsonValue>() - 8usize];
	["Alignment of JsonValue"][::core::mem::align_of::<JsonValue>() - 4usize];
	["Offset of field: JsonValue::type_"][::core::mem::offset_of!(JsonValue, type_) - 0usize];
	["Offset of field: JsonValue::data"][::core::mem::offset_of!(JsonValue, data) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct JsonDecoder {
	pub decodeError: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                             error: *const core::ffi::c_char,
	                                                             linenum: core::ffi::c_int)>,
	pub willDecodeSublist: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                                   name: *const core::ffi::c_char,
	                                                                   type_: JsonValueType)>,
	pub shouldDecodeTableValueForKey: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                                              key: *const core::ffi::c_char)
	                                                                              -> core::ffi::c_int>,
	pub didDecodeTableValue: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                                     key: *const core::ffi::c_char,
	                                                                     value: JsonValue)>,
	pub shouldDecodeArrayValueAtIndex: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                                               pos: core::ffi::c_int)
	                                                                               -> core::ffi::c_int>,
	pub didDecodeArrayValue: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                                     pos: core::ffi::c_int,
	                                                                     value: JsonValue)>,
	pub didDecodeSublist: ::core::option::Option<unsafe extern "C" fn(decoder: *mut JsonDecoder,
	                                                                  name: *const core::ffi::c_char,
	                                                                  type_: JsonValueType)
	                                                                  -> *mut core::ffi::c_void>,
	pub userdata: *mut core::ffi::c_void,
	pub returnString: core::ffi::c_int,
	pub path: *const core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of JsonDecoder"][::core::mem::size_of::<JsonDecoder>() - 40usize];
	["Alignment of JsonDecoder"][::core::mem::align_of::<JsonDecoder>() - 4usize];
	["Offset of field: JsonDecoder::decodeError"][::core::mem::offset_of!(JsonDecoder, decodeError) - 0usize];
	["Offset of field: JsonDecoder::willDecodeSublist"]
		[::core::mem::offset_of!(JsonDecoder, willDecodeSublist) - 4usize];
	["Offset of field: JsonDecoder::shouldDecodeTableValueForKey"]
		[::core::mem::offset_of!(JsonDecoder, shouldDecodeTableValueForKey) - 8usize];
	["Offset of field: JsonDecoder::didDecodeTableValue"]
		[::core::mem::offset_of!(JsonDecoder, didDecodeTableValue) - 12usize];
	["Offset of field: JsonDecoder::shouldDecodeArrayValueAtIndex"]
		[::core::mem::offset_of!(JsonDecoder, shouldDecodeArrayValueAtIndex) - 16usize];
	["Offset of field: JsonDecoder::didDecodeArrayValue"]
		[::core::mem::offset_of!(JsonDecoder, didDecodeArrayValue) - 20usize];
	["Offset of field: JsonDecoder::didDecodeSublist"]
		[::core::mem::offset_of!(JsonDecoder, didDecodeSublist) - 24usize];
	["Offset of field: JsonDecoder::userdata"][::core::mem::offset_of!(JsonDecoder, userdata) - 28usize];
	["Offset of field: JsonDecoder::returnString"][::core::mem::offset_of!(JsonDecoder, returnString) - 32usize];
	["Offset of field: JsonDecoder::path"][::core::mem::offset_of!(JsonDecoder, path) - 36usize];
};
pub type JsonReadFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                    buf: *mut u8,
                                                                    bufsize: core::ffi::c_int)
                                                                    -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct JsonReader {
	pub read: JsonReadFunc,
	pub userdata: *mut core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of JsonReader"][::core::mem::size_of::<JsonReader>() - 8usize];
	["Alignment of JsonReader"][::core::mem::align_of::<JsonReader>() - 4usize];
	["Offset of field: JsonReader::read"][::core::mem::offset_of!(JsonReader, read) - 0usize];
	["Offset of field: JsonReader::userdata"][::core::mem::offset_of!(JsonReader, userdata) - 4usize];
};
pub type JsonWriteFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                     str_: *const core::ffi::c_char,
                                                                     len: core::ffi::c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct JsonEncoder {
	pub writeStringFunc: JsonWriteFunc,
	pub userdata: *mut core::ffi::c_void,
	pub _bitfield_align_1: [u32; 0],
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
	pub startArray: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub addArrayMember: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub endArray: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub startTable: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub addTableMember: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder,
	                                                                name: *const core::ffi::c_char,
	                                                                len: core::ffi::c_int)>,
	pub endTable: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub writeNull: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub writeFalse: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub writeTrue: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder)>,
	pub writeInt: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder, num: core::ffi::c_int)>,
	pub writeDouble:
		::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder, num: core::ffi::c_double)>,
	pub writeString: ::core::option::Option<unsafe extern "C" fn(encoder: *mut JsonEncoder,
	                                                             str_: *const core::ffi::c_char,
	                                                             len: core::ffi::c_int)>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of JsonEncoder"][::core::mem::size_of::<JsonEncoder>() - 60usize];
	["Alignment of JsonEncoder"][::core::mem::align_of::<JsonEncoder>() - 4usize];
	["Offset of field: JsonEncoder::writeStringFunc"]
		[::core::mem::offset_of!(JsonEncoder, writeStringFunc) - 0usize];
	["Offset of field: JsonEncoder::userdata"][::core::mem::offset_of!(JsonEncoder, userdata) - 4usize];
	["Offset of field: JsonEncoder::startArray"][::core::mem::offset_of!(JsonEncoder, startArray) - 12usize];
	["Offset of field: JsonEncoder::addArrayMember"]
		[::core::mem::offset_of!(JsonEncoder, addArrayMember) - 16usize];
	["Offset of field: JsonEncoder::endArray"][::core::mem::offset_of!(JsonEncoder, endArray) - 20usize];
	["Offset of field: JsonEncoder::startTable"][::core::mem::offset_of!(JsonEncoder, startTable) - 24usize];
	["Offset of field: JsonEncoder::addTableMember"]
		[::core::mem::offset_of!(JsonEncoder, addTableMember) - 28usize];
	["Offset of field: JsonEncoder::endTable"][::core::mem::offset_of!(JsonEncoder, endTable) - 32usize];
	["Offset of field: JsonEncoder::writeNull"][::core::mem::offset_of!(JsonEncoder, writeNull) - 36usize];
	["Offset of field: JsonEncoder::writeFalse"][::core::mem::offset_of!(JsonEncoder, writeFalse) - 40usize];
	["Offset of field: JsonEncoder::writeTrue"][::core::mem::offset_of!(JsonEncoder, writeTrue) - 44usize];
	["Offset of field: JsonEncoder::writeInt"][::core::mem::offset_of!(JsonEncoder, writeInt) - 48usize];
	["Offset of field: JsonEncoder::writeDouble"][::core::mem::offset_of!(JsonEncoder, writeDouble) - 52usize];
	["Offset of field: JsonEncoder::writeString"][::core::mem::offset_of!(JsonEncoder, writeString) - 56usize];
};
impl JsonEncoder {
	#[inline]
	pub fn pretty(&self) -> core::ffi::c_int {
		unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_pretty(&mut self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub unsafe fn pretty_raw(this: *const Self) -> core::ffi::c_int {
		unsafe {
			:: core :: mem :: transmute (< __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_get (:: core :: ptr :: addr_of ! ((* this) . _bitfield_1) , 0usize , 1u8 ,) as u32)
		}
	}
	#[inline]
	pub unsafe fn set_pretty_raw(this: *mut Self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
			                                               ::core::ptr::addr_of_mut!((*this)._bitfield_1),
			                                               0usize,
			                                               1u8,
			                                               val as u64,
			)
		}
	}
	#[inline]
	pub fn startedTable(&self) -> core::ffi::c_int {
		unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_startedTable(&mut self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub unsafe fn startedTable_raw(this: *const Self) -> core::ffi::c_int {
		unsafe {
			:: core :: mem :: transmute (< __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_get (:: core :: ptr :: addr_of ! ((* this) . _bitfield_1) , 1usize , 1u8 ,) as u32)
		}
	}
	#[inline]
	pub unsafe fn set_startedTable_raw(this: *mut Self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
			                                               ::core::ptr::addr_of_mut!((*this)._bitfield_1),
			                                               1usize,
			                                               1u8,
			                                               val as u64,
			)
		}
	}
	#[inline]
	pub fn startedArray(&self) -> core::ffi::c_int {
		unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
	}
	#[inline]
	pub fn set_startedArray(&mut self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	#[inline]
	pub unsafe fn startedArray_raw(this: *const Self) -> core::ffi::c_int {
		unsafe {
			:: core :: mem :: transmute (< __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_get (:: core :: ptr :: addr_of ! ((* this) . _bitfield_1) , 2usize , 1u8 ,) as u32)
		}
	}
	#[inline]
	pub unsafe fn set_startedArray_raw(this: *mut Self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
			                                               ::core::ptr::addr_of_mut!((*this)._bitfield_1),
			                                               2usize,
			                                               1u8,
			                                               val as u64,
			)
		}
	}
	#[inline]
	pub fn depth(&self) -> core::ffi::c_int {
		unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 29u8) as u32) }
	}
	#[inline]
	pub fn set_depth(&mut self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			self._bitfield_1.set(3usize, 29u8, val as u64)
		}
	}
	#[inline]
	pub unsafe fn depth_raw(this: *const Self) -> core::ffi::c_int {
		unsafe {
			:: core :: mem :: transmute (< __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_get (:: core :: ptr :: addr_of ! ((* this) . _bitfield_1) , 3usize , 29u8 ,) as u32)
		}
	}
	#[inline]
	pub unsafe fn set_depth_raw(this: *mut Self, val: core::ffi::c_int) {
		unsafe {
			let val: u32 = ::core::mem::transmute(val);
			<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
			                                               ::core::ptr::addr_of_mut!((*this)._bitfield_1),
			                                               3usize,
			                                               29u8,
			                                               val as u64,
			)
		}
	}
	#[inline]
	pub fn new_bitfield_1(pretty: core::ffi::c_int,
	                      startedTable: core::ffi::c_int,
	                      startedArray: core::ffi::c_int,
	                      depth: core::ffi::c_int)
	                      -> __BindgenBitfieldUnit<[u8; 4usize]> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			                       let pretty: u32 = unsafe { ::core::mem::transmute(pretty) };
			                       pretty as u64
		                       });
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			                       let startedTable: u32 = unsafe { ::core::mem::transmute(startedTable) };
			                       startedTable as u64
		                       });
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			                       let startedArray: u32 = unsafe { ::core::mem::transmute(startedArray) };
			                       startedArray as u64
		                       });
		__bindgen_bitfield_unit.set(3usize, 29u8, {
			                       let depth: u32 = unsafe { ::core::mem::transmute(depth) };
			                       depth as u64
		                       });
		__bindgen_bitfield_unit
	}
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateJson {
	#[doc = "Populates the given json_encoder *encoder* with the functions necessary to encode arbitrary data into a JSON string. *userdata* is passed as the first argument of the given writeFunc *write*. When *pretty* is 1 the string is written with human-readable formatting.\n"]
	pub initEncoder: unsafe extern "C" fn(encoder: *mut JsonEncoder,
	                                      write: JsonWriteFunc,
	                                      userdata: *mut core::ffi::c_void,
	                                      pretty: core::ffi::c_int),
	#[doc = "Equivalent to [```\nplaydate.json.decode()```\n](<./Inside Playdate.html#f-json.decode>) in the Lua API.\n"]
	pub decode: unsafe extern "C" fn(functions: *mut JsonDecoder,
	                                 reader: JsonReader,
	                                 outval: *mut JsonValue) -> core::ffi::c_int,
	#[doc = "Decodes a JSON file or string with the given *decoder*. An instance of json_decoder must implement *decodeError*. The remaining functions are optional although you’ll probably want to implement at least *didDecodeTableValue* and *didDecodeArrayValue*. The *outval* pointer, if set, contains the value retured from the top-level *didDecodeSublist* callback.\n"]
	pub decodeString: unsafe extern "C" fn(functions: *mut JsonDecoder,
	                                       jsonString: *const core::ffi::c_char,
	                                       outval: *mut JsonValue)
	                                       -> core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateJson"][::core::mem::size_of::<PlaydateJson>() - 12usize];
	["Alignment of PlaydateJson"][::core::mem::align_of::<PlaydateJson>() - 4usize];
	["Offset of field: PlaydateJson::initEncoder"][::core::mem::offset_of!(PlaydateJson, initEncoder) - 0usize];
	["Offset of field: PlaydateJson::decode"][::core::mem::offset_of!(PlaydateJson, decode) - 4usize];
	["Offset of field: PlaydateJson::decodeString"][::core::mem::offset_of!(PlaydateJson, decodeString) - 8usize];
};
pub type SdFile = core::ffi::c_void;
impl FileOptions {
	pub const Read: FileOptions = FileOptions(1);
}
impl FileOptions {
	pub const ReadData: FileOptions = FileOptions(2);
}
impl FileOptions {
	pub const Write: FileOptions = FileOptions(4);
}
impl FileOptions {
	pub const Append: FileOptions = FileOptions(8);
}
impl ::core::ops::BitOr<FileOptions> for FileOptions {
	type Output = Self;
	#[inline]
	fn bitor(self, other: Self) -> Self { FileOptions(self.0 | other.0) }
}
impl ::core::ops::BitOrAssign for FileOptions {
	#[inline]
	fn bitor_assign(&mut self, rhs: FileOptions) { self.0 |= rhs.0; }
}
impl ::core::ops::BitAnd<FileOptions> for FileOptions {
	type Output = Self;
	#[inline]
	fn bitand(self, other: Self) -> Self { FileOptions(self.0 & other.0) }
}
impl ::core::ops::BitAndAssign for FileOptions {
	#[inline]
	fn bitand_assign(&mut self, rhs: FileOptions) { self.0 &= rhs.0; }
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct FileOptions(pub u8);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
#[must_use]
pub struct FileStat {
	pub isdir: core::ffi::c_int,
	pub size: core::ffi::c_uint,
	pub m_year: core::ffi::c_int,
	pub m_month: core::ffi::c_int,
	pub m_day: core::ffi::c_int,
	pub m_hour: core::ffi::c_int,
	pub m_minute: core::ffi::c_int,
	pub m_second: core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of FileStat"][::core::mem::size_of::<FileStat>() - 32usize];
	["Alignment of FileStat"][::core::mem::align_of::<FileStat>() - 4usize];
	["Offset of field: FileStat::isdir"][::core::mem::offset_of!(FileStat, isdir) - 0usize];
	["Offset of field: FileStat::size"][::core::mem::offset_of!(FileStat, size) - 4usize];
	["Offset of field: FileStat::m_year"][::core::mem::offset_of!(FileStat, m_year) - 8usize];
	["Offset of field: FileStat::m_month"][::core::mem::offset_of!(FileStat, m_month) - 12usize];
	["Offset of field: FileStat::m_day"][::core::mem::offset_of!(FileStat, m_day) - 16usize];
	["Offset of field: FileStat::m_hour"][::core::mem::offset_of!(FileStat, m_hour) - 20usize];
	["Offset of field: FileStat::m_minute"][::core::mem::offset_of!(FileStat, m_minute) - 24usize];
	["Offset of field: FileStat::m_second"][::core::mem::offset_of!(FileStat, m_second) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateFile { # [doc = "Returns human-readable text describing the most recent error (usually indicated by a -1 return from a filesystem function).\n"] pub geterr : unsafe extern "C" fn () -> * const core :: ffi :: c_char , # [doc = "Calls the given callback function for every file at *path*. Subfolders are indicated by a trailing slash '/' in *filename*. *listfiles()* does not recurse into subfolders. If *showhidden* is set, files beginning with a period will be included; otherwise, they are skipped. Returns 0 on success, -1 if no folder exists at *path* or it can’t be opened.\nEquivalent to [```\nplaydate.file.listFiles()```\n](<./Inside Playdate.html#f-file.listFiles>) in the Lua API.\n"] pub listfiles : unsafe extern "C" fn (path : * const core :: ffi :: c_char , callback : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , userdata : * mut core :: ffi :: c_void) > , userdata : * mut core :: ffi :: c_void , showhidden : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Populates the FileStat *stat* with information about the file at *path*. Returns 0 on success, or -1 in case of error.\nFileStattypedef struct\n{\n\tint isdir;\n\tunsigned int size;\n\tint m_year;\n\tint m_month;\n\tint m_day;\n\tint m_hour;\n\tint m_minute;\n\tint m_second;\n} FileStat;"] pub stat : unsafe extern "C" fn (path : * const core :: ffi :: c_char , stat : * mut FileStat) -> core :: ffi :: c_int , # [doc = "Creates the given *path* in the Data/&lt;gameid&gt; folder. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\nEquivalent to [```\nplaydate.file.mkdir()```\n](<./Inside Playdate.html#f-file.mkdir>) in the Lua API.\n"] pub mkdir : unsafe extern "C" fn (path : * const core :: ffi :: c_char) -> core :: ffi :: c_int , # [doc = "Deletes the file at *path*. Returns 0 on success, or -1 in case of error. If recursive is 1 and the target path is a folder, this deletes everything inside the folder (including folders, folders inside those, and so on) as well as the folder itself.\n"] pub unlink : unsafe extern "C" fn (name : * const core :: ffi :: c_char , recursive : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Renames the file at *from* to *to*. It will overwrite the file at *to* without confirmation. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\nEquivalent to [```\nplaydate.file.rename()```\n](<./Inside Playdate.html#f-file.rename>) in the Lua API.\n"] pub rename : unsafe extern "C" fn (from : * const core :: ffi :: c_char , to : * const core :: ffi :: c_char) -> core :: ffi :: c_int , # [doc = "Opens a handle for the file at *path*. The *kFileRead* mode opens a file in the game pdx, while *kFileReadData* searches the game’s data folder; to search the data folder first then fall back on the game pdx, use the bitwise combination *kFileRead|kFileReadData*.*kFileWrite* and *kFileAppend* always write to the data folder. The function returns NULL if a file at *path* cannot be opened, and [playdate-&gt;file-&gt;geterr()](#f-file.geterr) will describe the error. The filesystem has a limit of 64 simultaneous open files. The returned file handle should be [closed](#f-file.close), not freed, when it is no longer in use.\nFileOptionstypedef enum\n{\n\tkFileRead,\n\tkFileReadData,\n\tkFileWrite,\n\tkFileAppend\n} FileOptions;Equivalent to [```\nplaydate.file.open()```\n](<./Inside Playdate.html#f-file.open>) in the Lua API.\n"] pub open : unsafe extern "C" fn (name : * const core :: ffi :: c_char , mode : FileOptions) -> * mut SdFile , # [doc = "Closes the given *file* handle. Returns 0 on success, or -1 in case of error.\nEquivalent to [```\nplaydate.file.close()```\n](<./Inside Playdate.html#f-file.close>) in the Lua API.\n"] pub close : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , # [doc = "Reads up to *len* bytes from the *file* into the buffer *buf*. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.\nEquivalent to [```\nplaydate.file.file:read()```\n](<./Inside Playdate.html#m-file.read>) in the Lua API.\n"] pub read : unsafe extern "C" fn (file : * mut SdFile , buf : * mut core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int , # [doc = "Writes the buffer of bytes *buf* to the *file*. Returns the number of bytes written, or -1 in case of error.\nEquivalent to [```\nplaydate.file.file:write()```\n](<./Inside Playdate.html#m-file.write>) in the Lua API.\n"] pub write : unsafe extern "C" fn (file : * mut SdFile , buf : * const core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int , # [doc = "Flushes the output buffer of *file* immediately. Returns the number of bytes written, or -1 in case of error.\nEquivalent to [```\nplaydate.file.flush()```\n](<./Inside Playdate.html#f-file.flush>) in the Lua API.\n"] pub flush : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , # [doc = "Returns the current read/write offset in the given *file* handle, or -1 on error.\nEquivalent to [```\nplaydate.file.file:tell()```\n](<./Inside Playdate.html#m-file.tell>) in the Lua API.\n"] pub tell : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , # [doc = "Sets the read/write offset in the given *file* handle to *pos*, relative to the *whence* macro. SEEK_SET is relative to the beginning of the file, SEEK_CUR is relative to the current position of the file pointer, and SEEK_END is relative to the end of the file. Returns 0 on success, -1 on error.\nEquivalent to [```\nplaydate.file.file:seek()```\n](<./Inside Playdate.html#m-file.seek>) in the Lua API.\n"] pub seek : unsafe extern "C" fn (file : * mut SdFile , pos : core :: ffi :: c_int , whence : core :: ffi :: c_int) -> core :: ffi :: c_int , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateFile"][::core::mem::size_of::<PlaydateFile>() - 52usize];
	["Alignment of PlaydateFile"][::core::mem::align_of::<PlaydateFile>() - 4usize];
	["Offset of field: PlaydateFile::geterr"][::core::mem::offset_of!(PlaydateFile, geterr) - 0usize];
	["Offset of field: PlaydateFile::listfiles"][::core::mem::offset_of!(PlaydateFile, listfiles) - 4usize];
	["Offset of field: PlaydateFile::stat"][::core::mem::offset_of!(PlaydateFile, stat) - 8usize];
	["Offset of field: PlaydateFile::mkdir"][::core::mem::offset_of!(PlaydateFile, mkdir) - 12usize];
	["Offset of field: PlaydateFile::unlink"][::core::mem::offset_of!(PlaydateFile, unlink) - 16usize];
	["Offset of field: PlaydateFile::rename"][::core::mem::offset_of!(PlaydateFile, rename) - 20usize];
	["Offset of field: PlaydateFile::open"][::core::mem::offset_of!(PlaydateFile, open) - 24usize];
	["Offset of field: PlaydateFile::close"][::core::mem::offset_of!(PlaydateFile, close) - 28usize];
	["Offset of field: PlaydateFile::read"][::core::mem::offset_of!(PlaydateFile, read) - 32usize];
	["Offset of field: PlaydateFile::write"][::core::mem::offset_of!(PlaydateFile, write) - 36usize];
	["Offset of field: PlaydateFile::flush"][::core::mem::offset_of!(PlaydateFile, flush) - 40usize];
	["Offset of field: PlaydateFile::tell"][::core::mem::offset_of!(PlaydateFile, tell) - 44usize];
	["Offset of field: PlaydateFile::seek"][::core::mem::offset_of!(PlaydateFile, seek) - 48usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SpriteCollisionResponseType {
	Slide = 0,
	Freeze = 1,
	Overlap = 2,
	Bounce = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
#[must_use]
pub struct Rect {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
	pub width: core::ffi::c_float,
	pub height: core::ffi::c_float,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of Rect"][::core::mem::size_of::<Rect>() - 16usize];
	["Alignment of Rect"][::core::mem::align_of::<Rect>() - 4usize];
	["Offset of field: Rect::x"][::core::mem::offset_of!(Rect, x) - 0usize];
	["Offset of field: Rect::y"][::core::mem::offset_of!(Rect, y) - 4usize];
	["Offset of field: Rect::width"][::core::mem::offset_of!(Rect, width) - 8usize];
	["Offset of field: Rect::height"][::core::mem::offset_of!(Rect, height) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
#[must_use]
pub struct CollisionPoint {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of CollisionPoint"][::core::mem::size_of::<CollisionPoint>() - 8usize];
	["Alignment of CollisionPoint"][::core::mem::align_of::<CollisionPoint>() - 4usize];
	["Offset of field: CollisionPoint::x"][::core::mem::offset_of!(CollisionPoint, x) - 0usize];
	["Offset of field: CollisionPoint::y"][::core::mem::offset_of!(CollisionPoint, y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
#[must_use]
pub struct CollisionVector {
	pub x: core::ffi::c_int,
	pub y: core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of CollisionVector"][::core::mem::size_of::<CollisionVector>() - 8usize];
	["Alignment of CollisionVector"][::core::mem::align_of::<CollisionVector>() - 4usize];
	["Offset of field: CollisionVector::x"][::core::mem::offset_of!(CollisionVector, x) - 0usize];
	["Offset of field: CollisionVector::y"][::core::mem::offset_of!(CollisionVector, y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[must_use]
pub struct SpriteCollisionInfo {
	pub sprite: *mut Sprite,
	pub other: *mut Sprite,
	pub responseType: SpriteCollisionResponseType,
	pub overlaps: u8,
	pub ti: core::ffi::c_float,
	pub move_: CollisionPoint,
	pub normal: CollisionVector,
	pub touch: CollisionPoint,
	pub spriteRect: Rect,
	pub otherRect: Rect,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of SpriteCollisionInfo"][::core::mem::size_of::<SpriteCollisionInfo>() - 72usize];
	["Alignment of SpriteCollisionInfo"][::core::mem::align_of::<SpriteCollisionInfo>() - 4usize];
	["Offset of field: SpriteCollisionInfo::sprite"]
		[::core::mem::offset_of!(SpriteCollisionInfo, sprite) - 0usize];
	["Offset of field: SpriteCollisionInfo::other"][::core::mem::offset_of!(SpriteCollisionInfo, other) - 4usize];
	["Offset of field: SpriteCollisionInfo::responseType"]
		[::core::mem::offset_of!(SpriteCollisionInfo, responseType) - 8usize];
	["Offset of field: SpriteCollisionInfo::overlaps"]
		[::core::mem::offset_of!(SpriteCollisionInfo, overlaps) - 9usize];
	["Offset of field: SpriteCollisionInfo::ti"][::core::mem::offset_of!(SpriteCollisionInfo, ti) - 12usize];
	["Offset of field: SpriteCollisionInfo::move_"][::core::mem::offset_of!(SpriteCollisionInfo, move_) - 16usize];
	["Offset of field: SpriteCollisionInfo::normal"]
		[::core::mem::offset_of!(SpriteCollisionInfo, normal) - 24usize];
	["Offset of field: SpriteCollisionInfo::touch"][::core::mem::offset_of!(SpriteCollisionInfo, touch) - 32usize];
	["Offset of field: SpriteCollisionInfo::spriteRect"]
		[::core::mem::offset_of!(SpriteCollisionInfo, spriteRect) - 40usize];
	["Offset of field: SpriteCollisionInfo::otherRect"]
		[::core::mem::offset_of!(SpriteCollisionInfo, otherRect) - 56usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[must_use]
pub struct SpriteQueryInfo {
	pub sprite: *mut Sprite,
	pub ti1: core::ffi::c_float,
	pub ti2: core::ffi::c_float,
	pub entryPoint: CollisionPoint,
	pub exitPoint: CollisionPoint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of SpriteQueryInfo"][::core::mem::size_of::<SpriteQueryInfo>() - 28usize];
	["Alignment of SpriteQueryInfo"][::core::mem::align_of::<SpriteQueryInfo>() - 4usize];
	["Offset of field: SpriteQueryInfo::sprite"][::core::mem::offset_of!(SpriteQueryInfo, sprite) - 0usize];
	["Offset of field: SpriteQueryInfo::ti1"][::core::mem::offset_of!(SpriteQueryInfo, ti1) - 4usize];
	["Offset of field: SpriteQueryInfo::ti2"][::core::mem::offset_of!(SpriteQueryInfo, ti2) - 8usize];
	["Offset of field: SpriteQueryInfo::entryPoint"]
		[::core::mem::offset_of!(SpriteQueryInfo, entryPoint) - 12usize];
	["Offset of field: SpriteQueryInfo::exitPoint"][::core::mem::offset_of!(SpriteQueryInfo, exitPoint) - 20usize];
};
pub type SpriteDrawFunction =
	::core::option::Option<unsafe extern "C" fn(sprite: *mut Sprite, bounds: Rect, drawrect: Rect)>;
pub type SpriteUpdateFunction = ::core::option::Option<unsafe extern "C" fn(sprite: *mut Sprite)>;
pub type SpriteCollisionFilterProc = ::core::option::Option<unsafe extern "C" fn(sprite: *mut Sprite,
                                                                                 other: *mut Sprite)
                                                                                 -> SpriteCollisionResponseType>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSprite {
	#[doc = "When *flag* is set to 1, this causes all sprites to draw each frame, whether or not they have been marked dirty. This may speed up the performance of your game if the system’s dirty rect tracking is taking up too much time - for example if there are many sprites moving around on screen at once.\n"]
	pub setAlwaysRedraw: unsafe extern "C" fn(flag: core::ffi::c_int),
	#[doc = "Marks the given *dirtyRect* (in screen coordinates) as needing a redraw. Graphics drawing functions now call this automatically, adding their drawn areas to the sprite’s dirty list, so there’s usually no need to call this manually.\n"]
	pub addDirtyRect: unsafe extern "C" fn(dirtyRect: Aabb),
	#[doc = "Draws every sprite in the display list.\n"]
	pub drawSprites: unsafe extern "C" fn(),
	#[doc = "Updates and draws every sprite in the display list.\n"]
	pub updateAndDrawSprites: unsafe extern "C" fn(),
	#[doc = "Allocates and returns a new LCDSprite.\n"]
	pub newSprite: unsafe extern "C" fn() -> *mut Sprite,
	pub freeSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Allocates and returns a copy of the given *sprite*.\n"]
	pub copy: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut Sprite,
	#[doc = "Adds the given *sprite* to the display list, so that it is drawn in the current scene.\n"]
	pub addSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Removes the given *sprite* from the display list.\n"]
	pub removeSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Removes the given *count* sized array of *sprites* from the display list.\n"]
	pub removeSprites: unsafe extern "C" fn(sprites: *mut *mut Sprite, count: core::ffi::c_int),
	#[doc = "Removes all sprites from the display list.\n"]
	pub removeAllSprites: unsafe extern "C" fn(),
	#[doc = "Returns the total number of sprites in the display list.\n"]
	pub getSpriteCount: unsafe extern "C" fn() -> core::ffi::c_int,
	#[doc = "Sets the bounds of the given *sprite* with *bounds*.\n"]
	pub setBounds: unsafe extern "C" fn(sprite: *mut Sprite, bounds: Rect),
	#[doc = "Returns the bounds of the given *sprite* as an PDRect;\n"]
	pub getBounds: unsafe extern "C" fn(sprite: *mut Sprite) -> Rect,
	#[doc = "Moves the given *sprite* to *x*, *y* and resets its bounds based on the bitmap dimensions and center.\n"]
	pub moveTo: unsafe extern "C" fn(sprite: *mut Sprite, x: core::ffi::c_float, y: core::ffi::c_float),
	#[doc = "Moves the given *sprite* to by offsetting its current position by *dx*, *dy*.\n"]
	pub moveBy: unsafe extern "C" fn(sprite: *mut Sprite, dx: core::ffi::c_float, dy: core::ffi::c_float),
	#[doc = "Sets the given *sprite*'s image to the given *bitmap*.\n"]
	pub setImage: unsafe extern "C" fn(sprite: *mut Sprite, image: *mut Bitmap, flip: BitmapFlip),
	#[doc = "Returns the LCDBitmap currently assigned to the given *sprite*.\n"]
	pub getImage: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut Bitmap,
	#[doc = "Sets the size. The size is used to set the sprite’s bounds when calling moveTo().\n"]
	pub setSize: unsafe extern "C" fn(s: *mut Sprite, width: core::ffi::c_float, height: core::ffi::c_float),
	#[doc = "Sets the Z order of the given *sprite*. Higher Z sprites are drawn on top of those with lower Z order.\n"]
	pub setZIndex: unsafe extern "C" fn(sprite: *mut Sprite, zIndex: i16),
	#[doc = "Returns the Z index of the given *sprite*.\n"]
	pub getZIndex: unsafe extern "C" fn(sprite: *mut Sprite) -> i16,
	#[doc = "Sets the mode for drawing the sprite’s bitmap.\n"]
	pub setDrawMode: unsafe extern "C" fn(sprite: *mut Sprite, mode: BitmapDrawMode),
	#[doc = "Flips the bitmap.\n"]
	pub setImageFlip: unsafe extern "C" fn(sprite: *mut Sprite, flip: BitmapFlip),
	#[doc = "Returns the flip setting of the sprite’s bitmap.\n"]
	pub getImageFlip: unsafe extern "C" fn(sprite: *mut Sprite) -> BitmapFlip,
	#[doc = "Specifies a stencil image to be set on the frame buffer before the sprite is drawn.\n"]
	pub setStencil: unsafe extern "C" fn(sprite: *mut Sprite, stencil: *mut Bitmap),
	#[doc = "Sets the clipping rectangle for sprite drawing.\n"]
	pub setClipRect: unsafe extern "C" fn(sprite: *mut Sprite, clipRect: Aabb),
	#[doc = "Clears the sprite’s clipping rectangle.\n"]
	pub clearClipRect: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Sets the clipping rectangle for *all* sprites with a Z index within *startZ* and *endZ* inclusive.\n"]
	pub setClipRectsInRange:
		unsafe extern "C" fn(clipRect: Aabb, startZ: core::ffi::c_int, endZ: core::ffi::c_int),
	#[doc = "Clears the clipping rectangle for *all* sprites with a Z index within *startZ* and *endZ* inclusive.\n"]
	pub clearClipRectsInRange: unsafe extern "C" fn(startZ: core::ffi::c_int, endZ: core::ffi::c_int),
	#[doc = "Set the updatesEnabled flag of the given *sprite* (determines whether the sprite has its update function called). One is true, 0 is false.\n"]
	pub setUpdatesEnabled: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	#[doc = "Get the updatesEnabled flag of the given *sprite*.\n"]
	pub updatesEnabled: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	#[doc = "Set the collisionsEnabled flag of the given *sprite* (along with the collideRect, this determines whether the sprite participates in collisions). One is true, 0 is false. Set to 1 by default.\n"]
	pub setCollisionsEnabled: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	#[doc = "Get the collisionsEnabled flag of the given *sprite*.\n"]
	pub collisionsEnabled: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	#[doc = "Set the visible flag of the given *sprite* (determines whether the sprite has its draw function called). One is true, 0 is false.\n"]
	pub setVisible: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	#[doc = "Get the visible flag of the given *sprite*.\n"]
	pub isVisible: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	#[doc = "Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.\n"]
	pub setOpaque: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	#[doc = "Forces the given *sprite* to redraw.\n"]
	pub markDirty: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Sets the tag of the given *sprite*. This can be useful for identifying sprites or types of sprites when using the collision API.\n"]
	pub setTag: unsafe extern "C" fn(sprite: *mut Sprite, tag: u8),
	#[doc = "Returns the tag of the given *sprite*.\n"]
	pub getTag: unsafe extern "C" fn(sprite: *mut Sprite) -> u8,
	#[doc = "When *flag* is set to 1, the *sprite* will draw in screen coordinates, ignoring the currently-set drawOffset.\nThis only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space.\n"]
	pub setIgnoresDrawOffset: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	#[doc = "Sets the update function for the given *sprite*.\n"]
	pub setUpdateFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteUpdateFunction),
	#[doc = "Sets the draw function for the given *sprite*. Note that the callback is only called when the sprite is on screen and has a size specified via [playdate→sprite→setSize()](#f-sprite.setSize) or [playdate→sprite→setBounds()](#f-sprite.setBounds).\n"]
	pub setDrawFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteDrawFunction),
	#[doc = "Sets *x* and *y* to the current position of *sprite*.\n"]
	pub getPosition:
		unsafe extern "C" fn(sprite: *mut Sprite, x: *mut core::ffi::c_float, y: *mut core::ffi::c_float),
	#[doc = "Frees and reallocates internal collision data, resetting everything to its default state.\n"]
	pub resetCollisionWorld: unsafe extern "C" fn(),
	#[doc = "Marks the area of the given *sprite*, relative to its bounds, to be checked for collisions with other sprites' collide rects.\n"]
	pub setCollideRect: unsafe extern "C" fn(sprite: *mut Sprite, collideRect: Rect),
	#[doc = "Returns the given *sprite*’s collide rect.\n"]
	pub getCollideRect: unsafe extern "C" fn(sprite: *mut Sprite) -> Rect,
	#[doc = "Clears the given *sprite*’s collide rect.\n"]
	pub clearCollideRect: unsafe extern "C" fn(sprite: *mut Sprite),
	#[doc = "Set a callback that returns a [SpriteCollisionResponseType](#_SpriteCollisionResponseType) for a collision between *sprite* and *other*.\nLCDSpriteCollisionFilterProctypedef SpriteCollisionResponseType LCDSpriteCollisionFilterProc(LCDSprite* sprite, LCDSprite* other);"]
	pub setCollisionResponseFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteCollisionFilterProc),
	#[doc = "Returns the same values as [playdate-&gt;sprite-&gt;moveWithCollisions()](#f-sprite.moveWithCollisions) but does not actually move the sprite. The caller is responsible for freeing the returned array.\n"]
	pub checkCollisions: unsafe extern "C" fn(sprite: *mut Sprite,
	                                          goalX: core::ffi::c_float,
	                                          goalY: core::ffi::c_float,
	                                          actualX: *mut core::ffi::c_float,
	                                          actualY: *mut core::ffi::c_float,
	                                          len: *mut core::ffi::c_int)
	                                          -> *mut SpriteCollisionInfo,
	#[doc = "Moves the given *sprite* towards *goalX*, *goalY* taking collisions into account and returns an array of SpriteCollisionInfo. *len* is set to the size of the array and *actualX*, *actualY* are set to the sprite’s position after collisions. If no collisions occurred, this will be the same as *goalX*, *goalY*. The caller is responsible for freeing the returned array.\n"]
	pub moveWithCollisions: unsafe extern "C" fn(sprite: *mut Sprite,
	                                             goalX: core::ffi::c_float,
	                                             goalY: core::ffi::c_float,
	                                             actualX: *mut core::ffi::c_float,
	                                             actualY: *mut core::ffi::c_float,
	                                             len: *mut core::ffi::c_int)
	                                             -> *mut SpriteCollisionInfo,
	#[doc = "Returns an array of all sprites with collision rects containing the point at *x*, *y*. *len* is set to the size of the array. The caller is responsible for freeing the returned array.\n"]
	pub querySpritesAtPoint: unsafe extern "C" fn(x: core::ffi::c_float,
	                                              y: core::ffi::c_float,
	                                              len: *mut core::ffi::c_int)
	                                              -> *mut *mut Sprite,
	#[doc = "Returns an array of all sprites with collision rects that intersect the *width* by *height* rect at *x*, *y*. *len* is set to the size of the array. The caller is responsible for freeing the returned array.\n"]
	pub querySpritesInRect: unsafe extern "C" fn(x: core::ffi::c_float,
	                                             y: core::ffi::c_float,
	                                             width: core::ffi::c_float,
	                                             height: core::ffi::c_float,
	                                             len: *mut core::ffi::c_int)
	                                             -> *mut *mut Sprite,
	#[doc = "Returns an array of all sprites with collision rects that intersect the line connecting *x1*, *y1* and  *x2*, *y2*. *len* is set to the size of the array. The caller is responsible for freeing the returned array.\n"]
	pub querySpritesAlongLine: unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                y1: core::ffi::c_float,
	                                                x2: core::ffi::c_float,
	                                                y2: core::ffi::c_float,
	                                                len: *mut core::ffi::c_int)
	                                                -> *mut *mut Sprite,
	#[doc = "Returns an array of SpriteQueryInfo for all sprites with collision rects that intersect the line connecting *x1*, *y1* and  *x2*, *y2*. *len* is set to the size of the array. If you don’t need this information, use querySpritesAlongLine() as it will be faster. The caller is responsible for freeing the returned array.\n"]
	pub querySpriteInfoAlongLine: unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                   y1: core::ffi::c_float,
	                                                   x2: core::ffi::c_float,
	                                                   y2: core::ffi::c_float,
	                                                   len: *mut core::ffi::c_int)
	                                                   -> *mut SpriteQueryInfo,
	#[doc = "Returns an array of sprites that have collide rects that are currently overlapping the given *sprite*’s collide rect. *len* is set to the size of the array. The caller is responsible for freeing the returned array.\n"]
	pub overlappingSprites:
		unsafe extern "C" fn(sprite: *mut Sprite, len: *mut core::ffi::c_int) -> *mut *mut Sprite,
	#[doc = "Returns an array of all sprites that have collide rects that are currently overlapping. Each consecutive pair of sprites is overlapping (eg. 0 &amp; 1 overlap, 2 &amp; 3 overlap, etc). *len* is set to the size of the array. The caller is responsible for freeing the returned array.\n"]
	pub allOverlappingSprites: unsafe extern "C" fn(len: *mut core::ffi::c_int) -> *mut *mut Sprite,
	#[doc = "Sets the sprite’s stencil to the given pattern.\n"]
	pub setStencilPattern: unsafe extern "C" fn(sprite: *mut Sprite, pattern: *mut [u8; 8usize]),
	#[doc = "Clears the sprite’s stencil.\n"]
	pub clearStencil: unsafe extern "C" fn(sprite: *mut Sprite),
	pub setUserdata: unsafe extern "C" fn(sprite: *mut Sprite, userdata: *mut core::ffi::c_void),
	#[doc = "Sets and gets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.\n"]
	pub getUserdata: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut core::ffi::c_void,
	#[doc = "Specifies a stencil image to be set on the frame buffer before the sprite is drawn. If *tile* is set, the stencil will be tiled. Tiled stencils must have width evenly divisible by 32.\n"]
	pub setStencilImage: unsafe extern "C" fn(sprite: *mut Sprite, stencil: *mut Bitmap, tile: core::ffi::c_int),
	#[doc = "Sets the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width. Default is 0.5, 0.5 (the center of the sprite). This means that when you call [sprite→moveTo(sprite, x, y)](#f-sprite.moveTo), the center of your sprite will be positioned at *x*, *y*. If you want x and y to represent the upper left corner of your sprite, specify the center as 0, 0.\n"]
	pub setCenter: unsafe extern "C" fn(s: *mut Sprite, x: core::ffi::c_float, y: core::ffi::c_float),
	#[doc = "Sets the values in ```\noutx```\n and ```\nouty```\n to the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width.\n"]
	pub getCenter: unsafe extern "C" fn(s: *mut Sprite, x: *mut core::ffi::c_float, y: *mut core::ffi::c_float),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSprite"][::core::mem::size_of::<PlaydateSprite>() - 252usize];
	["Alignment of PlaydateSprite"][::core::mem::align_of::<PlaydateSprite>() - 4usize];
	["Offset of field: PlaydateSprite::setAlwaysRedraw"]
		[::core::mem::offset_of!(PlaydateSprite, setAlwaysRedraw) - 0usize];
	["Offset of field: PlaydateSprite::addDirtyRect"]
		[::core::mem::offset_of!(PlaydateSprite, addDirtyRect) - 4usize];
	["Offset of field: PlaydateSprite::drawSprites"]
		[::core::mem::offset_of!(PlaydateSprite, drawSprites) - 8usize];
	["Offset of field: PlaydateSprite::updateAndDrawSprites"]
		[::core::mem::offset_of!(PlaydateSprite, updateAndDrawSprites) - 12usize];
	["Offset of field: PlaydateSprite::newSprite"][::core::mem::offset_of!(PlaydateSprite, newSprite) - 16usize];
	["Offset of field: PlaydateSprite::freeSprite"][::core::mem::offset_of!(PlaydateSprite, freeSprite) - 20usize];
	["Offset of field: PlaydateSprite::copy"][::core::mem::offset_of!(PlaydateSprite, copy) - 24usize];
	["Offset of field: PlaydateSprite::addSprite"][::core::mem::offset_of!(PlaydateSprite, addSprite) - 28usize];
	["Offset of field: PlaydateSprite::removeSprite"]
		[::core::mem::offset_of!(PlaydateSprite, removeSprite) - 32usize];
	["Offset of field: PlaydateSprite::removeSprites"]
		[::core::mem::offset_of!(PlaydateSprite, removeSprites) - 36usize];
	["Offset of field: PlaydateSprite::removeAllSprites"]
		[::core::mem::offset_of!(PlaydateSprite, removeAllSprites) - 40usize];
	["Offset of field: PlaydateSprite::getSpriteCount"]
		[::core::mem::offset_of!(PlaydateSprite, getSpriteCount) - 44usize];
	["Offset of field: PlaydateSprite::setBounds"][::core::mem::offset_of!(PlaydateSprite, setBounds) - 48usize];
	["Offset of field: PlaydateSprite::getBounds"][::core::mem::offset_of!(PlaydateSprite, getBounds) - 52usize];
	["Offset of field: PlaydateSprite::moveTo"][::core::mem::offset_of!(PlaydateSprite, moveTo) - 56usize];
	["Offset of field: PlaydateSprite::moveBy"][::core::mem::offset_of!(PlaydateSprite, moveBy) - 60usize];
	["Offset of field: PlaydateSprite::setImage"][::core::mem::offset_of!(PlaydateSprite, setImage) - 64usize];
	["Offset of field: PlaydateSprite::getImage"][::core::mem::offset_of!(PlaydateSprite, getImage) - 68usize];
	["Offset of field: PlaydateSprite::setSize"][::core::mem::offset_of!(PlaydateSprite, setSize) - 72usize];
	["Offset of field: PlaydateSprite::setZIndex"][::core::mem::offset_of!(PlaydateSprite, setZIndex) - 76usize];
	["Offset of field: PlaydateSprite::getZIndex"][::core::mem::offset_of!(PlaydateSprite, getZIndex) - 80usize];
	["Offset of field: PlaydateSprite::setDrawMode"]
		[::core::mem::offset_of!(PlaydateSprite, setDrawMode) - 84usize];
	["Offset of field: PlaydateSprite::setImageFlip"]
		[::core::mem::offset_of!(PlaydateSprite, setImageFlip) - 88usize];
	["Offset of field: PlaydateSprite::getImageFlip"]
		[::core::mem::offset_of!(PlaydateSprite, getImageFlip) - 92usize];
	["Offset of field: PlaydateSprite::setStencil"][::core::mem::offset_of!(PlaydateSprite, setStencil) - 96usize];
	["Offset of field: PlaydateSprite::setClipRect"]
		[::core::mem::offset_of!(PlaydateSprite, setClipRect) - 100usize];
	["Offset of field: PlaydateSprite::clearClipRect"]
		[::core::mem::offset_of!(PlaydateSprite, clearClipRect) - 104usize];
	["Offset of field: PlaydateSprite::setClipRectsInRange"]
		[::core::mem::offset_of!(PlaydateSprite, setClipRectsInRange) - 108usize];
	["Offset of field: PlaydateSprite::clearClipRectsInRange"]
		[::core::mem::offset_of!(PlaydateSprite, clearClipRectsInRange) - 112usize];
	["Offset of field: PlaydateSprite::setUpdatesEnabled"]
		[::core::mem::offset_of!(PlaydateSprite, setUpdatesEnabled) - 116usize];
	["Offset of field: PlaydateSprite::updatesEnabled"]
		[::core::mem::offset_of!(PlaydateSprite, updatesEnabled) - 120usize];
	["Offset of field: PlaydateSprite::setCollisionsEnabled"]
		[::core::mem::offset_of!(PlaydateSprite, setCollisionsEnabled) - 124usize];
	["Offset of field: PlaydateSprite::collisionsEnabled"]
		[::core::mem::offset_of!(PlaydateSprite, collisionsEnabled) - 128usize];
	["Offset of field: PlaydateSprite::setVisible"]
		[::core::mem::offset_of!(PlaydateSprite, setVisible) - 132usize];
	["Offset of field: PlaydateSprite::isVisible"][::core::mem::offset_of!(PlaydateSprite, isVisible) - 136usize];
	["Offset of field: PlaydateSprite::setOpaque"][::core::mem::offset_of!(PlaydateSprite, setOpaque) - 140usize];
	["Offset of field: PlaydateSprite::markDirty"][::core::mem::offset_of!(PlaydateSprite, markDirty) - 144usize];
	["Offset of field: PlaydateSprite::setTag"][::core::mem::offset_of!(PlaydateSprite, setTag) - 148usize];
	["Offset of field: PlaydateSprite::getTag"][::core::mem::offset_of!(PlaydateSprite, getTag) - 152usize];
	["Offset of field: PlaydateSprite::setIgnoresDrawOffset"]
		[::core::mem::offset_of!(PlaydateSprite, setIgnoresDrawOffset) - 156usize];
	["Offset of field: PlaydateSprite::setUpdateFunction"]
		[::core::mem::offset_of!(PlaydateSprite, setUpdateFunction) - 160usize];
	["Offset of field: PlaydateSprite::setDrawFunction"]
		[::core::mem::offset_of!(PlaydateSprite, setDrawFunction) - 164usize];
	["Offset of field: PlaydateSprite::getPosition"]
		[::core::mem::offset_of!(PlaydateSprite, getPosition) - 168usize];
	["Offset of field: PlaydateSprite::resetCollisionWorld"]
		[::core::mem::offset_of!(PlaydateSprite, resetCollisionWorld) - 172usize];
	["Offset of field: PlaydateSprite::setCollideRect"]
		[::core::mem::offset_of!(PlaydateSprite, setCollideRect) - 176usize];
	["Offset of field: PlaydateSprite::getCollideRect"]
		[::core::mem::offset_of!(PlaydateSprite, getCollideRect) - 180usize];
	["Offset of field: PlaydateSprite::clearCollideRect"]
		[::core::mem::offset_of!(PlaydateSprite, clearCollideRect) - 184usize];
	["Offset of field: PlaydateSprite::setCollisionResponseFunction"]
		[::core::mem::offset_of!(PlaydateSprite, setCollisionResponseFunction) - 188usize];
	["Offset of field: PlaydateSprite::checkCollisions"]
		[::core::mem::offset_of!(PlaydateSprite, checkCollisions) - 192usize];
	["Offset of field: PlaydateSprite::moveWithCollisions"]
		[::core::mem::offset_of!(PlaydateSprite, moveWithCollisions) - 196usize];
	["Offset of field: PlaydateSprite::querySpritesAtPoint"]
		[::core::mem::offset_of!(PlaydateSprite, querySpritesAtPoint) - 200usize];
	["Offset of field: PlaydateSprite::querySpritesInRect"]
		[::core::mem::offset_of!(PlaydateSprite, querySpritesInRect) - 204usize];
	["Offset of field: PlaydateSprite::querySpritesAlongLine"]
		[::core::mem::offset_of!(PlaydateSprite, querySpritesAlongLine) - 208usize];
	["Offset of field: PlaydateSprite::querySpriteInfoAlongLine"]
		[::core::mem::offset_of!(PlaydateSprite, querySpriteInfoAlongLine) - 212usize];
	["Offset of field: PlaydateSprite::overlappingSprites"]
		[::core::mem::offset_of!(PlaydateSprite, overlappingSprites) - 216usize];
	["Offset of field: PlaydateSprite::allOverlappingSprites"]
		[::core::mem::offset_of!(PlaydateSprite, allOverlappingSprites) - 220usize];
	["Offset of field: PlaydateSprite::setStencilPattern"]
		[::core::mem::offset_of!(PlaydateSprite, setStencilPattern) - 224usize];
	["Offset of field: PlaydateSprite::clearStencil"]
		[::core::mem::offset_of!(PlaydateSprite, clearStencil) - 228usize];
	["Offset of field: PlaydateSprite::setUserdata"]
		[::core::mem::offset_of!(PlaydateSprite, setUserdata) - 232usize];
	["Offset of field: PlaydateSprite::getUserdata"]
		[::core::mem::offset_of!(PlaydateSprite, getUserdata) - 236usize];
	["Offset of field: PlaydateSprite::setStencilImage"]
		[::core::mem::offset_of!(PlaydateSprite, setStencilImage) - 240usize];
	["Offset of field: PlaydateSprite::setCenter"][::core::mem::offset_of!(PlaydateSprite, setCenter) - 244usize];
	["Offset of field: PlaydateSprite::getCenter"][::core::mem::offset_of!(PlaydateSprite, getCenter) - 248usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SoundFormat {
	Mono8bit = 0,
	Stereo8bit = 1,
	Mono16bit = 2,
	Stereo16bit = 3,
	MonoADPCM = 4,
	StereoADPCM = 5,
}
pub type MidiNote = core::ffi::c_float;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundSource {
	_unused: [u8; 0],
}
pub type SndCallbackProc =
	::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSource {
	#[doc = "Sets the playback volume (0.0 - 1.0) for left and right channels of the source.\n"]
	pub setVolume: unsafe extern "C" fn(c: *mut SoundSource, lvol: core::ffi::c_float, rvol: core::ffi::c_float),
	#[doc = "Gets the playback volume (0.0 - 1.0) for left and right channels of the source.\n"]
	pub getVolume:
		unsafe extern "C" fn(c: *mut SoundSource, outl: *mut core::ffi::c_float, outr: *mut core::ffi::c_float),
	#[doc = "Returns 1 if the source is currently playing.\n"]
	pub isPlaying: unsafe extern "C" fn(c: *mut SoundSource) -> core::ffi::c_int,
	pub setFinishCallback:
		unsafe extern "C" fn(c: *mut SoundSource, callback: SndCallbackProc, userdata: *mut core::ffi::c_void),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundSource"][::core::mem::size_of::<PlaydateSoundSource>() - 16usize];
	["Alignment of PlaydateSoundSource"][::core::mem::align_of::<PlaydateSoundSource>() - 4usize];
	["Offset of field: PlaydateSoundSource::setVolume"]
		[::core::mem::offset_of!(PlaydateSoundSource, setVolume) - 0usize];
	["Offset of field: PlaydateSoundSource::getVolume"]
		[::core::mem::offset_of!(PlaydateSoundSource, getVolume) - 4usize];
	["Offset of field: PlaydateSoundSource::isPlaying"]
		[::core::mem::offset_of!(PlaydateSoundSource, isPlaying) - 8usize];
	["Offset of field: PlaydateSoundSource::setFinishCallback"]
		[::core::mem::offset_of!(PlaydateSoundSource, setFinishCallback) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct FilePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundFileplayer { # [doc = "Allocates a new FilePlayer.\n"] pub newPlayer : unsafe extern "C" fn () -> * mut FilePlayer , # [doc = "Frees the given *player*.\n"] pub freePlayer : unsafe extern "C" fn (player : * mut FilePlayer) , # [doc = "Prepares *player* to stream the file at *path*. Returns 1 if the file exists, otherwise 0.\n"] pub loadIntoPlayer : unsafe extern "C" fn (player : * mut FilePlayer , path : * const core :: ffi :: c_char) -> core :: ffi :: c_int , # [doc = "Sets the buffer length of *player* to *bufferLen* seconds;\n"] pub setBufferLength : unsafe extern "C" fn (player : * mut FilePlayer , bufferLen : core :: ffi :: c_float) , # [doc = "Starts playing the file *player*. If *repeat* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with [playdate-&gt;sound-&gt;fileplayer-&gt;stop()](#f-sound.fileplayer.stop). Returns 1 on success, 0 if buffer allocation failed.\n"] pub play : unsafe extern "C" fn (player : * mut FilePlayer , repeat : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Returns one if *player* is playing, zero if not.\n"] pub isPlaying : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int , # [doc = "Pauses the file *player*.\n"] pub pause : unsafe extern "C" fn (player : * mut FilePlayer) , # [doc = "Stops playing the file.\n"] pub stop : unsafe extern "C" fn (player : * mut FilePlayer) , # [doc = "Sets the playback volume for left and right channels of *player*.\n"] pub setVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float) , # [doc = "Gets the left and right channel playback volume for *player*.\n"] pub getVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : * mut core :: ffi :: c_float , right : * mut core :: ffi :: c_float) , # [doc = "Returns the length, in seconds, of the file loaded into *player*.\n"] pub getLength : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , # [doc = "Sets the current *offset* in seconds.\n"] pub setOffset : unsafe extern "C" fn (player : * mut FilePlayer , offset : core :: ffi :: c_float) , # [doc = "Sets the playback *rate* for the *player*. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate &lt; 0).\n"] pub setRate : unsafe extern "C" fn (player : * mut FilePlayer , rate : core :: ffi :: c_float) , # [doc = "Sets the *start* and *end* of the loop region for playback, in seconds. If *end* is omitted, the end of the file is used.\n"] pub setLoopRange : unsafe extern "C" fn (player : * mut FilePlayer , start : core :: ffi :: c_float , end : core :: ffi :: c_float) , # [doc = "Returns one if *player* has underrun, zero if not.\n"] pub didUnderrun : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int , # [doc = "Sets a function to be called when playback has completed. This is an alias for [playdate→sound→source→setFinishCallback()](#f-sound.source.setFinishCallback).\nsndCallbackProctypedef void sndCallbackProc(SoundSource* c, void* userdata);"] pub setFinishCallback : unsafe extern "C" fn (player : * mut FilePlayer , callback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub setLoopCallback : unsafe extern "C" fn (player : * mut FilePlayer , callback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , # [doc = "Returns the current offset in seconds for *player*.\n"] pub getOffset : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , # [doc = "Returns the playback rate for *player*.\n"] pub getRate : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , # [doc = "If *flag* evaluates to true, the *player* will restart playback (after an audible stutter) as soon as data is available.\n"] pub setStopOnUnderrun : unsafe extern "C" fn (player : * mut FilePlayer , flag : core :: ffi :: c_int) , # [doc = "Changes the volume of the fileplayer to *left* and *right* over a length of *len* sample frames, then calls the provided callback (if set).\n"] pub fadeVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float , len : i32 , finishCallback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub setMP3StreamSource : unsafe extern "C" fn (player : * mut FilePlayer , dataSource : :: core :: option :: Option < unsafe extern "C" fn (data : * mut u8 , bytes : core :: ffi :: c_int , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_int > , userdata : * mut core :: ffi :: c_void , bufferLen : core :: ffi :: c_float) , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundFileplayer"][::core::mem::size_of::<PlaydateSoundFileplayer>() - 88usize];
	["Alignment of PlaydateSoundFileplayer"][::core::mem::align_of::<PlaydateSoundFileplayer>() - 4usize];
	["Offset of field: PlaydateSoundFileplayer::newPlayer"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, newPlayer) - 0usize];
	["Offset of field: PlaydateSoundFileplayer::freePlayer"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, freePlayer) - 4usize];
	["Offset of field: PlaydateSoundFileplayer::loadIntoPlayer"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, loadIntoPlayer) - 8usize];
	["Offset of field: PlaydateSoundFileplayer::setBufferLength"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setBufferLength) - 12usize];
	["Offset of field: PlaydateSoundFileplayer::play"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, play) - 16usize];
	["Offset of field: PlaydateSoundFileplayer::isPlaying"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, isPlaying) - 20usize];
	["Offset of field: PlaydateSoundFileplayer::pause"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, pause) - 24usize];
	["Offset of field: PlaydateSoundFileplayer::stop"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, stop) - 28usize];
	["Offset of field: PlaydateSoundFileplayer::setVolume"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setVolume) - 32usize];
	["Offset of field: PlaydateSoundFileplayer::getVolume"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, getVolume) - 36usize];
	["Offset of field: PlaydateSoundFileplayer::getLength"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, getLength) - 40usize];
	["Offset of field: PlaydateSoundFileplayer::setOffset"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setOffset) - 44usize];
	["Offset of field: PlaydateSoundFileplayer::setRate"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setRate) - 48usize];
	["Offset of field: PlaydateSoundFileplayer::setLoopRange"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setLoopRange) - 52usize];
	["Offset of field: PlaydateSoundFileplayer::didUnderrun"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, didUnderrun) - 56usize];
	["Offset of field: PlaydateSoundFileplayer::setFinishCallback"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setFinishCallback) - 60usize];
	["Offset of field: PlaydateSoundFileplayer::setLoopCallback"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setLoopCallback) - 64usize];
	["Offset of field: PlaydateSoundFileplayer::getOffset"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, getOffset) - 68usize];
	["Offset of field: PlaydateSoundFileplayer::getRate"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, getRate) - 72usize];
	["Offset of field: PlaydateSoundFileplayer::setStopOnUnderrun"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setStopOnUnderrun) - 76usize];
	["Offset of field: PlaydateSoundFileplayer::fadeVolume"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, fadeVolume) - 80usize];
	["Offset of field: PlaydateSoundFileplayer::setMP3StreamSource"]
		[::core::mem::offset_of!(PlaydateSoundFileplayer, setMP3StreamSource) - 84usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct AudioSample {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SamplePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSample {
	#[doc = "Allocates and returns a new AudioSample with a buffer large enough to load a file of *length* bytes.\n"]
	pub newSampleBuffer: unsafe extern "C" fn(byteCount: core::ffi::c_int) -> *mut AudioSample,
	#[doc = "Loads the sound data from the file at *path* into an existing AudioSample, *sample*.\n"]
	pub loadIntoSample:
		unsafe extern "C" fn(sample: *mut AudioSample, path: *const core::ffi::c_char) -> core::ffi::c_int,
	#[doc = "Allocates and returns a new AudioSample, with the sound data loaded in memory. If there is no file at *path*, the function returns null.\n"]
	pub load: unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut AudioSample,
	#[doc = "Returns a new AudioSample referencing the given audio data. If *shouldFreeData* is set, *data* is freed when the sample object is [freed](#f-sound.sample.freeSample). The sample keeps a pointer to the data instead of copying it, so the data must remain valid while the sample is active. *format* is one of the following values:\nSoundFormattypedef enum\n{\n\tkSound8bitMono = 0,\n\tkSound8bitStereo = 1,\n\tkSound16bitMono = 2,\n\tkSound16bitStereo = 3,\n\tkSoundADPCMMono = 4,\n\tkSoundADPCMStereo = 5\n} SoundFormat;```\npd_api_sound.h```\n also provides some helper macros and functions:\n```\n<span>#define</span> SoundFormatIsStereo(f) ((f)&amp;<span>1</span>)\n<span>#define</span> SoundFormatIs16bit(f) ((f)&gt;=kSound16bitMono)\n<span>static</span><span>inline</span> uint32_t SoundFormat_bytesPerFrame(SoundFormat fmt);```\n"]
	pub newSampleFromData: unsafe extern "C" fn(data: *mut u8,
	                                            format: SoundFormat,
	                                            sampleRate: u32,
	                                            byteCount: core::ffi::c_int,
	                                            shouldFreeData: core::ffi::c_int)
	                                            -> *mut AudioSample,
	pub getData: unsafe extern "C" fn(sample: *mut AudioSample,
	                                  data: *mut *mut u8,
	                                  format: *mut SoundFormat,
	                                  sampleRate: *mut u32,
	                                  bytelength: *mut u32),
	#[doc = "Frees the given *sample*. If the sample was created with [playdate→sound→sample→newSampleFromData()](#f-sound.sample.newSampleFromData) and the *shouldFreeData* flag was set, the sample’s source data is also freed.\n"]
	pub freeSample: unsafe extern "C" fn(sample: *mut AudioSample),
	#[doc = "Returns the length, in seconds, of *sample*.\n"]
	pub getLength: unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_float,
	#[doc = "If the sample is ADPCM compressed, decompresses the sample data to 16-bit PCM data. This increases the sample’s memory footprint by 4x and does not affect the quality in any way, but it is necessary if you want to use the sample in a synth or play the file backwards. Returns 1 if successful, 0 if there’s not enough memory for the uncompressed data.\n"]
	pub decompress: unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundSample"][::core::mem::size_of::<PlaydateSoundSample>() - 32usize];
	["Alignment of PlaydateSoundSample"][::core::mem::align_of::<PlaydateSoundSample>() - 4usize];
	["Offset of field: PlaydateSoundSample::newSampleBuffer"]
		[::core::mem::offset_of!(PlaydateSoundSample, newSampleBuffer) - 0usize];
	["Offset of field: PlaydateSoundSample::loadIntoSample"]
		[::core::mem::offset_of!(PlaydateSoundSample, loadIntoSample) - 4usize];
	["Offset of field: PlaydateSoundSample::load"][::core::mem::offset_of!(PlaydateSoundSample, load) - 8usize];
	["Offset of field: PlaydateSoundSample::newSampleFromData"]
		[::core::mem::offset_of!(PlaydateSoundSample, newSampleFromData) - 12usize];
	["Offset of field: PlaydateSoundSample::getData"]
		[::core::mem::offset_of!(PlaydateSoundSample, getData) - 16usize];
	["Offset of field: PlaydateSoundSample::freeSample"]
		[::core::mem::offset_of!(PlaydateSoundSample, freeSample) - 20usize];
	["Offset of field: PlaydateSoundSample::getLength"]
		[::core::mem::offset_of!(PlaydateSoundSample, getLength) - 24usize];
	["Offset of field: PlaydateSoundSample::decompress"]
		[::core::mem::offset_of!(PlaydateSoundSample, decompress) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSampleplayer {
	#[doc = "Allocates and returns a new SamplePlayer.\n"]
	pub newPlayer: unsafe extern "C" fn() -> *mut SamplePlayer,
	#[doc = "Frees the given *player*.\n"]
	pub freePlayer: unsafe extern "C" fn(player: *mut SamplePlayer),
	#[doc = "Assigns *sample* to *player*.\n"]
	pub setSample: unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample),
	#[doc = "Starts playing the sample in *player*.\nIf *repeat* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with [playdate-&gt;sound-&gt;sampleplayer-&gt;stop()](#f-sound.sampleplayer.stop). If negative one, it does ping-pong looping.\n*rate* is the playback rate for the sample; 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.\nReturns 1 on success (which is always, currently).\n"]
	pub play: unsafe extern "C" fn(player: *mut SamplePlayer,
	                               repeat: core::ffi::c_int,
	                               rate: core::ffi::c_float) -> core::ffi::c_int,
	#[doc = "Returns one if *player* is playing a sample, zero if not.\n"]
	pub isPlaying: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_int,
	#[doc = "Stops playing the sample.\n"]
	pub stop: unsafe extern "C" fn(player: *mut SamplePlayer),
	#[doc = "Sets the playback volume for left and right channels.\n"]
	pub setVolume:
		unsafe extern "C" fn(player: *mut SamplePlayer, left: core::ffi::c_float, right: core::ffi::c_float),
	#[doc = "Gets the current left and right channel volume of the sampleplayer.\n"]
	pub getVolume: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                    left: *mut core::ffi::c_float,
	                                    right: *mut core::ffi::c_float),
	#[doc = "Returns the length, in seconds, of the sample assigned to *player*.\n"]
	pub getLength: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	#[doc = "Sets the current *offset* of the SamplePlayer, in seconds.\n"]
	pub setOffset: unsafe extern "C" fn(player: *mut SamplePlayer, offset: core::ffi::c_float),
	#[doc = "Sets the playback *rate* for the *player*. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.\n"]
	pub setRate: unsafe extern "C" fn(player: *mut SamplePlayer, rate: core::ffi::c_float),
	#[doc = "When used with a repeat of -1, does ping-pong looping, with a *start* and *end* position in frames.\n"]
	pub setPlayRange:
		unsafe extern "C" fn(player: *mut SamplePlayer, start: core::ffi::c_int, end: core::ffi::c_int),
	#[doc = "Sets a function to be called when playback has completed. See [sndCallbackProc](#_sndCallbackProc).\n"]
	pub setFinishCallback: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                            callback: SndCallbackProc,
	                                            userdata: *mut core::ffi::c_void),
	pub setLoopCallback: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                          callback: SndCallbackProc,
	                                          userdata: *mut core::ffi::c_void),
	#[doc = "Returns the current offset in seconds for *player*.\n"]
	pub getOffset: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	#[doc = "Returns the playback rate for *player*.\n"]
	pub getRate: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	#[doc = "Pauses or resumes playback.\n"]
	pub setPaused: unsafe extern "C" fn(player: *mut SamplePlayer, flag: core::ffi::c_int),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundSampleplayer"][::core::mem::size_of::<PlaydateSoundSampleplayer>() - 68usize];
	["Alignment of PlaydateSoundSampleplayer"][::core::mem::align_of::<PlaydateSoundSampleplayer>() - 4usize];
	["Offset of field: PlaydateSoundSampleplayer::newPlayer"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, newPlayer) - 0usize];
	["Offset of field: PlaydateSoundSampleplayer::freePlayer"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, freePlayer) - 4usize];
	["Offset of field: PlaydateSoundSampleplayer::setSample"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setSample) - 8usize];
	["Offset of field: PlaydateSoundSampleplayer::play"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, play) - 12usize];
	["Offset of field: PlaydateSoundSampleplayer::isPlaying"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, isPlaying) - 16usize];
	["Offset of field: PlaydateSoundSampleplayer::stop"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, stop) - 20usize];
	["Offset of field: PlaydateSoundSampleplayer::setVolume"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setVolume) - 24usize];
	["Offset of field: PlaydateSoundSampleplayer::getVolume"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, getVolume) - 28usize];
	["Offset of field: PlaydateSoundSampleplayer::getLength"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, getLength) - 32usize];
	["Offset of field: PlaydateSoundSampleplayer::setOffset"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setOffset) - 36usize];
	["Offset of field: PlaydateSoundSampleplayer::setRate"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setRate) - 40usize];
	["Offset of field: PlaydateSoundSampleplayer::setPlayRange"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setPlayRange) - 44usize];
	["Offset of field: PlaydateSoundSampleplayer::setFinishCallback"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setFinishCallback) - 48usize];
	["Offset of field: PlaydateSoundSampleplayer::setLoopCallback"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setLoopCallback) - 52usize];
	["Offset of field: PlaydateSoundSampleplayer::getOffset"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, getOffset) - 56usize];
	["Offset of field: PlaydateSoundSampleplayer::getRate"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, getRate) - 60usize];
	["Offset of field: PlaydateSoundSampleplayer::setPaused"]
		[::core::mem::offset_of!(PlaydateSoundSampleplayer, setPaused) - 64usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SynthSignalValue {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SynthSignal {
	_unused: [u8; 0],
}
pub type SignalStepFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                      ioframes: *mut core::ffi::c_int,
                                                                      ifval: *mut core::ffi::c_float)
                                                                      -> core::ffi::c_float>;
pub type SignalNoteOnFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                        note: MidiNote,
                                                                        vel: core::ffi::c_float,
                                                                        len: core::ffi::c_float)>;
pub type SignalNoteOffFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                         stopped: core::ffi::c_int,
                                                                         offset: core::ffi::c_int)>;
pub type SignalDeallocFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSignal {
	#[doc = "SignalCallbackstypedef float (*signalStepFunc)(void* userdata, int* iosamples, float* ifval);\ntypedef void (*signalNoteOnFunc)(void* userdata, MIDINote note, float vel, float len); // len = -1 for indefinite\ntypedef void (*signalNoteOffFunc)(void* userdata, int stopped, int offset); // stopped = 0 on note release, = 1 when note actually stops playing; offset is # of frames into the current cycle\ntypedef void (*signalDeallocFunc)(void* userdata);Provides a custom implementation for the signal. *signalStepFunc step* is the only required function, returning the value at the end of the current frame. When called, the *ioframes* pointer contains the number of samples until the end of the frame. If the signal needs to provide a value in the middle of the frame (e.g. an LFO that needs to be sample-accurate) it should return the \"interframe\" value in *ifval* and set *iosamples* to the sample offset of the value. The functions are called on the audio render thread, so they should return as quickly as possible.\n"]
	pub newSignal: unsafe extern "C" fn(step: SignalStepFunc,
	                                    noteOn: SignalNoteOnFunc,
	                                    noteOff: SignalNoteOffFunc,
	                                    dealloc: SignalDeallocFunc,
	                                    userdata: *mut core::ffi::c_void)
	                                    -> *mut SynthSignal,
	#[doc = "Frees a signal created with *playdate→sound→signal→newSignal()*.\n"]
	pub freeSignal: unsafe extern "C" fn(signal: *mut SynthSignal),
	#[doc = "Returns the current output value of *signal*. The signal can be a custom signal created with newSignal(), or any of the PDSynthSignal subclasses.\n"]
	pub getValue: unsafe extern "C" fn(signal: *mut SynthSignal) -> core::ffi::c_float,
	#[doc = "Scales the signal’s output by the given factor. The scale is applied before the offset.\n"]
	pub setValueScale: unsafe extern "C" fn(signal: *mut SynthSignal, scale: core::ffi::c_float),
	#[doc = "Offsets the signal’s output by the given amount.\n"]
	pub setValueOffset: unsafe extern "C" fn(signal: *mut SynthSignal, offset: core::ffi::c_float),
	#[doc = "Creates a new PDSynthSignal that tracks a PDSynthSignalValue.\n"]
	pub newSignalForValue: unsafe extern "C" fn(value: *mut SynthSignalValue) -> *mut SynthSignal,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundSignal"][::core::mem::size_of::<PlaydateSoundSignal>() - 24usize];
	["Alignment of PlaydateSoundSignal"][::core::mem::align_of::<PlaydateSoundSignal>() - 4usize];
	["Offset of field: PlaydateSoundSignal::newSignal"]
		[::core::mem::offset_of!(PlaydateSoundSignal, newSignal) - 0usize];
	["Offset of field: PlaydateSoundSignal::freeSignal"]
		[::core::mem::offset_of!(PlaydateSoundSignal, freeSignal) - 4usize];
	["Offset of field: PlaydateSoundSignal::getValue"]
		[::core::mem::offset_of!(PlaydateSoundSignal, getValue) - 8usize];
	["Offset of field: PlaydateSoundSignal::setValueScale"]
		[::core::mem::offset_of!(PlaydateSoundSignal, setValueScale) - 12usize];
	["Offset of field: PlaydateSoundSignal::setValueOffset"]
		[::core::mem::offset_of!(PlaydateSoundSignal, setValueOffset) - 16usize];
	["Offset of field: PlaydateSoundSignal::newSignalForValue"]
		[::core::mem::offset_of!(PlaydateSoundSignal, newSignalForValue) - 20usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LfoType {
	Square = 0,
	Triangle = 1,
	Sine = 2,
	SampleAndHold = 3,
	SawtoothUp = 4,
	SawtoothDown = 5,
	Arpeggiator = 6,
	Function = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SynthLfo {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundLfo { # [doc = "Returns a new LFO object, which can be used to modulate sounds. The *type* argument is one of the following values:\nLFOTypetypedef enum\n{\n\tkLFOTypeSquare,\n\tkLFOTypeTriangle,\n\tkLFOTypeSine,\n\tkLFOTypeSampleAndHold,\n\tkLFOTypeSawtoothUp,\n\tkLFOTypeSawtoothDown,\n\tkLFOTypeArpeggiator,\n\tkLFOTypeFunction\n} LFOType;"] pub newLFO : unsafe extern "C" fn (type_ : LfoType) -> * mut SynthLfo , # [doc = "Frees the LFO.\n"] pub freeLFO : unsafe extern "C" fn (lfo : * mut SynthLfo) , # [doc = "Sets the LFO shape to one of the values given above.\n"] pub setType : unsafe extern "C" fn (lfo : * mut SynthLfo , type_ : LfoType) , # [doc = "Sets the LFO’s rate, in cycles per second.\n"] pub setRate : unsafe extern "C" fn (lfo : * mut SynthLfo , rate : core :: ffi :: c_float) , # [doc = "Sets the LFO’s phase, from 0 to 1.\n"] pub setPhase : unsafe extern "C" fn (lfo : * mut SynthLfo , phase : core :: ffi :: c_float) , # [doc = "Sets the center value for the LFO.\n"] pub setCenter : unsafe extern "C" fn (lfo : * mut SynthLfo , center : core :: ffi :: c_float) , # [doc = "Sets the depth of the LFO.\n"] pub setDepth : unsafe extern "C" fn (lfo : * mut SynthLfo , depth : core :: ffi :: c_float) , # [doc = "Sets the LFO type to arpeggio, where the given values are in half-steps from the center note. For example, the sequence (0, 4, 7, 12) plays the notes of a major chord.\n"] pub setArpeggiation : unsafe extern "C" fn (lfo : * mut SynthLfo , nSteps : core :: ffi :: c_int , steps : * mut core :: ffi :: c_float) , # [doc = "Provides a custom function for LFO values.\n"] pub setFunction : unsafe extern "C" fn (lfo : * mut SynthLfo , lfoFunc : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut SynthLfo , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_float > , userdata : * mut core :: ffi :: c_void , interpolate : core :: ffi :: c_int) , # [doc = "Sets an initial holdoff time for the LFO where the LFO remains at its center value, and a ramp time where the value increases linearly to its maximum depth. Values are in seconds.\n"] pub setDelay : unsafe extern "C" fn (lfo : * mut SynthLfo , holdoff : core :: ffi :: c_float , ramptime : core :: ffi :: c_float) , # [doc = "If retrigger is on, the LFO’s phase is reset to its initial phase (default 0) when a synth using the LFO starts playing a note.\n"] pub setRetrigger : unsafe extern "C" fn (lfo : * mut SynthLfo , flag : core :: ffi :: c_int) , # [doc = "Return the current output value of the LFO.\n"] pub getValue : unsafe extern "C" fn (lfo : * mut SynthLfo) -> core :: ffi :: c_float , # [doc = "If *global* is set, the LFO is continuously updated whether or not it’s currently in use.\n"] pub setGlobal : unsafe extern "C" fn (lfo : * mut SynthLfo , global : core :: ffi :: c_int) , # [doc = "Sets the LFO’s initial phase, from 0 to 1.\n"] pub setStartPhase : unsafe extern "C" fn (lfo : * mut SynthLfo , phase : core :: ffi :: c_float) , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundLfo"][::core::mem::size_of::<PlaydateSoundLfo>() - 56usize];
	["Alignment of PlaydateSoundLfo"][::core::mem::align_of::<PlaydateSoundLfo>() - 4usize];
	["Offset of field: PlaydateSoundLfo::newLFO"][::core::mem::offset_of!(PlaydateSoundLfo, newLFO) - 0usize];
	["Offset of field: PlaydateSoundLfo::freeLFO"][::core::mem::offset_of!(PlaydateSoundLfo, freeLFO) - 4usize];
	["Offset of field: PlaydateSoundLfo::setType"][::core::mem::offset_of!(PlaydateSoundLfo, setType) - 8usize];
	["Offset of field: PlaydateSoundLfo::setRate"][::core::mem::offset_of!(PlaydateSoundLfo, setRate) - 12usize];
	["Offset of field: PlaydateSoundLfo::setPhase"][::core::mem::offset_of!(PlaydateSoundLfo, setPhase) - 16usize];
	["Offset of field: PlaydateSoundLfo::setCenter"]
		[::core::mem::offset_of!(PlaydateSoundLfo, setCenter) - 20usize];
	["Offset of field: PlaydateSoundLfo::setDepth"][::core::mem::offset_of!(PlaydateSoundLfo, setDepth) - 24usize];
	["Offset of field: PlaydateSoundLfo::setArpeggiation"]
		[::core::mem::offset_of!(PlaydateSoundLfo, setArpeggiation) - 28usize];
	["Offset of field: PlaydateSoundLfo::setFunction"]
		[::core::mem::offset_of!(PlaydateSoundLfo, setFunction) - 32usize];
	["Offset of field: PlaydateSoundLfo::setDelay"][::core::mem::offset_of!(PlaydateSoundLfo, setDelay) - 36usize];
	["Offset of field: PlaydateSoundLfo::setRetrigger"]
		[::core::mem::offset_of!(PlaydateSoundLfo, setRetrigger) - 40usize];
	["Offset of field: PlaydateSoundLfo::getValue"][::core::mem::offset_of!(PlaydateSoundLfo, getValue) - 44usize];
	["Offset of field: PlaydateSoundLfo::setGlobal"]
		[::core::mem::offset_of!(PlaydateSoundLfo, setGlobal) - 48usize];
	["Offset of field: PlaydateSoundLfo::setStartPhase"]
		[::core::mem::offset_of!(PlaydateSoundLfo, setStartPhase) - 52usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SynthEnvelope {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEnvelope {
	#[doc = "Creates a new envelope with the given parameters.\n"]
	pub newEnvelope: unsafe extern "C" fn(attack: core::ffi::c_float,
	                                      decay: core::ffi::c_float,
	                                      sustain: core::ffi::c_float,
	                                      release: core::ffi::c_float)
	                                      -> *mut SynthEnvelope,
	#[doc = "Frees the envelope.\n"]
	pub freeEnvelope: unsafe extern "C" fn(env: *mut SynthEnvelope),
	pub setAttack: unsafe extern "C" fn(env: *mut SynthEnvelope, attack: core::ffi::c_float),
	pub setDecay: unsafe extern "C" fn(env: *mut SynthEnvelope, decay: core::ffi::c_float),
	pub setSustain: unsafe extern "C" fn(env: *mut SynthEnvelope, sustain: core::ffi::c_float),
	#[doc = "Sets the ADSR parameters of the envelope.\n"]
	pub setRelease: unsafe extern "C" fn(env: *mut SynthEnvelope, release: core::ffi::c_float),
	#[doc = "Sets whether to use legato phrasing for the envelope. If the legato flag is set, when the envelope is re-triggered before it’s released, it remains in the sustain phase instead of jumping back to the attack phase.\n"]
	pub setLegato: unsafe extern "C" fn(env: *mut SynthEnvelope, flag: core::ffi::c_int),
	#[doc = "If retrigger is on, the envelope always starts from 0 when a note starts playing, instead of the current value if it’s active.\n"]
	pub setRetrigger: unsafe extern "C" fn(lfo: *mut SynthEnvelope, flag: core::ffi::c_int),
	#[doc = "Return the current output value of the envelope.\n"]
	pub getValue: unsafe extern "C" fn(env: *mut SynthEnvelope) -> core::ffi::c_float,
	#[doc = "Smoothly changes the envelope’s shape from linear (amount=0) to exponential (amount=1).\n"]
	pub setCurvature: unsafe extern "C" fn(env: *mut SynthEnvelope, amount: core::ffi::c_float),
	#[doc = "Changes the amount by which note velocity scales output level. At the default value of 1, output is proportional to velocity; at 0 velocity has no effect on output level.\n"]
	pub setVelocitySensitivity: unsafe extern "C" fn(env: *mut SynthEnvelope, velsens: core::ffi::c_float),
	#[doc = "Scales the envelope rate according to the played note. For notes below ```\nstart```\n, the envelope’s set rate is used; for notes above ```\nend```\n envelope rates are scaled by the ```\nscaling```\n parameter. Between the two notes the scaling factor is interpolated from 1.0 to ```\nscaling```\n.\n"]
	pub setRateScaling:
		unsafe extern "C" fn(env: *mut SynthEnvelope, scaling: core::ffi::c_float, start: MidiNote, end: MidiNote),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEnvelope"][::core::mem::size_of::<PlaydateSoundEnvelope>() - 48usize];
	["Alignment of PlaydateSoundEnvelope"][::core::mem::align_of::<PlaydateSoundEnvelope>() - 4usize];
	["Offset of field: PlaydateSoundEnvelope::newEnvelope"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, newEnvelope) - 0usize];
	["Offset of field: PlaydateSoundEnvelope::freeEnvelope"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, freeEnvelope) - 4usize];
	["Offset of field: PlaydateSoundEnvelope::setAttack"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setAttack) - 8usize];
	["Offset of field: PlaydateSoundEnvelope::setDecay"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setDecay) - 12usize];
	["Offset of field: PlaydateSoundEnvelope::setSustain"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setSustain) - 16usize];
	["Offset of field: PlaydateSoundEnvelope::setRelease"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setRelease) - 20usize];
	["Offset of field: PlaydateSoundEnvelope::setLegato"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setLegato) - 24usize];
	["Offset of field: PlaydateSoundEnvelope::setRetrigger"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setRetrigger) - 28usize];
	["Offset of field: PlaydateSoundEnvelope::getValue"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, getValue) - 32usize];
	["Offset of field: PlaydateSoundEnvelope::setCurvature"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setCurvature) - 36usize];
	["Offset of field: PlaydateSoundEnvelope::setVelocitySensitivity"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setVelocitySensitivity) - 40usize];
	["Offset of field: PlaydateSoundEnvelope::setRateScaling"]
		[::core::mem::offset_of!(PlaydateSoundEnvelope, setRateScaling) - 44usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SoundWaveform {
	Square = 0,
	Triangle = 1,
	Sine = 2,
	Noise = 3,
	Sawtooth = 4,
	PoPhase = 5,
	PoDigital = 6,
	PoVosim = 7,
}
pub type SynthRenderFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                       left: *mut i32,
                                                                       right: *mut i32,
                                                                       nsamples: core::ffi::c_int,
                                                                       rate: u32,
                                                                       drate: i32)
                                                                       -> core::ffi::c_int>;
pub type SynthNoteOnFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                       note: MidiNote,
                                                                       velocity: core::ffi::c_float,
                                                                       len: core::ffi::c_float)>;
pub type SynthReleaseFunc =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void, stop: core::ffi::c_int)>;
pub type SynthSetParameterFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                             parameter: core::ffi::c_int,
                                                                             value: core::ffi::c_float)
                                                                             -> core::ffi::c_int>;
pub type SynthDeallocFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
pub type SynthCopyUserdata =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Synth {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSynth {
	#[doc = "Creates a new synth object.\n"]
	pub newSynth: unsafe extern "C" fn() -> *mut Synth,
	#[doc = "Frees a synth object, first removing it from the sound engine if needed.\n"]
	pub freeSynth: unsafe extern "C" fn(synth: *mut Synth),
	#[doc = "Sets the waveform of the synth. The SoundWaveform enum contains the following values:\nSoundWaveformtypedef enum\n{\n\tkWaveformSquare,\n\tkWaveformTriangle,\n\tkWaveformSine,\n\tkWaveformNoise,\n\tkWaveformSawtooth,\n\tkWaveformPOPhase,\n\tkWaveformPODigital,\n\tkWaveformPOVosim\n} SoundWaveform;"]
	pub setWaveform: unsafe extern "C" fn(synth: *mut Synth, wave: SoundWaveform),
	pub setGenerator_deprecated: unsafe extern "C" fn(synth: *mut Synth,
	                                                  stereo: core::ffi::c_int,
	                                                  render: SynthRenderFunc,
	                                                  noteOn: SynthNoteOnFunc,
	                                                  release: SynthReleaseFunc,
	                                                  setparam: SynthSetParameterFunc,
	                                                  dealloc: SynthDeallocFunc,
	                                                  userdata: *mut core::ffi::c_void),
	#[doc = "Provides a sample for the synth to play. Sample data must be uncompressed PCM, not ADPCM. If a sustain range is set, it is looped while the synth is playing a note. When the note ends, if an envelope has been set on the synth and the sustain range goes to the end of the sample (i.e. there’s no release section of the sample after the sustain range) then the sustain section continues looping during the envelope release; otherwise it plays through the end of the sample and stops. As a convenience, if ```\nsustainEnd```\n is zero and ```\nsustainStart```\n is greater than zero, ```\nsustainEnd```\n will be set to the length of the sample.\n"]
	pub setSample:
		unsafe extern "C" fn(synth: *mut Synth, sample: *mut AudioSample, sustainStart: u32, sustainEnd: u32),
	pub setAttackTime: unsafe extern "C" fn(synth: *mut Synth, attack: core::ffi::c_float),
	pub setDecayTime: unsafe extern "C" fn(synth: *mut Synth, decay: core::ffi::c_float),
	pub setSustainLevel: unsafe extern "C" fn(synth: *mut Synth, sustain: core::ffi::c_float),
	#[doc = "Sets the parameters of the synth’s ADSR envelope.\n"]
	pub setReleaseTime: unsafe extern "C" fn(synth: *mut Synth, release: core::ffi::c_float),
	#[doc = "Transposes the synth’s output by the given number of half steps. For example, if the transpose is set to 2 and a C note is played, the synth will output a D instead.\n"]
	pub setTranspose: unsafe extern "C" fn(synth: *mut Synth, halfSteps: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the synth’s frequency. The signal is scaled so that a value of 1 doubles the synth pitch (i.e. an octave up) and -1 halves it (an octave down). Set to *NULL* to clear the modulator.\n"]
	pub setFrequencyModulator: unsafe extern "C" fn(synth: *mut Synth, mod_: *mut SynthSignalValue),
	#[doc = "Returns the currently set frequency modulator.\n"]
	pub getFrequencyModulator: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthSignalValue,
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the synth’s output amplitude. Set to *NULL* to clear the modulator.\n"]
	pub setAmplitudeModulator: unsafe extern "C" fn(synth: *mut Synth, mod_: *mut SynthSignalValue),
	#[doc = "Returns the currently set amplitude modulator.\n"]
	pub getAmplitudeModulator: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthSignalValue,
	#[doc = "Returns the number of parameters advertised by the synth.\n"]
	pub getParameterCount: unsafe extern "C" fn(synth: *mut Synth) -> core::ffi::c_int,
	#[doc = "Sets the (1-based) parameter at position *num* to the given value. Returns 0 if *num* is not a valid parameter index.\n"]
	pub setParameter: unsafe extern "C" fn(synth: *mut Synth,
	                                       parameter: core::ffi::c_int,
	                                       value: core::ffi::c_float)
	                                       -> core::ffi::c_int,
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the parameter at index *num*. Set to *NULL* to clear the modulator.\n"]
	pub setParameterModulator:
		unsafe extern "C" fn(synth: *mut Synth, parameter: core::ffi::c_int, mod_: *mut SynthSignalValue),
	#[doc = "Returns the currently set parameter modulator for the given index.\n"]
	pub getParameterModulator:
		unsafe extern "C" fn(synth: *mut Synth, parameter: core::ffi::c_int) -> *mut SynthSignalValue,
	#[doc = "Plays a note on the synth, at the given frequency. Specify *len* = -1 to leave the note playing until a subsequent noteOff() call. If *when* is 0, the note is played immediately, otherwise the note is scheduled for the given time. Use [playdate→sound→getCurrentTime()](#f-sound.getCurrentTime) to get the current time.\n"]
	pub playNote: unsafe extern "C" fn(synth: *mut Synth,
	                                   freq: core::ffi::c_float,
	                                   vel: core::ffi::c_float,
	                                   len: core::ffi::c_float,
	                                   when: u32),
	#[doc = "The same as [playNote](#f-sound.synth.playNote) but uses MIDI note (where 60 = C4) instead of frequency. Note that ```\nMIDINote```\n is a typedef for `float', meaning fractional values are allowed (for all you microtuning enthusiasts).\n"]
	pub playMIDINote: unsafe extern "C" fn(synth: *mut Synth,
	                                       note: MidiNote,
	                                       vel: core::ffi::c_float,
	                                       len: core::ffi::c_float,
	                                       when: u32),
	#[doc = "Sends a note off event to the synth, either immediately (*when* = 0) or at the scheduled time.\n"]
	pub noteOff: unsafe extern "C" fn(synth: *mut Synth, when: u32),
	pub stop: unsafe extern "C" fn(synth: *mut Synth),
	#[doc = "Sets the playback volume (0.0 - 1.0) for the left and, if the synth is stereo, right channels of the synth. This is equivalent to\nplaydate-&gt;sound-&gt;source-&gt;setVolume((SoundSource*)synth, lvol, rvol);"]
	pub setVolume: unsafe extern "C" fn(synth: *mut Synth, left: core::ffi::c_float, right: core::ffi::c_float),
	#[doc = "Gets the playback volume for the left and right (if stereo) channels of the synth. This is equivalent to\nplaydate-&gt;sound-&gt;source-&gt;getVolume((SoundSource*)synth, outlvol, outrvol);"]
	pub getVolume:
		unsafe extern "C" fn(synth: *mut Synth, left: *mut core::ffi::c_float, right: *mut core::ffi::c_float),
	#[doc = "Returns 1 if the synth is currently playing.\n"]
	pub isPlaying: unsafe extern "C" fn(synth: *mut Synth) -> core::ffi::c_int,
	#[doc = "Returns the synth’s envelope. The PDSynth object owns this envelope, so it must not be freed.\n"]
	pub getEnvelope: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthEnvelope,
	#[doc = "Sets a wavetable for the synth to play. Sample data must be 16-bit mono uncompressed. ```\nlog2size```\n is the base 2 logarithm of the number of samples in each waveform \"cell\" in the table, and ```\ncolumns```\n and ```\nrows```\n gives the number of cells in each direction; for example, if the wavetable is arranged in an 8x8 grid, ```\ncolumns```\n and ```\nrows```\n are both 8 and ```\nlog2size```\n is 6, since 2^6 = 8x8.\nThe function returns 1 on success. If it fails, use [playdate→sound→getError()](#f-sound.getError) to get a human-readable error message.\nThe synth’s \"position\" in the wavetable is set manually with [setParameter()](#f-sound.synth.setParameter) or automated with [setParameterModulator()](#f-sound.synth.setParameterModulator). In some cases it’s easier to use a parameter that matches the waveform position in the table, in others (notably when using envelopes and lfos) it’s more convenient to use a 0-1 scale, so there’s some redundancy here. Parameters are\n- 1: x position, values are from 0 to the table width\n- 2: x position, values are from 0 to 1, parameter is scaled up to table width\nFor 2-D tables (```\nheight```\n &gt; 1):\n- 3: y position, values are from 0 to the table height\n- 4: y position, values are from 0 to 1, parameter is scaled up to table height\n"]
	pub setWavetable: unsafe extern "C" fn(synth: *mut Synth,
	                                       sample: *mut AudioSample,
	                                       log2size: core::ffi::c_int,
	                                       columns: core::ffi::c_int,
	                                       rows: core::ffi::c_int)
	                                       -> core::ffi::c_int,
	#[doc = "GeneratorCallbackstypedef int (*synthRenderFunc)(void* userdata, int32_t* left, int32_t* right, int nsamples, uint32_t rate, int32_t drate);\ntypedef void (*synthNoteOnFunc)(void* userdata, MIDINote note, float velocity, float len); // len == -1 if indefinite\ntypedef void (*synthReleaseFunc)(void* userdata, int endoffset);\ntypedef int (*synthSetParameterFunc)(void* userdata, int parameter, float value);\ntypedef void (*synthDeallocFunc)(void* userdata);\ntypedef void* (*synthCopyUserdata)(void* userdata);Provides custom waveform generator functions for the synth. These functions are called on the audio render thread, so they should return as quickly as possible. *synthRenderFunc*, the data provider callback, is the only required function.\n*synthRenderFunc*: called every audio cycle to get the samples for playback. *left* (and *right* if *setGenerator()* was called with the stereo flag set) are sample buffers in Q8.24 format. *rate* is the amount to change a (Q32) phase accumulator each sample, and *drate* is the amount to change *rate* each sample. Custom synths can ignore this and use the *note* paramter in the noteOn function to handle pitch, but synth→setFrequencyModulator() won’t work as expected.\n*synthNoteOnFunc*: called when the synth receives a note on event. *len* is the length of the note in seconds, or -1 if it’s not known yet when the note will end.\n*synthReleaseFunc*: called when the synth receives a note off event. *endoffset* is how many samples into the current render cycle the note ends, allowing for sample-accurate timing.\n*synthSetParameterFunc*: called when a parameter change is received from [synth→setParameter()](#f-sound.synth.setParameter) or a modulator.\n*synthDeallocFunc*: called when the synth is being dealloced. This function should free anything that was allocated for the synth and also free the *userdata* itself.\n*synthCopyUserdata*: called when [synth→copy()](#f-sound.synth.copy) needs a unique copy of the synth’s userdata. External objects should be retained or copied so that the object isn’t freed while the synth is still using it.\n"]
	pub setGenerator: unsafe extern "C" fn(synth: *mut Synth,
	                                       stereo: core::ffi::c_int,
	                                       render: SynthRenderFunc,
	                                       noteOn: SynthNoteOnFunc,
	                                       release: SynthReleaseFunc,
	                                       setparam: SynthSetParameterFunc,
	                                       dealloc: SynthDeallocFunc,
	                                       copyUserdata: SynthCopyUserdata,
	                                       userdata: *mut core::ffi::c_void),
	#[doc = "Returns a copy of the given synth. Caller assumes ownership of the returned object and should free it when it is no longer in use.\n"]
	pub copy: unsafe extern "C" fn(synth: *mut Synth) -> *mut Synth,
	#[doc = "Clears the synth’s envelope settings.\n"]
	pub clearEnvelope: unsafe extern "C" fn(synth: *mut Synth),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundSynth"][::core::mem::size_of::<PlaydateSoundSynth>() - 120usize];
	["Alignment of PlaydateSoundSynth"][::core::mem::align_of::<PlaydateSoundSynth>() - 4usize];
	["Offset of field: PlaydateSoundSynth::newSynth"]
		[::core::mem::offset_of!(PlaydateSoundSynth, newSynth) - 0usize];
	["Offset of field: PlaydateSoundSynth::freeSynth"]
		[::core::mem::offset_of!(PlaydateSoundSynth, freeSynth) - 4usize];
	["Offset of field: PlaydateSoundSynth::setWaveform"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setWaveform) - 8usize];
	["Offset of field: PlaydateSoundSynth::setGenerator_deprecated"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setGenerator_deprecated) - 12usize];
	["Offset of field: PlaydateSoundSynth::setSample"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setSample) - 16usize];
	["Offset of field: PlaydateSoundSynth::setAttackTime"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setAttackTime) - 20usize];
	["Offset of field: PlaydateSoundSynth::setDecayTime"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setDecayTime) - 24usize];
	["Offset of field: PlaydateSoundSynth::setSustainLevel"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setSustainLevel) - 28usize];
	["Offset of field: PlaydateSoundSynth::setReleaseTime"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setReleaseTime) - 32usize];
	["Offset of field: PlaydateSoundSynth::setTranspose"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setTranspose) - 36usize];
	["Offset of field: PlaydateSoundSynth::setFrequencyModulator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setFrequencyModulator) - 40usize];
	["Offset of field: PlaydateSoundSynth::getFrequencyModulator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, getFrequencyModulator) - 44usize];
	["Offset of field: PlaydateSoundSynth::setAmplitudeModulator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setAmplitudeModulator) - 48usize];
	["Offset of field: PlaydateSoundSynth::getAmplitudeModulator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, getAmplitudeModulator) - 52usize];
	["Offset of field: PlaydateSoundSynth::getParameterCount"]
		[::core::mem::offset_of!(PlaydateSoundSynth, getParameterCount) - 56usize];
	["Offset of field: PlaydateSoundSynth::setParameter"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setParameter) - 60usize];
	["Offset of field: PlaydateSoundSynth::setParameterModulator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setParameterModulator) - 64usize];
	["Offset of field: PlaydateSoundSynth::getParameterModulator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, getParameterModulator) - 68usize];
	["Offset of field: PlaydateSoundSynth::playNote"]
		[::core::mem::offset_of!(PlaydateSoundSynth, playNote) - 72usize];
	["Offset of field: PlaydateSoundSynth::playMIDINote"]
		[::core::mem::offset_of!(PlaydateSoundSynth, playMIDINote) - 76usize];
	["Offset of field: PlaydateSoundSynth::noteOff"]
		[::core::mem::offset_of!(PlaydateSoundSynth, noteOff) - 80usize];
	["Offset of field: PlaydateSoundSynth::stop"][::core::mem::offset_of!(PlaydateSoundSynth, stop) - 84usize];
	["Offset of field: PlaydateSoundSynth::setVolume"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setVolume) - 88usize];
	["Offset of field: PlaydateSoundSynth::getVolume"]
		[::core::mem::offset_of!(PlaydateSoundSynth, getVolume) - 92usize];
	["Offset of field: PlaydateSoundSynth::isPlaying"]
		[::core::mem::offset_of!(PlaydateSoundSynth, isPlaying) - 96usize];
	["Offset of field: PlaydateSoundSynth::getEnvelope"]
		[::core::mem::offset_of!(PlaydateSoundSynth, getEnvelope) - 100usize];
	["Offset of field: PlaydateSoundSynth::setWavetable"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setWavetable) - 104usize];
	["Offset of field: PlaydateSoundSynth::setGenerator"]
		[::core::mem::offset_of!(PlaydateSoundSynth, setGenerator) - 108usize];
	["Offset of field: PlaydateSoundSynth::copy"][::core::mem::offset_of!(PlaydateSoundSynth, copy) - 112usize];
	["Offset of field: PlaydateSoundSynth::clearEnvelope"]
		[::core::mem::offset_of!(PlaydateSoundSynth, clearEnvelope) - 116usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct ControlSignal {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateControlSignal {
	#[doc = "Creates a new control signal object.\n"]
	pub newSignal: unsafe extern "C" fn() -> *mut ControlSignal,
	#[doc = "Frees the given signal.\n"]
	pub freeSignal: unsafe extern "C" fn(signal: *mut ControlSignal),
	#[doc = "Clears all events from the given signal.\n"]
	pub clearEvents: unsafe extern "C" fn(control: *mut ControlSignal),
	#[doc = "Adds a value to the signal’s timeline at the given step. If *interpolate* is set, the value is interpolated between the previous step+value and this one.\n"]
	pub addEvent: unsafe extern "C" fn(control: *mut ControlSignal,
	                                   step: core::ffi::c_int,
	                                   value: core::ffi::c_float,
	                                   interpolate: core::ffi::c_int),
	#[doc = "Removes the control event at the given step.\n"]
	pub removeEvent: unsafe extern "C" fn(control: *mut ControlSignal, step: core::ffi::c_int),
	#[doc = "Returns the MIDI controller number for this ControlSignal, if it was created from a MIDI file via [playdate→sound→sequence→loadMIDIFile()](#f-sound.sequence.loadMIDIFile).\n"]
	pub getMIDIControllerNumber: unsafe extern "C" fn(control: *mut ControlSignal) -> core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateControlSignal"][::core::mem::size_of::<PlaydateControlSignal>() - 24usize];
	["Alignment of PlaydateControlSignal"][::core::mem::align_of::<PlaydateControlSignal>() - 4usize];
	["Offset of field: PlaydateControlSignal::newSignal"]
		[::core::mem::offset_of!(PlaydateControlSignal, newSignal) - 0usize];
	["Offset of field: PlaydateControlSignal::freeSignal"]
		[::core::mem::offset_of!(PlaydateControlSignal, freeSignal) - 4usize];
	["Offset of field: PlaydateControlSignal::clearEvents"]
		[::core::mem::offset_of!(PlaydateControlSignal, clearEvents) - 8usize];
	["Offset of field: PlaydateControlSignal::addEvent"]
		[::core::mem::offset_of!(PlaydateControlSignal, addEvent) - 12usize];
	["Offset of field: PlaydateControlSignal::removeEvent"]
		[::core::mem::offset_of!(PlaydateControlSignal, removeEvent) - 16usize];
	["Offset of field: PlaydateControlSignal::getMIDIControllerNumber"]
		[::core::mem::offset_of!(PlaydateControlSignal, getMIDIControllerNumber) - 20usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SynthInstrument {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundInstrument {
	#[doc = "Creates a new PDSynthInstrument object.\n"]
	pub newInstrument: unsafe extern "C" fn() -> *mut SynthInstrument,
	#[doc = "Frees the given instrument, first removing it from the sound engine if needed.\n"]
	pub freeInstrument: unsafe extern "C" fn(inst: *mut SynthInstrument),
	#[doc = "Adds the given [PDSynth](#C-sound.synth) to the instrument. The synth will respond to playNote events between *rangeState* and *rangeEnd*, inclusive. The *transpose* argument is in half-step units, and is added to the instrument’s [transpose](#f-sound.instrument.setTranspose) parameter. The function returns 1 if successful, or 0 if the synth is already in another instrument or channel.\n"]
	pub addVoice: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                   synth: *mut Synth,
	                                   rangeStart: MidiNote,
	                                   rangeEnd: MidiNote,
	                                   transpose: core::ffi::c_float)
	                                   -> core::ffi::c_int,
	pub playNote: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                   frequency: core::ffi::c_float,
	                                   vel: core::ffi::c_float,
	                                   len: core::ffi::c_float,
	                                   when: u32) -> *mut Synth,
	#[doc = "The instrument passes the playNote/playMIDINote() event to the synth in its collection that has been off for the longest, or has been playing longest if all synths are currently playing. See also [playdate→sound→synth→playNote()](#f-sound.synth.playNote). The PDSynth that received the playNote event is returned.\n"]
	pub playMIDINote: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                       note: MidiNote,
	                                       vel: core::ffi::c_float,
	                                       len: core::ffi::c_float,
	                                       when: u32) -> *mut Synth,
	#[doc = "Sets the pitch bend to be applied to the voices in the instrument, as a fraction of the full range.\n"]
	pub setPitchBend: unsafe extern "C" fn(inst: *mut SynthInstrument, bend: core::ffi::c_float),
	#[doc = "Sets the pitch bend range for the voices in the instrument. The default range is 12, for a full octave.\n"]
	pub setPitchBendRange: unsafe extern "C" fn(inst: *mut SynthInstrument, halfSteps: core::ffi::c_float),
	#[doc = "Sets the transpose parameter for all voices in the instrument.\n"]
	pub setTranspose: unsafe extern "C" fn(inst: *mut SynthInstrument, halfSteps: core::ffi::c_float),
	#[doc = "Forwards the noteOff() event to the synth currently playing the given note. See also [playdate→sound→synth→noteOff()](#f-sound.synth.noteOff).\n"]
	pub noteOff: unsafe extern "C" fn(inst: *mut SynthInstrument, note: MidiNote, when: u32),
	#[doc = "Sends a noteOff event to all voices in the instrument.\n"]
	pub allNotesOff: unsafe extern "C" fn(inst: *mut SynthInstrument, when: u32),
	pub setVolume:
		unsafe extern "C" fn(inst: *mut SynthInstrument, left: core::ffi::c_float, right: core::ffi::c_float),
	#[doc = "Sets and gets the playback volume (0.0 - 1.0) for left and right channels of the synth. This is equivalent to\nplaydate-&gt;sound-&gt;source-&gt;setVolume((SoundSource*)instrument, lvol, rvol);\nplaydate-&gt;sound-&gt;source-&gt;getVolume((SoundSource*)instrument, &amp;lvol, &amp;rvol);"]
	pub getVolume: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                    left: *mut core::ffi::c_float,
	                                    right: *mut core::ffi::c_float),
	#[doc = "Returns the number of voices in the instrument currently playing.\n"]
	pub activeVoiceCount: unsafe extern "C" fn(inst: *mut SynthInstrument) -> core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundInstrument"][::core::mem::size_of::<PlaydateSoundInstrument>() - 52usize];
	["Alignment of PlaydateSoundInstrument"][::core::mem::align_of::<PlaydateSoundInstrument>() - 4usize];
	["Offset of field: PlaydateSoundInstrument::newInstrument"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, newInstrument) - 0usize];
	["Offset of field: PlaydateSoundInstrument::freeInstrument"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, freeInstrument) - 4usize];
	["Offset of field: PlaydateSoundInstrument::addVoice"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, addVoice) - 8usize];
	["Offset of field: PlaydateSoundInstrument::playNote"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, playNote) - 12usize];
	["Offset of field: PlaydateSoundInstrument::playMIDINote"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, playMIDINote) - 16usize];
	["Offset of field: PlaydateSoundInstrument::setPitchBend"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, setPitchBend) - 20usize];
	["Offset of field: PlaydateSoundInstrument::setPitchBendRange"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, setPitchBendRange) - 24usize];
	["Offset of field: PlaydateSoundInstrument::setTranspose"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, setTranspose) - 28usize];
	["Offset of field: PlaydateSoundInstrument::noteOff"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, noteOff) - 32usize];
	["Offset of field: PlaydateSoundInstrument::allNotesOff"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, allNotesOff) - 36usize];
	["Offset of field: PlaydateSoundInstrument::setVolume"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, setVolume) - 40usize];
	["Offset of field: PlaydateSoundInstrument::getVolume"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, getVolume) - 44usize];
	["Offset of field: PlaydateSoundInstrument::activeVoiceCount"]
		[::core::mem::offset_of!(PlaydateSoundInstrument, activeVoiceCount) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SequenceTrack {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundTrack {
	#[doc = "Returns a new SequenceTrack.\n"]
	pub newTrack: unsafe extern "C" fn() -> *mut SequenceTrack,
	#[doc = "Frees the SequenceTrack.\n"]
	pub freeTrack: unsafe extern "C" fn(track: *mut SequenceTrack),
	pub setInstrument: unsafe extern "C" fn(track: *mut SequenceTrack, inst: *mut SynthInstrument),
	#[doc = "Sets or gets the [PDSynthInstrument](#C-sound.PDSynthInstrument) assigned to the track.\n"]
	pub getInstrument: unsafe extern "C" fn(track: *mut SequenceTrack) -> *mut SynthInstrument,
	#[doc = "Adds a single note event to the track.\n"]
	pub addNoteEvent: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                       step: u32,
	                                       len: u32,
	                                       note: MidiNote,
	                                       velocity: core::ffi::c_float),
	#[doc = "Removes the event at *step* playing *note*.\n"]
	pub removeNoteEvent: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32, note: MidiNote),
	#[doc = "Clears all notes from the track.\n"]
	pub clearNotes: unsafe extern "C" fn(track: *mut SequenceTrack),
	#[doc = "Returns the number of [ControlSignal](#C-sound.ControlSignal) objects in the track.\n"]
	pub getControlSignalCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	#[doc = "Returns the [ControlSignal](#C-sound.ControlSignal) at index *idx*.\n"]
	pub getControlSignal:
		unsafe extern "C" fn(track: *mut SequenceTrack, idx: core::ffi::c_int) -> *mut ControlSignal,
	#[doc = "Clears all [ControlSignals](#C-sound.ControlSignal) from the track.\n"]
	pub clearControlEvents: unsafe extern "C" fn(track: *mut SequenceTrack),
	#[doc = "Returns the maximum number of simultaneously playing notes in the track. (Currently, this value is only set when the track was loaded from a MIDI file. We don’t yet track polyphony for user-created events.)\n"]
	pub getPolyphony: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	#[doc = "Returns the number of voices currently playing in the track’s instrument.\n"]
	pub activeVoiceCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	#[doc = "Mutes or unmutes the track.\n"]
	pub setMuted: unsafe extern "C" fn(track: *mut SequenceTrack, mute: core::ffi::c_int),
	#[doc = "Returns the length, in steps, of the track—\u{200b}that is, the step where the last note in the track ends.\n"]
	pub getLength: unsafe extern "C" fn(track: *mut SequenceTrack) -> u32,
	#[doc = "Returns the internal array index for the first note at the given step.\n"]
	pub getIndexForStep: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32) -> core::ffi::c_int,
	#[doc = "If the given index is in range, sets the data in the out pointers and returns 1; otherwise, returns 0.\n"]
	pub getNoteAtIndex: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                         index: core::ffi::c_int,
	                                         outStep: *mut u32,
	                                         outLen: *mut u32,
	                                         outNote: *mut MidiNote,
	                                         outVelocity: *mut core::ffi::c_float)
	                                         -> core::ffi::c_int,
	#[doc = "Returns the [ControlSignal](#C-sound.ControlSignal) for MIDI controller number *controller*, creating it if the **create** flag is set and it doesn’t yet exist.\n"]
	pub getSignalForController: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                                 controller: core::ffi::c_int,
	                                                 create: core::ffi::c_int)
	                                                 -> *mut ControlSignal,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundTrack"][::core::mem::size_of::<PlaydateSoundTrack>() - 68usize];
	["Alignment of PlaydateSoundTrack"][::core::mem::align_of::<PlaydateSoundTrack>() - 4usize];
	["Offset of field: PlaydateSoundTrack::newTrack"]
		[::core::mem::offset_of!(PlaydateSoundTrack, newTrack) - 0usize];
	["Offset of field: PlaydateSoundTrack::freeTrack"]
		[::core::mem::offset_of!(PlaydateSoundTrack, freeTrack) - 4usize];
	["Offset of field: PlaydateSoundTrack::setInstrument"]
		[::core::mem::offset_of!(PlaydateSoundTrack, setInstrument) - 8usize];
	["Offset of field: PlaydateSoundTrack::getInstrument"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getInstrument) - 12usize];
	["Offset of field: PlaydateSoundTrack::addNoteEvent"]
		[::core::mem::offset_of!(PlaydateSoundTrack, addNoteEvent) - 16usize];
	["Offset of field: PlaydateSoundTrack::removeNoteEvent"]
		[::core::mem::offset_of!(PlaydateSoundTrack, removeNoteEvent) - 20usize];
	["Offset of field: PlaydateSoundTrack::clearNotes"]
		[::core::mem::offset_of!(PlaydateSoundTrack, clearNotes) - 24usize];
	["Offset of field: PlaydateSoundTrack::getControlSignalCount"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getControlSignalCount) - 28usize];
	["Offset of field: PlaydateSoundTrack::getControlSignal"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getControlSignal) - 32usize];
	["Offset of field: PlaydateSoundTrack::clearControlEvents"]
		[::core::mem::offset_of!(PlaydateSoundTrack, clearControlEvents) - 36usize];
	["Offset of field: PlaydateSoundTrack::getPolyphony"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getPolyphony) - 40usize];
	["Offset of field: PlaydateSoundTrack::activeVoiceCount"]
		[::core::mem::offset_of!(PlaydateSoundTrack, activeVoiceCount) - 44usize];
	["Offset of field: PlaydateSoundTrack::setMuted"]
		[::core::mem::offset_of!(PlaydateSoundTrack, setMuted) - 48usize];
	["Offset of field: PlaydateSoundTrack::getLength"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getLength) - 52usize];
	["Offset of field: PlaydateSoundTrack::getIndexForStep"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getIndexForStep) - 56usize];
	["Offset of field: PlaydateSoundTrack::getNoteAtIndex"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getNoteAtIndex) - 60usize];
	["Offset of field: PlaydateSoundTrack::getSignalForController"]
		[::core::mem::offset_of!(PlaydateSoundTrack, getSignalForController) - 64usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundSequence {
	_unused: [u8; 0],
}
pub type SequenceFinishedCallback =
	::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSequence {
	pub newSequence: unsafe extern "C" fn() -> *mut SoundSequence,
	#[doc = "Creates or destroys a SoundSequence object.\n"]
	pub freeSequence: unsafe extern "C" fn(sequence: *mut SoundSequence),
	#[doc = "If the sequence is empty, attempts to load data from the MIDI file at *path* into the sequence. Returns 1 on success, 0 on failure.\n"]
	pub loadMIDIFile:
		unsafe extern "C" fn(seq: *mut SoundSequence, path: *const core::ffi::c_char) -> core::ffi::c_int,
	pub getTime: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
	#[doc = "Gets or sets the current time in the sequence, in samples since the start of the file. Note that which step this moves the sequence to depends on the current tempo.\n"]
	pub setTime: unsafe extern "C" fn(seq: *mut SoundSequence, time: u32),
	#[doc = "Sets the looping range of the sequence. If *loops* is 0, the loop repeats endlessly.\n"]
	pub setLoops: unsafe extern "C" fn(seq: *mut SoundSequence,
	                                   loopstart: core::ffi::c_int,
	                                   loopend: core::ffi::c_int,
	                                   loops: core::ffi::c_int),
	pub getTempo_deprecated: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	#[doc = "Sets or gets the tempo of the sequence, in steps per second.\n"]
	pub setTempo: unsafe extern "C" fn(seq: *mut SoundSequence, stepsPerSecond: core::ffi::c_float),
	#[doc = "Returns the number of tracks in the sequence.\n"]
	pub getTrackCount: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	#[doc = "Adds the given [playdate.sound.track](#C-sound.track) to the sequence.\n"]
	pub addTrack: unsafe extern "C" fn(seq: *mut SoundSequence) -> *mut SequenceTrack,
	pub getTrackAtIndex:
		unsafe extern "C" fn(seq: *mut SoundSequence, track: core::ffi::c_uint) -> *mut SequenceTrack,
	#[doc = "Sets or gets the given [SoundTrack](#C-sound.track) object at position *idx* in the sequence.\n"]
	pub setTrackAtIndex:
		unsafe extern "C" fn(seq: *mut SoundSequence, track: *mut SequenceTrack, idx: core::ffi::c_uint),
	#[doc = "Sends a stop signal to all playing notes on all tracks.\n"]
	pub allNotesOff: unsafe extern "C" fn(seq: *mut SoundSequence),
	#[doc = "Returns 1 if the sequence is currently playing, otherwise 0.\n"]
	pub isPlaying: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	#[doc = "Returns the length of the longest track in the sequence, in steps. See also [playdate.sound.track.getLength()](#m-sound.track:getLength).\n"]
	pub getLength: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
	pub play: unsafe extern "C" fn(seq: *mut SoundSequence,
	                               finishCallback: SequenceFinishedCallback,
	                               userdata: *mut core::ffi::c_void),
	#[doc = "Starts or stops playing the sequence. ```\nfinishCallback```\n is an optional function to be called when the sequence finishes playing or is stopped.\nSequenceFinishedCallbacktypedef void (*SequenceFinishedCallback)(SoundSequence* seq, void* userdata);"]
	pub stop: unsafe extern "C" fn(seq: *mut SoundSequence),
	#[doc = "Returns the step number the sequence is currently at. If *timeOffset* is not NULL, its contents are set to the current sample offset within the step.\n"]
	pub getCurrentStep:
		unsafe extern "C" fn(seq: *mut SoundSequence, timeOffset: *mut core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Set the current step for the sequence. *timeOffset* is a sample offset within the step. If *playNotes* is set, notes at the given step (ignoring *timeOffset*) are played.\n"]
	pub setCurrentStep: unsafe extern "C" fn(seq: *mut SoundSequence,
	                                         step: core::ffi::c_int,
	                                         timeOffset: core::ffi::c_int,
	                                         playNotes: core::ffi::c_int),
	pub getTempo: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_float,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundSequence"][::core::mem::size_of::<PlaydateSoundSequence>() - 80usize];
	["Alignment of PlaydateSoundSequence"][::core::mem::align_of::<PlaydateSoundSequence>() - 4usize];
	["Offset of field: PlaydateSoundSequence::newSequence"]
		[::core::mem::offset_of!(PlaydateSoundSequence, newSequence) - 0usize];
	["Offset of field: PlaydateSoundSequence::freeSequence"]
		[::core::mem::offset_of!(PlaydateSoundSequence, freeSequence) - 4usize];
	["Offset of field: PlaydateSoundSequence::loadMIDIFile"]
		[::core::mem::offset_of!(PlaydateSoundSequence, loadMIDIFile) - 8usize];
	["Offset of field: PlaydateSoundSequence::getTime"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getTime) - 12usize];
	["Offset of field: PlaydateSoundSequence::setTime"]
		[::core::mem::offset_of!(PlaydateSoundSequence, setTime) - 16usize];
	["Offset of field: PlaydateSoundSequence::setLoops"]
		[::core::mem::offset_of!(PlaydateSoundSequence, setLoops) - 20usize];
	["Offset of field: PlaydateSoundSequence::getTempo_deprecated"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getTempo_deprecated) - 24usize];
	["Offset of field: PlaydateSoundSequence::setTempo"]
		[::core::mem::offset_of!(PlaydateSoundSequence, setTempo) - 28usize];
	["Offset of field: PlaydateSoundSequence::getTrackCount"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getTrackCount) - 32usize];
	["Offset of field: PlaydateSoundSequence::addTrack"]
		[::core::mem::offset_of!(PlaydateSoundSequence, addTrack) - 36usize];
	["Offset of field: PlaydateSoundSequence::getTrackAtIndex"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getTrackAtIndex) - 40usize];
	["Offset of field: PlaydateSoundSequence::setTrackAtIndex"]
		[::core::mem::offset_of!(PlaydateSoundSequence, setTrackAtIndex) - 44usize];
	["Offset of field: PlaydateSoundSequence::allNotesOff"]
		[::core::mem::offset_of!(PlaydateSoundSequence, allNotesOff) - 48usize];
	["Offset of field: PlaydateSoundSequence::isPlaying"]
		[::core::mem::offset_of!(PlaydateSoundSequence, isPlaying) - 52usize];
	["Offset of field: PlaydateSoundSequence::getLength"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getLength) - 56usize];
	["Offset of field: PlaydateSoundSequence::play"]
		[::core::mem::offset_of!(PlaydateSoundSequence, play) - 60usize];
	["Offset of field: PlaydateSoundSequence::stop"]
		[::core::mem::offset_of!(PlaydateSoundSequence, stop) - 64usize];
	["Offset of field: PlaydateSoundSequence::getCurrentStep"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getCurrentStep) - 68usize];
	["Offset of field: PlaydateSoundSequence::setCurrentStep"]
		[::core::mem::offset_of!(PlaydateSoundSequence, setCurrentStep) - 72usize];
	["Offset of field: PlaydateSoundSequence::getTempo"]
		[::core::mem::offset_of!(PlaydateSoundSequence, getTempo) - 76usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct TwoPoleFilter {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum TwoPoleFilterType {
	LowPass = 0,
	HighPass = 1,
	BandPass = 2,
	Notch = 3,
	Peq = 4,
	LowShelf = 5,
	HighShelf = 6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectTwopolefilter {
	#[doc = "Creates a new two pole filter effect.\n"]
	pub newFilter: unsafe extern "C" fn() -> *mut TwoPoleFilter,
	#[doc = "Frees the given filter.\n"]
	pub freeFilter: unsafe extern "C" fn(filter: *mut TwoPoleFilter),
	#[doc = "TwoPoleFilterTypetypedef enum\n{\n\tkFilterTypeLowPass,\n\tkFilterTypeHighPass,\n\tkFilterTypeBandPass,\n\tkFilterTypeNotch,\n\tkFilterTypePEQ,\n\tkFilterTypeLowShelf,\n\tkFilterTypeHighShelf\n} TwoPoleFilterType;Sets the type of the filter.\n"]
	pub setType: unsafe extern "C" fn(filter: *mut TwoPoleFilter, type_: TwoPoleFilterType),
	#[doc = "Sets the center/corner frequency of the filter. Value is in Hz.\n"]
	pub setFrequency: unsafe extern "C" fn(filter: *mut TwoPoleFilter, frequency: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the effect’s frequency. The signal is scaled so that a value of 1.0 corresponds to half the sample rate. Set to *NULL* to clear the modulator.\n"]
	pub setFrequencyModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut SynthSignalValue),
	#[doc = "Returns the filter’s current frequency modulator.\n"]
	pub getFrequencyModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut SynthSignalValue,
	#[doc = "Sets the filter gain.\n"]
	pub setGain: unsafe extern "C" fn(filter: *mut TwoPoleFilter, gain: core::ffi::c_float),
	#[doc = "Sets the filter resonance.\n"]
	pub setResonance: unsafe extern "C" fn(filter: *mut TwoPoleFilter, resonance: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the filter resonance. Set to *NULL* to clear the modulator.\n"]
	pub setResonanceModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut SynthSignalValue),
	#[doc = "Returns the filter’s current resonance modulator.\n"]
	pub getResonanceModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut SynthSignalValue,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffectTwopolefilter"]
		[::core::mem::size_of::<PlaydateSoundEffectTwopolefilter>() - 40usize];
	["Alignment of PlaydateSoundEffectTwopolefilter"]
		[::core::mem::align_of::<PlaydateSoundEffectTwopolefilter>() - 4usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::newFilter"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, newFilter) - 0usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::freeFilter"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, freeFilter) - 4usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::setType"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, setType) - 8usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::setFrequency"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, setFrequency) - 12usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::setFrequencyModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, setFrequencyModulator) - 16usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::getFrequencyModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, getFrequencyModulator) - 20usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::setGain"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, setGain) - 24usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::setResonance"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, setResonance) - 28usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::setResonanceModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, setResonanceModulator) - 32usize];
	["Offset of field: PlaydateSoundEffectTwopolefilter::getResonanceModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectTwopolefilter, getResonanceModulator) - 36usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct OnePoleFilter {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectOnepolefilter {
	#[doc = "Creates a new one pole filter.\n"]
	pub newFilter: unsafe extern "C" fn() -> *mut OnePoleFilter,
	#[doc = "Frees the filter.\n"]
	pub freeFilter: unsafe extern "C" fn(filter: *mut OnePoleFilter),
	#[doc = "Sets the filter’s single parameter (cutoff frequency) to *p*. Values above 0 (up to 1) are high-pass, values below 0 (down to -1) are low-pass.\n"]
	pub setParameter: unsafe extern "C" fn(filter: *mut OnePoleFilter, parameter: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the filter parameter. Set to *NULL* to clear the modulator.\n"]
	pub setParameterModulator: unsafe extern "C" fn(filter: *mut OnePoleFilter, signal: *mut SynthSignalValue),
	#[doc = "Returns the filter’s current parameter modulator.\n"]
	pub getParameterModulator: unsafe extern "C" fn(filter: *mut OnePoleFilter) -> *mut SynthSignalValue,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffectOnepolefilter"]
		[::core::mem::size_of::<PlaydateSoundEffectOnepolefilter>() - 20usize];
	["Alignment of PlaydateSoundEffectOnepolefilter"]
		[::core::mem::align_of::<PlaydateSoundEffectOnepolefilter>() - 4usize];
	["Offset of field: PlaydateSoundEffectOnepolefilter::newFilter"]
		[::core::mem::offset_of!(PlaydateSoundEffectOnepolefilter, newFilter) - 0usize];
	["Offset of field: PlaydateSoundEffectOnepolefilter::freeFilter"]
		[::core::mem::offset_of!(PlaydateSoundEffectOnepolefilter, freeFilter) - 4usize];
	["Offset of field: PlaydateSoundEffectOnepolefilter::setParameter"]
		[::core::mem::offset_of!(PlaydateSoundEffectOnepolefilter, setParameter) - 8usize];
	["Offset of field: PlaydateSoundEffectOnepolefilter::setParameterModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectOnepolefilter, setParameterModulator) - 12usize];
	["Offset of field: PlaydateSoundEffectOnepolefilter::getParameterModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectOnepolefilter, getParameterModulator) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct BitCrusher {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectBitcrusher {
	#[doc = "Returns a new BitCrusher effect.\n"]
	pub newBitCrusher: unsafe extern "C" fn() -> *mut BitCrusher,
	#[doc = "Frees the given effect.\n"]
	pub freeBitCrusher: unsafe extern "C" fn(filter: *mut BitCrusher),
	#[doc = "Sets the amount of crushing to *amount*. Valid values are 0 (no effect) to 1 (quantizing output to 1-bit).\n"]
	pub setAmount: unsafe extern "C" fn(filter: *mut BitCrusher, amount: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the crushing amount. Set to *NULL* to clear the modulator.\n"]
	pub setAmountModulator: unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut SynthSignalValue),
	#[doc = "Returns the currently set modulator.\n"]
	pub getAmountModulator: unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut SynthSignalValue,
	#[doc = "Sets the number of samples to repeat, quantizing the input in time. A value of 0 produces no undersampling, 1 repeats every other sample, etc.\n"]
	pub setUndersampling: unsafe extern "C" fn(filter: *mut BitCrusher, undersampling: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the undersampling amount. Set to *NULL* to clear the modulator.\n"]
	pub setUndersampleModulator: unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut SynthSignalValue),
	#[doc = "Returns the currently set modulator.\n"]
	pub getUndersampleModulator: unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut SynthSignalValue,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffectBitcrusher"][::core::mem::size_of::<PlaydateSoundEffectBitcrusher>() - 32usize];
	["Alignment of PlaydateSoundEffectBitcrusher"]
		[::core::mem::align_of::<PlaydateSoundEffectBitcrusher>() - 4usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::newBitCrusher"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, newBitCrusher) - 0usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::freeBitCrusher"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, freeBitCrusher) - 4usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::setAmount"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, setAmount) - 8usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::setAmountModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, setAmountModulator) - 12usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::getAmountModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, getAmountModulator) - 16usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::setUndersampling"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, setUndersampling) - 20usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::setUndersampleModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, setUndersampleModulator) - 24usize];
	["Offset of field: PlaydateSoundEffectBitcrusher::getUndersampleModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectBitcrusher, getUndersampleModulator) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct RingModulator {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectRingmodulator {
	#[doc = "Returns a new ring modulator effect.\n"]
	pub newRingmod: unsafe extern "C" fn() -> *mut RingModulator,
	pub freeRingmod: unsafe extern "C" fn(filter: *mut RingModulator),
	#[doc = "Sets the frequency of the modulation signal.\n"]
	pub setFrequency: unsafe extern "C" fn(filter: *mut RingModulator, frequency: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the frequency of the ring modulator. Set to *NULL* to clear the modulator.\n"]
	pub setFrequencyModulator: unsafe extern "C" fn(filter: *mut RingModulator, signal: *mut SynthSignalValue),
	#[doc = "Returns the currently set frequency modulator.\n"]
	pub getFrequencyModulator: unsafe extern "C" fn(filter: *mut RingModulator) -> *mut SynthSignalValue,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffectRingmodulator"]
		[::core::mem::size_of::<PlaydateSoundEffectRingmodulator>() - 20usize];
	["Alignment of PlaydateSoundEffectRingmodulator"]
		[::core::mem::align_of::<PlaydateSoundEffectRingmodulator>() - 4usize];
	["Offset of field: PlaydateSoundEffectRingmodulator::newRingmod"]
		[::core::mem::offset_of!(PlaydateSoundEffectRingmodulator, newRingmod) - 0usize];
	["Offset of field: PlaydateSoundEffectRingmodulator::freeRingmod"]
		[::core::mem::offset_of!(PlaydateSoundEffectRingmodulator, freeRingmod) - 4usize];
	["Offset of field: PlaydateSoundEffectRingmodulator::setFrequency"]
		[::core::mem::offset_of!(PlaydateSoundEffectRingmodulator, setFrequency) - 8usize];
	["Offset of field: PlaydateSoundEffectRingmodulator::setFrequencyModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectRingmodulator, setFrequencyModulator) - 12usize];
	["Offset of field: PlaydateSoundEffectRingmodulator::getFrequencyModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectRingmodulator, getFrequencyModulator) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct DelayLine {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct DelayLineTap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectDelayline {
	#[doc = "Creates a new delay line effect. The *length* parameter is given in samples.\n"]
	pub newDelayLine: unsafe extern "C" fn(length: core::ffi::c_int, stereo: core::ffi::c_int) -> *mut DelayLine,
	#[doc = "Frees the delay line.\n"]
	pub freeDelayLine: unsafe extern "C" fn(filter: *mut DelayLine),
	#[doc = "Changes the length of the delay line, clearing its contents. This function reallocates the audio buffer, so it is not safe to call while the delay line is in use.\n"]
	pub setLength: unsafe extern "C" fn(d: *mut DelayLine, frames: core::ffi::c_int),
	#[doc = "Sets the feedback level of the delay line.\n"]
	pub setFeedback: unsafe extern "C" fn(d: *mut DelayLine, fb: core::ffi::c_float),
	#[doc = "Returns a new tap on the delay line, at the given position. *delay* must be less than or equal to the length of the delay line.\n"]
	pub addTap: unsafe extern "C" fn(d: *mut DelayLine, delay: core::ffi::c_int) -> *mut DelayLineTap,
	#[doc = "Frees a tap previously created with [playdate→sound→delayline→addTap()](#f-sound.effect.delayline.addTap), first removing it from the sound engine if needed.\n"]
	pub freeTap: unsafe extern "C" fn(tap: *mut DelayLineTap),
	#[doc = "Sets the position of the tap on the delay line, up to the delay line’s length.\n"]
	pub setTapDelay: unsafe extern "C" fn(t: *mut DelayLineTap, frames: core::ffi::c_int),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the tap delay. If the signal is continuous (e.g. an envelope or a triangle LFO, but not a square LFO) playback is sped up or slowed down to compress or expand time. Set to *NULL* to clear the modulator.\n"]
	pub setTapDelayModulator: unsafe extern "C" fn(t: *mut DelayLineTap, mod_: *mut SynthSignalValue),
	#[doc = "Returns the current delay modulator.\n"]
	pub getTapDelayModulator: unsafe extern "C" fn(t: *mut DelayLineTap) -> *mut SynthSignalValue,
	#[doc = "If the delay line is stereo and *flip* is set, the tap outputs the delay line’s left channel to its right output and vice versa.\n"]
	pub setTapChannelsFlipped: unsafe extern "C" fn(t: *mut DelayLineTap, flip: core::ffi::c_int),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffectDelayline"][::core::mem::size_of::<PlaydateSoundEffectDelayline>() - 40usize];
	["Alignment of PlaydateSoundEffectDelayline"]
		[::core::mem::align_of::<PlaydateSoundEffectDelayline>() - 4usize];
	["Offset of field: PlaydateSoundEffectDelayline::newDelayLine"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, newDelayLine) - 0usize];
	["Offset of field: PlaydateSoundEffectDelayline::freeDelayLine"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, freeDelayLine) - 4usize];
	["Offset of field: PlaydateSoundEffectDelayline::setLength"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, setLength) - 8usize];
	["Offset of field: PlaydateSoundEffectDelayline::setFeedback"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, setFeedback) - 12usize];
	["Offset of field: PlaydateSoundEffectDelayline::addTap"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, addTap) - 16usize];
	["Offset of field: PlaydateSoundEffectDelayline::freeTap"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, freeTap) - 20usize];
	["Offset of field: PlaydateSoundEffectDelayline::setTapDelay"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, setTapDelay) - 24usize];
	["Offset of field: PlaydateSoundEffectDelayline::setTapDelayModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, setTapDelayModulator) - 28usize];
	["Offset of field: PlaydateSoundEffectDelayline::getTapDelayModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, getTapDelayModulator) - 32usize];
	["Offset of field: PlaydateSoundEffectDelayline::setTapChannelsFlipped"]
		[::core::mem::offset_of!(PlaydateSoundEffectDelayline, setTapChannelsFlipped) - 36usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Overdrive {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectOverdrive {
	#[doc = "Returns a new overdrive effect.\n"]
	pub newOverdrive: unsafe extern "C" fn() -> *mut Overdrive,
	#[doc = "Frees the given effect.\n"]
	pub freeOverdrive: unsafe extern "C" fn(filter: *mut Overdrive),
	#[doc = "Sets the gain of the overdrive effect.\n"]
	pub setGain: unsafe extern "C" fn(o: *mut Overdrive, gain: core::ffi::c_float),
	#[doc = "Sets the level where the amplified input clips.\n"]
	pub setLimit: unsafe extern "C" fn(o: *mut Overdrive, limit: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the limit parameter. Set to *NULL* to clear the modulator.\n"]
	pub setLimitModulator: unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut SynthSignalValue),
	#[doc = "Returns the currently set limit modulator.\n"]
	pub getLimitModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut SynthSignalValue,
	#[doc = "Adds an offset to the upper and lower limits to create an asymmetric clipping.\n"]
	pub setOffset: unsafe extern "C" fn(o: *mut Overdrive, offset: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the offset parameter. Set to *NULL* to clear the modulator.\n"]
	pub setOffsetModulator: unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut SynthSignalValue),
	#[doc = "Returns the currently set offset modulator.\n"]
	pub getOffsetModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut SynthSignalValue,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffectOverdrive"][::core::mem::size_of::<PlaydateSoundEffectOverdrive>() - 36usize];
	["Alignment of PlaydateSoundEffectOverdrive"]
		[::core::mem::align_of::<PlaydateSoundEffectOverdrive>() - 4usize];
	["Offset of field: PlaydateSoundEffectOverdrive::newOverdrive"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, newOverdrive) - 0usize];
	["Offset of field: PlaydateSoundEffectOverdrive::freeOverdrive"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, freeOverdrive) - 4usize];
	["Offset of field: PlaydateSoundEffectOverdrive::setGain"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, setGain) - 8usize];
	["Offset of field: PlaydateSoundEffectOverdrive::setLimit"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, setLimit) - 12usize];
	["Offset of field: PlaydateSoundEffectOverdrive::setLimitModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, setLimitModulator) - 16usize];
	["Offset of field: PlaydateSoundEffectOverdrive::getLimitModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, getLimitModulator) - 20usize];
	["Offset of field: PlaydateSoundEffectOverdrive::setOffset"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, setOffset) - 24usize];
	["Offset of field: PlaydateSoundEffectOverdrive::setOffsetModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, setOffsetModulator) - 28usize];
	["Offset of field: PlaydateSoundEffectOverdrive::getOffsetModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffectOverdrive, getOffsetModulator) - 32usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundEffect {
	_unused: [u8; 0],
}
pub type EffectProc = ::core::option::Option<unsafe extern "C" fn(e: *mut SoundEffect,
                                                                  left: *mut i32,
                                                                  right: *mut i32,
                                                                  nsamples: core::ffi::c_int,
                                                                  bufactive: core::ffi::c_int)
                                                                  -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffect {
	#[doc = "effectProctypedef int effectProc(SoundEffect* e, int32_t* left, int32_t* right, int nsamples, int bufactive);Creates a new effect using the given processing function. *bufactive* is 1 if samples have been set in the left or right buffers. The function should return 1 if it changed the buffer samples, otherwise 0. *left* and *right* (if the effect is on a stereo channel) are sample buffers in Q8.24 format.\n"]
	pub newEffect: unsafe extern "C" fn(proc_: EffectProc, userdata: *mut core::ffi::c_void) -> *mut SoundEffect,
	#[doc = "Frees the given effect.\n"]
	pub freeEffect: unsafe extern "C" fn(effect: *mut SoundEffect),
	#[doc = "Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix (which is useful if you’re using a delay line with taps and don’t want to hear the delay line itself).\n"]
	pub setMix: unsafe extern "C" fn(effect: *mut SoundEffect, level: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the effect’s mix level. Set to *NULL* to clear the modulator.\n"]
	pub setMixModulator: unsafe extern "C" fn(effect: *mut SoundEffect, signal: *mut SynthSignalValue),
	#[doc = "Returns the current mix modulator for the effect.\n"]
	pub getMixModulator: unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut SynthSignalValue,
	pub setUserdata: unsafe extern "C" fn(effect: *mut SoundEffect, userdata: *mut core::ffi::c_void),
	#[doc = "Sets or gets a userdata value for the effect.\n"]
	pub getUserdata: unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut core::ffi::c_void,
	pub twopolefilter: &'static PlaydateSoundEffectTwopolefilter,
	pub onepolefilter: &'static PlaydateSoundEffectOnepolefilter,
	pub bitcrusher: &'static PlaydateSoundEffectBitcrusher,
	pub ringmodulator: &'static PlaydateSoundEffectRingmodulator,
	pub delayline: &'static PlaydateSoundEffectDelayline,
	pub overdrive: &'static PlaydateSoundEffectOverdrive,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundEffect"][::core::mem::size_of::<PlaydateSoundEffect>() - 52usize];
	["Alignment of PlaydateSoundEffect"][::core::mem::align_of::<PlaydateSoundEffect>() - 4usize];
	["Offset of field: PlaydateSoundEffect::newEffect"]
		[::core::mem::offset_of!(PlaydateSoundEffect, newEffect) - 0usize];
	["Offset of field: PlaydateSoundEffect::freeEffect"]
		[::core::mem::offset_of!(PlaydateSoundEffect, freeEffect) - 4usize];
	["Offset of field: PlaydateSoundEffect::setMix"]
		[::core::mem::offset_of!(PlaydateSoundEffect, setMix) - 8usize];
	["Offset of field: PlaydateSoundEffect::setMixModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffect, setMixModulator) - 12usize];
	["Offset of field: PlaydateSoundEffect::getMixModulator"]
		[::core::mem::offset_of!(PlaydateSoundEffect, getMixModulator) - 16usize];
	["Offset of field: PlaydateSoundEffect::setUserdata"]
		[::core::mem::offset_of!(PlaydateSoundEffect, setUserdata) - 20usize];
	["Offset of field: PlaydateSoundEffect::getUserdata"]
		[::core::mem::offset_of!(PlaydateSoundEffect, getUserdata) - 24usize];
	["Offset of field: PlaydateSoundEffect::twopolefilter"]
		[::core::mem::offset_of!(PlaydateSoundEffect, twopolefilter) - 28usize];
	["Offset of field: PlaydateSoundEffect::onepolefilter"]
		[::core::mem::offset_of!(PlaydateSoundEffect, onepolefilter) - 32usize];
	["Offset of field: PlaydateSoundEffect::bitcrusher"]
		[::core::mem::offset_of!(PlaydateSoundEffect, bitcrusher) - 36usize];
	["Offset of field: PlaydateSoundEffect::ringmodulator"]
		[::core::mem::offset_of!(PlaydateSoundEffect, ringmodulator) - 40usize];
	["Offset of field: PlaydateSoundEffect::delayline"]
		[::core::mem::offset_of!(PlaydateSoundEffect, delayline) - 44usize];
	["Offset of field: PlaydateSoundEffect::overdrive"]
		[::core::mem::offset_of!(PlaydateSoundEffect, overdrive) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundChannel {
	_unused: [u8; 0],
}
pub type AudioSourceFunction = ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void,
                                                                           left: *mut i16,
                                                                           right: *mut i16,
                                                                           len: core::ffi::c_int)
                                                                           -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundChannel {
	#[doc = "Returns a new *SoundChannel* object.\n"]
	pub newChannel: unsafe extern "C" fn() -> *mut SoundChannel,
	#[doc = "Frees the given *SoundChannel*.\n"]
	pub freeChannel: unsafe extern "C" fn(channel: *mut SoundChannel),
	#[doc = "Adds a [SoundSource](#f-sound.source) to the channel. If a source is not assigned to a channel, it plays on the default global channel.\n"]
	pub addSource: unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> core::ffi::c_int,
	#[doc = "Removes a [SoundSource](#f-sound.source) to the channel. Returns 1 if the source was found in (and removed from) the channel, otherwise 0.\n"]
	pub removeSource:
		unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> core::ffi::c_int,
	#[doc = "Creates a new [SoundSource](#f-sound.source) using the given data provider callback and adds it to the channel.\nAudioSourceFunctionint AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)This function should fill the passed-in *left* buffer (and *right* if it’s a stereo source) with *len* samples each and return 1, or return 0 if the source is silent through the cycle. The caller takes ownership of the allocated SoundSource, and should free it with\nplaydate-&gt;system-&gt;realloc(source, 0);when it is not longer in use.\n"]
	pub addCallbackSource: unsafe extern "C" fn(channel: *mut SoundChannel,
	                                            callback: AudioSourceFunction,
	                                            context: *mut core::ffi::c_void,
	                                            stereo: core::ffi::c_int)
	                                            -> *mut SoundSource,
	#[doc = "Adds a [SoundEffect](#f-sound.effect) to the channel.\n"]
	pub addEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
	#[doc = "Removes a [SoundEffect](#f-sound.effect) from the channel.\n"]
	pub removeEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
	#[doc = "Sets the volume for the channel, in the range [0-1].\n"]
	pub setVolume: unsafe extern "C" fn(channel: *mut SoundChannel, volume: core::ffi::c_float),
	#[doc = "Gets the volume for the channel, in the range [0-1].\n"]
	pub getVolume: unsafe extern "C" fn(channel: *mut SoundChannel) -> core::ffi::c_float,
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the channel volume. Set to *NULL* to clear the modulator.\n"]
	pub setVolumeModulator: unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut SynthSignalValue),
	#[doc = "Gets a [signal](#C-sound.signal) modulating the channel volume.\n"]
	pub getVolumeModulator: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	#[doc = "Sets the pan parameter for the channel. Valid values are in the range [-1,1], where -1 is left, 0 is center, and 1 is right.\n"]
	pub setPan: unsafe extern "C" fn(channel: *mut SoundChannel, pan: core::ffi::c_float),
	#[doc = "Sets a [signal](#C-sound.signal) to modulate the channel pan. Set to *NULL* to clear the modulator.\n"]
	pub setPanModulator: unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut SynthSignalValue),
	#[doc = "Gets a [signal](#C-sound.signal) modulating the channel pan.\n"]
	pub getPanModulator: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	#[doc = "Returns a signal that follows the volume of the channel before effects are applied.\n"]
	pub getDryLevelSignal: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	#[doc = "Returns a signal that follows the volume of the channel after effects are applied.\n"]
	pub getWetLevelSignal: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSoundChannel"][::core::mem::size_of::<PlaydateSoundChannel>() - 64usize];
	["Alignment of PlaydateSoundChannel"][::core::mem::align_of::<PlaydateSoundChannel>() - 4usize];
	["Offset of field: PlaydateSoundChannel::newChannel"]
		[::core::mem::offset_of!(PlaydateSoundChannel, newChannel) - 0usize];
	["Offset of field: PlaydateSoundChannel::freeChannel"]
		[::core::mem::offset_of!(PlaydateSoundChannel, freeChannel) - 4usize];
	["Offset of field: PlaydateSoundChannel::addSource"]
		[::core::mem::offset_of!(PlaydateSoundChannel, addSource) - 8usize];
	["Offset of field: PlaydateSoundChannel::removeSource"]
		[::core::mem::offset_of!(PlaydateSoundChannel, removeSource) - 12usize];
	["Offset of field: PlaydateSoundChannel::addCallbackSource"]
		[::core::mem::offset_of!(PlaydateSoundChannel, addCallbackSource) - 16usize];
	["Offset of field: PlaydateSoundChannel::addEffect"]
		[::core::mem::offset_of!(PlaydateSoundChannel, addEffect) - 20usize];
	["Offset of field: PlaydateSoundChannel::removeEffect"]
		[::core::mem::offset_of!(PlaydateSoundChannel, removeEffect) - 24usize];
	["Offset of field: PlaydateSoundChannel::setVolume"]
		[::core::mem::offset_of!(PlaydateSoundChannel, setVolume) - 28usize];
	["Offset of field: PlaydateSoundChannel::getVolume"]
		[::core::mem::offset_of!(PlaydateSoundChannel, getVolume) - 32usize];
	["Offset of field: PlaydateSoundChannel::setVolumeModulator"]
		[::core::mem::offset_of!(PlaydateSoundChannel, setVolumeModulator) - 36usize];
	["Offset of field: PlaydateSoundChannel::getVolumeModulator"]
		[::core::mem::offset_of!(PlaydateSoundChannel, getVolumeModulator) - 40usize];
	["Offset of field: PlaydateSoundChannel::setPan"]
		[::core::mem::offset_of!(PlaydateSoundChannel, setPan) - 44usize];
	["Offset of field: PlaydateSoundChannel::setPanModulator"]
		[::core::mem::offset_of!(PlaydateSoundChannel, setPanModulator) - 48usize];
	["Offset of field: PlaydateSoundChannel::getPanModulator"]
		[::core::mem::offset_of!(PlaydateSoundChannel, getPanModulator) - 52usize];
	["Offset of field: PlaydateSoundChannel::getDryLevelSignal"]
		[::core::mem::offset_of!(PlaydateSoundChannel, getDryLevelSignal) - 56usize];
	["Offset of field: PlaydateSoundChannel::getWetLevelSignal"]
		[::core::mem::offset_of!(PlaydateSoundChannel, getWetLevelSignal) - 60usize];
};
pub type RecordCallback = ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void,
                                                                      buffer: *mut i16,
                                                                      length: core::ffi::c_int)
                                                                      -> core::ffi::c_int>;
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum MicSource {
	Autodetect = 0,
	Internal = 1,
	Headset = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSound { pub channel : & 'static PlaydateSoundChannel , pub fileplayer : & 'static PlaydateSoundFileplayer , pub sample : & 'static PlaydateSoundSample , pub sampleplayer : & 'static PlaydateSoundSampleplayer , pub synth : & 'static PlaydateSoundSynth , pub sequence : & 'static PlaydateSoundSequence , pub effect : & 'static PlaydateSoundEffect , pub lfo : & 'static PlaydateSoundLfo , pub envelope : & 'static PlaydateSoundEnvelope , pub source : & 'static PlaydateSoundSource , pub controlsignal : & 'static PlaydateControlSignal , pub track : & 'static PlaydateSoundTrack , pub instrument : & 'static PlaydateSoundInstrument , # [doc = "Returns the sound engine’s current time value, in units of sample frames, 44,100 per second.\nEquivalent to [```\nplaydate.sound.getCurrentTime()```\n](<./Inside Playdate.html#f-sound.getCurrentTime>) in the Lua API.\n"] pub getCurrentTime : unsafe extern "C" fn () -> u32 , # [doc = "The *callback* function you pass in will be called every audio render cycle.\nAudioSourceFunctionint AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)This function should fill the passed-in *left* buffer (and *right* if it’s a stereo source) with *len* samples each and return 1, or return 0 if the source is silent through the cycle.\n"] pub addSource : unsafe extern "C" fn (callback : AudioSourceFunction , context : * mut core :: ffi :: c_void , stereo : core :: ffi :: c_int) -> * mut SoundSource , # [doc = "Returns the default channel, where sound sources play if they haven’t been explicity assigned to a different channel.\n"] pub getDefaultChannel : unsafe extern "C" fn () -> * mut SoundChannel , # [doc = "Adds the given channel to the sound engine. Returns 1 if the channel was added, 0 if it was already in the engine.\n"] pub addChannel : unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int , # [doc = "Removes the given channel from the sound engine. Returns 1 if the channel was successfully removed, 0 if the channel is the default channel or hadn’t been previously added.\n"] pub removeChannel : unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int , # [doc = "The *callback* you pass in will be called every audio cycle.\nAudioInputFunctionint AudioInputFunction(void* context, int16_t* data, int len)enum MicSourceenum MicSource {\n\tkMicInputAutodetect = 0,\n\tkMicInputInternal = 1,\n\tkMicInputHeadset = 2\n};Your input callback will be called with the recorded audio data, a monophonic stream of samples. The function should return 1 to continue recording, 0 to stop recording.\nThe Playdate hardware has a circuit that attempts to autodetect the presence of a headset mic, but there are cases where you may want to override this. For instance, if you’re using a headphone splitter to wire an external source to the mic input, the detector may not always see the input. Setting the source to ```\nkMicInputHeadset```\n forces recording from the headset. Using ```\nkMicInputInternal```\n records from the device mic even when a headset with a mic is plugged in. And ```\nkMicInputAutodetect```\n uses a headset mic if one is detected, otherwise the device microphone. ```\nsetMicCallback()```\n returns which source the function used, internal or headset, or 0 on error.\n"] pub setMicCallback : unsafe extern "C" fn (callback : RecordCallback , context : * mut core :: ffi :: c_void , source : MicSource) -> core :: ffi :: c_int , # [doc = "If *headphone* contains a pointer to an int, the value is set to 1 if headphones are currently plugged in. Likewise, *mic* is set if the headphones include a microphone. If *changeCallback* is provided, it will be called when the headset or mic status changes, and audio output will **not** automatically switch from speaker to headphones when headphones are plugged in (and vice versa). In this case, the callback should use ```\nplaydate→sound→setOutputsActive()```\n to change the output if needed.\nEquivalent to [```\nplaydate.sound.getHeadphoneState()```\n](<./Inside Playdate.html#f-sound.getHeadphoneState>) in the Lua API.\n"] pub getHeadphoneState : unsafe extern "C" fn (headphone : * mut core :: ffi :: c_int , headsetmic : * mut core :: ffi :: c_int , changeCallback : :: core :: option :: Option < unsafe extern "C" fn (headphone : core :: ffi :: c_int , mic : core :: ffi :: c_int) >) , # [doc = "Force audio output to the given outputs, regardless of headphone status.\nEquivalent to [```\nplaydate.sound.setOutputsActive()```\n](<./Inside Playdate.html#f-sound.setOutputsActive>) in the Lua API.\n"] pub setOutputsActive : unsafe extern "C" fn (headphone : core :: ffi :: c_int , speaker : core :: ffi :: c_int) , # [doc = "Removes the given [SoundSource](#C-sound.source) object from its channel, whether it’s in the default channel or a channel created with [playdate→sound→addChannel()](#f-sound.addChannel). Returns 1 if a source was removed, 0 if the source wasn’t in a channel.\n"] pub removeSource : unsafe extern "C" fn (source : * mut SoundSource) -> core :: ffi :: c_int , pub signal : & 'static PlaydateSoundSignal , pub getError : unsafe extern "C" fn () -> * const core :: ffi :: c_char , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateSound"][::core::mem::size_of::<PlaydateSound>() - 96usize];
	["Alignment of PlaydateSound"][::core::mem::align_of::<PlaydateSound>() - 4usize];
	["Offset of field: PlaydateSound::channel"][::core::mem::offset_of!(PlaydateSound, channel) - 0usize];
	["Offset of field: PlaydateSound::fileplayer"][::core::mem::offset_of!(PlaydateSound, fileplayer) - 4usize];
	["Offset of field: PlaydateSound::sample"][::core::mem::offset_of!(PlaydateSound, sample) - 8usize];
	["Offset of field: PlaydateSound::sampleplayer"]
		[::core::mem::offset_of!(PlaydateSound, sampleplayer) - 12usize];
	["Offset of field: PlaydateSound::synth"][::core::mem::offset_of!(PlaydateSound, synth) - 16usize];
	["Offset of field: PlaydateSound::sequence"][::core::mem::offset_of!(PlaydateSound, sequence) - 20usize];
	["Offset of field: PlaydateSound::effect"][::core::mem::offset_of!(PlaydateSound, effect) - 24usize];
	["Offset of field: PlaydateSound::lfo"][::core::mem::offset_of!(PlaydateSound, lfo) - 28usize];
	["Offset of field: PlaydateSound::envelope"][::core::mem::offset_of!(PlaydateSound, envelope) - 32usize];
	["Offset of field: PlaydateSound::source"][::core::mem::offset_of!(PlaydateSound, source) - 36usize];
	["Offset of field: PlaydateSound::controlsignal"]
		[::core::mem::offset_of!(PlaydateSound, controlsignal) - 40usize];
	["Offset of field: PlaydateSound::track"][::core::mem::offset_of!(PlaydateSound, track) - 44usize];
	["Offset of field: PlaydateSound::instrument"][::core::mem::offset_of!(PlaydateSound, instrument) - 48usize];
	["Offset of field: PlaydateSound::getCurrentTime"]
		[::core::mem::offset_of!(PlaydateSound, getCurrentTime) - 52usize];
	["Offset of field: PlaydateSound::addSource"][::core::mem::offset_of!(PlaydateSound, addSource) - 56usize];
	["Offset of field: PlaydateSound::getDefaultChannel"]
		[::core::mem::offset_of!(PlaydateSound, getDefaultChannel) - 60usize];
	["Offset of field: PlaydateSound::addChannel"][::core::mem::offset_of!(PlaydateSound, addChannel) - 64usize];
	["Offset of field: PlaydateSound::removeChannel"]
		[::core::mem::offset_of!(PlaydateSound, removeChannel) - 68usize];
	["Offset of field: PlaydateSound::setMicCallback"]
		[::core::mem::offset_of!(PlaydateSound, setMicCallback) - 72usize];
	["Offset of field: PlaydateSound::getHeadphoneState"]
		[::core::mem::offset_of!(PlaydateSound, getHeadphoneState) - 76usize];
	["Offset of field: PlaydateSound::setOutputsActive"]
		[::core::mem::offset_of!(PlaydateSound, setOutputsActive) - 80usize];
	["Offset of field: PlaydateSound::removeSource"]
		[::core::mem::offset_of!(PlaydateSound, removeSource) - 84usize];
	["Offset of field: PlaydateSound::signal"][::core::mem::offset_of!(PlaydateSound, signal) - 88usize];
	["Offset of field: PlaydateSound::getError"][::core::mem::offset_of!(PlaydateSound, getError) - 92usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateDisplay {
	#[doc = "Returns the width of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 200 instead of 400.\nEquivalent to [```\nplaydate.display.getWidth()```\n](<./Inside Playdate.html#f-display.getWidth>) in the Lua API.\n"]
	pub getWidth: unsafe extern "C" fn() -> core::ffi::c_int,
	#[doc = "Returns the height of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 120 instead of 240.\nEquivalent to [```\nplaydate.display.getHeight()```\n](<./Inside Playdate.html#f-display.getHeight>) in the Lua API.\n"]
	pub getHeight: unsafe extern "C" fn() -> core::ffi::c_int,
	#[doc = "Sets the nominal refresh rate in frames per second. The default is 30 fps, which is a recommended figure that balances animation smoothness with performance and power considerations. Maximum is 50 fps.\nIf *rate* is 0, the game’s update callback (either Lua’s ```\nplaydate.update()```\n or the function specified by [playdate→system→setUpdateCallback()](#f-system.setUpdateCallback)) is called as soon as possible. Since the display refreshes line-by-line, and unchanged lines aren’t sent to the display, the update cycle will be faster than 30 times a second but at an indeterminate rate.\nEquivalent to [```\nplaydate.display.setRefreshRate()```\n](<./Inside Playdate.html#f-display.setRefreshRate>) in the Lua API.\n"]
	pub setRefreshRate: unsafe extern "C" fn(rate: core::ffi::c_float),
	#[doc = "If *flag* evaluates to true, the frame buffer is drawn inverted—black instead of white, and vice versa.\nEquivalent to [```\nplaydate.display.setInverted()```\n](<./Inside Playdate.html#f-display.setInverted>) in the Lua API.\n"]
	pub setInverted: unsafe extern "C" fn(flag: core::ffi::c_int),
	#[doc = "Sets the display scale factor. Valid values for *scale* are 1, 2, 4, and 8.\nThe top-left corner of the frame buffer is scaled up to fill the display; e.g., if the scale is set to 4, the pixels in rectangle [0,100] x [0,60] are drawn on the screen as 4 x 4 squares.\nEquivalent to [```\nplaydate.display.setScale()```\n](<./Inside Playdate.html#f-display.setScale>) in the Lua API.\n"]
	pub setScale: unsafe extern "C" fn(s: core::ffi::c_uint),
	#[doc = "Adds a mosaic effect to the display. Valid *x* and *y* values are between 0 and 3, inclusive.\nEquivalent to [```\nplaydate.display.setMosaic```\n](<./Inside Playdate.html#f-display.setMosaic>) in the Lua API.\n"]
	pub setMosaic: unsafe extern "C" fn(x: core::ffi::c_uint, y: core::ffi::c_uint),
	#[doc = "Flips the display on the x or y axis, or both.\nEquivalent to [```\nplaydate.display.setFlipped()```\n](<./Inside Playdate.html#f-display.setFlipped>) in the Lua API.\n"]
	pub setFlipped: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
	#[doc = "Offsets the display by the given amount. Areas outside of the displayed area are filled with the current [background color](#f-graphics.setBackgroundColor).\nEquivalent to [```\nplaydate.display.setOffset()```\n](<./Inside Playdate.html#f-display.setOffset>) in the Lua API.\n"]
	pub setOffset: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateDisplay"][::core::mem::size_of::<PlaydateDisplay>() - 32usize];
	["Alignment of PlaydateDisplay"][::core::mem::align_of::<PlaydateDisplay>() - 4usize];
	["Offset of field: PlaydateDisplay::getWidth"][::core::mem::offset_of!(PlaydateDisplay, getWidth) - 0usize];
	["Offset of field: PlaydateDisplay::getHeight"][::core::mem::offset_of!(PlaydateDisplay, getHeight) - 4usize];
	["Offset of field: PlaydateDisplay::setRefreshRate"]
		[::core::mem::offset_of!(PlaydateDisplay, setRefreshRate) - 8usize];
	["Offset of field: PlaydateDisplay::setInverted"]
		[::core::mem::offset_of!(PlaydateDisplay, setInverted) - 12usize];
	["Offset of field: PlaydateDisplay::setScale"][::core::mem::offset_of!(PlaydateDisplay, setScale) - 16usize];
	["Offset of field: PlaydateDisplay::setMosaic"][::core::mem::offset_of!(PlaydateDisplay, setMosaic) - 20usize];
	["Offset of field: PlaydateDisplay::setFlipped"]
		[::core::mem::offset_of!(PlaydateDisplay, setFlipped) - 24usize];
	["Offset of field: PlaydateDisplay::setOffset"][::core::mem::offset_of!(PlaydateDisplay, setOffset) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct Score {
	pub rank: u32,
	pub value: u32,
	pub player: *mut core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of Score"][::core::mem::size_of::<Score>() - 12usize];
	["Alignment of Score"][::core::mem::align_of::<Score>() - 4usize];
	["Offset of field: Score::rank"][::core::mem::offset_of!(Score, rank) - 0usize];
	["Offset of field: Score::value"][::core::mem::offset_of!(Score, value) - 4usize];
	["Offset of field: Score::player"][::core::mem::offset_of!(Score, player) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct ScoresList {
	pub boardID: *mut core::ffi::c_char,
	pub count: core::ffi::c_uint,
	pub lastUpdated: u32,
	pub playerIncluded: core::ffi::c_int,
	pub limit: core::ffi::c_uint,
	pub scores: *mut Score,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of ScoresList"][::core::mem::size_of::<ScoresList>() - 24usize];
	["Alignment of ScoresList"][::core::mem::align_of::<ScoresList>() - 4usize];
	["Offset of field: ScoresList::boardID"][::core::mem::offset_of!(ScoresList, boardID) - 0usize];
	["Offset of field: ScoresList::count"][::core::mem::offset_of!(ScoresList, count) - 4usize];
	["Offset of field: ScoresList::lastUpdated"][::core::mem::offset_of!(ScoresList, lastUpdated) - 8usize];
	["Offset of field: ScoresList::playerIncluded"][::core::mem::offset_of!(ScoresList, playerIncluded) - 12usize];
	["Offset of field: ScoresList::limit"][::core::mem::offset_of!(ScoresList, limit) - 16usize];
	["Offset of field: ScoresList::scores"][::core::mem::offset_of!(ScoresList, scores) - 20usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct Board {
	pub boardID: *mut core::ffi::c_char,
	pub name: *mut core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of Board"][::core::mem::size_of::<Board>() - 8usize];
	["Alignment of Board"][::core::mem::align_of::<Board>() - 4usize];
	["Offset of field: Board::boardID"][::core::mem::offset_of!(Board, boardID) - 0usize];
	["Offset of field: Board::name"][::core::mem::offset_of!(Board, name) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct BoardsList {
	pub count: core::ffi::c_uint,
	pub lastUpdated: u32,
	pub boards: *mut Board,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of BoardsList"][::core::mem::size_of::<BoardsList>() - 12usize];
	["Alignment of BoardsList"][::core::mem::align_of::<BoardsList>() - 4usize];
	["Offset of field: BoardsList::count"][::core::mem::offset_of!(BoardsList, count) - 0usize];
	["Offset of field: BoardsList::lastUpdated"][::core::mem::offset_of!(BoardsList, lastUpdated) - 4usize];
	["Offset of field: BoardsList::boards"][::core::mem::offset_of!(BoardsList, boards) - 8usize];
};
pub type AddScoreCallback =
	::core::option::Option<unsafe extern "C" fn(score: *mut Score, errorMessage: *const core::ffi::c_char)>;
pub type PersonalBestCallback =
	::core::option::Option<unsafe extern "C" fn(score: *mut Score, errorMessage: *const core::ffi::c_char)>;
pub type BoardsListCallback =
	::core::option::Option<unsafe extern "C" fn(boards: *mut BoardsList, errorMessage: *const core::ffi::c_char)>;
pub type ScoresCallback =
	::core::option::Option<unsafe extern "C" fn(scores: *mut ScoresList, errorMessage: *const core::ffi::c_char)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateScoreboards {
	pub addScore: unsafe extern "C" fn(boardId: *const core::ffi::c_char,
	                                   value: u32,
	                                   callback: AddScoreCallback)
	                                   -> core::ffi::c_int,
	pub getPersonalBest:
		unsafe extern "C" fn(boardId: *const core::ffi::c_char, callback: PersonalBestCallback) -> core::ffi::c_int,
	pub freeScore: unsafe extern "C" fn(score: *mut Score),
	pub getScoreboards: unsafe extern "C" fn(callback: BoardsListCallback) -> core::ffi::c_int,
	pub freeBoardsList: unsafe extern "C" fn(boardsList: *mut BoardsList),
	pub getScores:
		unsafe extern "C" fn(boardId: *const core::ffi::c_char, callback: ScoresCallback) -> core::ffi::c_int,
	pub freeScoresList: unsafe extern "C" fn(scoresList: *mut ScoresList),
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateScoreboards"][::core::mem::size_of::<PlaydateScoreboards>() - 28usize];
	["Alignment of PlaydateScoreboards"][::core::mem::align_of::<PlaydateScoreboards>() - 4usize];
	["Offset of field: PlaydateScoreboards::addScore"]
		[::core::mem::offset_of!(PlaydateScoreboards, addScore) - 0usize];
	["Offset of field: PlaydateScoreboards::getPersonalBest"]
		[::core::mem::offset_of!(PlaydateScoreboards, getPersonalBest) - 4usize];
	["Offset of field: PlaydateScoreboards::freeScore"]
		[::core::mem::offset_of!(PlaydateScoreboards, freeScore) - 8usize];
	["Offset of field: PlaydateScoreboards::getScoreboards"]
		[::core::mem::offset_of!(PlaydateScoreboards, getScoreboards) - 12usize];
	["Offset of field: PlaydateScoreboards::freeBoardsList"]
		[::core::mem::offset_of!(PlaydateScoreboards, freeBoardsList) - 16usize];
	["Offset of field: PlaydateScoreboards::getScores"]
		[::core::mem::offset_of!(PlaydateScoreboards, getScores) - 20usize];
	["Offset of field: PlaydateScoreboards::freeScoresList"]
		[::core::mem::offset_of!(PlaydateScoreboards, freeScoresList) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct Playdate {
	pub system: &'static PlaydateSys,
	pub file: &'static PlaydateFile,
	pub graphics: &'static PlaydateGraphics,
	pub sprite: &'static PlaydateSprite,
	pub display: &'static PlaydateDisplay,
	pub sound: &'static PlaydateSound,
	pub lua: &'static PlaydateLua,
	pub json: &'static PlaydateJson,
	pub scoreboards: &'static PlaydateScoreboards,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of Playdate"][::core::mem::size_of::<Playdate>() - 36usize];
	["Alignment of Playdate"][::core::mem::align_of::<Playdate>() - 4usize];
	["Offset of field: Playdate::system"][::core::mem::offset_of!(Playdate, system) - 0usize];
	["Offset of field: Playdate::file"][::core::mem::offset_of!(Playdate, file) - 4usize];
	["Offset of field: Playdate::graphics"][::core::mem::offset_of!(Playdate, graphics) - 8usize];
	["Offset of field: Playdate::sprite"][::core::mem::offset_of!(Playdate, sprite) - 12usize];
	["Offset of field: Playdate::display"][::core::mem::offset_of!(Playdate, display) - 16usize];
	["Offset of field: Playdate::sound"][::core::mem::offset_of!(Playdate, sound) - 20usize];
	["Offset of field: Playdate::lua"][::core::mem::offset_of!(Playdate, lua) - 24usize];
	["Offset of field: Playdate::json"][::core::mem::offset_of!(Playdate, json) - 28usize];
	["Offset of field: Playdate::scoreboards"][::core::mem::offset_of!(Playdate, scoreboards) - 32usize];
};
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SystemEvent {
	Init = 0,
	InitLua = 1,
	Lock = 2,
	Unlock = 3,
	Pause = 4,
	Resume = 5,
	Terminate = 6,
	KeyPressed = 7,
	KeyReleased = 8,
	LowPower = 9,
}
