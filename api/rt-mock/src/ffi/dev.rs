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
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const LCD_COLUMNS: u32 = 400;
pub const LCD_ROWS: u32 = 240;
pub const LCD_ROWSIZE: u32 = 52;
pub const AUDIO_FRAMES_PER_CYCLE: u32 = 512;
pub const NOTE_C4: u32 = 60;
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default, :: core :: marker :: ConstParamTy)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateFile { pub geterr : unsafe extern "C" fn () -> * const core :: ffi :: c_char , pub listfiles : unsafe extern "C" fn (path : * const core :: ffi :: c_char , callback : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , userdata : * mut core :: ffi :: c_void) > , userdata : * mut core :: ffi :: c_void , showhidden : core :: ffi :: c_int) -> core :: ffi :: c_int , pub stat : unsafe extern "C" fn (path : * const core :: ffi :: c_char , stat : * mut FileStat) -> core :: ffi :: c_int , pub mkdir : unsafe extern "C" fn (path : * const core :: ffi :: c_char) -> core :: ffi :: c_int , pub unlink : unsafe extern "C" fn (name : * const core :: ffi :: c_char , recursive : core :: ffi :: c_int) -> core :: ffi :: c_int , pub rename : unsafe extern "C" fn (from : * const core :: ffi :: c_char , to : * const core :: ffi :: c_char) -> core :: ffi :: c_int , pub open : unsafe extern "C" fn (name : * const core :: ffi :: c_char , mode : FileOptions) -> * mut SdFile , pub close : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , pub read : unsafe extern "C" fn (file : * mut SdFile , buf : * mut core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int , pub write : unsafe extern "C" fn (file : * mut SdFile , buf : * const core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int , pub flush : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , pub tell : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , pub seek : unsafe extern "C" fn (file : * mut SdFile , pos : core :: ffi :: c_int , whence : core :: ffi :: c_int) -> core :: ffi :: c_int , }
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
#[must_use]
pub struct Aabb {
	pub left: core::ffi::c_int,
	pub right: core::ffi::c_int,
	pub top: core::ffi::c_int,
	pub bottom: core::ffi::c_int,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
pub enum BitmapFlip {
	Unflipped = 0,
	FlippedX = 1,
	FlippedY = 2,
	FlippedXy = 3,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
pub enum SolidColor {
	Black = 0,
	White = 1,
	Clear = 2,
	XOR = 3,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
pub enum StringEncoding {
	ASCII = 0,
	UTF8 = 1,
	UTF16 = 2,
}
pub type Pattern = [u8; 16usize];
pub type Color = usize;
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
pub struct TileMap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct VideoPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct StreamPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct HttpConnection {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct TcpConnection {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct FilePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateVideo {
	pub loadVideo: unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut VideoPlayer,
	pub freePlayer: unsafe extern "C" fn(p: *mut VideoPlayer),
	pub setContext: unsafe extern "C" fn(p: *mut VideoPlayer, context: *mut Bitmap) -> core::ffi::c_int,
	pub useScreenContext: unsafe extern "C" fn(p: *mut VideoPlayer),
	pub renderFrame: unsafe extern "C" fn(p: *mut VideoPlayer, n: core::ffi::c_int) -> core::ffi::c_int,
	pub getError: unsafe extern "C" fn(p: *mut VideoPlayer) -> *const core::ffi::c_char,
	pub getInfo: unsafe extern "C" fn(p: *mut VideoPlayer,
	                                  outWidth: *mut core::ffi::c_int,
	                                  outHeight: *mut core::ffi::c_int,
	                                  outFrameRate: *mut core::ffi::c_float,
	                                  outFrameCount: *mut core::ffi::c_int,
	                                  outCurrentFrame: *mut core::ffi::c_int),
	pub getContext: unsafe extern "C" fn(p: *mut VideoPlayer) -> *mut Bitmap,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateVideoStream {
	pub newPlayer: unsafe extern "C" fn() -> *mut StreamPlayer,
	pub freePlayer: unsafe extern "C" fn(p: *mut StreamPlayer),
	pub setBufferSize:
		unsafe extern "C" fn(p: *mut StreamPlayer, video: core::ffi::c_int, audio: core::ffi::c_int),
	pub setFile: unsafe extern "C" fn(p: *mut StreamPlayer, file: *mut SdFile),
	pub setHTTPConnection: unsafe extern "C" fn(p: *mut StreamPlayer, conn: *mut HttpConnection),
	pub getFilePlayer: unsafe extern "C" fn(p: *mut StreamPlayer) -> *mut FilePlayer,
	pub getVideoPlayer: unsafe extern "C" fn(p: *mut StreamPlayer) -> *mut VideoPlayer,
	pub update: unsafe extern "C" fn(p: *mut StreamPlayer) -> bool,
	pub getBufferedFrameCount: unsafe extern "C" fn(p: *mut StreamPlayer) -> core::ffi::c_int,
	pub getBytesRead: unsafe extern "C" fn(p: *mut StreamPlayer) -> u32,
	pub setTCPConnection: unsafe extern "C" fn(p: *mut StreamPlayer, conn: *mut TcpConnection),
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateTilemap {
	pub newTilemap: unsafe extern "C" fn() -> *mut TileMap,
	pub freeTilemap: unsafe extern "C" fn(m: *mut TileMap),
	pub setImageTable: unsafe extern "C" fn(m: *mut TileMap, table: *mut BitmapTable),
	pub getImageTable: unsafe extern "C" fn(m: *mut TileMap) -> *mut BitmapTable,
	pub setSize: unsafe extern "C" fn(m: *mut TileMap, tilesWide: core::ffi::c_int, tilesHigh: core::ffi::c_int),
	pub getSize:
		unsafe extern "C" fn(m: *mut TileMap, tilesWide: *mut core::ffi::c_int, tilesHigh: *mut core::ffi::c_int),
	pub getPixelSize: unsafe extern "C" fn(m: *mut TileMap, outWidth: *mut u32, outHeight: *mut u32),
	pub setTiles: unsafe extern "C" fn(m: *mut TileMap,
	                                   indexes: *mut u16,
	                                   count: core::ffi::c_int,
	                                   rowwidth: core::ffi::c_int),
	pub setTileAtPosition:
		unsafe extern "C" fn(m: *mut TileMap, x: core::ffi::c_int, y: core::ffi::c_int, idx: u16),
	pub getTileAtPosition:
		unsafe extern "C" fn(m: *mut TileMap, x: core::ffi::c_int, y: core::ffi::c_int) -> core::ffi::c_int,
	pub drawAtPoint: unsafe extern "C" fn(m: *mut TileMap, x: core::ffi::c_float, y: core::ffi::c_float),
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateGraphics {
	// pub video: &'static PlaydateVideo,
	pub clear: unsafe extern "C" fn(color: Color),
	// pub setBackgroundColor: unsafe extern "C" fn(color: SolidColor),
	// pub setStencil: unsafe extern "C" fn(stencil: *mut Bitmap),
	pub setDrawMode: unsafe extern "C" fn(mode: BitmapDrawMode) -> BitmapDrawMode,
	// pub setDrawOffset: unsafe extern "C" fn(dx: core::ffi::c_int, dy: core::ffi::c_int),
	// pub setClipRect: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                       y: core::ffi::c_int,
	//                                       width: core::ffi::c_int,
	//                                       height: core::ffi::c_int),
	// pub clearClipRect: unsafe extern "C" fn(),
	// pub setLineCapStyle: unsafe extern "C" fn(endCapStyle: LineCapStyle),
	// pub setFont: unsafe extern "C" fn(font: *mut Font),
	// pub setTextTracking: unsafe extern "C" fn(tracking: core::ffi::c_int),
	// pub pushContext: unsafe extern "C" fn(target: *mut Bitmap),
	// pub popContext: unsafe extern "C" fn(),
	// pub drawBitmap:
	// 	unsafe extern "C" fn(bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int, flip: BitmapFlip),
	// pub tileBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	//                                      x: core::ffi::c_int,
	//                                      y: core::ffi::c_int,
	//                                      width: core::ffi::c_int,
	//                                      height: core::ffi::c_int,
	//                                      flip: BitmapFlip),
	// pub drawLine: unsafe extern "C" fn(x1: core::ffi::c_int,
	//                                    y1: core::ffi::c_int,
	//                                    x2: core::ffi::c_int,
	//                                    y2: core::ffi::c_int,
	//                                    width: core::ffi::c_int,
	//                                    color: Color),
	// pub fillTriangle: unsafe extern "C" fn(x1: core::ffi::c_int,
	//                                        y1: core::ffi::c_int,
	//                                        x2: core::ffi::c_int,
	//                                        y2: core::ffi::c_int,
	//                                        x3: core::ffi::c_int,
	//                                        y3: core::ffi::c_int,
	//                                        color: Color),
	// pub drawRect: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                    y: core::ffi::c_int,
	//                                    width: core::ffi::c_int,
	//                                    height: core::ffi::c_int,
	//                                    color: Color),
	pub fillRect: unsafe extern "C" fn(x: core::ffi::c_int,
	                                   y: core::ffi::c_int,
	                                   width: core::ffi::c_int,
	                                   height: core::ffi::c_int,
	                                   color: Color),
	// pub drawEllipse: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                       y: core::ffi::c_int,
	//                                       width: core::ffi::c_int,
	//                                       height: core::ffi::c_int,
	//                                       lineWidth: core::ffi::c_int,
	//                                       startAngle: core::ffi::c_float,
	//                                       endAngle: core::ffi::c_float,
	//                                       color: Color),
	// pub fillEllipse: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                       y: core::ffi::c_int,
	//                                       width: core::ffi::c_int,
	//                                       height: core::ffi::c_int,
	//                                       startAngle: core::ffi::c_float,
	//                                       endAngle: core::ffi::c_float,
	//                                       color: Color),
	// pub drawScaledBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	//                                            x: core::ffi::c_int,
	//                                            y: core::ffi::c_int,
	//                                            xscale: core::ffi::c_float,
	//                                            yscale: core::ffi::c_float),
	pub drawText: unsafe extern "C" fn(text: *const core::ffi::c_void,
	                                   len: usize,
	                                   encoding: StringEncoding,
	                                   x: core::ffi::c_int,
	                                   y: core::ffi::c_int) -> core::ffi::c_int,
	// pub newBitmap:
	// 	unsafe extern "C" fn(width: core::ffi::c_int, height: core::ffi::c_int, bgcolor: Color) -> *mut Bitmap,
	// pub freeBitmap: unsafe extern "C" fn(arg1: *mut Bitmap),
	// pub loadBitmap:
	// 	unsafe extern "C" fn(path: *const core::ffi::c_char, outerr: *mut *const core::ffi::c_char) -> *mut Bitmap,
	// pub copyBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap) -> *mut Bitmap,
	// pub loadIntoBitmap: unsafe extern "C" fn(path: *const core::ffi::c_char,
	//                                          bitmap: *mut Bitmap,
	//                                          outerr: *mut *const core::ffi::c_char),
	// pub getBitmapData: unsafe extern "C" fn(bitmap: *mut Bitmap,
	//                                         width: *mut core::ffi::c_int,
	//                                         height: *mut core::ffi::c_int,
	//                                         rowbytes: *mut core::ffi::c_int,
	//                                         mask: *mut *mut u8,
	//                                         data: *mut *mut u8),
	// pub clearBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap, bgcolor: Color),
	// pub rotatedBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	//                                         rotation: core::ffi::c_float,
	//                                         xscale: core::ffi::c_float,
	//                                         yscale: core::ffi::c_float,
	//                                         allocedSize: *mut core::ffi::c_int)
	//                                         -> *mut Bitmap,
	// pub newBitmapTable: unsafe extern "C" fn(count: core::ffi::c_int,
	//                                          width: core::ffi::c_int,
	//                                          height: core::ffi::c_int)
	//                                          -> *mut BitmapTable,
	// pub freeBitmapTable: unsafe extern "C" fn(table: *mut BitmapTable),
	// pub loadBitmapTable: unsafe extern "C" fn(path: *const core::ffi::c_char,
	//                                           outerr: *mut *const core::ffi::c_char)
	//                                           -> *mut BitmapTable,
	// pub loadIntoBitmapTable: unsafe extern "C" fn(path: *const core::ffi::c_char,
	//                                               table: *mut BitmapTable,
	//                                               outerr: *mut *const core::ffi::c_char),
	// pub getTableBitmap: unsafe extern "C" fn(table: *mut BitmapTable, idx: core::ffi::c_int) -> *mut Bitmap,
	// pub loadFont:
	// 	unsafe extern "C" fn(path: *const core::ffi::c_char, outErr: *mut *const core::ffi::c_char) -> *mut Font,
	// pub getFontPage: unsafe extern "C" fn(font: *mut Font, c: u32) -> *mut FontPage,
	// pub getPageGlyph: unsafe extern "C" fn(page: *mut FontPage,
	//                                        c: u32,
	//                                        bitmap: *mut *mut Bitmap,
	//                                        advance: *mut core::ffi::c_int)
	//                                        -> *mut FontGlyph,
	// pub getGlyphKerning:
	// 	unsafe extern "C" fn(glyph: *mut FontGlyph, glyphcode: u32, nextcode: u32) -> core::ffi::c_int,
	pub getTextWidth: unsafe extern "C" fn(font: *mut Font,
	                                       text: *const core::ffi::c_void,
	                                       len: usize,
	                                       encoding: StringEncoding,
	                                       tracking: core::ffi::c_int)
	                                       -> core::ffi::c_int,
	// pub getFrame: unsafe extern "C" fn() -> *mut u8,
	// pub getDisplayFrame: unsafe extern "C" fn() -> *mut u8,
	pub getDebugBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut Bitmap>,
	// pub copyFrameBufferBitmap: unsafe extern "C" fn() -> *mut Bitmap,
	// pub markUpdatedRows: unsafe extern "C" fn(start: core::ffi::c_int, end: core::ffi::c_int),
	// pub display: unsafe extern "C" fn(),
	// pub setColorToPattern:
	// 	unsafe extern "C" fn(color: *mut Color, bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int),
	// pub checkMaskCollision: unsafe extern "C" fn(bitmap1: *mut Bitmap,
	//                                              x1: core::ffi::c_int,
	//                                              y1: core::ffi::c_int,
	//                                              flip1: BitmapFlip,
	//                                              bitmap2: *mut Bitmap,
	//                                              x2: core::ffi::c_int,
	//                                              y2: core::ffi::c_int,
	//                                              flip2: BitmapFlip,
	//                                              rect: Aabb)
	//                                              -> core::ffi::c_int,
	// pub setScreenClipRect: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                             y: core::ffi::c_int,
	//                                             width: core::ffi::c_int,
	//                                             height: core::ffi::c_int),
	// pub fillPolygon: unsafe extern "C" fn(nPoints: core::ffi::c_int,
	//                                       coords: *mut core::ffi::c_int,
	//                                       color: Color,
	//                                       fillrule: PolygonFillRule),
	// pub getFontHeight: unsafe extern "C" fn(font: *mut Font) -> u8,
	// pub getDisplayBufferBitmap: unsafe extern "C" fn() -> *mut Bitmap,
	// pub drawRotatedBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap,
	//                                             x: core::ffi::c_int,
	//                                             y: core::ffi::c_int,
	//                                             rotation: core::ffi::c_float,
	//                                             centerx: core::ffi::c_float,
	//                                             centery: core::ffi::c_float,
	//                                             xscale: core::ffi::c_float,
	//                                             yscale: core::ffi::c_float),
	// pub setTextLeading: unsafe extern "C" fn(lineHeightAdustment: core::ffi::c_int),
	// pub setBitmapMask: unsafe extern "C" fn(bitmap: *mut Bitmap, mask: *mut Bitmap) -> core::ffi::c_int,
	// pub getBitmapMask: unsafe extern "C" fn(bitmap: *mut Bitmap) -> *mut Bitmap,
	// pub setStencilImage: unsafe extern "C" fn(stencil: *mut Bitmap, tile: core::ffi::c_int),
	// pub makeFontFromData: unsafe extern "C" fn(data: *mut FontData, wide: core::ffi::c_int) -> *mut Font,
	// pub getTextTracking: unsafe extern "C" fn() -> core::ffi::c_int,
	// pub setPixel: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int, c: Color),
	// pub getBitmapPixel:
	// 	unsafe extern "C" fn(bitmap: *mut Bitmap, x: core::ffi::c_int, y: core::ffi::c_int) -> SolidColor,
	// pub getBitmapTableInfo:
	// 	unsafe extern "C" fn(table: *mut BitmapTable, count: *mut core::ffi::c_int, width: *mut core::ffi::c_int),
	// pub drawTextInRect: unsafe extern "C" fn(text: *const core::ffi::c_void,
	//                                          len: usize,
	//                                          encoding: StringEncoding,
	//                                          x: core::ffi::c_int,
	//                                          y: core::ffi::c_int,
	//                                          width: core::ffi::c_int,
	//                                          height: core::ffi::c_int,
	//                                          wrap: TextWrappingMode,
	//                                          align: TextAlignment),
	// pub getTextHeightForMaxWidth: unsafe extern "C" fn(font: *mut Font,
	//                                                    text: *const core::ffi::c_void,
	//                                                    len: usize,
	//                                                    maxwidth: core::ffi::c_int,
	//                                                    encoding: StringEncoding,
	//                                                    wrap: TextWrappingMode,
	//                                                    tracking: core::ffi::c_int,
	//                                                    extraLeading: core::ffi::c_int)
	//                                                    -> core::ffi::c_int,
	// pub drawRoundRect: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                         y: core::ffi::c_int,
	//                                         width: core::ffi::c_int,
	//                                         height: core::ffi::c_int,
	//                                         radius: core::ffi::c_int,
	//                                         lineWidth: core::ffi::c_int,
	//                                         color: Color),
	// pub fillRoundRect: unsafe extern "C" fn(x: core::ffi::c_int,
	//                                         y: core::ffi::c_int,
	//                                         width: core::ffi::c_int,
	//                                         height: core::ffi::c_int,
	//                                         radius: core::ffi::c_int,
	//                                         color: Color),
	// pub tilemap: &'static PlaydateTilemap,
	// pub videostream: &'static PlaydateVideoStream,
}
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default, :: core :: marker :: ConstParamTy)]
pub struct Buttons(pub u8);
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
pub enum Language {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
pub type AccessRequestCallback =
	::core::option::Option<unsafe extern "C" fn(allowed: bool, userdata: *mut core::ffi::c_void)>;
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum AccessReply {
	AccessAsk = 0,
	AccessDeny = 1,
	AccessAllow = 2,
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct MenuItem {
	_unused: [u8; 0],
}
#[repr(u16)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
pub struct PlaydateSys {
	pub realloc: unsafe extern "C" fn(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void,
	// pub formatString : unsafe extern "C" fn (ret : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int ,
	pub logToConsole: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...),
	pub error: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...) -> !,
	// pub getLanguage : unsafe extern "C" fn () -> Language ,
	// pub getCurrentTimeMilliseconds : unsafe extern "C" fn () -> core :: ffi :: c_uint ,
	// pub getSecondsSinceEpoch : unsafe extern "C" fn (milliseconds : * mut core :: ffi :: c_uint) -> core :: ffi :: c_uint ,
	pub drawFPS: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
	pub setUpdateCallback: unsafe extern "C" fn(update: CallbackFunction, userdata: *mut core::ffi::c_void),
	pub getButtonState: unsafe extern "C" fn(current: *mut Buttons, pushed: *mut Buttons, released: *mut Buttons),
	// pub setPeripheralsEnabled : unsafe extern "C" fn (mask : Peripherals) ,
	// pub getAccelerometer : unsafe extern "C" fn (outx : * mut core :: ffi :: c_float , outy : * mut core :: ffi :: c_float , outz : * mut core :: ffi :: c_float) ,
	// pub getCrankChange : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// pub getCrankAngle : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// pub isCrankDocked : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// pub setCrankSoundsDisabled : unsafe extern "C" fn (flag : core :: ffi :: c_int) -> core :: ffi :: c_int ,
	// pub getFlipped : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// pub setAutoLockDisabled : unsafe extern "C" fn (disable : core :: ffi :: c_int) ,
	// pub setMenuImage : unsafe extern "C" fn (bitmap : * mut Bitmap , xOffset : core :: ffi :: c_int) ,
	// pub addMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , callback : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem ,
	// pub addCheckmarkMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , value : core :: ffi :: c_int , callback : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem ,
	// pub addOptionsMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , optionTitles : * mut * const core :: ffi :: c_char , optionsCount : core :: ffi :: c_int , f : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem ,
	// pub removeAllMenuItems : unsafe extern "C" fn () ,
	// pub removeMenuItem : unsafe extern "C" fn (menuItem : * mut MenuItem) ,
	// pub getMenuItemValue : unsafe extern "C" fn (menuItem : * mut MenuItem) -> core :: ffi :: c_int ,
	// pub setMenuItemValue : unsafe extern "C" fn (menuItem : * mut MenuItem , value : core :: ffi :: c_int) ,
	// pub getMenuItemTitle : unsafe extern "C" fn (menuItem : * mut MenuItem) -> * const core :: ffi :: c_char ,
	// pub setMenuItemTitle : unsafe extern "C" fn (menuItem : * mut MenuItem , title : * const core :: ffi :: c_char) ,
	// pub getMenuItemUserdata : unsafe extern "C" fn (menuItem : * mut MenuItem) -> * mut core :: ffi :: c_void ,
	// pub setMenuItemUserdata : unsafe extern "C" fn (menuItem : * mut MenuItem , ud : * mut core :: ffi :: c_void) ,
	// pub getReduceFlashing : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// pub getElapsedTime : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// pub resetElapsedTime : unsafe extern "C" fn () ,
	// pub getBatteryPercentage : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// pub getBatteryVoltage : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// pub getTimezoneOffset : unsafe extern "C" fn () -> i32 ,
	// pub shouldDisplay24HourTime : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// pub convertEpochToDateTime : unsafe extern "C" fn (epoch : u32 , datetime : * mut DateTime) ,
	// pub convertDateTimeToEpoch : unsafe extern "C" fn (datetime : * mut DateTime) -> u32 ,
	// pub clearICache : unsafe extern "C" fn () ,
	// pub setButtonCallback : unsafe extern "C" fn (cb : ButtonCallbackFunction , buttonud : * mut core :: ffi :: c_void , queuesize : core :: ffi :: c_int) ,
	// pub setSerialMessageCallback : unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (data : * const core :: ffi :: c_char) >) ,
	// pub vaFormatString : unsafe extern "C" fn (outstr : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , args : va_list) -> core :: ffi :: c_int ,
	// pub parseString : unsafe extern "C" fn (str_ : * const core :: ffi :: c_char , format : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int ,
	// pub delay : unsafe extern "C" fn (milliseconds : u32) ,
	// pub getServerTime : unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (time : * const core :: ffi :: c_char , err : * const core :: ffi :: c_char) >) ,
	// pub sendMirrorData : unsafe extern "C" fn (command : u8 , data : * mut core :: ffi :: c_void , len : core :: ffi :: c_int) -> bool
}
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateLua {
	pub addFunction: unsafe extern "C" fn(f: LuaCFunction,
	                                      name: *const core::ffi::c_char,
	                                      outErr: *mut *const core::ffi::c_char)
	                                      -> core::ffi::c_int,
	pub registerClass: unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                        reg: *const LuaReg,
	                                        vals: *const LuaVal,
	                                        isstatic: core::ffi::c_int,
	                                        outErr: *mut *const core::ffi::c_char)
	                                        -> core::ffi::c_int,
	pub pushFunction: unsafe extern "C" fn(f: LuaCFunction),
	pub indexMetatable: unsafe extern "C" fn() -> core::ffi::c_int,
	pub stop: unsafe extern "C" fn(),
	pub start: unsafe extern "C" fn(),
	pub getArgCount: unsafe extern "C" fn() -> core::ffi::c_int,
	pub getArgType:
		unsafe extern "C" fn(pos: core::ffi::c_int, outClass: *mut *const core::ffi::c_char) -> LuaType,
	pub argIsNil: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	pub getArgBool: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	pub getArgInt: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_int,
	pub getArgFloat: unsafe extern "C" fn(pos: core::ffi::c_int) -> core::ffi::c_float,
	pub getArgString: unsafe extern "C" fn(pos: core::ffi::c_int) -> *const core::ffi::c_char,
	pub getArgBytes: unsafe extern "C" fn(pos: core::ffi::c_int, outlen: *mut usize) -> *const core::ffi::c_char,
	pub getArgObject: unsafe extern "C" fn(pos: core::ffi::c_int,
	                                       type_: *mut core::ffi::c_char,
	                                       outud: *mut *mut LuaUdObject)
	                                       -> *mut core::ffi::c_void,
	pub getBitmap: unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut Bitmap,
	pub getSprite: unsafe extern "C" fn(pos: core::ffi::c_int) -> *mut Sprite,
	pub pushNil: unsafe extern "C" fn(),
	pub pushBool: unsafe extern "C" fn(val: core::ffi::c_int),
	pub pushInt: unsafe extern "C" fn(val: core::ffi::c_int),
	pub pushFloat: unsafe extern "C" fn(val: core::ffi::c_float),
	pub pushString: unsafe extern "C" fn(str_: *const core::ffi::c_char),
	pub pushBytes: unsafe extern "C" fn(str_: *const core::ffi::c_char, len: usize),
	pub pushBitmap: unsafe extern "C" fn(bitmap: *mut Bitmap),
	pub pushSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	pub pushObject: unsafe extern "C" fn(obj: *mut core::ffi::c_void,
	                                     type_: *mut core::ffi::c_char,
	                                     nValues: core::ffi::c_int)
	                                     -> *mut LuaUdObject,
	pub retainObject: unsafe extern "C" fn(obj: *mut LuaUdObject) -> *mut LuaUdObject,
	pub releaseObject: unsafe extern "C" fn(obj: *mut LuaUdObject),
	pub setUserValue: unsafe extern "C" fn(obj: *mut LuaUdObject, slot: core::ffi::c_uint),
	pub getUserValue: unsafe extern "C" fn(obj: *mut LuaUdObject, slot: core::ffi::c_uint) -> core::ffi::c_int,
	pub callFunction_deprecated: unsafe extern "C" fn(name: *const core::ffi::c_char, nargs: core::ffi::c_int),
	pub callFunction: unsafe extern "C" fn(name: *const core::ffi::c_char,
	                                       nargs: core::ffi::c_int,
	                                       outerr: *mut *const core::ffi::c_char)
	                                       -> core::ffi::c_int,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
	pub initEncoder: unsafe extern "C" fn(encoder: *mut JsonEncoder,
	                                      write: JsonWriteFunc,
	                                      userdata: *mut core::ffi::c_void,
	                                      pretty: core::ffi::c_int),
	pub decode: unsafe extern "C" fn(functions: *mut JsonDecoder,
	                                 reader: JsonReader,
	                                 outval: *mut JsonValue) -> core::ffi::c_int,
	pub decodeString: unsafe extern "C" fn(functions: *mut JsonDecoder,
	                                       jsonString: *const core::ffi::c_char,
	                                       outval: *mut JsonValue)
	                                       -> core::ffi::c_int,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
#[must_use]
pub struct CollisionPoint {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
#[must_use]
pub struct CollisionVector {
	pub x: core::ffi::c_int,
	pub y: core::ffi::c_int,
}
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
	pub setAlwaysRedraw: unsafe extern "C" fn(flag: core::ffi::c_int),
	pub addDirtyRect: unsafe extern "C" fn(dirtyRect: Aabb),
	pub drawSprites: unsafe extern "C" fn(),
	pub updateAndDrawSprites: unsafe extern "C" fn(),
	pub newSprite: unsafe extern "C" fn() -> *mut Sprite,
	pub freeSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	pub copy: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut Sprite,
	pub addSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	pub removeSprite: unsafe extern "C" fn(sprite: *mut Sprite),
	pub removeSprites: unsafe extern "C" fn(sprites: *mut *mut Sprite, count: core::ffi::c_int),
	pub removeAllSprites: unsafe extern "C" fn(),
	pub getSpriteCount: unsafe extern "C" fn() -> core::ffi::c_int,
	pub setBounds: unsafe extern "C" fn(sprite: *mut Sprite, bounds: Rect),
	pub getBounds: unsafe extern "C" fn(sprite: *mut Sprite) -> Rect,
	pub moveTo: unsafe extern "C" fn(sprite: *mut Sprite, x: core::ffi::c_float, y: core::ffi::c_float),
	pub moveBy: unsafe extern "C" fn(sprite: *mut Sprite, dx: core::ffi::c_float, dy: core::ffi::c_float),
	pub setImage: unsafe extern "C" fn(sprite: *mut Sprite, image: *mut Bitmap, flip: BitmapFlip),
	pub getImage: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut Bitmap,
	pub setSize: unsafe extern "C" fn(s: *mut Sprite, width: core::ffi::c_float, height: core::ffi::c_float),
	pub setZIndex: unsafe extern "C" fn(sprite: *mut Sprite, zIndex: i16),
	pub getZIndex: unsafe extern "C" fn(sprite: *mut Sprite) -> i16,
	pub setDrawMode: unsafe extern "C" fn(sprite: *mut Sprite, mode: BitmapDrawMode),
	pub setImageFlip: unsafe extern "C" fn(sprite: *mut Sprite, flip: BitmapFlip),
	pub getImageFlip: unsafe extern "C" fn(sprite: *mut Sprite) -> BitmapFlip,
	pub setStencil: unsafe extern "C" fn(sprite: *mut Sprite, stencil: *mut Bitmap),
	pub setClipRect: unsafe extern "C" fn(sprite: *mut Sprite, clipRect: Aabb),
	pub clearClipRect: unsafe extern "C" fn(sprite: *mut Sprite),
	pub setClipRectsInRange:
		unsafe extern "C" fn(clipRect: Aabb, startZ: core::ffi::c_int, endZ: core::ffi::c_int),
	pub clearClipRectsInRange: unsafe extern "C" fn(startZ: core::ffi::c_int, endZ: core::ffi::c_int),
	pub setUpdatesEnabled: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	pub updatesEnabled: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	pub setCollisionsEnabled: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	pub collisionsEnabled: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	pub setVisible: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	pub isVisible: unsafe extern "C" fn(sprite: *mut Sprite) -> core::ffi::c_int,
	pub setOpaque: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	pub markDirty: unsafe extern "C" fn(sprite: *mut Sprite),
	pub setTag: unsafe extern "C" fn(sprite: *mut Sprite, tag: u8),
	pub getTag: unsafe extern "C" fn(sprite: *mut Sprite) -> u8,
	pub setIgnoresDrawOffset: unsafe extern "C" fn(sprite: *mut Sprite, flag: core::ffi::c_int),
	pub setUpdateFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteUpdateFunction),
	pub setDrawFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteDrawFunction),
	pub getPosition:
		unsafe extern "C" fn(sprite: *mut Sprite, x: *mut core::ffi::c_float, y: *mut core::ffi::c_float),
	pub resetCollisionWorld: unsafe extern "C" fn(),
	pub setCollideRect: unsafe extern "C" fn(sprite: *mut Sprite, collideRect: Rect),
	pub getCollideRect: unsafe extern "C" fn(sprite: *mut Sprite) -> Rect,
	pub clearCollideRect: unsafe extern "C" fn(sprite: *mut Sprite),
	pub setCollisionResponseFunction: unsafe extern "C" fn(sprite: *mut Sprite, func: SpriteCollisionFilterProc),
	pub checkCollisions: unsafe extern "C" fn(sprite: *mut Sprite,
	                                          goalX: core::ffi::c_float,
	                                          goalY: core::ffi::c_float,
	                                          actualX: *mut core::ffi::c_float,
	                                          actualY: *mut core::ffi::c_float,
	                                          len: *mut core::ffi::c_int)
	                                          -> *mut SpriteCollisionInfo,
	pub moveWithCollisions: unsafe extern "C" fn(sprite: *mut Sprite,
	                                             goalX: core::ffi::c_float,
	                                             goalY: core::ffi::c_float,
	                                             actualX: *mut core::ffi::c_float,
	                                             actualY: *mut core::ffi::c_float,
	                                             len: *mut core::ffi::c_int)
	                                             -> *mut SpriteCollisionInfo,
	pub querySpritesAtPoint: unsafe extern "C" fn(x: core::ffi::c_float,
	                                              y: core::ffi::c_float,
	                                              len: *mut core::ffi::c_int)
	                                              -> *mut *mut Sprite,
	pub querySpritesInRect: unsafe extern "C" fn(x: core::ffi::c_float,
	                                             y: core::ffi::c_float,
	                                             width: core::ffi::c_float,
	                                             height: core::ffi::c_float,
	                                             len: *mut core::ffi::c_int)
	                                             -> *mut *mut Sprite,
	pub querySpritesAlongLine: unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                y1: core::ffi::c_float,
	                                                x2: core::ffi::c_float,
	                                                y2: core::ffi::c_float,
	                                                len: *mut core::ffi::c_int)
	                                                -> *mut *mut Sprite,
	pub querySpriteInfoAlongLine: unsafe extern "C" fn(x1: core::ffi::c_float,
	                                                   y1: core::ffi::c_float,
	                                                   x2: core::ffi::c_float,
	                                                   y2: core::ffi::c_float,
	                                                   len: *mut core::ffi::c_int)
	                                                   -> *mut SpriteQueryInfo,
	pub overlappingSprites:
		unsafe extern "C" fn(sprite: *mut Sprite, len: *mut core::ffi::c_int) -> *mut *mut Sprite,
	pub allOverlappingSprites: unsafe extern "C" fn(len: *mut core::ffi::c_int) -> *mut *mut Sprite,
	pub setStencilPattern: unsafe extern "C" fn(sprite: *mut Sprite, pattern: *mut [u8; 8usize]),
	pub clearStencil: unsafe extern "C" fn(sprite: *mut Sprite),
	pub setUserdata: unsafe extern "C" fn(sprite: *mut Sprite, userdata: *mut core::ffi::c_void),
	pub getUserdata: unsafe extern "C" fn(sprite: *mut Sprite) -> *mut core::ffi::c_void,
	pub setStencilImage: unsafe extern "C" fn(sprite: *mut Sprite, stencil: *mut Bitmap, tile: core::ffi::c_int),
	pub setCenter: unsafe extern "C" fn(s: *mut Sprite, x: core::ffi::c_float, y: core::ffi::c_float),
	pub getCenter: unsafe extern "C" fn(s: *mut Sprite, x: *mut core::ffi::c_float, y: *mut core::ffi::c_float),
	pub setTilemap: unsafe extern "C" fn(s: *mut Sprite, tilemap: *mut TileMap),
	pub getTilemap: unsafe extern "C" fn(s: *mut Sprite) -> *mut TileMap,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
	pub setVolume: unsafe extern "C" fn(c: *mut SoundSource, lvol: core::ffi::c_float, rvol: core::ffi::c_float),
	pub getVolume:
		unsafe extern "C" fn(c: *mut SoundSource, outl: *mut core::ffi::c_float, outr: *mut core::ffi::c_float),
	pub isPlaying: unsafe extern "C" fn(c: *mut SoundSource) -> core::ffi::c_int,
	pub setFinishCallback:
		unsafe extern "C" fn(c: *mut SoundSource, callback: SndCallbackProc, userdata: *mut core::ffi::c_void),
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundFileplayer { pub newPlayer : unsafe extern "C" fn () -> * mut FilePlayer , pub freePlayer : unsafe extern "C" fn (player : * mut FilePlayer) , pub loadIntoPlayer : unsafe extern "C" fn (player : * mut FilePlayer , path : * const core :: ffi :: c_char) -> core :: ffi :: c_int , pub setBufferLength : unsafe extern "C" fn (player : * mut FilePlayer , bufferLen : core :: ffi :: c_float) , pub play : unsafe extern "C" fn (player : * mut FilePlayer , repeat : core :: ffi :: c_int) -> core :: ffi :: c_int , pub isPlaying : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int , pub pause : unsafe extern "C" fn (player : * mut FilePlayer) , pub stop : unsafe extern "C" fn (player : * mut FilePlayer) , pub setVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float) , pub getVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : * mut core :: ffi :: c_float , right : * mut core :: ffi :: c_float) , pub getLength : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , pub setOffset : unsafe extern "C" fn (player : * mut FilePlayer , offset : core :: ffi :: c_float) , pub setRate : unsafe extern "C" fn (player : * mut FilePlayer , rate : core :: ffi :: c_float) , pub setLoopRange : unsafe extern "C" fn (player : * mut FilePlayer , start : core :: ffi :: c_float , end : core :: ffi :: c_float) , pub didUnderrun : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int , pub setFinishCallback : unsafe extern "C" fn (player : * mut FilePlayer , callback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub setLoopCallback : unsafe extern "C" fn (player : * mut FilePlayer , callback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub getOffset : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , pub getRate : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , pub setStopOnUnderrun : unsafe extern "C" fn (player : * mut FilePlayer , flag : core :: ffi :: c_int) , pub fadeVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float , len : i32 , finishCallback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub setMP3StreamSource : unsafe extern "C" fn (player : * mut FilePlayer , dataSource : :: core :: option :: Option < unsafe extern "C" fn (data : * mut u8 , bytes : core :: ffi :: c_int , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_int > , userdata : * mut core :: ffi :: c_void , bufferLen : core :: ffi :: c_float) , }
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
	pub newSampleBuffer: unsafe extern "C" fn(byteCount: core::ffi::c_int) -> *mut AudioSample,
	pub loadIntoSample:
		unsafe extern "C" fn(sample: *mut AudioSample, path: *const core::ffi::c_char) -> core::ffi::c_int,
	pub load: unsafe extern "C" fn(path: *const core::ffi::c_char) -> *mut AudioSample,
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
	pub freeSample: unsafe extern "C" fn(sample: *mut AudioSample),
	pub getLength: unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_float,
	pub decompress: unsafe extern "C" fn(sample: *mut AudioSample) -> core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSampleplayer {
	pub newPlayer: unsafe extern "C" fn() -> *mut SamplePlayer,
	pub freePlayer: unsafe extern "C" fn(player: *mut SamplePlayer),
	pub setSample: unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample),
	pub play: unsafe extern "C" fn(player: *mut SamplePlayer,
	                               repeat: core::ffi::c_int,
	                               rate: core::ffi::c_float) -> core::ffi::c_int,
	pub isPlaying: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_int,
	pub stop: unsafe extern "C" fn(player: *mut SamplePlayer),
	pub setVolume:
		unsafe extern "C" fn(player: *mut SamplePlayer, left: core::ffi::c_float, right: core::ffi::c_float),
	pub getVolume: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                    left: *mut core::ffi::c_float,
	                                    right: *mut core::ffi::c_float),
	pub getLength: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	pub setOffset: unsafe extern "C" fn(player: *mut SamplePlayer, offset: core::ffi::c_float),
	pub setRate: unsafe extern "C" fn(player: *mut SamplePlayer, rate: core::ffi::c_float),
	pub setPlayRange:
		unsafe extern "C" fn(player: *mut SamplePlayer, start: core::ffi::c_int, end: core::ffi::c_int),
	pub setFinishCallback: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                            callback: SndCallbackProc,
	                                            userdata: *mut core::ffi::c_void),
	pub setLoopCallback: unsafe extern "C" fn(player: *mut SamplePlayer,
	                                          callback: SndCallbackProc,
	                                          userdata: *mut core::ffi::c_void),
	pub getOffset: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	pub getRate: unsafe extern "C" fn(player: *mut SamplePlayer) -> core::ffi::c_float,
	pub setPaused: unsafe extern "C" fn(player: *mut SamplePlayer, flag: core::ffi::c_int),
}
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
	pub newSignal: unsafe extern "C" fn(step: SignalStepFunc,
	                                    noteOn: SignalNoteOnFunc,
	                                    noteOff: SignalNoteOffFunc,
	                                    dealloc: SignalDeallocFunc,
	                                    userdata: *mut core::ffi::c_void)
	                                    -> *mut SynthSignal,
	pub freeSignal: unsafe extern "C" fn(signal: *mut SynthSignal),
	pub getValue: unsafe extern "C" fn(signal: *mut SynthSignal) -> core::ffi::c_float,
	pub setValueScale: unsafe extern "C" fn(signal: *mut SynthSignal, scale: core::ffi::c_float),
	pub setValueOffset: unsafe extern "C" fn(signal: *mut SynthSignal, offset: core::ffi::c_float),
	pub newSignalForValue: unsafe extern "C" fn(value: *mut SynthSignalValue) -> *mut SynthSignal,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
pub struct PlaydateSoundLfo { pub newLFO : unsafe extern "C" fn (type_ : LfoType) -> * mut SynthLfo , pub freeLFO : unsafe extern "C" fn (lfo : * mut SynthLfo) , pub setType : unsafe extern "C" fn (lfo : * mut SynthLfo , type_ : LfoType) , pub setRate : unsafe extern "C" fn (lfo : * mut SynthLfo , rate : core :: ffi :: c_float) , pub setPhase : unsafe extern "C" fn (lfo : * mut SynthLfo , phase : core :: ffi :: c_float) , pub setCenter : unsafe extern "C" fn (lfo : * mut SynthLfo , center : core :: ffi :: c_float) , pub setDepth : unsafe extern "C" fn (lfo : * mut SynthLfo , depth : core :: ffi :: c_float) , pub setArpeggiation : unsafe extern "C" fn (lfo : * mut SynthLfo , nSteps : core :: ffi :: c_int , steps : * mut core :: ffi :: c_float) , pub setFunction : unsafe extern "C" fn (lfo : * mut SynthLfo , lfoFunc : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut SynthLfo , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_float > , userdata : * mut core :: ffi :: c_void , interpolate : core :: ffi :: c_int) , pub setDelay : unsafe extern "C" fn (lfo : * mut SynthLfo , holdoff : core :: ffi :: c_float , ramptime : core :: ffi :: c_float) , pub setRetrigger : unsafe extern "C" fn (lfo : * mut SynthLfo , flag : core :: ffi :: c_int) , pub getValue : unsafe extern "C" fn (lfo : * mut SynthLfo) -> core :: ffi :: c_float , pub setGlobal : unsafe extern "C" fn (lfo : * mut SynthLfo , global : core :: ffi :: c_int) , pub setStartPhase : unsafe extern "C" fn (lfo : * mut SynthLfo , phase : core :: ffi :: c_float) , }
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
	pub newEnvelope: unsafe extern "C" fn(attack: core::ffi::c_float,
	                                      decay: core::ffi::c_float,
	                                      sustain: core::ffi::c_float,
	                                      release: core::ffi::c_float)
	                                      -> *mut SynthEnvelope,
	pub freeEnvelope: unsafe extern "C" fn(env: *mut SynthEnvelope),
	pub setAttack: unsafe extern "C" fn(env: *mut SynthEnvelope, attack: core::ffi::c_float),
	pub setDecay: unsafe extern "C" fn(env: *mut SynthEnvelope, decay: core::ffi::c_float),
	pub setSustain: unsafe extern "C" fn(env: *mut SynthEnvelope, sustain: core::ffi::c_float),
	pub setRelease: unsafe extern "C" fn(env: *mut SynthEnvelope, release: core::ffi::c_float),
	pub setLegato: unsafe extern "C" fn(env: *mut SynthEnvelope, flag: core::ffi::c_int),
	pub setRetrigger: unsafe extern "C" fn(lfo: *mut SynthEnvelope, flag: core::ffi::c_int),
	pub getValue: unsafe extern "C" fn(env: *mut SynthEnvelope) -> core::ffi::c_float,
	pub setCurvature: unsafe extern "C" fn(env: *mut SynthEnvelope, amount: core::ffi::c_float),
	pub setVelocitySensitivity: unsafe extern "C" fn(env: *mut SynthEnvelope, velsens: core::ffi::c_float),
	pub setRateScaling:
		unsafe extern "C" fn(env: *mut SynthEnvelope, scaling: core::ffi::c_float, start: MidiNote, end: MidiNote),
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
	pub newSynth: unsafe extern "C" fn() -> *mut Synth,
	pub freeSynth: unsafe extern "C" fn(synth: *mut Synth),
	pub setWaveform: unsafe extern "C" fn(synth: *mut Synth, wave: SoundWaveform),
	pub setGenerator_deprecated: unsafe extern "C" fn(synth: *mut Synth,
	                                                  stereo: core::ffi::c_int,
	                                                  render: SynthRenderFunc,
	                                                  noteOn: SynthNoteOnFunc,
	                                                  release: SynthReleaseFunc,
	                                                  setparam: SynthSetParameterFunc,
	                                                  dealloc: SynthDeallocFunc,
	                                                  userdata: *mut core::ffi::c_void),
	pub setSample:
		unsafe extern "C" fn(synth: *mut Synth, sample: *mut AudioSample, sustainStart: u32, sustainEnd: u32),
	pub setAttackTime: unsafe extern "C" fn(synth: *mut Synth, attack: core::ffi::c_float),
	pub setDecayTime: unsafe extern "C" fn(synth: *mut Synth, decay: core::ffi::c_float),
	pub setSustainLevel: unsafe extern "C" fn(synth: *mut Synth, sustain: core::ffi::c_float),
	pub setReleaseTime: unsafe extern "C" fn(synth: *mut Synth, release: core::ffi::c_float),
	pub setTranspose: unsafe extern "C" fn(synth: *mut Synth, halfSteps: core::ffi::c_float),
	pub setFrequencyModulator: unsafe extern "C" fn(synth: *mut Synth, mod_: *mut SynthSignalValue),
	pub getFrequencyModulator: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthSignalValue,
	pub setAmplitudeModulator: unsafe extern "C" fn(synth: *mut Synth, mod_: *mut SynthSignalValue),
	pub getAmplitudeModulator: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthSignalValue,
	pub getParameterCount: unsafe extern "C" fn(synth: *mut Synth) -> core::ffi::c_int,
	pub setParameter: unsafe extern "C" fn(synth: *mut Synth,
	                                       parameter: core::ffi::c_int,
	                                       value: core::ffi::c_float)
	                                       -> core::ffi::c_int,
	pub setParameterModulator:
		unsafe extern "C" fn(synth: *mut Synth, parameter: core::ffi::c_int, mod_: *mut SynthSignalValue),
	pub getParameterModulator:
		unsafe extern "C" fn(synth: *mut Synth, parameter: core::ffi::c_int) -> *mut SynthSignalValue,
	pub playNote: unsafe extern "C" fn(synth: *mut Synth,
	                                   freq: core::ffi::c_float,
	                                   vel: core::ffi::c_float,
	                                   len: core::ffi::c_float,
	                                   when: u32),
	pub playMIDINote: unsafe extern "C" fn(synth: *mut Synth,
	                                       note: MidiNote,
	                                       vel: core::ffi::c_float,
	                                       len: core::ffi::c_float,
	                                       when: u32),
	pub noteOff: unsafe extern "C" fn(synth: *mut Synth, when: u32),
	pub stop: unsafe extern "C" fn(synth: *mut Synth),
	pub setVolume: unsafe extern "C" fn(synth: *mut Synth, left: core::ffi::c_float, right: core::ffi::c_float),
	pub getVolume:
		unsafe extern "C" fn(synth: *mut Synth, left: *mut core::ffi::c_float, right: *mut core::ffi::c_float),
	pub isPlaying: unsafe extern "C" fn(synth: *mut Synth) -> core::ffi::c_int,
	pub getEnvelope: unsafe extern "C" fn(synth: *mut Synth) -> *mut SynthEnvelope,
	pub setWavetable: unsafe extern "C" fn(synth: *mut Synth,
	                                       sample: *mut AudioSample,
	                                       log2size: core::ffi::c_int,
	                                       columns: core::ffi::c_int,
	                                       rows: core::ffi::c_int)
	                                       -> core::ffi::c_int,
	pub setGenerator: unsafe extern "C" fn(synth: *mut Synth,
	                                       stereo: core::ffi::c_int,
	                                       render: SynthRenderFunc,
	                                       noteOn: SynthNoteOnFunc,
	                                       release: SynthReleaseFunc,
	                                       setparam: SynthSetParameterFunc,
	                                       dealloc: SynthDeallocFunc,
	                                       copyUserdata: SynthCopyUserdata,
	                                       userdata: *mut core::ffi::c_void),
	pub copy: unsafe extern "C" fn(synth: *mut Synth) -> *mut Synth,
	pub clearEnvelope: unsafe extern "C" fn(synth: *mut Synth),
}
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
	pub newSignal: unsafe extern "C" fn() -> *mut ControlSignal,
	pub freeSignal: unsafe extern "C" fn(signal: *mut ControlSignal),
	pub clearEvents: unsafe extern "C" fn(control: *mut ControlSignal),
	pub addEvent: unsafe extern "C" fn(control: *mut ControlSignal,
	                                   step: core::ffi::c_int,
	                                   value: core::ffi::c_float,
	                                   interpolate: core::ffi::c_int),
	pub removeEvent: unsafe extern "C" fn(control: *mut ControlSignal, step: core::ffi::c_int),
	pub getMIDIControllerNumber: unsafe extern "C" fn(control: *mut ControlSignal) -> core::ffi::c_int,
}
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
	pub newInstrument: unsafe extern "C" fn() -> *mut SynthInstrument,
	pub freeInstrument: unsafe extern "C" fn(inst: *mut SynthInstrument),
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
	pub playMIDINote: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                       note: MidiNote,
	                                       vel: core::ffi::c_float,
	                                       len: core::ffi::c_float,
	                                       when: u32) -> *mut Synth,
	pub setPitchBend: unsafe extern "C" fn(inst: *mut SynthInstrument, bend: core::ffi::c_float),
	pub setPitchBendRange: unsafe extern "C" fn(inst: *mut SynthInstrument, halfSteps: core::ffi::c_float),
	pub setTranspose: unsafe extern "C" fn(inst: *mut SynthInstrument, halfSteps: core::ffi::c_float),
	pub noteOff: unsafe extern "C" fn(inst: *mut SynthInstrument, note: MidiNote, when: u32),
	pub allNotesOff: unsafe extern "C" fn(inst: *mut SynthInstrument, when: u32),
	pub setVolume:
		unsafe extern "C" fn(inst: *mut SynthInstrument, left: core::ffi::c_float, right: core::ffi::c_float),
	pub getVolume: unsafe extern "C" fn(inst: *mut SynthInstrument,
	                                    left: *mut core::ffi::c_float,
	                                    right: *mut core::ffi::c_float),
	pub activeVoiceCount: unsafe extern "C" fn(inst: *mut SynthInstrument) -> core::ffi::c_int,
}
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
	pub newTrack: unsafe extern "C" fn() -> *mut SequenceTrack,
	pub freeTrack: unsafe extern "C" fn(track: *mut SequenceTrack),
	pub setInstrument: unsafe extern "C" fn(track: *mut SequenceTrack, inst: *mut SynthInstrument),
	pub getInstrument: unsafe extern "C" fn(track: *mut SequenceTrack) -> *mut SynthInstrument,
	pub addNoteEvent: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                       step: u32,
	                                       len: u32,
	                                       note: MidiNote,
	                                       velocity: core::ffi::c_float),
	pub removeNoteEvent: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32, note: MidiNote),
	pub clearNotes: unsafe extern "C" fn(track: *mut SequenceTrack),
	pub getControlSignalCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	pub getControlSignal:
		unsafe extern "C" fn(track: *mut SequenceTrack, idx: core::ffi::c_int) -> *mut ControlSignal,
	pub clearControlEvents: unsafe extern "C" fn(track: *mut SequenceTrack),
	pub getPolyphony: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	pub activeVoiceCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> core::ffi::c_int,
	pub setMuted: unsafe extern "C" fn(track: *mut SequenceTrack, mute: core::ffi::c_int),
	pub getLength: unsafe extern "C" fn(track: *mut SequenceTrack) -> u32,
	pub getIndexForStep: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32) -> core::ffi::c_int,
	pub getNoteAtIndex: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                         index: core::ffi::c_int,
	                                         outStep: *mut u32,
	                                         outLen: *mut u32,
	                                         outNote: *mut MidiNote,
	                                         outVelocity: *mut core::ffi::c_float)
	                                         -> core::ffi::c_int,
	pub getSignalForController: unsafe extern "C" fn(track: *mut SequenceTrack,
	                                                 controller: core::ffi::c_int,
	                                                 create: core::ffi::c_int)
	                                                 -> *mut ControlSignal,
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundSequence {
	pub newSequence: unsafe extern "C" fn() -> *mut SoundSequence,
	pub freeSequence: unsafe extern "C" fn(sequence: *mut SoundSequence),
	pub loadMIDIFile:
		unsafe extern "C" fn(seq: *mut SoundSequence, path: *const core::ffi::c_char) -> core::ffi::c_int,
	pub getTime: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
	pub setTime: unsafe extern "C" fn(seq: *mut SoundSequence, time: u32),
	pub setLoops: unsafe extern "C" fn(seq: *mut SoundSequence,
	                                   loopstart: core::ffi::c_int,
	                                   loopend: core::ffi::c_int,
	                                   loops: core::ffi::c_int),
	pub getTempo_deprecated: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	pub setTempo: unsafe extern "C" fn(seq: *mut SoundSequence, stepsPerSecond: core::ffi::c_float),
	pub getTrackCount: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	pub addTrack: unsafe extern "C" fn(seq: *mut SoundSequence) -> *mut SequenceTrack,
	pub getTrackAtIndex:
		unsafe extern "C" fn(seq: *mut SoundSequence, track: core::ffi::c_uint) -> *mut SequenceTrack,
	pub setTrackAtIndex:
		unsafe extern "C" fn(seq: *mut SoundSequence, track: *mut SequenceTrack, idx: core::ffi::c_uint),
	pub allNotesOff: unsafe extern "C" fn(seq: *mut SoundSequence),
	pub isPlaying: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_int,
	pub getLength: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
	pub play: unsafe extern "C" fn(seq: *mut SoundSequence,
	                               finishCallback: SequenceFinishedCallback,
	                               userdata: *mut core::ffi::c_void),
	pub stop: unsafe extern "C" fn(seq: *mut SoundSequence),
	pub getCurrentStep:
		unsafe extern "C" fn(seq: *mut SoundSequence, timeOffset: *mut core::ffi::c_int) -> core::ffi::c_int,
	pub setCurrentStep: unsafe extern "C" fn(seq: *mut SoundSequence,
	                                         step: core::ffi::c_int,
	                                         timeOffset: core::ffi::c_int,
	                                         playNotes: core::ffi::c_int),
	pub getTempo: unsafe extern "C" fn(seq: *mut SoundSequence) -> core::ffi::c_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct TwoPoleFilter {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
	pub newFilter: unsafe extern "C" fn() -> *mut TwoPoleFilter,
	pub freeFilter: unsafe extern "C" fn(filter: *mut TwoPoleFilter),
	pub setType: unsafe extern "C" fn(filter: *mut TwoPoleFilter, type_: TwoPoleFilterType),
	pub setFrequency: unsafe extern "C" fn(filter: *mut TwoPoleFilter, frequency: core::ffi::c_float),
	pub setFrequencyModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut SynthSignalValue),
	pub getFrequencyModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut SynthSignalValue,
	pub setGain: unsafe extern "C" fn(filter: *mut TwoPoleFilter, gain: core::ffi::c_float),
	pub setResonance: unsafe extern "C" fn(filter: *mut TwoPoleFilter, resonance: core::ffi::c_float),
	pub setResonanceModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut SynthSignalValue),
	pub getResonanceModulator: unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut SynthSignalValue,
}
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
	pub newFilter: unsafe extern "C" fn() -> *mut OnePoleFilter,
	pub freeFilter: unsafe extern "C" fn(filter: *mut OnePoleFilter),
	pub setParameter: unsafe extern "C" fn(filter: *mut OnePoleFilter, parameter: core::ffi::c_float),
	pub setParameterModulator: unsafe extern "C" fn(filter: *mut OnePoleFilter, signal: *mut SynthSignalValue),
	pub getParameterModulator: unsafe extern "C" fn(filter: *mut OnePoleFilter) -> *mut SynthSignalValue,
}
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
	pub newBitCrusher: unsafe extern "C" fn() -> *mut BitCrusher,
	pub freeBitCrusher: unsafe extern "C" fn(filter: *mut BitCrusher),
	pub setAmount: unsafe extern "C" fn(filter: *mut BitCrusher, amount: core::ffi::c_float),
	pub setAmountModulator: unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut SynthSignalValue),
	pub getAmountModulator: unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut SynthSignalValue,
	pub setUndersampling: unsafe extern "C" fn(filter: *mut BitCrusher, undersampling: core::ffi::c_float),
	pub setUndersampleModulator: unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut SynthSignalValue),
	pub getUndersampleModulator: unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut SynthSignalValue,
}
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
	pub newRingmod: unsafe extern "C" fn() -> *mut RingModulator,
	pub freeRingmod: unsafe extern "C" fn(filter: *mut RingModulator),
	pub setFrequency: unsafe extern "C" fn(filter: *mut RingModulator, frequency: core::ffi::c_float),
	pub setFrequencyModulator: unsafe extern "C" fn(filter: *mut RingModulator, signal: *mut SynthSignalValue),
	pub getFrequencyModulator: unsafe extern "C" fn(filter: *mut RingModulator) -> *mut SynthSignalValue,
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundEffectDelayline {
	pub newDelayLine: unsafe extern "C" fn(length: core::ffi::c_int, stereo: core::ffi::c_int) -> *mut DelayLine,
	pub freeDelayLine: unsafe extern "C" fn(filter: *mut DelayLine),
	pub setLength: unsafe extern "C" fn(d: *mut DelayLine, frames: core::ffi::c_int),
	pub setFeedback: unsafe extern "C" fn(d: *mut DelayLine, fb: core::ffi::c_float),
	pub addTap: unsafe extern "C" fn(d: *mut DelayLine, delay: core::ffi::c_int) -> *mut DelayLineTap,
	pub freeTap: unsafe extern "C" fn(tap: *mut DelayLineTap),
	pub setTapDelay: unsafe extern "C" fn(t: *mut DelayLineTap, frames: core::ffi::c_int),
	pub setTapDelayModulator: unsafe extern "C" fn(t: *mut DelayLineTap, mod_: *mut SynthSignalValue),
	pub getTapDelayModulator: unsafe extern "C" fn(t: *mut DelayLineTap) -> *mut SynthSignalValue,
	pub setTapChannelsFlipped: unsafe extern "C" fn(t: *mut DelayLineTap, flip: core::ffi::c_int),
}
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
	pub newOverdrive: unsafe extern "C" fn() -> *mut Overdrive,
	pub freeOverdrive: unsafe extern "C" fn(filter: *mut Overdrive),
	pub setGain: unsafe extern "C" fn(o: *mut Overdrive, gain: core::ffi::c_float),
	pub setLimit: unsafe extern "C" fn(o: *mut Overdrive, limit: core::ffi::c_float),
	pub setLimitModulator: unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut SynthSignalValue),
	pub getLimitModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut SynthSignalValue,
	pub setOffset: unsafe extern "C" fn(o: *mut Overdrive, offset: core::ffi::c_float),
	pub setOffsetModulator: unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut SynthSignalValue),
	pub getOffsetModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut SynthSignalValue,
}
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
	pub newEffect: unsafe extern "C" fn(proc_: EffectProc, userdata: *mut core::ffi::c_void) -> *mut SoundEffect,
	pub freeEffect: unsafe extern "C" fn(effect: *mut SoundEffect),
	pub setMix: unsafe extern "C" fn(effect: *mut SoundEffect, level: core::ffi::c_float),
	pub setMixModulator: unsafe extern "C" fn(effect: *mut SoundEffect, signal: *mut SynthSignalValue),
	pub getMixModulator: unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut SynthSignalValue,
	pub setUserdata: unsafe extern "C" fn(effect: *mut SoundEffect, userdata: *mut core::ffi::c_void),
	pub getUserdata: unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut core::ffi::c_void,
	pub twopolefilter: &'static PlaydateSoundEffectTwopolefilter,
	pub onepolefilter: &'static PlaydateSoundEffectOnepolefilter,
	pub bitcrusher: &'static PlaydateSoundEffectBitcrusher,
	pub ringmodulator: &'static PlaydateSoundEffectRingmodulator,
	pub delayline: &'static PlaydateSoundEffectDelayline,
	pub overdrive: &'static PlaydateSoundEffectOverdrive,
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
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateSoundChannel {
	pub newChannel: unsafe extern "C" fn() -> *mut SoundChannel,
	pub freeChannel: unsafe extern "C" fn(channel: *mut SoundChannel),
	pub addSource: unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> core::ffi::c_int,
	pub removeSource:
		unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> core::ffi::c_int,
	pub addCallbackSource: unsafe extern "C" fn(channel: *mut SoundChannel,
	                                            callback: AudioSourceFunction,
	                                            context: *mut core::ffi::c_void,
	                                            stereo: core::ffi::c_int)
	                                            -> *mut SoundSource,
	pub addEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect) -> core::ffi::c_int,
	pub removeEffect:
		unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect) -> core::ffi::c_int,
	pub setVolume: unsafe extern "C" fn(channel: *mut SoundChannel, volume: core::ffi::c_float),
	pub getVolume: unsafe extern "C" fn(channel: *mut SoundChannel) -> core::ffi::c_float,
	pub setVolumeModulator: unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut SynthSignalValue),
	pub getVolumeModulator: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	pub setPan: unsafe extern "C" fn(channel: *mut SoundChannel, pan: core::ffi::c_float),
	pub setPanModulator: unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut SynthSignalValue),
	pub getPanModulator: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	pub getDryLevelSignal: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
	pub getWetLevelSignal: unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut SynthSignalValue,
}
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
pub struct PlaydateSound { pub channel : & 'static PlaydateSoundChannel , pub fileplayer : & 'static PlaydateSoundFileplayer , pub sample : & 'static PlaydateSoundSample , pub sampleplayer : & 'static PlaydateSoundSampleplayer , pub synth : & 'static PlaydateSoundSynth , pub sequence : & 'static PlaydateSoundSequence , pub effect : & 'static PlaydateSoundEffect , pub lfo : & 'static PlaydateSoundLfo , pub envelope : & 'static PlaydateSoundEnvelope , pub source : & 'static PlaydateSoundSource , pub controlsignal : & 'static PlaydateControlSignal , pub track : & 'static PlaydateSoundTrack , pub instrument : & 'static PlaydateSoundInstrument , pub getCurrentTime : unsafe extern "C" fn () -> u32 , pub addSource : unsafe extern "C" fn (callback : AudioSourceFunction , context : * mut core :: ffi :: c_void , stereo : core :: ffi :: c_int) -> * mut SoundSource , pub getDefaultChannel : unsafe extern "C" fn () -> * mut SoundChannel , pub addChannel : unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int , pub removeChannel : unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int , pub setMicCallback : unsafe extern "C" fn (callback : RecordCallback , context : * mut core :: ffi :: c_void , source : MicSource) -> core :: ffi :: c_int , pub getHeadphoneState : unsafe extern "C" fn (headphone : * mut core :: ffi :: c_int , headsetmic : * mut core :: ffi :: c_int , changeCallback : :: core :: option :: Option < unsafe extern "C" fn (headphone : core :: ffi :: c_int , mic : core :: ffi :: c_int) >) , pub setOutputsActive : unsafe extern "C" fn (headphone : core :: ffi :: c_int , speaker : core :: ffi :: c_int) , pub removeSource : unsafe extern "C" fn (source : * mut SoundSource) -> core :: ffi :: c_int , pub signal : & 'static PlaydateSoundSignal , pub getError : unsafe extern "C" fn () -> * const core :: ffi :: c_char , }
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateDisplay {
	pub getWidth: unsafe extern "C" fn() -> core::ffi::c_int,
	pub getHeight: unsafe extern "C" fn() -> core::ffi::c_int,
	pub setRefreshRate: unsafe extern "C" fn(rate: core::ffi::c_float),
	pub setInverted: unsafe extern "C" fn(flag: core::ffi::c_int),
	pub setScale: unsafe extern "C" fn(s: core::ffi::c_uint),
	pub setMosaic: unsafe extern "C" fn(x: core::ffi::c_uint, y: core::ffi::c_uint),
	pub setFlipped: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
	pub setOffset: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
	pub getRefreshRate: unsafe extern "C" fn() -> core::ffi::c_float,
	pub getFPS: unsafe extern "C" fn() -> core::ffi::c_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct Score {
	pub rank: u32,
	pub value: u32,
	pub player: *mut core::ffi::c_char,
}
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
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct Board {
	pub boardID: *mut core::ffi::c_char,
	pub name: *mut core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct BoardsList {
	pub count: core::ffi::c_uint,
	pub lastUpdated: u32,
	pub boards: *mut Board,
}
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
#[repr(i8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum NetworkError {
	Ok = 0,
	NoDevice = -1,
	Busy = -2,
	WriteError = -3,
	WriteBusy = -4,
	WriteTimeout = -5,
	ReadError = -6,
	ReadBusy = -7,
	ReadTimeout = -8,
	ReadOverflow = -9,
	FrameError = -10,
	BadResponse = -11,
	ErrorResponse = -12,
	ResetTimeout = -13,
	BufferTooSmall = -14,
	UnexpectedResponse = -15,
	NotConnectedToAp = -16,
	NotImplemented = -17,
	ConnectionClosed = -18,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum WifiStatus {
	#[doc = "!< Not connected to an AP"]
	NotConnected = 0,
	#[doc = "!< Device is connected to an AP"]
	Connected = 1,
	#[doc = "!< A connection has been attempted and no configured AP was available"]
	NotAvailable = 2,
}
pub type HttpConnectionCallback = ::core::option::Option<unsafe extern "C" fn(connection: *mut HttpConnection)>;
pub type HttpHeaderCallback = ::core::option::Option<unsafe extern "C" fn(conn: *mut HttpConnection,
                                                                          key: *const core::ffi::c_char,
                                                                          value: *const core::ffi::c_char)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateHttp {
	pub requestAccess: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool,
	                                        purpose: *const core::ffi::c_char,
	                                        requestCallback: AccessRequestCallback,
	                                        userdata: *mut core::ffi::c_void)
	                                        -> AccessReply,
	pub newConnection: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool)
	                                        -> *mut HttpConnection,
	pub retain: unsafe extern "C" fn(http: *mut HttpConnection) -> *mut HttpConnection,
	pub release: unsafe extern "C" fn(http: *mut HttpConnection),
	pub setConnectTimeout: unsafe extern "C" fn(connection: *mut HttpConnection, ms: core::ffi::c_int),
	pub setKeepAlive: unsafe extern "C" fn(connection: *mut HttpConnection, keepalive: bool),
	pub setByteRange:
		unsafe extern "C" fn(connection: *mut HttpConnection, start: core::ffi::c_int, end: core::ffi::c_int),
	pub setUserdata: unsafe extern "C" fn(connection: *mut HttpConnection, userdata: *mut core::ffi::c_void),
	pub getUserdata: unsafe extern "C" fn(connection: *mut HttpConnection) -> *mut core::ffi::c_void,
	pub get: unsafe extern "C" fn(conn: *mut HttpConnection,
	                              path: *const core::ffi::c_char,
	                              headers: *const core::ffi::c_char,
	                              headerlen: usize) -> NetworkError,
	pub post: unsafe extern "C" fn(conn: *mut HttpConnection,
	                               path: *const core::ffi::c_char,
	                               headers: *const core::ffi::c_char,
	                               headerlen: usize,
	                               body: *const core::ffi::c_char,
	                               bodylen: usize) -> NetworkError,
	pub query: unsafe extern "C" fn(conn: *mut HttpConnection,
	                                method: *const core::ffi::c_char,
	                                path: *const core::ffi::c_char,
	                                headers: *const core::ffi::c_char,
	                                headerlen: usize,
	                                body: *const core::ffi::c_char,
	                                bodylen: usize) -> NetworkError,
	pub getError: unsafe extern "C" fn(connection: *mut HttpConnection) -> NetworkError,
	pub getProgress:
		unsafe extern "C" fn(conn: *mut HttpConnection, read: *mut core::ffi::c_int, total: *mut core::ffi::c_int),
	pub getResponseStatus: unsafe extern "C" fn(connection: *mut HttpConnection) -> core::ffi::c_int,
	pub getBytesAvailable: unsafe extern "C" fn(conn: *mut HttpConnection) -> usize,
	pub setReadTimeout: unsafe extern "C" fn(conn: *mut HttpConnection, ms: core::ffi::c_int),
	pub setReadBufferSize: unsafe extern "C" fn(conn: *mut HttpConnection, bytes: core::ffi::c_int),
	pub read: unsafe extern "C" fn(conn: *mut HttpConnection,
	                               buf: *mut core::ffi::c_void,
	                               buflen: core::ffi::c_uint) -> core::ffi::c_int,
	pub close: unsafe extern "C" fn(connection: *mut HttpConnection),
	pub setHeaderReceivedCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, headercb: HttpHeaderCallback),
	pub setHeadersReadCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
	pub setResponseCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
	pub setRequestCompleteCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
	pub setConnectionClosedCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
}
pub type TcpConnectionCallback =
	::core::option::Option<unsafe extern "C" fn(connection: *mut TcpConnection, err: NetworkError)>;
pub type TcpOpenCallback = ::core::option::Option<unsafe extern "C" fn(conn: *mut TcpConnection,
                                                                       err: NetworkError,
                                                                       ud: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateTcp {
	pub requestAccess: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool,
	                                        purpose: *const core::ffi::c_char,
	                                        requestCallback: AccessRequestCallback,
	                                        userdata: *mut core::ffi::c_void)
	                                        -> AccessReply,
	pub newConnection: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool)
	                                        -> *mut TcpConnection,
	pub retain: unsafe extern "C" fn(http: *mut TcpConnection) -> *mut TcpConnection,
	pub release: unsafe extern "C" fn(http: *mut TcpConnection),
	pub getError: unsafe extern "C" fn(connection: *mut TcpConnection) -> NetworkError,
	pub setConnectTimeout: unsafe extern "C" fn(connection: *mut TcpConnection, ms: core::ffi::c_int),
	pub setUserdata: unsafe extern "C" fn(connection: *mut TcpConnection, userdata: *mut core::ffi::c_void),
	pub getUserdata: unsafe extern "C" fn(connection: *mut TcpConnection) -> *mut core::ffi::c_void,
	pub open: unsafe extern "C" fn(conn: *mut TcpConnection,
	                               cb: TcpOpenCallback,
	                               ud: *mut core::ffi::c_void) -> NetworkError,
	pub close: unsafe extern "C" fn(conn: *mut TcpConnection) -> NetworkError,
	pub setConnectionClosedCallback:
		unsafe extern "C" fn(conn: *mut TcpConnection, callback: TcpConnectionCallback),
	pub setReadTimeout: unsafe extern "C" fn(conn: *mut TcpConnection, ms: core::ffi::c_int),
	pub setReadBufferSize: unsafe extern "C" fn(conn: *mut TcpConnection, bytes: core::ffi::c_int),
	pub getBytesAvailable: unsafe extern "C" fn(conn: *mut TcpConnection) -> usize,
	pub read: unsafe extern "C" fn(conn: *mut TcpConnection,
	                               buffer: *mut core::ffi::c_void,
	                               length: usize) -> core::ffi::c_int,
	pub write: unsafe extern "C" fn(conn: *mut TcpConnection,
	                                buffer: *const core::ffi::c_void,
	                                length: usize) -> core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PlaydateNetwork {
	pub http: &'static PlaydateHttp,
	pub tcp: &'static PlaydateTcp,
	pub getStatus: unsafe extern "C" fn() -> WifiStatus,
	pub setEnabled:
		unsafe extern "C" fn(flag: bool, callback: ::core::option::Option<unsafe extern "C" fn(err: NetworkError)>),
	pub reserved: [usize; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct Playdate {
	pub system: &'static PlaydateSys,
	// pub file: &'static PlaydateFile,
	pub graphics: &'static PlaydateGraphics,
	// pub sprite: &'static PlaydateSprite,
	pub display: &'static PlaydateDisplay,
	// pub sound: &'static PlaydateSound,
	// pub lua: &'static PlaydateLua,
	// pub json: &'static PlaydateJson,
	// pub scoreboards: &'static PlaydateScoreboards,
	// pub network: &'static PlaydateNetwork,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, :: core :: marker :: ConstParamTy)]
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
	MirrorStarted = 10,
	MirrorEnded = 11,
}
