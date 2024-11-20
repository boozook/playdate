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
	pub fn get_bit(&self, index: usize) -> bool {
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = self.storage.as_ref()[byte_index];
		let bit_index = if cfg!(target_endian = "big") {
			7 - (index % 8)
		} else {
			index % 8
		};
		let mask = 1 << bit_index;
		byte & mask == mask
	}
	#[inline]
	pub fn set_bit(&mut self, index: usize, val: bool) {
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = &mut self.storage.as_mut()[byte_index];
		let bit_index = if cfg!(target_endian = "big") {
			7 - (index % 8)
		} else {
			index % 8
		};
		let mask = 1 << bit_index;
		if val {
			*byte |= mask;
		} else {
			*byte &= !mask;
		}
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
}
pub const LCD_COLUMNS: u32 = 400;
pub const LCD_ROWS: u32 = 240;
pub const LCD_ROWSIZE: u32 = 52;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const AUDIO_FRAMES_PER_CYCLE: u32 = 512;
pub const NOTE_C4: u32 = 60;
pub type va_list = *mut core::ffi::c_char;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct LCDRect {
	pub left: core::ffi::c_int,
	pub right: core::ffi::c_int,
	pub top: core::ffi::c_int,
	pub bottom: core::ffi::c_int,
}
#[test]
fn bindgen_test_layout_LCDRect() {
	const UNINIT: ::core::mem::MaybeUninit<LCDRect> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<LCDRect>(),
	           16usize,
	           concat!("Size of: ", stringify!(LCDRect))
	);
	assert_eq!(
	           ::core::mem::align_of::<LCDRect>(),
	           4usize,
	           concat!("Alignment of ", stringify!(LCDRect))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).left) as usize - ptr as usize },
	           0usize,
	           concat!("Offset of field: ", stringify!(LCDRect), "::", stringify!(left))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).right) as usize - ptr as usize },
	           4usize,
	           concat!("Offset of field: ", stringify!(LCDRect), "::", stringify!(right))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).top) as usize - ptr as usize },
	           8usize,
	           concat!("Offset of field: ", stringify!(LCDRect), "::", stringify!(top))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).bottom) as usize - ptr as usize },
	           12usize,
	           concat!("Offset of field: ", stringify!(LCDRect), "::", stringify!(bottom))
	);
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDBitmapDrawMode {
	kDrawModeCopy = 0,
	kDrawModeWhiteTransparent = 1,
	kDrawModeBlackTransparent = 2,
	kDrawModeFillWhite = 3,
	kDrawModeFillBlack = 4,
	kDrawModeXOR = 5,
	kDrawModeNXOR = 6,
	kDrawModeInverted = 7,
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDBitmapFlip {
	kBitmapUnflipped = 0,
	kBitmapFlippedX = 1,
	kBitmapFlippedY = 2,
	kBitmapFlippedXY = 3,
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDSolidColor {
	kColorBlack = 0,
	kColorWhite = 1,
	kColorClear = 2,
	kColorXOR = 3,
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDLineCapStyle {
	kLineCapStyleButt = 0,
	kLineCapStyleSquare = 1,
	kLineCapStyleRound = 2,
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDStringEncoding {
	kASCIIEncoding = 0,
	kUTF8Encoding = 1,
	k16BitLEEncoding = 2,
}
pub type LCDPattern = [u8; 16usize];
pub type LCDColor = usize;
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDPolygonFillRule {
	kPolygonFillNonZero = 0,
	kPolygonFillEvenOdd = 1,
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDTextWrappingMode {
	kWrapClip = 0,
	kWrapCharacter = 1,
	kWrapWord = 2,
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDTextAlignment {
	kAlignTextLeft = 0,
	kAlignTextCenter = 1,
	kAlignTextRight = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDBitmap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDBitmapTable {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDFont {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDFontData {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDFontPage {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDFontGlyph {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDVideoPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_video {
	#[doc = "`LCDVideoPlayer playdate->graphics->video->loadVideo(const char* path)`\n\nOpens the *pdv* file at *path* and returns a new video player object for rendering its frames."]
	pub loadVideo:
		::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut LCDVideoPlayer>,
	#[doc = "`void playdate->graphics->video->freePlayer(LCDVideoPlayer* p)`\n\nFrees the given video player."]
	pub freePlayer: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer)>,
	#[doc = "`int playdate->graphics->video->setContext(LCDVideoPlayer* p, LCDBitmap* context)`\n\nSets the rendering destination for the video player to the given bitmap. If the function fails, it returns 0 and sets an error message that can be read via [getError()](#f-graphics.video.getError)."]
	pub setContext: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer,
	                                                            context: *mut LCDBitmap)
	                                                            -> core::ffi::c_int>,
	#[doc = "`void playdate->graphics->video->useScreenContext(LCDVideoPlayer* p)`\n\nSets the rendering destination for the video player to the screen."]
	pub useScreenContext: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer)>,
	#[doc = "`void playdate->graphics->video->renderFrame(LCDVideoPlayer* p, int n)`\n\nRenders frame number *n* into the current context. In case of error, the function returns 0 and sets an error message that can be read via [getError()](#f-graphics.video.getError)."]
	pub renderFrame: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer,
	                                                             n: core::ffi::c_int)
	                                                             -> core::ffi::c_int>,
	#[doc = "`const char* playdate->graphics->video->getError(LCDVideoPlayer* p)`\n\nReturns text describing the most recent error."]
	pub getError: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *const core::ffi::c_char>,
	#[doc = "`void playdate->graphics->video->getInfo(LCDVideoPlayer* p, int* outWidth, int* outHeight, float* outFrameRate, int* outFrameCount, int* outCurrentFrame)`\n\nRetrieves information about the video, by passing in (possibly NULL) value pointers."]
	pub getInfo: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer,
	                                                         outWidth: *mut core::ffi::c_int,
	                                                         outHeight: *mut core::ffi::c_int,
	                                                         outFrameRate: *mut core::ffi::c_float,
	                                                         outFrameCount: *mut core::ffi::c_int,
	                                                         outCurrentFrame: *mut core::ffi::c_int)>,
	#[doc = "`LCBitmap* playdate->graphics->video->getContext(LCDVideoPlayer* p)`\n\nGets the rendering destination for the video player. If no rendering context has been setallocates a context bitmap with the same dimensions as the vieo will be allocated."]
	pub getContext: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *mut LCDBitmap>,
}
#[test]
fn bindgen_test_layout_playdate_video() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_video> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_video>(),
	           64usize,
	           concat!("Size of: ", stringify!(playdate_video))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_video>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_video))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadVideo) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(loadVideo)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freePlayer) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(freePlayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setContext) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(setContext)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).useScreenContext) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(useScreenContext)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).renderFrame) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(renderFrame)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getError) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(getError)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getInfo) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(getInfo)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getContext) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_video),
		"::",
		stringify!(getContext)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_graphics {
	pub video: *const playdate_video,
	#[doc = "`void playdate->graphics->clear(LCDColor color);`\n\nClears the entire display, filling it with *color*.\n\nEquivalent to [`playdate.graphics.clear()`](./Inside%20Playdate.html#f-graphics.clear) in the Lua API."]
	pub clear: ::core::option::Option<unsafe extern "C" fn(color: LCDColor)>,
	#[doc = "`void playdate->graphics->setBackgroundColor(LCDColor color);`\n\nSets the background color shown when the display is [offset](#f-display.setOffset) or for clearing dirty areas in the sprite system.\n\nEquivalent to [`playdate.graphics.setBackgroundColor()`](./Inside%20Playdate.html#f-graphics.setBackgroundColor) in the Lua API."]
	pub setBackgroundColor: ::core::option::Option<unsafe extern "C" fn(color: LCDSolidColor)>,
	#[doc = "`void playdate->graphics->setStencil(LCDBitmap* stencil);`\n\nSets the stencil used for drawing. For a tiled stencil, use *setStencilImage()* instead. To clear the stencil, set it to *NULL*."]
	pub setStencil: ::core::option::Option<unsafe extern "C" fn(stencil: *mut LCDBitmap)>,
	#[doc = "`LCDBitmapDrawMode playdate->graphics->setDrawMode(LCDBitmapDrawMode mode);`\n\nSets the mode used for drawing bitmaps. Note that text drawing uses bitmaps, so this affects how fonts are displayed as well. Returns the previous draw mode, in case you need to restore it after drawing.\n\nLCDBitmapDrawMode\n\n```cpp\ntypedef enum\n{\n\tkDrawModeCopy,\n\tkDrawModeWhiteTransparent,\n\tkDrawModeBlackTransparent,\n\tkDrawModeFillWhite,\n\tkDrawModeFillBlack,\n\tkDrawModeXOR,\n\tkDrawModeNXOR,\n\tkDrawModeInverted\n} LCDBitmapDrawMode;\n```\n\nEquivalent to [`playdate.graphics.setImageDrawMode()`](./Inside%20Playdate.html#f-graphics.setImageDrawMode) in the Lua API."]
	pub setDrawMode: ::core::option::Option<unsafe extern "C" fn(mode: LCDBitmapDrawMode) -> LCDBitmapDrawMode>,
	#[doc = "`void playdate->graphics->setDrawOffset(int dx, int dy);`\n\nOffsets the origin point for all drawing calls to *x*, *y* (can be negative).\n\nThis is useful, for example, for centering a \"camera\" on a sprite that is moving around a world larger than the screen.\n\nEquivalent to [`playdate.graphics.setDrawOffset()`](./Inside%20Playdate.html#f-graphics.setDrawOffset) in the Lua API."]
	pub setDrawOffset: ::core::option::Option<unsafe extern "C" fn(dx: core::ffi::c_int, dy: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->setClipRect(int x, int y, int width, int height);`\n\nSets the current clip rect, using world coordinates—\u{200b}that is, the given rectangle will be translated by the current drawing offset. The clip rect is cleared at the beginning of each update.\n\nEquivalent to [`playdate.graphics.setClipRect()`](./Inside%20Playdate.html#f-graphics.setClipRect) in the Lua API."]
	pub setClipRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                             y: core::ffi::c_int,
	                                                             width: core::ffi::c_int,
	                                                             height: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->clearClipRect(void);`\n\nClears the current clip rect.\n\nEquivalent to [`playdate.graphics.clearClipRect()`](./Inside%20Playdate.html#f-graphics.clearClipRect) in the Lua API."]
	pub clearClipRect: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->graphics->setLineCapStyle(LCDLineCapStyle endCapStyle);`\n\nSets the end cap style used in the line drawing functions.\n\nLCDLineCapStyle\n\n```cpp\ntypedef enum\n{\n\tkLineCapStyleButt,\n\tkLineCapStyleSquare,\n\tkLineCapStyleRound\n} LCDLineCapStyle;\n```\n\nEquivalent to [`playdate.graphics.setLineCapStyle()`](./Inside%20Playdate.html#f-graphics.setLineCapStyle) in the Lua API."]
	pub setLineCapStyle: ::core::option::Option<unsafe extern "C" fn(endCapStyle: LCDLineCapStyle)>,
	#[doc = "`void playdate->graphics->setFont(LCDFont* font);`\n\nSets the font to use in subsequent [drawText](#f-graphics.drawText) calls.\n\nEquivalent to [`playdate.graphics.setFont()`](./Inside%20Playdate.html#f-graphics.setFont) in the Lua API."]
	pub setFont: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont)>,
	#[doc = "`void playdate->graphics->setTextTracking(int tracking);`\n\nSets the tracking to use when drawing text.\n\nEquivalent to [`playdate.graphics.font:setTracking()`](./Inside%20Playdate.html#m-graphics.font.setTracking) in the Lua API."]
	pub setTextTracking: ::core::option::Option<unsafe extern "C" fn(tracking: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->pushContext(LCDBitmap* target);`\n\nPush a new drawing context for drawing into the given bitmap. If *target* is *NULL*, the drawing functions will use the display framebuffer.\n\nEquivalent to [`playdate.graphics.pushContext()`](./Inside%20Playdate.html#f-graphics.pushContext) in the Lua API."]
	pub pushContext: ::core::option::Option<unsafe extern "C" fn(target: *mut LCDBitmap)>,
	#[doc = "`void playdate->graphics->popContext(void);`\n\nPops a context off the stack (if any are left), restoring the drawing settings from before the context was pushed.\n\nEquivalent to [`playdate.graphics.popContext()`](./Inside%20Playdate.html#f-graphics.popContext) in the Lua API."]
	pub popContext: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->graphics->drawBitmap(LCDBitmap* bitmap, int x, int y, LCDBitmapFlip flip);`\n\nDraws the *bitmap* with its upper-left corner at location *x*, *y*, using the given flip orientation."]
	pub drawBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                            x: core::ffi::c_int,
	                                                            y: core::ffi::c_int,
	                                                            flip: LCDBitmapFlip)>,
	#[doc = "`void playdate->graphics->tileBitmap(LCDBitmap* bitmap, int x, int y, int width, int height, LCDBitmapFlip flip);`\n\nDraws the *bitmap* with its upper-left corner at location *x*, *y* tiled inside a *width* by *height* rectangle."]
	pub tileBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                            x: core::ffi::c_int,
	                                                            y: core::ffi::c_int,
	                                                            width: core::ffi::c_int,
	                                                            height: core::ffi::c_int,
	                                                            flip: LCDBitmapFlip)>,
	#[doc = "`void playdate->graphics->drawLine(int x1, int y1, int x2, int y2, int width, LCDColor color);`\n\nDraws a line from *x1*, *y1* to *x2*, *y2* with a stroke width of *width*.\n\nEquivalent to [`playdate.graphics.drawLine()`](./Inside%20Playdate.html#f-graphics.drawLine) in the Lua API."]
	pub drawLine: ::core::option::Option<unsafe extern "C" fn(x1: core::ffi::c_int,
	                                                          y1: core::ffi::c_int,
	                                                          x2: core::ffi::c_int,
	                                                          y2: core::ffi::c_int,
	                                                          width: core::ffi::c_int,
	                                                          color: LCDColor)>,
	#[doc = "`void playdate->graphics->fillTriangle(int x1, int y1, int x2, int y2, int x3, int y3, LCDColor color);`\n\nDraws a filled triangle with points at *x1*, *y1*, *x2*, *y2*, and *x3*, *y3*.\n\nLCDWindingRule\n\n```cpp\ntypedef enum\n{\n\tkPolygonFillNonZero,\n\tkPolygonFillEvenOdd\n} LCDPolygonFillRule;\n```\n\nEquivalent to [`playdate.graphics.fillTriangle()`](./Inside%20Playdate.html#f-graphics.fillTriangle) in the Lua API."]
	pub fillTriangle: ::core::option::Option<unsafe extern "C" fn(x1: core::ffi::c_int,
	                                                              y1: core::ffi::c_int,
	                                                              x2: core::ffi::c_int,
	                                                              y2: core::ffi::c_int,
	                                                              x3: core::ffi::c_int,
	                                                              y3: core::ffi::c_int,
	                                                              color: LCDColor)>,
	#[doc = "`void playdate->graphics->drawRect(int x, int y, int width, int height, LCDColor color);`\n\nDraws a *width* by *height* rect at *x*, *y*.\n\nEquivalent to [`playdate.graphics.drawRect()`](./Inside%20Playdate.html#f-graphics.drawRect) in the Lua API."]
	pub drawRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                          y: core::ffi::c_int,
	                                                          width: core::ffi::c_int,
	                                                          height: core::ffi::c_int,
	                                                          color: LCDColor)>,
	#[doc = "`void playdate->graphics->fillRect(int x, int y, int width, int height, LCDColor color);`\n\nDraws a filled *width* by *height* rect at *x*, *y*.\n\nEquivalent to [`playdate.graphics.fillRect()`](./Inside%20Playdate.html#f-graphics.fillRect) in the Lua API."]
	pub fillRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                          y: core::ffi::c_int,
	                                                          width: core::ffi::c_int,
	                                                          height: core::ffi::c_int,
	                                                          color: LCDColor)>,
	#[doc = "`void playdate->graphics->drawEllipse(int x, int y, int width, int height, int lineWidth, float startAngle, float endAngle, LCDColor color);`\n\nDraws an ellipse inside the rectangle {x, y, width, height} of width *lineWidth* (inset from the rectangle bounds). If *startAngle* != \\_endAngle, this draws an arc between the given angles. Angles are given in degrees, clockwise from due north."]
	pub drawEllipse: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                             y: core::ffi::c_int,
	                                                             width: core::ffi::c_int,
	                                                             height: core::ffi::c_int,
	                                                             lineWidth: core::ffi::c_int,
	                                                             startAngle: core::ffi::c_float,
	                                                             endAngle: core::ffi::c_float,
	                                                             color: LCDColor)>,
	#[doc = "`void playdate->graphics->fillEllipse(int x, int y, int width, int height, float startAngle, float endAngle, LCDColor color);`\n\nFills an ellipse inside the rectangle {x, y, width, height}. If *startAngle* != \\_endAngle, this draws a wedge/Pacman between the given angles. Angles are given in degrees, clockwise from due north."]
	pub fillEllipse: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                             y: core::ffi::c_int,
	                                                             width: core::ffi::c_int,
	                                                             height: core::ffi::c_int,
	                                                             startAngle: core::ffi::c_float,
	                                                             endAngle: core::ffi::c_float,
	                                                             color: LCDColor)>,
	#[doc = "`void playdate->graphics->drawScaledBitmap(LCDBitmap* bitmap, int x, int y, float xscale, float yscale);`\n\nDraws the *bitmap* scaled to *xscale* and *yscale* with its upper-left corner at location *x*, *y*. Note that *flip* is not available when drawing scaled bitmaps but negative scale values will achieve the same effect."]
	pub drawScaledBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                                  x: core::ffi::c_int,
	                                                                  y: core::ffi::c_int,
	                                                                  xscale: core::ffi::c_float,
	                                                                  yscale: core::ffi::c_float)>,
	#[doc = "`int playdate->graphics->drawText(const void* text, size_t len, PDStringEncoding encoding, int x, int y);`\n\nDraws the given text using the provided options. If no font has been set with [setFont](#f-graphics.setFont), the default system font Asheville Sans 14 Light is used. Note that `len` is the length of the **decoded** string—\u{200b}that is, the number of codepoints in the string, not the number of bytes; however, since the parser stops at the NUL terminator it’s safe to pass `strlen(text)` in here when you want to draw the entire string.\n\nEquivalent to [`playdate.graphics.drawText()`](./Inside%20Playdate.html#f-graphics.drawText) in the Lua API."]
	pub drawText: ::core::option::Option<unsafe extern "C" fn(text: *const core::ffi::c_void,
	                                                          len: usize,
	                                                          encoding: PDStringEncoding,
	                                                          x: core::ffi::c_int,
	                                                          y: core::ffi::c_int)
	                                                          -> core::ffi::c_int>,
	#[doc = "`LCDBitmap* playdate->graphics->newBitmap(int width, int height, LCDColor bgcolor);`\n\nAllocates and returns a new *width* by *height* LCDBitmap filled with *bgcolor*."]
	pub newBitmap: ::core::option::Option<unsafe extern "C" fn(width: core::ffi::c_int,
	                                                           height: core::ffi::c_int,
	                                                           bgcolor: LCDColor)
	                                                           -> *mut LCDBitmap>,
	#[doc = "`void playdate->graphics->freeBitmap(LCDBitmap*);`\n\nFrees the given *bitmap*."]
	pub freeBitmap: ::core::option::Option<unsafe extern "C" fn(arg1: *mut LCDBitmap)>,
	#[doc = "`LCDBitmap* playdate->graphics->loadBitmap(const char* path, const char** outerr);`\n\nAllocates and returns a new LCDBitmap from the file at *path*. If there is no file at *path*, the function returns null."]
	pub loadBitmap: ::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                                            outerr: *mut *const core::ffi::c_char)
	                                                            -> *mut LCDBitmap>,
	#[doc = "`LCDBitmap* playdate->graphics->copyBitmap(LCDBitmap* bitmap);`\n\nReturns a new LCDBitmap that is an exact copy of *bitmap*."]
	pub copyBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap>,
	#[doc = "`void playdate->graphics->loadIntoBitmap(const char* path, LCDBitmap* bitmap, const char** outerr);`\n\nLoads the image at *path* into the previously allocated *bitmap*."]
	pub loadIntoBitmap: ::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                                                bitmap: *mut LCDBitmap,
	                                                                outerr: *mut *const core::ffi::c_char)>,
	#[doc = "`void playdate->graphics->getBitmapData(LCDBitmap* bitmap, int* width, int* height, int* rowbytes, uint8_t** mask, uint8_t** data);`\n\nGets various info about *bitmap* including its *width* and *height* and raw pixel data. The data is 1 bit per pixel packed format, in MSB order; in other words, the high bit of the first byte in `data` is the top left pixel of the image. If the bitmap has a mask, a pointer to its data is returned in *mask*, else NULL is returned."]
	pub getBitmapData: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                               width: *mut core::ffi::c_int,
	                                                               height: *mut core::ffi::c_int,
	                                                               rowbytes: *mut core::ffi::c_int,
	                                                               mask: *mut *mut u8,
	                                                               data: *mut *mut u8)>,
	#[doc = "`void playdate->graphics->clearBitmap(LCDBitmap* bitmap, LCDColor bgcolor);`\n\nClears *bitmap*, filling with the given *bgcolor*."]
	pub clearBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap, bgcolor: LCDColor)>,
	#[doc = "`LCDBitmap* playdate->graphics->rotatedBitmap(LCDBitmap* bitmap, float rotation, float xscale, float yscale, int* allocedSize);`\n\nReturns a new, rotated and scaled LCDBitmap based on the given *bitmap*."]
	pub rotatedBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                               rotation: core::ffi::c_float,
	                                                               xscale: core::ffi::c_float,
	                                                               yscale: core::ffi::c_float,
	                                                               allocedSize: *mut core::ffi::c_int)
	                                                               -> *mut LCDBitmap>,
	#[doc = "`LCDBitmapTable* playdate->graphics->newBitmapTable(int count, int width, int height);`\n\nAllocates and returns a new LCDBitmapTable that can hold *count* *width* by *height* LCDBitmaps."]
	pub newBitmapTable: ::core::option::Option<unsafe extern "C" fn(count: core::ffi::c_int,
	                                                                width: core::ffi::c_int,
	                                                                height: core::ffi::c_int)
	                                                                -> *mut LCDBitmapTable>,
	#[doc = "`void playdate->graphics->freeBitmapTable(LCDBitmapTable* table);`\n\nFrees the given bitmap table. Note that this will invalidate any bitmaps returned by `getTableBitmap()`."]
	pub freeBitmapTable: ::core::option::Option<unsafe extern "C" fn(table: *mut LCDBitmapTable)>,
	#[doc = "`LCDBitmapTable* playdate->graphics->loadBitmapTable(const char* path, const char** outerr);`\n\nAllocates and returns a new LCDBitmap from the file at *path*. If there is no file at *path*, the function returns null."]
	pub loadBitmapTable: ::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                                                 outerr: *mut *const core::ffi::c_char)
	                                                                 -> *mut LCDBitmapTable>,
	#[doc = "`void playdate->graphics->loadIntoBitmapTable(const char* path, LCDBitmapTable* table, const char** outerr);`\n\nLoads the imagetable at *path* into the previously allocated *table*."]
	pub loadIntoBitmapTable: ::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                                                     table: *mut LCDBitmapTable,
	                                                                     outerr: *mut *const core::ffi::c_char)>,
	#[doc = "`LCDBitmap* playdate->graphics->getTableBitmap(LCDBitmapTable* table, int idx);`\n\nReturns the *idx* bitmap in *table*, If *idx* is out of bounds, the function returns NULL."]
	pub getTableBitmap: ::core::option::Option<unsafe extern "C" fn(table: *mut LCDBitmapTable,
	                                                                idx: core::ffi::c_int)
	                                                                -> *mut LCDBitmap>,
	#[doc = "`LCDFont* playdate->graphics->loadFont(const char* path, const char** outErr);`\n\nReturns the LCDFont object for the font file at *path*. In case of error, *outErr* points to a string describing the error. The returned font can be freed with [playdate→system→realloc(font, 0)](#f-system.realloc) when it is no longer in use."]
	pub loadFont: ::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                                          outErr: *mut *const core::ffi::c_char)
	                                                          -> *mut LCDFont>,
	#[doc = "`LCDFontPage* playdate->graphics->getFontPage(LCDFont* font, uint32_t c);`\n\nReturns an LCDFontPage object for the given character code. Each LCDFontPage contains information for 256 characters; specifically, if `(c1 & ~0xff) == (c2 & ~0xff)`, then *c1* and *c2* belong to the same page and the same LCDFontPage can be used to fetch the character data for both instead of searching for the page twice."]
	pub getFontPage: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont, c: u32) -> *mut LCDFontPage>,
	#[doc = "`LCDFontGlyph* playdate->graphics->getPageGlyph(LCDFontPage* page, uint32_t c, LCDBitmap** bitmap, int* advance);`\n\nReturns an LCDFontGlyph object for character *c* in LCDFontPage *page*, and optionally returns the glyph’s bitmap and advance value."]
	pub getPageGlyph: ::core::option::Option<unsafe extern "C" fn(page: *mut LCDFontPage,
	                                                              c: u32,
	                                                              bitmap: *mut *mut LCDBitmap,
	                                                              advance: *mut core::ffi::c_int)
	                                                              -> *mut LCDFontGlyph>,
	#[doc = "`int playdate->graphics->getGlyphKerning(LCDFontGlyph* glyph, uint32_t c1, uint32_t c2);`\n\nReturns the kerning adjustment between characters *c1* and *c2* as specified by the font."]
	pub getGlyphKerning: ::core::option::Option<unsafe extern "C" fn(glyph: *mut LCDFontGlyph,
	                                                                 glyphcode: u32,
	                                                                 nextcode: u32)
	                                                                 -> core::ffi::c_int>,
	#[doc = "`int playdate->graphics->getTextWidth(LCDFont* font, const void* text, size_t len, PDStringEncoding encoding, int tracking);`\n\nReturns the width of the given text in the given font. See the [note above](#f-graphics.drawText) about the `len` argument.\n\nPDStringEncoding\n\n```cpp\ntypedef enum\n{\n\tkASCIIEncoding,\n\tkUTF8Encoding,\n\tk16BitLEEncoding\n} PDStringEncoding;\n```"]
	pub getTextWidth: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont,
	                                                              text: *const core::ffi::c_void,
	                                                              len: usize,
	                                                              encoding: PDStringEncoding,
	                                                              tracking: core::ffi::c_int)
	                                                              -> core::ffi::c_int>,
	#[doc = "`uint8_t* playdate->graphics->getFrame(void);`\n\nReturns the current display frame buffer. Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored. Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row."]
	pub getFrame: ::core::option::Option<unsafe extern "C" fn() -> *mut u8>,
	#[doc = "`uint8_t* playdate->graphics->getDisplayFrame(void);`\n\nReturns the raw bits in the display buffer, the last completed frame."]
	pub getDisplayFrame: ::core::option::Option<unsafe extern "C" fn() -> *mut u8>,
	#[doc = "`LCDBitmap* playdate->graphics->getDebugBitmap(void);`\n\nOnly valid in the Simulator; function is NULL on device. Returns the debug framebuffer as a bitmap. White pixels drawn in the image are overlaid on the display in 50% transparent red."]
	pub getDebugBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDBitmap>,
	#[doc = "`LCDBitmap* playdate->graphics->copyFrameBufferBitmap(void);`\n\nReturns a copy the contents of the working frame buffer as a bitmap. The caller is responsible for freeing the returned bitmap with [playdate-\\>graphics-\\>freeBitmap()](#f-graphics.freeBitmap)."]
	pub copyFrameBufferBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDBitmap>,
	#[doc = "`void playdate->graphics->markUpdatedRows(int start, int end);`\n\nAfter updating pixels in the buffer returned by getFrame(), you must tell the graphics system which rows were updated. This function marks a contiguous range of rows as updated (e.g., markUpdatedRows(0,LCD\\_ROWS-1) tells the system to update the entire display). Both “start” and “end” are included in the range."]
	pub markUpdatedRows:
		::core::option::Option<unsafe extern "C" fn(start: core::ffi::c_int, end: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->display(void);`\n\nManually flushes the current frame buffer out to the display. This function is automatically called after each pass through the run loop, so there shouldn’t be any need to call it yourself."]
	pub display: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->graphics->setColorToPattern(LCDColor* color, LCDBitmap* bitmap, int x, int y);`\n\nSets *color* to an 8 x 8 pattern using the given *bitmap*. *x*, *y* indicates the top left corner of the 8 x 8 pattern."]
	pub setColorToPattern: ::core::option::Option<unsafe extern "C" fn(color: *mut LCDColor,
	                                                                   bitmap: *mut LCDBitmap,
	                                                                   x: core::ffi::c_int,
	                                                                   y: core::ffi::c_int)>,
	#[doc = "`int playdate->graphics->checkMaskCollision(LCDBitmap* bitmap1, int x1, int y1, LCDBitmapFlip flip1, LCDBitmap* bitmap2, int x2, int y2, LCDBitmapFlip flip2, LCDRect rect);`\n\nReturns 1 if any of the opaque pixels in *bitmap1* when positioned at *x1*, *y1* with *flip1* overlap any of the opaque pixels in *bitmap2* at *x2*, *y2* with *flip2* within the non-empty *rect*, or 0 if no pixels overlap or if one or both fall completely outside of *rect*."]
	pub checkMaskCollision: ::core::option::Option<unsafe extern "C" fn(bitmap1: *mut LCDBitmap,
	                                                                    x1: core::ffi::c_int,
	                                                                    y1: core::ffi::c_int,
	                                                                    flip1: LCDBitmapFlip,
	                                                                    bitmap2: *mut LCDBitmap,
	                                                                    x2: core::ffi::c_int,
	                                                                    y2: core::ffi::c_int,
	                                                                    flip2: LCDBitmapFlip,
	                                                                    rect: LCDRect)
	                                                                    -> core::ffi::c_int>,
	#[doc = "`void playdate->graphics->setScreenClipRect(int x, int y, int width, int height);`\n\nSets the current clip rect in screen coordinates.\n\nEquivalent to [`playdate.graphics.setScreenClipRect()`](./Inside%20Playdate.html#f-graphics.setScreenClipRect) in the Lua API."]
	pub setScreenClipRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                                   y: core::ffi::c_int,
	                                                                   width: core::ffi::c_int,
	                                                                   height: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->fillPolygon(int nPoints, int* points, LCDColor color, LCDPolygonFillRule fillrule);`\n\nFills the polygon with vertices at the given coordinates (an array of 2\\*`nPoints` ints containing alternating x and y values) using the given color and fill, or winding, rule. See [https://en.wikipedia.org/wiki/Nonzero-rule](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule. An edge between the last vertex and the first is assumed.\n\nEquivalent to [`playdate.graphics.fillPolygon()`](./Inside%20Playdate.html#f-graphics.fillPolygon) in the Lua API."]
	pub fillPolygon: ::core::option::Option<unsafe extern "C" fn(nPoints: core::ffi::c_int,
	                                                             coords: *mut core::ffi::c_int,
	                                                             color: LCDColor,
	                                                             fillrule: LCDPolygonFillRule)>,
	#[doc = "`uint8_t playdate->graphics->getFontHeight(LCDFont* font);`\n\nReturns the height of the given font."]
	pub getFontHeight: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont) -> u8>,
	#[doc = "`LCDBitmap* playdate->graphics->getDisplayBufferBitmap(void);`\n\nReturns a bitmap containing the contents of the display buffer. The system owns this bitmap—\u{200b}do not free it!"]
	pub getDisplayBufferBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDBitmap>,
	#[doc = "`LCDBitmap* playdate->graphics->drawRotatedBitmap(LCDBitmap* bitmap, int x, int y, float degrees, float centerx, float centery, float xscale, float yscale);`\n\nDraws the *bitmap* scaled to *xscale* and *yscale* then rotated by *degrees* with its center as given by proportions *centerx* and *centery* at *x*, *y*; that is: if *centerx* and *centery* are both 0.5 the center of the image is at (*x*,*y*), if *centerx* and *centery* are both 0 the top left corner of the image (before rotation) is at (*x*,*y*), etc."]
	pub drawRotatedBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                                   x: core::ffi::c_int,
	                                                                   y: core::ffi::c_int,
	                                                                   rotation: core::ffi::c_float,
	                                                                   centerx: core::ffi::c_float,
	                                                                   centery: core::ffi::c_float,
	                                                                   xscale: core::ffi::c_float,
	                                                                   yscale: core::ffi::c_float)>,
	#[doc = "`void playdate->graphics->setTextLeading(int leading);`\n\nSets the leading adjustment (added to the leading specified in the font) to use when drawing text.\n\nEquivalent to [`playdate.graphics.font:setLeading()`](./Inside%20Playdate.html#m-graphics.font.setLeading) in the Lua API."]
	pub setTextLeading: ::core::option::Option<unsafe extern "C" fn(lineHeightAdustment: core::ffi::c_int)>,
	#[doc = "`int playdate->graphics->setBitmapMask(LCDBitmap* bitmap, LCDBitmap* mask);`\n\nSets a mask image for the given *bitmap*. The set mask must be the same size as the target bitmap."]
	pub setBitmapMask: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                               mask: *mut LCDBitmap)
	                                                               -> core::ffi::c_int>,
	#[doc = "`LCDBitmap* playdate->graphics->getBitmapMask(LCDBitmap* bitmap);`\n\nGets a mask image for the given *bitmap*, or returns NULL if the *bitmap* doesn’t have a mask layer. The returned image points to *bitmap*'s data, so drawing into the mask image affects the source bitmap directly. The caller takes ownership of the returned LCDBitmap and is responsible for freeing it when it’s no longer in use."]
	pub getBitmapMask: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap>,
	#[doc = "`void playdate->graphics->setStencilImage(LCDBitmap* stencil, int tile);`\n\nSets the stencil used for drawing. If the *tile* flag is set the stencil image will be tiled. Tiled stencils must have width equal to a multiple of 32 pixels. To clear the stencil, call `playdate→graphics→setStencil(NULL);`.\n\nEquivalent to [`playdate.graphics.setStencilImage()`](./Inside%20Playdate.html#f-graphics.setStencilImage) in the Lua API."]
	pub setStencilImage:
		::core::option::Option<unsafe extern "C" fn(stencil: *mut LCDBitmap, tile: core::ffi::c_int)>,
	#[doc = "`LCDFont* playdate->graphics->makeFontFromData(LCDFontData* data, int wide);`\n\nReturns an LCDFont object wrapping the LCDFontData *data* comprising the contents (minus 16-byte header) of an uncompressed pft file. *wide* corresponds to the flag in the header indicating whether the font contains glyphs at codepoints above U+1FFFF."]
	pub makeFontFromData: ::core::option::Option<unsafe extern "C" fn(data: *mut LCDFontData,
	                                                                  wide: core::ffi::c_int)
	                                                                  -> *mut LCDFont>,
	#[doc = "`int playdate->graphics->getTextTracking(void);`\n\nGets the tracking used when drawing text.\n\nEquivalent to [`playdate.graphics.font:getTracking()`](./Inside%20Playdate.html#m-graphics.font.getTracking) in the Lua API."]
	pub getTextTracking: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,
	#[doc = "`void playdate->graphics->setPixel(int x, int y, LCDColor color);`\n\nSets the pixel at *(x,y)* in the current drawing context (by default the screen) to the given *color*. Be aware that setting a pixel at a time is not very efficient: In our testing, more than around 20,000 calls in a tight loop will drop the frame rate below 30 fps."]
	pub setPixel:
		::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int, c: LCDColor)>,
	#[doc = "`LCDSolidColor playdate->graphics->getBitmapPixel(LCDBitmap* bitmap, int x, int y);`\n\nGets the color of the pixel at *(x,y)* in the given *bitmap*. If the coordinate is outside the bounds of the bitmap, or if the bitmap has a mask and the pixel is marked transparent, the function returns `kColorClear`; otherwise the return value is `kColorWhite` or `kColorBlack`."]
	pub getBitmapPixel: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap,
	                                                                x: core::ffi::c_int,
	                                                                y: core::ffi::c_int)
	                                                                -> LCDSolidColor>,
	#[doc = "`void playdate->graphics->getBitmapTableInfo(LCDBitmapTable* table, int* count, int* cellswide);`\n\nReturns the bitmap table’s image count in the *count* pointer (if not NULL) and number of cells across in the *cellswide* pointer (ditto)."]
	pub getBitmapTableInfo: ::core::option::Option<unsafe extern "C" fn(table: *mut LCDBitmapTable,
	                                                                    count: *mut core::ffi::c_int,
	                                                                    width: *mut core::ffi::c_int)>,
	#[doc = "`int playdate->graphics->drawTextInRect(const void* text, size_t len, PDStringEncoding encoding, int x, int y, int width, int height, PDTextWrappingMode wrap, PDTextAlignment align);`\n\nDraws the text in the given rectangle using the provided options. If no font has been set with [setFont](#f-graphics.setFont), the default system font Asheville Sans 14 Light is used. See the [above note](#f-graphics.drawText) about the `len` argument.\n\nThe *wrap* argument is one of\n\nPDTextWrappingMode\n\n```cpp\ntypedef enum\n{\n\tkWrapClip,\n\tkWrapCharacter,\n\tkWrapWord,\n} PDTextWrappingMode;\n```\n\nand *align* is one of\n\nPDTextAlignment\n\n```cpp\ntypedef enum\n{\n\tkAlignTextLeft,\n\tkAlignTextCenter,\n\tkAlignTextRight\n} PDTextAlignment;\n```"]
	pub drawTextInRect: ::core::option::Option<unsafe extern "C" fn(text: *const core::ffi::c_void,
	                                                                len: usize,
	                                                                encoding: PDStringEncoding,
	                                                                x: core::ffi::c_int,
	                                                                y: core::ffi::c_int,
	                                                                width: core::ffi::c_int,
	                                                                height: core::ffi::c_int,
	                                                                wrap: PDTextWrappingMode,
	                                                                align: PDTextAlignment)>,
}
#[test]
fn bindgen_test_layout_playdate_graphics() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_graphics> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_graphics>(),
	           512usize,
	           concat!("Size of: ", stringify!(playdate_graphics))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_graphics>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_graphics))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).video) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(video)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clear) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(clear)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setBackgroundColor) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setBackgroundColor)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStencil) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setStencil)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDrawMode) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setDrawMode)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDrawOffset) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setDrawOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setClipRect) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setClipRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearClipRect) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(clearClipRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLineCapStyle) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setLineCapStyle)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFont) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setFont)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTextTracking) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setTextTracking)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushContext) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(pushContext)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).popContext) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(popContext)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawBitmap) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).tileBitmap) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(tileBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawLine) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawLine)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).fillTriangle) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(fillTriangle)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawRect) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).fillRect) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(fillRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawEllipse) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawEllipse)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).fillEllipse) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(fillEllipse)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawScaledBitmap) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawScaledBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawText) as usize - ptr as usize },
	           176usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawText)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newBitmap) as usize - ptr as usize },
	           184usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(newBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeBitmap) as usize - ptr as usize },
	           192usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(freeBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadBitmap) as usize - ptr as usize },
	           200usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(loadBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).copyBitmap) as usize - ptr as usize },
	           208usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(copyBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadIntoBitmap) as usize - ptr as usize },
	           216usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(loadIntoBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBitmapData) as usize - ptr as usize },
	           224usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getBitmapData)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearBitmap) as usize - ptr as usize },
	           232usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(clearBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).rotatedBitmap) as usize - ptr as usize },
	           240usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(rotatedBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newBitmapTable) as usize - ptr as usize },
	           248usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(newBitmapTable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeBitmapTable) as usize - ptr as usize },
	           256usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(freeBitmapTable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadBitmapTable) as usize - ptr as usize },
	           264usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(loadBitmapTable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadIntoBitmapTable) as usize - ptr as usize },
	           272usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(loadIntoBitmapTable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTableBitmap) as usize - ptr as usize },
	           280usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getTableBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadFont) as usize - ptr as usize },
	           288usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(loadFont)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFontPage) as usize - ptr as usize },
	           296usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getFontPage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getPageGlyph) as usize - ptr as usize },
	           304usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getPageGlyph)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getGlyphKerning) as usize - ptr as usize },
	           312usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getGlyphKerning)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTextWidth) as usize - ptr as usize },
	           320usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getTextWidth)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFrame) as usize - ptr as usize },
	           328usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getFrame)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getDisplayFrame) as usize - ptr as usize },
	           336usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getDisplayFrame)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getDebugBitmap) as usize - ptr as usize },
	           344usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getDebugBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).copyFrameBufferBitmap) as usize - ptr as usize },
	           352usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(copyFrameBufferBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).markUpdatedRows) as usize - ptr as usize },
	           360usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(markUpdatedRows)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).display) as usize - ptr as usize },
	           368usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(display)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setColorToPattern) as usize - ptr as usize },
	           376usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setColorToPattern)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).checkMaskCollision) as usize - ptr as usize },
	           384usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(checkMaskCollision)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setScreenClipRect) as usize - ptr as usize },
	           392usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setScreenClipRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).fillPolygon) as usize - ptr as usize },
	           400usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(fillPolygon)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFontHeight) as usize - ptr as usize },
	           408usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getFontHeight)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getDisplayBufferBitmap) as usize - ptr as usize },
	           416usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getDisplayBufferBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawRotatedBitmap) as usize - ptr as usize },
	           424usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawRotatedBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTextLeading) as usize - ptr as usize },
	           432usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setTextLeading)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setBitmapMask) as usize - ptr as usize },
	           440usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setBitmapMask)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBitmapMask) as usize - ptr as usize },
	           448usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getBitmapMask)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStencilImage) as usize - ptr as usize },
	           456usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setStencilImage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).makeFontFromData) as usize - ptr as usize },
	           464usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(makeFontFromData)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTextTracking) as usize - ptr as usize },
	           472usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getTextTracking)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPixel) as usize - ptr as usize },
	           480usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(setPixel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBitmapPixel) as usize - ptr as usize },
	           488usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getBitmapPixel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBitmapTableInfo) as usize - ptr as usize },
	           496usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(getBitmapTableInfo)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawTextInRect) as usize - ptr as usize },
	           504usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_graphics),
		"::",
		stringify!(drawTextInRect)
	)
	);
}
impl Default for playdate_graphics {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
impl PDButtons {
	pub const kButtonLeft: PDButtons = PDButtons(1);
}
impl PDButtons {
	pub const kButtonRight: PDButtons = PDButtons(2);
}
impl PDButtons {
	pub const kButtonUp: PDButtons = PDButtons(4);
}
impl PDButtons {
	pub const kButtonDown: PDButtons = PDButtons(8);
}
impl PDButtons {
	pub const kButtonB: PDButtons = PDButtons(16);
}
impl PDButtons {
	pub const kButtonA: PDButtons = PDButtons(32);
}
impl ::core::ops::BitOr<PDButtons> for PDButtons {
	type Output = Self;
	#[inline]
	fn bitor(self, other: Self) -> Self { PDButtons(self.0 | other.0) }
}
impl ::core::ops::BitOrAssign for PDButtons {
	#[inline]
	fn bitor_assign(&mut self, rhs: PDButtons) { self.0 |= rhs.0; }
}
impl ::core::ops::BitAnd<PDButtons> for PDButtons {
	type Output = Self;
	#[inline]
	fn bitand(self, other: Self) -> Self { PDButtons(self.0 & other.0) }
}
impl ::core::ops::BitAndAssign for PDButtons {
	#[inline]
	fn bitand_assign(&mut self, rhs: PDButtons) { self.0 &= rhs.0; }
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PDButtons(pub i32);
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDLanguage {
	kPDLanguageEnglish = 0,
	kPDLanguageJapanese = 1,
	kPDLanguageUnknown = 2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PDDateTime {
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub weekday: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
}
#[test]
fn bindgen_test_layout_PDDateTime() {
	const UNINIT: ::core::mem::MaybeUninit<PDDateTime> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PDDateTime>(),
	           8usize,
	           concat!("Size of: ", stringify!(PDDateTime))
	);
	assert_eq!(
	           ::core::mem::align_of::<PDDateTime>(),
	           2usize,
	           concat!("Alignment of ", stringify!(PDDateTime))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).year) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDDateTime),
		"::",
		stringify!(year)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).month) as usize - ptr as usize },
	           2usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDDateTime),
		"::",
		stringify!(month)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).day) as usize - ptr as usize },
	           3usize,
	           concat!("Offset of field: ", stringify!(PDDateTime), "::", stringify!(day))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).weekday) as usize - ptr as usize },
	           4usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDDateTime),
		"::",
		stringify!(weekday)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).hour) as usize - ptr as usize },
	           5usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDDateTime),
		"::",
		stringify!(hour)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).minute) as usize - ptr as usize },
	           6usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDDateTime),
		"::",
		stringify!(minute)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).second) as usize - ptr as usize },
	           7usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDDateTime),
		"::",
		stringify!(second)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDMenuItem {
	_unused: [u8; 0],
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDPeripherals {
	kNone = 0,
	kAccelerometer = 1,
	kAllPeripherals = 65535,
}
pub type PDCallbackFunction =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void) -> core::ffi::c_int>;
pub type PDMenuItemCallbackFunction =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
pub type PDButtonCallbackFunction =
	::core::option::Option<unsafe extern "C" fn(button: PDButtons,
	                                            down: core::ffi::c_int,
	                                            when: u32,
	                                            userdata: *mut core::ffi::c_void)
	                                            -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sys { # [doc = "`void* playdate->system->realloc(void* ptr, size_t size)`\n\nAllocates heap space if *ptr* is NULL, else reallocates the given pointer. If *size* is zero, frees the given pointer."] pub realloc : :: core :: option :: Option < unsafe extern "C" fn (ptr : * mut core :: ffi :: c_void , size : usize) -> * mut core :: ffi :: c_void > , # [doc = "`int playdate->system->formatString(char **outstring, const char *format, ...)`\n\nCreates a formatted string and returns it via the *outstring* argument. The arguments and return value match libc’s `asprintf()`: the format string is standard `printf()` style, the string returned in *outstring* should be freed by the caller when it’s no longer in use, and the return value is the length of the formatted string."] pub formatString : :: core :: option :: Option < unsafe extern "C" fn (ret : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int > , # [doc = "`void playdate->system->logToConsole(const char* format, ...)`\n\nCalls the log function.\n\nEquivalent to [`print()`](./Inside%20Playdate.html#f-print) in the Lua API."] pub logToConsole : :: core :: option :: Option < unsafe extern "C" fn (fmt : * const core :: ffi :: c_char , ...) > , # [doc = "`void playdate->system->error(const char* format, ...)`\n\nCalls the log function, outputting an error in red to the console, then pauses execution."] pub error : :: core :: option :: Option < unsafe extern "C" fn (fmt : * const core :: ffi :: c_char , ...) > , # [doc = "`PDLanguage playdate->system->getLanguage(void);`\n\nReturns the current language of the system."] pub getLanguage : :: core :: option :: Option < unsafe extern "C" fn () -> PDLanguage > , # [doc = "`unsigned int playdate->system->getCurrentTimeMilliseconds(void)`\n\nReturns the number of milliseconds since…\u{200b}some arbitrary point in time. This should present a consistent timebase while a game is running, but the counter will be disabled when the device is sleeping."] pub getCurrentTimeMilliseconds : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_uint > , # [doc = "`unsigned int playdate->system->getSecondsSinceEpoch(unsigned int *milliseconds)`\n\nReturns the number of seconds (and sets *milliseconds* if not NULL) elapsed since midnight (hour 0), January 1, 2000."] pub getSecondsSinceEpoch : :: core :: option :: Option < unsafe extern "C" fn (milliseconds : * mut core :: ffi :: c_uint) -> core :: ffi :: c_uint > , # [doc = "`void playdate->system->drawFPS(int x, int y)`\n\nCalculates the current frames per second and draws that value at *x, y*."] pub drawFPS : :: core :: option :: Option < unsafe extern "C" fn (x : core :: ffi :: c_int , y : core :: ffi :: c_int) > , # [doc = "`void playdate->system->setUpdateCallback(PDCallbackFunction* update, void* userdata)`\n\nPDCallbackFunction\n\n```cpp\nint PDCallbackFunction(void* userdata);\n```\n\nReplaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed."] pub setUpdateCallback : :: core :: option :: Option < unsafe extern "C" fn (update : PDCallbackFunction , userdata : * mut core :: ffi :: c_void) > , # [doc = "`void playdate->system->getButtonState(PDButtons* current, PDButtons* pushed, PDButtons* released)`\n\nSets the value pointed to by *current* to a bitmask indicating which buttons are currently down. *pushed* and *released* reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.\n\nPDButton\n\n```cpp\nkButtonLeft\nkButtonRight\nkButtonUp\nkButtonDown\nkButtonB\nkButtonA\n```"] pub getButtonState : :: core :: option :: Option < unsafe extern "C" fn (current : * mut PDButtons , pushed : * mut PDButtons , released : * mut PDButtons) > , # [doc = "`void playdate->system->setPeripheralsEnabled(PDPeripherals mask)`\n\nBy default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.\n\nPDPeripherals\n\n```cpp\nkNone\nkAccelerometer\n```"] pub setPeripheralsEnabled : :: core :: option :: Option < unsafe extern "C" fn (mask : PDPeripherals) > , # [doc = "`void playdate->system->getAccelerometer(float* outx, float* outy, float* outz)`\n\nReturns the last-read accelerometer data."] pub getAccelerometer : :: core :: option :: Option < unsafe extern "C" fn (outx : * mut core :: ffi :: c_float , outy : * mut core :: ffi :: c_float , outz : * mut core :: ffi :: c_float) > , # [doc = "`float playdate->system->getCrankChange(void)`\n\nReturns the angle change of the crank since the last time this function was called. Negative values are anti-clockwise."] pub getCrankChange : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`float playdate->system->getCrankAngle(void)`\n\nReturns the current position of the crank, in the range 0-360. Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device."] pub getCrankAngle : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`int playdate->system->isCrankDocked(void)`\n\nReturns 1 or 0 indicating whether or not the crank is folded into the unit."] pub isCrankDocked : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`int playdate->system->setCrankSoundsDisabled(int disable)`\n\nThe function returns the previous value for this setting."] pub setCrankSoundsDisabled : :: core :: option :: Option < unsafe extern "C" fn (flag : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->system->getFlipped()`\n\nReturns 1 if the global \"flipped\" system setting is set, otherwise 0."] pub getFlipped : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`void playdate->system->setAutoLockDisabled(int disable)`\n\nDisables or enables the 3 minute auto lock feature. When called, the timer is reset to 3 minutes."] pub setAutoLockDisabled : :: core :: option :: Option < unsafe extern "C" fn (disable : core :: ffi :: c_int) > , # [doc = "`void playdate->system->setMenuImage(LCDBitmap* bitmap, int xOffset);`\n\nA game can optionally provide an image to be displayed alongside the system menu. *bitmap* must be a 400x240 LCDBitmap. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.\n\nOptionally, a non-zero *xoffset*, can be provided. This must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by xoffset pixels as the menu is animated in.\n\nThis function could be called in response to the kEventPause *event* in your implementation of [eventHandler()](#_eventHandler)."] pub setMenuImage : :: core :: option :: Option < unsafe extern "C" fn (bitmap : * mut LCDBitmap , xOffset : core :: ffi :: c_int) > , # [doc = "`PDMenuItem* playdate->system->addMenuItem(const char* title, PDMenuItemCallbackFunction* callback, void* userdata)`\n\n*title* will be the title displayed by the menu item.\n\nAdds a new menu item to the System Menu. When invoked by the user, this menu item will:\n\n1. Invoke your *callback* function.\n\n2. Hide the System Menu.\n\n3. Unpause your game and call [eventHandler()](#_eventHandler) with the kEventResume *event*.\n\nYour game can then present an options interface to the player, or take other action, in whatever manner you choose.\n\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually."] pub addMenuItem : :: core :: option :: Option < unsafe extern "C" fn (title : * const core :: ffi :: c_char , callback : PDMenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut PDMenuItem > , # [doc = "`PDMenuItem* playdate->system->addCheckmarkMenuItem(const char* title, int value, PDMenuItemCallbackFunction* callback, void* userdata)`\n\nAdds a new menu item that can be checked or unchecked by the player.\n\n*title* will be the title displayed by the menu item.\n\n*value* should be 0 for unchecked, 1 for checked.\n\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\n\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually."] pub addCheckmarkMenuItem : :: core :: option :: Option < unsafe extern "C" fn (title : * const core :: ffi :: c_char , value : core :: ffi :: c_int , callback : PDMenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut PDMenuItem > , # [doc = "`PDMenuItem* playdate->system->addOptionsMenuItem(const char* title, const char** options, int optionsCount, PDMenuItemCallbackFunction* callback, void* userdata)`\n\nAdds a new menu item that allows the player to cycle through a set of options.\n\n*title* will be the title displayed by the menu item.\n\n*options* should be an array of strings representing the states this menu item can cycle through. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.\n\n*optionsCount* should be the number of items contained in *options*.\n\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\n\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually."] pub addOptionsMenuItem : :: core :: option :: Option < unsafe extern "C" fn (title : * const core :: ffi :: c_char , optionTitles : * mut * const core :: ffi :: c_char , optionsCount : core :: ffi :: c_int , f : PDMenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut PDMenuItem > , # [doc = "`void playdate->system->removeAllMenuItems()`\n\nRemoves all custom menu items from the system menu."] pub removeAllMenuItems : :: core :: option :: Option < unsafe extern "C" fn () > , # [doc = "`void playdate->system->removeMenuItem(PDMenuItem *menuItem)`\n\nRemoves the menu item from the system menu."] pub removeMenuItem : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) > , # [doc = "`int playdate->system->getMenuItemValue(PDMenuItem *menuItem)`"] pub getMenuItemValue : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) -> core :: ffi :: c_int > , # [doc = "`void playdate->system->setMenuItemValue(PDMenuItem *menuItem, int value)`\n\nGets or sets the integer value of the menu item.\n\nFor checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option."] pub setMenuItemValue : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem , value : core :: ffi :: c_int) > , # [doc = "`const char* playdate->system->getMenuItemTitle(PDMenuItem *menuItem)`"] pub getMenuItemTitle : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) -> * const core :: ffi :: c_char > , # [doc = "`void playdate->system->setMenuItemTitle(PDMenuItem *menuItem, const char* title)`\n\nGets or sets the display title of the menu item."] pub setMenuItemTitle : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem , title : * const core :: ffi :: c_char) > , # [doc = "`void* playdate->system->getMenuItemUserdata(PDMenuItem *menuItem)`"] pub getMenuItemUserdata : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) -> * mut core :: ffi :: c_void > , # [doc = "`void playdate->system->setMenuItemUserdata(PDMenuItem *menuItem, void* userdata)`\n\nGets or sets the userdata value associated with this menu item."] pub setMenuItemUserdata : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem , ud : * mut core :: ffi :: c_void) > , # [doc = "`int playdate->system->getReduceFlashing()`\n\nReturns 1 if the global \"reduce flashing\" system setting is set, otherwise 0."] pub getReduceFlashing : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`float playdate->system->getElapsedTime()`\n\nReturns the number of seconds since `playdate.resetElapsedTime()` was called. The value is a floating-point number with microsecond accuracy."] pub getElapsedTime : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`void playdate->system->resetElapsedTime(void)`\n\nResets the high-resolution timer."] pub resetElapsedTime : :: core :: option :: Option < unsafe extern "C" fn () > , # [doc = "`float playdate->system->getBatteryPercentage()`\n\nReturns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full."] pub getBatteryPercentage : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`float playdate->system->getBatteryVoltage()`\n\nReturns the battery’s current voltage level."] pub getBatteryVoltage : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`int32_t playdate->system->getTimezoneOffset()`\n\nReturns the system timezone offset from GMT, in seconds."] pub getTimezoneOffset : :: core :: option :: Option < unsafe extern "C" fn () -> i32 > , # [doc = "`int playdate->system->shouldDisplay24HourTime()`\n\nReturns 1 if the user has set the 24-Hour Time preference in the Settings program."] pub shouldDisplay24HourTime : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`void playdate->system->convertEpochToDateTime(uint32_t epoch, struct PDDateTime* datetime)`\n\nConverts the given epoch time to a PDDateTime."] pub convertEpochToDateTime : :: core :: option :: Option < unsafe extern "C" fn (epoch : u32 , datetime : * mut PDDateTime) > , # [doc = "`uint32_t playdate->system->convertDateTimeToEpoch(struct PDDateTime* datetime)`\n\nConverts the given PDDateTime to an epoch time."] pub convertDateTimeToEpoch : :: core :: option :: Option < unsafe extern "C" fn (datetime : * mut PDDateTime) -> u32 > , # [doc = "`float playdate->system->clearICache()`\n\nFlush the CPU instruction cache, on the very unlikely chance you’re modifying instruction code on the fly. (If you don’t know what I’m talking about, you don’t need this. :smile:)"] pub clearICache : :: core :: option :: Option < unsafe extern "C" fn () > , # [doc = "`void playdate->system->setButtonCallback(PDButtonCallbackFunction* cb, void* userdata, int queuesize)`\n\nAs an alternative to polling for button presses using `getButtonState()`, this function allows a callback function to be set. The function is called for each button up/down event (possibly multiple events on the same button) that occurred during the previous update cycle. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. The function should return 0 on success or a non-zero value to signal an error.\n\nPDButtonCallbackFunction\n\n```cpp\ntypedef int PDButtonCallbackFunction(PDButtons button, int down, uint32_t when, void* userdata);\n```"] pub setButtonCallback : :: core :: option :: Option < unsafe extern "C" fn (cb : PDButtonCallbackFunction , buttonud : * mut core :: ffi :: c_void , queuesize : core :: ffi :: c_int) > , # [doc = "`void playdate->system->setSerialMessageCallback(void (*callback)(const char* data));`\n\nProvides a callback to receive messages sent to the device over the serial port using the `msg` command. If no device is connected, you can send these messages to a game in the simulator by entering `!msg <message>` in the Lua console."] pub setSerialMessageCallback : :: core :: option :: Option < unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (data : * const core :: ffi :: c_char) >) > , # [doc = "`int playdate->system->vaFormatString(char **ret, const char *format, va_list args)`\n\nAllocates and formats a string using a variadic `va_list` argument, in the style of `vasprintf()`. The string returned via *ret* should be freed by the caller when it is no longer in use. The return value from the function is the length of the formatted string."] pub vaFormatString : :: core :: option :: Option < unsafe extern "C" fn (outstr : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , args : va_list) -> core :: ffi :: c_int > , # [doc = "`int playdate->system->parseString(const char *str, const char *format, ...)`\n\nLike libc `sscanf()`, parses a string according to a format string and places the values into pointers passed in after the format. The return value is the number of items matched."] pub parseString : :: core :: option :: Option < unsafe extern "C" fn (str_ : * const core :: ffi :: c_char , format : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int > , }
#[test]
fn bindgen_test_layout_playdate_sys() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sys> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sys>(),
	           352usize,
	           concat!("Size of: ", stringify!(playdate_sys))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sys>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sys))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).realloc) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(realloc)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).formatString) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(formatString)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).logToConsole) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(logToConsole)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(error)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLanguage) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getLanguage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCurrentTimeMilliseconds) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getCurrentTimeMilliseconds)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getSecondsSinceEpoch) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getSecondsSinceEpoch)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawFPS) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(drawFPS)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUpdateCallback) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setUpdateCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getButtonState) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getButtonState)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPeripheralsEnabled) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setPeripheralsEnabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getAccelerometer) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getAccelerometer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCrankChange) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getCrankChange)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCrankAngle) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getCrankAngle)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isCrankDocked) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(isCrankDocked)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCrankSoundsDisabled) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setCrankSoundsDisabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFlipped) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getFlipped)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAutoLockDisabled) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setAutoLockDisabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMenuImage) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setMenuImage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addMenuItem) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(addMenuItem)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addCheckmarkMenuItem) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(addCheckmarkMenuItem)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addOptionsMenuItem) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(addOptionsMenuItem)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeAllMenuItems) as usize - ptr as usize },
	           176usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(removeAllMenuItems)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeMenuItem) as usize - ptr as usize },
	           184usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(removeMenuItem)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getMenuItemValue) as usize - ptr as usize },
	           192usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getMenuItemValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMenuItemValue) as usize - ptr as usize },
	           200usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setMenuItemValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getMenuItemTitle) as usize - ptr as usize },
	           208usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getMenuItemTitle)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMenuItemTitle) as usize - ptr as usize },
	           216usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setMenuItemTitle)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getMenuItemUserdata) as usize - ptr as usize },
	           224usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getMenuItemUserdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMenuItemUserdata) as usize - ptr as usize },
	           232usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setMenuItemUserdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getReduceFlashing) as usize - ptr as usize },
	           240usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getReduceFlashing)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getElapsedTime) as usize - ptr as usize },
	           248usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getElapsedTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).resetElapsedTime) as usize - ptr as usize },
	           256usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(resetElapsedTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBatteryPercentage) as usize - ptr as usize },
	           264usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getBatteryPercentage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBatteryVoltage) as usize - ptr as usize },
	           272usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getBatteryVoltage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTimezoneOffset) as usize - ptr as usize },
	           280usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(getTimezoneOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).shouldDisplay24HourTime) as usize - ptr as usize },
	           288usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(shouldDisplay24HourTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).convertEpochToDateTime) as usize - ptr as usize },
	           296usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(convertEpochToDateTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).convertDateTimeToEpoch) as usize - ptr as usize },
	           304usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(convertDateTimeToEpoch)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearICache) as usize - ptr as usize },
	           312usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(clearICache)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setButtonCallback) as usize - ptr as usize },
	           320usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setButtonCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setSerialMessageCallback) as usize - ptr as usize },
	           328usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(setSerialMessageCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).vaFormatString) as usize - ptr as usize },
	           336usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(vaFormatString)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).parseString) as usize - ptr as usize },
	           344usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sys),
		"::",
		stringify!(parseString)
	)
	);
}
pub type lua_State = *mut core::ffi::c_void;
pub type lua_CFunction = ::core::option::Option<unsafe extern "C" fn(L: *mut lua_State) -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LuaUDObject {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDSprite {
	_unused: [u8; 0],
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum l_valtype {
	kInt = 0,
	kFloat = 1,
	kStr = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct lua_reg {
	pub name: *const core::ffi::c_char,
	pub func: lua_CFunction,
}
#[test]
fn bindgen_test_layout_lua_reg() {
	const UNINIT: ::core::mem::MaybeUninit<lua_reg> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<lua_reg>(),
	           16usize,
	           concat!("Size of: ", stringify!(lua_reg))
	);
	assert_eq!(
	           ::core::mem::align_of::<lua_reg>(),
	           8usize,
	           concat!("Alignment of ", stringify!(lua_reg))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
	           0usize,
	           concat!("Offset of field: ", stringify!(lua_reg), "::", stringify!(name))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
	           8usize,
	           concat!("Offset of field: ", stringify!(lua_reg), "::", stringify!(func))
	);
}
impl Default for lua_reg {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LuaType {
	kTypeNil = 0,
	kTypeBool = 1,
	kTypeInt = 2,
	kTypeFloat = 3,
	kTypeString = 4,
	kTypeTable = 5,
	kTypeFunction = 6,
	kTypeThread = 7,
	kTypeObject = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub struct lua_val {
	pub name: *const core::ffi::c_char,
	pub type_: l_valtype,
	pub v: lua_val__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub union lua_val__bindgen_ty_1 {
	pub intval: core::ffi::c_uint,
	pub floatval: core::ffi::c_float,
	pub strval: *const core::ffi::c_char,
}
#[test]
fn bindgen_test_layout_lua_val__bindgen_ty_1() {
	const UNINIT: ::core::mem::MaybeUninit<lua_val__bindgen_ty_1> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<lua_val__bindgen_ty_1>(),
	           8usize,
	           concat!("Size of: ", stringify!(lua_val__bindgen_ty_1))
	);
	assert_eq!(
	           ::core::mem::align_of::<lua_val__bindgen_ty_1>(),
	           8usize,
	           concat!("Alignment of ", stringify!(lua_val__bindgen_ty_1))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).intval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(lua_val__bindgen_ty_1),
		"::",
		stringify!(intval)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).floatval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(lua_val__bindgen_ty_1),
		"::",
		stringify!(floatval)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).strval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(lua_val__bindgen_ty_1),
		"::",
		stringify!(strval)
	)
	);
}
impl Default for lua_val__bindgen_ty_1 {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[test]
fn bindgen_test_layout_lua_val() {
	const UNINIT: ::core::mem::MaybeUninit<lua_val> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<lua_val>(),
	           24usize,
	           concat!("Size of: ", stringify!(lua_val))
	);
	assert_eq!(
	           ::core::mem::align_of::<lua_val>(),
	           8usize,
	           concat!("Alignment of ", stringify!(lua_val))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
	           0usize,
	           concat!("Offset of field: ", stringify!(lua_val), "::", stringify!(name))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
	           8usize,
	           concat!("Offset of field: ", stringify!(lua_val), "::", stringify!(type_))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
	           16usize,
	           concat!("Offset of field: ", stringify!(lua_val), "::", stringify!(v))
	);
}
impl Default for lua_val {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_lua {
	#[doc = "`int playdate->lua->addFunction(lua_CFunction f, const char* name, const char** outErr);`\n\nAdds the Lua function *f* to the Lua runtime, with name *name*. (*name* can be a table path using dots, e.g. if name = “mycode.myDrawingFunction” adds the function “myDrawingFunction” to the global table “myCode”.) Returns 1 on success or 0 with an error message in *outErr*."]
	pub addFunction: ::core::option::Option<unsafe extern "C" fn(f: lua_CFunction,
	                                                             name: *const core::ffi::c_char,
	                                                             outErr: *mut *const core::ffi::c_char)
	                                                             -> core::ffi::c_int>,
	#[doc = "`int playdate->lua->registerClass(const char* name, const lua_reg* reg, const lua_val* vals, int isstatic, const char** outErr);`\n\nCreates a new \"class\" (i.e., a Lua metatable containing functions) with the given name and adds the given functions and constants to it. If the table is simply a list of functions that won’t be used as a metatable, *isstatic* should be set to 1 to create a plain table instead of a metatable. Please see `C_API/Examples/Array` for an example of how to use `registerClass` to create a Lua table-like object from C."]
	pub registerClass: ::core::option::Option<unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                                               reg: *const lua_reg,
	                                                               vals: *const lua_val,
	                                                               isstatic: core::ffi::c_int,
	                                                               outErr: *mut *const core::ffi::c_char)
	                                                               -> core::ffi::c_int>,
	#[doc = "`void playdate->lua->pushFunction(lua_CFunction f);`\n\nPushes a [lua\\_CFunction](#f-lua.cFunction) onto the stack."]
	pub pushFunction: ::core::option::Option<unsafe extern "C" fn(f: lua_CFunction)>,
	#[doc = "`int playdate->lua->indexMetatable(void);`\n\nIf a class includes an `__index` function, it should call this first to check if the indexed variable exists in the metatable. If the indexMetatable() call returns 1, it has located the variable and put it on the stack, and the `__index` function should return 1 to indicate a value was found. If indexMetatable() doesn’t find a value, the `__index` function can then do its custom getter magic."]
	pub indexMetatable: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,
	#[doc = "`void playdate->lua->stop(void);`\n\nStops the run loop."]
	pub stop: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->lua->start(void);`\n\nStarts the run loop back up."]
	pub start: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`int playdate->lua->getArgCount(void);`\n\nReturns the number of arguments passed to the function."]
	pub getArgCount: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,
	#[doc = "`enum LuaType playdate->lua->getArgType(int pos, const char** outClass);`\n\nReturns the type of the variable at stack position *pos*. If the type is *kTypeObject* and *outClass* is non-NULL, it returns the name of the object’s metatable."]
	pub getArgType: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int,
	                                                            outClass: *mut *const core::ffi::c_char)
	                                                            -> LuaType>,
	#[doc = "`int playdate->lua->argIsNil(int pos);`\n\nReturns 1 if the argument at the given position *pos* is nil."]
	pub argIsNil: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int>,
	#[doc = "`int playdate->lua->getArgBool(int pos);`\n\nReturns one if the argument at position *pos* is true, zero if not."]
	pub getArgBool: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int>,
	#[doc = "`int playdate->lua->getArgInt(int pos);`\n\nReturns the argument at position *pos* as an int."]
	pub getArgInt: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int>,
	#[doc = "`float playdate->lua->getArgFloat(int pos);`\n\nReturns the argument at position *pos* as a float."]
	pub getArgFloat: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_float>,
	#[doc = "`const char* playdate->lua->getArgString(int pos);`\n\nReturns the argument at position *pos* as a string."]
	pub getArgString:
		::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> *const core::ffi::c_char>,
	#[doc = "`const char* playdate->lua->getArgBytes(int pos, size_t* outlen);`\n\nReturns the argument at position *pos* as a string and sets *outlen* to its length."]
	pub getArgBytes: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int,
	                                                             outlen: *mut usize)
	                                                             -> *const core::ffi::c_char>,
	#[doc = "`void* playdate->lua->getArgObject(int pos, char* type, LuaUDObject** outud);`\n\nChecks the object type of the argument at position *pos* and returns a pointer to it if it’s the correct type. Optionally sets *outud* to a pointer to the opaque LuaUDObject for the given stack."]
	pub getArgObject: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int,
	                                                              type_: *mut core::ffi::c_char,
	                                                              outud: *mut *mut LuaUDObject)
	                                                              -> *mut core::ffi::c_void>,
	#[doc = "`LCDBitmap* playdate->lua->getBitmap(int pos);`\n\nReturns the argument at position *pos* as an LCDBitmap."]
	pub getBitmap: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut LCDBitmap>,
	#[doc = "`LCDSprite* playdate->lua->getSprite(int pos);`\n\nReturns the argument at position *pos* as an LCDSprite."]
	pub getSprite: ::core::option::Option<unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut LCDSprite>,
	#[doc = "`void playdate->lua->pushNil(void);`\n\nPushes nil onto the stack."]
	pub pushNil: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->lua->pushBool(int val);`\n\nPushes the int *val* onto the stack."]
	pub pushBool: ::core::option::Option<unsafe extern "C" fn(val: core::ffi::c_int)>,
	#[doc = "`void playdate->lua->pushInt(int val);`\n\nPushes the int *val* onto the stack."]
	pub pushInt: ::core::option::Option<unsafe extern "C" fn(val: core::ffi::c_int)>,
	#[doc = "`void playdate->lua->pushFloat(float val);`\n\nPushes the float *val* onto the stack."]
	pub pushFloat: ::core::option::Option<unsafe extern "C" fn(val: core::ffi::c_float)>,
	#[doc = "`void playdate->lua->pushString(char* str);`\n\nPushes the string *str* onto the stack."]
	pub pushString: ::core::option::Option<unsafe extern "C" fn(str_: *const core::ffi::c_char)>,
	#[doc = "`void playdate->lua->pushBytes(char* str, size_t len);`\n\nLike *pushString()*, but pushes an arbitrary byte array to the stack, ignoring \\\\0 characters."]
	pub pushBytes: ::core::option::Option<unsafe extern "C" fn(str_: *const core::ffi::c_char, len: usize)>,
	#[doc = "`void playdate->lua->pushBitmap(LCDBitmap* bitmap);`\n\nPushes the LCDBitmap *bitmap* onto the stack."]
	pub pushBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap)>,
	#[doc = "`void playdate->lua->pushSprite(LCDSprite* sprite);`\n\nPushes the LCDSprite *sprite* onto the stack."]
	pub pushSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`LuaUDObject* playdate->lua->pushObject(void* obj, char* type, int nValues);`\n\nPushes the given custom object *obj* onto the stack and returns a pointer to the opaque LuaUDObject. *type* must match the class name used in [playdate-\\>lua-\\>registerClass()](#f-lua.registerClass). *nValues* is the number of slots to allocate for Lua values (see [set/getObjectValue()](#f-lua.setObjectValue))."]
	pub pushObject: ::core::option::Option<unsafe extern "C" fn(obj: *mut core::ffi::c_void,
	                                                            type_: *mut core::ffi::c_char,
	                                                            nValues: core::ffi::c_int)
	                                                            -> *mut LuaUDObject>,
	#[doc = "`LuaUDObject* playdate->lua->retainObject(LuaUDObject* obj);`\n\nRetains the opaque LuaUDObject *obj* and returns same."]
	pub retainObject: ::core::option::Option<unsafe extern "C" fn(obj: *mut LuaUDObject) -> *mut LuaUDObject>,
	#[doc = "`void playdate->lua->releaseObject(LuaUDObject* obj);`\n\nReleases the opaque LuaUDObject *obj*."]
	pub releaseObject: ::core::option::Option<unsafe extern "C" fn(obj: *mut LuaUDObject)>,
	#[doc = "`void playdate->lua->setUserValue(LuaUDObject* obj, int slot);`\n\nSets the value of object *obj*'s uservalue slot number *slot* (starting at 1, not zero) to the value at the top of the stack."]
	pub setUserValue: ::core::option::Option<unsafe extern "C" fn(obj: *mut LuaUDObject, slot: core::ffi::c_uint)>,
	#[doc = "`int playdate->lua->getUserValue(LuaUDObject* obj, int slot);`\n\nCopies the value at *obj*'s given uservalue *slot* to the top of the stack and returns its stack position."]
	pub getUserValue: ::core::option::Option<unsafe extern "C" fn(obj: *mut LuaUDObject,
	                                                              slot: core::ffi::c_uint)
	                                                              -> core::ffi::c_int>,
	pub callFunction_deprecated:
		::core::option::Option<unsafe extern "C" fn(name: *const core::ffi::c_char, nargs: core::ffi::c_int)>,
	#[doc = "`int playdate->lua->callFunction(const char* name, int nargs, const char** outerr);`\n\nCalls the Lua function *name* and and indicates that *nargs* number of arguments have already been pushed to the stack for the function to use. *name* can be a table path using dots, e.g. “playdate.apiVersion”. Returns 1 on success; on failure, returns 0 and puts an error message into the `outerr` pointer, if it’s set. Calling Lua from C is slow, so use sparingly."]
	pub callFunction: ::core::option::Option<unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                                              nargs: core::ffi::c_int,
	                                                              outerr: *mut *const core::ffi::c_char)
	                                                              -> core::ffi::c_int>,
}
#[test]
fn bindgen_test_layout_playdate_lua() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_lua> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_lua>(),
	           256usize,
	           concat!("Size of: ", stringify!(playdate_lua))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_lua>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_lua))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addFunction) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(addFunction)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).registerClass) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(registerClass)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushFunction) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushFunction)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).indexMetatable) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(indexMetatable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(stop)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).start) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(start)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgCount) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgCount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgType) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgType)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).argIsNil) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(argIsNil)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgBool) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgBool)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgInt) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgInt)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgFloat) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgFloat)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgString) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgString)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgBytes) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgBytes)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getArgObject) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getArgObject)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBitmap) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getSprite) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getSprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushNil) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushNil)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushBool) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushBool)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushInt) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushInt)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushFloat) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushFloat)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushString) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushString)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushBytes) as usize - ptr as usize },
	           176usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushBytes)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushBitmap) as usize - ptr as usize },
	           184usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushBitmap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushSprite) as usize - ptr as usize },
	           192usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushSprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pushObject) as usize - ptr as usize },
	           200usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(pushObject)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).retainObject) as usize - ptr as usize },
	           208usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(retainObject)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).releaseObject) as usize - ptr as usize },
	           216usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(releaseObject)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUserValue) as usize - ptr as usize },
	           224usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(setUserValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getUserValue) as usize - ptr as usize },
	           232usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(getUserValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).callFunction_deprecated) as usize - ptr as usize },
	           240usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(callFunction_deprecated)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).callFunction) as usize - ptr as usize },
	           248usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_lua),
		"::",
		stringify!(callFunction)
	)
	);
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum json_value_type {
	kJSONNull = 0,
	kJSONTrue = 1,
	kJSONFalse = 2,
	kJSONInteger = 3,
	kJSONFloat = 4,
	kJSONString = 5,
	kJSONArray = 6,
	kJSONTable = 7,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub struct json_value {
	pub type_: core::ffi::c_char,
	pub data: json_value__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[must_use]
pub union json_value__bindgen_ty_1 {
	pub intval: core::ffi::c_int,
	pub floatval: core::ffi::c_float,
	pub stringval: *mut core::ffi::c_char,
	pub arrayval: *mut core::ffi::c_void,
	pub tableval: *mut core::ffi::c_void,
}
#[test]
fn bindgen_test_layout_json_value__bindgen_ty_1() {
	const UNINIT: ::core::mem::MaybeUninit<json_value__bindgen_ty_1> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<json_value__bindgen_ty_1>(),
	           8usize,
	           concat!("Size of: ", stringify!(json_value__bindgen_ty_1))
	);
	assert_eq!(
	           ::core::mem::align_of::<json_value__bindgen_ty_1>(),
	           8usize,
	           concat!("Alignment of ", stringify!(json_value__bindgen_ty_1))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).intval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value__bindgen_ty_1),
		"::",
		stringify!(intval)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).floatval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value__bindgen_ty_1),
		"::",
		stringify!(floatval)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stringval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value__bindgen_ty_1),
		"::",
		stringify!(stringval)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).arrayval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value__bindgen_ty_1),
		"::",
		stringify!(arrayval)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).tableval) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value__bindgen_ty_1),
		"::",
		stringify!(tableval)
	)
	);
}
impl Default for json_value__bindgen_ty_1 {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[test]
fn bindgen_test_layout_json_value() {
	const UNINIT: ::core::mem::MaybeUninit<json_value> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<json_value>(),
	           16usize,
	           concat!("Size of: ", stringify!(json_value))
	);
	assert_eq!(
	           ::core::mem::align_of::<json_value>(),
	           8usize,
	           concat!("Alignment of ", stringify!(json_value))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value),
		"::",
		stringify!(type_)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_value),
		"::",
		stringify!(data)
	)
	);
}
impl Default for json_value {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct json_decoder {
	pub decodeError: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                             error: *const core::ffi::c_char,
	                                                             linenum: core::ffi::c_int)>,
	pub willDecodeSublist: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                                   name: *const core::ffi::c_char,
	                                                                   type_: json_value_type)>,
	pub shouldDecodeTableValueForKey: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                                              key: *const core::ffi::c_char)
	                                                                              -> core::ffi::c_int>,
	pub didDecodeTableValue: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                                     key: *const core::ffi::c_char,
	                                                                     value: json_value)>,
	pub shouldDecodeArrayValueAtIndex: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                                               pos: core::ffi::c_int)
	                                                                               -> core::ffi::c_int>,
	pub didDecodeArrayValue: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                                     pos: core::ffi::c_int,
	                                                                     value: json_value)>,
	pub didDecodeSublist: ::core::option::Option<unsafe extern "C" fn(decoder: *mut json_decoder,
	                                                                  name: *const core::ffi::c_char,
	                                                                  type_: json_value_type)
	                                                                  -> *mut core::ffi::c_void>,
	pub userdata: *mut core::ffi::c_void,
	pub returnString: core::ffi::c_int,
	pub path: *const core::ffi::c_char,
}
#[test]
fn bindgen_test_layout_json_decoder() {
	const UNINIT: ::core::mem::MaybeUninit<json_decoder> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<json_decoder>(),
	           80usize,
	           concat!("Size of: ", stringify!(json_decoder))
	);
	assert_eq!(
	           ::core::mem::align_of::<json_decoder>(),
	           8usize,
	           concat!("Alignment of ", stringify!(json_decoder))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).decodeError) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(decodeError)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).willDecodeSublist) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(willDecodeSublist)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).shouldDecodeTableValueForKey) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(shouldDecodeTableValueForKey)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).didDecodeTableValue) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(didDecodeTableValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).shouldDecodeArrayValueAtIndex) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(shouldDecodeArrayValueAtIndex)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).didDecodeArrayValue) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(didDecodeArrayValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).didDecodeSublist) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(didDecodeSublist)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).userdata) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(userdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).returnString) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(returnString)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).path) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_decoder),
		"::",
		stringify!(path)
	)
	);
}
impl Default for json_decoder {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
pub type json_readFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                     buf: *mut u8,
                                                                     bufsize: core::ffi::c_int)
                                                                     -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct json_reader {
	pub read: json_readFunc,
	pub userdata: *mut core::ffi::c_void,
}
#[test]
fn bindgen_test_layout_json_reader() {
	const UNINIT: ::core::mem::MaybeUninit<json_reader> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<json_reader>(),
	           16usize,
	           concat!("Size of: ", stringify!(json_reader))
	);
	assert_eq!(
	           ::core::mem::align_of::<json_reader>(),
	           8usize,
	           concat!("Alignment of ", stringify!(json_reader))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).read) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_reader),
		"::",
		stringify!(read)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).userdata) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_reader),
		"::",
		stringify!(userdata)
	)
	);
}
impl Default for json_reader {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
pub type json_writeFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                      str_: *const core::ffi::c_char,
                                                                      len: core::ffi::c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct json_encoder {
	pub writeStringFunc: json_writeFunc,
	pub userdata: *mut core::ffi::c_void,
	pub _bitfield_align_1: [u32; 0],
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
	pub startArray: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub addArrayMember: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub endArray: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub startTable: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub addTableMember: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder,
	                                                                name: *const core::ffi::c_char,
	                                                                len: core::ffi::c_int)>,
	pub endTable: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub writeNull: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub writeFalse: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub writeTrue: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder)>,
	pub writeInt: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder, num: core::ffi::c_int)>,
	pub writeDouble:
		::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder, num: core::ffi::c_double)>,
	pub writeString: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder,
	                                                             str_: *const core::ffi::c_char,
	                                                             len: core::ffi::c_int)>,
}
#[test]
fn bindgen_test_layout_json_encoder() {
	const UNINIT: ::core::mem::MaybeUninit<json_encoder> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<json_encoder>(),
	           120usize,
	           concat!("Size of: ", stringify!(json_encoder))
	);
	assert_eq!(
	           ::core::mem::align_of::<json_encoder>(),
	           8usize,
	           concat!("Alignment of ", stringify!(json_encoder))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeStringFunc) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeStringFunc)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).userdata) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(userdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).startArray) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(startArray)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addArrayMember) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(addArrayMember)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).endArray) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(endArray)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).startTable) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(startTable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addTableMember) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(addTableMember)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).endTable) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(endTable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeNull) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeNull)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeFalse) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeFalse)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeTrue) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeTrue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeInt) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeInt)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeDouble) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeDouble)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).writeString) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(json_encoder),
		"::",
		stringify!(writeString)
	)
	);
}
impl Default for json_encoder {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
impl json_encoder {
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_json {
	#[doc = "`void playdate->json->initEncoder(json_encoder* encoder, writeFunc* write, void* userdata, int pretty);`\n\nPopulates the given json\\_encoder *encoder* with the functions necessary to encode arbitrary data into a JSON string. *userdata* is passed as the first argument of the given writeFunc *write*. When *pretty* is 1 the string is written with human-readable formatting."]
	pub initEncoder: ::core::option::Option<unsafe extern "C" fn(encoder: *mut json_encoder,
	                                                             write: json_writeFunc,
	                                                             userdata: *mut core::ffi::c_void,
	                                                             pretty: core::ffi::c_int)>,
	#[doc = "`int playdate->json->decode(struct json_decoder* decoder, json_reader reader, json_value* outval);`\n\nEquivalent to [`playdate.json.decode()`](./Inside%20Playdate.html#f-json.decode) in the Lua API."]
	pub decode: ::core::option::Option<unsafe extern "C" fn(functions: *mut json_decoder,
	                                                        reader: json_reader,
	                                                        outval: *mut json_value)
	                                                        -> core::ffi::c_int>,
	#[doc = "`int playdate->json->decodeString(struct json_decoder* decoder, const char* jsonString, json_value* outval);`\n\nDecodes a JSON file or string with the given *decoder*. An instance of json\\_decoder must implement *decodeError*. The remaining functions are optional although you’ll probably want to implement at least *didDecodeTableValue* and *didDecodeArrayValue*. The *outval* pointer, if set, contains the value retured from the top-level *didDecodeSublist* callback."]
	pub decodeString: ::core::option::Option<unsafe extern "C" fn(functions: *mut json_decoder,
	                                                              jsonString: *const core::ffi::c_char,
	                                                              outval: *mut json_value)
	                                                              -> core::ffi::c_int>,
}
#[test]
fn bindgen_test_layout_playdate_json() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_json> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_json>(),
	           24usize,
	           concat!("Size of: ", stringify!(playdate_json))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_json>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_json))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).initEncoder) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_json),
		"::",
		stringify!(initEncoder)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).decode) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_json),
		"::",
		stringify!(decode)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).decodeString) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_json),
		"::",
		stringify!(decodeString)
	)
	);
}
pub type SDFile = core::ffi::c_void;
impl FileOptions {
	pub const kFileRead: FileOptions = FileOptions(1);
}
impl FileOptions {
	pub const kFileReadData: FileOptions = FileOptions(2);
}
impl FileOptions {
	pub const kFileWrite: FileOptions = FileOptions(4);
}
impl FileOptions {
	pub const kFileAppend: FileOptions = FileOptions(8);
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct FileOptions(pub i32);
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
#[test]
fn bindgen_test_layout_FileStat() {
	const UNINIT: ::core::mem::MaybeUninit<FileStat> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<FileStat>(),
	           32usize,
	           concat!("Size of: ", stringify!(FileStat))
	);
	assert_eq!(
	           ::core::mem::align_of::<FileStat>(),
	           4usize,
	           concat!("Alignment of ", stringify!(FileStat))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isdir) as usize - ptr as usize },
	           0usize,
	           concat!("Offset of field: ", stringify!(FileStat), "::", stringify!(isdir))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
	           4usize,
	           concat!("Offset of field: ", stringify!(FileStat), "::", stringify!(size))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).m_year) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(FileStat),
		"::",
		stringify!(m_year)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).m_month) as usize - ptr as usize },
	           12usize,
	           concat!(
		"Offset of field: ",
		stringify!(FileStat),
		"::",
		stringify!(m_month)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).m_day) as usize - ptr as usize },
	           16usize,
	           concat!("Offset of field: ", stringify!(FileStat), "::", stringify!(m_day))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).m_hour) as usize - ptr as usize },
	           20usize,
	           concat!(
		"Offset of field: ",
		stringify!(FileStat),
		"::",
		stringify!(m_hour)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).m_minute) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(FileStat),
		"::",
		stringify!(m_minute)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).m_second) as usize - ptr as usize },
	           28usize,
	           concat!(
		"Offset of field: ",
		stringify!(FileStat),
		"::",
		stringify!(m_second)
	)
	);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_file { # [doc = "`const char* playdate->file->geterr(void);`\n\nReturns human-readable text describing the most recent error (usually indicated by a -1 return from a filesystem function)."] pub geterr : :: core :: option :: Option < unsafe extern "C" fn () -> * const core :: ffi :: c_char > , # [doc = "`int playdate->file->listfiles(const char* path, void (*callback)(const char* filename, void* userdata), void* userdata, int showhidden);`\n\nCalls the given callback function for every file at *path*. Subfolders are indicated by a trailing slash '/' in *filename*. *listfiles()* does not recurse into subfolders. If *showhidden* is set, files beginning with a period will be included; otherwise, they are skipped. Returns 0 on success, -1 if no folder exists at *path* or it can’t be opened.\n\nEquivalent to [`playdate.file.listFiles()`](./Inside%20Playdate.html#f-file.listFiles) in the Lua API."] pub listfiles : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , callback : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , userdata : * mut core :: ffi :: c_void) > , userdata : * mut core :: ffi :: c_void , showhidden : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->stat(const char* path, FileStat* stat);`\n\nPopulates the FileStat *stat* with information about the file at *path*. Returns 0 on success, or -1 in case of error.\n\nFileStat\n\n```cpp\ntypedef struct\n{\n\tint isdir;\n\tunsigned int size;\n\tint m_year;\n\tint m_month;\n\tint m_day;\n\tint m_hour;\n\tint m_minute;\n\tint m_second;\n} FileStat;\n```"] pub stat : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , stat : * mut FileStat) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->mkdir(const char* path);`\n\nCreates the given *path* in the Data/\\<gameid\\> folder. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\n\nEquivalent to [`playdate.file.mkdir()`](./Inside%20Playdate.html#f-file.mkdir) in the Lua API."] pub mkdir : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->unlink(const char* path, int recursive);`\n\nDeletes the file at *path*. Returns 0 on success, or -1 in case of error. If recursive is 1 and the target path is a folder, this deletes everything inside the folder (including folders, folders inside those, and so on) as well as the folder itself."] pub unlink : :: core :: option :: Option < unsafe extern "C" fn (name : * const core :: ffi :: c_char , recursive : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->rename(const char* from, const char* to);`\n\nRenames the file at *from* to *to*. It will overwrite the file at *to* without confirmation. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\n\nEquivalent to [`playdate.file.rename()`](./Inside%20Playdate.html#f-file.rename) in the Lua API."] pub rename : :: core :: option :: Option < unsafe extern "C" fn (from : * const core :: ffi :: c_char , to : * const core :: ffi :: c_char) -> core :: ffi :: c_int > , # [doc = "`SDFile* playdate->file->open(const char* path, FileOptions mode);`\n\nOpens a handle for the file at *path*. The *kFileRead* mode opens a file in the game pdx, while *kFileReadData* searches the game’s data folder; to search the data folder first then fall back on the game pdx, use the bitwise combination *kFileRead|kFileReadData*.*kFileWrite* and *kFileAppend* always write to the data folder. The function returns NULL if a file at *path* cannot be opened, and [playdate-\\>file-\\>geterr()](#f-file.geterr) will describe the error. The filesystem has a limit of 64 simultaneous open files. The returned file handle should be [closed](#f-file.close), not freed, when it is no longer in use.\n\nFileOptions\n\n```cpp\ntypedef enum\n{\n\tkFileRead,\n\tkFileReadData,\n\tkFileWrite,\n\tkFileAppend\n} FileOptions;\n```\n\nEquivalent to [`playdate.file.open()`](./Inside%20Playdate.html#f-file.open) in the Lua API."] pub open : :: core :: option :: Option < unsafe extern "C" fn (name : * const core :: ffi :: c_char , mode : FileOptions) -> * mut SDFile > , # [doc = "`int playdate->file->close(SDFile* file);`\n\nCloses the given *file* handle. Returns 0 on success, or -1 in case of error.\n\nEquivalent to [`playdate.file.close()`](./Inside%20Playdate.html#f-file.close) in the Lua API."] pub close : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->read(SDFile* file, void* buf, unsigned int len);`\n\nReads up to *len* bytes from the *file* into the buffer *buf*. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.\n\nEquivalent to [`playdate.file.file:read()`](./Inside%20Playdate.html#m-file.read) in the Lua API."] pub read : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile , buf : * mut core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->write(SDFile* file, const void* buf, unsigned int len);`\n\nWrites the buffer of bytes *buf* to the *file*. Returns the number of bytes written, or -1 in case of error.\n\nEquivalent to [`playdate.file.file:write()`](./Inside%20Playdate.html#m-file.write) in the Lua API."] pub write : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile , buf : * const core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->flush(SDFile* file);`\n\nFlushes the output buffer of *file* immediately. Returns the number of bytes written, or -1 in case of error.\n\nEquivalent to [`playdate.file.flush()`](./Inside%20Playdate.html#f-file.flush) in the Lua API."] pub flush : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->tell(SDFile* file);`\n\nReturns the current read/write offset in the given *file* handle, or -1 on error.\n\nEquivalent to [`playdate.file.file:tell()`](./Inside%20Playdate.html#m-file.tell) in the Lua API."] pub tell : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->seek(SDFile* file, int pos, int whence);`\n\nSets the read/write offset in the given *file* handle to *pos*, relative to the *whence* macro. SEEK\\_SET is relative to the beginning of the file, SEEK\\_CUR is relative to the current position of the file pointer, and SEEK\\_END is relative to the end of the file. Returns 0 on success, -1 on error.\n\nEquivalent to [`playdate.file.file:seek()`](./Inside%20Playdate.html#m-file.seek) in the Lua API."] pub seek : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile , pos : core :: ffi :: c_int , whence : core :: ffi :: c_int) -> core :: ffi :: c_int > , }
#[test]
fn bindgen_test_layout_playdate_file() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_file> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_file>(),
	           104usize,
	           concat!("Size of: ", stringify!(playdate_file))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_file>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_file))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).geterr) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(geterr)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).listfiles) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(listfiles)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stat) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(stat)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).mkdir) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(mkdir)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).unlink) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(unlink)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).rename) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(rename)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).open) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(open)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).close) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(close)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).read) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(read)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).write) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(write)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).flush) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(flush)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).tell) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(tell)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).seek) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_file),
		"::",
		stringify!(seek)
	)
	);
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SpriteCollisionResponseType {
	kCollisionTypeSlide = 0,
	kCollisionTypeFreeze = 1,
	kCollisionTypeOverlap = 2,
	kCollisionTypeBounce = 3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
#[must_use]
pub struct PDRect {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
	pub width: core::ffi::c_float,
	pub height: core::ffi::c_float,
}
#[test]
fn bindgen_test_layout_PDRect() {
	const UNINIT: ::core::mem::MaybeUninit<PDRect> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PDRect>(),
	           16usize,
	           concat!("Size of: ", stringify!(PDRect))
	);
	assert_eq!(
	           ::core::mem::align_of::<PDRect>(),
	           4usize,
	           concat!("Alignment of ", stringify!(PDRect))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
	           0usize,
	           concat!("Offset of field: ", stringify!(PDRect), "::", stringify!(x))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
	           4usize,
	           concat!("Offset of field: ", stringify!(PDRect), "::", stringify!(y))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
	           8usize,
	           concat!("Offset of field: ", stringify!(PDRect), "::", stringify!(width))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
	           12usize,
	           concat!("Offset of field: ", stringify!(PDRect), "::", stringify!(height))
	);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
#[must_use]
pub struct CollisionPoint {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
}
#[test]
fn bindgen_test_layout_CollisionPoint() {
	const UNINIT: ::core::mem::MaybeUninit<CollisionPoint> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<CollisionPoint>(),
	           8usize,
	           concat!("Size of: ", stringify!(CollisionPoint))
	);
	assert_eq!(
	           ::core::mem::align_of::<CollisionPoint>(),
	           4usize,
	           concat!("Alignment of ", stringify!(CollisionPoint))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(CollisionPoint),
		"::",
		stringify!(x)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
	           4usize,
	           concat!(
		"Offset of field: ",
		stringify!(CollisionPoint),
		"::",
		stringify!(y)
	)
	);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct CollisionVector {
	pub x: core::ffi::c_int,
	pub y: core::ffi::c_int,
}
#[test]
fn bindgen_test_layout_CollisionVector() {
	const UNINIT: ::core::mem::MaybeUninit<CollisionVector> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<CollisionVector>(),
	           8usize,
	           concat!("Size of: ", stringify!(CollisionVector))
	);
	assert_eq!(
	           ::core::mem::align_of::<CollisionVector>(),
	           4usize,
	           concat!("Alignment of ", stringify!(CollisionVector))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(CollisionVector),
		"::",
		stringify!(x)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
	           4usize,
	           concat!(
		"Offset of field: ",
		stringify!(CollisionVector),
		"::",
		stringify!(y)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[must_use]
pub struct SpriteCollisionInfo {
	pub sprite: *mut LCDSprite,
	pub other: *mut LCDSprite,
	pub responseType: SpriteCollisionResponseType,
	pub overlaps: u8,
	pub ti: core::ffi::c_float,
	pub move_: CollisionPoint,
	pub normal: CollisionVector,
	pub touch: CollisionPoint,
	pub spriteRect: PDRect,
	pub otherRect: PDRect,
}
#[test]
fn bindgen_test_layout_SpriteCollisionInfo() {
	const UNINIT: ::core::mem::MaybeUninit<SpriteCollisionInfo> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<SpriteCollisionInfo>(),
	           88usize,
	           concat!("Size of: ", stringify!(SpriteCollisionInfo))
	);
	assert_eq!(
	           ::core::mem::align_of::<SpriteCollisionInfo>(),
	           8usize,
	           concat!("Alignment of ", stringify!(SpriteCollisionInfo))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sprite) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(sprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).other) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(other)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).responseType) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(responseType)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).overlaps) as usize - ptr as usize },
	           20usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(overlaps)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).ti) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(ti)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).move_) as usize - ptr as usize },
	           28usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(move_)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).normal) as usize - ptr as usize },
	           36usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(normal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).touch) as usize - ptr as usize },
	           44usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(touch)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).spriteRect) as usize - ptr as usize },
	           52usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(spriteRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).otherRect) as usize - ptr as usize },
	           68usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteCollisionInfo),
		"::",
		stringify!(otherRect)
	)
	);
}
impl Default for SpriteCollisionInfo {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[must_use]
pub struct SpriteQueryInfo {
	pub sprite: *mut LCDSprite,
	pub ti1: core::ffi::c_float,
	pub ti2: core::ffi::c_float,
	pub entryPoint: CollisionPoint,
	pub exitPoint: CollisionPoint,
}
#[test]
fn bindgen_test_layout_SpriteQueryInfo() {
	const UNINIT: ::core::mem::MaybeUninit<SpriteQueryInfo> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<SpriteQueryInfo>(),
	           32usize,
	           concat!("Size of: ", stringify!(SpriteQueryInfo))
	);
	assert_eq!(
	           ::core::mem::align_of::<SpriteQueryInfo>(),
	           8usize,
	           concat!("Alignment of ", stringify!(SpriteQueryInfo))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sprite) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteQueryInfo),
		"::",
		stringify!(sprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).ti1) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteQueryInfo),
		"::",
		stringify!(ti1)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).ti2) as usize - ptr as usize },
	           12usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteQueryInfo),
		"::",
		stringify!(ti2)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).entryPoint) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteQueryInfo),
		"::",
		stringify!(entryPoint)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).exitPoint) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(SpriteQueryInfo),
		"::",
		stringify!(exitPoint)
	)
	);
}
impl Default for SpriteQueryInfo {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
pub type LCDSpriteDrawFunction =
	::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, bounds: PDRect, drawrect: PDRect)>;
pub type LCDSpriteUpdateFunction = ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>;
pub type LCDSpriteCollisionFilterProc =
	::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                            other: *mut LCDSprite)
	                                            -> SpriteCollisionResponseType>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sprite {
	#[doc = "`void playdate->sprite->setAlwaysRedraw(int flag);`\n\nWhen *flag* is set to 1, this causes all sprites to draw each frame, whether or not they have been marked dirty. This may speed up the performance of your game if the system’s dirty rect tracking is taking up too much time - for example if there are many sprites moving around on screen at once."]
	pub setAlwaysRedraw: ::core::option::Option<unsafe extern "C" fn(flag: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->addDirtyRect(LCDRect dirtyRect);`\n\nMarks the given *dirtyRect* (in screen coordinates) as needing a redraw. Graphics drawing functions now call this automatically, adding their drawn areas to the sprite’s dirty list, so there’s usually no need to call this manually."]
	pub addDirtyRect: ::core::option::Option<unsafe extern "C" fn(dirtyRect: LCDRect)>,
	#[doc = "`void playdate->sprite->drawSprites(void);`\n\nDraws every sprite in the display list."]
	pub drawSprites: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->sprite->updateAndDrawSprites(void);`\n\nUpdates and draws every sprite in the display list."]
	pub updateAndDrawSprites: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`LCDSprite* playdate->sprite->newSprite(void);`\n\nAllocates and returns a new LCDSprite."]
	pub newSprite: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDSprite>,
	pub freeSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`LCDSprite* playdate->sprite->copy(LCDSprite *sprite);`\n\nAllocates and returns a copy of the given *sprite*."]
	pub copy: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut LCDSprite>,
	#[doc = "`void playdate->sprite->addSprite(LCDSprite *sprite);`\n\nAdds the given *sprite* to the display list, so that it is drawn in the current scene."]
	pub addSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`void playdate->sprite->removeSprite(LCDSprite *sprite);`\n\nRemoves the given *sprite* from the display list."]
	pub removeSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`void playdate->sprite->removeSprites(LCDSprite **sprites, int count);`\n\nRemoves the given *count* sized array of *sprites* from the display list."]
	pub removeSprites:
		::core::option::Option<unsafe extern "C" fn(sprites: *mut *mut LCDSprite, count: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->removeAllSprites(void);`\n\nRemoves all sprites from the display list."]
	pub removeAllSprites: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`int playdate->sprite->getSpriteCount(void);`\n\nReturns the total number of sprites in the display list."]
	pub getSpriteCount: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,
	#[doc = "`void playdate->sprite->setBounds(LCDSprite *sprite, PDRect bounds);`\n\nSets the bounds of the given *sprite* with *bounds*."]
	pub setBounds: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, bounds: PDRect)>,
	#[doc = "`PDRect playdate->sprite->getBounds(LCDSprite *sprite);`\n\nReturns the bounds of the given *sprite* as an PDRect;"]
	pub getBounds: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> PDRect>,
	#[doc = "`void playdate->sprite->moveTo(LCDSprite *sprite, float x, float y);`\n\nMoves the given *sprite* to *x*, *y* and resets its bounds based on the bitmap dimensions and center."]
	pub moveTo: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                        x: core::ffi::c_float,
	                                                        y: core::ffi::c_float)>,
	#[doc = "`void playdate->sprite->moveBy(LCDSprite *sprite, float dx, float dy);`\n\nMoves the given *sprite* to by offsetting its current position by *dx*, *dy*."]
	pub moveBy: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                        dx: core::ffi::c_float,
	                                                        dy: core::ffi::c_float)>,
	#[doc = "`void playdate->sprite->setImage(LCDSprite *sprite, LCDBitmap *image, LCDBitmapFlip flip);`\n\nSets the given *sprite*'s image to the given *bitmap*."]
	pub setImage: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                          image: *mut LCDBitmap,
	                                                          flip: LCDBitmapFlip)>,
	#[doc = "`LCDBitmap* playdate->sprite->getImage(LCDSprite *sprite);`\n\nReturns the LCDBitmap currently assigned to the given *sprite*."]
	pub getImage: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut LCDBitmap>,
	#[doc = "`void playdate->sprite->setSize(LCDSprite *s, float width, float height);`\n\nSets the size. The size is used to set the sprite’s bounds when calling moveTo()."]
	pub setSize: ::core::option::Option<unsafe extern "C" fn(s: *mut LCDSprite,
	                                                         width: core::ffi::c_float,
	                                                         height: core::ffi::c_float)>,
	#[doc = "`void playdate->sprite->setZIndex(LCDSprite *sprite, int16_t zIndex);`\n\nSets the Z order of the given *sprite*. Higher Z sprites are drawn on top of those with lower Z order."]
	pub setZIndex: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, zIndex: i16)>,
	#[doc = "`int16_t playdate->sprite->getZIndex(LCDSprite *sprite);`\n\nReturns the Z index of the given *sprite*."]
	pub getZIndex: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> i16>,
	#[doc = "`void playdate->sprite->setDrawMode(LCDSprite *sprite, LCDBitmapDrawMode mode);`\n\nSets the mode for drawing the sprite’s bitmap."]
	pub setDrawMode: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, mode: LCDBitmapDrawMode)>,
	#[doc = "`void playdate->sprite->setImageFlip(LCDSprite *sprite, LCDBitmapFlip flip);`\n\nFlips the bitmap."]
	pub setImageFlip: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flip: LCDBitmapFlip)>,
	#[doc = "`LCDBitmapFlip playdate->sprite->getImageFlip(LCDSprite *sprite);`\n\nReturns the flip setting of the sprite’s bitmap."]
	pub getImageFlip: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> LCDBitmapFlip>,
	#[doc = "`void playdate->sprite->setStencil(LCDSprite *sprite, LCDBitmap* stencil);`\n\nSpecifies a stencil image to be set on the frame buffer before the sprite is drawn."]
	pub setStencil: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, stencil: *mut LCDBitmap)>,
	#[doc = "`void playdate->sprite->setClipRect(LCDSprite *sprite, LCDRect clipRect);`\n\nSets the clipping rectangle for sprite drawing."]
	pub setClipRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, clipRect: LCDRect)>,
	#[doc = "`void playdate->sprite->clearClipRect(LCDSprite *sprite);`\n\nClears the sprite’s clipping rectangle."]
	pub clearClipRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`void playdate->sprite->setClipRectsInRange(LCDRect clipRect, int startZ, int endZ);`\n\nSets the clipping rectangle for *all* sprites with a Z index within *startZ* and *endZ* inclusive."]
	pub setClipRectsInRange: ::core::option::Option<unsafe extern "C" fn(clipRect: LCDRect,
	                                                                     startZ: core::ffi::c_int,
	                                                                     endZ: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->clearClipRectsInRange(int startZ, int endZ);`\n\nClears the clipping rectangle for *all* sprites with a Z index within *startZ* and *endZ* inclusive."]
	pub clearClipRectsInRange:
		::core::option::Option<unsafe extern "C" fn(startZ: core::ffi::c_int, endZ: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->setUpdatesEnabled(LCDSprite *sprite, int flag);`\n\nSet the updatesEnabled flag of the given *sprite* (determines whether the sprite has its update function called). One is true, 0 is false."]
	pub setUpdatesEnabled:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flag: core::ffi::c_int)>,
	#[doc = "`int playdate->sprite->updatesEnabled(LCDSprite *sprite);`\n\nGet the updatesEnabled flag of the given *sprite*."]
	pub updatesEnabled: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> core::ffi::c_int>,
	#[doc = "`void playdate->sprite->setCollisionsEnabled(LCDSprite *sprite, int flag);`\n\nSet the collisionsEnabled flag of the given *sprite* (along with the collideRect, this determines whether the sprite participates in collisions). One is true, 0 is false. Set to 1 by default."]
	pub setCollisionsEnabled:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flag: core::ffi::c_int)>,
	#[doc = "`int playdate->sprite->collisionsEnabled(LCDSprite *sprite);`\n\nGet the collisionsEnabled flag of the given *sprite*."]
	pub collisionsEnabled:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> core::ffi::c_int>,
	#[doc = "`void playdate->sprite->setVisible(LCDSprite *sprite, int flag);`\n\nSet the visible flag of the given *sprite* (determines whether the sprite has its draw function called). One is true, 0 is false."]
	pub setVisible: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flag: core::ffi::c_int)>,
	#[doc = "`int playdate->sprite->isVisible(LCDSprite *sprite);`\n\nGet the visible flag of the given *sprite*."]
	pub isVisible: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> core::ffi::c_int>,
	#[doc = "`void playdate->sprite->setOpaque(LCDSprite *sprite, int flag);`\n\nMarking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag."]
	pub setOpaque: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flag: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->markDirty(LCDSprite *sprite);`\n\nForces the given *sprite* to redraw."]
	pub markDirty: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`void playdate->sprite->setTag(LCDSprite *sprite, uint8_t tag);`\n\nSets the tag of the given *sprite*. This can be useful for identifying sprites or types of sprites when using the collision API."]
	pub setTag: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, tag: u8)>,
	#[doc = "`uint8_t playdate->sprite->getTag(LCDSprite *sprite);`\n\nReturns the tag of the given *sprite*."]
	pub getTag: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> u8>,
	#[doc = "`void playdate->sprite->setIgnoresDrawOffset(LCDSprite *sprite, int flag);`\n\nWhen *flag* is set to 1, the *sprite* will draw in screen coordinates, ignoring the currently-set drawOffset.\n\nThis only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space."]
	pub setIgnoresDrawOffset:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flag: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->setUpdateFunction(LCDSprite *sprite, LCDSpriteUpdateFunction *func);`\n\nSets the update function for the given *sprite*."]
	pub setUpdateFunction:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteUpdateFunction)>,
	#[doc = "`void playdate->sprite->setDrawFunction(LCDSprite *sprite, LCDSpriteDrawFunction *func);`\n\nSets the draw function for the given *sprite*. Note that the callback is only called when the sprite is on screen and has a size specified via [playdate→sprite→setSize()](#f-sprite.setSize) or [playdate→sprite→setBounds()](#f-sprite.setBounds)."]
	pub setDrawFunction:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteDrawFunction)>,
	#[doc = "`void playdate->sprite->getPosition(LCDSprite *sprite, float *x, float *y);`\n\nSets *x* and *y* to the current position of *sprite*."]
	pub getPosition: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                             x: *mut core::ffi::c_float,
	                                                             y: *mut core::ffi::c_float)>,
	#[doc = "`void playdate->sprite->resetCollisionWorld(void);`\n\nFrees and reallocates internal collision data, resetting everything to its default state."]
	pub resetCollisionWorld: ::core::option::Option<unsafe extern "C" fn()>,
	#[doc = "`void playdate->sprite->setCollideRect(LCDSprite *sprite, PDRect collideRect);`\n\nMarks the area of the given *sprite*, relative to its bounds, to be checked for collisions with other sprites' collide rects."]
	pub setCollideRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, collideRect: PDRect)>,
	#[doc = "`PDRect playdate->sprite->getCollideRect(LCDSprite *sprite);`\n\nReturns the given *sprite*’s collide rect."]
	pub getCollideRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> PDRect>,
	#[doc = "`void playdate->sprite->clearCollideRect(LCDSprite *sprite);`\n\nClears the given *sprite*’s collide rect."]
	pub clearCollideRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`void playdate->sprite->setCollisionResponseFunction(LCDSprite *sprite, LCDSpriteCollisionFilterProc *func);`\n\nSet a callback that returns a [SpriteCollisionResponseType](#_SpriteCollisionResponseType) for a collision between *sprite* and *other*.\n\nLCDSpriteCollisionFilterProc\n\n```cpp\ntypedef SpriteCollisionResponseType LCDSpriteCollisionFilterProc(LCDSprite* sprite, LCDSprite* other);\n```"]
	pub setCollisionResponseFunction:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteCollisionFilterProc)>,
	#[doc = "`SpriteCollisionInfo* playdate->sprite->checkCollisions(LCDSprite *sprite, float goalX, float goalY, float *actualX, float *actualY, int *len);`\n\nReturns the same values as [playdate-\\>sprite-\\>moveWithCollisions()](#f-sprite.moveWithCollisions) but does not actually move the sprite. The caller is responsible for freeing the returned array."]
	pub checkCollisions: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                                 goalX: core::ffi::c_float,
	                                                                 goalY: core::ffi::c_float,
	                                                                 actualX: *mut core::ffi::c_float,
	                                                                 actualY: *mut core::ffi::c_float,
	                                                                 len: *mut core::ffi::c_int)
	                                                                 -> *mut SpriteCollisionInfo>,
	#[doc = "`SpriteCollisionInfo* playdate->sprite->moveWithCollisions(LCDSprite *sprite, float goalX, float goalY, float *actualX, float *actualY, int *len);`\n\nMoves the given *sprite* towards *goalX*, *goalY* taking collisions into account and returns an array of SpriteCollisionInfo. *len* is set to the size of the array and *actualX*, *actualY* are set to the sprite’s position after collisions. If no collisions occurred, this will be the same as *goalX*, *goalY*. The caller is responsible for freeing the returned array."]
	pub moveWithCollisions: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                                    goalX: core::ffi::c_float,
	                                                                    goalY: core::ffi::c_float,
	                                                                    actualX: *mut core::ffi::c_float,
	                                                                    actualY: *mut core::ffi::c_float,
	                                                                    len: *mut core::ffi::c_int)
	                                                                    -> *mut SpriteCollisionInfo>,
	#[doc = "`LCDSprite** playdate->sprite->querySpritesAtPoint(float x, float y, int *len);`\n\nReturns an array of all sprites with collision rects containing the point at *x*, *y*. *len* is set to the size of the array. The caller is responsible for freeing the returned array."]
	pub querySpritesAtPoint: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_float,
	                                                                     y: core::ffi::c_float,
	                                                                     len: *mut core::ffi::c_int)
	                                                                     -> *mut *mut LCDSprite>,
	#[doc = "`LCDSprite** playdate->sprite->querySpritesInRect(float x, float y, float width, float height, int *len);`\n\nReturns an array of all sprites with collision rects that intersect the *width* by *height* rect at *x*, *y*. *len* is set to the size of the array. The caller is responsible for freeing the returned array."]
	pub querySpritesInRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_float,
	                                                                    y: core::ffi::c_float,
	                                                                    width: core::ffi::c_float,
	                                                                    height: core::ffi::c_float,
	                                                                    len: *mut core::ffi::c_int)
	                                                                    -> *mut *mut LCDSprite>,
	#[doc = "`LCDSprite** playdate->sprite->querySpritesAlongLine(float x1, float y1, float x2, float y2, int *len);`\n\nReturns an array of all sprites with collision rects that intersect the line connecting *x1*, *y1* and *x2*, *y2*. *len* is set to the size of the array. The caller is responsible for freeing the returned array."]
	pub querySpritesAlongLine: ::core::option::Option<unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                                       y1: core::ffi::c_float,
	                                                                       x2: core::ffi::c_float,
	                                                                       y2: core::ffi::c_float,
	                                                                       len: *mut core::ffi::c_int)
	                                                                       -> *mut *mut LCDSprite>,
	#[doc = "`SpriteQueryInfo* playdate->sprite->querySpriteInfoAlongLine(float x1, float y1, float x2, float y2, int *len);`\n\nReturns an array of SpriteQueryInfo for all sprites with collision rects that intersect the line connecting *x1*, *y1* and *x2*, *y2*. *len* is set to the size of the array. If you don’t need this information, use querySpritesAlongLine() as it will be faster. The caller is responsible for freeing the returned array."]
	pub querySpriteInfoAlongLine: ::core::option::Option<unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                                          y1: core::ffi::c_float,
	                                                                          x2: core::ffi::c_float,
	                                                                          y2: core::ffi::c_float,
	                                                                          len: *mut core::ffi::c_int)
	                                                                          -> *mut SpriteQueryInfo>,
	#[doc = "`LCDSprite** playdate->sprite->overlappingSprites(LCDSprite *sprite, int *len);`\n\nReturns an array of sprites that have collide rects that are currently overlapping the given *sprite*’s collide rect. *len* is set to the size of the array. The caller is responsible for freeing the returned array."]
	pub overlappingSprites: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                                    len: *mut core::ffi::c_int)
	                                                                    -> *mut *mut LCDSprite>,
	#[doc = "`LCDSprite** playdate->sprite->allOverlappingSprites(int *len);`\n\nReturns an array of all sprites that have collide rects that are currently overlapping. Each consecutive pair of sprites is overlapping (eg. 0 & 1 overlap, 2 & 3 overlap, etc). *len* is set to the size of the array. The caller is responsible for freeing the returned array."]
	pub allOverlappingSprites:
		::core::option::Option<unsafe extern "C" fn(len: *mut core::ffi::c_int) -> *mut *mut LCDSprite>,
	#[doc = "`void playdate->sprite->setStencilPattern(LCDSprite* sprite, uint8_t pattern[8]);`\n\nSets the sprite’s stencil to the given pattern."]
	pub setStencilPattern:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, pattern: *mut [u8; 8usize])>,
	#[doc = "`void playdate->sprite->clearStencil(LCDSprite *sprite);`\n\nClears the sprite’s stencil."]
	pub clearStencil: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
	#[doc = "`void playdate->sprite->setUserdata(LCDSprite *sprite, void* userdata);`"]
	pub setUserdata:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, userdata: *mut core::ffi::c_void)>,
	#[doc = "`void* playdate->sprite->getUserdata(LCDSprite *sprite);`\n\nSets and gets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data."]
	pub getUserdata:
		::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut core::ffi::c_void>,
	#[doc = "`void playdate->sprite->setStencilImage(LCDSprite *sprite, LCDBitmap* stencil, int tile);`\n\nSpecifies a stencil image to be set on the frame buffer before the sprite is drawn. If *tile* is set, the stencil will be tiled. Tiled stencils must have width evenly divisible by 32."]
	pub setStencilImage: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite,
	                                                                 stencil: *mut LCDBitmap,
	                                                                 tile: core::ffi::c_int)>,
	#[doc = "`void playdate->sprite->setCenter(LCDSprite *sprite, float x, float y);`\n\nSets the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width. Default is 0.5, 0.5 (the center of the sprite). This means that when you call [sprite→moveTo(sprite, x, y)](#f-sprite.moveTo), the center of your sprite will be positioned at *x*, *y*. If you want x and y to represent the upper left corner of your sprite, specify the center as 0, 0."]
	pub setCenter: ::core::option::Option<unsafe extern "C" fn(s: *mut LCDSprite,
	                                                           x: core::ffi::c_float,
	                                                           y: core::ffi::c_float)>,
	#[doc = "`void playdate->sprite->getCenter(LCDSprite *sprite, float *outx, float *outy);`\n\nSets the values in `outx` and `outy` to the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width."]
	pub getCenter: ::core::option::Option<unsafe extern "C" fn(s: *mut LCDSprite,
	                                                           x: *mut core::ffi::c_float,
	                                                           y: *mut core::ffi::c_float)>,
}
#[test]
fn bindgen_test_layout_playdate_sprite() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sprite> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sprite>(),
	           504usize,
	           concat!("Size of: ", stringify!(playdate_sprite))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sprite>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sprite))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAlwaysRedraw) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setAlwaysRedraw)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addDirtyRect) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(addDirtyRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).drawSprites) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(drawSprites)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).updateAndDrawSprites) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(updateAndDrawSprites)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSprite) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(newSprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeSprite) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(freeSprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).copy) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(copy)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addSprite) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(addSprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeSprite) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(removeSprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeSprites) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(removeSprites)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeAllSprites) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(removeAllSprites)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getSpriteCount) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getSpriteCount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setBounds) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setBounds)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getBounds) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getBounds)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).moveTo) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(moveTo)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).moveBy) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(moveBy)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setImage) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setImage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getImage) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getImage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setSize) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setSize)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setZIndex) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setZIndex)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getZIndex) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getZIndex)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDrawMode) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setDrawMode)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setImageFlip) as usize - ptr as usize },
	           176usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setImageFlip)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getImageFlip) as usize - ptr as usize },
	           184usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getImageFlip)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStencil) as usize - ptr as usize },
	           192usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setStencil)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setClipRect) as usize - ptr as usize },
	           200usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setClipRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearClipRect) as usize - ptr as usize },
	           208usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(clearClipRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setClipRectsInRange) as usize - ptr as usize },
	           216usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setClipRectsInRange)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearClipRectsInRange) as usize - ptr as usize },
	           224usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(clearClipRectsInRange)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUpdatesEnabled) as usize - ptr as usize },
	           232usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setUpdatesEnabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).updatesEnabled) as usize - ptr as usize },
	           240usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(updatesEnabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCollisionsEnabled) as usize - ptr as usize },
	           248usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setCollisionsEnabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).collisionsEnabled) as usize - ptr as usize },
	           256usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(collisionsEnabled)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVisible) as usize - ptr as usize },
	           264usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setVisible)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isVisible) as usize - ptr as usize },
	           272usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(isVisible)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOpaque) as usize - ptr as usize },
	           280usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setOpaque)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).markDirty) as usize - ptr as usize },
	           288usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(markDirty)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTag) as usize - ptr as usize },
	           296usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setTag)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTag) as usize - ptr as usize },
	           304usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getTag)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setIgnoresDrawOffset) as usize - ptr as usize },
	           312usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setIgnoresDrawOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUpdateFunction) as usize - ptr as usize },
	           320usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setUpdateFunction)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDrawFunction) as usize - ptr as usize },
	           328usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setDrawFunction)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getPosition) as usize - ptr as usize },
	           336usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getPosition)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).resetCollisionWorld) as usize - ptr as usize },
	           344usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(resetCollisionWorld)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCollideRect) as usize - ptr as usize },
	           352usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setCollideRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCollideRect) as usize - ptr as usize },
	           360usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getCollideRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearCollideRect) as usize - ptr as usize },
	           368usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(clearCollideRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCollisionResponseFunction) as usize - ptr as usize },
	           376usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setCollisionResponseFunction)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).checkCollisions) as usize - ptr as usize },
	           384usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(checkCollisions)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).moveWithCollisions) as usize - ptr as usize },
	           392usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(moveWithCollisions)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).querySpritesAtPoint) as usize - ptr as usize },
	           400usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(querySpritesAtPoint)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).querySpritesInRect) as usize - ptr as usize },
	           408usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(querySpritesInRect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).querySpritesAlongLine) as usize - ptr as usize },
	           416usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(querySpritesAlongLine)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).querySpriteInfoAlongLine) as usize - ptr as usize },
	           424usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(querySpriteInfoAlongLine)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).overlappingSprites) as usize - ptr as usize },
	           432usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(overlappingSprites)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).allOverlappingSprites) as usize - ptr as usize },
	           440usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(allOverlappingSprites)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStencilPattern) as usize - ptr as usize },
	           448usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setStencilPattern)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearStencil) as usize - ptr as usize },
	           456usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(clearStencil)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUserdata) as usize - ptr as usize },
	           464usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setUserdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getUserdata) as usize - ptr as usize },
	           472usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getUserdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStencilImage) as usize - ptr as usize },
	           480usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setStencilImage)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCenter) as usize - ptr as usize },
	           488usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(setCenter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCenter) as usize - ptr as usize },
	           496usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sprite),
		"::",
		stringify!(getCenter)
	)
	);
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SoundFormat {
	kSound8bitMono = 0,
	kSound8bitStereo = 1,
	kSound16bitMono = 2,
	kSound16bitStereo = 3,
	kSoundADPCMMono = 4,
	kSoundADPCMStereo = 5,
}
pub type MIDINote = core::ffi::c_float;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundSource {
	_unused: [u8; 0],
}
pub type sndCallbackProc =
	::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_source {
	#[doc = "`void playdate->sound->source->setVolume(SoundSource* c, float lvol, float rvol)`\n\nSets the playback volume (0.0 - 1.0) for left and right channels of the source."]
	pub setVolume: ::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource,
	                                                           lvol: core::ffi::c_float,
	                                                           rvol: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->source->getVolume(SoundSource* c, float* outlvol, float* outrvol)`\n\nGets the playback volume (0.0 - 1.0) for left and right channels of the source."]
	pub getVolume: ::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource,
	                                                           outl: *mut core::ffi::c_float,
	                                                           outr: *mut core::ffi::c_float)>,
	#[doc = "`int playdate->sound->source->isPlaying(SoundSource* c)`\n\nReturns 1 if the source is currently playing."]
	pub isPlaying: ::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource) -> core::ffi::c_int>,
	pub setFinishCallback: ::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource,
	                                                                   callback: sndCallbackProc,
	                                                                   userdata: *mut core::ffi::c_void)>,
}
#[test]
fn bindgen_test_layout_playdate_sound_source() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_source> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_source>(),
	           32usize,
	           concat!("Size of: ", stringify!(playdate_sound_source))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_source>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_source))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolume) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_source),
		"::",
		stringify!(setVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolume) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_source),
		"::",
		stringify!(getVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isPlaying) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_source),
		"::",
		stringify!(isPlaying)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFinishCallback) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_source),
		"::",
		stringify!(setFinishCallback)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct FilePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_fileplayer { # [doc = "`FilePlayer* playdate->sound->fileplayer->newPlayer(void);`\n\nAllocates a new FilePlayer."] pub newPlayer : :: core :: option :: Option < unsafe extern "C" fn () -> * mut FilePlayer > , # [doc = "`void playdate->sound->fileplayer->freePlayer(FilePlayer* player);`\n\nFrees the given *player*."] pub freePlayer : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) > , # [doc = "`int playdate->sound->fileplayer->loadIntoPlayer(FilePlayer* player, const char* path);`\n\nPrepares *player* to stream the file at *path*. Returns 1 if the file exists, otherwise 0."] pub loadIntoPlayer : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , path : * const core :: ffi :: c_char) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->fileplayer->setBufferLength(FilePlayer* player, float bufferLen);`\n\nSets the buffer length of *player* to *bufferLen* seconds;"] pub setBufferLength : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , bufferLen : core :: ffi :: c_float) > , # [doc = "`int playdate->sound->fileplayer->play(FilePlayer* player, int repeat);`\n\nStarts playing the file *player*. If *repeat* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with [playdate-\\>sound-\\>fileplayer-\\>stop()](#f-sound.fileplayer.stop). Returns 1 on success, 0 if buffer allocation failed."] pub play : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , repeat : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->sound->fileplayer->isPlaying(FilePlayer* player);`\n\nReturns one if *player* is playing, zero if not."] pub isPlaying : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->fileplayer->pause(FilePlayer* player);`\n\nPauses the file *player*."] pub pause : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) > , # [doc = "`void playdate->sound->fileplayer->stop(FilePlayer* player);`\n\nStops playing the file."] pub stop : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) > , # [doc = "`void playdate->sound->fileplayer->setVolume(FilePlayer* player, float left, float right);`\n\nSets the playback volume for left and right channels of *player*."] pub setVolume : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->fileplayer->getVolume(FilePlayer* player, float* outleft, float* outright);`\n\nGets the left and right channel playback volume for *player*."] pub getVolume : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , left : * mut core :: ffi :: c_float , right : * mut core :: ffi :: c_float) > , # [doc = "`float playdate->sound->fileplayer->getLength(FilePlayer* player);`\n\nReturns the length, in seconds, of the file loaded into *player*."] pub getLength : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float > , # [doc = "`void playdate->sound->fileplayer->setOffset(FilePlayer* player, float offset);`\n\nSets the current *offset* in seconds."] pub setOffset : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , offset : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->fileplayer->setRate(FilePlayer* player, float rate)`\n\nSets the playback *rate* for the *player*. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate \\< 0)."] pub setRate : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , rate : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->fileplayer->setLoopRange(FilePlayer* player, float start, float end);`\n\nSets the *start* and *end* of the loop region for playback, in seconds. If *end* is omitted, the end of the file is used."] pub setLoopRange : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , start : core :: ffi :: c_float , end : core :: ffi :: c_float) > , # [doc = "`int playdate->sound->fileplayer->didUnderrun(FilePlayer* player);`\n\nReturns one if *player* has underrun, zero if not."] pub didUnderrun : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->fileplayer->setFinishCallback(FilePlayer* player, sndCallbackProc callback, void* userdata);`\n\nSets a function to be called when playback has completed. This is an alias for [playdate→sound→source→setFinishCallback()](#f-sound.source.setFinishCallback).\n\nsndCallbackProc\n\n```cpp\ntypedef void sndCallbackProc(SoundSource* c, void* userdata);\n```"] pub setFinishCallback : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , callback : sndCallbackProc , userdata : * mut core :: ffi :: c_void) > , pub setLoopCallback : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , callback : sndCallbackProc , userdata : * mut core :: ffi :: c_void) > , # [doc = "`float playdate->sound->fileplayer->getOffset(FilePlayer* player);`\n\nReturns the current offset in seconds for *player*."] pub getOffset : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float > , # [doc = "`float playdate->sound->fileplayer->getRate(FilePlayer* player)`\n\nReturns the playback rate for *player*."] pub getRate : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float > , # [doc = "`void playdate->sound->fileplayer->setStopOnUnderrun(FilePlayer* player, int flag)`\n\nIf *flag* evaluates to true, the *player* will restart playback (after an audible stutter) as soon as data is available."] pub setStopOnUnderrun : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , flag : core :: ffi :: c_int) > , # [doc = "`void playdate->sound->fileplayer->fadeVolume(FilePlayer* player, float left, float right, int32_t len, sndCallbackProc finishCallback);`\n\nChanges the volume of the fileplayer to *left* and *right* over a length of *len* sample frames, then calls the provided callback (if set)."] pub fadeVolume : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float , len : i32 , finishCallback : sndCallbackProc , userdata : * mut core :: ffi :: c_void) > , pub setMP3StreamSource : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , dataSource : :: core :: option :: Option < unsafe extern "C" fn (data : * mut u8 , bytes : core :: ffi :: c_int , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_int > , userdata : * mut core :: ffi :: c_void , bufferLen : core :: ffi :: c_float) > , }
#[test]
fn bindgen_test_layout_playdate_sound_fileplayer() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_fileplayer> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_fileplayer>(),
	           176usize,
	           concat!("Size of: ", stringify!(playdate_sound_fileplayer))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_fileplayer>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_fileplayer))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newPlayer) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(newPlayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freePlayer) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(freePlayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadIntoPlayer) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(loadIntoPlayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setBufferLength) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setBufferLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).play) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(play)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isPlaying) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(isPlaying)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).pause) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(pause)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(stop)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolume) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolume) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(getVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLength) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(getLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOffset) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRate) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setRate)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLoopRange) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setLoopRange)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).didUnderrun) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(didUnderrun)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFinishCallback) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setFinishCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLoopCallback) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setLoopCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getOffset) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(getOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getRate) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(getRate)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStopOnUnderrun) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setStopOnUnderrun)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).fadeVolume) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(fadeVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMP3StreamSource) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_fileplayer),
		"::",
		stringify!(setMP3StreamSource)
	)
	);
}
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_sample {
	#[doc = "`AudioSample* playdate->sound->sample->newSampleBuffer(int length)`\n\nAllocates and returns a new AudioSample with a buffer large enough to load a file of *length* bytes."]
	pub newSampleBuffer:
		::core::option::Option<unsafe extern "C" fn(byteCount: core::ffi::c_int) -> *mut AudioSample>,
	#[doc = "`void playdate->sound->sample->loadIntoSample(AudioSample* sample, const char* path)`\n\nLoads the sound data from the file at *path* into an existing AudioSample, *sample*."]
	pub loadIntoSample: ::core::option::Option<unsafe extern "C" fn(sample: *mut AudioSample,
	                                                                path: *const core::ffi::c_char)
	                                                                -> core::ffi::c_int>,
	#[doc = "`AudioSample* playdate->sound->sample->load(const char* path)`\n\nAllocates and returns a new AudioSample, with the sound data loaded in memory. If there is no file at *path*, the function returns null."]
	pub load: ::core::option::Option<unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut AudioSample>,
	#[doc = "`AudioSample* playdate->sound->sample->newSampleFromData(uint8_t* data, SoundFormat format, uint32_t sampleRate, int byteCount, int shouldFreeData)`\n\nReturns a new AudioSample referencing the given audio data. If *shouldFreeData* is set, *data* is freed when the sample object is [freed](#f-sound.sample.freeSample). The sample keeps a pointer to the data instead of copying it, so the data must remain valid while the sample is active. *format* is one of the following values:\n\nSoundFormat\n\n```cpp\ntypedef enum\n{\n\tkSound8bitMono = 0,\n\tkSound8bitStereo = 1,\n\tkSound16bitMono = 2,\n\tkSound16bitStereo = 3,\n\tkSoundADPCMMono = 4,\n\tkSoundADPCMStereo = 5\n} SoundFormat;\n```\n\n`pd_api_sound.h` also provides some helper macros and functions:\n\n```cpp\n#define SoundFormatIsStereo(f) ((f)&1)\n#define SoundFormatIs16bit(f) ((f)>=kSound16bitMono)\nstatic inline uint32_t SoundFormat_bytesPerFrame(SoundFormat fmt);\n```"]
	pub newSampleFromData: ::core::option::Option<unsafe extern "C" fn(data: *mut u8,
	                                                                   format: SoundFormat,
	                                                                   sampleRate: u32,
	                                                                   byteCount: core::ffi::c_int,
	                                                                   shouldFreeData: core::ffi::c_int)
	                                                                   -> *mut AudioSample>,
	pub getData: ::core::option::Option<unsafe extern "C" fn(sample: *mut AudioSample,
	                                                         data: *mut *mut u8,
	                                                         format: *mut SoundFormat,
	                                                         sampleRate: *mut u32,
	                                                         bytelength: *mut u32)>,
	#[doc = "`void playdate->sound->sample->freeSample(AudioSample* sample)`\n\nFrees the given *sample*. If the sample was created with [playdate→sound→sample→newSampleFromData()](#f-sound.sample.newSampleFromData) and the *shouldFreeData* flag was set, the sample’s source data is also freed."]
	pub freeSample: ::core::option::Option<unsafe extern "C" fn(sample: *mut AudioSample)>,
	#[doc = "`float playdate->sound->sample->getLength(AudioSample* sample)`\n\nReturns the length, in seconds, of *sample*."]
	pub getLength: ::core::option::Option<unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_float>,
	#[doc = "`int playdate->sound->sample->decompress(void)`\n\nIf the sample is ADPCM compressed, decompresses the sample data to 16-bit PCM data. This increases the sample’s memory footprint by 4x and does not affect the quality in any way, but it is necessary if you want to use the sample in a synth or play the file backwards. Returns 1 if successful, 0 if there’s not enough memory for the uncompressed data."]
	pub decompress: ::core::option::Option<unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_int>,
}
#[test]
fn bindgen_test_layout_playdate_sound_sample() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_sample> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_sample>(),
	           64usize,
	           concat!("Size of: ", stringify!(playdate_sound_sample))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_sample>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_sample))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSampleBuffer) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(newSampleBuffer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadIntoSample) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(loadIntoSample)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).load) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(load)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSampleFromData) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(newSampleFromData)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getData) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(getData)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeSample) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(freeSample)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLength) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(getLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).decompress) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sample),
		"::",
		stringify!(decompress)
	)
	);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_sampleplayer {
	#[doc = "`SamplePlayer* playdate->sound->sampleplayer->newPlayer(void)`\n\nAllocates and returns a new SamplePlayer."]
	pub newPlayer: ::core::option::Option<unsafe extern "C" fn() -> *mut SamplePlayer>,
	#[doc = "`void playdate->sound->sampleplayer->freePlayer(SamplePlayer* player)`\n\nFrees the given *player*."]
	pub freePlayer: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer)>,
	#[doc = "`void playdate->sound->sampleplayer->setSample(SamplePlayer* player, AudioSample* sample)`\n\nAssigns *sample* to *player*."]
	pub setSample:
		::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample)>,
	#[doc = "`int playdate->sound->sampleplayer->play(SamplePlayer* player, int repeat, float rate)`\n\nStarts playing the sample in *player*.\n\nIf *repeat* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with [playdate-\\>sound-\\>sampleplayer-\\>stop()](#f-sound.sampleplayer.stop). If negative one, it does ping-pong looping.\n\n*rate* is the playback rate for the sample; 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.\n\nReturns 1 on success (which is always, currently)."]
	pub play: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer,
	                                                      repeat: core::ffi::c_int,
	                                                      rate: core::ffi::c_float)
	                                                      -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->sampleplayer->isPlaying(SamplePlayer* player)`\n\nReturns one if *player* is playing a sample, zero if not."]
	pub isPlaying: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->sampleplayer->stop(SamplePlayer* player)`\n\nStops playing the sample."]
	pub stop: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer)>,
	#[doc = "`void playdate->sound->sampleplayer->setVolume(SamplePlayer* player, float left, float right)`\n\nSets the playback volume for left and right channels."]
	pub setVolume: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer,
	                                                           left: core::ffi::c_float,
	                                                           right: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->sampleplayer->getVolume(SamplePlayer* player, float* outleft, float* outright)`\n\nGets the current left and right channel volume of the sampleplayer."]
	pub getVolume: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer,
	                                                           left: *mut core::ffi::c_float,
	                                                           right: *mut core::ffi::c_float)>,
	#[doc = "`float playdate->sound->sampleplayer->getLength(SamplePlayer* player)`\n\nReturns the length, in seconds, of the sample assigned to *player*."]
	pub getLength: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float>,
	#[doc = "`void playdate->sound->sampleplayer->setOffset(SamplePlayer* player, float offset)`\n\nSets the current *offset* of the SamplePlayer, in seconds."]
	pub setOffset:
		::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer, offset: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->sampleplayer->setRate(SamplePlayer* player, float rate)`\n\nSets the playback *rate* for the *player*. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files."]
	pub setRate: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer, rate: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->sampleplayer->setPlayRange(SamplePlayer* player, int start, int end)`\n\nWhen used with a repeat of -1, does ping-pong looping, with a *start* and *end* position in frames."]
	pub setPlayRange: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer,
	                                                              start: core::ffi::c_int,
	                                                              end: core::ffi::c_int)>,
	#[doc = "`void playdate->sound->sampleplayer->setFinishCallback(SamplePlayer* player, sndCallbackProc callback, void* userdata)`\n\nSets a function to be called when playback has completed. See [sndCallbackProc](#_sndCallbackProc)."]
	pub setFinishCallback: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer,
	                                                                   callback: sndCallbackProc,
	                                                                   userdata: *mut core::ffi::c_void)>,
	pub setLoopCallback: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer,
	                                                                 callback: sndCallbackProc,
	                                                                 userdata: *mut core::ffi::c_void)>,
	#[doc = "`float playdate->sound->sampleplayer->getOffset(SamplePlayer* player);`\n\nReturns the current offset in seconds for *player*."]
	pub getOffset: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float>,
	#[doc = "`float playdate->sound->sampleplayer->getRate(SamplePlayer* player)`\n\nReturns the playback rate for *player*."]
	pub getRate: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float>,
	#[doc = "`void playdate->sound->sampleplayer->setPaused(SamplePlayer* player, int paused)`\n\nPauses or resumes playback."]
	pub setPaused: ::core::option::Option<unsafe extern "C" fn(player: *mut SamplePlayer, flag: core::ffi::c_int)>,
}
#[test]
fn bindgen_test_layout_playdate_sound_sampleplayer() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_sampleplayer> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_sampleplayer>(),
	           136usize,
	           concat!("Size of: ", stringify!(playdate_sound_sampleplayer))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_sampleplayer>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_sampleplayer))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newPlayer) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(newPlayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freePlayer) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(freePlayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setSample) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setSample)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).play) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(play)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isPlaying) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(isPlaying)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(stop)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolume) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolume) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(getVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLength) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(getLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOffset) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRate) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setRate)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPlayRange) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setPlayRange)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFinishCallback) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setFinishCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLoopCallback) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setLoopCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getOffset) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(getOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getRate) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(getRate)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPaused) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sampleplayer),
		"::",
		stringify!(setPaused)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDSynthSignalValue {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDSynthSignal {
	_unused: [u8; 0],
}
pub type signalStepFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                      ioframes: *mut core::ffi::c_int,
                                                                      ifval: *mut core::ffi::c_float)
                                                                      -> core::ffi::c_float>;
pub type signalNoteOnFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                        note: MIDINote,
                                                                        vel: core::ffi::c_float,
                                                                        len: core::ffi::c_float)>;
pub type signalNoteOffFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                         stop: core::ffi::c_int,
                                                                         offset: core::ffi::c_int)>;
pub type signalDeallocFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_signal {
	#[doc = "`PDSynthSignal* playdate->sound->signal->newSignal(signalStepFunc step, signalNoteOnFunc noteOn, signalNoteOffFunc noteOff, signalDeallocFunc dealloc, void* userdata)`\n\nSignalCallbacks\n\n```cpp\ntypedef float (*signalStepFunc)(void* userdata, int* iosamples, float* ifval);\ntypedef void (*signalNoteOnFunc)(void* userdata, MIDINote note, float vel, float len); // len = -1 for indefinite\ntypedef void (*signalNoteOffFunc)(void* userdata, int stopped, int offset); // ended = 0 for note release, = 1 when note stops playing\ntypedef void (*signalDeallocFunc)(void* userdata);\n```\n\nProvides a custom implementation for the signal. *signalStepFunc step* is the only required function, returning the value at the end of the current frame. When called, the *ioframes* pointer contains the number of samples until the end of the frame. If the signal needs to provide a value in the middle of the frame (e.g. an LFO that needs to be sample-accurate) it should return the \"interframe\" value in *ifval* and set *iosamples* to the sample offset of the value. The functions are called on the audio render thread, so they should return as quickly as possible."]
	pub newSignal: ::core::option::Option<unsafe extern "C" fn(step: signalStepFunc,
	                                                           noteOn: signalNoteOnFunc,
	                                                           noteOff: signalNoteOffFunc,
	                                                           dealloc: signalDeallocFunc,
	                                                           userdata: *mut core::ffi::c_void)
	                                                           -> *mut PDSynthSignal>,
	#[doc = "`void playdate->sound->signal->freeSignal(PDSynthSignal* signal);`\n\nFrees a signal created with *playdate→sound→signal→newSignal()*."]
	pub freeSignal: ::core::option::Option<unsafe extern "C" fn(signal: *mut PDSynthSignal)>,
	#[doc = "`float playdate->sound->signal->getValue(PDSynthSignal* signal);`\n\nReturns the current output value of *signal*. The signal can be a custom signal created with newSignal(), or any of the PDSynthSignal subclasses."]
	pub getValue: ::core::option::Option<unsafe extern "C" fn(signal: *mut PDSynthSignal) -> core::ffi::c_float>,
	#[doc = "`void playdate->sound->signal->setValueScale(PDSynthSignal* signal, float scale);`\n\nScales the signal’s output by the given factor. The scale is applied before the offset."]
	pub setValueScale:
		::core::option::Option<unsafe extern "C" fn(signal: *mut PDSynthSignal, scale: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->signal->setValueOffset(PDSynthSignal* signal, float offset);`\n\nOffsets the signal’s output by the given amount."]
	pub setValueOffset:
		::core::option::Option<unsafe extern "C" fn(signal: *mut PDSynthSignal, offset: core::ffi::c_float)>,
	#[doc = "`PDSynthSignal* playdate->sound->signal->newSignalForValue(PDSynthSignalValue* value)`\n\nCreates a new PDSynthSignal that tracks a PDSynthSignalValue."]
	pub newSignalForValue:
		::core::option::Option<unsafe extern "C" fn(value: *mut PDSynthSignalValue) -> *mut PDSynthSignal>,
}
#[test]
fn bindgen_test_layout_playdate_sound_signal() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_signal> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_signal>(),
	           48usize,
	           concat!("Size of: ", stringify!(playdate_sound_signal))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_signal>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_signal))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSignal) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_signal),
		"::",
		stringify!(newSignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeSignal) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_signal),
		"::",
		stringify!(freeSignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getValue) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_signal),
		"::",
		stringify!(getValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setValueScale) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_signal),
		"::",
		stringify!(setValueScale)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setValueOffset) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_signal),
		"::",
		stringify!(setValueOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSignalForValue) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_signal),
		"::",
		stringify!(newSignalForValue)
	)
	);
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LFOType {
	kLFOTypeSquare = 0,
	kLFOTypeTriangle = 1,
	kLFOTypeSine = 2,
	kLFOTypeSampleAndHold = 3,
	kLFOTypeSawtoothUp = 4,
	kLFOTypeSawtoothDown = 5,
	kLFOTypeArpeggiator = 6,
	kLFOTypeFunction = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDSynthLFO {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_lfo { # [doc = "`PDSynthLFO* playdate->sound->lfo->newLFO(LFOType type)`\n\nReturns a new LFO object, which can be used to modulate sounds. The *type* argument is one of the following values:\n\nLFOType\n\n```cpp\ntypedef enum\n{\n\tkLFOTypeSquare,\n\tkLFOTypeTriangle,\n\tkLFOTypeSine,\n\tkLFOTypeSampleAndHold,\n\tkLFOTypeSawtoothUp,\n\tkLFOTypeSawtoothDown,\n\tkLFOTypeArpeggiator,\n\tkLFOTypeFunction\n} LFOType;\n```"] pub newLFO : :: core :: option :: Option < unsafe extern "C" fn (type_ : LFOType) -> * mut PDSynthLFO > , # [doc = "`void playdate->sound->lfo->freeLFO(PDSynthLFO* lfo)`\n\nFrees the LFO."] pub freeLFO : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO) > , # [doc = "`void playdate->sound->lfo->setType(PDSynthLFO* lfo, LFOType type)`\n\nSets the LFO shape to one of the values given above."] pub setType : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , type_ : LFOType) > , # [doc = "`void playdate->sound->lfo->setRate(PDSynthLFO* lfo, float rate)`\n\nSets the LFO’s rate, in cycles per second."] pub setRate : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , rate : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->lfo->setPhase(PDSynthLFO* lfo, float phase)`\n\nSets the LFO’s phase, from 0 to 1."] pub setPhase : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , phase : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->lfo->setCenter(PDSynthLFO* lfo, float center)`\n\nSets the center value for the LFO."] pub setCenter : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , center : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->lfo->setDepth(PDSynthLFO* lfo, float depth)`\n\nSets the depth of the LFO."] pub setDepth : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , depth : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->lfo->setArpeggiation(PDSynthLFO* lfo, int nSteps, float* steps)`\n\nSets the LFO type to arpeggio, where the given values are in half-steps from the center note. For example, the sequence (0, 4, 7, 12) plays the notes of a major chord."] pub setArpeggiation : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , nSteps : core :: ffi :: c_int , steps : * mut core :: ffi :: c_float) > , # [doc = "`void playdate->sound->lfo->setFunction(PDSynthLFO* lfo, float (*lfoFunc)(PDSynthLFO* lfo, void* userdata), void* userdata, int interpolate)`\n\nProvides a custom function for LFO values."] pub setFunction : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , lfoFunc : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_float > , userdata : * mut core :: ffi :: c_void , interpolate : core :: ffi :: c_int) > , # [doc = "`void playdate->sound->lfo->setDelay(PDSynthLFO* lfo, float holdoff, float ramptime)`\n\nSets an initial holdoff time for the LFO where the LFO remains at its center value, and a ramp time where the value increases linearly to its maximum depth. Values are in seconds."] pub setDelay : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , holdoff : core :: ffi :: c_float , ramptime : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->lfo->setRetrigger(PDSynthLFO* lfo, int flag)`\n\nIf retrigger is on, the LFO’s phase is reset to its initial phase (default 0) when a synth using the LFO starts playing a note."] pub setRetrigger : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , flag : core :: ffi :: c_int) > , # [doc = "`float playdate->sound->lfo->getValue(PDSynthLFO* lfo)`\n\nReturn the current output value of the LFO."] pub getValue : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO) -> core :: ffi :: c_float > , # [doc = "`void playdate->sound->lfo->setGlobal(PDSynthLFO* lfo, int global)`\n\nIf *global* is set, the LFO is continuously updated whether or not it’s currently in use."] pub setGlobal : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , global : core :: ffi :: c_int) > , # [doc = "`void playdate->sound->lfo->setStartPhase(PDSynthLFO* lfo, float phase)`\n\nSets the LFO’s initial phase, from 0 to 1."] pub setStartPhase : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut PDSynthLFO , phase : core :: ffi :: c_float) > , }
#[test]
fn bindgen_test_layout_playdate_sound_lfo() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_lfo> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_lfo>(),
	           112usize,
	           concat!("Size of: ", stringify!(playdate_sound_lfo))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_lfo>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_lfo))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newLFO) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(newLFO)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeLFO) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(freeLFO)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setType) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setType)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRate) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setRate)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPhase) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setPhase)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCenter) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setCenter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDepth) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setDepth)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setArpeggiation) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setArpeggiation)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFunction) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setFunction)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDelay) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setDelay)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRetrigger) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setRetrigger)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getValue) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(getValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setGlobal) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setGlobal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setStartPhase) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_lfo),
		"::",
		stringify!(setStartPhase)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDSynthEnvelope {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_envelope {
	#[doc = "`PDSynthEnvelope* playdate->sound->envelope->newEnvelope(float attack, float decay, float sustain, float release)`\n\nCreates a new envelope with the given parameters."]
	pub newEnvelope: ::core::option::Option<unsafe extern "C" fn(attack: core::ffi::c_float,
	                                                             decay: core::ffi::c_float,
	                                                             sustain: core::ffi::c_float,
	                                                             release: core::ffi::c_float)
	                                                             -> *mut PDSynthEnvelope>,
	#[doc = "`void playdate->sound->envelope->freeEnvelope(PDSynthEnvelope* env)`\n\nFrees the envelope."]
	pub freeEnvelope: ::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope)>,
	#[doc = "`void playdate->sound->envelope->setAttack(PDSynthEnvelope* env, float attack)`"]
	pub setAttack:
		::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, attack: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->envelope->setDecay(PDSynthEnvelope* env, float decay)`"]
	pub setDecay:
		::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, decay: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->envelope->setSustain(PDSynthEnvelope* env, float sustain)`"]
	pub setSustain:
		::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, sustain: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->envelope->setRelease(PDSynthEnvelope* env, float release)`\n\nSets the ADSR parameters of the envelope."]
	pub setRelease:
		::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, release: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->envelope->setLegato(PDSynthEnvelope* env, int flag)`\n\nSets whether to use legato phrasing for the envelope. If the legato flag is set, when the envelope is re-triggered before it’s released, it remains in the sustain phase instead of jumping back to the attack phase."]
	pub setLegato: ::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, flag: core::ffi::c_int)>,
	#[doc = "`void playdate->sound->envelope->setRetrigger(PDSynthEnvelope* env, int flag)`\n\nIf retrigger is on, the envelope always starts from 0 when a note starts playing, instead of the current value if it’s active."]
	pub setRetrigger:
		::core::option::Option<unsafe extern "C" fn(lfo: *mut PDSynthEnvelope, flag: core::ffi::c_int)>,
	#[doc = "`float playdate->sound->envelope->getValue(PDSynthEnvelope* env)`\n\nReturn the current output value of the envelope."]
	pub getValue: ::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope) -> core::ffi::c_float>,
	#[doc = "`void playdate->sound->envelope->setCurvature(PDSynthEnvelope* env, float amount)`\n\nSmoothly changes the envelope’s shape from linear (amount=0) to exponential (amount=1)."]
	pub setCurvature:
		::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, amount: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->envelope->setVelocitySensitivity(PDSynthEnvelope* env, float velsens)`\n\nChanges the amount by which note velocity scales output level. At the default value of 1, output is proportional to velocity; at 0 velocity has no effect on output level."]
	pub setVelocitySensitivity:
		::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope, velsens: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->envelope->setRateScaling(PDSynthEnvelope* env, float scaling, MIDINote start, MIDINote end)`\n\nScales the envelope rate according to the played note. For notes below `start`, the envelope’s set rate is used; for notes above `end` envelope rates are scaled by the `scaling` parameter. Between the two notes the scaling factor is interpolated from 1.0 to `scaling`."]
	pub setRateScaling: ::core::option::Option<unsafe extern "C" fn(env: *mut PDSynthEnvelope,
	                                                                scaling: core::ffi::c_float,
	                                                                start: MIDINote,
	                                                                end: MIDINote)>,
}
#[test]
fn bindgen_test_layout_playdate_sound_envelope() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_envelope> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_envelope>(),
	           96usize,
	           concat!("Size of: ", stringify!(playdate_sound_envelope))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_envelope>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_envelope))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newEnvelope) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(newEnvelope)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeEnvelope) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(freeEnvelope)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAttack) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setAttack)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDecay) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setDecay)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setSustain) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setSustain)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRelease) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setRelease)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLegato) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setLegato)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRetrigger) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setRetrigger)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getValue) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(getValue)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCurvature) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setCurvature)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVelocitySensitivity) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setVelocitySensitivity)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRateScaling) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_envelope),
		"::",
		stringify!(setRateScaling)
	)
	);
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum SoundWaveform {
	kWaveformSquare = 0,
	kWaveformTriangle = 1,
	kWaveformSine = 2,
	kWaveformNoise = 3,
	kWaveformSawtooth = 4,
	kWaveformPOPhase = 5,
	kWaveformPODigital = 6,
	kWaveformPOVosim = 7,
}
pub type synthRenderFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                       left: *mut i32,
                                                                       right: *mut i32,
                                                                       nsamples: core::ffi::c_int,
                                                                       rate: u32,
                                                                       drate: i32)
                                                                       -> core::ffi::c_int>;
pub type synthNoteOnFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                       note: MIDINote,
                                                                       velocity: core::ffi::c_float,
                                                                       len: core::ffi::c_float)>;
pub type synthReleaseFunc =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void, stop: core::ffi::c_int)>;
pub type synthSetParameterFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                             parameter: core::ffi::c_int,
                                                                             value: core::ffi::c_float)
                                                                             -> core::ffi::c_int>;
pub type synthDeallocFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
pub type synthCopyUserdata =
	::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDSynth {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_synth {
	#[doc = "`PDSynth* playdate->sound->synth->newSynth(void)`\n\nCreates a new synth object."]
	pub newSynth: ::core::option::Option<unsafe extern "C" fn() -> *mut PDSynth>,
	#[doc = "`void playdate->sound->synth->freeSynth(PDSynth* synth)`\n\nFrees a synth object, first removing it from the sound engine if needed."]
	pub freeSynth: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth)>,
	#[doc = "`void playdate->sound->synth->setWaveform(PDSynth* synth, SoundWaveform wave)`\n\nSets the waveform of the synth. The SoundWaveform enum contains the following values:\n\nSoundWaveform\n\n```cpp\ntypedef enum\n{\n\tkWaveformSquare,\n\tkWaveformTriangle,\n\tkWaveformSine,\n\tkWaveformNoise,\n\tkWaveformSawtooth,\n\tkWaveformPOPhase,\n\tkWaveformPODigital,\n\tkWaveformPOVosim\n} SoundWaveform;\n```"]
	pub setWaveform: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, wave: SoundWaveform)>,
	pub setGenerator_deprecated: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                                         stereo: core::ffi::c_int,
	                                                                         render: synthRenderFunc,
	                                                                         noteOn: synthNoteOnFunc,
	                                                                         release: synthReleaseFunc,
	                                                                         setparam: synthSetParameterFunc,
	                                                                         dealloc: synthDeallocFunc,
	                                                                         userdata: *mut core::ffi::c_void)>,
	#[doc = "`void playdate->sound->synth->setSample(PDSynth* synth, AudioSample* sample, uint32_t sustainStart, uint32_t sustainEnd)`\n\nProvides a sample for the synth to play. Sample data must be uncompressed PCM, not ADPCM. If a sustain range is set, it is looped while the synth is playing a note. When the note ends, if an envelope has been set on the synth and the sustain range goes to the end of the sample (i.e. there’s no release section of the sample after the sustain range) then the sustain section continues looping during the envelope release; otherwise it plays through the end of the sample and stops. As a convenience, if `sustainEnd` is zero and `sustainStart` is greater than zero, `sustainEnd` will be set to the length of the sample."]
	pub setSample: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                           sample: *mut AudioSample,
	                                                           sustainStart: u32,
	                                                           sustainEnd: u32)>,
	#[doc = "`void playdate->sound->synth->setAttackTime(PDSynth* synth, float attack)`"]
	pub setAttackTime:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, attack: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->synth->setDecayTime(PDSynth* synth, float decay)`"]
	pub setDecayTime: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, decay: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->synth->setSustainLevel(PDSynth* synth, float sustain)`"]
	pub setSustainLevel:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, sustain: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->synth->setReleaseTime(PDSynth* synth, float release)`\n\nSets the parameters of the synth’s ADSR envelope."]
	pub setReleaseTime:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, release: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->synth->setTranspose(PDSynth* synth, float halfSteps)`\n\nTransposes the synth’s output by the given number of half steps. For example, if the transpose is set to 2 and a C note is played, the synth will output a D instead."]
	pub setTranspose:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, halfSteps: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->synth->setFrequencyModulator(PDSynth* synth, PDSynthSignalValue* mod)`\n\nSets a [signal](#C-sound.signal) to modulate the synth’s frequency. The signal is scaled so that a value of 1 doubles the synth pitch (i.e. an octave up) and -1 halves it (an octave down). Set to *NULL* to clear the modulator."]
	pub setFrequencyModulator:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->synth->getFrequencyModulator(PDSynth* synth)`\n\nReturns the currently set frequency modulator."]
	pub getFrequencyModulator:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->synth->setAmplitudeModulator(PDSynth* synth, PDSynthSignalValue* mod)`\n\nSets a [signal](#C-sound.signal) to modulate the synth’s output amplitude. Set to *NULL* to clear the modulator."]
	pub setAmplitudeModulator:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->synth->getAmplitudeModulator(PDSynth* synth)`\n\nReturns the currently set amplitude modulator."]
	pub getAmplitudeModulator:
		::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthSignalValue>,
	#[doc = "`int playdate->sound->synth->getParameterCount(PDSynth* synth)`\n\nReturns the number of parameters advertised by the synth."]
	pub getParameterCount: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth) -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->synth->setParameter(PDSynth* synth, int num, float value)`\n\nSets the (1-based) parameter at position *num* to the given value. Returns 0 if *num* is not a valid parameter index."]
	pub setParameter: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                              parameter: core::ffi::c_int,
	                                                              value: core::ffi::c_float)
	                                                              -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->synth->setParameterModulator(PDSynth* synth, int num, PDSynthSignalValue* mod)`\n\nSets a [signal](#C-sound.signal) to modulate the parameter at index *num*. Set to *NULL* to clear the modulator."]
	pub setParameterModulator: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                                       parameter: core::ffi::c_int,
	                                                                       mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->synth->getParameterModulator(PDSynth* synth, int num)`\n\nReturns the currently set parameter modulator for the given index."]
	pub getParameterModulator: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                                       parameter: core::ffi::c_int)
	                                                                       -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->synth->playNote(PDSynth* synth, float freq, float vel, float len, uint32_t when)`\n\nPlays a note on the synth, at the given frequency. Specify *len* = -1 to leave the note playing until a subsequent noteOff() call. If *when* is 0, the note is played immediately, otherwise the note is scheduled for the given time. Use [playdate→sound→getCurrentTime()](#f-sound.getCurrentTime) to get the current time."]
	pub playNote: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                          freq: core::ffi::c_float,
	                                                          vel: core::ffi::c_float,
	                                                          len: core::ffi::c_float,
	                                                          when: u32)>,
	#[doc = "`void playdate->sound->synth->playMIDINote(PDSynth* synth, MIDINote note, float vel, float len, uint32_t when)`\n\nThe same as [playNote](#f-sound.synth.playNote) but uses MIDI note (where 60 = C4) instead of frequency. Note that `MIDINote` is a typedef for `float', meaning fractional values are allowed (for all you microtuning enthusiasts)."]
	pub playMIDINote: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                              note: MIDINote,
	                                                              vel: core::ffi::c_float,
	                                                              len: core::ffi::c_float,
	                                                              when: u32)>,
	#[doc = "`void playdate->sound->synth->noteOff(PDSynth* synth, uint32_t when)`\n\nSends a note off event to the synth, either immediately (*when* = 0) or at the scheduled time."]
	pub noteOff: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth, when: u32)>,
	pub stop: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth)>,
	#[doc = "`void playdate->sound->synth->setVolume(PDSynth* synth, float lvol, float rvol)`\n\nSets the playback volume (0.0 - 1.0) for the left and, if the synth is stereo, right channels of the synth. This is equivalent to\n\n```cpp\nplaydate->sound->source->setVolume((SoundSource*)synth, lvol, rvol);\n```"]
	pub setVolume: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                           left: core::ffi::c_float,
	                                                           right: core::ffi::c_float)>,
	#[doc = "`float playdate->sound->synth->getVolume(PDSynth* synth, float* outlvol, float* outrvol)`\n\nGets the playback volume for the left and right (if stereo) channels of the synth. This is equivalent to\n\n```cpp\nplaydate->sound->source->getVolume((SoundSource*)synth, outlvol, outrvol);\n```"]
	pub getVolume: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                           left: *mut core::ffi::c_float,
	                                                           right: *mut core::ffi::c_float)>,
	#[doc = "`int playdate->sound->synth->isPlaying(PDSynth* synth)`\n\nReturns 1 if the synth is currently playing."]
	pub isPlaying: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth) -> core::ffi::c_int>,
	#[doc = "`PDSynthEnvelope* playdate->sound->synth->getEnvelope(PDSynth* synth)`\n\nReturns the synth’s envelope. The PDSynth object owns this envelope, so it must not be freed."]
	pub getEnvelope: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthEnvelope>,
	#[doc = "`int playdate->sound->synth->setWavetable(PDSynth* synth, AudioSample* sample, int log2size, int columns, rows)`\n\nSets a wavetable for the synth to play. Sample data must be 16-bit mono uncompressed. `log2size` is the base 2 logarithm of the number of samples in each waveform \"cell\" in the table, and `columns` and `rows` gives the number of cells in each direction; for example, if the wavetable is arranged in an 8x8 grid, `columns` and `rows` are both 8 and `log2size` is 6, since 2^6 = 8x8.\n\nThe function returns 1 on success. If it fails, use [playdate→sound→getError()](#f-sound.getError) to get a human-readable error message.\n\nThe synth’s \"position\" in the wavetable is set manually with [setParameter()](#f-sound.synth.setParameter) or automated with [setParameterModulator()](#f-sound.synth.setParameterModulator). In some cases it’s easier to use a parameter that matches the waveform position in the table, in others (notably when using envelopes and lfos) it’s more convenient to use a 0-1 scale, so there’s some redundancy here. Parameters are\n\n* 1: x position, values are from 0 to the table width\n\n* 2: x position, values are from 0 to 1, parameter is scaled up to table width\n\nFor 2-D tables (`height` \\> 1):\n\n* 3: y position, values are from 0 to the table height\n\n* 4: y position, values are from 0 to 1, parameter is scaled up to table height"]
	pub setWavetable: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                              sample: *mut AudioSample,
	                                                              log2size: core::ffi::c_int,
	                                                              columns: core::ffi::c_int,
	                                                              rows: core::ffi::c_int)
	                                                              -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->synth->setGenerator(PDSynth* synth, int stereo, synthRenderFunc* render, synthNoteOnFunc* noteOn, synthReleaseFunc* release, synthSetParameterFunc* setparam, synthDeallocFunc* dealloc, synthCopyUserdataFunc copyUserdata, void* userdata)`\n\nGeneratorCallbacks\n\n```cpp\ntypedef int (*synthRenderFunc)(void* userdata, int32_t* left, int32_t* right, int nsamples, uint32_t rate, int32_t drate);\ntypedef void (*synthNoteOnFunc)(void* userdata, MIDINote note, float velocity, float len); // len == -1 if indefinite\ntypedef void (*synthReleaseFunc)(void* userdata, int endoffset);\ntypedef int (*synthSetParameterFunc)(void* userdata, int parameter, float value);\ntypedef void (*synthDeallocFunc)(void* userdata);\ntypedef void* (*synthCopyUserdata)(void* userdata);\n```\n\nProvides custom waveform generator functions for the synth. These functions are called on the audio render thread, so they should return as quickly as possible. *synthRenderFunc*, the data provider callback, is the only required function.\n\n*synthRenderFunc*: called every audio cycle to get the samples for playback. *left* (and *right* if *setGenerator()* was called with the stereo flag set) are sample buffers in Q8.24 format. *rate* is the amount to change a (Q32) phase accumulator each sample, and *drate* is the amount to change *rate* each sample. Custom synths can ignore this and use the *note* paramter in the noteOn function to handle pitch, but synth→setFrequencyModulator() won’t work as expected.\n\n*synthNoteOnFunc*: called when the synth receives a note on event. *len* is the length of the note in seconds, or -1 if it’s not known yet when the note will end.\n\n*synthReleaseFunc*: called when the synth receives a note off event. *endoffset* is how many samples into the current render cycle the note ends, allowing for sample-accurate timing.\n\n*synthSetParameterFunc*: called when a parameter change is received from [synth→setParameter()](#f-sound.synth.setParameter) or a modulator.\n\n*synthDeallocFunc*: called when the synth is being dealloced. This function should free anything that was allocated for the synth and also free the *userdata* itself.\n\n*synthCopyUserdata*: called when [synth→copy()](#f-sound.synth.copy) needs a unique copy of the synth’s userdata. External objects should be retained or copied so that the object isn’t freed while the synth is still using it."]
	pub setGenerator: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth,
	                                                              stereo: core::ffi::c_int,
	                                                              render: synthRenderFunc,
	                                                              noteOn: synthNoteOnFunc,
	                                                              release: synthReleaseFunc,
	                                                              setparam: synthSetParameterFunc,
	                                                              dealloc: synthDeallocFunc,
	                                                              copyUserdata: synthCopyUserdata,
	                                                              userdata: *mut core::ffi::c_void)>,
	#[doc = "`PDSynth* playdate->sound->synth->copy(PDSynth* synth)`\n\nReturns a copy of the given synth. Caller assumes ownership of the returned object and should free it when it is no longer in use."]
	pub copy: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynth>,
	#[doc = "`void playdate->sound->synth->clearEnvelope(PDSynth* synth)`\n\nClears the synth’s envelope settings."]
	pub clearEnvelope: ::core::option::Option<unsafe extern "C" fn(synth: *mut PDSynth)>,
}
#[test]
fn bindgen_test_layout_playdate_sound_synth() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_synth> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_synth>(),
	           240usize,
	           concat!("Size of: ", stringify!(playdate_sound_synth))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_synth>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_synth))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSynth) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(newSynth)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeSynth) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(freeSynth)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setWaveform) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setWaveform)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setGenerator_deprecated) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setGenerator_deprecated)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setSample) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setSample)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAttackTime) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setAttackTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setDecayTime) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setDecayTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setSustainLevel) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setSustainLevel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setReleaseTime) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setReleaseTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTranspose) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setTranspose)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFrequencyModulator) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setFrequencyModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFrequencyModulator) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(getFrequencyModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAmplitudeModulator) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setAmplitudeModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getAmplitudeModulator) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(getAmplitudeModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getParameterCount) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(getParameterCount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setParameter) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setParameter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setParameterModulator) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setParameterModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getParameterModulator) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(getParameterModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).playNote) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(playNote)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).playMIDINote) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(playMIDINote)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).noteOff) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(noteOff)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(stop)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolume) as usize - ptr as usize },
	           176usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolume) as usize - ptr as usize },
	           184usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(getVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isPlaying) as usize - ptr as usize },
	           192usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(isPlaying)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getEnvelope) as usize - ptr as usize },
	           200usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(getEnvelope)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setWavetable) as usize - ptr as usize },
	           208usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setWavetable)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setGenerator) as usize - ptr as usize },
	           216usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(setGenerator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).copy) as usize - ptr as usize },
	           224usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(copy)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearEnvelope) as usize - ptr as usize },
	           232usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_synth),
		"::",
		stringify!(clearEnvelope)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct ControlSignal {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_control_signal {
	#[doc = "`ControlSignal* playdate->sound->controlsignal->newSignal(void)`\n\nCreates a new control signal object."]
	pub newSignal: ::core::option::Option<unsafe extern "C" fn() -> *mut ControlSignal>,
	#[doc = "`void playdate->sound->controlsignal->freeSignal(ControlSignal* signal)`\n\nFrees the given signal."]
	pub freeSignal: ::core::option::Option<unsafe extern "C" fn(signal: *mut ControlSignal)>,
	#[doc = "`void playdate->sound->controlsignal->clearEvents(ControlSignal* signal)`\n\nClears all events from the given signal."]
	pub clearEvents: ::core::option::Option<unsafe extern "C" fn(control: *mut ControlSignal)>,
	#[doc = "`void playdate->sound->controlsignal->addEvent(ControlSignal* signal, int step, float value, int interpolate)`\n\nAdds a value to the signal’s timeline at the given step. If *interpolate* is set, the value is interpolated between the previous step+value and this one."]
	pub addEvent: ::core::option::Option<unsafe extern "C" fn(control: *mut ControlSignal,
	                                                          step: core::ffi::c_int,
	                                                          value: core::ffi::c_float,
	                                                          interpolate: core::ffi::c_int)>,
	#[doc = "`void playdate->sound->controlsignal->removeEvent(ControlSignal* signal, int step)`\n\nRemoves the control event at the given step."]
	pub removeEvent:
		::core::option::Option<unsafe extern "C" fn(control: *mut ControlSignal, step: core::ffi::c_int)>,
	#[doc = "`int playdate->sound->controlsignal->getMIDIControllerNumber(ControlSignal* signal)`\n\nReturns the MIDI controller number for this ControlSignal, if it was created from a MIDI file via [playdate→sound→sequence→loadMIDIFile()](#f-sound.sequence.loadMIDIFile)."]
	pub getMIDIControllerNumber:
		::core::option::Option<unsafe extern "C" fn(control: *mut ControlSignal) -> core::ffi::c_int>,
}
#[test]
fn bindgen_test_layout_playdate_control_signal() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_control_signal> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_control_signal>(),
	           48usize,
	           concat!("Size of: ", stringify!(playdate_control_signal))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_control_signal>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_control_signal))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSignal) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_control_signal),
		"::",
		stringify!(newSignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeSignal) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_control_signal),
		"::",
		stringify!(freeSignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearEvents) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_control_signal),
		"::",
		stringify!(clearEvents)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addEvent) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_control_signal),
		"::",
		stringify!(addEvent)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeEvent) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_control_signal),
		"::",
		stringify!(removeEvent)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getMIDIControllerNumber) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_control_signal),
		"::",
		stringify!(getMIDIControllerNumber)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDSynthInstrument {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_instrument {
	#[doc = "`PDSynthInstrument* playdate->sound->instrument->newInstrument(void)`\n\nCreates a new PDSynthInstrument object."]
	pub newInstrument: ::core::option::Option<unsafe extern "C" fn() -> *mut PDSynthInstrument>,
	#[doc = "`void playdate->sound->instrument->freeInstrument(PDSynthInstrument* instrument)`\n\nFrees the given instrument, first removing it from the sound engine if needed."]
	pub freeInstrument: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument)>,
	#[doc = "`int playdate->sound->instrument->addVoice(PDSynthInstrument* instrument, PDSynth* synth, MIDINote rangeStart, MIDINote rangeEnd, float transpose)`\n\nAdds the given [PDSynth](#C-sound.synth) to the instrument. The synth will respond to playNote events between *rangeState* and *rangeEnd*, inclusive. The *transpose* argument is in half-step units, and is added to the instrument’s [transpose](#f-sound.instrument.setTranspose) parameter. The function returns 1 if successful, or 0 if the synth is already in another instrument or channel."]
	pub addVoice: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument,
	                                                          synth: *mut PDSynth,
	                                                          rangeStart: MIDINote,
	                                                          rangeEnd: MIDINote,
	                                                          transpose: core::ffi::c_float)
	                                                          -> core::ffi::c_int>,
	#[doc = "`PDSynth* playdate->sound->instrument->playNote(PDSynthInstrument* instrument, float frequency, float vel, float len, uint32_t when)`"]
	pub playNote: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument,
	                                                          frequency: core::ffi::c_float,
	                                                          vel: core::ffi::c_float,
	                                                          len: core::ffi::c_float,
	                                                          when: u32)
	                                                          -> *mut PDSynth>,
	#[doc = "`PDSynth* playdate->sound->instrument->playMIDINote(PDSynthInstrument* instrument, MIDINote note, float vel, float len, uint32_t when)`\n\nThe instrument passes the playNote/playMIDINote() event to the synth in its collection that has been off for the longest, or has been playing longest if all synths are currently playing. See also [playdate→sound→synth→playNote()](#f-sound.synth.playNote). The PDSynth that received the playNote event is returned."]
	pub playMIDINote: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument,
	                                                              note: MIDINote,
	                                                              vel: core::ffi::c_float,
	                                                              len: core::ffi::c_float,
	                                                              when: u32)
	                                                              -> *mut PDSynth>,
	#[doc = "`void playdate->sound->instrument->setPitchBend(PDSynthInstrument* instrument, float amount)`\n\nSets the pitch bend to be applied to the voices in the instrument, as a fraction of the full range."]
	pub setPitchBend:
		::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument, bend: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->instrument->setPitchBendRange(PDSynthInstrument* instrument, float halfSteps)`\n\nSets the pitch bend range for the voices in the instrument. The default range is 12, for a full octave."]
	pub setPitchBendRange:
		::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument, halfSteps: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->instrument->setTranspose(PDSynthInstrument* instrument, float halfSteps)`\n\nSets the transpose parameter for all voices in the instrument."]
	pub setTranspose:
		::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument, halfSteps: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->instrument->noteOff(PDSynthInstrument* instrument, MIDINote note, uint32_t when)`\n\nForwards the noteOff() event to the synth currently playing the given note. See also [playdate→sound→synth→noteOff()](#f-sound.synth.noteOff)."]
	pub noteOff:
		::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument, note: MIDINote, when: u32)>,
	#[doc = "`void playdate->sound->instrument->allNotesOff(PDSynthInstrument* instrument, uint32_t when)`\n\nSends a noteOff event to all voices in the instrument."]
	pub allNotesOff: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument, when: u32)>,
	#[doc = "`void playdate->sound->instrument->setVolume(PDSynthInstrument* instrument, float lvol, float rvol)`"]
	pub setVolume: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument,
	                                                           left: core::ffi::c_float,
	                                                           right: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->instrument->getVolume(PDSynthInstrument* instrument, float* outlvol, float* outrvol)`\n\nSets and gets the playback volume (0.0 - 1.0) for left and right channels of the synth. This is equivalent to\n\n```cpp\nplaydate->sound->source->setVolume((SoundSource*)instrument, lvol, rvol);\nplaydate->sound->source->getVolume((SoundSource*)instrument, &lvol, &rvol);\n```"]
	pub getVolume: ::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument,
	                                                           left: *mut core::ffi::c_float,
	                                                           right: *mut core::ffi::c_float)>,
	#[doc = "`int playdate->sound->instrument->activeVoiceCount(PDSynthInstrument* instrument)`\n\nReturns the number of voices in the instrument currently playing."]
	pub activeVoiceCount:
		::core::option::Option<unsafe extern "C" fn(inst: *mut PDSynthInstrument) -> core::ffi::c_int>,
}
#[test]
fn bindgen_test_layout_playdate_sound_instrument() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_instrument> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_instrument>(),
	           104usize,
	           concat!("Size of: ", stringify!(playdate_sound_instrument))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_instrument>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_instrument))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newInstrument) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(newInstrument)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeInstrument) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(freeInstrument)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addVoice) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(addVoice)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).playNote) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(playNote)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).playMIDINote) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(playMIDINote)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPitchBend) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(setPitchBend)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPitchBendRange) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(setPitchBendRange)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTranspose) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(setTranspose)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).noteOff) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(noteOff)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).allNotesOff) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(allNotesOff)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolume) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(setVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolume) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(getVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).activeVoiceCount) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_instrument),
		"::",
		stringify!(activeVoiceCount)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SequenceTrack {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_track {
	#[doc = "`SequenceTrack* playdate->sound->track->newTrack(void)`\n\nReturns a new SequenceTrack."]
	pub newTrack: ::core::option::Option<unsafe extern "C" fn() -> *mut SequenceTrack>,
	#[doc = "`void playdate->sound->track->freeTrack(SequenceTrack* track)`\n\nFrees the SequenceTrack."]
	pub freeTrack: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack)>,
	#[doc = "`void playdate->sound->track->setInstrument(SequenceTrack* track, PDSynthInstrument* instrument)`"]
	pub setInstrument:
		::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack, inst: *mut PDSynthInstrument)>,
	#[doc = "`PDSynthInstrument* playdate->sound->track->getInstrument(SequenceTrack* track)`\n\nSets or gets the [PDSynthInstrument](#C-sound.PDSynthInstrument) assigned to the track."]
	pub getInstrument:
		::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack) -> *mut PDSynthInstrument>,
	#[doc = "`void playdate->sound->track->addNoteEvent(SequenceTrack* track, uint32_t step, uint32_t length, MIDINote note, float vel)`\n\nAdds a single note event to the track."]
	pub addNoteEvent: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack,
	                                                              step: u32,
	                                                              len: u32,
	                                                              note: MIDINote,
	                                                              velocity: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->track->removeNoteEvent(SequenceTrack* track, uint32_t step, MIDINote note)`\n\nRemoves the event at *step* playing *note*."]
	pub removeNoteEvent:
		::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack, step: u32, note: MIDINote)>,
	#[doc = "`void playdate->sound->track->clearNotes(SequenceTrack* track)`\n\nClears all notes from the track."]
	pub clearNotes: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack)>,
	#[doc = "`void playdate->sound->track->getControlSignalCount(SequenceTrack* track)`\n\nReturns the number of [ControlSignal](#C-sound.ControlSignal) objects in the track."]
	pub getControlSignalCount:
		::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->track->getControlSignal(SequenceTrack* track, int idx)`\n\nReturns the [ControlSignal](#C-sound.ControlSignal) at index *idx*."]
	pub getControlSignal: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack,
	                                                                  idx: core::ffi::c_int)
	                                                                  -> *mut ControlSignal>,
	#[doc = "`void playdate->sound->track->clearControlEvents(SequenceTrack* track)`\n\nClears all [ControlSignals](#C-sound.ControlSignal) from the track."]
	pub clearControlEvents: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack)>,
	#[doc = "`int playdate->sound->track->getPolyphony(SequenceTrack* track)`\n\nReturns the maximum number of simultaneously playing notes in the track. (Currently, this value is only set when the track was loaded from a MIDI file. We don’t yet track polyphony for user-created events.)"]
	pub getPolyphony: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->track->activeVoiceCount(SequenceTrack* track)`\n\nReturns the number of voices currently playing in the track’s instrument."]
	pub activeVoiceCount:
		::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->track->setMuted(SequenceTrack* track, int mute)`\n\nMutes or unmutes the track."]
	pub setMuted: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack, mute: core::ffi::c_int)>,
	#[doc = "`int playdate->sound->track->getLength(SequenceTrack* track)`\n\nReturns the length, in steps, of the track—\u{200b}that is, the step where the last note in the track ends."]
	pub getLength: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack) -> u32>,
	#[doc = "`int playdate->sound->track->getIndexForStep(SequenceTrack* track, uint32_t step)`\n\nReturns the internal array index for the first note at the given step."]
	pub getIndexForStep:
		::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack, step: u32) -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->track->getNoteAtIndex(SequenceTrack* track, int index, uint32_t* outStep, uint32_t* outLen, MIDINote* outNote, float* outVelocity)`\n\nIf the given index is in range, sets the data in the out pointers and returns 1; otherwise, returns 0."]
	pub getNoteAtIndex: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack,
	                                                                index: core::ffi::c_int,
	                                                                outStep: *mut u32,
	                                                                outLen: *mut u32,
	                                                                outNote: *mut MIDINote,
	                                                                outVelocity: *mut core::ffi::c_float)
	                                                                -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->track->getSignalForController(SequenceTrack* track, int controller, int create)`\n\nReturns the [ControlSignal](#C-sound.ControlSignal) for MIDI controller number *controller*, creating it if the **create** flag is set and it doesn’t yet exist."]
	pub getSignalForController: ::core::option::Option<unsafe extern "C" fn(track: *mut SequenceTrack,
	                                                                        controller: core::ffi::c_int,
	                                                                        create: core::ffi::c_int)
	                                                                        -> *mut ControlSignal>,
}
#[test]
fn bindgen_test_layout_playdate_sound_track() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_track> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_track>(),
	           136usize,
	           concat!("Size of: ", stringify!(playdate_sound_track))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_track>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_track))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newTrack) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(newTrack)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeTrack) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(freeTrack)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setInstrument) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(setInstrument)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getInstrument) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getInstrument)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addNoteEvent) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(addNoteEvent)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeNoteEvent) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(removeNoteEvent)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearNotes) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(clearNotes)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getControlSignalCount) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getControlSignalCount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getControlSignal) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getControlSignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).clearControlEvents) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(clearControlEvents)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getPolyphony) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getPolyphony)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).activeVoiceCount) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(activeVoiceCount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMuted) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(setMuted)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLength) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getIndexForStep) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getIndexForStep)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getNoteAtIndex) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getNoteAtIndex)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getSignalForController) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_track),
		"::",
		stringify!(getSignalForController)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundSequence {
	_unused: [u8; 0],
}
pub type SequenceFinishedCallback =
	::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_sequence {
	#[doc = "`SoundSequence* playdate->sound->sequence->newSequence(void)`"]
	pub newSequence: ::core::option::Option<unsafe extern "C" fn() -> *mut SoundSequence>,
	#[doc = "`void playdate->sound->sequence->freeSequence(SoundSequence* sequence)`\n\nCreates or destroys a SoundSequence object."]
	pub freeSequence: ::core::option::Option<unsafe extern "C" fn(sequence: *mut SoundSequence)>,
	#[doc = "`int playdate->sound->sequence->loadMIDIFile(SoundSequence* sequence, const char* path)`\n\nIf the sequence is empty, attempts to load data from the MIDI file at *path* into the sequence. Returns 1 on success, 0 on failure."]
	pub loadMIDIFile: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                              path: *const core::ffi::c_char)
	                                                              -> core::ffi::c_int>,
	#[doc = "`uint32_t playdate->sound->sequence->getTime(SoundSequence* sequence)`"]
	pub getTime: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> u32>,
	#[doc = "`void playdate->sound->sequence->setTime(SoundSequence* sequence, uint32_t time)`\n\nGets or sets the current time in the sequence, in samples since the start of the file. Note that which step this moves the sequence to depends on the current tempo."]
	pub setTime: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence, time: u32)>,
	#[doc = "`void playdate->sound->sequence->setLoops(SoundSequence* sequence, int startstep, int endstep, int loops)`\n\nSets the looping range of the sequence. If *loops* is 0, the loop repeats endlessly."]
	pub setLoops: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                          loopstart: core::ffi::c_int,
	                                                          loopend: core::ffi::c_int,
	                                                          loops: core::ffi::c_int)>,
	pub getTempo_deprecated:
		::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->sequence->setTempo(SoundSequence* sequence, float stepsPerSecond)`\n\nSets or gets the tempo of the sequence, in steps per second."]
	pub setTempo:
		::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence, stepsPerSecond: core::ffi::c_float)>,
	#[doc = "`int playdate->sound->sequence->getTrackCount(SoundSequence* sequence)`\n\nReturns the number of tracks in the sequence."]
	pub getTrackCount: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int>,
	#[doc = "`SequenceTrack* playdate->sound->sequence->addTrack(SoundSequence* sequence)`\n\nAdds the given [playdate.sound.track](#C-sound.track) to the sequence."]
	pub addTrack: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> *mut SequenceTrack>,
	#[doc = "`SequenceTrack* playdate->sound->sequence->getTrackAtIndex(SoundSequence* sequence, unsigned int idx)`"]
	pub getTrackAtIndex: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                                 track: core::ffi::c_uint)
	                                                                 -> *mut SequenceTrack>,
	#[doc = "`void playdate->sound->sequence->setTrackAtIndex(SoundSequence* sequence, SequenceTrack* track, unsigned int idx)`\n\nSets or gets the given [SoundTrack](#C-sound.track) object at position *idx* in the sequence."]
	pub setTrackAtIndex: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                                 track: *mut SequenceTrack,
	                                                                 idx: core::ffi::c_uint)>,
	#[doc = "`void playdate->sound->sequence->allNotesOff(SoundSequence* sequence)`\n\nSends a stop signal to all playing notes on all tracks."]
	pub allNotesOff: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence)>,
	#[doc = "`int playdate->sound->sequence->isPlaying(SoundSequence* sequence)`\n\nReturns 1 if the sequence is currently playing, otherwise 0."]
	pub isPlaying: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->sequence->getLength(SoundSequence* sequence)`\n\nReturns the length of the longest track in the sequence, in steps. See also [playdate.sound.track.getLength()](#m-sound.track:getLength)."]
	pub getLength: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> u32>,
	#[doc = "`void playdate->sound->sequence->play(SoundSequence* sequence, SequenceFinishedCallback finishCallback, void* userdata)`"]
	pub play: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                      finishCallback: SequenceFinishedCallback,
	                                                      userdata: *mut core::ffi::c_void)>,
	#[doc = "`void playdate->sound->sequence->stop(SoundSequence* sequence)`\n\nStarts or stops playing the sequence. `finishCallback` is an optional function to be called when the sequence finishes playing or is stopped.\n\nSequenceFinishedCallback\n\n```cpp\ntypedef void (*SequenceFinishedCallback)(SoundSequence* seq, void* userdata);\n```"]
	pub stop: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence)>,
	#[doc = "`int playdate->sound->sequence->getCurrentStep(SoundSequence* sequence, int* timeOffset)`\n\nReturns the step number the sequence is currently at. If *timeOffset* is not NULL, its contents are set to the current sample offset within the step."]
	pub getCurrentStep: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                                timeOffset: *mut core::ffi::c_int)
	                                                                -> core::ffi::c_int>,
	#[doc = "`void playdate->sound->sequence->setCurrentStep(SoundSequence* seq, int step, int timeOffset, int playNotes)`\n\nSet the current step for the sequence. *timeOffset* is a sample offset within the step. If *playNotes* is set, notes at the given step (ignoring *timeOffset*) are played."]
	pub setCurrentStep: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence,
	                                                                step: core::ffi::c_int,
	                                                                timeOffset: core::ffi::c_int,
	                                                                playNotes: core::ffi::c_int)>,
	#[doc = "`float playdate->sound->sequence->getTempo(SoundSequence* sequence)`"]
	pub getTempo: ::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_float>,
}
#[test]
fn bindgen_test_layout_playdate_sound_sequence() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_sequence> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_sequence>(),
	           160usize,
	           concat!("Size of: ", stringify!(playdate_sound_sequence))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_sequence>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_sequence))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newSequence) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(newSequence)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeSequence) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(freeSequence)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).loadMIDIFile) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(loadMIDIFile)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTime) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTime) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(setTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLoops) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(setLoops)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTempo_deprecated) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getTempo_deprecated)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTempo) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(setTempo)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTrackCount) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getTrackCount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addTrack) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(addTrack)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTrackAtIndex) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getTrackAtIndex)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTrackAtIndex) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(setTrackAtIndex)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).allNotesOff) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(allNotesOff)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).isPlaying) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(isPlaying)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLength) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).play) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(play)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(stop)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCurrentStep) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getCurrentStep)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setCurrentStep) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(setCurrentStep)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTempo) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_sequence),
		"::",
		stringify!(getTempo)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct TwoPoleFilter {
	_unused: [u8; 0],
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum TwoPoleFilterType {
	kFilterTypeLowPass = 0,
	kFilterTypeHighPass = 1,
	kFilterTypeBandPass = 2,
	kFilterTypeNotch = 3,
	kFilterTypePEQ = 4,
	kFilterTypeLowShelf = 5,
	kFilterTypeHighShelf = 6,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect_twopolefilter {
	#[doc = "`TwoPoleFilter* playdate->sound->effect->twopolefilter->newFilter(void)`\n\nCreates a new two pole filter effect."]
	pub newFilter: ::core::option::Option<unsafe extern "C" fn() -> *mut TwoPoleFilter>,
	#[doc = "`void playdate->sound->effect->twopolefilter->freeFilter(TwoPoleFilter* filter)`\n\nFrees the given filter."]
	pub freeFilter: ::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter)>,
	#[doc = "`void playdate->sound->effect->twopolefilter->setType(TwoPoleFilter* filter, TwoPoleFilterType type)`\n\nTwoPoleFilterType\n\n```cpp\ntypedef enum\n{\n\tkFilterTypeLowPass,\n\tkFilterTypeHighPass,\n\tkFilterTypeBandPass,\n\tkFilterTypeNotch,\n\tkFilterTypePEQ,\n\tkFilterTypeLowShelf,\n\tkFilterTypeHighShelf\n} TwoPoleFilterType;\n```\n\nSets the type of the filter."]
	pub setType:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter, type_: TwoPoleFilterType)>,
	#[doc = "`void playdate->sound->effect->twopolefilter->setFrequency(TwoPoleFilter* filter, float frequency)`\n\nSets the center/corner frequency of the filter. Value is in Hz."]
	pub setFrequency:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter, frequency: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->twopolefilter->setFrequencyModulator(TwoPoleFilter* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the effect’s frequency. The signal is scaled so that a value of 1.0 corresponds to half the sample rate. Set to *NULL* to clear the modulator."]
	pub setFrequencyModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->twopolefilter->getFrequencyModulator(TwoPoleFilter* filter)`\n\nReturns the filter’s current frequency modulator."]
	pub getFrequencyModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->effect->twopolefilter->setGain(TwoPoleFilter* filter, float gain)`\n\nSets the filter gain."]
	pub setGain:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter, gain: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->twopolefilter->setResonance(TwoPoleFilter* filter, float resonance)`\n\nSets the filter resonance."]
	pub setResonance:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter, resonance: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->twopolefilter->setResonanceModulator(TwoPoleFilter* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the filter resonance. Set to *NULL* to clear the modulator."]
	pub setResonanceModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->twopolefilter->getResonanceModulator(TwoPoleFilter* filter)`\n\nReturns the filter’s current resonance modulator."]
	pub getResonanceModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut PDSynthSignalValue>,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect_twopolefilter() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect_twopolefilter> =
		::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect_twopolefilter>(),
	           80usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect_twopolefilter))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect_twopolefilter>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect_twopolefilter))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newFilter) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(newFilter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeFilter) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(freeFilter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setType) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(setType)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFrequency) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(setFrequency)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFrequencyModulator) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(setFrequencyModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFrequencyModulator) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(getFrequencyModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setGain) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(setGain)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setResonance) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(setResonance)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setResonanceModulator) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(setResonanceModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getResonanceModulator) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_twopolefilter),
		"::",
		stringify!(getResonanceModulator)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct OnePoleFilter {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect_onepolefilter {
	#[doc = "`OnePoleFilter* playdate->sound->effect->onepolefilter->newFilter(void)`\n\nCreates a new one pole filter."]
	pub newFilter: ::core::option::Option<unsafe extern "C" fn() -> *mut OnePoleFilter>,
	#[doc = "`void playdate->sound->effect->onepolefilter->freeFilter(OnePoleFilter* filter)`\n\nFrees the filter."]
	pub freeFilter: ::core::option::Option<unsafe extern "C" fn(filter: *mut OnePoleFilter)>,
	#[doc = "`void playdate->sound->effect->onepolefilter->setParameter(OnePoleFilter* filter, float parameter)`\n\nSets the filter’s single parameter (cutoff frequency) to *p*. Values above 0 (up to 1) are high-pass, values below 0 (down to -1) are low-pass."]
	pub setParameter:
		::core::option::Option<unsafe extern "C" fn(filter: *mut OnePoleFilter, parameter: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->onepolefilter->setParameterModulator(OnePoleFilter* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the filter parameter. Set to *NULL* to clear the modulator."]
	pub setParameterModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut OnePoleFilter, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->onepolefilter->getParameterModulator(OnePoleFilter* filter)`\n\nReturns the filter’s current parameter modulator."]
	pub getParameterModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut OnePoleFilter) -> *mut PDSynthSignalValue>,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect_onepolefilter() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect_onepolefilter> =
		::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect_onepolefilter>(),
	           40usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect_onepolefilter))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect_onepolefilter>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect_onepolefilter))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newFilter) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_onepolefilter),
		"::",
		stringify!(newFilter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeFilter) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_onepolefilter),
		"::",
		stringify!(freeFilter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setParameter) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_onepolefilter),
		"::",
		stringify!(setParameter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setParameterModulator) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_onepolefilter),
		"::",
		stringify!(setParameterModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getParameterModulator) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_onepolefilter),
		"::",
		stringify!(getParameterModulator)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct BitCrusher {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect_bitcrusher {
	#[doc = "`BitCrusher* playdate->sound->effect->bitcrusher->newBitCrusher(void)`\n\nReturns a new BitCrusher effect."]
	pub newBitCrusher: ::core::option::Option<unsafe extern "C" fn() -> *mut BitCrusher>,
	#[doc = "`void playdate->sound->effect->bitcrusher->freeBitCrusher(BitCrusher* filter)`\n\nFrees the given effect."]
	pub freeBitCrusher: ::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher)>,
	#[doc = "`void playdate->sound->effect->bitcrusher->setAmount(BitCrusher* filter, float amount)`\n\nSets the amount of crushing to *amount*. Valid values are 0 (no effect) to 1 (quantizing output to 1-bit)."]
	pub setAmount:
		::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher, amount: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->bitcrusher->setAmountModulator(BitCrusher* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the crushing amount. Set to *NULL* to clear the modulator."]
	pub setAmountModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->bitcrusher->getAmountModulator(BitCrusher* filter)`\n\nReturns the currently set modulator."]
	pub getAmountModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->effect->bitcrusher->setUndersampling(BitCrusher* filter, float undersample)`\n\nSets the number of samples to repeat, quantizing the input in time. A value of 0 produces no undersampling, 1 repeats every other sample, etc."]
	pub setUndersampling:
		::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher, undersampling: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->bitcrusher->setUndersampleModulator(BitCrusher* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the undersampling amount. Set to *NULL* to clear the modulator."]
	pub setUndersampleModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->bitcrusher->getUndersampleModulator(BitCrusher* filter)`\n\nReturns the currently set modulator."]
	pub getUndersampleModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut PDSynthSignalValue>,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect_bitcrusher() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect_bitcrusher> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect_bitcrusher>(),
	           64usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect_bitcrusher))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect_bitcrusher>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect_bitcrusher))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newBitCrusher) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(newBitCrusher)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeBitCrusher) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(freeBitCrusher)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAmount) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(setAmount)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setAmountModulator) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(setAmountModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getAmountModulator) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(getAmountModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUndersampling) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(setUndersampling)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUndersampleModulator) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(setUndersampleModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getUndersampleModulator) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_bitcrusher),
		"::",
		stringify!(getUndersampleModulator)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct RingModulator {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect_ringmodulator {
	#[doc = "`RingModulator* playdate->sound->effect->ringmodulator->newRingmod(void)`\n\nReturns a new ring modulator effect."]
	pub newRingmod: ::core::option::Option<unsafe extern "C" fn() -> *mut RingModulator>,
	pub freeRingmod: ::core::option::Option<unsafe extern "C" fn(filter: *mut RingModulator)>,
	#[doc = "`void playdate->sound->effect->ringmodulator->setFrequency(RingModulator* filter, float frequency)`\n\nSets the frequency of the modulation signal."]
	pub setFrequency:
		::core::option::Option<unsafe extern "C" fn(filter: *mut RingModulator, frequency: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->ringmodulator->setFrequencyModulator(RingModulator* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the frequency of the ring modulator. Set to *NULL* to clear the modulator."]
	pub setFrequencyModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut RingModulator, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->ringmodulator->getFrequencyModulator(RingModulator* filter)`\n\nReturns the currently set frequency modulator."]
	pub getFrequencyModulator:
		::core::option::Option<unsafe extern "C" fn(filter: *mut RingModulator) -> *mut PDSynthSignalValue>,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect_ringmodulator() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect_ringmodulator> =
		::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect_ringmodulator>(),
	           40usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect_ringmodulator))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect_ringmodulator>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect_ringmodulator))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newRingmod) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_ringmodulator),
		"::",
		stringify!(newRingmod)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeRingmod) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_ringmodulator),
		"::",
		stringify!(freeRingmod)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFrequency) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_ringmodulator),
		"::",
		stringify!(setFrequency)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFrequencyModulator) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_ringmodulator),
		"::",
		stringify!(setFrequencyModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getFrequencyModulator) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_ringmodulator),
		"::",
		stringify!(getFrequencyModulator)
	)
	);
}
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect_delayline {
	#[doc = "`DelayLine* playdate->sound->effect->delayline->newDelayLine(int length, int stereo)`\n\nCreates a new delay line effect. The *length* parameter is given in samples."]
	pub newDelayLine: ::core::option::Option<unsafe extern "C" fn(length: core::ffi::c_int,
	                                                              stereo: core::ffi::c_int)
	                                                              -> *mut DelayLine>,
	#[doc = "`void playdate->sound->effect->delayline->freeDelayLine(DelayLine* delay)`\n\nFrees the delay line."]
	pub freeDelayLine: ::core::option::Option<unsafe extern "C" fn(filter: *mut DelayLine)>,
	#[doc = "`void playdate->sound->effect->delayline->setLength(DelayLine* d, int frames)`\n\nChanges the length of the delay line, clearing its contents. This function reallocates the audio buffer, so it is not safe to call while the delay line is in use."]
	pub setLength: ::core::option::Option<unsafe extern "C" fn(d: *mut DelayLine, frames: core::ffi::c_int)>,
	#[doc = "`void playdate->sound->effect->delayline->setFeedback(DelayLine* d, float fb)`\n\nSets the feedback level of the delay line."]
	pub setFeedback: ::core::option::Option<unsafe extern "C" fn(d: *mut DelayLine, fb: core::ffi::c_float)>,
	#[doc = "`DelayLineTap* playdate->sound->effect->delayline->addTap(DelayLine* d, int delay)`\n\nReturns a new tap on the delay line, at the given position. *delay* must be less than or equal to the length of the delay line."]
	pub addTap: ::core::option::Option<unsafe extern "C" fn(d: *mut DelayLine,
	                                                        delay: core::ffi::c_int)
	                                                        -> *mut DelayLineTap>,
	#[doc = "`void playdate->sound->effect->delayline->freeTap(DelayLineTap* tap)`\n\nFrees a tap previously created with [playdate→sound→delayline→addTap()](#f-sound.effect.delayline.addTap), first removing it from the sound engine if needed."]
	pub freeTap: ::core::option::Option<unsafe extern "C" fn(tap: *mut DelayLineTap)>,
	#[doc = "`void playdate->sound->effect->delayline->setTapDelay(DelayLineTap* tap, int frames)`\n\nSets the position of the tap on the delay line, up to the delay line’s length."]
	pub setTapDelay: ::core::option::Option<unsafe extern "C" fn(t: *mut DelayLineTap, frames: core::ffi::c_int)>,
	#[doc = "`void playdate->sound->effect->delayline->setTapDelayModulator(DelayLineTap* tap, PDSynthSignalValue* mod)`\n\nSets a [signal](#C-sound.signal) to modulate the tap delay. If the signal is continuous (e.g. an envelope or a triangle LFO, but not a square LFO) playback is sped up or slowed down to compress or expand time. Set to *NULL* to clear the modulator."]
	pub setTapDelayModulator:
		::core::option::Option<unsafe extern "C" fn(t: *mut DelayLineTap, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->delayline->getTapDelayModulator(DelayLineTap* tap)`\n\nReturns the current delay modulator."]
	pub getTapDelayModulator:
		::core::option::Option<unsafe extern "C" fn(t: *mut DelayLineTap) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->effect->delayline->setTapChannelsFlipped(DelayLineTap* tap, int flip)`\n\nIf the delay line is stereo and *flip* is set, the tap outputs the delay line’s left channel to its right output and vice versa."]
	pub setTapChannelsFlipped:
		::core::option::Option<unsafe extern "C" fn(t: *mut DelayLineTap, flip: core::ffi::c_int)>,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect_delayline() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect_delayline> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect_delayline>(),
	           80usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect_delayline))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect_delayline>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect_delayline))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newDelayLine) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(newDelayLine)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeDelayLine) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(freeDelayLine)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLength) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(setLength)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFeedback) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(setFeedback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addTap) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(addTap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeTap) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(freeTap)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTapDelay) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(setTapDelay)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTapDelayModulator) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(setTapDelayModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getTapDelayModulator) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(getTapDelayModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setTapChannelsFlipped) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_delayline),
		"::",
		stringify!(setTapChannelsFlipped)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Overdrive {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect_overdrive {
	#[doc = "`Overdrive* playdate->sound->effect->overdrive->newOverdrive(void)`\n\nReturns a new overdrive effect."]
	pub newOverdrive: ::core::option::Option<unsafe extern "C" fn() -> *mut Overdrive>,
	#[doc = "`void playdate->sound->effect->overdrive->freeOverdrive(Overdrive* filter)`\n\nFrees the given effect."]
	pub freeOverdrive: ::core::option::Option<unsafe extern "C" fn(filter: *mut Overdrive)>,
	#[doc = "`void playdate->sound->effect->overdrive->setGain(Overdrive* filter, float gain)`\n\nSets the gain of the overdrive effect."]
	pub setGain: ::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive, gain: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->overdrive->setLimit(Overdrive* filter, float limit)`\n\nSets the level where the amplified input clips."]
	pub setLimit: ::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive, limit: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->overdrive->setLimitModulator(Overdrive* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the limit parameter. Set to *NULL* to clear the modulator."]
	pub setLimitModulator:
		::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->overdrive->getLimitModulator(RingMoOverdrivedulator* filter)`\n\nReturns the currently set limit modulator."]
	pub getLimitModulator:
		::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->effect->overdrive->setOffset(Overdrive* filter, float offset)`\n\nAdds an offset to the upper and lower limits to create an asymmetric clipping."]
	pub setOffset: ::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive, offset: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->overdrive->setOffsetModulator(Overdrive* filter, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the offset parameter. Set to *NULL* to clear the modulator."]
	pub setOffsetModulator:
		::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->overdrive->getOffsetModulator(RingMoOverdrivedulator* filter)`\n\nReturns the currently set offset modulator."]
	pub getOffsetModulator:
		::core::option::Option<unsafe extern "C" fn(o: *mut Overdrive) -> *mut PDSynthSignalValue>,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect_overdrive() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect_overdrive> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect_overdrive>(),
	           72usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect_overdrive))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect_overdrive>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect_overdrive))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newOverdrive) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(newOverdrive)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeOverdrive) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(freeOverdrive)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setGain) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(setGain)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLimit) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(setLimit)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setLimitModulator) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(setLimitModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getLimitModulator) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(getLimitModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOffset) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(setOffset)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOffsetModulator) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(setOffsetModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getOffsetModulator) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect_overdrive),
		"::",
		stringify!(getOffsetModulator)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SoundEffect {
	_unused: [u8; 0],
}
pub type effectProc = ::core::option::Option<unsafe extern "C" fn(e: *mut SoundEffect,
                                                                  left: *mut i32,
                                                                  right: *mut i32,
                                                                  nsamples: core::ffi::c_int,
                                                                  bufactive: core::ffi::c_int)
                                                                  -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_effect {
	#[doc = "`SoundEffect* playdate->sound->effect->newEffect(effectProc* proc, void* userdata)`\n\neffectProc\n\n```cpp\ntypedef int effectProc(SoundEffect* e, int32_t* left, int32_t* right, int nsamples, int bufactive);\n```\n\nCreates a new effect using the given processing function. *bufactive* is 1 if samples have been set in the left or right buffers. The function should return 1 if it changed the buffer samples, otherwise 0. *left* and *right* (if the effect is on a stereo channel) are sample buffers in Q8.24 format."]
	pub newEffect: ::core::option::Option<unsafe extern "C" fn(proc_: effectProc,
	                                                           userdata: *mut core::ffi::c_void)
	                                                           -> *mut SoundEffect>,
	#[doc = "`void playdate->sound->effect->freeEffect(SoundEffect* effect)`\n\nFrees the given effect."]
	pub freeEffect: ::core::option::Option<unsafe extern "C" fn(effect: *mut SoundEffect)>,
	#[doc = "`void playdate->sound->effect->setMix(SoundEffect* effect, float level)`\n\nSets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix (which is useful if you’re using a delay line with taps and don’t want to hear the delay line itself)."]
	pub setMix: ::core::option::Option<unsafe extern "C" fn(effect: *mut SoundEffect, level: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->effect->setMixModulator(SoundEffect* effect, PDSynthSignalValue* signal)`\n\nSets a [signal](#C-sound.signal) to modulate the effect’s mix level. Set to *NULL* to clear the modulator."]
	pub setMixModulator:
		::core::option::Option<unsafe extern "C" fn(effect: *mut SoundEffect, signal: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->effect->getMixModulator(SoundEffect* effect)`\n\nReturns the current mix modulator for the effect."]
	pub getMixModulator:
		::core::option::Option<unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->effect->setUserdata(SoundEffect* effect, void* userdata)`"]
	pub setUserdata:
		::core::option::Option<unsafe extern "C" fn(effect: *mut SoundEffect, userdata: *mut core::ffi::c_void)>,
	#[doc = "`void* playdate->sound->effect->getUserdata(SoundEffect* effect)`\n\nSets or gets a userdata value for the effect."]
	pub getUserdata:
		::core::option::Option<unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut core::ffi::c_void>,
	pub twopolefilter: *const playdate_sound_effect_twopolefilter,
	pub onepolefilter: *const playdate_sound_effect_onepolefilter,
	pub bitcrusher: *const playdate_sound_effect_bitcrusher,
	pub ringmodulator: *const playdate_sound_effect_ringmodulator,
	pub delayline: *const playdate_sound_effect_delayline,
	pub overdrive: *const playdate_sound_effect_overdrive,
}
#[test]
fn bindgen_test_layout_playdate_sound_effect() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_effect> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_effect>(),
	           104usize,
	           concat!("Size of: ", stringify!(playdate_sound_effect))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_effect>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_effect))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newEffect) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(newEffect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeEffect) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(freeEffect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMix) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(setMix)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMixModulator) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(setMixModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getMixModulator) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(getMixModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setUserdata) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(setUserdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getUserdata) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(getUserdata)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).twopolefilter) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(twopolefilter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).onepolefilter) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(onepolefilter)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).bitcrusher) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(bitcrusher)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).ringmodulator) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(ringmodulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).delayline) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(delayline)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).overdrive) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_effect),
		"::",
		stringify!(overdrive)
	)
	);
}
impl Default for playdate_sound_effect {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_channel {
	#[doc = "`SoundChannel* playdate->sound->channel->newChannel(void)`\n\nReturns a new *SoundChannel* object."]
	pub newChannel: ::core::option::Option<unsafe extern "C" fn() -> *mut SoundChannel>,
	#[doc = "`void playdate->sound->channel->freeChannel(SoundChannel* channel)`\n\nFrees the given *SoundChannel*."]
	pub freeChannel: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel)>,
	#[doc = "`void playdate->sound->channel->addSource(SoundChannel* channel, SoundSource* source)`\n\nAdds a [SoundSource](#f-sound.source) to the channel. If a source is not assigned to a channel, it plays on the default global channel."]
	pub addSource: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel,
	                                                           source: *mut SoundSource)
	                                                           -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->channel->removeSource(SoundChannel* channel, SoundSource* source)`\n\nRemoves a [SoundSource](#f-sound.source) to the channel. Returns 1 if the source was found in (and removed from) the channel, otherwise 0."]
	pub removeSource: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel,
	                                                              source: *mut SoundSource)
	                                                              -> core::ffi::c_int>,
	#[doc = "`SoundSource* playdate->sound->channel->addCallbackSource(SoundChannel* channel, AudioSourceFunction* callback, void* context, int stereo)`\n\nCreates a new [SoundSource](#f-sound.source) using the given data provider callback and adds it to the channel.\n\nAudioSourceFunction\n\n```cpp\nint AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)\n```\n\nThis function should fill the passed-in *left* buffer (and *right* if it’s a stereo source) with *len* samples each and return 1, or return 0 if the source is silent through the cycle. The caller takes ownership of the allocated SoundSource, and should free it with\n\n```cpp\nplaydate->system->realloc(source, 0);\n```\n\nwhen it is not longer in use."]
	pub addCallbackSource: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel,
	                                                                   callback: AudioSourceFunction,
	                                                                   context: *mut core::ffi::c_void,
	                                                                   stereo: core::ffi::c_int)
	                                                                   -> *mut SoundSource>,
	#[doc = "`void playdate->sound->channel->addEffect(SoundChannel* channel, SoundEffect* effect)`\n\nAdds a [SoundEffect](#f-sound.effect) to the channel."]
	pub addEffect:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect)>,
	#[doc = "`void playdate->sound->channel->removeEffect(SoundChannel* channel, SoundEffect* effect)`\n\nRemoves a [SoundEffect](#f-sound.effect) from the channel."]
	pub removeEffect:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect)>,
	#[doc = "`void playdate->sound->channel->setVolume(SoundChannel* channel, float volume)`\n\nSets the volume for the channel, in the range [0-1]."]
	pub setVolume:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel, volume: core::ffi::c_float)>,
	#[doc = "`float playdate->sound->channel->getVolume(SoundChannel* channel)`\n\nGets the volume for the channel, in the range [0-1]."]
	pub getVolume: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel) -> core::ffi::c_float>,
	#[doc = "`void playdate->sound->channel->setVolumeModulator(SoundChannel* channel, PDSynthSignalValue* mod)`\n\nSets a [signal](#C-sound.signal) to modulate the channel volume. Set to *NULL* to clear the modulator."]
	pub setVolumeModulator:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->channel->getVolumeModulator(SoundChannel* channel)`\n\nGets a [signal](#C-sound.signal) modulating the channel volume."]
	pub getVolumeModulator:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue>,
	#[doc = "`void playdate->sound->channel->setPan(SoundChannel* channel, float pan)`\n\nSets the pan parameter for the channel. Valid values are in the range [-1,1], where -1 is left, 0 is center, and 1 is right."]
	pub setPan: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel, pan: core::ffi::c_float)>,
	#[doc = "`void playdate->sound->channel->setPanModulator(SoundChannel* channel, PDSynthSignalValue* mod)`\n\nSets a [signal](#C-sound.signal) to modulate the channel pan. Set to *NULL* to clear the modulator."]
	pub setPanModulator:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut PDSynthSignalValue)>,
	#[doc = "`PDSynthSignalValue* playdate->sound->channel->getPanModulator(SoundChannel* channel)`\n\nGets a [signal](#C-sound.signal) modulating the channel pan."]
	pub getPanModulator:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue>,
	#[doc = "`PDSynthSignalValue* playdate->sound->channel->getDryLevelSignal(SoundChannel* channel)`\n\nReturns a signal that follows the volume of the channel before effects are applied."]
	pub getDryLevelSignal:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue>,
	#[doc = "`PDSynthSignalValue* playdate->sound->channel->getWetLevelSignal(SoundChannel* channel)`\n\nReturns a signal that follows the volume of the channel after effects are applied."]
	pub getWetLevelSignal:
		::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue>,
}
#[test]
fn bindgen_test_layout_playdate_sound_channel() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound_channel> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound_channel>(),
	           128usize,
	           concat!("Size of: ", stringify!(playdate_sound_channel))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound_channel>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound_channel))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).newChannel) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(newChannel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeChannel) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(freeChannel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addSource) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(addSource)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeSource) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(removeSource)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addCallbackSource) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(addCallbackSource)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addEffect) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(addEffect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeEffect) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(removeEffect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolume) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(setVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolume) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(getVolume)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setVolumeModulator) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(setVolumeModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getVolumeModulator) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(getVolumeModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPan) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(setPan)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setPanModulator) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(setPanModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getPanModulator) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(getPanModulator)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getDryLevelSignal) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(getDryLevelSignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getWetLevelSignal) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound_channel),
		"::",
		stringify!(getWetLevelSignal)
	)
	);
}
pub type RecordCallback = ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void,
                                                                      buffer: *mut i16,
                                                                      length: core::ffi::c_int)
                                                                      -> core::ffi::c_int>;
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum MicSource {
	kMicInputAutodetect = 0,
	kMicInputInternal = 1,
	kMicInputHeadset = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound { pub channel : * const playdate_sound_channel , pub fileplayer : * const playdate_sound_fileplayer , pub sample : * const playdate_sound_sample , pub sampleplayer : * const playdate_sound_sampleplayer , pub synth : * const playdate_sound_synth , pub sequence : * const playdate_sound_sequence , pub effect : * const playdate_sound_effect , pub lfo : * const playdate_sound_lfo , pub envelope : * const playdate_sound_envelope , pub source : * const playdate_sound_source , pub controlsignal : * const playdate_control_signal , pub track : * const playdate_sound_track , pub instrument : * const playdate_sound_instrument , # [doc = "`uint32_t playdate->sound->getCurrentTime(void)`\n\nReturns the sound engine’s current time value, in units of sample frames, 44,100 per second.\n\nEquivalent to [`playdate.sound.getCurrentTime()`](./Inside%20Playdate.html#f-sound.getCurrentTime) in the Lua API."] pub getCurrentTime : :: core :: option :: Option < unsafe extern "C" fn () -> u32 > , # [doc = "`SoundSource* playdate->sound->addSource(AudioSourceFunction* callback, void* context, int stereo)`\n\nThe *callback* function you pass in will be called every audio render cycle.\n\nAudioSourceFunction\n\n```cpp\nint AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)\n```\n\nThis function should fill the passed-in *left* buffer (and *right* if it’s a stereo source) with *len* samples each and return 1, or return 0 if the source is silent through the cycle."] pub addSource : :: core :: option :: Option < unsafe extern "C" fn (callback : AudioSourceFunction , context : * mut core :: ffi :: c_void , stereo : core :: ffi :: c_int) -> * mut SoundSource > , # [doc = "`SoundChannel* playdate->sound->getDefaultChannel(void)`\n\nReturns the default channel, where sound sources play if they haven’t been explicity assigned to a different channel."] pub getDefaultChannel : :: core :: option :: Option < unsafe extern "C" fn () -> * mut SoundChannel > , # [doc = "`int playdate->sound->addChannel(SoundChannel* channel)`\n\nAdds the given channel to the sound engine. Returns 1 if the channel was added, 0 if it was already in the engine."] pub addChannel : :: core :: option :: Option < unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int > , # [doc = "`int playdate->sound->removeChannel(SoundChannel* channel)`\n\nRemoves the given channel from the sound engine. Returns 1 if the channel was successfully removed, 0 if the channel is the default channel or hadn’t been previously added."] pub removeChannel : :: core :: option :: Option < unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int > , # [doc = "`int playdate->sound->setMicCallback(AudioInputFunction* callback, void* context, enum MicSource source)`\n\nThe *callback* you pass in will be called every audio cycle.\n\nAudioInputFunction\n\n```cpp\nint AudioInputFunction(void* context, int16_t* data, int len)\n```\n\nenum MicSource\n\n```cpp\nenum MicSource {\n\tkMicInputAutodetect = 0,\n\tkMicInputInternal = 1,\n\tkMicInputHeadset = 2\n};\n```\n\nYour input callback will be called with the recorded audio data, a monophonic stream of samples. The function should return 1 to continue recording, 0 to stop recording.\n\nThe Playdate hardware has a circuit that attempts to autodetect the presence of a headset mic, but there are cases where you may want to override this. For instance, if you’re using a headphone splitter to wire an external source to the mic input, the detector may not always see the input. Setting the source to `kMicInputHeadset` forces recording from the headset. Using `kMicInputInternal` records from the device mic even when a headset with a mic is plugged in. And `kMicInputAutodetect` uses a headset mic if one is detected, otherwise the device microphone. `setMicCallback()` returns which source the function used, internal or headset, or 0 on error."] pub setMicCallback : :: core :: option :: Option < unsafe extern "C" fn (callback : RecordCallback , context : * mut core :: ffi :: c_void , source : MicSource) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->getHeadphoneState(int* headphone, int* mic, void (*changeCallback)(int headphone, int mic))`\n\nIf *headphone* contains a pointer to an int, the value is set to 1 if headphones are currently plugged in. Likewise, *mic* is set if the headphones include a microphone. If *changeCallback* is provided, it will be called when the headset or mic status changes, and audio output will **not** automatically switch from speaker to headphones when headphones are plugged in (and vice versa). In this case, the callback should use `playdate→sound→setOutputsActive()` to change the output if needed.\n\nEquivalent to [`playdate.sound.getHeadphoneState()`](./Inside%20Playdate.html#f-sound.getHeadphoneState) in the Lua API."] pub getHeadphoneState : :: core :: option :: Option < unsafe extern "C" fn (headphone : * mut core :: ffi :: c_int , headsetmic : * mut core :: ffi :: c_int , changeCallback : :: core :: option :: Option < unsafe extern "C" fn (headphone : core :: ffi :: c_int , mic : core :: ffi :: c_int) >) > , # [doc = "`void playdate->sound->setOutputsActive(int headphone, int speaker)`\n\nForce audio output to the given outputs, regardless of headphone status.\n\nEquivalent to [`playdate.sound.setOutputsActive()`](./Inside%20Playdate.html#f-sound.setOutputsActive) in the Lua API."] pub setOutputsActive : :: core :: option :: Option < unsafe extern "C" fn (headphone : core :: ffi :: c_int , speaker : core :: ffi :: c_int) > , # [doc = "`int playdate->sound->removeSource(SoundSource* source)`\n\nRemoves the given [SoundSource](#C-sound.source) object from its channel, whether it’s in the default channel or a channel created with [playdate→sound→addChannel()](#f-sound.addChannel). Returns 1 if a source was removed, 0 if the source wasn’t in a channel."] pub removeSource : :: core :: option :: Option < unsafe extern "C" fn (source : * mut SoundSource) -> core :: ffi :: c_int > , pub signal : * const playdate_sound_signal , pub getError : :: core :: option :: Option < unsafe extern "C" fn () -> * const core :: ffi :: c_char > , }
#[test]
fn bindgen_test_layout_playdate_sound() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_sound> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_sound>(),
	           192usize,
	           concat!("Size of: ", stringify!(playdate_sound))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_sound>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_sound))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).channel) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(channel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).fileplayer) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(fileplayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sample) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(sample)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sampleplayer) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(sampleplayer)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).synth) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(synth)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(sequence)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).effect) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(effect)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).lfo) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(lfo)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).envelope) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(envelope)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).source) as usize - ptr as usize },
	           72usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(source)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).controlsignal) as usize - ptr as usize },
	           80usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(controlsignal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).track) as usize - ptr as usize },
	           88usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(track)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).instrument) as usize - ptr as usize },
	           96usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(instrument)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getCurrentTime) as usize - ptr as usize },
	           104usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(getCurrentTime)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addSource) as usize - ptr as usize },
	           112usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(addSource)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getDefaultChannel) as usize - ptr as usize },
	           120usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(getDefaultChannel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addChannel) as usize - ptr as usize },
	           128usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(addChannel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeChannel) as usize - ptr as usize },
	           136usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(removeChannel)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMicCallback) as usize - ptr as usize },
	           144usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(setMicCallback)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getHeadphoneState) as usize - ptr as usize },
	           152usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(getHeadphoneState)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOutputsActive) as usize - ptr as usize },
	           160usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(setOutputsActive)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).removeSource) as usize - ptr as usize },
	           168usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(removeSource)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).signal) as usize - ptr as usize },
	           176usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(signal)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getError) as usize - ptr as usize },
	           184usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_sound),
		"::",
		stringify!(getError)
	)
	);
}
impl Default for playdate_sound {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_display {
	#[doc = "`int playdate->display->getWidth(void)`\n\nReturns the width of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 200 instead of 400.\n\nEquivalent to [`playdate.display.getWidth()`](./Inside%20Playdate.html#f-display.getWidth) in the Lua API."]
	pub getWidth: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,
	#[doc = "`int playdate->display->getHeight(void)`\n\nReturns the height of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 120 instead of 240.\n\nEquivalent to [`playdate.display.getHeight()`](./Inside%20Playdate.html#f-display.getHeight) in the Lua API."]
	pub getHeight: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_int>,
	#[doc = "`void playdate->display->setRefreshRate(float rate)`\n\nSets the nominal refresh rate in frames per second. The default is 30 fps, which is a recommended figure that balances animation smoothness with performance and power considerations. Maximum is 50 fps.\n\nIf *rate* is 0, the game’s update callback (either Lua’s `playdate.update()` or the function specified by [playdate→system→setUpdateCallback()](#f-system.setUpdateCallback)) is called as soon as possible. Since the display refreshes line-by-line, and unchanged lines aren’t sent to the display, the update cycle will be faster than 30 times a second but at an indeterminate rate.\n\nEquivalent to [`playdate.display.setRefreshRate()`](./Inside%20Playdate.html#f-display.setRefreshRate) in the Lua API."]
	pub setRefreshRate: ::core::option::Option<unsafe extern "C" fn(rate: core::ffi::c_float)>,
	#[doc = "`void playdate->display->setInverted(int flag)`\n\nIf *flag* evaluates to true, the frame buffer is drawn inverted—black instead of white, and vice versa.\n\nEquivalent to [`playdate.display.setInverted()`](./Inside%20Playdate.html#f-display.setInverted) in the Lua API."]
	pub setInverted: ::core::option::Option<unsafe extern "C" fn(flag: core::ffi::c_int)>,
	#[doc = "`void playdate->display->setScale(unsigned int s)`\n\nSets the display scale factor. Valid values for *scale* are 1, 2, 4, and 8.\n\nThe top-left corner of the frame buffer is scaled up to fill the display; e.g., if the scale is set to 4, the pixels in rectangle [0,100] x [0,60] are drawn on the screen as 4 x 4 squares.\n\nEquivalent to [`playdate.display.setScale()`](./Inside%20Playdate.html#f-display.setScale) in the Lua API."]
	pub setScale: ::core::option::Option<unsafe extern "C" fn(s: core::ffi::c_uint)>,
	#[doc = "`void playdate->display->setMosaic(unsigned int x, unsigned int y)`\n\nAdds a mosaic effect to the display. Valid *x* and *y* values are between 0 and 3, inclusive.\n\nEquivalent to [`playdate.display.setMosaic`](./Inside%20Playdate.html#f-display.setMosaic) in the Lua API."]
	pub setMosaic: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_uint, y: core::ffi::c_uint)>,
	#[doc = "`void playdate->display->setFlipped(int x, int y)`\n\nFlips the display on the x or y axis, or both.\n\nEquivalent to [`playdate.display.setFlipped()`](./Inside%20Playdate.html#f-display.setFlipped) in the Lua API."]
	pub setFlipped: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int)>,
	#[doc = "`void playdate->display->setOffset(int dx, int dy)`\n\nOffsets the display by the given amount. Areas outside of the displayed area are filled with the current [background color](#f-graphics.setBackgroundColor).\n\nEquivalent to [`playdate.display.setOffset()`](./Inside%20Playdate.html#f-display.setOffset) in the Lua API."]
	pub setOffset: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int)>,
}
#[test]
fn bindgen_test_layout_playdate_display() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_display> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_display>(),
	           64usize,
	           concat!("Size of: ", stringify!(playdate_display))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_display>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_display))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getWidth) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(getWidth)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getHeight) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(getHeight)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setRefreshRate) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(setRefreshRate)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setInverted) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(setInverted)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setScale) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(setScale)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setMosaic) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(setMosaic)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setFlipped) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(setFlipped)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).setOffset) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_display),
		"::",
		stringify!(setOffset)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PDScore {
	pub rank: u32,
	pub value: u32,
	pub player: *mut core::ffi::c_char,
}
#[test]
fn bindgen_test_layout_PDScore() {
	const UNINIT: ::core::mem::MaybeUninit<PDScore> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PDScore>(),
	           16usize,
	           concat!("Size of: ", stringify!(PDScore))
	);
	assert_eq!(
	           ::core::mem::align_of::<PDScore>(),
	           8usize,
	           concat!("Alignment of ", stringify!(PDScore))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).rank) as usize - ptr as usize },
	           0usize,
	           concat!("Offset of field: ", stringify!(PDScore), "::", stringify!(rank))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
	           4usize,
	           concat!("Offset of field: ", stringify!(PDScore), "::", stringify!(value))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).player) as usize - ptr as usize },
	           8usize,
	           concat!("Offset of field: ", stringify!(PDScore), "::", stringify!(player))
	);
}
impl Default for PDScore {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PDScoresList {
	pub boardID: *mut core::ffi::c_char,
	pub count: core::ffi::c_uint,
	pub lastUpdated: u32,
	pub playerIncluded: core::ffi::c_int,
	pub limit: core::ffi::c_uint,
	pub scores: *mut PDScore,
}
#[test]
fn bindgen_test_layout_PDScoresList() {
	const UNINIT: ::core::mem::MaybeUninit<PDScoresList> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PDScoresList>(),
	           32usize,
	           concat!("Size of: ", stringify!(PDScoresList))
	);
	assert_eq!(
	           ::core::mem::align_of::<PDScoresList>(),
	           8usize,
	           concat!("Alignment of ", stringify!(PDScoresList))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).boardID) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDScoresList),
		"::",
		stringify!(boardID)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDScoresList),
		"::",
		stringify!(count)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).lastUpdated) as usize - ptr as usize },
	           12usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDScoresList),
		"::",
		stringify!(lastUpdated)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).playerIncluded) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDScoresList),
		"::",
		stringify!(playerIncluded)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).limit) as usize - ptr as usize },
	           20usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDScoresList),
		"::",
		stringify!(limit)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).scores) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDScoresList),
		"::",
		stringify!(scores)
	)
	);
}
impl Default for PDScoresList {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PDBoard {
	pub boardID: *mut core::ffi::c_char,
	pub name: *mut core::ffi::c_char,
}
#[test]
fn bindgen_test_layout_PDBoard() {
	const UNINIT: ::core::mem::MaybeUninit<PDBoard> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PDBoard>(),
	           16usize,
	           concat!("Size of: ", stringify!(PDBoard))
	);
	assert_eq!(
	           ::core::mem::align_of::<PDBoard>(),
	           8usize,
	           concat!("Alignment of ", stringify!(PDBoard))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).boardID) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDBoard),
		"::",
		stringify!(boardID)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
	           8usize,
	           concat!("Offset of field: ", stringify!(PDBoard), "::", stringify!(name))
	);
}
impl Default for PDBoard {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PDBoardsList {
	pub count: core::ffi::c_uint,
	pub lastUpdated: u32,
	pub boards: *mut PDBoard,
}
#[test]
fn bindgen_test_layout_PDBoardsList() {
	const UNINIT: ::core::mem::MaybeUninit<PDBoardsList> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PDBoardsList>(),
	           16usize,
	           concat!("Size of: ", stringify!(PDBoardsList))
	);
	assert_eq!(
	           ::core::mem::align_of::<PDBoardsList>(),
	           8usize,
	           concat!("Alignment of ", stringify!(PDBoardsList))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDBoardsList),
		"::",
		stringify!(count)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).lastUpdated) as usize - ptr as usize },
	           4usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDBoardsList),
		"::",
		stringify!(lastUpdated)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).boards) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(PDBoardsList),
		"::",
		stringify!(boards)
	)
	);
}
impl Default for PDBoardsList {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
pub type AddScoreCallback =
	::core::option::Option<unsafe extern "C" fn(score: *mut PDScore, errorMessage: *const core::ffi::c_char)>;
pub type PersonalBestCallback =
	::core::option::Option<unsafe extern "C" fn(score: *mut PDScore, errorMessage: *const core::ffi::c_char)>;
pub type BoardsListCallback =
	::core::option::Option<unsafe extern "C" fn(boards: *mut PDBoardsList,
	                                            errorMessage: *const core::ffi::c_char)>;
pub type ScoresCallback = ::core::option::Option<unsafe extern "C" fn(scores: *mut PDScoresList,
                                                                      errorMessage: *const core::ffi::c_char)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_scoreboards {
	pub addScore: ::core::option::Option<unsafe extern "C" fn(boardId: *const core::ffi::c_char,
	                                                          value: u32,
	                                                          callback: AddScoreCallback)
	                                                          -> core::ffi::c_int>,
	pub getPersonalBest: ::core::option::Option<unsafe extern "C" fn(boardId: *const core::ffi::c_char,
	                                                                 callback: PersonalBestCallback)
	                                                                 -> core::ffi::c_int>,
	pub freeScore: ::core::option::Option<unsafe extern "C" fn(score: *mut PDScore)>,
	pub getScoreboards:
		::core::option::Option<unsafe extern "C" fn(callback: BoardsListCallback) -> core::ffi::c_int>,
	pub freeBoardsList: ::core::option::Option<unsafe extern "C" fn(boardsList: *mut PDBoardsList)>,
	pub getScores: ::core::option::Option<unsafe extern "C" fn(boardId: *const core::ffi::c_char,
	                                                           callback: ScoresCallback)
	                                                           -> core::ffi::c_int>,
	pub freeScoresList: ::core::option::Option<unsafe extern "C" fn(scoresList: *mut PDScoresList)>,
}
#[test]
fn bindgen_test_layout_playdate_scoreboards() {
	const UNINIT: ::core::mem::MaybeUninit<playdate_scoreboards> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<playdate_scoreboards>(),
	           56usize,
	           concat!("Size of: ", stringify!(playdate_scoreboards))
	);
	assert_eq!(
	           ::core::mem::align_of::<playdate_scoreboards>(),
	           8usize,
	           concat!("Alignment of ", stringify!(playdate_scoreboards))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).addScore) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(addScore)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getPersonalBest) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(getPersonalBest)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeScore) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(freeScore)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getScoreboards) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(getScoreboards)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeBoardsList) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(freeBoardsList)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).getScores) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(getScores)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).freeScoresList) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(playdate_scoreboards),
		"::",
		stringify!(freeScoresList)
	)
	);
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateAPI {
	pub system: *const playdate_sys,
	pub file: *const playdate_file,
	pub graphics: *const playdate_graphics,
	pub sprite: *const playdate_sprite,
	pub display: *const playdate_display,
	pub sound: *const playdate_sound,
	pub lua: *const playdate_lua,
	pub json: *const playdate_json,
	pub scoreboards: *const playdate_scoreboards,
}
#[test]
fn bindgen_test_layout_PlaydateAPI() {
	const UNINIT: ::core::mem::MaybeUninit<PlaydateAPI> = ::core::mem::MaybeUninit::uninit();
	let ptr = UNINIT.as_ptr();
	assert_eq!(
	           ::core::mem::size_of::<PlaydateAPI>(),
	           72usize,
	           concat!("Size of: ", stringify!(PlaydateAPI))
	);
	assert_eq!(
	           ::core::mem::align_of::<PlaydateAPI>(),
	           8usize,
	           concat!("Alignment of ", stringify!(PlaydateAPI))
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).system) as usize - ptr as usize },
	           0usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(system)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).file) as usize - ptr as usize },
	           8usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(file)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).graphics) as usize - ptr as usize },
	           16usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(graphics)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sprite) as usize - ptr as usize },
	           24usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(sprite)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).display) as usize - ptr as usize },
	           32usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(display)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).sound) as usize - ptr as usize },
	           40usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(sound)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).lua) as usize - ptr as usize },
	           48usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(lua)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).json) as usize - ptr as usize },
	           56usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(json)
	)
	);
	assert_eq!(
	           unsafe { ::core::ptr::addr_of!((*ptr).scoreboards) as usize - ptr as usize },
	           64usize,
	           concat!(
		"Offset of field: ",
		stringify!(PlaydateAPI),
		"::",
		stringify!(scoreboards)
	)
	);
}
impl Default for PlaydateAPI {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDSystemEvent {
	kEventInit = 0,
	kEventInitLua = 1,
	kEventLock = 2,
	kEventUnlock = 3,
	kEventPause = 4,
	kEventResume = 5,
	kEventTerminate = 6,
	kEventKeyPressed = 7,
	kEventKeyReleased = 8,
	kEventLowPower = 9,
}
