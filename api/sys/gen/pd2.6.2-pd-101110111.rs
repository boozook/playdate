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
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),);
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
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),);
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
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),);
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
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),);
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
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
	#[inline]
	pub const fn new() -> Self { __BindgenUnionField(::core::marker::PhantomData) }
	#[inline]
	pub unsafe fn as_ref(&self) -> &T { ::core::mem::transmute(self) }
	#[inline]
	pub unsafe fn as_mut(&mut self) -> &mut T { ::core::mem::transmute(self) }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
	#[inline]
	fn default() -> Self { Self::new() }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
	#[inline]
	fn clone(&self) -> Self { *self }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
	fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
		fmt.write_str("__BindgenUnionField")
	}
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
	fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
	fn eq(&self, _other: &__BindgenUnionField<T>) -> bool { true }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const LCD_COLUMNS: u32 = 400;
pub const LCD_ROWS: u32 = 240;
pub const LCD_ROWSIZE: u32 = 52;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const AUDIO_FRAMES_PER_CYCLE: u32 = 512;
pub const NOTE_C4: u32 = 60;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, PartialEq)]
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum BitmapFlip {
	Unflipped = 0,
	FlippedX = 1,
	FlippedY = 2,
	FlippedXy = 3,
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum SolidColor {
	Black = 0,
	White = 1,
	Clear = 2,
	XOR = 3,
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum LineCapStyle {
	Butt = 0,
	Square = 1,
	Round = 2,
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum FontLanguage {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum StringEncoding {
	ASCII = 0,
	UTF8 = 1,
	UTF16 = 2,
}
pub type Pattern = [u8; 16usize];
pub type Color = usize;
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum PolygonFillRule {
	NonZero = 0,
	EvenOdd = 1,
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum TextWrappingMode {
	Clip = 0,
	Character = 1,
	Word = 2,
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum TextAlignment {
	Left = 0,
	Center = 1,
	Right = 2,
}
#[repr(C)]
#[must_use]
pub struct Bitmap {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct BitmapTable {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct Font {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct FontData {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct FontPage {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct FontGlyph {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct VideoPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateVideo {
	/**
	<code class="title">LCDVideoPlayer playdate-&gt;graphics-&gt;video-&gt;loadVideo(const char* path)</code>
	<div class="content">
	<div class="paragraph">
	<p>Opens the <em>pdv</em> file at <em>path</em> and returns a new video player object for rendering its frames.</p>
	</div>
	</div>
	*/
	pub loadVideo: unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut VideoPlayer,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;video-&gt;freePlayer(LCDVideoPlayer* p)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given video player.</p>
	</div>
	</div>
	*/
	pub freePlayer: unsafe extern "C" fn(p: *mut VideoPlayer),
	/**
	<code class="title">int playdate-&gt;graphics-&gt;video-&gt;setContext(LCDVideoPlayer* p, LCDBitmap* context)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the rendering destination for the video player to the given bitmap. If the function fails, it returns 0 and sets an error message that can be read via <a href="#f-graphics.video.getError">getError()</a>.</p>
	</div>
	</div>
	*/
	pub setContext: unsafe extern "C" fn(p: *mut VideoPlayer, context: *mut Bitmap) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;video-&gt;useScreenContext(LCDVideoPlayer* p)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the rendering destination for the video player to the screen.</p>
	</div>
	</div>
	*/
	pub useScreenContext: unsafe extern "C" fn(p: *mut VideoPlayer),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;video-&gt;renderFrame(LCDVideoPlayer* p, int n)</code>
	<div class="content">
	<div class="paragraph">
	<p>Renders frame number <em>n</em> into the current context. In case of error, the function returns 0 and sets an error message that can be read via <a href="#f-graphics.video.getError">getError()</a>.</p>
	</div>
	</div>
	*/
	pub renderFrame: unsafe extern "C" fn(p: *mut VideoPlayer, n: core::ffi::c_int) -> core::ffi::c_int,
	/**
	<code class="title">const char* playdate-&gt;graphics-&gt;video-&gt;getError(LCDVideoPlayer* p)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns text describing the most recent error.</p>
	</div>
	</div>
	*/
	pub getError: unsafe extern "C" fn(p: *mut VideoPlayer) -> *const core::ffi::c_char,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;video-&gt;getInfo(LCDVideoPlayer* p, int* outWidth, int* outHeight, float* outFrameRate, int* outFrameCount, int* outCurrentFrame)</code>
	<div class="content">
	<div class="paragraph">
	<p>Retrieves information about the video, by passing in (possibly NULL) value pointers.</p>
	</div>
	</div>
	*/
	pub getInfo: unsafe extern "C" fn(p: *mut VideoPlayer,
	                                  outWidth: *mut core::ffi::c_int,
	                                  outHeight: *mut core::ffi::c_int,
	                                  outFrameRate: *mut core::ffi::c_float,
	                                  outFrameCount: *mut core::ffi::c_int,
	                                  outCurrentFrame: *mut core::ffi::c_int),
	/**
	<code class="title">LCBitmap* playdate-&gt;graphics-&gt;video-&gt;getContext(LCDVideoPlayer* p)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the rendering destination for the video player. If no rendering context has been setallocates a context bitmap with the same dimensions as the vieo will be allocated.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct PlaydateGraphics {
	pub video: &'static PlaydateVideo,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;clear(LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the entire display, filling it with <em>color</em>.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.clear"><code>playdate.graphics.clear()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub clear: unsafe extern "C" fn(color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setBackgroundColor(LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the background color shown when the display is <a href="#f-display.setOffset">offset</a> or for clearing dirty areas in the sprite system.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setBackgroundColor"><code>playdate.graphics.setBackgroundColor()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setBackgroundColor: unsafe extern "C" fn(color: SolidColor),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setStencil(LCDBitmap* stencil);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the stencil used for drawing. For a tiled stencil, use <em>setStencilImage()</em> instead. To clear the stencil, set it to <em>NULL</em>.</p>
	</div>
	</div>
	*/
	pub setStencil: unsafe extern "C" fn(stencil: *mut Bitmap),
	/**
	<code class="title">LCDBitmapDrawMode playdate-&gt;graphics-&gt;setDrawMode(LCDBitmapDrawMode mode);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the mode used for drawing bitmaps. Note that text drawing uses bitmaps, so this affects how fonts are displayed as well. Returns the previous draw mode, in case you need to restore it after drawing.</p>
	</div>
	<div class="literalblock">
	<div class="title">LCDBitmapDrawMode</div>
	<div class="content">
	<pre>typedef enum
	{
		kDrawModeCopy,
		kDrawModeWhiteTransparent,
		kDrawModeBlackTransparent,
		kDrawModeFillWhite,
		kDrawModeFillBlack,
		kDrawModeXOR,
		kDrawModeNXOR,
		kDrawModeInverted
	} LCDBitmapDrawMode;</pre>
	</div>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setImageDrawMode"><code>playdate.graphics.setImageDrawMode()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setDrawMode: unsafe extern "C" fn(mode: BitmapDrawMode) -> BitmapDrawMode,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setDrawOffset(int dx, int dy);</code>
	<div class="content">
	<div class="paragraph">
	<p>Offsets the origin point for all drawing calls to <em>x</em>, <em>y</em> (can be negative).</p>
	</div>
	<div class="paragraph">
	<p>This is useful, for example, for centering a "camera" on a sprite that is moving around a world larger than the screen.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setDrawOffset"><code>playdate.graphics.setDrawOffset()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setDrawOffset: unsafe extern "C" fn(dx: core::ffi::c_int, dy: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setClipRect(int x, int y, int width, int height);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the current clip rect, using world coordinates—​that is, the given rectangle will be translated by the current drawing offset. The clip rect is cleared at the beginning of each update.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setClipRect"><code>playdate.graphics.setClipRect()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setClipRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                      y: core::ffi::c_int,
	                                      width: core::ffi::c_int,
	                                      height: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;clearClipRect(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the current clip rect.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.clearClipRect"><code>playdate.graphics.clearClipRect()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub clearClipRect: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setLineCapStyle(LCDLineCapStyle endCapStyle);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the end cap style used in the line drawing functions.</p>
	</div>
	<div class="literalblock">
	<div class="title">LCDLineCapStyle</div>
	<div class="content">
	<pre>typedef enum
	{
		kLineCapStyleButt,
		kLineCapStyleSquare,
		kLineCapStyleRound
	} LCDLineCapStyle;</pre>
	</div>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setLineCapStyle"><code>playdate.graphics.setLineCapStyle()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setLineCapStyle: unsafe extern "C" fn(endCapStyle: LineCapStyle),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setFont(LCDFont* font);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the font to use in subsequent <a href="#f-graphics.drawText">drawText</a> calls.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setFont"><code>playdate.graphics.setFont()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setFont: unsafe extern "C" fn(font: *mut Font),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setTextTracking(int tracking);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the tracking to use when drawing text.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#m-graphics.font.setTracking"><code>playdate.graphics.font:setTracking()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setTextTracking: unsafe extern "C" fn(tracking: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;pushContext(LCDBitmap* target);</code>
	<div class="content">
	<div class="paragraph">
	<p>Push a new drawing context for drawing into the given bitmap. If <em>target</em> is <em>NULL</em>, the drawing functions will use the display framebuffer.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.pushContext"><code>playdate.graphics.pushContext()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub pushContext: unsafe extern "C" fn(target: *mut Bitmap),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;popContext(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pops a context off the stack (if any are left), restoring the drawing settings from before the context was pushed.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.popContext"><code>playdate.graphics.popContext()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub popContext: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;drawBitmap(LCDBitmap* bitmap, int x, int y, LCDBitmapFlip flip);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws the <em>bitmap</em> with its upper-left corner at location <em>x</em>, <em>y</em>, using the given flip orientation.</p>
	</div>
	</div>
	*/
	pub drawBitmap:
		unsafe extern "C" fn(bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int, flip: BitmapFlip),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;tileBitmap(LCDBitmap* bitmap, int x, int y, int width, int height, LCDBitmapFlip flip);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws the <em>bitmap</em> with its upper-left corner at location <em>x</em>, <em>y</em> tiled inside a <em>width</em> by <em>height</em> rectangle.</p>
	</div>
	</div>
	*/
	pub tileBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                     x: core::ffi::c_int,
	                                     y: core::ffi::c_int,
	                                     width: core::ffi::c_int,
	                                     height: core::ffi::c_int,
	                                     flip: BitmapFlip),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;drawLine(int x1, int y1, int x2, int y2, int width, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws a line from <em>x1</em>, <em>y1</em> to <em>x2</em>, <em>y2</em> with a stroke width of <em>width</em>.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.drawLine"><code>playdate.graphics.drawLine()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub drawLine: unsafe extern "C" fn(x1: core::ffi::c_int,
	                                   y1: core::ffi::c_int,
	                                   x2: core::ffi::c_int,
	                                   y2: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;fillTriangle(int x1, int y1, int x2, int y2, int x3, int y3, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws a filled triangle with points at <em>x1</em>, <em>y1</em>, <em>x2</em>, <em>y2</em>, and <em>x3</em>, <em>y3</em>.</p>
	</div>
	<div class="literalblock">
	<div class="title">LCDWindingRule</div>
	<div class="content">
	<pre>typedef enum
	{
		kPolygonFillNonZero,
		kPolygonFillEvenOdd
	} LCDPolygonFillRule;</pre>
	</div>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.fillTriangle"><code>playdate.graphics.fillTriangle()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub fillTriangle: unsafe extern "C" fn(x1: core::ffi::c_int,
	                                       y1: core::ffi::c_int,
	                                       x2: core::ffi::c_int,
	                                       y2: core::ffi::c_int,
	                                       x3: core::ffi::c_int,
	                                       y3: core::ffi::c_int,
	                                       color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;drawRect(int x, int y, int width, int height, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws a <em>width</em> by <em>height</em> rect at <em>x</em>, <em>y</em>.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.drawRect"><code>playdate.graphics.drawRect()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub drawRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                   y: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   height: core::ffi::c_int,
	                                   color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;fillRect(int x, int y, int width, int height, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws a filled <em>width</em> by <em>height</em> rect at <em>x</em>, <em>y</em>.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.fillRect"><code>playdate.graphics.fillRect()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub fillRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                   y: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   height: core::ffi::c_int,
	                                   color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;drawEllipse(int x, int y, int width, int height, int lineWidth, float startAngle, float endAngle, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws an ellipse inside the rectangle {x, y, width, height} of width <em>lineWidth</em> (inset from the rectangle bounds). If <em>startAngle</em> != _endAngle, this draws an arc between the given angles. Angles are given in degrees, clockwise from due north.</p>
	</div>
	</div>
	*/
	pub drawEllipse: unsafe extern "C" fn(x: core::ffi::c_int,
	                                      y: core::ffi::c_int,
	                                      width: core::ffi::c_int,
	                                      height: core::ffi::c_int,
	                                      lineWidth: core::ffi::c_int,
	                                      startAngle: core::ffi::c_float,
	                                      endAngle: core::ffi::c_float,
	                                      color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;fillEllipse(int x, int y, int width, int height, float startAngle, float endAngle, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Fills an ellipse inside the rectangle {x, y, width, height}. If <em>startAngle</em> != _endAngle, this draws a wedge/Pacman between the given angles. Angles are given in degrees, clockwise from due north.</p>
	</div>
	</div>
	*/
	pub fillEllipse: unsafe extern "C" fn(x: core::ffi::c_int,
	                                      y: core::ffi::c_int,
	                                      width: core::ffi::c_int,
	                                      height: core::ffi::c_int,
	                                      startAngle: core::ffi::c_float,
	                                      endAngle: core::ffi::c_float,
	                                      color: Color),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;drawScaledBitmap(LCDBitmap* bitmap, int x, int y, float xscale, float yscale);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws the <em>bitmap</em> scaled to <em>xscale</em> and <em>yscale</em> with its upper-left corner at location <em>x</em>, <em>y</em>. Note that <em>flip</em> is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.</p>
	</div>
	</div>
	*/
	pub drawScaledBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                           x: core::ffi::c_int,
	                                           y: core::ffi::c_int,
	                                           xscale: core::ffi::c_float,
	                                           yscale: core::ffi::c_float),
	/**
	<code class="title">int playdate-&gt;graphics-&gt;drawText(const void* text, size_t len, PDStringEncoding encoding, int x, int y);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws the given text using the provided options. If no font has been set with <a href="#f-graphics.setFont">setFont</a>, the default system font Asheville Sans 14 Light is used. Note that <code>len</code> is the length of the <strong>decoded</strong> string—​that is, the number of codepoints in the string, not the number of bytes; however, since the parser stops at the NUL terminator it’s safe to pass <code>strlen(text)</code> in here when you want to draw the entire string.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.drawText"><code>playdate.graphics.drawText()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub drawText: unsafe extern "C" fn(text: *const core::ffi::c_void,
	                                   len: usize,
	                                   encoding: StringEncoding,
	                                   x: core::ffi::c_int,
	                                   y: core::ffi::c_int) -> core::ffi::c_int,
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;newBitmap(int width, int height, LCDColor bgcolor);</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new <em>width</em> by <em>height</em> LCDBitmap filled with <em>bgcolor</em>.</p>
	</div>
	</div>
	*/
	pub newBitmap:
		unsafe extern "C" fn(width: core::ffi::c_int, height: core::ffi::c_int, bgcolor: Color) -> *mut Bitmap,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;freeBitmap(LCDBitmap*);</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given <em>bitmap</em>.</p>
	</div>
	</div>
	*/
	pub freeBitmap: unsafe extern "C" fn(arg1: *mut Bitmap),
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;loadBitmap(const char* path, const char** outerr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new LCDBitmap from the file at <em>path</em>. If there is no file at <em>path</em>, the function returns null.</p>
	</div>
	</div>
	*/
	pub loadBitmap:
		unsafe extern "C" fn(path: *const core::ffi::c_char, outerr: *mut *const core::ffi::c_char) -> *mut Bitmap,
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;copyBitmap(LCDBitmap* bitmap);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new LCDBitmap that is an exact copy of <em>bitmap</em>.</p>
	</div>
	</div>
	*/
	pub copyBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap) -> *mut Bitmap,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;loadIntoBitmap(const char* path, LCDBitmap* bitmap, const char** outerr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Loads the image at <em>path</em> into the previously allocated <em>bitmap</em>.</p>
	</div>
	</div>
	*/
	pub loadIntoBitmap: unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                         bitmap: *mut Bitmap,
	                                         outerr: *mut *const core::ffi::c_char),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;getBitmapData(LCDBitmap* bitmap, int* width, int* height, int* rowbytes, uint8_t** mask, uint8_t** data);</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets various info about <em>bitmap</em> including its <em>width</em> and <em>height</em> and raw pixel data. The data is 1 bit per pixel packed format, in MSB order; in other words, the high bit of the first byte in <code>data</code> is the top left pixel of the image. If the bitmap has a mask, a pointer to its data is returned in <em>mask</em>, else NULL is returned.</p>
	</div>
	</div>
	*/
	pub getBitmapData: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                        width: *mut core::ffi::c_int,
	                                        height: *mut core::ffi::c_int,
	                                        rowbytes: *mut core::ffi::c_int,
	                                        mask: *mut *mut u8,
	                                        data: *mut *mut u8),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;clearBitmap(LCDBitmap* bitmap, LCDColor bgcolor);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears <em>bitmap</em>, filling with the given <em>bgcolor</em>.</p>
	</div>
	</div>
	*/
	pub clearBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap, bgcolor: Color),
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;rotatedBitmap(LCDBitmap* bitmap, float rotation, float xscale, float yscale, int* allocedSize);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new, rotated and scaled LCDBitmap based on the given <em>bitmap</em>.</p>
	</div>
	</div>
	*/
	pub rotatedBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                        rotation: core::ffi::c_float,
	                                        xscale: core::ffi::c_float,
	                                        yscale: core::ffi::c_float,
	                                        allocedSize: *mut core::ffi::c_int)
	                                        -> *mut Bitmap,
	/**
	<code class="title">LCDBitmapTable* playdate-&gt;graphics-&gt;newBitmapTable(int count, int width, int height);</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new LCDBitmapTable that can hold <em>count</em> <em>width</em> by <em>height</em> LCDBitmaps.</p>
	</div>
	</div>
	*/
	pub newBitmapTable: unsafe extern "C" fn(count: core::ffi::c_int,
	                                         width: core::ffi::c_int,
	                                         height: core::ffi::c_int)
	                                         -> *mut BitmapTable,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;freeBitmapTable(LCDBitmapTable* table);</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given bitmap table. Note that this will invalidate any bitmaps returned by <code>getTableBitmap()</code>.</p>
	</div>
	</div>
	*/
	pub freeBitmapTable: unsafe extern "C" fn(table: *mut BitmapTable),
	/**
	<code class="title">LCDBitmapTable* playdate-&gt;graphics-&gt;loadBitmapTable(const char* path, const char** outerr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new LCDBitmap from the file at <em>path</em>. If there is no file at <em>path</em>, the function returns null.</p>
	</div>
	</div>
	*/
	pub loadBitmapTable: unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                          outerr: *mut *const core::ffi::c_char)
	                                          -> *mut BitmapTable,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;loadIntoBitmapTable(const char* path, LCDBitmapTable* table, const char** outerr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Loads the imagetable at <em>path</em> into the previously allocated <em>table</em>.</p>
	</div>
	</div>
	*/
	pub loadIntoBitmapTable: unsafe extern "C" fn(path: *const core::ffi::c_char,
	                                              table: *mut BitmapTable,
	                                              outerr: *mut *const core::ffi::c_char),
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;getTableBitmap(LCDBitmapTable* table, int idx);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the <em>idx</em> bitmap in <em>table</em>, If <em>idx</em> is out of bounds, the function returns NULL.</p>
	</div>
	</div>
	*/
	pub getTableBitmap: unsafe extern "C" fn(table: *mut BitmapTable, idx: core::ffi::c_int) -> *mut Bitmap,
	/**
	<code class="title">LCDFont* playdate-&gt;graphics-&gt;loadFont(const char* path, const char** outErr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the LCDFont object for the font file at <em>path</em>. In case of error, <em>outErr</em> points to a string describing the error. The returned font can be freed with <a href="#f-system.realloc">playdate→system→realloc(font, 0)</a> when it is no longer in use.</p>
	</div>
	</div>
	*/
	pub loadFont:
		unsafe extern "C" fn(path: *const core::ffi::c_char, outErr: *mut *const core::ffi::c_char) -> *mut Font,
	/**
	<code class="title">LCDFontPage* playdate-&gt;graphics-&gt;getFontPage(LCDFont* font, uint32_t c);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an LCDFontPage object for the given character code. Each LCDFontPage contains information for 256 characters; specifically, if <code>(c1 &amp; ~0xff) == (c2 &amp; ~0xff)</code>, then <em>c1</em> and <em>c2</em> belong to the same page and the same LCDFontPage can be used to fetch the character data for both instead of searching for the page twice.</p>
	</div>
	</div>
	*/
	pub getFontPage: unsafe extern "C" fn(font: *mut Font, c: u32) -> *mut FontPage,
	/**
	<code class="title">LCDFontGlyph* playdate-&gt;graphics-&gt;getPageGlyph(LCDFontPage* page, uint32_t c, LCDBitmap** bitmap, int* advance);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an LCDFontGlyph object for character <em>c</em> in LCDFontPage <em>page</em>, and optionally returns the glyph’s bitmap and advance value.</p>
	</div>
	</div>
	*/
	pub getPageGlyph: unsafe extern "C" fn(page: *mut FontPage,
	                                       c: u32,
	                                       bitmap: *mut *mut Bitmap,
	                                       advance: *mut core::ffi::c_int)
	                                       -> *mut FontGlyph,
	/**
	<code class="title">int playdate-&gt;graphics-&gt;getGlyphKerning(LCDFontGlyph* glyph, uint32_t c1, uint32_t c2);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the kerning adjustment between characters <em>c1</em> and <em>c2</em> as specified by the font.</p>
	</div>
	</div>
	*/
	pub getGlyphKerning:
		unsafe extern "C" fn(glyph: *mut FontGlyph, glyphcode: u32, nextcode: u32) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;graphics-&gt;getTextWidth(LCDFont* font, const void* text, size_t len, PDStringEncoding encoding, int tracking);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the width of the given text in the given font. See the <a href="#f-graphics.drawText">note above</a> about the <code>len</code> argument.</p>
	</div>
	<div class="literalblock">
	<div class="title">PDStringEncoding</div>
	<div class="content">
	<pre>typedef enum
	{
		kASCIIEncoding,
		kUTF8Encoding,
		k16BitLEEncoding
	} PDStringEncoding;</pre>
	</div>
	</div>
	</div>
	*/
	pub getTextWidth: unsafe extern "C" fn(font: *mut Font,
	                                       text: *const core::ffi::c_void,
	                                       len: usize,
	                                       encoding: StringEncoding,
	                                       tracking: core::ffi::c_int)
	                                       -> core::ffi::c_int,
	/**
	<code class="title">uint8_t* playdate-&gt;graphics-&gt;getFrame(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the current display frame buffer. Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored. Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.</p>
	</div>
	</div>
	*/
	pub getFrame: unsafe extern "C" fn() -> *mut u8,
	/**
	<code class="title">uint8_t* playdate-&gt;graphics-&gt;getDisplayFrame(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the raw bits in the display buffer, the last completed frame.</p>
	</div>
	</div>
	*/
	pub getDisplayFrame: unsafe extern "C" fn() -> *mut u8,
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;getDebugBitmap(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Only valid in the Simulator; function is NULL on device. Returns the debug framebuffer as a bitmap. White pixels drawn in the image are overlaid on the display in 50% transparent red.</p>
	</div>
	</div>
	*/
	pub getDebugBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut Bitmap>,
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;copyFrameBufferBitmap(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a copy the contents of the working frame buffer as a bitmap. The caller is responsible for freeing the returned bitmap with <a href="#f-graphics.freeBitmap">playdate-&gt;graphics-&gt;freeBitmap()</a>.</p>
	</div>
	</div>
	*/
	pub copyFrameBufferBitmap: unsafe extern "C" fn() -> *mut Bitmap,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;markUpdatedRows(int start, int end);</code>
	<div class="content">
	<div class="paragraph">
	<p>After updating pixels in the buffer returned by getFrame(), you must tell the graphics system which rows were updated. This function marks a contiguous range of rows as updated (e.g., markUpdatedRows(0,LCD_ROWS-1) tells the system to update the entire display). Both “start” and “end” are included in the range.</p>
	</div>
	</div>
	*/
	pub markUpdatedRows: unsafe extern "C" fn(start: core::ffi::c_int, end: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;display(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Manually flushes the current frame buffer out to the display. This function is automatically called after each pass through the run loop, so there shouldn’t be any need to call it yourself.</p>
	</div>
	</div>
	*/
	pub display: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setColorToPattern(LCDColor* color, LCDBitmap* bitmap, int x, int y);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets <em>color</em> to an 8 x 8 pattern using the given <em>bitmap</em>. <em>x</em>, <em>y</em> indicates the top left corner of the 8 x 8 pattern.</p>
	</div>
	</div>
	*/
	pub setColorToPattern:
		unsafe extern "C" fn(color: *mut Color, bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;graphics-&gt;checkMaskCollision(LCDBitmap* bitmap1, int x1, int y1, LCDBitmapFlip flip1, LCDBitmap* bitmap2, int x2, int y2, LCDBitmapFlip flip2, LCDRect rect);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns 1 if any of the opaque pixels in <em>bitmap1</em> when positioned at <em>x1</em>, <em>y1</em> with <em>flip1</em> overlap any of the opaque pixels in <em>bitmap2</em> at <em>x2</em>, <em>y2</em> with <em>flip2</em> within the non-empty <em>rect</em>, or 0 if no pixels overlap or if one or both fall completely outside of <em>rect</em>.</p>
	</div>
	</div>
	*/
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
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setScreenClipRect(int x, int y, int width, int height);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the current clip rect in screen coordinates.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setScreenClipRect"><code>playdate.graphics.setScreenClipRect()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setScreenClipRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                            y: core::ffi::c_int,
	                                            width: core::ffi::c_int,
	                                            height: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;fillPolygon(int nPoints, int* points, LCDColor color, LCDPolygonFillRule fillrule);</code>
	<div class="content">
	<div class="paragraph">
	<p>Fills the polygon with vertices at the given coordinates (an array of 2*<code>nPoints</code> ints containing alternating x and y values) using the given color and fill, or winding, rule. See <a href="https://en.wikipedia.org/wiki/Nonzero-rule" class="bare">https://en.wikipedia.org/wiki/Nonzero-rule</a> for an explanation of the winding rule. An edge between the last vertex and the first is assumed.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.fillPolygon"><code>playdate.graphics.fillPolygon()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub fillPolygon: unsafe extern "C" fn(nPoints: core::ffi::c_int,
	                                      coords: *mut core::ffi::c_int,
	                                      color: Color,
	                                      fillrule: PolygonFillRule),
	/**
	<code class="title">uint8_t playdate-&gt;graphics-&gt;getFontHeight(LCDFont* font);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the height of the given font.</p>
	</div>
	</div>
	*/
	pub getFontHeight: unsafe extern "C" fn(font: *mut Font) -> u8,
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;getDisplayBufferBitmap(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a bitmap containing the contents of the display buffer. The system owns this bitmap—​do not free it!</p>
	</div>
	</div>
	*/
	pub getDisplayBufferBitmap: unsafe extern "C" fn() -> *mut Bitmap,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;drawRotatedBitmap(LCDBitmap* bitmap, int x, int y, float degrees, float centerx, float centery, float xscale, float yscale);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws the <em>bitmap</em> scaled to <em>xscale</em> and <em>yscale</em> then rotated by <em>degrees</em> with its center as given by proportions <em>centerx</em> and <em>centery</em> at <em>x</em>, <em>y</em>; that is: if <em>centerx</em> and <em>centery</em> are both 0.5 the center of the image is at (<em>x</em>,<em>y</em>), if <em>centerx</em> and <em>centery</em> are both 0 the top left corner of the image (before rotation) is at (<em>x</em>,<em>y</em>), etc.</p>
	</div>
	</div>
	*/
	pub drawRotatedBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	                                            x: core::ffi::c_int,
	                                            y: core::ffi::c_int,
	                                            rotation: core::ffi::c_float,
	                                            centerx: core::ffi::c_float,
	                                            centery: core::ffi::c_float,
	                                            xscale: core::ffi::c_float,
	                                            yscale: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setTextLeading(int leading);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#m-graphics.font.setLeading"><code>playdate.graphics.font:setLeading()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setTextLeading: unsafe extern "C" fn(lineHeightAdustment: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;graphics-&gt;setBitmapMask(LCDBitmap* bitmap, LCDBitmap* mask);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a mask image for the given <em>bitmap</em>. The set mask must be the same size as the target bitmap.</p>
	</div>
	</div>
	*/
	pub setBitmapMask: unsafe extern "C" fn(bitmap: *mut Bitmap, mask: *mut Bitmap) -> core::ffi::c_int,
	/**
	<code class="title">LCDBitmap* playdate-&gt;graphics-&gt;getBitmapMask(LCDBitmap* bitmap);</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets a mask image for the given <em>bitmap</em>, or returns NULL if the <em>bitmap</em> doesn’t have a mask layer. The returned image points to <em>bitmap</em>'s data, so drawing into the mask image affects the source bitmap directly. The caller takes ownership of the returned LCDBitmap and is responsible for freeing it when it’s no longer in use.</p>
	</div>
	</div>
	*/
	pub getBitmapMask: unsafe extern "C" fn(bitmap: *mut Bitmap) -> *mut Bitmap,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setStencilImage(LCDBitmap* stencil, int tile);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the stencil used for drawing. If the <em>tile</em> flag is set the stencil image will be tiled. Tiled stencils must have width equal to a multiple of 32 pixels. To clear the stencil, call <code>playdate→graphics→setStencil(NULL);</code>.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-graphics.setStencilImage"><code>playdate.graphics.setStencilImage()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setStencilImage: unsafe extern "C" fn(stencil: *mut Bitmap, tile: core::ffi::c_int),
	/**
	<code class="title">LCDFont* playdate-&gt;graphics-&gt;makeFontFromData(LCDFontData* data, int wide);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an LCDFont object wrapping the LCDFontData <em>data</em> comprising the contents (minus 16-byte header) of an uncompressed pft file. <em>wide</em> corresponds to the flag in the header indicating whether the font contains glyphs at codepoints above U+1FFFF.</p>
	</div>
	</div>
	*/
	pub makeFontFromData: unsafe extern "C" fn(data: *mut FontData, wide: core::ffi::c_int) -> *mut Font,
	/**
	<code class="title">int playdate-&gt;graphics-&gt;getTextTracking(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the tracking used when drawing text.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#m-graphics.font.getTracking"><code>playdate.graphics.font:getTracking()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub getTextTracking: unsafe extern "C" fn() -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;setPixel(int x, int y, LCDColor color);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the pixel at <em>(x,y)</em> in the current drawing context (by default the screen) to the given <em>color</em>. Be aware that setting a pixel at a time is not very efficient: In our testing, more than around 20,000 calls in a tight loop will drop the frame rate below 30 fps.</p>
	</div>
	</div>
	*/
	pub setPixel: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int, c: Color),
	/**
	<code class="title">LCDSolidColor playdate-&gt;graphics-&gt;getBitmapPixel(LCDBitmap* bitmap, int x, int y);</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the color of the pixel at <em>(x,y)</em> in the given <em>bitmap</em>. If the coordinate is outside the bounds of the bitmap, or if the bitmap has a mask and the pixel is marked transparent, the function returns <code>kColorClear</code>; otherwise the return value is <code>kColorWhite</code> or <code>kColorBlack</code>.</p>
	</div>
	</div>
	*/
	pub getBitmapPixel:
		unsafe extern "C" fn(bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int) -> SolidColor,
	/**
	<code class="title">void playdate-&gt;graphics-&gt;getBitmapTableInfo(LCDBitmapTable* table, int* count, int* cellswide);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the bitmap table’s image count in the <em>count</em> pointer (if not NULL) and number of cells across in the <em>cellswide</em> pointer (ditto).</p>
	</div>
	</div>
	*/
	pub getBitmapTableInfo:
		unsafe extern "C" fn(table: *mut BitmapTable, count: *mut core::ffi::c_int, width: *mut core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;graphics-&gt;drawTextInRect(const void* text, size_t len, PDStringEncoding encoding, int x, int y, int width, int height, PDTextWrappingMode wrap, PDTextAlignment align);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws the text in the given rectangle using the provided options. If no font has been set with <a href="#f-graphics.setFont">setFont</a>, the default system font Asheville Sans 14 Light is used. See the <a href="#f-graphics.drawText">above note</a> about the <code>len</code> argument.</p>
	</div>
	<div class="paragraph">
	<p>The <em>wrap</em> argument is one of</p>
	</div>
	<div class="literalblock">
	<div class="title">PDTextWrappingMode</div>
	<div class="content">
	<pre>typedef enum
	{
		kWrapClip,
		kWrapCharacter,
		kWrapWord,
	} PDTextWrappingMode;</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>and <em>align</em> is one of</p>
	</div>
	<div class="literalblock">
	<div class="title">PDTextAlignment</div>
	<div class="content">
	<pre>typedef enum
	{
		kAlignTextLeft,
		kAlignTextCenter,
		kAlignTextRight
	} PDTextAlignment;</pre>
	</div>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub struct Buttons(pub u8);
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum Language {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, PartialEq)]
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
#[must_use]
pub struct MenuItem {
	_unused: [u8; 0],
}
#[repr(u16)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
#[must_use]
pub struct PlaydateSys {
    /**
<code class="title">void* playdate-&gt;system-&gt;realloc(void* ptr, size_t size)</code>
<div class="content">
<div class="paragraph">
<p>Allocates heap space if <em>ptr</em> is NULL, else reallocates the given pointer. If <em>size</em> is zero, frees the given pointer.</p>
</div>
</div>
*/
    pub realloc: unsafe extern "C" fn(
        ptr: *mut core::ffi::c_void,
        size: usize,
    ) -> *mut core::ffi::c_void,
    /**
<code class="title">int playdate-&gt;system-&gt;formatString(char **outstring, const char *format, ...)</code>
<div class="content">
<div class="paragraph">
<p>Creates a formatted string and returns it via the <em>outstring</em> argument. The arguments and return value match libc’s <code>asprintf()</code>: the format string is standard <code>printf()</code> style, the string returned in <em>outstring</em> should be freed by the caller when it’s no longer in use, and the return value is the length of the formatted string.</p>
</div>
</div>
*/
    pub formatString: unsafe extern "C" fn(
        ret: *mut *mut core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;system-&gt;logToConsole(const char* format, ...)</code>
<div class="content">
<div class="paragraph">
<p>Calls the log function.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-print"><code>print()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub logToConsole: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...),
    /**
<code class="title">void playdate-&gt;system-&gt;error(const char* format, ...)</code>
<div class="content">
<div class="paragraph">
<p>Calls the log function, outputting an error in red to the console, then pauses execution.</p>
</div>
</div>
*/
    pub error: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...) -> !,
    /**
<code class="title">PDLanguage playdate-&gt;system-&gt;getLanguage(void);</code>
<div class="content">
<div class="paragraph">
<p>Returns the current language of the system.</p>
</div>
</div>
*/
    pub getLanguage: unsafe extern "C" fn() -> Language,
    /**
<code class="title">unsigned int playdate-&gt;system-&gt;getCurrentTimeMilliseconds(void)</code>
<div class="content">
<div class="paragraph">
<p>Returns the number of milliseconds since…​some arbitrary point in time. This should present a consistent timebase while a game is running, but the counter will be disabled when the device is sleeping.</p>
</div>
</div>
*/
    pub getCurrentTimeMilliseconds: unsafe extern "C" fn() -> core::ffi::c_uint,
    /**
<code class="title">unsigned int playdate-&gt;system-&gt;getSecondsSinceEpoch(unsigned int *milliseconds)</code>
<div class="content">
<div class="paragraph">
<p>Returns the number of seconds (and sets <em>milliseconds</em> if not NULL) elapsed since midnight (hour 0), January 1, 2000.</p>
</div>
</div>
*/
    pub getSecondsSinceEpoch: unsafe extern "C" fn(
        milliseconds: *mut core::ffi::c_uint,
    ) -> core::ffi::c_uint,
    /**
<code class="title">void playdate-&gt;system-&gt;drawFPS(int x, int y)</code>
<div class="content">
<div class="paragraph">
<p>Calculates the current frames per second and draws that value at <em>x, y</em>.</p>
</div>
</div>
*/
    pub drawFPS: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
    /**
<code class="title">void playdate-&gt;system-&gt;setUpdateCallback(PDCallbackFunction* update, void* userdata)</code>
<div class="content">
<div class="literalblock">
<div class="title">PDCallbackFunction</div>
<div class="content">
<pre>int PDCallbackFunction(void* userdata);</pre>
</div>
</div>
<div class="paragraph">
<p>Replaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed.</p>
</div>
</div>
*/
    pub setUpdateCallback: unsafe extern "C" fn(
        update: CallbackFunction,
        userdata: *mut core::ffi::c_void,
    ),
    /**
<code class="title">void playdate-&gt;system-&gt;getButtonState(PDButtons* current, PDButtons* pushed, PDButtons* released)</code>
<div class="content">
<div class="paragraph">
<p>Sets the value pointed to by <em>current</em> to a bitmask indicating which buttons are currently down. <em>pushed</em> and <em>released</em> reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.</p>
</div>
<div id="_PDButton" class="literalblock">
<div class="title">PDButton</div>
<div class="content">
<pre>kButtonLeft
kButtonRight
kButtonUp
kButtonDown
kButtonB
kButtonA</pre>
</div>
</div>
</div>
*/
    pub getButtonState: unsafe extern "C" fn(
        current: *mut Buttons,
        pushed: *mut Buttons,
        released: *mut Buttons,
    ),
    /**
<code class="title">void playdate-&gt;system-&gt;setPeripheralsEnabled(PDPeripherals mask)</code>
<div class="content">
<div class="paragraph">
<p>By default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.</p>
</div>
<div class="literalblock">
<div class="title">PDPeripherals</div>
<div class="content">
<pre>kNone
kAccelerometer</pre>
</div>
</div>
</div>
*/
    pub setPeripheralsEnabled: unsafe extern "C" fn(mask: Peripherals),
    /**
<code class="title">void playdate-&gt;system-&gt;getAccelerometer(float* outx, float* outy, float* outz)</code>
<div class="content">
<div class="paragraph">
<p>Returns the last-read accelerometer data.</p>
</div>
</div>
*/
    pub getAccelerometer: unsafe extern "C" fn(
        outx: *mut core::ffi::c_float,
        outy: *mut core::ffi::c_float,
        outz: *mut core::ffi::c_float,
    ),
    /**
<code class="title">float playdate-&gt;system-&gt;getCrankChange(void)</code>
<div class="content">
<div class="paragraph">
<p>Returns the angle change of the crank since the last time this function was called. Negative values are anti-clockwise.</p>
</div>
</div>
*/
    pub getCrankChange: unsafe extern "C" fn() -> core::ffi::c_float,
    /**
<code class="title">float playdate-&gt;system-&gt;getCrankAngle(void)</code>
<div class="content">
<div class="paragraph">
<p>Returns the current position of the crank, in the range 0-360. Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.</p>
</div>
</div>
*/
    pub getCrankAngle: unsafe extern "C" fn() -> core::ffi::c_float,
    /**
<code class="title">int playdate-&gt;system-&gt;isCrankDocked(void)</code>
<div class="content">
<div class="paragraph">
<p>Returns 1 or 0 indicating whether or not the crank is folded into the unit.</p>
</div>
</div>
*/
    pub isCrankDocked: unsafe extern "C" fn() -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;system-&gt;setCrankSoundsDisabled(int disable)</code>
<div class="content">
<div class="paragraph">
<p>The function returns the previous value for this setting.</p>
</div>
</div>
*/
    pub setCrankSoundsDisabled: unsafe extern "C" fn(
        flag: core::ffi::c_int,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;system-&gt;getFlipped()</code>
<div class="content">
<div class="paragraph">
<p>Returns 1 if the global "flipped" system setting is set, otherwise 0.</p>
</div>
</div>
*/
    pub getFlipped: unsafe extern "C" fn() -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;system-&gt;setAutoLockDisabled(int disable)</code>
<div class="content">
<div class="paragraph">
<p>Disables or enables the 3 minute auto lock feature. When called, the timer is reset to 3 minutes.</p>
</div>
</div>
*/
    pub setAutoLockDisabled: unsafe extern "C" fn(disable: core::ffi::c_int),
    /**
<code class="title">void playdate-&gt;system-&gt;setMenuImage(LCDBitmap* bitmap, int xOffset);</code>
<div class="content">
<div class="paragraph">
<p>A game can optionally provide an image to be displayed alongside the system menu. <em>bitmap</em> must be a 400x240 LCDBitmap. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.</p>
</div>
<div class="paragraph">
<p>Optionally, a non-zero <em>xoffset</em>, can be provided. This must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by xoffset pixels as the menu is animated in.</p>
</div>
<div class="paragraph">
<p>This function could be called in response to the kEventPause <em>event</em> in your implementation of <a href="#_eventHandler">eventHandler()</a>.</p>
</div>
</div>
*/
    pub setMenuImage: unsafe extern "C" fn(
        bitmap: *mut Bitmap,
        xOffset: core::ffi::c_int,
    ),
    /**
<code class="title">PDMenuItem* playdate-&gt;system-&gt;addMenuItem(const char* title, PDMenuItemCallbackFunction* callback, void* userdata)</code>
<div class="content">
<div class="paragraph">
<p><em>title</em> will be the title displayed by the menu item.</p>
</div>
<div class="paragraph">
<p>Adds a new menu item to the System Menu. When invoked by the user, this menu item will:</p>
</div>
<div class="olist arabic">
<ol class="arabic">
<li>
<p>Invoke your <em>callback</em> function.</p>
</li>
<li>
<p>Hide the System Menu.</p>
</li>
<li>
<p>Unpause your game and call <a href="#_eventHandler">eventHandler()</a> with the kEventResume <em>event</em>.</p>
</li>
</ol>
</div>
<div class="paragraph">
<p>Your game can then present an options interface to the player, or take other action, in whatever manner you choose.</p>
</div>
<div class="paragraph">
<p>The returned menu item is freed when removed from the menu; it does not need to be freed manually.</p>
</div>
</div>
*/
    pub addMenuItem: unsafe extern "C" fn(
        title: *const core::ffi::c_char,
        callback: MenuItemCallbackFunction,
        userdata: *mut core::ffi::c_void,
    ) -> *mut MenuItem,
    /**
<code class="title">PDMenuItem* playdate-&gt;system-&gt;addCheckmarkMenuItem(const char* title, int value, PDMenuItemCallbackFunction* callback, void* userdata)</code>
<div class="content">
<div class="paragraph">
<p>Adds a new menu item that can be checked or unchecked by the player.</p>
</div>
<div class="paragraph">
<p><em>title</em> will be the title displayed by the menu item.</p>
</div>
<div class="paragraph">
<p><em>value</em> should be 0 for unchecked, 1 for checked.</p>
</div>
<div class="paragraph">
<p>If this menu item is interacted with while the system menu is open, <em>callback</em> will be called when the menu is closed.</p>
</div>
<div class="paragraph">
<p>The returned menu item is freed when removed from the menu; it does not need to be freed manually.</p>
</div>
</div>
*/
    pub addCheckmarkMenuItem: unsafe extern "C" fn(
        title: *const core::ffi::c_char,
        value: core::ffi::c_int,
        callback: MenuItemCallbackFunction,
        userdata: *mut core::ffi::c_void,
    ) -> *mut MenuItem,
    /**
<code class="title">PDMenuItem* playdate-&gt;system-&gt;addOptionsMenuItem(const char* title, const char** options, int optionsCount, PDMenuItemCallbackFunction* callback, void* userdata)</code>
<div class="content">
<div class="paragraph">
<p>Adds a new menu item that allows the player to cycle through a set of options.</p>
</div>
<div class="paragraph">
<p><em>title</em> will be the title displayed by the menu item.</p>
</div>
<div class="paragraph">
<p><em>options</em> should be an array of strings representing the states this menu item can cycle through. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.</p>
</div>
<div class="paragraph">
<p><em>optionsCount</em> should be the number of items contained in <em>options</em>.</p>
</div>
<div class="paragraph">
<p>If this menu item is interacted with while the system menu is open, <em>callback</em> will be called when the menu is closed.</p>
</div>
<div class="paragraph">
<p>The returned menu item is freed when removed from the menu; it does not need to be freed manually.</p>
</div>
</div>
*/
    pub addOptionsMenuItem: unsafe extern "C" fn(
        title: *const core::ffi::c_char,
        optionTitles: *mut *const core::ffi::c_char,
        optionsCount: core::ffi::c_int,
        f: MenuItemCallbackFunction,
        userdata: *mut core::ffi::c_void,
    ) -> *mut MenuItem,
    /**
<code class="title">void playdate-&gt;system-&gt;removeAllMenuItems()</code>
<div class="content">
<div class="paragraph">
<p>Removes all custom menu items from the system menu.</p>
</div>
</div>
*/
    pub removeAllMenuItems: unsafe extern "C" fn(),
    /**
<code class="title">void playdate-&gt;system-&gt;removeMenuItem(PDMenuItem *menuItem)</code>
<div class="content">
<div class="paragraph">
<p>Removes the menu item from the system menu.</p>
</div>
</div>
*/
    pub removeMenuItem: unsafe extern "C" fn(menuItem: *mut MenuItem),
    /**
<code class="title">int playdate-&gt;system-&gt;getMenuItemValue(PDMenuItem *menuItem)</code>
<div class="content">

</div>
*/
    pub getMenuItemValue: unsafe extern "C" fn(
        menuItem: *mut MenuItem,
    ) -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;system-&gt;setMenuItemValue(PDMenuItem *menuItem, int value)</code>
<div class="content">
<div class="paragraph">
<p>Gets or sets the integer value of the menu item.</p>
</div>
<div class="paragraph">
<p>For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.</p>
</div>
</div>
*/
    pub setMenuItemValue: unsafe extern "C" fn(
        menuItem: *mut MenuItem,
        value: core::ffi::c_int,
    ),
    /**
<code class="title">const char* playdate-&gt;system-&gt;getMenuItemTitle(PDMenuItem *menuItem)</code>
<div class="content">

</div>
*/
    pub getMenuItemTitle: unsafe extern "C" fn(
        menuItem: *mut MenuItem,
    ) -> *const core::ffi::c_char,
    /**
<code class="title">void playdate-&gt;system-&gt;setMenuItemTitle(PDMenuItem *menuItem, const char* title)</code>
<div class="content">
<div class="paragraph">
<p>Gets or sets the display title of the menu item.</p>
</div>
</div>
*/
    pub setMenuItemTitle: unsafe extern "C" fn(
        menuItem: *mut MenuItem,
        title: *const core::ffi::c_char,
    ),
    /**
<code class="title">void* playdate-&gt;system-&gt;getMenuItemUserdata(PDMenuItem *menuItem)</code>
<div class="content">

</div>
*/
    pub getMenuItemUserdata: unsafe extern "C" fn(
        menuItem: *mut MenuItem,
    ) -> *mut core::ffi::c_void,
    /**
<code class="title">void playdate-&gt;system-&gt;setMenuItemUserdata(PDMenuItem *menuItem, void* userdata)</code>
<div class="content">
<div class="paragraph">
<p>Gets or sets the userdata value associated with this menu item.</p>
</div>
</div>
*/
    pub setMenuItemUserdata: unsafe extern "C" fn(
        menuItem: *mut MenuItem,
        ud: *mut core::ffi::c_void,
    ),
    /**
<code class="title">int playdate-&gt;system-&gt;getReduceFlashing()</code>
<div class="content">
<div class="paragraph">
<p>Returns 1 if the global "reduce flashing" system setting is set, otherwise 0.</p>
</div>
</div>
*/
    pub getReduceFlashing: unsafe extern "C" fn() -> core::ffi::c_int,
    /**
<code class="title">float playdate-&gt;system-&gt;getElapsedTime()</code>
<div class="content">
<div class="paragraph">
<p>Returns the number of seconds since <code>playdate.resetElapsedTime()</code> was called. The value is a floating-point number with microsecond accuracy.</p>
</div>
</div>
*/
    pub getElapsedTime: unsafe extern "C" fn() -> core::ffi::c_float,
    /**
<code class="title">void playdate-&gt;system-&gt;resetElapsedTime(void)</code>
<div class="content">
<div class="paragraph">
<p>Resets the high-resolution timer.</p>
</div>
</div>
*/
    pub resetElapsedTime: unsafe extern "C" fn(),
    /**
<code class="title">float playdate-&gt;system-&gt;getBatteryPercentage()</code>
<div class="content">
<div class="paragraph">
<p>Returns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full.</p>
</div>
</div>
*/
    pub getBatteryPercentage: unsafe extern "C" fn() -> core::ffi::c_float,
    /**
<code class="title">float playdate-&gt;system-&gt;getBatteryVoltage()</code>
<div class="content">
<div class="paragraph">
<p>Returns the battery’s current voltage level.</p>
</div>
</div>
*/
    pub getBatteryVoltage: unsafe extern "C" fn() -> core::ffi::c_float,
    /**
<code class="title">int32_t playdate-&gt;system-&gt;getTimezoneOffset()</code>
<div class="content">
<div class="paragraph">
<p>Returns the system timezone offset from GMT, in seconds.</p>
</div>
</div>
*/
    pub getTimezoneOffset: unsafe extern "C" fn() -> i32,
    /**
<code class="title">int playdate-&gt;system-&gt;shouldDisplay24HourTime()</code>
<div class="content">
<div class="paragraph">
<p>Returns 1 if the user has set the 24-Hour Time preference in the Settings program.</p>
</div>
</div>
*/
    pub shouldDisplay24HourTime: unsafe extern "C" fn() -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;system-&gt;convertEpochToDateTime(uint32_t epoch, struct PDDateTime* datetime)</code>
<div class="content">
<div class="paragraph">
<p>Converts the given epoch time to a PDDateTime.</p>
</div>
</div>
*/
    pub convertEpochToDateTime: unsafe extern "C" fn(
        epoch: u32,
        datetime: *mut DateTime,
    ),
    /**
<code class="title">uint32_t playdate-&gt;system-&gt;convertDateTimeToEpoch(struct PDDateTime* datetime)</code>
<div class="content">
<div class="paragraph">
<p>Converts the given PDDateTime to an epoch time.</p>
</div>
</div>
*/
    pub convertDateTimeToEpoch: unsafe extern "C" fn(datetime: *mut DateTime) -> u32,
    /**
<code class="title">float playdate-&gt;system-&gt;clearICache()</code>
<div class="content">
<div class="paragraph">
<p>Flush the CPU instruction cache, on the very unlikely chance you’re modifying instruction code on the fly. (If you don’t know what I’m talking about, you don’t need this. :smile:)</p>
</div>
</div>
*/
    pub clearICache: unsafe extern "C" fn(),
    /**
<code class="title">void playdate-&gt;system-&gt;setButtonCallback(PDButtonCallbackFunction* cb, void* userdata, int queuesize)</code>
<div class="content">
<div class="paragraph">
<p>As an alternative to polling for button presses using <code>getButtonState()</code>, this function allows a callback function to be set. The function is called for each button up/down event (possibly multiple events on the same button) that occurred during the previous update cycle. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. The function should return 0 on success or a non-zero value to signal an error.</p>
</div>
<div class="literalblock">
<div class="title">PDButtonCallbackFunction</div>
<div class="content">
<pre>typedef int PDButtonCallbackFunction(PDButtons button, int down, uint32_t when, void* userdata);</pre>
</div>
</div>
</div>
*/
    pub setButtonCallback: unsafe extern "C" fn(
        cb: ButtonCallbackFunction,
        buttonud: *mut core::ffi::c_void,
        queuesize: core::ffi::c_int,
    ),
    /**
<code class="title">void playdate-&gt;system-&gt;setSerialMessageCallback(void (*callback)(const char* data));</code>
<div class="content">
<div class="paragraph">
<p>Provides a callback to receive messages sent to the device over the serial port using the <code>msg</code> command. If no device is connected, you can send these messages to a game in the simulator by entering <code>!msg &lt;message&gt;</code> in the Lua console.</p>
</div>
</div>
*/
    pub setSerialMessageCallback: unsafe extern "C" fn(
        callback: ::core::option::Option<
            unsafe extern "C" fn(data: *const core::ffi::c_char),
        >,
    ),
    /**
<code class="title">int playdate-&gt;system-&gt;vaFormatString(char **ret, const char *format, va_list args)</code>
<div class="content">
<div class="paragraph">
<p>Allocates and formats a string using a variadic <code>va_list</code> argument, in the style of <code>vasprintf()</code>. The string returned via <em>ret</em> should be freed by the caller when it is no longer in use. The return value from the function is the length of the formatted string.</p>
</div>
</div>
*/
    pub vaFormatString: unsafe extern "C" fn(
        outstr: *mut *mut core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        args: va_list,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;system-&gt;parseString(const char *str, const char *format, ...)</code>
<div class="content">
<div class="paragraph">
<p>Like libc <code>sscanf()</code>, parses a string according to a format string and places the values into pointers passed in after the format. The return value is the number of items matched.</p>
</div>
</div>
*/
    pub parseString: unsafe extern "C" fn(
        str_: *const core::ffi::c_char,
        format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int,
}
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
#[must_use]
pub struct LuaUdObject {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct Sprite {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum LuaValueType {
	Int = 0,
	Float = 1,
	Str = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq)]
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
impl Default for LuaReg {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
impl Default for LuaValBindgenTy1 {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of LuaVal"][::core::mem::size_of::<LuaVal>() - 12usize];
	["Alignment of LuaVal"][::core::mem::align_of::<LuaVal>() - 4usize];
	["Offset of field: LuaVal::name"][::core::mem::offset_of!(LuaVal, name) - 0usize];
	["Offset of field: LuaVal::type_"][::core::mem::offset_of!(LuaVal, type_) - 4usize];
	["Offset of field: LuaVal::v"][::core::mem::offset_of!(LuaVal, v) - 8usize];
};
impl Default for LuaVal {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(C)]
#[must_use]
pub struct PlaydateLua {
	/**
	<code class="title">int playdate-&gt;lua-&gt;addFunction(lua_CFunction f, const char* name, const char** outErr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds the Lua function <em>f</em> to the Lua runtime, with name <em>name</em>. (<em>name</em> can be a table path using dots, e.g. if name = “mycode.myDrawingFunction” adds the function “myDrawingFunction” to the global table “myCode”.) Returns 1 on success or 0 with an error message in <em>outErr</em>.</p>
	</div>
	</div>
	*/
	pub addFunction: unsafe extern "C" fn(f: LuaCFunction,
	                                      name: *const core::ffi::c_char,
	                                      outErr: *mut *const core::ffi::c_char)
	                                      -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;lua-&gt;registerClass(const char* name, const lua_reg* reg, const lua_val* vals, int isstatic, const char** outErr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new "class" (i.e., a Lua metatable containing functions) with the given name and adds the given functions and constants to it. If the table is simply a list of functions that won’t be used as a metatable, <em>isstatic</em> should be set to 1 to create a plain table instead of a metatable. Please see <code>C_API/Examples/Array</code> for an example of how to use <code>registerClass</code> to create a Lua table-like object from C.</p>
	</div>
	</div>
	*/
	pub registerClass: unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                        reg: *const LuaReg,
	                                        vals: *const LuaVal,
	                                        isstatic: core::ffi::c_int,
	                                        outErr: *mut *const core::ffi::c_char)
	                                        -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushFunction(lua_CFunction f);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes a <a href="#f-lua.cFunction">lua_CFunction</a> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushFunction: unsafe extern "C" fn(f: LuaCFunction),
	/**
	<code class="title">int playdate-&gt;lua-&gt;indexMetatable(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>If a class includes an <code>__index</code> function, it should call this first to check if the indexed variable exists in the metatable. If the indexMetatable() call returns 1, it has located the variable and put it on the stack, and the <code>__index</code> function should return 1 to indicate a value was found. If indexMetatable() doesn’t find a value, the <code>__index</code> function can then do its custom getter magic.</p>
	</div>
	</div>
	*/
	pub indexMetatable: unsafe extern "C" fn() -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;lua-&gt;stop(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Stops the run loop.</p>
	</div>
	</div>
	*/
	pub stop: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;lua-&gt;start(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Starts the run loop back up.</p>
	</div>
	</div>
	*/
	pub start: unsafe extern "C" fn(),
	/**
	<code class="title">int playdate-&gt;lua-&gt;getArgCount(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the number of arguments passed to the function.</p>
	</div>
	</div>
	*/
	pub getArgCount: unsafe extern "C" fn() -> core::ffi::c_int,
	/**
	<code class="title">enum LuaType playdate-&gt;lua-&gt;getArgType(int pos, const char** outClass);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the type of the variable at stack position <em>pos</em>. If the type is <em>kTypeObject</em> and <em>outClass</em> is non-NULL, it returns the name of the object’s metatable.</p>
	</div>
	</div>
	*/
	pub getArgType:
		unsafe extern "C" fn(pos: core::ffi::c_int, outClass: *mut *const core::ffi::c_char) -> LuaType,
	/**
	<code class="title">int playdate-&gt;lua-&gt;argIsNil(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns 1 if the argument at the given position <em>pos</em> is nil.</p>
	</div>
	</div>
	*/
	pub argIsNil: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;lua-&gt;getArgBool(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns one if the argument at position <em>pos</em> is true, zero if not.</p>
	</div>
	</div>
	*/
	pub getArgBool: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;lua-&gt;getArgInt(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the argument at position <em>pos</em> as an int.</p>
	</div>
	</div>
	*/
	pub getArgInt: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	/**
	<code class="title">float playdate-&gt;lua-&gt;getArgFloat(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the argument at position <em>pos</em> as a float.</p>
	</div>
	</div>
	*/
	pub getArgFloat: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_float,
	/**
	<code class="title">const char* playdate-&gt;lua-&gt;getArgString(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the argument at position <em>pos</em> as a string.</p>
	</div>
	</div>
	*/
	pub getArgString: unsafe extern "C" fn(pos: core::ffi::c_int) -> *const core::ffi::c_char,
	/**
	<code class="title">const char* playdate-&gt;lua-&gt;getArgBytes(int pos, size_t* outlen);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the argument at position <em>pos</em> as a string and sets <em>outlen</em> to its length.</p>
	</div>
	</div>
	*/
	pub getArgBytes: unsafe extern "C" fn(pos: core::ffi::c_int, outlen: *mut usize) -> *const core::ffi::c_char,
	/**
	<code class="title">void* playdate-&gt;lua-&gt;getArgObject(int pos, char* type, LuaUDObject** outud);</code>
	<div class="content">
	<div class="paragraph">
	<p>Checks the object type of the argument at position <em>pos</em> and returns a pointer to it if it’s the correct type. Optionally sets <em>outud</em> to a pointer to the opaque LuaUDObject for the given stack.</p>
	</div>
	</div>
	*/
	pub getArgObject: unsafe extern "C" fn(pos: core::ffi::c_int,
	                                       type_: *mut core::ffi::c_char,
	                                       outud: *mut *mut LuaUdObject)
	                                       -> *mut core::ffi::c_void,
	/**
	<code class="title">LCDBitmap* playdate-&gt;lua-&gt;getBitmap(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the argument at position <em>pos</em> as an LCDBitmap.</p>
	</div>
	</div>
	*/
	pub getBitmap: unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut Bitmap,
	/**
	<code class="title">LCDSprite* playdate-&gt;lua-&gt;getSprite(int pos);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the argument at position <em>pos</em> as an LCDSprite.</p>
	</div>
	</div>
	*/
	pub getSprite: unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut Sprite,
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushNil(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes nil onto the stack.</p>
	</div>
	</div>
	*/
	pub pushNil: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushBool(int val);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the int <em>val</em> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushBool: unsafe extern "C" fn(val: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushInt(int val);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the int <em>val</em> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushInt: unsafe extern "C" fn(val: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushFloat(float val);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the float <em>val</em> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushFloat: unsafe extern "C" fn(val: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushString(char* str);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the string <em>str</em> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushString: unsafe extern "C" fn(str_: *const core::ffi::c_char),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushBytes(char* str, size_t len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Like <em>pushString()</em>, but pushes an arbitrary byte array to the stack, ignoring \0 characters.</p>
	</div>
	</div>
	*/
	pub pushBytes: unsafe extern "C" fn(str_: *const core::ffi::c_char, len: usize),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushBitmap(LCDBitmap* bitmap);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the LCDBitmap <em>bitmap</em> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap),
	/**
	<code class="title">void playdate-&gt;lua-&gt;pushSprite(LCDSprite* sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the LCDSprite <em>sprite</em> onto the stack.</p>
	</div>
	</div>
	*/
	pub pushSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">LuaUDObject* playdate-&gt;lua-&gt;pushObject(void* obj, char* type, int nValues);</code>
	<div class="content">
	<div class="paragraph">
	<p>Pushes the given custom object <em>obj</em> onto the stack and returns a pointer to the opaque LuaUDObject. <em>type</em> must match the class name used in <a href="#f-lua.registerClass">playdate-&gt;lua-&gt;registerClass()</a>. <em>nValues</em> is the number of slots to allocate for Lua values (see <a href="#f-lua.setObjectValue">set/getObjectValue()</a>).</p>
	</div>
	</div>
	*/
	pub pushObject: unsafe extern "C" fn(obj: *mut core::ffi::c_void,
	                                     type_: *mut core::ffi::c_char,
	                                     nValues: core::ffi::c_int)
	                                     -> *mut LuaUdObject,
	/**
	<code class="title">LuaUDObject* playdate-&gt;lua-&gt;retainObject(LuaUDObject* obj);</code>
	<div class="content">
	<div class="paragraph">
	<p>Retains the opaque LuaUDObject <em>obj</em> and returns same.</p>
	</div>
	</div>
	*/
	pub retainObject: unsafe extern "C" fn(obj: *mut LuaUdObject) -> *mut LuaUdObject,
	/**
	<code class="title">void playdate-&gt;lua-&gt;releaseObject(LuaUDObject* obj);</code>
	<div class="content">
	<div class="paragraph">
	<p>Releases the opaque LuaUDObject <em>obj</em>.</p>
	</div>
	</div>
	*/
	pub releaseObject: unsafe extern "C" fn(obj: *mut LuaUdObject),
	/**
	<code class="title">void playdate-&gt;lua-&gt;setUserValue(LuaUDObject* obj, int slot);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the value of object <em>obj</em>'s uservalue slot number <em>slot</em> (starting at 1, not zero) to the value at the top of the stack.</p>
	</div>
	</div>
	*/
	pub setUserValue: unsafe extern "C" fn(obj: *mut LuaUdObject, slot: core::ffi::c_uint),
	/**
	<code class="title">int playdate-&gt;lua-&gt;getUserValue(LuaUDObject* obj, int slot);</code>
	<div class="content">
	<div class="paragraph">
	<p>Copies the value at <em>obj</em>'s given uservalue <em>slot</em> to the top of the stack and returns its stack position.</p>
	</div>
	</div>
	*/
	pub getUserValue: unsafe extern "C" fn(obj: *mut LuaUdObject, slot: core::ffi::c_uint) -> core::ffi::c_int,
	pub callFunction_deprecated: unsafe extern "C" fn(name: *const core::ffi::c_char, nargs: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;lua-&gt;callFunction(const char* name, int nargs, const char** outerr);</code>
	<div class="content">
	<div class="paragraph">
	<p>Calls the Lua function <em>name</em> and and indicates that <em>nargs</em> number of arguments have already been pushed to the stack for the function to use. <em>name</em> can be a table path using dots, e.g. “playdate.apiVersion”. Returns 1 on success; on failure, returns 0 and puts an error message into the <code>outerr</code> pointer, if it’s set. Calling Lua from C is slow, so use sparingly.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Clone, Hash, PartialEq, Eq)]
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
#[must_use]
pub struct JsonValue {
	pub type_: core::ffi::c_char,
	pub data: JsonValueBindgenTy1,
}
#[repr(C)]
#[must_use]
pub struct JsonValueBindgenTy1 {
	pub intval: __BindgenUnionField<core::ffi::c_int>,
	pub floatval: __BindgenUnionField<core::ffi::c_float>,
	pub stringval: __BindgenUnionField<*mut core::ffi::c_char>,
	pub arrayval: __BindgenUnionField<*mut core::ffi::c_void>,
	pub tableval: __BindgenUnionField<*mut core::ffi::c_void>,
	pub bindgen_union_field: u32,
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
			::core::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 0usize, 1u8)
                    as u32,
            )
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
			::core::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 1usize, 1u8)
                    as u32,
            )
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
			::core::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 2usize, 1u8)
                    as u32,
            )
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
			::core::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 4usize],
                >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 3usize, 29u8)
                    as u32,
            )
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
#[must_use]
pub struct PlaydateJson {
	/**
	<code class="title">void playdate-&gt;json-&gt;initEncoder(json_encoder* encoder, writeFunc* write, void* userdata, int pretty);</code>
	<div class="content">
	<div class="paragraph">
	<p>Populates the given json_encoder <em>encoder</em> with the functions necessary to encode arbitrary data into a JSON string. <em>userdata</em> is passed as the first argument of the given writeFunc <em>write</em>. When <em>pretty</em> is 1 the string is written with human-readable formatting.</p>
	</div>
	</div>
	*/
	pub initEncoder: unsafe extern "C" fn(encoder: *mut JsonEncoder,
	                                      write: JsonWriteFunc,
	                                      userdata: *mut core::ffi::c_void,
	                                      pretty: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;json-&gt;decode(struct json_decoder* decoder, json_reader reader, json_value* outval);</code>
	<div class="content">
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-json.decode"><code>playdate.json.decode()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub decode: unsafe extern "C" fn(functions: *mut JsonDecoder,
	                                 reader: JsonReader,
	                                 outval: *mut JsonValue) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;json-&gt;decodeString(struct json_decoder* decoder, const char* jsonString, json_value* outval);</code>
	<div class="content">
	<div class="paragraph">
	<p>Decodes a JSON file or string with the given <em>decoder</em>. An instance of json_decoder must implement <em>decodeError</em>. The remaining functions are optional although you’ll probably want to implement at least <em>didDecodeTableValue</em> and <em>didDecodeArrayValue</em>. The <em>outval</em> pointer, if set, contains the value retured from the top-level <em>didDecodeSublist</em> callback.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub struct FileOptions(pub u8);
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, PartialEq)]
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
#[must_use]
pub struct PlaydateFile {
    /**
<code class="title">const char* playdate-&gt;file-&gt;geterr(void);</code>
<div class="content">
<div class="paragraph">
<p>Returns human-readable text describing the most recent error (usually indicated by a -1 return from a filesystem function).</p>
</div>
</div>
*/
    pub geterr: unsafe extern "C" fn() -> *const core::ffi::c_char,
    /**
<code class="title">int playdate-&gt;file-&gt;listfiles(const char* path, void (*callback)(const char* filename, void* userdata), void* userdata, int showhidden);</code>
<div class="content">
<div class="paragraph">
<p>Calls the given callback function for every file at <em>path</em>. Subfolders are indicated by a trailing slash '/' in <em>filename</em>. <em>listfiles()</em> does not recurse into subfolders. If <em>showhidden</em> is set, files beginning with a period will be included; otherwise, they are skipped. Returns 0 on success, -1 if no folder exists at <em>path</em> or it can’t be opened.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-file.listFiles"><code>playdate.file.listFiles()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub listfiles: unsafe extern "C" fn(
        path: *const core::ffi::c_char,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                path: *const core::ffi::c_char,
                userdata: *mut core::ffi::c_void,
            ),
        >,
        userdata: *mut core::ffi::c_void,
        showhidden: core::ffi::c_int,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;stat(const char* path, FileStat* stat);</code>
<div class="content">
<div class="paragraph">
<p>Populates the FileStat <em>stat</em> with information about the file at <em>path</em>. Returns 0 on success, or -1 in case of error.</p>
</div>
<div class="literalblock">
<div class="title">FileStat</div>
<div class="content">
<pre>typedef struct
{
	int isdir;
	unsigned int size;
	int m_year;
	int m_month;
	int m_day;
	int m_hour;
	int m_minute;
	int m_second;
} FileStat;</pre>
</div>
</div>
</div>
*/
    pub stat: unsafe extern "C" fn(
        path: *const core::ffi::c_char,
        stat: *mut FileStat,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;mkdir(const char* path);</code>
<div class="content">
<div class="paragraph">
<p>Creates the given <em>path</em> in the Data/&lt;gameid&gt; folder. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-file.mkdir"><code>playdate.file.mkdir()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub mkdir: unsafe extern "C" fn(path: *const core::ffi::c_char) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;unlink(const char* path, int recursive);</code>
<div class="content">
<div class="paragraph">
<p>Deletes the file at <em>path</em>. Returns 0 on success, or -1 in case of error. If recursive is 1 and the target path is a folder, this deletes everything inside the folder (including folders, folders inside those, and so on) as well as the folder itself.</p>
</div>
</div>
*/
    pub unlink: unsafe extern "C" fn(
        name: *const core::ffi::c_char,
        recursive: core::ffi::c_int,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;rename(const char* from, const char* to);</code>
<div class="content">
<div class="paragraph">
<p>Renames the file at <em>from</em> to <em>to</em>. It will overwrite the file at <em>to</em> without confirmation. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-file.rename"><code>playdate.file.rename()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub rename: unsafe extern "C" fn(
        from: *const core::ffi::c_char,
        to: *const core::ffi::c_char,
    ) -> core::ffi::c_int,
    /**
<code class="title">SDFile* playdate-&gt;file-&gt;open(const char* path, FileOptions mode);</code>
<div class="content">
<div class="paragraph">
<p>Opens a handle for the file at <em>path</em>. The <em>kFileRead</em> mode opens a file in the game pdx, while <em>kFileReadData</em> searches the game’s data folder; to search the data folder first then fall back on the game pdx, use the bitwise combination <em>kFileRead|kFileReadData</em>.<em>kFileWrite</em> and <em>kFileAppend</em> always write to the data folder. The function returns NULL if a file at <em>path</em> cannot be opened, and <a href="#f-file.geterr">playdate-&gt;file-&gt;geterr()</a> will describe the error. The filesystem has a limit of 64 simultaneous open files. The returned file handle should be <a href="#f-file.close">closed</a>, not freed, when it is no longer in use.</p>
</div>
<div class="literalblock">
<div class="title">FileOptions</div>
<div class="content">
<pre>typedef enum
{
	kFileRead,
	kFileReadData,
	kFileWrite,
	kFileAppend
} FileOptions;</pre>
</div>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-file.open"><code>playdate.file.open()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub open: unsafe extern "C" fn(
        name: *const core::ffi::c_char,
        mode: FileOptions,
    ) -> *mut SdFile,
    /**
<code class="title">int playdate-&gt;file-&gt;close(SDFile* file);</code>
<div class="content">
<div class="paragraph">
<p>Closes the given <em>file</em> handle. Returns 0 on success, or -1 in case of error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-file.close"><code>playdate.file.close()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub close: unsafe extern "C" fn(file: *mut SdFile) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;read(SDFile* file, void* buf, unsigned int len);</code>
<div class="content">
<div class="paragraph">
<p>Reads up to <em>len</em> bytes from the <em>file</em> into the buffer <em>buf</em>. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#m-file.read"><code>playdate.file.file:read()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub read: unsafe extern "C" fn(
        file: *mut SdFile,
        buf: *mut core::ffi::c_void,
        len: core::ffi::c_uint,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;write(SDFile* file, const void* buf, unsigned int len);</code>
<div class="content">
<div class="paragraph">
<p>Writes the buffer of bytes <em>buf</em> to the <em>file</em>. Returns the number of bytes written, or -1 in case of error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#m-file.write"><code>playdate.file.file:write()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub write: unsafe extern "C" fn(
        file: *mut SdFile,
        buf: *const core::ffi::c_void,
        len: core::ffi::c_uint,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;flush(SDFile* file);</code>
<div class="content">
<div class="paragraph">
<p>Flushes the output buffer of <em>file</em> immediately. Returns the number of bytes written, or -1 in case of error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-file.flush"><code>playdate.file.flush()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub flush: unsafe extern "C" fn(file: *mut SdFile) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;tell(SDFile* file);</code>
<div class="content">
<div class="paragraph">
<p>Returns the current read/write offset in the given <em>file</em> handle, or -1 on error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#m-file.tell"><code>playdate.file.file:tell()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub tell: unsafe extern "C" fn(file: *mut SdFile) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;file-&gt;seek(SDFile* file, int pos, int whence);</code>
<div class="content">
<div class="paragraph">
<p>Sets the read/write offset in the given <em>file</em> handle to <em>pos</em>, relative to the <em>whence</em> macro. SEEK_SET is relative to the beginning of the file, SEEK_CUR is relative to the current position of the file pointer, and SEEK_END is relative to the end of the file. Returns 0 on success, -1 on error.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#m-file.seek"><code>playdate.file.file:seek()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub seek: unsafe extern "C" fn(
        file: *mut SdFile,
        pos: core::ffi::c_int,
        whence: core::ffi::c_int,
    ) -> core::ffi::c_int,
}
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum SpriteCollisionResponseType {
	Slide = 0,
	Freeze = 1,
	Overlap = 2,
	Bounce = 3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
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
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, PartialEq)]
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
#[must_use]
pub struct PlaydateSprite {
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setAlwaysRedraw(int flag);</code>
	<div class="content">
	<div class="paragraph">
	<p>When <em>flag</em> is set to 1, this causes all sprites to draw each frame, whether or not they have been marked dirty. This may speed up the performance of your game if the system’s dirty rect tracking is taking up too much time - for example if there are many sprites moving around on screen at once.</p>
	</div>
	</div>
	*/
	pub setAlwaysRedraw: unsafe extern "C" fn(flag: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;addDirtyRect(LCDRect dirtyRect);</code>
	<div class="content">
	<div class="paragraph">
	<p>Marks the given <em>dirtyRect</em> (in screen coordinates) as needing a redraw. Graphics drawing functions now call this automatically, adding their drawn areas to the sprite’s dirty list, so there’s usually no need to call this manually.</p>
	</div>
	</div>
	*/
	pub addDirtyRect: unsafe extern "C" fn(dirtyRect: Aabb),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;drawSprites(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Draws every sprite in the display list.</p>
	</div>
	</div>
	*/
	pub drawSprites: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;updateAndDrawSprites(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Updates and draws every sprite in the display list.</p>
	</div>
	</div>
	*/
	pub updateAndDrawSprites: unsafe extern "C" fn(),
	/**
	<code class="title">LCDSprite* playdate-&gt;sprite-&gt;newSprite(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new LCDSprite.</p>
	</div>
	</div>
	*/
	pub newSprite: unsafe extern "C" fn() -> *mut Sprite,
	pub freeSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">LCDSprite* playdate-&gt;sprite-&gt;copy(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a copy of the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub copy: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut Sprite,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;addSprite(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds the given <em>sprite</em> to the display list, so that it is drawn in the current scene.</p>
	</div>
	</div>
	*/
	pub addSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;removeSprite(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes the given <em>sprite</em> from the display list.</p>
	</div>
	</div>
	*/
	pub removeSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;removeSprites(LCDSprite **sprites, int count);</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes the given <em>count</em> sized array of <em>sprites</em> from the display list.</p>
	</div>
	</div>
	*/
	pub removeSprites: unsafe extern "C" fn(sprites: *mut *mut Sprite, count: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;removeAllSprites(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes all sprites from the display list.</p>
	</div>
	</div>
	*/
	pub removeAllSprites: unsafe extern "C" fn(),
	/**
	<code class="title">int playdate-&gt;sprite-&gt;getSpriteCount(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the total number of sprites in the display list.</p>
	</div>
	</div>
	*/
	pub getSpriteCount: unsafe extern "C" fn() -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setBounds(LCDSprite *sprite, PDRect bounds);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the bounds of the given <em>sprite</em> with <em>bounds</em>.</p>
	</div>
	</div>
	*/
	pub setBounds: unsafe extern "C" fn(sprite: *mut Sprite, bounds: Rect),
	/**
	<code class="title">PDRect playdate-&gt;sprite-&gt;getBounds(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the bounds of the given <em>sprite</em> as an PDRect;</p>
	</div>
	</div>
	*/
	pub getBounds: unsafe extern "C" fn(sprite: *mut Sprite) -> Rect,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;moveTo(LCDSprite *sprite, float x, float y);</code>
	<div class="content">
	<div class="paragraph">
	<p>Moves the given <em>sprite</em> to <em>x</em>, <em>y</em> and resets its bounds based on the bitmap dimensions and center.</p>
	</div>
	</div>
	*/
	pub moveTo: unsafe extern "C" fn(sprite: *mut Sprite, x: core::ffi::c_float, y: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;moveBy(LCDSprite *sprite, float dx, float dy);</code>
	<div class="content">
	<div class="paragraph">
	<p>Moves the given <em>sprite</em> to by offsetting its current position by <em>dx</em>, <em>dy</em>.</p>
	</div>
	</div>
	*/
	pub moveBy: unsafe extern "C" fn(sprite: *mut Sprite, dx: core::ffi::c_float, dy: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setImage(LCDSprite *sprite, LCDBitmap *image, LCDBitmapFlip flip);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the given <em>sprite</em>'s image to the given <em>bitmap</em>.</p>
	</div>
	</div>
	*/
	pub setImage: unsafe extern "C" fn(sprite: *mut Sprite, image: *mut Bitmap, flip: BitmapFlip),
	/**
	<code class="title">LCDBitmap* playdate-&gt;sprite-&gt;getImage(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the LCDBitmap currently assigned to the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub getImage: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut Bitmap,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setSize(LCDSprite *s, float width, float height);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the size. The size is used to set the sprite’s bounds when calling moveTo().</p>
	</div>
	</div>
	*/
	pub setSize: unsafe extern "C" fn(s: *mut Sprite, width: core::ffi::c_float, height: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setZIndex(LCDSprite *sprite, int16_t zIndex);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the Z order of the given <em>sprite</em>. Higher Z sprites are drawn on top of those with lower Z order.</p>
	</div>
	</div>
	*/
	pub setZIndex: unsafe extern "C" fn(sprite: *mut Sprite, zIndex: i16),
	/**
	<code class="title">int16_t playdate-&gt;sprite-&gt;getZIndex(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the Z index of the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub getZIndex: unsafe extern "C" fn(sprite: *mut Sprite) -> i16,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setDrawMode(LCDSprite *sprite, LCDBitmapDrawMode mode);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the mode for drawing the sprite’s bitmap.</p>
	</div>
	</div>
	*/
	pub setDrawMode: unsafe extern "C" fn(sprite: *mut Sprite, mode: BitmapDrawMode),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setImageFlip(LCDSprite *sprite, LCDBitmapFlip flip);</code>
	<div class="content">
	<div class="paragraph">
	<p>Flips the bitmap.</p>
	</div>
	</div>
	*/
	pub setImageFlip: unsafe extern "C" fn(sprite: *mut Sprite, flip: BitmapFlip),
	/**
	<code class="title">LCDBitmapFlip playdate-&gt;sprite-&gt;getImageFlip(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the flip setting of the sprite’s bitmap.</p>
	</div>
	</div>
	*/
	pub getImageFlip: unsafe extern "C" fn(sprite: *mut Sprite) -> BitmapFlip,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setStencil(LCDSprite *sprite, LCDBitmap* stencil);</code>
	<div class="content">
	<div class="paragraph">
	<p>Specifies a stencil image to be set on the frame buffer before the sprite is drawn.</p>
	</div>
	</div>
	*/
	pub setStencil: unsafe extern "C" fn(sprite: *mut Sprite, stencil: *mut Bitmap),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setClipRect(LCDSprite *sprite, LCDRect clipRect);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the clipping rectangle for sprite drawing.</p>
	</div>
	</div>
	*/
	pub setClipRect: unsafe extern "C" fn(sprite: *mut Sprite, clipRect: Aabb),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;clearClipRect(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the sprite’s clipping rectangle.</p>
	</div>
	</div>
	*/
	pub clearClipRect: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setClipRectsInRange(LCDRect clipRect, int startZ, int endZ);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the clipping rectangle for <em>all</em> sprites with a Z index within <em>startZ</em> and <em>endZ</em> inclusive.</p>
	</div>
	</div>
	*/
	pub setClipRectsInRange:
		unsafe extern "C" fn(clipRect: Aabb, startZ: core::ffi::c_int, endZ: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;clearClipRectsInRange(int startZ, int endZ);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the clipping rectangle for <em>all</em> sprites with a Z index within <em>startZ</em> and <em>endZ</em> inclusive.</p>
	</div>
	</div>
	*/
	pub clearClipRectsInRange: unsafe extern "C" fn(startZ: core::ffi::c_int, endZ: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setUpdatesEnabled(LCDSprite *sprite, int flag);</code>
	<div class="content">
	<div class="paragraph">
	<p>Set the updatesEnabled flag of the given <em>sprite</em> (determines whether the sprite has its update function called). One is true, 0 is false.</p>
	</div>
	</div>
	*/
	pub setUpdatesEnabled: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;sprite-&gt;updatesEnabled(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Get the updatesEnabled flag of the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub updatesEnabled: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setCollisionsEnabled(LCDSprite *sprite, int flag);</code>
	<div class="content">
	<div class="paragraph">
	<p>Set the collisionsEnabled flag of the given <em>sprite</em> (along with the collideRect, this determines whether the sprite participates in collisions). One is true, 0 is false. Set to 1 by default.</p>
	</div>
	</div>
	*/
	pub setCollisionsEnabled: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;sprite-&gt;collisionsEnabled(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Get the collisionsEnabled flag of the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub collisionsEnabled: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setVisible(LCDSprite *sprite, int flag);</code>
	<div class="content">
	<div class="paragraph">
	<p>Set the visible flag of the given <em>sprite</em> (determines whether the sprite has its draw function called). One is true, 0 is false.</p>
	</div>
	</div>
	*/
	pub setVisible: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;sprite-&gt;isVisible(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Get the visible flag of the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub isVisible: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setOpaque(LCDSprite *sprite, int flag);</code>
	<div class="content">
	<div class="paragraph">
	<p>Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.</p>
	</div>
	</div>
	*/
	pub setOpaque: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;markDirty(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Forces the given <em>sprite</em> to redraw.</p>
	</div>
	</div>
	*/
	pub markDirty: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setTag(LCDSprite *sprite, uint8_t tag);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the tag of the given <em>sprite</em>. This can be useful for identifying sprites or types of sprites when using the collision API.</p>
	</div>
	</div>
	*/
	pub setTag: unsafe extern "C" fn(sprite: *mut Sprite, tag: u8),
	/**
	<code class="title">uint8_t playdate-&gt;sprite-&gt;getTag(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the tag of the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub getTag: unsafe extern "C" fn(sprite: *mut Sprite) -> u8,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setIgnoresDrawOffset(LCDSprite *sprite, int flag);</code>
	<div class="content">
	<div class="paragraph">
	<p>When <em>flag</em> is set to 1, the <em>sprite</em> will draw in screen coordinates, ignoring the currently-set drawOffset.</p>
	</div>
	<div class="paragraph">
	<p>This only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space.</p>
	</div>
	</div>
	*/
	pub setIgnoresDrawOffset: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setUpdateFunction(LCDSprite *sprite, LCDSpriteUpdateFunction *func);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the update function for the given <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub setUpdateFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteUpdateFunction),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setDrawFunction(LCDSprite *sprite, LCDSpriteDrawFunction *func);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the draw function for the given <em>sprite</em>. Note that the callback is only called when the sprite is on screen and has a size specified via <a href="#f-sprite.setSize">playdate→sprite→setSize()</a> or <a href="#f-sprite.setBounds">playdate→sprite→setBounds()</a>.</p>
	</div>
	</div>
	*/
	pub setDrawFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteDrawFunction),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;getPosition(LCDSprite *sprite, float *x, float *y);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets <em>x</em> and <em>y</em> to the current position of <em>sprite</em>.</p>
	</div>
	</div>
	*/
	pub getPosition:
		unsafe extern "C" fn(sprite: *mut Sprite, x: *mut core::ffi::c_float, y: *mut core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;resetCollisionWorld(void);</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees and reallocates internal collision data, resetting everything to its default state.</p>
	</div>
	</div>
	*/
	pub resetCollisionWorld: unsafe extern "C" fn(),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setCollideRect(LCDSprite *sprite, PDRect collideRect);</code>
	<div class="content">
	<div class="paragraph">
	<p>Marks the area of the given <em>sprite</em>, relative to its bounds, to be checked for collisions with other sprites' collide rects.</p>
	</div>
	</div>
	*/
	pub setCollideRect: unsafe extern "C" fn(sprite: *mut Sprite, collideRect: Rect),
	/**
	<code class="title">PDRect playdate-&gt;sprite-&gt;getCollideRect(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the given <em>sprite</em>’s collide rect.</p>
	</div>
	</div>
	*/
	pub getCollideRect: unsafe extern "C" fn(sprite: *mut Sprite) -> Rect,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;clearCollideRect(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the given <em>sprite</em>’s collide rect.</p>
	</div>
	</div>
	*/
	pub clearCollideRect: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setCollisionResponseFunction(LCDSprite *sprite, LCDSpriteCollisionFilterProc *func);</code>
	<div class="content">
	<div class="paragraph">
	<p>Set a callback that returns a <a href="#_SpriteCollisionResponseType">SpriteCollisionResponseType</a> for a collision between <em>sprite</em> and <em>other</em>.</p>
	</div>
	<div class="literalblock">
	<div class="title">LCDSpriteCollisionFilterProc</div>
	<div class="content">
	<pre>typedef SpriteCollisionResponseType LCDSpriteCollisionFilterProc(LCDSprite* sprite, LCDSprite* other);</pre>
	</div>
	</div>
	</div>
	*/
	pub setCollisionResponseFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteCollisionFilterProc),
	/**
	<code class="title">SpriteCollisionInfo* playdate-&gt;sprite-&gt;checkCollisions(LCDSprite *sprite, float goalX, float goalY, float *actualX, float *actualY, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the same values as <a href="#f-sprite.moveWithCollisions">playdate-&gt;sprite-&gt;moveWithCollisions()</a> but does not actually move the sprite. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub checkCollisions: unsafe extern "C" fn(sprite: *mut Sprite,
	                                          goalX: core::ffi::c_float,
	                                          goalY: core::ffi::c_float,
	                                          actualX: *mut core::ffi::c_float,
	                                          actualY: *mut core::ffi::c_float,
	                                          len: *mut core::ffi::c_int)
	                                          -> *mut SpriteCollisionInfo,
	/**
	<code class="title">SpriteCollisionInfo* playdate-&gt;sprite-&gt;moveWithCollisions(LCDSprite *sprite, float goalX, float goalY, float *actualX, float *actualY, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Moves the given <em>sprite</em> towards <em>goalX</em>, <em>goalY</em> taking collisions into account and returns an array of SpriteCollisionInfo. <em>len</em> is set to the size of the array and <em>actualX</em>, <em>actualY</em> are set to the sprite’s position after collisions. If no collisions occurred, this will be the same as <em>goalX</em>, <em>goalY</em>. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub moveWithCollisions: unsafe extern "C" fn(sprite: *mut Sprite,
	                                             goalX: core::ffi::c_float,
	                                             goalY: core::ffi::c_float,
	                                             actualX: *mut core::ffi::c_float,
	                                             actualY: *mut core::ffi::c_float,
	                                             len: *mut core::ffi::c_int)
	                                             -> *mut SpriteCollisionInfo,
	/**
	<code class="title">LCDSprite** playdate-&gt;sprite-&gt;querySpritesAtPoint(float x, float y, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an array of all sprites with collision rects containing the point at <em>x</em>, <em>y</em>. <em>len</em> is set to the size of the array. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub querySpritesAtPoint: unsafe extern "C" fn(x: core::ffi::c_float,
	                                              y: core::ffi::c_float,
	                                              len: *mut core::ffi::c_int)
	                                              -> *mut *mut Sprite,
	/**
	<code class="title">LCDSprite** playdate-&gt;sprite-&gt;querySpritesInRect(float x, float y, float width, float height, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an array of all sprites with collision rects that intersect the <em>width</em> by <em>height</em> rect at <em>x</em>, <em>y</em>. <em>len</em> is set to the size of the array. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub querySpritesInRect: unsafe extern "C" fn(x: core::ffi::c_float,
	                                             y: core::ffi::c_float,
	                                             width: core::ffi::c_float,
	                                             height: core::ffi::c_float,
	                                             len: *mut core::ffi::c_int)
	                                             -> *mut *mut Sprite,
	/**
	<code class="title">LCDSprite** playdate-&gt;sprite-&gt;querySpritesAlongLine(float x1, float y1, float x2, float y2, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an array of all sprites with collision rects that intersect the line connecting <em>x1</em>, <em>y1</em> and  <em>x2</em>, <em>y2</em>. <em>len</em> is set to the size of the array. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub querySpritesAlongLine: unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                y1: core::ffi::c_float,
	                                                x2: core::ffi::c_float,
	                                                y2: core::ffi::c_float,
	                                                len: *mut core::ffi::c_int)
	                                                -> *mut *mut Sprite,
	/**
	<code class="title">SpriteQueryInfo* playdate-&gt;sprite-&gt;querySpriteInfoAlongLine(float x1, float y1, float x2, float y2, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an array of SpriteQueryInfo for all sprites with collision rects that intersect the line connecting <em>x1</em>, <em>y1</em> and  <em>x2</em>, <em>y2</em>. <em>len</em> is set to the size of the array. If you don’t need this information, use querySpritesAlongLine() as it will be faster. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub querySpriteInfoAlongLine: unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                   y1: core::ffi::c_float,
	                                                   x2: core::ffi::c_float,
	                                                   y2: core::ffi::c_float,
	                                                   len: *mut core::ffi::c_int)
	                                                   -> *mut SpriteQueryInfo,
	/**
	<code class="title">LCDSprite** playdate-&gt;sprite-&gt;overlappingSprites(LCDSprite *sprite, int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an array of sprites that have collide rects that are currently overlapping the given <em>sprite</em>’s collide rect. <em>len</em> is set to the size of the array. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub overlappingSprites:
		unsafe extern "C" fn(sprite: *mut Sprite, len: *mut core::ffi::c_int) -> *mut *mut Sprite,
	/**
	<code class="title">LCDSprite** playdate-&gt;sprite-&gt;allOverlappingSprites(int *len);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns an array of all sprites that have collide rects that are currently overlapping. Each consecutive pair of sprites is overlapping (eg. 0 &amp; 1 overlap, 2 &amp; 3 overlap, etc). <em>len</em> is set to the size of the array. The caller is responsible for freeing the returned array.</p>
	</div>
	</div>
	*/
	pub allOverlappingSprites: unsafe extern "C" fn(len: *mut core::ffi::c_int) -> *mut *mut Sprite,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setStencilPattern(LCDSprite* sprite, uint8_t pattern[8]);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the sprite’s stencil to the given pattern.</p>
	</div>
	</div>
	*/
	pub setStencilPattern: unsafe extern "C" fn(sprite: *mut Sprite, pattern: *mut [u8; 8usize]),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;clearStencil(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the sprite’s stencil.</p>
	</div>
	</div>
	*/
	pub clearStencil: unsafe extern "C" fn(sprite: *mut Sprite),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setUserdata(LCDSprite *sprite, void* userdata);</code>
	<div class="content">

	</div>
	*/
	pub setUserdata: unsafe extern "C" fn(sprite: *mut Sprite, userdata: *mut core::ffi::c_void),
	/**
	<code class="title">void* playdate-&gt;sprite-&gt;getUserdata(LCDSprite *sprite);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets and gets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.</p>
	</div>
	</div>
	*/
	pub getUserdata: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut core::ffi::c_void,
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setStencilImage(LCDSprite *sprite, LCDBitmap* stencil, int tile);</code>
	<div class="content">
	<div class="paragraph">
	<p>Specifies a stencil image to be set on the frame buffer before the sprite is drawn. If <em>tile</em> is set, the stencil will be tiled. Tiled stencils must have width evenly divisible by 32.</p>
	</div>
	</div>
	*/
	pub setStencilImage: unsafe extern "C" fn(sprite: *mut Sprite, stencil: *mut Bitmap, tile: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;setCenter(LCDSprite *sprite, float x, float y);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width. Default is 0.5, 0.5 (the center of the sprite). This means that when you call <a href="#f-sprite.moveTo">sprite→moveTo(sprite, x, y)</a>, the center of your sprite will be positioned at <em>x</em>, <em>y</em>. If you want x and y to represent the upper left corner of your sprite, specify the center as 0, 0.</p>
	</div>
	</div>
	*/
	pub setCenter: unsafe extern "C" fn(s: *mut Sprite, x: core::ffi::c_float, y: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sprite-&gt;getCenter(LCDSprite *sprite, float *outx, float *outy);</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the values in <code>outx</code> and <code>outy</code> to the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
#[must_use]
pub struct SoundSource {
	_unused: [u8; 0],
}
pub type SndCallbackProc =
	::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[must_use]
pub struct PlaydateSoundSource {
	/**
	<code class="title">void playdate-&gt;sound-&gt;source-&gt;setVolume(SoundSource* c, float lvol, float rvol)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the playback volume (0.0 - 1.0) for left and right channels of the source.</p>
	</div>
	</div>
	*/
	pub setVolume: unsafe extern "C" fn(c: *mut SoundSource, lvol: core::ffi::c_float, rvol: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;source-&gt;getVolume(SoundSource* c, float* outlvol, float* outrvol)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the playback volume (0.0 - 1.0) for left and right channels of the source.</p>
	</div>
	</div>
	*/
	pub getVolume:
		unsafe extern "C" fn(c: *mut SoundSource, outl: *mut core::ffi::c_float, outr: *mut core::ffi::c_float),
	/**
	<code class="title">int playdate-&gt;sound-&gt;source-&gt;isPlaying(SoundSource* c)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns 1 if the source is currently playing.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct FilePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundFileplayer {
    /**
<code class="title">FilePlayer* playdate-&gt;sound-&gt;fileplayer-&gt;newPlayer(void);</code>
<div class="content">
<div class="paragraph">
<p>Allocates a new FilePlayer.</p>
</div>
</div>
*/
    pub newPlayer: unsafe extern "C" fn() -> *mut FilePlayer,
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;freePlayer(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Frees the given <em>player</em>.</p>
</div>
</div>
*/
    pub freePlayer: unsafe extern "C" fn(player: *mut FilePlayer),
    /**
<code class="title">int playdate-&gt;sound-&gt;fileplayer-&gt;loadIntoPlayer(FilePlayer* player, const char* path);</code>
<div class="content">
<div class="paragraph">
<p>Prepares <em>player</em> to stream the file at <em>path</em>. Returns 1 if the file exists, otherwise 0.</p>
</div>
</div>
*/
    pub loadIntoPlayer: unsafe extern "C" fn(
        player: *mut FilePlayer,
        path: *const core::ffi::c_char,
    ) -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setBufferLength(FilePlayer* player, float bufferLen);</code>
<div class="content">
<div class="paragraph">
<p>Sets the buffer length of <em>player</em> to <em>bufferLen</em> seconds;</p>
</div>
</div>
*/
    pub setBufferLength: unsafe extern "C" fn(
        player: *mut FilePlayer,
        bufferLen: core::ffi::c_float,
    ),
    /**
<code class="title">int playdate-&gt;sound-&gt;fileplayer-&gt;play(FilePlayer* player, int repeat);</code>
<div class="content">
<div class="paragraph">
<p>Starts playing the file <em>player</em>. If <em>repeat</em> is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with <a href="#f-sound.fileplayer.stop">playdate-&gt;sound-&gt;fileplayer-&gt;stop()</a>. Returns 1 on success, 0 if buffer allocation failed.</p>
</div>
</div>
*/
    pub play: unsafe extern "C" fn(
        player: *mut FilePlayer,
        repeat: core::ffi::c_int,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;sound-&gt;fileplayer-&gt;isPlaying(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Returns one if <em>player</em> is playing, zero if not.</p>
</div>
</div>
*/
    pub isPlaying: unsafe extern "C" fn(player: *mut FilePlayer) -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;pause(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Pauses the file <em>player</em>.</p>
</div>
</div>
*/
    pub pause: unsafe extern "C" fn(player: *mut FilePlayer),
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;stop(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Stops playing the file.</p>
</div>
</div>
*/
    pub stop: unsafe extern "C" fn(player: *mut FilePlayer),
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setVolume(FilePlayer* player, float left, float right);</code>
<div class="content">
<div class="paragraph">
<p>Sets the playback volume for left and right channels of <em>player</em>.</p>
</div>
</div>
*/
    pub setVolume: unsafe extern "C" fn(
        player: *mut FilePlayer,
        left: core::ffi::c_float,
        right: core::ffi::c_float,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;getVolume(FilePlayer* player, float* outleft, float* outright);</code>
<div class="content">
<div class="paragraph">
<p>Gets the left and right channel playback volume for <em>player</em>.</p>
</div>
</div>
*/
    pub getVolume: unsafe extern "C" fn(
        player: *mut FilePlayer,
        left: *mut core::ffi::c_float,
        right: *mut core::ffi::c_float,
    ),
    /**
<code class="title">float playdate-&gt;sound-&gt;fileplayer-&gt;getLength(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Returns the length, in seconds, of the file loaded into <em>player</em>.</p>
</div>
</div>
*/
    pub getLength: unsafe extern "C" fn(player: *mut FilePlayer) -> core::ffi::c_float,
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setOffset(FilePlayer* player, float offset);</code>
<div class="content">
<div class="paragraph">
<p>Sets the current <em>offset</em> in seconds.</p>
</div>
</div>
*/
    pub setOffset: unsafe extern "C" fn(
        player: *mut FilePlayer,
        offset: core::ffi::c_float,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setRate(FilePlayer* player, float rate)</code>
<div class="content">
<div class="paragraph">
<p>Sets the playback <em>rate</em> for the <em>player</em>. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate &lt; 0).</p>
</div>
</div>
*/
    pub setRate: unsafe extern "C" fn(player: *mut FilePlayer, rate: core::ffi::c_float),
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setLoopRange(FilePlayer* player, float start, float end);</code>
<div class="content">
<div class="paragraph">
<p>Sets the <em>start</em> and <em>end</em> of the loop region for playback, in seconds. If <em>end</em> is omitted, the end of the file is used.</p>
</div>
</div>
*/
    pub setLoopRange: unsafe extern "C" fn(
        player: *mut FilePlayer,
        start: core::ffi::c_float,
        end: core::ffi::c_float,
    ),
    /**
<code class="title">int playdate-&gt;sound-&gt;fileplayer-&gt;didUnderrun(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Returns one if <em>player</em> has underrun, zero if not.</p>
</div>
</div>
*/
    pub didUnderrun: unsafe extern "C" fn(player: *mut FilePlayer) -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setFinishCallback(FilePlayer* player, sndCallbackProc callback, void* userdata);</code>
<div class="content">
<div class="paragraph">
<p>Sets a function to be called when playback has completed. This is an alias for <a href="#f-sound.source.setFinishCallback">playdate→sound→source→setFinishCallback()</a>.</p>
</div>
<div id="_sndCallbackProc" class="literalblock">
<div class="title">sndCallbackProc</div>
<div class="content">
<pre>typedef void sndCallbackProc(SoundSource* c, void* userdata);</pre>
</div>
</div>
</div>
*/
    pub setFinishCallback: unsafe extern "C" fn(
        player: *mut FilePlayer,
        callback: SndCallbackProc,
        userdata: *mut core::ffi::c_void,
    ),
    pub setLoopCallback: unsafe extern "C" fn(
        player: *mut FilePlayer,
        callback: SndCallbackProc,
        userdata: *mut core::ffi::c_void,
    ),
    /**
<code class="title">float playdate-&gt;sound-&gt;fileplayer-&gt;getOffset(FilePlayer* player);</code>
<div class="content">
<div class="paragraph">
<p>Returns the current offset in seconds for <em>player</em>.</p>
</div>
</div>
*/
    pub getOffset: unsafe extern "C" fn(player: *mut FilePlayer) -> core::ffi::c_float,
    /**
<code class="title">float playdate-&gt;sound-&gt;fileplayer-&gt;getRate(FilePlayer* player)</code>
<div class="content">
<div class="paragraph">
<p>Returns the playback rate for <em>player</em>.</p>
</div>
</div>
*/
    pub getRate: unsafe extern "C" fn(player: *mut FilePlayer) -> core::ffi::c_float,
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;setStopOnUnderrun(FilePlayer* player, int flag)</code>
<div class="content">
<div class="paragraph">
<p>If <em>flag</em> evaluates to true, the <em>player</em> will restart playback (after an audible stutter) as soon as data is available.</p>
</div>
</div>
*/
    pub setStopOnUnderrun: unsafe extern "C" fn(
        player: *mut FilePlayer,
        flag: core::ffi::c_int,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;fileplayer-&gt;fadeVolume(FilePlayer* player, float left, float right, int32_t len, sndCallbackProc finishCallback, void* userdata);</code>
<div class="content">
<div class="paragraph">
<p>Changes the volume of the fileplayer to <em>left</em> and <em>right</em> over a length of <em>len</em> sample frames, then calls the provided callback (if set).</p>
</div>
</div>
*/
    pub fadeVolume: unsafe extern "C" fn(
        player: *mut FilePlayer,
        left: core::ffi::c_float,
        right: core::ffi::c_float,
        len: i32,
        finishCallback: SndCallbackProc,
        userdata: *mut core::ffi::c_void,
    ),
    pub setMP3StreamSource: unsafe extern "C" fn(
        player: *mut FilePlayer,
        dataSource: ::core::option::Option<
            unsafe extern "C" fn(
                data: *mut u8,
                bytes: core::ffi::c_int,
                userdata: *mut core::ffi::c_void,
            ) -> core::ffi::c_int,
        >,
        userdata: *mut core::ffi::c_void,
        bufferLen: core::ffi::c_float,
    ),
}
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
#[must_use]
pub struct AudioSample {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct SamplePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundSample {
	/**
	<code class="title">AudioSample* playdate-&gt;sound-&gt;sample-&gt;newSampleBuffer(int length)</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new AudioSample with a buffer large enough to load a file of <em>length</em> bytes.</p>
	</div>
	</div>
	*/
	pub newSampleBuffer: unsafe extern "C" fn(byteCount: core::ffi::c_int) -> *mut AudioSample,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sample-&gt;loadIntoSample(AudioSample* sample, const char* path)</code>
	<div class="content">
	<div class="paragraph">
	<p>Loads the sound data from the file at <em>path</em> into an existing AudioSample, <em>sample</em>.</p>
	</div>
	</div>
	*/
	pub loadIntoSample:
		unsafe extern "C" fn(sample: *mut AudioSample, path: *const core::ffi::c_char) -> core::ffi::c_int,
	/**
	<code class="title">AudioSample* playdate-&gt;sound-&gt;sample-&gt;load(const char* path)</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new AudioSample, with the sound data loaded in memory. If there is no file at <em>path</em>, the function returns null.</p>
	</div>
	</div>
	*/
	pub load: unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut AudioSample,
	/**
	<code class="title">AudioSample* playdate-&gt;sound-&gt;sample-&gt;newSampleFromData(uint8_t* data, SoundFormat format, uint32_t sampleRate, int byteCount, int shouldFreeData)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new AudioSample referencing the given audio data. If <em>shouldFreeData</em> is set, <em>data</em> is freed when the sample object is <a href="#f-sound.sample.freeSample">freed</a>. The sample keeps a pointer to the data instead of copying it, so the data must remain valid while the sample is active. <em>format</em> is one of the following values:</p>
	</div>
	<div class="literalblock">
	<div class="title">SoundFormat</div>
	<div class="content">
	<pre>typedef enum
	{
		kSound8bitMono = 0,
		kSound8bitStereo = 1,
		kSound16bitMono = 2,
		kSound16bitStereo = 3,
		kSoundADPCMMono = 4,
		kSoundADPCMStereo = 5
	} SoundFormat;</pre>
	</div>
	</div>
	<div class="paragraph">
	<p><code>pd_api_sound.h</code> also provides some helper macros and functions:</p>
	</div>
	<div class="listingblock">
	<div class="content">
	<pre class="CodeRay highlight"><code data-lang="c"><span class="preprocessor">#define</span> SoundFormatIsStereo(f) ((f)&amp;<span class="integer">1</span>)
	<span class="preprocessor">#define</span> SoundFormatIs16bit(f) ((f)&gt;=kSound16bitMono)
	<span class="directive">static</span> <span class="directive">inline</span> uint32_t SoundFormat_bytesPerFrame(SoundFormat fmt);</code></pre>
	</div>
	</div>
	</div>
	*/
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
	/**
	<code class="title">void playdate-&gt;sound-&gt;sample-&gt;freeSample(AudioSample* sample)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given <em>sample</em>. If the sample was created with <a href="#f-sound.sample.newSampleFromData">playdate→sound→sample→newSampleFromData()</a> and the <em>shouldFreeData</em> flag was set, the sample’s source data is also freed.</p>
	</div>
	</div>
	*/
	pub freeSample: unsafe extern "C" fn(sample: *mut AudioSample),
	/**
	<code class="title">float playdate-&gt;sound-&gt;sample-&gt;getLength(AudioSample* sample)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the length, in seconds, of <em>sample</em>.</p>
	</div>
	</div>
	*/
	pub getLength: unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_float,
	/**
	<code class="title">int playdate-&gt;sound-&gt;sample-&gt;decompress(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>If the sample is ADPCM compressed, decompresses the sample data to 16-bit PCM data. This increases the sample’s memory footprint by 4x and does not affect the quality in any way, but it is necessary if you want to use the sample in a synth or play the file backwards. Returns 1 if successful, 0 if there’s not enough memory for the uncompressed data.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct PlaydateSoundSampleplayer {
	/**
	<code class="title">SamplePlayer* playdate-&gt;sound-&gt;sampleplayer-&gt;newPlayer(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Allocates and returns a new SamplePlayer.</p>
	</div>
	</div>
	*/
	pub newPlayer: unsafe extern "C" fn() -> *mut SamplePlayer,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;freePlayer(SamplePlayer* player)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given <em>player</em>.</p>
	</div>
	</div>
	*/
	pub freePlayer: unsafe extern "C" fn(player: *mut SamplePlayer),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setSample(SamplePlayer* player, AudioSample* sample)</code>
	<div class="content">
	<div class="paragraph">
	<p>Assigns <em>sample</em> to <em>player</em>.</p>
	</div>
	</div>
	*/
	pub setSample: unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample),
	/**
	<code class="title">int playdate-&gt;sound-&gt;sampleplayer-&gt;play(SamplePlayer* player, int repeat, float rate)</code>
	<div class="content">
	<div class="paragraph">
	<p>Starts playing the sample in <em>player</em>.</p>
	</div>
	<div class="paragraph">
	<p>If <em>repeat</em> is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with <a href="#f-sound.sampleplayer.stop">playdate-&gt;sound-&gt;sampleplayer-&gt;stop()</a>. If negative one, it does ping-pong looping.</p>
	</div>
	<div class="paragraph">
	<p><em>rate</em> is the playback rate for the sample; 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.</p>
	</div>
	<div class="paragraph">
	<p>Returns 1 on success (which is always, currently).</p>
	</div>
	</div>
	*/
	pub play: unsafe extern "C" fn(player: *mut SamplePlayer,
	                               repeat: core::ffi::c_int,
	                               rate: core::ffi::c_float) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;sound-&gt;sampleplayer-&gt;isPlaying(SamplePlayer* player)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns one if <em>player</em> is playing a sample, zero if not.</p>
	</div>
	</div>
	*/
	pub isPlaying: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;stop(SamplePlayer* player)</code>
	<div class="content">
	<div class="paragraph">
	<p>Stops playing the sample.</p>
	</div>
	</div>
	*/
	pub stop: unsafe extern "C" fn(player: *mut SamplePlayer),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setVolume(SamplePlayer* player, float left, float right)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the playback volume for left and right channels.</p>
	</div>
	</div>
	*/
	pub setVolume:
		unsafe extern "C" fn(player: *mut SamplePlayer, left: core::ffi::c_float, right: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;getVolume(SamplePlayer* player, float* outleft, float* outright)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the current left and right channel volume of the sampleplayer.</p>
	</div>
	</div>
	*/
	pub getVolume: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                    left: *mut core::ffi::c_float,
	                                    right: *mut core::ffi::c_float),
	/**
	<code class="title">float playdate-&gt;sound-&gt;sampleplayer-&gt;getLength(SamplePlayer* player)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the length, in seconds, of the sample assigned to <em>player</em>.</p>
	</div>
	</div>
	*/
	pub getLength: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setOffset(SamplePlayer* player, float offset)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the current <em>offset</em> of the SamplePlayer, in seconds.</p>
	</div>
	</div>
	*/
	pub setOffset: unsafe extern "C" fn(player: *mut SamplePlayer, offset: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setRate(SamplePlayer* player, float rate)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the playback <em>rate</em> for the <em>player</em>. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.</p>
	</div>
	</div>
	*/
	pub setRate: unsafe extern "C" fn(player: *mut SamplePlayer, rate: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setPlayRange(SamplePlayer* player, int start, int end)</code>
	<div class="content">
	<div class="paragraph">
	<p>When used with a repeat of -1, does ping-pong looping, with a <em>start</em> and <em>end</em> position in frames.</p>
	</div>
	</div>
	*/
	pub setPlayRange:
		unsafe extern "C" fn(player: *mut SamplePlayer, start: core::ffi::c_int, end: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setFinishCallback(SamplePlayer* player, sndCallbackProc callback, void* userdata)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a function to be called when playback has completed. See <a href="#_sndCallbackProc">sndCallbackProc</a>.</p>
	</div>
	</div>
	*/
	pub setFinishCallback: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                            callback: SndCallbackProc,
	                                            userdata: *mut core::ffi::c_void),
	pub setLoopCallback: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                          callback: SndCallbackProc,
	                                          userdata: *mut core::ffi::c_void),
	/**
	<code class="title">float playdate-&gt;sound-&gt;sampleplayer-&gt;getOffset(SamplePlayer* player);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the current offset in seconds for <em>player</em>.</p>
	</div>
	</div>
	*/
	pub getOffset: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	/**
	<code class="title">float playdate-&gt;sound-&gt;sampleplayer-&gt;getRate(SamplePlayer* player)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the playback rate for <em>player</em>.</p>
	</div>
	</div>
	*/
	pub getRate: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sampleplayer-&gt;setPaused(SamplePlayer* player, int paused)</code>
	<div class="content">
	<div class="paragraph">
	<p>Pauses or resumes playback.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct SynthSignalValue {
	_unused: [u8; 0],
}
#[repr(C)]
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
#[must_use]
pub struct PlaydateSoundSignal {
	/**
	<code class="title">PDSynthSignal* playdate-&gt;sound-&gt;signal-&gt;newSignal(signalStepFunc step, signalNoteOnFunc noteOn, signalNoteOffFunc noteOff, signalDeallocFunc dealloc, void* userdata)</code>
	<div class="content">
	<div class="literalblock">
	<div class="title">SignalCallbacks</div>
	<div class="content">
	<pre>typedef float (*signalStepFunc)(void* userdata, int* iosamples, float* ifval);
	typedef void (*signalNoteOnFunc)(void* userdata, MIDINote note, float vel, float len); // len = -1 for indefinite
	typedef void (*signalNoteOffFunc)(void* userdata, int stopped, int offset); // stopped = 0 on note release, = 1 when note actually stops playing; offset is # of frames into the current cycle
	typedef void (*signalDeallocFunc)(void* userdata);</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>Provides a custom implementation for the signal. <em>signalStepFunc step</em> is the only required function, returning the value at the end of the current frame. When called, the <em>ioframes</em> pointer contains the number of samples until the end of the frame. If the signal needs to provide a value in the middle of the frame (e.g. an LFO that needs to be sample-accurate) it should return the "interframe" value in <em>ifval</em> and set <em>iosamples</em> to the sample offset of the value. The functions are called on the audio render thread, so they should return as quickly as possible.</p>
	</div>
	</div>
	*/
	pub newSignal: unsafe extern "C" fn(step: SignalStepFunc,
	                                    noteOn: SignalNoteOnFunc,
	                                    noteOff: SignalNoteOffFunc,
	                                    dealloc: SignalDeallocFunc,
	                                    userdata: *mut core::ffi::c_void)
	                                    -> *mut SynthSignal,
	/**
	<code class="title">void playdate-&gt;sound-&gt;signal-&gt;freeSignal(PDSynthSignal* signal);</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees a signal created with <em>playdate→sound→signal→newSignal()</em>.</p>
	</div>
	</div>
	*/
	pub freeSignal: unsafe extern "C" fn(signal: *mut SynthSignal),
	/**
	<code class="title">float playdate-&gt;sound-&gt;signal-&gt;getValue(PDSynthSignal* signal);</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the current output value of <em>signal</em>. The signal can be a custom signal created with newSignal(), or any of the PDSynthSignal subclasses.</p>
	</div>
	</div>
	*/
	pub getValue: unsafe extern "C" fn(signal: *mut SynthSignal) -> core::ffi::c_float,
	/**
	<code class="title">void playdate-&gt;sound-&gt;signal-&gt;setValueScale(PDSynthSignal* signal, float scale);</code>
	<div class="content">
	<div class="paragraph">
	<p>Scales the signal’s output by the given factor. The scale is applied before the offset.</p>
	</div>
	</div>
	*/
	pub setValueScale: unsafe extern "C" fn(signal: *mut SynthSignal, scale: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;signal-&gt;setValueOffset(PDSynthSignal* signal, float offset);</code>
	<div class="content">
	<div class="paragraph">
	<p>Offsets the signal’s output by the given amount.</p>
	</div>
	</div>
	*/
	pub setValueOffset: unsafe extern "C" fn(signal: *mut SynthSignal, offset: core::ffi::c_float),
	/**
	<code class="title">PDSynthSignal* playdate-&gt;sound-&gt;signal-&gt;newSignalForValue(PDSynthSignalValue* value)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new PDSynthSignal that tracks a PDSynthSignalValue.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
#[must_use]
pub struct SynthLfo {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundLfo {
    /**
<code class="title">PDSynthLFO* playdate-&gt;sound-&gt;lfo-&gt;newLFO(LFOType type)</code>
<div class="content">
<div class="paragraph">
<p>Returns a new LFO object, which can be used to modulate sounds. The <em>type</em> argument is one of the following values:</p>
</div>
<div class="literalblock">
<div class="title">LFOType</div>
<div class="content">
<pre>typedef enum
{
	kLFOTypeSquare,
	kLFOTypeTriangle,
	kLFOTypeSine,
	kLFOTypeSampleAndHold,
	kLFOTypeSawtoothUp,
	kLFOTypeSawtoothDown,
	kLFOTypeArpeggiator,
	kLFOTypeFunction
} LFOType;</pre>
</div>
</div>
</div>
*/
    pub newLFO: unsafe extern "C" fn(type_: LfoType) -> *mut SynthLfo,
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;freeLFO(PDSynthLFO* lfo)</code>
<div class="content">
<div class="paragraph">
<p>Frees the LFO.</p>
</div>
</div>
*/
    pub freeLFO: unsafe extern "C" fn(lfo: *mut SynthLfo),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setType(PDSynthLFO* lfo, LFOType type)</code>
<div class="content">
<div class="paragraph">
<p>Sets the LFO shape to one of the values given above.</p>
</div>
</div>
*/
    pub setType: unsafe extern "C" fn(lfo: *mut SynthLfo, type_: LfoType),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setRate(PDSynthLFO* lfo, float rate)</code>
<div class="content">
<div class="paragraph">
<p>Sets the LFO’s rate, in cycles per second.</p>
</div>
</div>
*/
    pub setRate: unsafe extern "C" fn(lfo: *mut SynthLfo, rate: core::ffi::c_float),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setPhase(PDSynthLFO* lfo, float phase)</code>
<div class="content">
<div class="paragraph">
<p>Sets the LFO’s phase, from 0 to 1.</p>
</div>
</div>
*/
    pub setPhase: unsafe extern "C" fn(lfo: *mut SynthLfo, phase: core::ffi::c_float),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setCenter(PDSynthLFO* lfo, float center)</code>
<div class="content">
<div class="paragraph">
<p>Sets the center value for the LFO.</p>
</div>
</div>
*/
    pub setCenter: unsafe extern "C" fn(lfo: *mut SynthLfo, center: core::ffi::c_float),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setDepth(PDSynthLFO* lfo, float depth)</code>
<div class="content">
<div class="paragraph">
<p>Sets the depth of the LFO.</p>
</div>
</div>
*/
    pub setDepth: unsafe extern "C" fn(lfo: *mut SynthLfo, depth: core::ffi::c_float),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setArpeggiation(PDSynthLFO* lfo, int nSteps, float* steps)</code>
<div class="content">
<div class="paragraph">
<p>Sets the LFO type to arpeggio, where the given values are in half-steps from the center note. For example, the sequence (0, 4, 7, 12) plays the notes of a major chord.</p>
</div>
</div>
*/
    pub setArpeggiation: unsafe extern "C" fn(
        lfo: *mut SynthLfo,
        nSteps: core::ffi::c_int,
        steps: *mut core::ffi::c_float,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setFunction(PDSynthLFO* lfo, float (*lfoFunc)(PDSynthLFO* lfo, void* userdata), void* userdata, int interpolate)</code>
<div class="content">
<div class="paragraph">
<p>Provides a custom function for LFO values.</p>
</div>
</div>
*/
    pub setFunction: unsafe extern "C" fn(
        lfo: *mut SynthLfo,
        lfoFunc: ::core::option::Option<
            unsafe extern "C" fn(
                lfo: *mut SynthLfo,
                userdata: *mut core::ffi::c_void,
            ) -> core::ffi::c_float,
        >,
        userdata: *mut core::ffi::c_void,
        interpolate: core::ffi::c_int,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setDelay(PDSynthLFO* lfo, float holdoff, float ramptime)</code>
<div class="content">
<div class="paragraph">
<p>Sets an initial holdoff time for the LFO where the LFO remains at its center value, and a ramp time where the value increases linearly to its maximum depth. Values are in seconds.</p>
</div>
</div>
*/
    pub setDelay: unsafe extern "C" fn(
        lfo: *mut SynthLfo,
        holdoff: core::ffi::c_float,
        ramptime: core::ffi::c_float,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setRetrigger(PDSynthLFO* lfo, int flag)</code>
<div class="content">
<div class="paragraph">
<p>If retrigger is on, the LFO’s phase is reset to its initial phase (default 0) when a synth using the LFO starts playing a note.</p>
</div>
</div>
*/
    pub setRetrigger: unsafe extern "C" fn(lfo: *mut SynthLfo, flag: core::ffi::c_int),
    /**
<code class="title">float playdate-&gt;sound-&gt;lfo-&gt;getValue(PDSynthLFO* lfo)</code>
<div class="content">
<div class="paragraph">
<p>Return the current output value of the LFO.</p>
</div>
</div>
*/
    pub getValue: unsafe extern "C" fn(lfo: *mut SynthLfo) -> core::ffi::c_float,
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setGlobal(PDSynthLFO* lfo, int global)</code>
<div class="content">
<div class="paragraph">
<p>If <em>global</em> is set, the LFO is continuously updated whether or not it’s currently in use.</p>
</div>
</div>
*/
    pub setGlobal: unsafe extern "C" fn(lfo: *mut SynthLfo, global: core::ffi::c_int),
    /**
<code class="title">void playdate-&gt;sound-&gt;lfo-&gt;setStartPhase(PDSynthLFO* lfo, float phase)</code>
<div class="content">
<div class="paragraph">
<p>Sets the LFO’s initial phase, from 0 to 1.</p>
</div>
</div>
*/
    pub setStartPhase: unsafe extern "C" fn(
        lfo: *mut SynthLfo,
        phase: core::ffi::c_float,
    ),
}
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
#[must_use]
pub struct SynthEnvelope {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundEnvelope {
	/**
	<code class="title">PDSynthEnvelope* playdate-&gt;sound-&gt;envelope-&gt;newEnvelope(float attack, float decay, float sustain, float release)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new envelope with the given parameters.</p>
	</div>
	</div>
	*/
	pub newEnvelope: unsafe extern "C" fn(attack: core::ffi::c_float,
	                                      decay: core::ffi::c_float,
	                                      sustain: core::ffi::c_float,
	                                      release: core::ffi::c_float)
	                                      -> *mut SynthEnvelope,
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;freeEnvelope(PDSynthEnvelope* env)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the envelope.</p>
	</div>
	</div>
	*/
	pub freeEnvelope: unsafe extern "C" fn(env: *mut SynthEnvelope),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setAttack(PDSynthEnvelope* env, float attack)</code>
	<div class="content">

	</div>
	*/
	pub setAttack: unsafe extern "C" fn(env: *mut SynthEnvelope, attack: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setDecay(PDSynthEnvelope* env, float decay)</code>
	<div class="content">

	</div>
	*/
	pub setDecay: unsafe extern "C" fn(env: *mut SynthEnvelope, decay: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setSustain(PDSynthEnvelope* env, float sustain)</code>
	<div class="content">

	</div>
	*/
	pub setSustain: unsafe extern "C" fn(env: *mut SynthEnvelope, sustain: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setRelease(PDSynthEnvelope* env, float release)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the ADSR parameters of the envelope.</p>
	</div>
	</div>
	*/
	pub setRelease: unsafe extern "C" fn(env: *mut SynthEnvelope, release: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setLegato(PDSynthEnvelope* env, int flag)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets whether to use legato phrasing for the envelope. If the legato flag is set, when the envelope is re-triggered before it’s released, it remains in the sustain phase instead of jumping back to the attack phase.</p>
	</div>
	</div>
	*/
	pub setLegato: unsafe extern "C" fn(env: *mut SynthEnvelope, flag: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setRetrigger(PDSynthEnvelope* env, int flag)</code>
	<div class="content">
	<div class="paragraph">
	<p>If retrigger is on, the envelope always starts from 0 when a note starts playing, instead of the current value if it’s active.</p>
	</div>
	</div>
	*/
	pub setRetrigger: unsafe extern "C" fn(lfo: *mut SynthEnvelope, flag: core::ffi::c_int),
	/**
	<code class="title">float playdate-&gt;sound-&gt;envelope-&gt;getValue(PDSynthEnvelope* env)</code>
	<div class="content">
	<div class="paragraph">
	<p>Return the current output value of the envelope.</p>
	</div>
	</div>
	*/
	pub getValue: unsafe extern "C" fn(env: *mut SynthEnvelope) -> core::ffi::c_float,
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setCurvature(PDSynthEnvelope* env, float amount)</code>
	<div class="content">
	<div class="paragraph">
	<p>Smoothly changes the envelope’s shape from linear (amount=0) to exponential (amount=1).</p>
	</div>
	</div>
	*/
	pub setCurvature: unsafe extern "C" fn(env: *mut SynthEnvelope, amount: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setVelocitySensitivity(PDSynthEnvelope* env, float velsens)</code>
	<div class="content">
	<div class="paragraph">
	<p>Changes the amount by which note velocity scales output level. At the default value of 1, output is proportional to velocity; at 0 velocity has no effect on output level.</p>
	</div>
	</div>
	*/
	pub setVelocitySensitivity: unsafe extern "C" fn(env: *mut SynthEnvelope, velsens: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;envelope-&gt;setRateScaling(PDSynthEnvelope* env, float scaling, MIDINote start, MIDINote end)</code>
	<div class="content">
	<div class="paragraph">
	<p>Scales the envelope rate according to the played note. For notes below <code>start</code>, the envelope’s set rate is used; for notes above <code>end</code> envelope rates are scaled by the <code>scaling</code> parameter. Between the two notes the scaling factor is interpolated from 1.0 to <code>scaling</code>.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
#[must_use]
pub struct Synth {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundSynth {
	/**
	<code class="title">PDSynth* playdate-&gt;sound-&gt;synth-&gt;newSynth(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new synth object.</p>
	</div>
	</div>
	*/
	pub newSynth: unsafe extern "C" fn() -> *mut Synth,
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;freeSynth(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees a synth object, first removing it from the sound engine if needed.</p>
	</div>
	</div>
	*/
	pub freeSynth: unsafe extern "C" fn(synth: *mut Synth),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setWaveform(PDSynth* synth, SoundWaveform wave)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the waveform of the synth. The SoundWaveform enum contains the following values:</p>
	</div>
	<div class="literalblock">
	<div class="title">SoundWaveform</div>
	<div class="content">
	<pre>typedef enum
	{
		kWaveformSquare,
		kWaveformTriangle,
		kWaveformSine,
		kWaveformNoise,
		kWaveformSawtooth,
		kWaveformPOPhase,
		kWaveformPODigital,
		kWaveformPOVosim
	} SoundWaveform;</pre>
	</div>
	</div>
	</div>
	*/
	pub setWaveform: unsafe extern "C" fn(synth: *mut Synth, wave: SoundWaveform),
	pub setGenerator_deprecated: unsafe extern "C" fn(synth: *mut Synth,
	                                                  stereo: core::ffi::c_int,
	                                                  render: SynthRenderFunc,
	                                                  noteOn: SynthNoteOnFunc,
	                                                  release: SynthReleaseFunc,
	                                                  setparam: SynthSetParameterFunc,
	                                                  dealloc: SynthDeallocFunc,
	                                                  userdata: *mut core::ffi::c_void),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setSample(PDSynth* synth, AudioSample* sample, uint32_t sustainStart, uint32_t sustainEnd)</code>
	<div class="content">
	<div class="paragraph">
	<p>Provides a sample for the synth to play. Sample data must be uncompressed PCM, not ADPCM. If a sustain range is set, it is looped while the synth is playing a note. When the note ends, if an envelope has been set on the synth and the sustain range goes to the end of the sample (i.e. there’s no release section of the sample after the sustain range) then the sustain section continues looping during the envelope release; otherwise it plays through the end of the sample and stops. As a convenience, if <code>sustainEnd</code> is zero and <code>sustainStart</code> is greater than zero, <code>sustainEnd</code> will be set to the length of the sample.</p>
	</div>
	</div>
	*/
	pub setSample:
		unsafe extern "C" fn(synth: *mut Synth, sample: *mut AudioSample, sustainStart: u32, sustainEnd: u32),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setAttackTime(PDSynth* synth, float attack)</code>
	<div class="content">

	</div>
	*/
	pub setAttackTime: unsafe extern "C" fn(synth: *mut Synth, attack: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setDecayTime(PDSynth* synth, float decay)</code>
	<div class="content">

	</div>
	*/
	pub setDecayTime: unsafe extern "C" fn(synth: *mut Synth, decay: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setSustainLevel(PDSynth* synth, float sustain)</code>
	<div class="content">

	</div>
	*/
	pub setSustainLevel: unsafe extern "C" fn(synth: *mut Synth, sustain: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setReleaseTime(PDSynth* synth, float release)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the parameters of the synth’s ADSR envelope.</p>
	</div>
	</div>
	*/
	pub setReleaseTime: unsafe extern "C" fn(synth: *mut Synth, release: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setTranspose(PDSynth* synth, float halfSteps)</code>
	<div class="content">
	<div class="paragraph">
	<p>Transposes the synth’s output by the given number of half steps. For example, if the transpose is set to 2 and a C note is played, the synth will output a D instead.</p>
	</div>
	</div>
	*/
	pub setTranspose: unsafe extern "C" fn(synth: *mut Synth, halfSteps: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setFrequencyModulator(PDSynth* synth, PDSynthSignalValue* mod)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the synth’s frequency. The signal is scaled so that a value of 1 doubles the synth pitch (i.e. an octave up) and -1 halves it (an octave down). Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setFrequencyModulator: unsafe extern "C" fn(synth: *mut Synth, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;synth-&gt;getFrequencyModulator(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set frequency modulator.</p>
	</div>
	</div>
	*/
	pub getFrequencyModulator: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setAmplitudeModulator(PDSynth* synth, PDSynthSignalValue* mod)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the synth’s output amplitude. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setAmplitudeModulator: unsafe extern "C" fn(synth: *mut Synth, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;synth-&gt;getAmplitudeModulator(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set amplitude modulator.</p>
	</div>
	</div>
	*/
	pub getAmplitudeModulator: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthSignalValue,
	/**
	<code class="title">int playdate-&gt;sound-&gt;synth-&gt;getParameterCount(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the number of parameters advertised by the synth.</p>
	</div>
	</div>
	*/
	pub getParameterCount: unsafe extern "C" fn(synth: *mut Synth) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;sound-&gt;synth-&gt;setParameter(PDSynth* synth, int num, float value)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the (1-based) parameter at position <em>num</em> to the given value. Returns 0 if <em>num</em> is not a valid parameter index.</p>
	</div>
	</div>
	*/
	pub setParameter: unsafe extern "C" fn(synth: *mut Synth,
	                                       parameter: core::ffi::c_int,
	                                       value: core::ffi::c_float)
	                                       -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setParameterModulator(PDSynth* synth, int num, PDSynthSignalValue* mod)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the parameter at index <em>num</em>. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setParameterModulator:
		unsafe extern "C" fn(synth: *mut Synth, parameter: core::ffi::c_int, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;synth-&gt;getParameterModulator(PDSynth* synth, int num)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set parameter modulator for the given index.</p>
	</div>
	</div>
	*/
	pub getParameterModulator:
		unsafe extern "C" fn(synth: *mut Synth, parameter: core::ffi::c_int) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;playNote(PDSynth* synth, float freq, float vel, float len, uint32_t when)</code>
	<div class="content">
	<div class="paragraph">
	<p>Plays a note on the synth, at the given frequency. Specify <em>len</em> = -1 to leave the note playing until a subsequent noteOff() call. If <em>when</em> is 0, the note is played immediately, otherwise the note is scheduled for the given time. Use <a href="#f-sound.getCurrentTime">playdate→sound→getCurrentTime()</a> to get the current time.</p>
	</div>
	</div>
	*/
	pub playNote: unsafe extern "C" fn(synth: *mut Synth,
	                                   freq: core::ffi::c_float,
	                                   vel: core::ffi::c_float,
	                                   len: core::ffi::c_float,
	                                   when: u32),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;playMIDINote(PDSynth* synth, MIDINote note, float vel, float len, uint32_t when)</code>
	<div class="content">
	<div class="paragraph">
	<p>The same as <a href="#f-sound.synth.playNote">playNote</a> but uses MIDI note (where 60 = C4) instead of frequency. Note that <code>MIDINote</code> is a typedef for `float', meaning fractional values are allowed (for all you microtuning enthusiasts).</p>
	</div>
	</div>
	*/
	pub playMIDINote: unsafe extern "C" fn(synth: *mut Synth,
	                                       note: MidiNote,
	                                       vel: core::ffi::c_float,
	                                       len: core::ffi::c_float,
	                                       when: u32),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;noteOff(PDSynth* synth, uint32_t when)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sends a note off event to the synth, either immediately (<em>when</em> = 0) or at the scheduled time.</p>
	</div>
	</div>
	*/
	pub noteOff: unsafe extern "C" fn(synth: *mut Synth, when: u32),
	pub stop: unsafe extern "C" fn(synth: *mut Synth),
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setVolume(PDSynth* synth, float lvol, float rvol)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the playback volume (0.0 - 1.0) for the left and, if the synth is stereo, right channels of the synth. This is equivalent to</p>
	</div>
	<div class="literalblock">
	<div class="content">
	<pre>playdate-&gt;sound-&gt;source-&gt;setVolume((SoundSource*)synth, lvol, rvol);</pre>
	</div>
	</div>
	</div>
	*/
	pub setVolume: unsafe extern "C" fn(synth: *mut Synth, left: core::ffi::c_float, right: core::ffi::c_float),
	/**
	<code class="title">float playdate-&gt;sound-&gt;synth-&gt;getVolume(PDSynth* synth, float* outlvol, float* outrvol)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the playback volume for the left and right (if stereo) channels of the synth. This is equivalent to</p>
	</div>
	<div class="literalblock">
	<div class="content">
	<pre>playdate-&gt;sound-&gt;source-&gt;getVolume((SoundSource*)synth, outlvol, outrvol);</pre>
	</div>
	</div>
	</div>
	*/
	pub getVolume:
		unsafe extern "C" fn(synth: *mut Synth, left: *mut core::ffi::c_float, right: *mut core::ffi::c_float),
	/**
	<code class="title">int playdate-&gt;sound-&gt;synth-&gt;isPlaying(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns 1 if the synth is currently playing.</p>
	</div>
	</div>
	*/
	pub isPlaying: unsafe extern "C" fn(synth: *mut Synth) -> core::ffi::c_int,
	/**
	<code class="title">PDSynthEnvelope* playdate-&gt;sound-&gt;synth-&gt;getEnvelope(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the synth’s envelope. The PDSynth object owns this envelope, so it must not be freed.</p>
	</div>
	</div>
	*/
	pub getEnvelope: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthEnvelope,
	/**
	<code class="title">int playdate-&gt;sound-&gt;synth-&gt;setWavetable(PDSynth* synth, AudioSample* sample, int log2size, int columns, rows)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a wavetable for the synth to play. Sample data must be 16-bit mono uncompressed. <code>log2size</code> is the base 2 logarithm of the number of samples in each waveform "cell" in the table, and <code>columns</code> and <code>rows</code> gives the number of cells in each direction; for example, if the wavetable is arranged in an 8x8 grid, <code>columns</code> and <code>rows</code> are both 8 and <code>log2size</code> is 6, since 2^6 = 8x8.</p>
	</div>
	<div class="paragraph">
	<p>The function returns 1 on success. If it fails, use <a href="#f-sound.getError">playdate→sound→getError()</a> to get a human-readable error message.</p>
	</div>
	<div class="paragraph">
	<p>The synth’s "position" in the wavetable is set manually with <a href="#f-sound.synth.setParameter">setParameter()</a> or automated with <a href="#f-sound.synth.setParameterModulator">setParameterModulator()</a>. In some cases it’s easier to use a parameter that matches the waveform position in the table, in others (notably when using envelopes and lfos) it’s more convenient to use a 0-1 scale, so there’s some redundancy here. Parameters are</p>
	</div>
	<div class="ulist">
	<ul>
	<li>
	<p>1: x position, values are from 0 to the table width</p>
	</li>
	<li>
	<p>2: x position, values are from 0 to 1, parameter is scaled up to table width</p>
	</li>
	</ul>
	</div>
	<div class="paragraph">
	<p>For 2-D tables (<code>height</code> &gt; 1):</p>
	</div>
	<div class="ulist">
	<ul>
	<li>
	<p>3: y position, values are from 0 to the table height</p>
	</li>
	<li>
	<p>4: y position, values are from 0 to 1, parameter is scaled up to table height</p>
	</li>
	</ul>
	</div>
	</div>
	*/
	pub setWavetable: unsafe extern "C" fn(synth: *mut Synth,
	                                       sample: *mut AudioSample,
	                                       log2size: core::ffi::c_int,
	                                       columns: core::ffi::c_int,
	                                       rows: core::ffi::c_int)
	                                       -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;setGenerator(PDSynth* synth, int stereo, synthRenderFunc* render, synthNoteOnFunc* noteOn, synthReleaseFunc* release, synthSetParameterFunc* setparam, synthDeallocFunc* dealloc, synthCopyUserdataFunc copyUserdata, void* userdata)</code>
	<div class="content">
	<div class="literalblock">
	<div class="title">GeneratorCallbacks</div>
	<div class="content">
	<pre>typedef int (*synthRenderFunc)(void* userdata, int32_t* left, int32_t* right, int nsamples, uint32_t rate, int32_t drate);
	typedef void (*synthNoteOnFunc)(void* userdata, MIDINote note, float velocity, float len); // len == -1 if indefinite
	typedef void (*synthReleaseFunc)(void* userdata, int endoffset);
	typedef int (*synthSetParameterFunc)(void* userdata, int parameter, float value);
	typedef void (*synthDeallocFunc)(void* userdata);
	typedef void* (*synthCopyUserdata)(void* userdata);</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>Provides custom waveform generator functions for the synth. These functions are called on the audio render thread, so they should return as quickly as possible. <em>synthRenderFunc</em>, the data provider callback, is the only required function.</p>
	</div>
	<div class="paragraph">
	<p><em>synthRenderFunc</em>: called every audio cycle to get the samples for playback. <em>left</em> (and <em>right</em> if <em>setGenerator()</em> was called with the stereo flag set) are sample buffers in Q8.24 format. <em>rate</em> is the amount to change a (Q32) phase accumulator each sample, and <em>drate</em> is the amount to change <em>rate</em> each sample. Custom synths can ignore this and use the <em>note</em> paramter in the noteOn function to handle pitch, but synth→setFrequencyModulator() won’t work as expected.</p>
	</div>
	<div class="paragraph">
	<p><em>synthNoteOnFunc</em>: called when the synth receives a note on event. <em>len</em> is the length of the note in seconds, or -1 if it’s not known yet when the note will end.</p>
	</div>
	<div class="paragraph">
	<p><em>synthReleaseFunc</em>: called when the synth receives a note off event. <em>endoffset</em> is how many samples into the current render cycle the note ends, allowing for sample-accurate timing.</p>
	</div>
	<div class="paragraph">
	<p><em>synthSetParameterFunc</em>: called when a parameter change is received from <a href="#f-sound.synth.setParameter">synth→setParameter()</a> or a modulator.</p>
	</div>
	<div class="paragraph">
	<p><em>synthDeallocFunc</em>: called when the synth is being dealloced. This function should free anything that was allocated for the synth and also free the <em>userdata</em> itself.</p>
	</div>
	<div class="paragraph">
	<p><em>synthCopyUserdata</em>: called when <a href="#f-sound.synth.copy">synth→copy()</a> needs a unique copy of the synth’s userdata. External objects should be retained or copied so that the object isn’t freed while the synth is still using it.</p>
	</div>
	</div>
	*/
	pub setGenerator: unsafe extern "C" fn(synth: *mut Synth,
	                                       stereo: core::ffi::c_int,
	                                       render: SynthRenderFunc,
	                                       noteOn: SynthNoteOnFunc,
	                                       release: SynthReleaseFunc,
	                                       setparam: SynthSetParameterFunc,
	                                       dealloc: SynthDeallocFunc,
	                                       copyUserdata: SynthCopyUserdata,
	                                       userdata: *mut core::ffi::c_void),
	/**
	<code class="title">PDSynth* playdate-&gt;sound-&gt;synth-&gt;copy(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a copy of the given synth. Caller assumes ownership of the returned object and should free it when it is no longer in use.</p>
	</div>
	</div>
	*/
	pub copy: unsafe extern "C" fn(synth: *mut Synth) -> *mut Synth,
	/**
	<code class="title">void playdate-&gt;sound-&gt;synth-&gt;clearEnvelope(PDSynth* synth)</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears the synth’s envelope settings.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct ControlSignal {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateControlSignal {
	/**
	<code class="title">ControlSignal* playdate-&gt;sound-&gt;controlsignal-&gt;newSignal(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new control signal object.</p>
	</div>
	</div>
	*/
	pub newSignal: unsafe extern "C" fn() -> *mut ControlSignal,
	/**
	<code class="title">void playdate-&gt;sound-&gt;controlsignal-&gt;freeSignal(ControlSignal* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given signal.</p>
	</div>
	</div>
	*/
	pub freeSignal: unsafe extern "C" fn(signal: *mut ControlSignal),
	/**
	<code class="title">void playdate-&gt;sound-&gt;controlsignal-&gt;clearEvents(ControlSignal* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears all events from the given signal.</p>
	</div>
	</div>
	*/
	pub clearEvents: unsafe extern "C" fn(control: *mut ControlSignal),
	/**
	<code class="title">void playdate-&gt;sound-&gt;controlsignal-&gt;addEvent(ControlSignal* signal, int step, float value, int interpolate)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds a value to the signal’s timeline at the given step. If <em>interpolate</em> is set, the value is interpolated between the previous step+value and this one.</p>
	</div>
	</div>
	*/
	pub addEvent: unsafe extern "C" fn(control: *mut ControlSignal,
	                                   step: core::ffi::c_int,
	                                   value: core::ffi::c_float,
	                                   interpolate: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sound-&gt;controlsignal-&gt;removeEvent(ControlSignal* signal, int step)</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes the control event at the given step.</p>
	</div>
	</div>
	*/
	pub removeEvent: unsafe extern "C" fn(control: *mut ControlSignal, step: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;sound-&gt;controlsignal-&gt;getMIDIControllerNumber(ControlSignal* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the MIDI controller number for this ControlSignal, if it was created from a MIDI file via <a href="#f-sound.sequence.loadMIDIFile">playdate→sound→sequence→loadMIDIFile()</a>.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct SynthInstrument {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundInstrument {
	/**
	<code class="title">PDSynthInstrument* playdate-&gt;sound-&gt;instrument-&gt;newInstrument(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new PDSynthInstrument object.</p>
	</div>
	</div>
	*/
	pub newInstrument: unsafe extern "C" fn() -> *mut SynthInstrument,
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;freeInstrument(PDSynthInstrument* instrument)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given instrument, first removing it from the sound engine if needed.</p>
	</div>
	</div>
	*/
	pub freeInstrument: unsafe extern "C" fn(inst: *mut SynthInstrument),
	/**
	<code class="title">int playdate-&gt;sound-&gt;instrument-&gt;addVoice(PDSynthInstrument* instrument, PDSynth* synth, MIDINote rangeStart, MIDINote rangeEnd, float transpose)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds the given <a href="#C-sound.synth">PDSynth</a> to the instrument. The synth will respond to playNote events between <em>rangeState</em> and <em>rangeEnd</em>, inclusive. The <em>transpose</em> argument is in half-step units, and is added to the instrument’s <a href="#f-sound.instrument.setTranspose">transpose</a> parameter. The function returns 1 if successful, or 0 if the synth is already in another instrument or channel.</p>
	</div>
	</div>
	*/
	pub addVoice: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                   synth: *mut Synth,
	                                   rangeStart: MidiNote,
	                                   rangeEnd: MidiNote,
	                                   transpose: core::ffi::c_float)
	                                   -> core::ffi::c_int,
	/**
	<code class="title">PDSynth* playdate-&gt;sound-&gt;instrument-&gt;playNote(PDSynthInstrument* instrument, float frequency, float vel, float len, uint32_t when)</code>
	<div class="content">

	</div>
	*/
	pub playNote: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                   frequency: core::ffi::c_float,
	                                   vel: core::ffi::c_float,
	                                   len: core::ffi::c_float,
	                                   when: u32) -> *mut Synth,
	/**
	<code class="title">PDSynth* playdate-&gt;sound-&gt;instrument-&gt;playMIDINote(PDSynthInstrument* instrument, MIDINote note, float vel, float len, uint32_t when)</code>
	<div class="content">
	<div class="paragraph">
	<p>The instrument passes the playNote/playMIDINote() event to the synth in its collection that has been off for the longest, or has been playing longest if all synths are currently playing. See also <a href="#f-sound.synth.playNote">playdate→sound→synth→playNote()</a>. The PDSynth that received the playNote event is returned.</p>
	</div>
	</div>
	*/
	pub playMIDINote: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                       note: MidiNote,
	                                       vel: core::ffi::c_float,
	                                       len: core::ffi::c_float,
	                                       when: u32) -> *mut Synth,
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;setPitchBend(PDSynthInstrument* instrument, float amount)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the pitch bend to be applied to the voices in the instrument, as a fraction of the full range.</p>
	</div>
	</div>
	*/
	pub setPitchBend: unsafe extern "C" fn(inst: *mut SynthInstrument, bend: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;setPitchBendRange(PDSynthInstrument* instrument, float halfSteps)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the pitch bend range for the voices in the instrument. The default range is 12, for a full octave.</p>
	</div>
	</div>
	*/
	pub setPitchBendRange: unsafe extern "C" fn(inst: *mut SynthInstrument, halfSteps: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;setTranspose(PDSynthInstrument* instrument, float halfSteps)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the transpose parameter for all voices in the instrument.</p>
	</div>
	</div>
	*/
	pub setTranspose: unsafe extern "C" fn(inst: *mut SynthInstrument, halfSteps: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;noteOff(PDSynthInstrument* instrument, MIDINote note, uint32_t when)</code>
	<div class="content">
	<div class="paragraph">
	<p>Forwards the noteOff() event to the synth currently playing the given note. See also <a href="#f-sound.synth.noteOff">playdate→sound→synth→noteOff()</a>.</p>
	</div>
	</div>
	*/
	pub noteOff: unsafe extern "C" fn(inst: *mut SynthInstrument, note: MidiNote, when: u32),
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;allNotesOff(PDSynthInstrument* instrument, uint32_t when)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sends a noteOff event to all voices in the instrument.</p>
	</div>
	</div>
	*/
	pub allNotesOff: unsafe extern "C" fn(inst: *mut SynthInstrument, when: u32),
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;setVolume(PDSynthInstrument* instrument, float lvol, float rvol)</code>
	<div class="content">

	</div>
	*/
	pub setVolume:
		unsafe extern "C" fn(inst: *mut SynthInstrument, left: core::ffi::c_float, right: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;instrument-&gt;getVolume(PDSynthInstrument* instrument, float* outlvol, float* outrvol)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets and gets the playback volume (0.0 - 1.0) for left and right channels of the synth. This is equivalent to</p>
	</div>
	<div class="literalblock">
	<div class="content">
	<pre>playdate-&gt;sound-&gt;source-&gt;setVolume((SoundSource*)instrument, lvol, rvol);
	playdate-&gt;sound-&gt;source-&gt;getVolume((SoundSource*)instrument, &amp;lvol, &amp;rvol);</pre>
	</div>
	</div>
	</div>
	*/
	pub getVolume: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                    left: *mut core::ffi::c_float,
	                                    right: *mut core::ffi::c_float),
	/**
	<code class="title">int playdate-&gt;sound-&gt;instrument-&gt;activeVoiceCount(PDSynthInstrument* instrument)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the number of voices in the instrument currently playing.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct SequenceTrack {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundTrack {
	/**
	<code class="title">SequenceTrack* playdate-&gt;sound-&gt;track-&gt;newTrack(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new SequenceTrack.</p>
	</div>
	</div>
	*/
	pub newTrack: unsafe extern "C" fn() -> *mut SequenceTrack,
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;freeTrack(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the SequenceTrack.</p>
	</div>
	</div>
	*/
	pub freeTrack: unsafe extern "C" fn(track: *mut SequenceTrack),
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;setInstrument(SequenceTrack* track, PDSynthInstrument* instrument)</code>
	<div class="content">

	</div>
	*/
	pub setInstrument: unsafe extern "C" fn(track: *mut SequenceTrack, inst: *mut SynthInstrument),
	/**
	<code class="title">PDSynthInstrument* playdate-&gt;sound-&gt;track-&gt;getInstrument(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets or gets the <a href="#C-sound.PDSynthInstrument">PDSynthInstrument</a> assigned to the track.</p>
	</div>
	</div>
	*/
	pub getInstrument: unsafe extern "C" fn(track: *mut SequenceTrack) -> *mut SynthInstrument,
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;addNoteEvent(SequenceTrack* track, uint32_t step, uint32_t length, MIDINote note, float vel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds a single note event to the track.</p>
	</div>
	</div>
	*/
	pub addNoteEvent: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                       step: u32,
	                                       len: u32,
	                                       note: MidiNote,
	                                       velocity: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;removeNoteEvent(SequenceTrack* track, uint32_t step, MIDINote note)</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes the event at <em>step</em> playing <em>note</em>.</p>
	</div>
	</div>
	*/
	pub removeNoteEvent: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32, note: MidiNote),
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;clearNotes(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears all notes from the track.</p>
	</div>
	</div>
	*/
	pub clearNotes: unsafe extern "C" fn(track: *mut SequenceTrack),
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;getControlSignalCount(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the number of <a href="#C-sound.ControlSignal">ControlSignal</a> objects in the track.</p>
	</div>
	</div>
	*/
	pub getControlSignalCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;getControlSignal(SequenceTrack* track, int idx)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the <a href="#C-sound.ControlSignal">ControlSignal</a> at index <em>idx</em>.</p>
	</div>
	</div>
	*/
	pub getControlSignal:
		unsafe extern "C" fn(track: *mut SequenceTrack, idx: core::ffi::c_int) -> *mut ControlSignal,
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;clearControlEvents(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Clears all <a href="#C-sound.ControlSignal">ControlSignals</a> from the track.</p>
	</div>
	</div>
	*/
	pub clearControlEvents: unsafe extern "C" fn(track: *mut SequenceTrack),
	/**
	<code class="title">int playdate-&gt;sound-&gt;track-&gt;getPolyphony(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the maximum number of simultaneously playing notes in the track. (Currently, this value is only set when the track was loaded from a MIDI file. We don’t yet track polyphony for user-created events.)</p>
	</div>
	</div>
	*/
	pub getPolyphony: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;sound-&gt;track-&gt;activeVoiceCount(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the number of voices currently playing in the track’s instrument.</p>
	</div>
	</div>
	*/
	pub activeVoiceCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;setMuted(SequenceTrack* track, int mute)</code>
	<div class="content">
	<div class="paragraph">
	<p>Mutes or unmutes the track.</p>
	</div>
	</div>
	*/
	pub setMuted: unsafe extern "C" fn(track: *mut SequenceTrack, mute: core::ffi::c_int),
	/**
	<code class="title">int playdate-&gt;sound-&gt;track-&gt;getLength(SequenceTrack* track)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the length, in steps, of the track—​that is, the step where the last note in the track ends.</p>
	</div>
	</div>
	*/
	pub getLength: unsafe extern "C" fn(track: *mut SequenceTrack) -> u32,
	/**
	<code class="title">int playdate-&gt;sound-&gt;track-&gt;getIndexForStep(SequenceTrack* track, uint32_t step)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the internal array index for the first note at the given step.</p>
	</div>
	</div>
	*/
	pub getIndexForStep: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;sound-&gt;track-&gt;getNoteAtIndex(SequenceTrack* track, int index, uint32_t* outStep, uint32_t* outLen, MIDINote* outNote, float* outVelocity)</code>
	<div class="content">
	<div class="paragraph">
	<p>If the given index is in range, sets the data in the out pointers and returns 1; otherwise, returns 0.</p>
	</div>
	</div>
	*/
	pub getNoteAtIndex: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                         index: core::ffi::c_int,
	                                         outStep: *mut u32,
	                                         outLen: *mut u32,
	                                         outNote: *mut MidiNote,
	                                         outVelocity: *mut core::ffi::c_float)
	                                         -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;track-&gt;getSignalForController(SequenceTrack* track, int controller, int create)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the <a href="#C-sound.ControlSignal">ControlSignal</a> for MIDI controller number <em>controller</em>, creating it if the <strong>create</strong> flag is set and it doesn’t yet exist.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct SoundSequence {
	_unused: [u8; 0],
}
pub type SequenceFinishedCallback =
	::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[must_use]
pub struct PlaydateSoundSequence {
	/**
	<code class="title">SoundSequence* playdate-&gt;sound-&gt;sequence-&gt;newSequence(void)</code>
	<div class="content">

	</div>
	*/
	pub newSequence: unsafe extern "C" fn() -> *mut SoundSequence,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;freeSequence(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates or destroys a SoundSequence object.</p>
	</div>
	</div>
	*/
	pub freeSequence: unsafe extern "C" fn(sequence: *mut SoundSequence),
	/**
	<code class="title">int playdate-&gt;sound-&gt;sequence-&gt;loadMIDIFile(SoundSequence* sequence, const char* path)</code>
	<div class="content">
	<div class="paragraph">
	<p>If the sequence is empty, attempts to load data from the MIDI file at <em>path</em> into the sequence. Returns 1 on success, 0 on failure.</p>
	</div>
	</div>
	*/
	pub loadMIDIFile:
		unsafe extern "C" fn(seq: *mut SoundSequence, path: *const core::ffi::c_char) -> core::ffi::c_int,
	/**
	<code class="title">uint32_t playdate-&gt;sound-&gt;sequence-&gt;getTime(SoundSequence* sequence)</code>
	<div class="content">

	</div>
	*/
	pub getTime: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;setTime(SoundSequence* sequence, uint32_t time)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets or sets the current time in the sequence, in samples since the start of the file. Note that which step this moves the sequence to depends on the current tempo.</p>
	</div>
	</div>
	*/
	pub setTime: unsafe extern "C" fn(seq: *mut SoundSequence, time: u32),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;setLoops(SoundSequence* sequence, int startstep, int endstep, int loops)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the looping range of the sequence. If <em>loops</em> is 0, the loop repeats endlessly.</p>
	</div>
	</div>
	*/
	pub setLoops: unsafe extern "C" fn(seq: *mut SoundSequence,
	                                   loopstart: core::ffi::c_int,
	                                   loopend: core::ffi::c_int,
	                                   loops: core::ffi::c_int),
	pub getTempo_deprecated: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;setTempo(SoundSequence* sequence, float stepsPerSecond)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets or gets the tempo of the sequence, in steps per second.</p>
	</div>
	</div>
	*/
	pub setTempo: unsafe extern "C" fn(seq: *mut SoundSequence, stepsPerSecond: core::ffi::c_float),
	/**
	<code class="title">int playdate-&gt;sound-&gt;sequence-&gt;getTrackCount(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the number of tracks in the sequence.</p>
	</div>
	</div>
	*/
	pub getTrackCount: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	/**
	<code class="title">SequenceTrack* playdate-&gt;sound-&gt;sequence-&gt;addTrack(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds the given <a href="#C-sound.track">playdate.sound.track</a> to the sequence.</p>
	</div>
	</div>
	*/
	pub addTrack: unsafe extern "C" fn(seq: *mut SoundSequence) -> *mut SequenceTrack,
	/**
	<code class="title">SequenceTrack* playdate-&gt;sound-&gt;sequence-&gt;getTrackAtIndex(SoundSequence* sequence, unsigned int idx)</code>
	<div class="content">

	</div>
	*/
	pub getTrackAtIndex:
		unsafe extern "C" fn(seq: *mut SoundSequence, track: core::ffi::c_uint) -> *mut SequenceTrack,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;setTrackAtIndex(SoundSequence* sequence, SequenceTrack* track, unsigned int idx)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets or gets the given <a href="#C-sound.track">SoundTrack</a> object at position <em>idx</em> in the sequence.</p>
	</div>
	</div>
	*/
	pub setTrackAtIndex:
		unsafe extern "C" fn(seq: *mut SoundSequence, track: *mut SequenceTrack, idx: core::ffi::c_uint),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;allNotesOff(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sends a stop signal to all playing notes on all tracks.</p>
	</div>
	</div>
	*/
	pub allNotesOff: unsafe extern "C" fn(seq: *mut SoundSequence),
	/**
	<code class="title">int playdate-&gt;sound-&gt;sequence-&gt;isPlaying(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns 1 if the sequence is currently playing, otherwise 0.</p>
	</div>
	</div>
	*/
	pub isPlaying: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;sound-&gt;sequence-&gt;getLength(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the length of the longest track in the sequence, in steps. See also <a href="#m-sound.track:getLength">playdate.sound.track.getLength()</a>.</p>
	</div>
	</div>
	*/
	pub getLength: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;play(SoundSequence* sequence, SequenceFinishedCallback finishCallback, void* userdata)</code>
	<div class="content">

	</div>
	*/
	pub play: unsafe extern "C" fn(seq: *mut SoundSequence,
	                               finishCallback: SequenceFinishedCallback,
	                               userdata: *mut core::ffi::c_void),
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;stop(SoundSequence* sequence)</code>
	<div class="content">
	<div class="paragraph">
	<p>Starts or stops playing the sequence. <code>finishCallback</code> is an optional function to be called when the sequence finishes playing or is stopped.</p>
	</div>
	<div id="_SequenceFinishedCallback" class="literalblock">
	<div class="title">SequenceFinishedCallback</div>
	<div class="content">
	<pre>typedef void (*SequenceFinishedCallback)(SoundSequence* seq, void* userdata);</pre>
	</div>
	</div>
	</div>
	*/
	pub stop: unsafe extern "C" fn(seq: *mut SoundSequence),
	/**
	<code class="title">int playdate-&gt;sound-&gt;sequence-&gt;getCurrentStep(SoundSequence* sequence, int* timeOffset)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the step number the sequence is currently at. If <em>timeOffset</em> is not NULL, its contents are set to the current sample offset within the step.</p>
	</div>
	</div>
	*/
	pub getCurrentStep:
		unsafe extern "C" fn(seq: *mut SoundSequence, timeOffset: *mut core::ffi::c_int) -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;sound-&gt;sequence-&gt;setCurrentStep(SoundSequence* seq, int step, int timeOffset, int playNotes)</code>
	<div class="content">
	<div class="paragraph">
	<p>Set the current step for the sequence. <em>timeOffset</em> is a sample offset within the step. If <em>playNotes</em> is set, notes at the given step (ignoring <em>timeOffset</em>) are played.</p>
	</div>
	</div>
	*/
	pub setCurrentStep: unsafe extern "C" fn(seq: *mut SoundSequence,
	                                         step: core::ffi::c_int,
	                                         timeOffset: core::ffi::c_int,
	                                         playNotes: core::ffi::c_int),
	/**
	<code class="title">float playdate-&gt;sound-&gt;sequence-&gt;getTempo(SoundSequence* sequence)</code>
	<div class="content">

	</div>
	*/
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
#[must_use]
pub struct TwoPoleFilter {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
#[must_use]
pub struct PlaydateSoundEffectTwopolefilter {
	/**
	<code class="title">TwoPoleFilter* playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;newFilter(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new two pole filter effect.</p>
	</div>
	</div>
	*/
	pub newFilter: unsafe extern "C" fn() -> *mut TwoPoleFilter,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;freeFilter(TwoPoleFilter* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given filter.</p>
	</div>
	</div>
	*/
	pub freeFilter: unsafe extern "C" fn(filter: *mut TwoPoleFilter),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;setType(TwoPoleFilter* filter, TwoPoleFilterType type)</code>
	<div class="content">
	<div class="literalblock">
	<div class="title">TwoPoleFilterType</div>
	<div class="content">
	<pre>typedef enum
	{
		kFilterTypeLowPass,
		kFilterTypeHighPass,
		kFilterTypeBandPass,
		kFilterTypeNotch,
		kFilterTypePEQ,
		kFilterTypeLowShelf,
		kFilterTypeHighShelf
	} TwoPoleFilterType;</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>Sets the type of the filter.</p>
	</div>
	</div>
	*/
	pub setType: unsafe extern "C" fn(filter: *mut TwoPoleFilter, type_: TwoPoleFilterType),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;setFrequency(TwoPoleFilter* filter, float frequency)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the center/corner frequency of the filter. Value is in Hz.</p>
	</div>
	</div>
	*/
	pub setFrequency: unsafe extern "C" fn(filter: *mut TwoPoleFilter, frequency: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;setFrequencyModulator(TwoPoleFilter* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the effect’s frequency. The signal is scaled so that a value of 1.0 corresponds to half the sample rate. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setFrequencyModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;getFrequencyModulator(TwoPoleFilter* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the filter’s current frequency modulator.</p>
	</div>
	</div>
	*/
	pub getFrequencyModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;setGain(TwoPoleFilter* filter, float gain)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the filter gain.</p>
	</div>
	</div>
	*/
	pub setGain: unsafe extern "C" fn(filter: *mut TwoPoleFilter, gain: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;setResonance(TwoPoleFilter* filter, float resonance)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the filter resonance.</p>
	</div>
	</div>
	*/
	pub setResonance: unsafe extern "C" fn(filter: *mut TwoPoleFilter, resonance: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;setResonanceModulator(TwoPoleFilter* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the filter resonance. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setResonanceModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;twopolefilter-&gt;getResonanceModulator(TwoPoleFilter* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the filter’s current resonance modulator.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct OnePoleFilter {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundEffectOnepolefilter {
	/**
	<code class="title">OnePoleFilter* playdate-&gt;sound-&gt;effect-&gt;onepolefilter-&gt;newFilter(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new one pole filter.</p>
	</div>
	</div>
	*/
	pub newFilter: unsafe extern "C" fn() -> *mut OnePoleFilter,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;onepolefilter-&gt;freeFilter(OnePoleFilter* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the filter.</p>
	</div>
	</div>
	*/
	pub freeFilter: unsafe extern "C" fn(filter: *mut OnePoleFilter),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;onepolefilter-&gt;setParameter(OnePoleFilter* filter, float parameter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the filter’s single parameter (cutoff frequency) to <em>p</em>. Values above 0 (up to 1) are high-pass, values below 0 (down to -1) are low-pass.</p>
	</div>
	</div>
	*/
	pub setParameter: unsafe extern "C" fn(filter: *mut OnePoleFilter, parameter: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;onepolefilter-&gt;setParameterModulator(OnePoleFilter* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the filter parameter. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setParameterModulator: unsafe extern "C" fn(filter: *mut OnePoleFilter, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;onepolefilter-&gt;getParameterModulator(OnePoleFilter* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the filter’s current parameter modulator.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct BitCrusher {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundEffectBitcrusher {
	/**
	<code class="title">BitCrusher* playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;newBitCrusher(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new BitCrusher effect.</p>
	</div>
	</div>
	*/
	pub newBitCrusher: unsafe extern "C" fn() -> *mut BitCrusher,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;freeBitCrusher(BitCrusher* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given effect.</p>
	</div>
	</div>
	*/
	pub freeBitCrusher: unsafe extern "C" fn(filter: *mut BitCrusher),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;setAmount(BitCrusher* filter, float amount)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the amount of crushing to <em>amount</em>. Valid values are 0 (no effect) to 1 (quantizing output to 1-bit).</p>
	</div>
	</div>
	*/
	pub setAmount: unsafe extern "C" fn(filter: *mut BitCrusher, amount: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;setAmountModulator(BitCrusher* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the crushing amount. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setAmountModulator: unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;getAmountModulator(BitCrusher* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set modulator.</p>
	</div>
	</div>
	*/
	pub getAmountModulator: unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;setUndersampling(BitCrusher* filter, float undersample)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the number of samples to repeat, quantizing the input in time. A value of 0 produces no undersampling, 1 repeats every other sample, etc.</p>
	</div>
	</div>
	*/
	pub setUndersampling: unsafe extern "C" fn(filter: *mut BitCrusher, undersampling: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;setUndersampleModulator(BitCrusher* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the undersampling amount. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setUndersampleModulator: unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;bitcrusher-&gt;getUndersampleModulator(BitCrusher* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set modulator.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct RingModulator {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundEffectRingmodulator {
	/**
	<code class="title">RingModulator* playdate-&gt;sound-&gt;effect-&gt;ringmodulator-&gt;newRingmod(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new ring modulator effect.</p>
	</div>
	</div>
	*/
	pub newRingmod: unsafe extern "C" fn() -> *mut RingModulator,
	pub freeRingmod: unsafe extern "C" fn(filter: *mut RingModulator),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;ringmodulator-&gt;setFrequency(RingModulator* filter, float frequency)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the frequency of the modulation signal.</p>
	</div>
	</div>
	*/
	pub setFrequency: unsafe extern "C" fn(filter: *mut RingModulator, frequency: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;ringmodulator-&gt;setFrequencyModulator(RingModulator* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the frequency of the ring modulator. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setFrequencyModulator: unsafe extern "C" fn(filter: *mut RingModulator, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;ringmodulator-&gt;getFrequencyModulator(RingModulator* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set frequency modulator.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct DelayLine {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct DelayLineTap {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundEffectDelayline {
	/**
	<code class="title">DelayLine* playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;newDelayLine(int length, int stereo)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new delay line effect. The <em>length</em> parameter is given in samples.</p>
	</div>
	</div>
	*/
	pub newDelayLine: unsafe extern "C" fn(length: core::ffi::c_int, stereo: core::ffi::c_int) -> *mut DelayLine,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;freeDelayLine(DelayLine* delay)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the delay line.</p>
	</div>
	</div>
	*/
	pub freeDelayLine: unsafe extern "C" fn(filter: *mut DelayLine),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;setLength(DelayLine* d, int frames)</code>
	<div class="content">
	<div class="paragraph">
	<p>Changes the length of the delay line, clearing its contents. This function reallocates the audio buffer, so it is not safe to call while the delay line is in use.</p>
	</div>
	</div>
	*/
	pub setLength: unsafe extern "C" fn(d: *mut DelayLine, frames: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;setFeedback(DelayLine* d, float fb)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the feedback level of the delay line.</p>
	</div>
	</div>
	*/
	pub setFeedback: unsafe extern "C" fn(d: *mut DelayLine, fb: core::ffi::c_float),
	/**
	<code class="title">DelayLineTap* playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;addTap(DelayLine* d, int delay)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new tap on the delay line, at the given position. <em>delay</em> must be less than or equal to the length of the delay line.</p>
	</div>
	</div>
	*/
	pub addTap: unsafe extern "C" fn(d: *mut DelayLine, delay: core::ffi::c_int) -> *mut DelayLineTap,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;freeTap(DelayLineTap* tap)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees a tap previously created with <a href="#f-sound.effect.delayline.addTap">playdate→sound→delayline→addTap()</a>, first removing it from the sound engine if needed.</p>
	</div>
	</div>
	*/
	pub freeTap: unsafe extern "C" fn(tap: *mut DelayLineTap),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;setTapDelay(DelayLineTap* tap, int frames)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the position of the tap on the delay line, up to the delay line’s length.</p>
	</div>
	</div>
	*/
	pub setTapDelay: unsafe extern "C" fn(t: *mut DelayLineTap, frames: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;setTapDelayModulator(DelayLineTap* tap, PDSynthSignalValue* mod)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the tap delay. If the signal is continuous (e.g. an envelope or a triangle LFO, but not a square LFO) playback is sped up or slowed down to compress or expand time. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setTapDelayModulator: unsafe extern "C" fn(t: *mut DelayLineTap, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;getTapDelayModulator(DelayLineTap* tap)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the current delay modulator.</p>
	</div>
	</div>
	*/
	pub getTapDelayModulator: unsafe extern "C" fn(t: *mut DelayLineTap) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;delayline-&gt;setTapChannelsFlipped(DelayLineTap* tap, int flip)</code>
	<div class="content">
	<div class="paragraph">
	<p>If the delay line is stereo and <em>flip</em> is set, the tap outputs the delay line’s left channel to its right output and vice versa.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct Overdrive {
	_unused: [u8; 0],
}
#[repr(C)]
#[must_use]
pub struct PlaydateSoundEffectOverdrive {
	/**
	<code class="title">Overdrive* playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;newOverdrive(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new overdrive effect.</p>
	</div>
	</div>
	*/
	pub newOverdrive: unsafe extern "C" fn() -> *mut Overdrive,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;freeOverdrive(Overdrive* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given effect.</p>
	</div>
	</div>
	*/
	pub freeOverdrive: unsafe extern "C" fn(filter: *mut Overdrive),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;setGain(Overdrive* filter, float gain)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the gain of the overdrive effect.</p>
	</div>
	</div>
	*/
	pub setGain: unsafe extern "C" fn(o: *mut Overdrive, gain: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;setLimit(Overdrive* filter, float limit)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the level where the amplified input clips.</p>
	</div>
	</div>
	*/
	pub setLimit: unsafe extern "C" fn(o: *mut Overdrive, limit: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;setLimitModulator(Overdrive* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the limit parameter. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setLimitModulator: unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;getLimitModulator(RingMoOverdrivedulator* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set limit modulator.</p>
	</div>
	</div>
	*/
	pub getLimitModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;setOffset(Overdrive* filter, float offset)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds an offset to the upper and lower limits to create an asymmetric clipping.</p>
	</div>
	</div>
	*/
	pub setOffset: unsafe extern "C" fn(o: *mut Overdrive, offset: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;setOffsetModulator(Overdrive* filter, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the offset parameter. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setOffsetModulator: unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;overdrive-&gt;getOffsetModulator(RingMoOverdrivedulator* filter)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the currently set offset modulator.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct PlaydateSoundEffect {
	/**
	<code class="title">SoundEffect* playdate-&gt;sound-&gt;effect-&gt;newEffect(effectProc* proc, void* userdata)</code>
	<div class="content">
	<div class="literalblock">
	<div class="title">effectProc</div>
	<div class="content">
	<pre>typedef int effectProc(SoundEffect* e, int32_t* left, int32_t* right, int nsamples, int bufactive);</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>Creates a new effect using the given processing function. <em>bufactive</em> is 1 if samples have been set in the left or right buffers. The function should return 1 if it changed the buffer samples, otherwise 0. <em>left</em> and <em>right</em> (if the effect is on a stereo channel) are sample buffers in Q8.24 format.</p>
	</div>
	</div>
	*/
	pub newEffect: unsafe extern "C" fn(proc_: EffectProc, userdata: *mut core::ffi::c_void) -> *mut SoundEffect,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;freeEffect(SoundEffect* effect)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given effect.</p>
	</div>
	</div>
	*/
	pub freeEffect: unsafe extern "C" fn(effect: *mut SoundEffect),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;setMix(SoundEffect* effect, float level)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix (which is useful if you’re using a delay line with taps and don’t want to hear the delay line itself).</p>
	</div>
	</div>
	*/
	pub setMix: unsafe extern "C" fn(effect: *mut SoundEffect, level: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;setMixModulator(SoundEffect* effect, PDSynthSignalValue* signal)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the effect’s mix level. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setMixModulator: unsafe extern "C" fn(effect: *mut SoundEffect, signal: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;effect-&gt;getMixModulator(SoundEffect* effect)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the current mix modulator for the effect.</p>
	</div>
	</div>
	*/
	pub getMixModulator: unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;effect-&gt;setUserdata(SoundEffect* effect, void* userdata)</code>
	<div class="content">

	</div>
	*/
	pub setUserdata: unsafe extern "C" fn(effect: *mut SoundEffect, userdata: *mut core::ffi::c_void),
	/**
	<code class="title">void* playdate-&gt;sound-&gt;effect-&gt;getUserdata(SoundEffect* effect)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets or gets a userdata value for the effect.</p>
	</div>
	</div>
	*/
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
#[must_use]
pub struct PlaydateSoundChannel {
	/**
	<code class="title">SoundChannel* playdate-&gt;sound-&gt;channel-&gt;newChannel(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a new <em>SoundChannel</em> object.</p>
	</div>
	</div>
	*/
	pub newChannel: unsafe extern "C" fn() -> *mut SoundChannel,
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;freeChannel(SoundChannel* channel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Frees the given <em>SoundChannel</em>.</p>
	</div>
	</div>
	*/
	pub freeChannel: unsafe extern "C" fn(channel: *mut SoundChannel),
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;addSource(SoundChannel* channel, SoundSource* source)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds a <a href="#f-sound.source">SoundSource</a> to the channel. If a source is not assigned to a channel, it plays on the default global channel.</p>
	</div>
	</div>
	*/
	pub addSource: unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;sound-&gt;channel-&gt;removeSource(SoundChannel* channel, SoundSource* source)</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes a <a href="#f-sound.source">SoundSource</a> to the channel. Returns 1 if the source was found in (and removed from) the channel, otherwise 0.</p>
	</div>
	</div>
	*/
	pub removeSource:
		unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> core::ffi::c_int,
	/**
	<code class="title">SoundSource* playdate-&gt;sound-&gt;channel-&gt;addCallbackSource(SoundChannel* channel, AudioSourceFunction* callback, void* context, int stereo)</code>
	<div class="content">
	<div class="paragraph">
	<p>Creates a new <a href="#f-sound.source">SoundSource</a> using the given data provider callback and adds it to the channel.</p>
	</div>
	<div class="literalblock">
	<div class="title">AudioSourceFunction</div>
	<div class="content">
	<pre>int AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>This function should fill the passed-in <em>left</em> buffer (and <em>right</em> if it’s a stereo source) with <em>len</em> samples each and return 1, or return 0 if the source is silent through the cycle. The caller takes ownership of the allocated SoundSource, and should free it with</p>
	</div>
	<div class="literalblock">
	<div class="content">
	<pre>playdate-&gt;system-&gt;realloc(source, 0);</pre>
	</div>
	</div>
	<div class="paragraph">
	<p>when it is not longer in use.</p>
	</div>
	</div>
	*/
	pub addCallbackSource: unsafe extern "C" fn(channel: *mut SoundChannel,
	                                            callback: AudioSourceFunction,
	                                            context: *mut core::ffi::c_void,
	                                            stereo: core::ffi::c_int)
	                                            -> *mut SoundSource,
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;addEffect(SoundChannel* channel, SoundEffect* effect)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds a <a href="#f-sound.effect">SoundEffect</a> to the channel.</p>
	</div>
	</div>
	*/
	pub addEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;removeEffect(SoundChannel* channel, SoundEffect* effect)</code>
	<div class="content">
	<div class="paragraph">
	<p>Removes a <a href="#f-sound.effect">SoundEffect</a> from the channel.</p>
	</div>
	</div>
	*/
	pub removeEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;setVolume(SoundChannel* channel, float volume)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the volume for the channel, in the range [0-1].</p>
	</div>
	</div>
	*/
	pub setVolume: unsafe extern "C" fn(channel: *mut SoundChannel, volume: core::ffi::c_float),
	/**
	<code class="title">float playdate-&gt;sound-&gt;channel-&gt;getVolume(SoundChannel* channel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets the volume for the channel, in the range [0-1].</p>
	</div>
	</div>
	*/
	pub getVolume: unsafe extern "C" fn(channel: *mut SoundChannel) -> core::ffi::c_float,
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;setVolumeModulator(SoundChannel* channel, PDSynthSignalValue* mod)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the channel volume. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setVolumeModulator: unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;channel-&gt;getVolumeModulator(SoundChannel* channel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets a <a href="#C-sound.signal">signal</a> modulating the channel volume.</p>
	</div>
	</div>
	*/
	pub getVolumeModulator: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;setPan(SoundChannel* channel, float pan)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the pan parameter for the channel. Valid values are in the range [-1,1], where -1 is left, 0 is center, and 1 is right.</p>
	</div>
	</div>
	*/
	pub setPan: unsafe extern "C" fn(channel: *mut SoundChannel, pan: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;sound-&gt;channel-&gt;setPanModulator(SoundChannel* channel, PDSynthSignalValue* mod)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets a <a href="#C-sound.signal">signal</a> to modulate the channel pan. Set to <em>NULL</em> to clear the modulator.</p>
	</div>
	</div>
	*/
	pub setPanModulator: unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut SynthSignalValue),
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;channel-&gt;getPanModulator(SoundChannel* channel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Gets a <a href="#C-sound.signal">signal</a> modulating the channel pan.</p>
	</div>
	</div>
	*/
	pub getPanModulator: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;channel-&gt;getDryLevelSignal(SoundChannel* channel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a signal that follows the volume of the channel before effects are applied.</p>
	</div>
	</div>
	*/
	pub getDryLevelSignal: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	/**
	<code class="title">PDSynthSignalValue* playdate-&gt;sound-&gt;channel-&gt;getWetLevelSignal(SoundChannel* channel)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns a signal that follows the volume of the channel after effects are applied.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub enum MicSource {
	Autodetect = 0,
	Internal = 1,
	Headset = 2,
}
#[repr(C)]
#[must_use]
pub struct PlaydateSound {
    pub channel: &'static PlaydateSoundChannel,
    pub fileplayer: &'static PlaydateSoundFileplayer,
    pub sample: &'static PlaydateSoundSample,
    pub sampleplayer: &'static PlaydateSoundSampleplayer,
    pub synth: &'static PlaydateSoundSynth,
    pub sequence: &'static PlaydateSoundSequence,
    pub effect: &'static PlaydateSoundEffect,
    pub lfo: &'static PlaydateSoundLfo,
    pub envelope: &'static PlaydateSoundEnvelope,
    pub source: &'static PlaydateSoundSource,
    pub controlsignal: &'static PlaydateControlSignal,
    pub track: &'static PlaydateSoundTrack,
    pub instrument: &'static PlaydateSoundInstrument,
    /**
<code class="title">uint32_t playdate-&gt;sound-&gt;getCurrentTime(void)</code>
<div class="content">
<div class="paragraph">
<p>Returns the sound engine’s current time value, in units of sample frames, 44,100 per second.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-sound.getCurrentTime"><code>playdate.sound.getCurrentTime()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub getCurrentTime: unsafe extern "C" fn() -> u32,
    /**
<code class="title">SoundSource* playdate-&gt;sound-&gt;addSource(AudioSourceFunction* callback, void* context, int stereo)</code>
<div class="content">
<div class="paragraph">
<p>The <em>callback</em> function you pass in will be called every audio render cycle.</p>
</div>
<div class="literalblock">
<div class="title">AudioSourceFunction</div>
<div class="content">
<pre>int AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)</pre>
</div>
</div>
<div class="paragraph">
<p>This function should fill the passed-in <em>left</em> buffer (and <em>right</em> if it’s a stereo source) with <em>len</em> samples each and return 1, or return 0 if the source is silent through the cycle.</p>
</div>
</div>
*/
    pub addSource: unsafe extern "C" fn(
        callback: AudioSourceFunction,
        context: *mut core::ffi::c_void,
        stereo: core::ffi::c_int,
    ) -> *mut SoundSource,
    /**
<code class="title">SoundChannel* playdate-&gt;sound-&gt;getDefaultChannel(void)</code>
<div class="content">
<div class="paragraph">
<p>Returns the default channel, where sound sources play if they haven’t been explicity assigned to a different channel.</p>
</div>
</div>
*/
    pub getDefaultChannel: unsafe extern "C" fn() -> *mut SoundChannel,
    /**
<code class="title">int playdate-&gt;sound-&gt;addChannel(SoundChannel* channel)</code>
<div class="content">
<div class="paragraph">
<p>Adds the given channel to the sound engine. Returns 1 if the channel was added, 0 if it was already in the engine.</p>
</div>
</div>
*/
    pub addChannel: unsafe extern "C" fn(channel: *mut SoundChannel) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;sound-&gt;removeChannel(SoundChannel* channel)</code>
<div class="content">
<div class="paragraph">
<p>Removes the given channel from the sound engine. Returns 1 if the channel was successfully removed, 0 if the channel is the default channel or hadn’t been previously added.</p>
</div>
</div>
*/
    pub removeChannel: unsafe extern "C" fn(
        channel: *mut SoundChannel,
    ) -> core::ffi::c_int,
    /**
<code class="title">int playdate-&gt;sound-&gt;setMicCallback(AudioInputFunction* callback, void* context, enum MicSource source)</code>
<div class="content">
<div class="paragraph">
<p>The <em>callback</em> you pass in will be called every audio cycle.</p>
</div>
<div class="literalblock">
<div class="title">AudioInputFunction</div>
<div class="content">
<pre>int AudioInputFunction(void* context, int16_t* data, int len)</pre>
</div>
</div>
<div class="literalblock">
<div class="title">enum MicSource</div>
<div class="content">
<pre>enum MicSource {
	kMicInputAutodetect = 0,
	kMicInputInternal = 1,
	kMicInputHeadset = 2
};</pre>
</div>
</div>
<div class="paragraph">
<p>Your input callback will be called with the recorded audio data, a monophonic stream of samples. The function should return 1 to continue recording, 0 to stop recording.</p>
</div>
<div class="paragraph">
<p>The Playdate hardware has a circuit that attempts to autodetect the presence of a headset mic, but there are cases where you may want to override this. For instance, if you’re using a headphone splitter to wire an external source to the mic input, the detector may not always see the input. Setting the source to <code>kMicInputHeadset</code> forces recording from the headset. Using <code>kMicInputInternal</code> records from the device mic even when a headset with a mic is plugged in. And <code>kMicInputAutodetect</code> uses a headset mic if one is detected, otherwise the device microphone. <code>setMicCallback()</code> returns which source the function used, internal or headset, or 0 on error.</p>
</div>
</div>
*/
    pub setMicCallback: unsafe extern "C" fn(
        callback: RecordCallback,
        context: *mut core::ffi::c_void,
        source: MicSource,
    ) -> core::ffi::c_int,
    /**
<code class="title">void playdate-&gt;sound-&gt;getHeadphoneState(int* headphone, int* mic, void (*changeCallback)(int headphone, int mic))</code>
<div class="content">
<div class="paragraph">
<p>If <em>headphone</em> contains a pointer to an int, the value is set to 1 if headphones are currently plugged in. Likewise, <em>mic</em> is set if the headphones include a microphone. If <em>changeCallback</em> is provided, it will be called when the headset or mic status changes, and audio output will <strong>not</strong> automatically switch from speaker to headphones when headphones are plugged in (and vice versa). In this case, the callback should use <code>playdate→sound→setOutputsActive()</code> to change the output if needed.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-sound.getHeadphoneState"><code>playdate.sound.getHeadphoneState()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub getHeadphoneState: unsafe extern "C" fn(
        headphone: *mut core::ffi::c_int,
        headsetmic: *mut core::ffi::c_int,
        changeCallback: ::core::option::Option<
            unsafe extern "C" fn(headphone: core::ffi::c_int, mic: core::ffi::c_int),
        >,
    ),
    /**
<code class="title">void playdate-&gt;sound-&gt;setOutputsActive(int headphone, int speaker)</code>
<div class="content">
<div class="paragraph">
<p>Force audio output to the given outputs, regardless of headphone status.</p>
</div>
<div class="paragraph xref xref-lua">
<p>Equivalent to <a href="./Inside%20Playdate.html#f-sound.setOutputsActive"><code>playdate.sound.setOutputsActive()</code></a> in the Lua API.</p>
</div>
</div>
*/
    pub setOutputsActive: unsafe extern "C" fn(
        headphone: core::ffi::c_int,
        speaker: core::ffi::c_int,
    ),
    /**
<code class="title">int playdate-&gt;sound-&gt;removeSource(SoundSource* source)</code>
<div class="content">
<div class="paragraph">
<p>Removes the given <a href="#C-sound.source">SoundSource</a> object from its channel, whether it’s in the default channel or a channel created with <a href="#f-sound.addChannel">playdate→sound→addChannel()</a>. Returns 1 if a source was removed, 0 if the source wasn’t in a channel.</p>
</div>
</div>
*/
    pub removeSource: unsafe extern "C" fn(source: *mut SoundSource) -> core::ffi::c_int,
    pub signal: &'static PlaydateSoundSignal,
    pub getError: unsafe extern "C" fn() -> *const core::ffi::c_char,
}
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
#[must_use]
pub struct PlaydateDisplay {
	/**
	<code class="title">int playdate-&gt;display-&gt;getWidth(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the width of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 200 instead of 400.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.getWidth"><code>playdate.display.getWidth()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub getWidth: unsafe extern "C" fn() -> core::ffi::c_int,
	/**
	<code class="title">int playdate-&gt;display-&gt;getHeight(void)</code>
	<div class="content">
	<div class="paragraph">
	<p>Returns the height of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 120 instead of 240.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.getHeight"><code>playdate.display.getHeight()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub getHeight: unsafe extern "C" fn() -> core::ffi::c_int,
	/**
	<code class="title">void playdate-&gt;display-&gt;setRefreshRate(float rate)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the nominal refresh rate in frames per second. The default is 30 fps, which is a recommended figure that balances animation smoothness with performance and power considerations. Maximum is 50 fps.</p>
	</div>
	<div class="paragraph">
	<p>If <em>rate</em> is 0, the game’s update callback (either Lua’s <code>playdate.update()</code> or the function specified by <a href="#f-system.setUpdateCallback">playdate→system→setUpdateCallback()</a>) is called as soon as possible. Since the display refreshes line-by-line, and unchanged lines aren’t sent to the display, the update cycle will be faster than 30 times a second but at an indeterminate rate.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.setRefreshRate"><code>playdate.display.setRefreshRate()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setRefreshRate: unsafe extern "C" fn(rate: core::ffi::c_float),
	/**
	<code class="title">void playdate-&gt;display-&gt;setInverted(int flag)</code>
	<div class="content">
	<div class="paragraph">
	<p>If <em>flag</em> evaluates to true, the frame buffer is drawn inverted—black instead of white, and vice versa.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.setInverted"><code>playdate.display.setInverted()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setInverted: unsafe extern "C" fn(flag: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;display-&gt;setScale(unsigned int s)</code>
	<div class="content">
	<div class="paragraph">
	<p>Sets the display scale factor. Valid values for <em>scale</em> are 1, 2, 4, and 8.</p>
	</div>
	<div class="paragraph">
	<p>The top-left corner of the frame buffer is scaled up to fill the display; e.g., if the scale is set to 4, the pixels in rectangle [0,100] x [0,60] are drawn on the screen as 4 x 4 squares.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.setScale"><code>playdate.display.setScale()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setScale: unsafe extern "C" fn(s: core::ffi::c_uint),
	/**
	<code class="title">void playdate-&gt;display-&gt;setMosaic(unsigned int x, unsigned int y)</code>
	<div class="content">
	<div class="paragraph">
	<p>Adds a mosaic effect to the display. Valid <em>x</em> and <em>y</em> values are between 0 and 3, inclusive.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.setMosaic"><code>playdate.display.setMosaic</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setMosaic: unsafe extern "C" fn(x: core::ffi::c_uint, y: core::ffi::c_uint),
	/**
	<code class="title">void playdate-&gt;display-&gt;setFlipped(int x, int y)</code>
	<div class="content">
	<div class="paragraph">
	<p>Flips the display on the x or y axis, or both.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.setFlipped"><code>playdate.display.setFlipped()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
	pub setFlipped: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
	/**
	<code class="title">void playdate-&gt;display-&gt;setOffset(int dx, int dy)</code>
	<div class="content">
	<div class="paragraph">
	<p>Offsets the display by the given amount. Areas outside of the displayed area are filled with the current <a href="#f-graphics.setBackgroundColor">background color</a>.</p>
	</div>
	<div class="paragraph xref xref-lua">
	<p>Equivalent to <a href="./Inside%20Playdate.html#f-display.setOffset"><code>playdate.display.setOffset()</code></a> in the Lua API.</p>
	</div>
	</div>
	*/
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
#[cfg_attr(feature = "const-types", derive(::core::marker::ConstParamTy))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
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
