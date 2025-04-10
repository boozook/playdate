#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
#![cfg_attr(test, allow(deref_nullptr))]


/// Preferred `CString` to use.
pub use alloc::ffi::CString;
/// Preferred `CStr` to use.
pub use core::ffi::CStr;


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
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct FileOptions(pub u8);
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
#[must_use]
pub struct PlaydateFile { # [doc = "Returns human-readable text describing the most recent error (usually indicated by a -1 return from a filesystem function).\n"] pub geterr : unsafe extern "C" fn () -> * const core :: ffi :: c_char , # [doc = "Calls the given callback function for every file at *path*. Subfolders are indicated by a trailing slash '/' in *filename*. *listfiles()* does not recurse into subfolders. If *showhidden* is set, files beginning with a period will be included; otherwise, they are skipped. Returns 0 on success, -1 if no folder exists at *path* or it can’t be opened.\nEquivalent to [```\nplaydate.file.listFiles()```\n](<./Inside Playdate.html#f-file.listFiles>) in the Lua API.\n"] pub listfiles : unsafe extern "C" fn (path : * const core :: ffi :: c_char , callback : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , userdata : * mut core :: ffi :: c_void) > , userdata : * mut core :: ffi :: c_void , showhidden : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Populates the FileStat *stat* with information about the file at *path*. Returns 0 on success, or -1 in case of error.\nFileStattypedef struct\n{\n\tint isdir;\n\tunsigned int size;\n\tint m_year;\n\tint m_month;\n\tint m_day;\n\tint m_hour;\n\tint m_minute;\n\tint m_second;\n} FileStat;"] pub stat : unsafe extern "C" fn (path : * const core :: ffi :: c_char , stat : * mut FileStat) -> core :: ffi :: c_int , # [doc = "Creates the given *path* in the Data/&lt;gameid&gt; folder. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\nEquivalent to [```\nplaydate.file.mkdir()```\n](<./Inside Playdate.html#f-file.mkdir>) in the Lua API.\n"] pub mkdir : unsafe extern "C" fn (path : * const core :: ffi :: c_char) -> core :: ffi :: c_int , # [doc = "Deletes the file at *path*. Returns 0 on success, or -1 in case of error. If recursive is 1 and the target path is a folder, this deletes everything inside the folder (including folders, folders inside those, and so on) as well as the folder itself.\n"] pub unlink : unsafe extern "C" fn (name : * const core :: ffi :: c_char , recursive : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Renames the file at *from* to *to*. It will overwrite the file at *to* without confirmation. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\nEquivalent to [```\nplaydate.file.rename()```\n](<./Inside Playdate.html#f-file.rename>) in the Lua API.\n"] pub rename : unsafe extern "C" fn (from : * const core :: ffi :: c_char , to : * const core :: ffi :: c_char) -> core :: ffi :: c_int , # [doc = "Opens a handle for the file at *path*. The *kFileRead* mode opens a file in the game pdx, while *kFileReadData* searches the game’s data folder; to search the data folder first then fall back on the game pdx, use the bitwise combination *kFileRead|kFileReadData*.*kFileWrite* and *kFileAppend* always write to the data folder. The function returns NULL if a file at *path* cannot be opened, and [playdate-&gt;file-&gt;geterr()](#f-file.geterr) will describe the error. The filesystem has a limit of 64 simultaneous open files. The returned file handle should be [closed](#f-file.close), not freed, when it is no longer in use.\nFileOptionstypedef enum\n{\n\tkFileRead,\n\tkFileReadData,\n\tkFileWrite,\n\tkFileAppend\n} FileOptions;Equivalent to [```\nplaydate.file.open()```\n](<./Inside Playdate.html#f-file.open>) in the Lua API.\n"] pub open : unsafe extern "C" fn (name : * const core :: ffi :: c_char , mode : FileOptions) -> * mut SdFile , # [doc = "Closes the given *file* handle. Returns 0 on success, or -1 in case of error.\nEquivalent to [```\nplaydate.file.close()```\n](<./Inside Playdate.html#f-file.close>) in the Lua API.\n"] pub close : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , # [doc = "Reads up to *len* bytes from the *file* into the buffer *buf*. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.\nEquivalent to [```\nplaydate.file.file:read()```\n](<./Inside Playdate.html#m-file.read>) in the Lua API.\n"] pub read : unsafe extern "C" fn (file : * mut SdFile , buf : * mut core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int , # [doc = "Writes the buffer of bytes *buf* to the *file*. Returns the number of bytes written, or -1 in case of error.\nEquivalent to [```\nplaydate.file.file:write()```\n](<./Inside Playdate.html#m-file.write>) in the Lua API.\n"] pub write : unsafe extern "C" fn (file : * mut SdFile , buf : * const core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int , # [doc = "Flushes the output buffer of *file* immediately. Returns the number of bytes written, or -1 in case of error.\nEquivalent to [```\nplaydate.file.flush()```\n](<./Inside Playdate.html#f-file.flush>) in the Lua API.\n"] pub flush : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , # [doc = "Returns the current read/write offset in the given *file* handle, or -1 on error.\nEquivalent to [```\nplaydate.file.file:tell()```\n](<./Inside Playdate.html#m-file.tell>) in the Lua API.\n"] pub tell : unsafe extern "C" fn (file : * mut SdFile) -> core :: ffi :: c_int , # [doc = "Sets the read/write offset in the given *file* handle to *pos*, relative to the *whence* macro. SEEK_SET is relative to the beginning of the file, SEEK_CUR is relative to the current position of the file pointer, and SEEK_END is relative to the end of the file. Returns 0 on success, -1 on error.\nEquivalent to [```\nplaydate.file.file:seek()```\n](<./Inside Playdate.html#m-file.seek>) in the Lua API.\n"] pub seek : unsafe extern "C" fn (file : * mut SdFile , pos : core :: ffi :: c_int , whence : core :: ffi :: c_int) -> core :: ffi :: c_int , }
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Aabb {
	pub left: core::ffi::c_int,
	pub right: core::ffi::c_int,
	pub top: core::ffi::c_int,
	pub bottom: core::ffi::c_int,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum BitmapFlip {
	Unflipped = 0,
	FlippedX = 1,
	FlippedY = 2,
	FlippedXy = 3,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SolidColor {
	Black = 0,
	White = 1,
	Clear = 2,
	XOR = 3,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LineCapStyle {
	Butt = 0,
	Square = 1,
	Round = 2,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FontLanguage {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StringEncoding {
	ASCII = 0,
	UTF8 = 1,
	UTF16 = 2,
}
pub type Pattern = [u8; 16usize];
pub type Color = usize;
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PolygonFillRule {
	NonZero = 0,
	EvenOdd = 1,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TextWrappingMode {
	Clip = 0,
	Character = 1,
	Word = 2,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TextAlignment {
	Left = 0,
	Center = 1,
	Right = 2,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Bitmap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct BitmapTable {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Font {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct FontData {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct FontPage {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct FontGlyph {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct TileMap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct VideoPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct StreamPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct HttpConnection {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct TcpConnection {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct FilePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
#[must_use]
pub struct PlaydateTilemap {
	#[doc = "Creates a new, empty LCDTileMap object.\n"]
	pub newTilemap: unsafe extern "C" fn() -> *mut TileMap,
	#[doc = "Frees an LCDTileMap object previously allocated with ```\nplaydate→graphics→tilemap→newTilemap()```\n.\n"]
	pub freeTilemap: unsafe extern "C" fn(m: *mut TileMap),
	#[doc = "Sets the image table to use for the tilemap’s tiles.\n"]
	pub setImageTable: unsafe extern "C" fn(m: *mut TileMap, table: *mut BitmapTable),
	#[doc = "Returns the LCDBitmapTable used for the tilemap’s tiles.\n"]
	pub getImageTable: unsafe extern "C" fn(m: *mut TileMap) -> *mut BitmapTable,
	#[doc = "Sets the tilemap’s width and height, in number of tiles.\n"]
	pub setSize: unsafe extern "C" fn(m: *mut TileMap, tilesWide: core::ffi::c_int, tilesHigh: core::ffi::c_int),
	#[doc = "Returns the size of the tile map, in tiles.\n"]
	pub getSize:
		unsafe extern "C" fn(m: *mut TileMap, tilesWide: *mut core::ffi::c_int, tilesHigh: *mut core::ffi::c_int),
	#[doc = "Returns the size of the tilemap in pixels; that is, the size of the tile image multiplied by the number of rows and columns in the tilemap.\n"]
	pub getPixelSize: unsafe extern "C" fn(m: *mut TileMap, outWidth: *mut u32, outHeight: *mut u32),
	#[doc = "Sets the tilemap’s width to *rowwidth* and height to *count/rowwidth* (*count* must be evenly divisible by *rowwidth*), then sets the tiles' indexes to the given list.\n"]
	pub setTiles: unsafe extern "C" fn(m: *mut TileMap,
	                                   indexes: *mut u16,
	                                   count: core::ffi::c_int,
	                                   rowwidth: core::ffi::c_int),
	#[doc = "Sets the index of the tile at tilemap position (*x*, *y*). *index* is the (0-based) index of the cell in the tilemap’s image table.\n"]
	pub setTileAtPosition:
		unsafe extern "C" fn(m: *mut TileMap, x: core::ffi::c_int, y: core::ffi::c_int, idx: u16),
	#[doc = "Returns the image index of the tile at the given *x* and *y* coordinate. If *x* or *y* is out of bounds, returns -1.\n"]
	pub getTileAtPosition:
		unsafe extern "C" fn(m: *mut TileMap, x: core::ffi::c_int, y: core::ffi::c_int) -> core::ffi::c_int,
	#[doc = "Draws the tile map at coordinate (*x*, *y*).\n"]
	pub drawAtPoint: unsafe extern "C" fn(m: *mut TileMap, x: core::ffi::c_float, y: core::ffi::c_float),
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Buttons(pub u8);
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Language {
	English = 0,
	Japanese = 1,
	Unknown = 2,
}
pub type AccessRequestCallback =
	::core::option::Option<unsafe extern "C" fn(allowed: bool, userdata: *mut core::ffi::c_void)>;
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AccessReply {
	AccessAsk = 0,
	AccessDeny = 1,
	AccessAllow = 2,
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
#[must_use]
pub struct MenuItem {
	_unused: [u8; 0],
}
#[repr(u16)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
#[derive(Debug)]
#[must_use]
pub struct PlaydateSys {
	#[doc = "Allocates heap space if *ptr* is NULL, else reallocates the given pointer. If *size* is zero, frees the given pointer.\n"]
	pub realloc: unsafe extern "C" fn(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void,
	// # [doc = "Creates a formatted string and returns it via the *outstring* argument. The arguments and return value match libc’s ```\nasprintf()```\n: the format string is standard ```\nprintf()```\n style, the string returned in *outstring* should be freed by the caller when it’s no longer in use, and the return value is the length of the formatted string.\n"]

	// pub formatString : unsafe extern "C" fn (ret : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int ,
	#[doc = "Calls the log function.\nEquivalent to [```\nprint()```\n](<./Inside Playdate.html#f-print>) in the Lua API.\n"]
	pub logToConsole: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...),
	#[doc = "Calls the log function, outputting an error in red to the console, then pauses execution.\n"]
	pub error: unsafe extern "C" fn(fmt: *const core::ffi::c_char, ...) -> !,
	#[doc = "Returns the current language of the system.\n"]
	// pub getLanguage : unsafe extern "C" fn () -> Language ,
	// # [doc = "Returns the number of milliseconds since…\u{200b}some arbitrary point in time. This should present a consistent timebase while a game is running, but the counter will be disabled when the device is sleeping.\n"]

	// pub getCurrentTimeMilliseconds : unsafe extern "C" fn () -> core :: ffi :: c_uint ,
	// # [doc = "Returns the number of seconds (and sets *milliseconds* if not NULL) elapsed since midnight (hour 0), January 1, 2000.\n"]

	// pub getSecondsSinceEpoch : unsafe extern "C" fn (milliseconds : * mut core :: ffi :: c_uint) -> core :: ffi :: c_uint ,
	// # [doc = "Calculates the current frames per second and draws that value at *x, y*.\n"]
	pub drawFPS: unsafe extern "C" fn(x: core::ffi::c_int, y: core::ffi::c_int),
	#[doc = "PDCallbackFunctionint PDCallbackFunction(void* userdata);Replaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed.\n"]
	pub setUpdateCallback: unsafe extern "C" fn(update: CallbackFunction, userdata: *mut core::ffi::c_void),
	#[doc = "Sets the value pointed to by *current* to a bitmask indicating which buttons are currently down. *pushed* and *released* reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.\nPDButtonkButtonLeft\nkButtonRight\nkButtonUp\nkButtonDown\nkButtonB\nkButtonA"]
	pub getButtonState: unsafe extern "C" fn(current: *mut Buttons, pushed: *mut Buttons, released: *mut Buttons),
	// # [doc = "By default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.\nPDPeripheralskNone\nkAccelerometer"]

	// pub setPeripheralsEnabled : unsafe extern "C" fn (mask : Peripherals) ,
	// # [doc = "Returns the last-read accelerometer data.\n"]

	// pub getAccelerometer : unsafe extern "C" fn (outx : * mut core :: ffi :: c_float , outy : * mut core :: ffi :: c_float , outz : * mut core :: ffi :: c_float) ,
	// # [doc = "Returns the angle change of the crank since the last time this function was called. Negative values are anti-clockwise.\n"]

	// pub getCrankChange : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// # [doc = "Returns the current position of the crank, in the range 0-360. Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.\n"]

	// pub getCrankAngle : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// # [doc = "Returns 1 or 0 indicating whether or not the crank is folded into the unit.\n"]

	// pub isCrankDocked : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// # [doc = "The function returns the previous value for this setting.\n"]

	// pub setCrankSoundsDisabled : unsafe extern "C" fn (flag : core :: ffi :: c_int) -> core :: ffi :: c_int ,
	// # [doc = "Returns 1 if the global \"flipped\" system setting is set, otherwise 0.\n"]

	// pub getFlipped : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// # [doc = "Disables or enables the 3 minute auto lock feature. When called, the timer is reset to 3 minutes.\n"]

	// pub setAutoLockDisabled : unsafe extern "C" fn (disable : core :: ffi :: c_int) ,
	// # [doc = "A game can optionally provide an image to be displayed alongside the system menu. *bitmap* must be a 400x240 LCDBitmap. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.\nOptionally, a non-zero *xoffset*, can be provided. This must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by xoffset pixels as the menu is animated in.\nThis function could be called in response to the kEventPause *event* in your implementation of [eventHandler()](#_eventHandler).\n"]

	// pub setMenuImage : unsafe extern "C" fn (bitmap : * mut Bitmap , xOffset : core :: ffi :: c_int) ,
	// # [doc = "*title* will be the title displayed by the menu item.\nAdds a new menu item to the System Menu. When invoked by the user, this menu item will:\n1. Invoke your *callback* function.\n2. Hide the System Menu.\n3. Unpause your game and call [eventHandler()](#_eventHandler) with the kEventResume *event*.\nYour game can then present an options interface to the player, or take other action, in whatever manner you choose.\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually.\n"]

	// pub addMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , callback : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem ,
	// # [doc = "Adds a new menu item that can be checked or unchecked by the player.\n*title* will be the title displayed by the menu item.\n*value* should be 0 for unchecked, 1 for checked.\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually.\n"]

	// pub addCheckmarkMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , value : core :: ffi :: c_int , callback : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem ,
	// # [doc = "Adds a new menu item that allows the player to cycle through a set of options.\n*title* will be the title displayed by the menu item.\n*options* should be an array of strings representing the states this menu item can cycle through. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.\n*optionsCount* should be the number of items contained in *options*.\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually.\n"]

	// pub addOptionsMenuItem : unsafe extern "C" fn (title : * const core :: ffi :: c_char , optionTitles : * mut * const core :: ffi :: c_char , optionsCount : core :: ffi :: c_int , f : MenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut MenuItem ,
	// # [doc = "Removes all custom menu items from the system menu.\n"]

	// pub removeAllMenuItems : unsafe extern "C" fn () ,
	// # [doc = "Removes the menu item from the system menu.\n"]

	// pub removeMenuItem : unsafe extern "C" fn (menuItem : * mut MenuItem) ,

	// pub getMenuItemValue : unsafe extern "C" fn (menuItem : * mut MenuItem) -> core :: ffi :: c_int ,
	// # [doc = "Gets or sets the integer value of the menu item.\nFor checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.\n"]

	// pub setMenuItemValue : unsafe extern "C" fn (menuItem : * mut MenuItem , value : core :: ffi :: c_int) ,

	// pub getMenuItemTitle : unsafe extern "C" fn (menuItem : * mut MenuItem) -> * const core :: ffi :: c_char ,
	// # [doc = "Gets or sets the display title of the menu item.\n"]

	// pub setMenuItemTitle : unsafe extern "C" fn (menuItem : * mut MenuItem , title : * const core :: ffi :: c_char) ,

	// pub getMenuItemUserdata : unsafe extern "C" fn (menuItem : * mut MenuItem) -> * mut core :: ffi :: c_void ,
	// # [doc = "Gets or sets the userdata value associated with this menu item.\n"]

	// pub setMenuItemUserdata : unsafe extern "C" fn (menuItem : * mut MenuItem , ud : * mut core :: ffi :: c_void) ,
	// # [doc = "Returns 1 if the global \"reduce flashing\" system setting is set, otherwise 0.\n"]

	// pub getReduceFlashing : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// # [doc = "Returns the number of seconds since ```\nplaydate.resetElapsedTime()```\n was called. The value is a floating-point number with microsecond accuracy.\n"]

	// pub getElapsedTime : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// # [doc = "Resets the high-resolution timer.\n"]

	// pub resetElapsedTime : unsafe extern "C" fn () ,
	// # [doc = "Returns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full.\n"]

	// pub getBatteryPercentage : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// # [doc = "Returns the battery’s current voltage level.\n"]

	// pub getBatteryVoltage : unsafe extern "C" fn () -> core :: ffi :: c_float ,
	// # [doc = "Returns the system timezone offset from GMT, in seconds.\n"]

	// pub getTimezoneOffset : unsafe extern "C" fn () -> i32 ,
	// # [doc = "Returns 1 if the user has set the 24-Hour Time preference in the Settings program.\n"]

	// pub shouldDisplay24HourTime : unsafe extern "C" fn () -> core :: ffi :: c_int ,
	// # [doc = "Converts the given epoch time to a PDDateTime.\n"]

	// pub convertEpochToDateTime : unsafe extern "C" fn (epoch : u32 , datetime : * mut DateTime) ,
	// # [doc = "Converts the given PDDateTime to an epoch time.\n"]

	// pub convertDateTimeToEpoch : unsafe extern "C" fn (datetime : * mut DateTime) -> u32 ,
	// # [doc = "Flush the CPU instruction cache, on the very unlikely chance you’re modifying instruction code on the fly. (If you don’t know what I’m talking about, you don’t need this. :smile:)\n"]

	// pub clearICache : unsafe extern "C" fn () ,
	// # [doc = "As an alternative to polling for button presses using ```\ngetButtonState()```\n, this function allows a callback function to be set. The function is called for each button up/down event (possibly multiple events on the same button) that occurred during the previous update cycle. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. The function should return 0 on success or a non-zero value to signal an error.\nPDButtonCallbackFunctiontypedef int PDButtonCallbackFunction(PDButtons button, int down, uint32_t when, void* userdata);"]

	// pub setButtonCallback : unsafe extern "C" fn (cb : ButtonCallbackFunction , buttonud : * mut core :: ffi :: c_void , queuesize : core :: ffi :: c_int) ,
	// # [doc = "Provides a callback to receive messages sent to the device over the serial port using the ```\nmsg```\n command. If no device is connected, you can send these messages to a game in the simulator by entering ```\n!msg &lt;message&gt;```\n in the Lua console.\n"]

	// pub setSerialMessageCallback : unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (data : * const core :: ffi :: c_char) >) ,
	// # [doc = "Allocates and formats a string using a variadic ```\nva_list```\n argument, in the style of ```\nvasprintf()```\n. The string returned via *ret* should be freed by the caller when it is no longer in use. The return value from the function is the length of the formatted string.\n"]

	// pub vaFormatString : unsafe extern "C" fn (outstr : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , args : va_list) -> core :: ffi :: c_int ,
	// # [doc = "Like libc ```\nsscanf()```\n, parses a string according to a format string and places the values into pointers passed in after the format. The return value is the number of items matched.\n"]

	// pub parseString : unsafe extern "C" fn (str_ : * const core :: ffi :: c_char , format : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int ,
	// # [doc = "Pauses execution for the given number of milliseconds.\n"]

	// pub delay : unsafe extern "C" fn (milliseconds : u32) ,
	// # [doc = "Queries the Playdate server for the current time, in seconds elapsed since midnight (hour 0), January 1 2000 UTC. This provides games with a reliable clock source, since the internal clock can be set by the user. The function is asynchronous, returning the server time to a callback function passed in. If an error occurred fetching the time, the ```\nerror```\n argument is set instead.\n"]

	// pub getServerTime : unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (time : * const core :: ffi :: c_char , err : * const core :: ffi :: c_char) >) ,
}
pub type LuaState = *mut core::ffi::c_void;
pub type LuaCFunction = ::core::option::Option<unsafe extern "C" fn(L: *mut LuaState) -> core::ffi::c_int>;
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct LuaUdObject {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Sprite {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LuaValueType {
	Int = 0,
	Float = 1,
	Str = 2,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct LuaReg {
	pub name: *const core::ffi::c_char,
	pub func: LuaCFunction,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
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
#[must_use]
pub struct LuaVal {
	pub name: *const core::ffi::c_char,
	pub type_: LuaValueType,
	pub v: LuaValBindgenTy1,
}
#[repr(C)]
#[must_use]
pub struct LuaValBindgenTy1 {
	pub intval: __BindgenUnionField<core::ffi::c_uint>,
	pub floatval: __BindgenUnionField<core::ffi::c_float>,
	pub strval: __BindgenUnionField<*const core::ffi::c_char>,
	pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
#[must_use]
pub struct JsonReader {
	pub read: JsonReadFunc,
	pub userdata: *mut core::ffi::c_void,
}
pub type JsonWriteFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void,
                                                                     str_: *const core::ffi::c_char,
                                                                     len: core::ffi::c_int)>;
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum SpriteCollisionResponseType {
	Slide = 0,
	Freeze = 1,
	Overlap = 2,
	Bounce = 3,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Rect {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
	pub width: core::ffi::c_float,
	pub height: core::ffi::c_float,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct CollisionPoint {
	pub x: core::ffi::c_float,
	pub y: core::ffi::c_float,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct CollisionVector {
	pub x: core::ffi::c_int,
	pub y: core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
#[must_use]
pub struct SpriteQueryInfo {
	pub sprite: *mut Sprite,
	pub ti1: core::ffi::c_float,
	pub ti2: core::ffi::c_float,
	pub entryPoint: CollisionPoint,
	pub exitPoint: CollisionPoint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
pub type SpriteDrawFunction =
	::core::option::Option<unsafe extern "C" fn(sprite: *mut Sprite, bounds: Rect, drawrect: Rect)>;
pub type SpriteUpdateFunction = ::core::option::Option<unsafe extern "C" fn(sprite: *mut Sprite)>;
pub type SpriteCollisionFilterProc = ::core::option::Option<unsafe extern "C" fn(sprite: *mut Sprite,
                                                                                 other: *mut Sprite)
                                                                                 -> SpriteCollisionResponseType>;
#[repr(C)]
#[derive(Debug)]
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
	#[doc = "Sets the given *sprite*'s image to the given tilemap_.\n"]
	pub setTilemap: unsafe extern "C" fn(s: *mut Sprite, tilemap: *mut TileMap),
	#[doc = "Returns the LCDTileMap currently assigned to the given *sprite*.\n"]
	pub getTilemap: unsafe extern "C" fn(s: *mut Sprite) -> *mut TileMap,
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
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
#[derive(Debug)]
#[must_use]
pub struct SoundSource {
	_unused: [u8; 0],
}
pub type SndCallbackProc =
	::core::option::Option<unsafe extern "C" fn(c: *mut SoundSource, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct PlaydateSoundFileplayer { # [doc = "Allocates a new FilePlayer.\n"] pub newPlayer : unsafe extern "C" fn () -> * mut FilePlayer , # [doc = "Frees the given *player*.\n"] pub freePlayer : unsafe extern "C" fn (player : * mut FilePlayer) , # [doc = "Prepares *player* to stream the file at *path*. Returns 1 if the file exists, otherwise 0.\n"] pub loadIntoPlayer : unsafe extern "C" fn (player : * mut FilePlayer , path : * const core :: ffi :: c_char) -> core :: ffi :: c_int , # [doc = "Sets the buffer length of *player* to *bufferLen* seconds;\n"] pub setBufferLength : unsafe extern "C" fn (player : * mut FilePlayer , bufferLen : core :: ffi :: c_float) , # [doc = "Starts playing the file *player*. If *repeat* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with [playdate-&gt;sound-&gt;fileplayer-&gt;stop()](#f-sound.fileplayer.stop). Returns 1 on success, 0 if buffer allocation failed.\n"] pub play : unsafe extern "C" fn (player : * mut FilePlayer , repeat : core :: ffi :: c_int) -> core :: ffi :: c_int , # [doc = "Returns one if *player* is playing, zero if not.\n"] pub isPlaying : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int , # [doc = "Pauses the file *player*.\n"] pub pause : unsafe extern "C" fn (player : * mut FilePlayer) , # [doc = "Stops playing the file.\n"] pub stop : unsafe extern "C" fn (player : * mut FilePlayer) , # [doc = "Sets the playback volume for left and right channels of *player*.\n"] pub setVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float) , # [doc = "Gets the left and right channel playback volume for *player*.\n"] pub getVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : * mut core :: ffi :: c_float , right : * mut core :: ffi :: c_float) , # [doc = "Returns the length, in seconds, of the file loaded into *player*.\n"] pub getLength : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , # [doc = "Sets the current *offset* in seconds.\n"] pub setOffset : unsafe extern "C" fn (player : * mut FilePlayer , offset : core :: ffi :: c_float) , # [doc = "Sets the playback *rate* for the *player*. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate &lt; 0).\n"] pub setRate : unsafe extern "C" fn (player : * mut FilePlayer , rate : core :: ffi :: c_float) , # [doc = "Sets the *start* and *end* of the loop region for playback, in seconds. If *end* is omitted, the end of the file is used.\n"] pub setLoopRange : unsafe extern "C" fn (player : * mut FilePlayer , start : core :: ffi :: c_float , end : core :: ffi :: c_float) , # [doc = "Returns one if *player* has underrun, zero if not.\n"] pub didUnderrun : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int , # [doc = "Sets a function to be called when playback has completed. This is an alias for [playdate→sound→source→setFinishCallback()](#f-sound.source.setFinishCallback).\nsndCallbackProctypedef void sndCallbackProc(SoundSource* c, void* userdata);"] pub setFinishCallback : unsafe extern "C" fn (player : * mut FilePlayer , callback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub setLoopCallback : unsafe extern "C" fn (player : * mut FilePlayer , callback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , # [doc = "Returns the current offset in seconds for *player*.\n"] pub getOffset : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , # [doc = "Returns the playback rate for *player*.\n"] pub getRate : unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float , # [doc = "If *flag* evaluates to true, the *player* will restart playback (after an audible stutter) as soon as data is available.\n"] pub setStopOnUnderrun : unsafe extern "C" fn (player : * mut FilePlayer , flag : core :: ffi :: c_int) , # [doc = "Changes the volume of the fileplayer to *left* and *right* over a length of *len* sample frames, then calls the provided callback (if set).\n"] pub fadeVolume : unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float , len : i32 , finishCallback : SndCallbackProc , userdata : * mut core :: ffi :: c_void) , pub setMP3StreamSource : unsafe extern "C" fn (player : * mut FilePlayer , dataSource : :: core :: option :: Option < unsafe extern "C" fn (data : * mut u8 , bytes : core :: ffi :: c_int , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_int > , userdata : * mut core :: ffi :: c_void , bufferLen : core :: ffi :: c_float) , }
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct AudioSample {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct SamplePlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct SynthSignalValue {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
#[derive(Debug)]
#[must_use]
pub struct SynthLfo {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct PlaydateSoundLfo { # [doc = "Returns a new LFO object, which can be used to modulate sounds. The *type* argument is one of the following values:\nLFOTypetypedef enum\n{\n\tkLFOTypeSquare,\n\tkLFOTypeTriangle,\n\tkLFOTypeSine,\n\tkLFOTypeSampleAndHold,\n\tkLFOTypeSawtoothUp,\n\tkLFOTypeSawtoothDown,\n\tkLFOTypeArpeggiator,\n\tkLFOTypeFunction\n} LFOType;"] pub newLFO : unsafe extern "C" fn (type_ : LfoType) -> * mut SynthLfo , # [doc = "Frees the LFO.\n"] pub freeLFO : unsafe extern "C" fn (lfo : * mut SynthLfo) , # [doc = "Sets the LFO shape to one of the values given above.\n"] pub setType : unsafe extern "C" fn (lfo : * mut SynthLfo , type_ : LfoType) , # [doc = "Sets the LFO’s rate, in cycles per second.\n"] pub setRate : unsafe extern "C" fn (lfo : * mut SynthLfo , rate : core :: ffi :: c_float) , # [doc = "Sets the LFO’s phase, from 0 to 1.\n"] pub setPhase : unsafe extern "C" fn (lfo : * mut SynthLfo , phase : core :: ffi :: c_float) , # [doc = "Sets the center value for the LFO.\n"] pub setCenter : unsafe extern "C" fn (lfo : * mut SynthLfo , center : core :: ffi :: c_float) , # [doc = "Sets the depth of the LFO.\n"] pub setDepth : unsafe extern "C" fn (lfo : * mut SynthLfo , depth : core :: ffi :: c_float) , # [doc = "Sets the LFO type to arpeggio, where the given values are in half-steps from the center note. For example, the sequence (0, 4, 7, 12) plays the notes of a major chord.\n"] pub setArpeggiation : unsafe extern "C" fn (lfo : * mut SynthLfo , nSteps : core :: ffi :: c_int , steps : * mut core :: ffi :: c_float) , # [doc = "Provides a custom function for LFO values.\n"] pub setFunction : unsafe extern "C" fn (lfo : * mut SynthLfo , lfoFunc : :: core :: option :: Option < unsafe extern "C" fn (lfo : * mut SynthLfo , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_float > , userdata : * mut core :: ffi :: c_void , interpolate : core :: ffi :: c_int) , # [doc = "Sets an initial holdoff time for the LFO where the LFO remains at its center value, and a ramp time where the value increases linearly to its maximum depth. Values are in seconds.\n"] pub setDelay : unsafe extern "C" fn (lfo : * mut SynthLfo , holdoff : core :: ffi :: c_float , ramptime : core :: ffi :: c_float) , # [doc = "If retrigger is on, the LFO’s phase is reset to its initial phase (default 0) when a synth using the LFO starts playing a note.\n"] pub setRetrigger : unsafe extern "C" fn (lfo : * mut SynthLfo , flag : core :: ffi :: c_int) , # [doc = "Return the current output value of the LFO.\n"] pub getValue : unsafe extern "C" fn (lfo : * mut SynthLfo) -> core :: ffi :: c_float , # [doc = "If *global* is set, the LFO is continuously updated whether or not it’s currently in use.\n"] pub setGlobal : unsafe extern "C" fn (lfo : * mut SynthLfo , global : core :: ffi :: c_int) , # [doc = "Sets the LFO’s initial phase, from 0 to 1.\n"] pub setStartPhase : unsafe extern "C" fn (lfo : * mut SynthLfo , phase : core :: ffi :: c_float) , }
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct SynthEnvelope {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
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
#[derive(Debug)]
#[must_use]
pub struct Synth {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct ControlSignal {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct SynthInstrument {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct SequenceTrack {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct SoundSequence {
	_unused: [u8; 0],
}
pub type SequenceFinishedCallback =
	::core::option::Option<unsafe extern "C" fn(seq: *mut SoundSequence, userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct TwoPoleFilter {
	_unused: [u8; 0],
}
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
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
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct OnePoleFilter {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct BitCrusher {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct RingModulator {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct DelayLine {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct DelayLineTap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Overdrive {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
	#[doc = "Adds a [SoundEffect](#f-sound.effect) to the channel. Returns 1 if successful, 0 if the effect is already in use.\n"]
	pub addEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect) -> core::ffi::c_int,
	#[doc = "Removes a [SoundEffect](#f-sound.effect) from the channel. Returns 1 if the effect was in the channel and removed, otherwise 0.\n"]
	pub removeEffect:
		unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect) -> core::ffi::c_int,
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
pub type RecordCallback = ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void,
                                                                      buffer: *mut i16,
                                                                      length: core::ffi::c_int)
                                                                      -> core::ffi::c_int>;
#[repr(u8)]
#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MicSource {
	Autodetect = 0,
	Internal = 1,
	Headset = 2,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct PlaydateSound { pub channel : & 'static PlaydateSoundChannel , pub fileplayer : & 'static PlaydateSoundFileplayer , pub sample : & 'static PlaydateSoundSample , pub sampleplayer : & 'static PlaydateSoundSampleplayer , pub synth : & 'static PlaydateSoundSynth , pub sequence : & 'static PlaydateSoundSequence , pub effect : & 'static PlaydateSoundEffect , pub lfo : & 'static PlaydateSoundLfo , pub envelope : & 'static PlaydateSoundEnvelope , pub source : & 'static PlaydateSoundSource , pub controlsignal : & 'static PlaydateControlSignal , pub track : & 'static PlaydateSoundTrack , pub instrument : & 'static PlaydateSoundInstrument , # [doc = "Returns the sound engine’s current time value, in units of sample frames, 44,100 per second.\nEquivalent to [```\nplaydate.sound.getCurrentTime()```\n](<./Inside Playdate.html#f-sound.getCurrentTime>) in the Lua API.\n"] pub getCurrentTime : unsafe extern "C" fn () -> u32 , # [doc = "The *callback* function you pass in will be called every audio render cycle.\nAudioSourceFunctionint AudioSourceFunction(void* context, int16_t* left, int16_t* right, int len)This function should fill the passed-in *left* buffer (and *right* if it’s a stereo source) with *len* samples each and return 1, or return 0 if the source is silent through the cycle.\n"] pub addSource : unsafe extern "C" fn (callback : AudioSourceFunction , context : * mut core :: ffi :: c_void , stereo : core :: ffi :: c_int) -> * mut SoundSource , # [doc = "Returns the default channel, where sound sources play if they haven’t been explicity assigned to a different channel.\n"] pub getDefaultChannel : unsafe extern "C" fn () -> * mut SoundChannel , # [doc = "Adds the given channel to the sound engine. Returns 1 if the channel was added, 0 if it was already in the engine.\n"] pub addChannel : unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int , # [doc = "Removes the given channel from the sound engine. Returns 1 if the channel was successfully removed, 0 if the channel is the default channel or hadn’t been previously added.\n"] pub removeChannel : unsafe extern "C" fn (channel : * mut SoundChannel) -> core :: ffi :: c_int , # [doc = "The *callback* you pass in will be called every audio cycle.\nAudioInputFunctionint AudioInputFunction(void* context, int16_t* data, int len)enum MicSourceenum MicSource {\n\tkMicInputAutodetect = 0,\n\tkMicInputInternal = 1,\n\tkMicInputHeadset = 2\n};Your input callback will be called with the recorded audio data, a monophonic stream of samples. The function should return 1 to continue recording, 0 to stop recording.\nThe Playdate hardware has a circuit that attempts to autodetect the presence of a headset mic, but there are cases where you may want to override this. For instance, if you’re using a headphone splitter to wire an external source to the mic input, the detector may not always see the input. Setting the source to ```\nkMicInputHeadset```\n forces recording from the headset. Using ```\nkMicInputInternal```\n records from the device mic even when a headset with a mic is plugged in. And ```\nkMicInputAutodetect```\n uses a headset mic if one is detected, otherwise the device microphone. ```\nsetMicCallback()```\n returns which source the function used, internal or headset, or 0 on error.\n"] pub setMicCallback : unsafe extern "C" fn (callback : RecordCallback , context : * mut core :: ffi :: c_void , source : MicSource) -> core :: ffi :: c_int , # [doc = "If *headphone* contains a pointer to an int, the value is set to 1 if headphones are currently plugged in. Likewise, *mic* is set if the headphones include a microphone. If *changeCallback* is provided, it will be called when the headset or mic status changes, and audio output will **not** automatically switch from speaker to headphones when headphones are plugged in (and vice versa). In this case, the callback should use ```\nplaydate→sound→setOutputsActive()```\n to change the output if needed.\nEquivalent to [```\nplaydate.sound.getHeadphoneState()```\n](<./Inside Playdate.html#f-sound.getHeadphoneState>) in the Lua API.\n"] pub getHeadphoneState : unsafe extern "C" fn (headphone : * mut core :: ffi :: c_int , headsetmic : * mut core :: ffi :: c_int , changeCallback : :: core :: option :: Option < unsafe extern "C" fn (headphone : core :: ffi :: c_int , mic : core :: ffi :: c_int) >) , # [doc = "Force audio output to the given outputs, regardless of headphone status.\nEquivalent to [```\nplaydate.sound.setOutputsActive()```\n](<./Inside Playdate.html#f-sound.setOutputsActive>) in the Lua API.\n"] pub setOutputsActive : unsafe extern "C" fn (headphone : core :: ffi :: c_int , speaker : core :: ffi :: c_int) , # [doc = "Removes the given [SoundSource](#C-sound.source) object from its channel, whether it’s in the default channel or a channel created with [playdate→sound→addChannel()](#f-sound.addChannel). Returns 1 if a source was removed, 0 if the source wasn’t in a channel.\n"] pub removeSource : unsafe extern "C" fn (source : * mut SoundSource) -> core :: ffi :: c_int , pub signal : & 'static PlaydateSoundSignal , pub getError : unsafe extern "C" fn () -> * const core :: ffi :: c_char , }
#[repr(C)]
#[derive(Debug)]
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
	#[doc = "Returns the current nominal display refresh rate. This is the frame rate the device is targeting, and does not account for lag due to (for example) code running too slow. To get the real time frame rate, use [playdate→display→getFPS()](#f-display.getFPS).\nEquivalent to [```\nplaydate.display.getRefreshRate()```\n](<./Inside Playdate.html#f-display.getRefreshRate>) in the Lua API.\n"]
	pub getRefreshRate: unsafe extern "C" fn() -> core::ffi::c_float,
	#[doc = "Returns the *measured, actual* refresh rate in frames per second. This value may be different from the *specified* refresh rate (see [playdate→display→getRefreshRate()](#f-display.getRefreshRate)) by a little or a lot depending upon how much calculation is being done per frame.\nEquivalent to [```\nplaydate.display.getFPS()```\n](<./Inside Playdate.html#f-display.getFPS>) in the Lua API.\n"]
	pub getFPS: unsafe extern "C" fn() -> core::ffi::c_float,
}
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct Score {
	pub rank: u32,
	pub value: u32,
	pub player: *mut core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
#[must_use]
pub struct Board {
	pub boardID: *mut core::ffi::c_char,
	pub name: *mut core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
#[derive(Debug)]
#[must_use]
pub struct PlaydateHttp {
	#[doc = "typedef void AccessRequestCallback(bool allowed, void* userdata);Before connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling http→newConnection(). ```\nserver```\n can be a parent domain of the connections opened, or NULL to request access to any HTTP server. ```\npurpose```\n is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, ```\nrequestCallback```\n is called with the given ```\nuserdata```\n argument. The return value is one of the following:\nenum accessReply\n{\n\tkAccessAsk,\n\tkAccessDeny,\n\tkAccessAllow\n};"]
	pub requestAccess: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool,
	                                        purpose: *const core::ffi::c_char,
	                                        requestCallback: AccessRequestCallback,
	                                        userdata: *mut core::ffi::c_void)
	                                        -> AccessReply,
	#[doc = "Returns an ```\nHTTPConnection```\n object for connecting to the given server, or NULL if permission has been denied or not yet granted. If ```\nport```\n is 0, the connection will use port 80 if ```\nusessl```\n is false, otherwise 443. No connection is attempted until [get()](#f-network.http.get) or [post()](#f-network.http.post) are called.\n"]
	pub newConnection: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool)
	                                        -> *mut HttpConnection,
	#[doc = "Adds 1 to the connection’s retain count, so that it won’t be freed when it scopes out of another context. This is used primarily so we can pass a connection created in Lua into C and not have to worry about the Lua wrapper’s lifespan.\n"]
	pub retain: unsafe extern "C" fn(http: *mut HttpConnection) -> *mut HttpConnection,
	#[doc = "Releases a previous retain on the connection.\n"]
	pub release: unsafe extern "C" fn(http: *mut HttpConnection),
	#[doc = "Sets the length of time (in milliseconds) to wait for the connection to the server to be made.\n"]
	pub setConnectTimeout: unsafe extern "C" fn(connection: *mut HttpConnection, ms: core::ffi::c_int),
	#[doc = "If ```\nkeepalive```\n is true, this causes the HTTP request to include a *Connection: keep-alive* header.\n"]
	pub setKeepAlive: unsafe extern "C" fn(connection: *mut HttpConnection, keepalive: bool),
	#[doc = "Adds a ```\nRange: bytes=&lt;start&gt;-&lt;end&gt;```\n header to the HTTP request.\n"]
	pub setByteRange:
		unsafe extern "C" fn(connection: *mut HttpConnection, start: core::ffi::c_int, end: core::ffi::c_int),
	#[doc = "Sets a custom userdata on the connection.\n"]
	pub setUserdata: unsafe extern "C" fn(connection: *mut HttpConnection, userdata: *mut core::ffi::c_void),
	#[doc = "Returns the userdata previously set on the connection.\n"]
	pub getUserdata: unsafe extern "C" fn(connection: *mut HttpConnection) -> *mut core::ffi::c_void,
	#[doc = "Opens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a GET request with the given path and additional *headers* if specified.\n"]
	pub get: unsafe extern "C" fn(conn: *mut HttpConnection,
	                              path: *const core::ffi::c_char,
	                              headers: *const core::ffi::c_char,
	                              headerlen: usize) -> NetworkError,
	#[doc = "Opens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a POST request with the given path, additional *headers* if specified, and the provided *data*.\n"]
	pub post: unsafe extern "C" fn(conn: *mut HttpConnection,
	                               path: *const core::ffi::c_char,
	                               headers: *const core::ffi::c_char,
	                               headerlen: usize,
	                               body: *const core::ffi::c_char,
	                               bodylen: usize) -> NetworkError,
	#[doc = "Returns a code for the last error on the connection, or NET_OK if none occurred.\n"]
	pub getError: unsafe extern "C" fn(connection: *mut HttpConnection) -> NetworkError,
	#[doc = "Returns the number of bytes already read from the connection and the total bytes the server plans to send, if known.\n"]
	pub getProgress:
		unsafe extern "C" fn(conn: *mut HttpConnection, read: *mut core::ffi::c_int, total: *mut core::ffi::c_int),
	#[doc = "Returns the HTTP status response code, if the request response headers have been received and parsed.\n"]
	pub getResponseStatus: unsafe extern "C" fn(connection: *mut HttpConnection) -> core::ffi::c_int,
	#[doc = "Returns the number of bytes currently available for reading from the connection.\n"]
	pub getBytesAvailable: unsafe extern "C" fn(conn: *mut HttpConnection) -> usize,
	#[doc = "Sets the length of time, in milliseconds, the [read()](#f-network.http.read) function will wait for incoming data before returning. The default value is 1000, or one second.\n"]
	pub setReadTimeout: unsafe extern "C" fn(conn: *mut HttpConnection, ms: core::ffi::c_int),
	#[doc = "Sets the size of the connection’s read buffer. The default buffer size is 64 KB.\n"]
	pub setReadBufferSize: unsafe extern "C" fn(conn: *mut HttpConnection, bytes: core::ffi::c_int),
	#[doc = "On success, returns up to ```\nlength```\n bytes (limited by the size of the read buffer) from the connection. If ```\nlength```\n is more than the number of bytes available the function will wait for more data up to the length of time set by [setReadTimeout()](#f-network.http.setReadTimeout) (default one second).\n"]
	pub read: unsafe extern "C" fn(conn: *mut HttpConnection,
	                               buf: *mut core::ffi::c_void,
	                               buflen: core::ffi::c_uint) -> core::ffi::c_int,
	#[doc = "Closes the HTTP connection. The connection may be used again for another request.\n"]
	pub close: unsafe extern "C" fn(connection: *mut HttpConnection),
	#[doc = "Sets a callback to be called when the HTTP parser reads a header line from the connection\n"]
	pub setHeaderReceivedCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, headercb: HttpHeaderCallback),
	#[doc = "Sets a function to be called after the connection has parsed the headers from the server response. At this point, [getResponseStatus()](#f-network.http.getResponseStatus) and [getProgress()](#f-network.http.getProgress) can be used to query the status and size of the response, and [get()](#f-network.http.get)/[post()](#f-network.http.post) can queue another request if ```\nconnection:setKeepAlive(true)```\n was set and the connection is still open.\n"]
	pub setHeadersReadCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
	#[doc = "Sets a function to be called when data is available for reading.\n"]
	pub setResponseCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
	#[doc = "Sets a function to be called when all data for the request has been received (if the response contained a Content-Length header and the size is known) or the request times out.\n"]
	pub setRequestCompleteCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
	#[doc = "Sets a function to be called when the server has closed the connection.\n"]
	pub setConnectionClosedCallback:
		unsafe extern "C" fn(connection: *mut HttpConnection, callback: HttpConnectionCallback),
}
pub type TcpConnectionCallback =
	::core::option::Option<unsafe extern "C" fn(connection: *mut TcpConnection, err: NetworkError)>;
pub type TcpOpenCallback = ::core::option::Option<unsafe extern "C" fn(conn: *mut TcpConnection,
                                                                       err: NetworkError,
                                                                       ud: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug)]
#[must_use]
pub struct PlaydateTcp {
	#[doc = "Before connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling [newConnection()](#f-network.tcp.newConnection()). ```\nserver```\n can be a parent domain of the connections opened, or NULL to request access to any HTTP server. Similarly, if ```\nport```\n is zero, this requests access to all ports on the target server(s). ```\npurpose```\n is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, ```\nrequestCallback```\n is called with the given ```\nuserdata```\n argument. The return value is one of the following:\nenum accessReply\n{\n\tkAccessAsk,\n\tkAccessDeny,\n\tkAccessAllow\n};"]
	pub requestAccess: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool,
	                                        purpose: *const core::ffi::c_char,
	                                        requestCallback: AccessRequestCallback,
	                                        userdata: *mut core::ffi::c_void)
	                                        -> AccessReply,
	#[doc = "Returns a ```\nplaydate.network.tcp```\n object for connecting to the given server, or NULL if permission has been denied or not yet granted. No connection is attempted until [open()](#f-network.tcp.open) is called.\n"]
	pub newConnection: unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                        port: core::ffi::c_int,
	                                        usessl: bool)
	                                        -> *mut TcpConnection,
	#[doc = "Adds 1 to the connection’s retain count, so that it won’t be freed when it scopes out of another context. This is used primarily so we can pass a connection created in Lua into C and not have to worry about the Lua wrapper’s lifespan.\n"]
	pub retain: unsafe extern "C" fn(http: *mut TcpConnection) -> *mut TcpConnection,
	#[doc = "Releases a previous retain on the connection.\n"]
	pub release: unsafe extern "C" fn(http: *mut TcpConnection),
	#[doc = "Returns a code for the last error on the connection, or NET_OK if none occurred.\n"]
	pub getError: unsafe extern "C" fn(connection: *mut TcpConnection) -> NetworkError,
	#[doc = "Sets the length of time (in milliseconds) to wait for the connection to the server to be made.\n"]
	pub setConnectTimeout: unsafe extern "C" fn(connection: *mut TcpConnection, ms: core::ffi::c_int),
	#[doc = "Sets a custom userdata on the connection.\n"]
	pub setUserdata: unsafe extern "C" fn(connection: *mut TcpConnection, userdata: *mut core::ffi::c_void),
	#[doc = "Returns the userdata previously set on the connection.\n"]
	pub getUserdata: unsafe extern "C" fn(connection: *mut TcpConnection) -> *mut core::ffi::c_void,
	#[doc = "typedef void TCPOpenCallback(TCPConnection* conn, PDNetErr err, void* ud);Attempts to open the connection to the server. Note that an error may be returned immediately, or in the open callback depending on where it occurs.\n"]
	pub open: unsafe extern "C" fn(conn: *mut TcpConnection,
	                               cb: TcpOpenCallback,
	                               ud: *mut core::ffi::c_void) -> NetworkError,
	#[doc = "Closes the connection. The connection may be used again for another request.\n"]
	pub close: unsafe extern "C" fn(conn: *mut TcpConnection) -> NetworkError,
	#[doc = "typedef void TCPConnectionCallback(TCPConnection* connection, PDNetErr err);Sets a callback to be called when the connection is closed.\n"]
	pub setConnectionClosedCallback:
		unsafe extern "C" fn(conn: *mut TcpConnection, callback: TcpConnectionCallback),
	#[doc = "Sets the length of time, in milliseconds, [read()](#f-network.tcp.read) will wait for incoming data before returning. The default value is 1000, or one second.\n"]
	pub setReadTimeout: unsafe extern "C" fn(conn: *mut TcpConnection, ms: core::ffi::c_int),
	#[doc = "Sets the size of the connection’s read buffer. The default buffer size is 64 KB.\n"]
	pub setReadBufferSize: unsafe extern "C" fn(conn: *mut TcpConnection, bytes: core::ffi::c_int),
	#[doc = "Returns the number of bytes currently available for reading from the connection.\n"]
	pub getBytesAvailable: unsafe extern "C" fn(conn: *mut TcpConnection) -> usize,
	#[doc = "Attempts to read up to ```\nlength```\n bytes from the connection into ```\nbuffer```\n. If ```\nlength```\n is more than the number of bytes available on the connection the function will wait for more data, up to the length of time set by [setReadTimeout()](#f-network.tcp.setReadTimeout) (default one second). Returns the number of bytes actually read, or a (negative) PDNetErr value on error.\n"]
	pub read: unsafe extern "C" fn(conn: *mut TcpConnection,
	                               buffer: *mut core::ffi::c_void,
	                               length: usize) -> core::ffi::c_int,
	#[doc = "Attempts to write up to ```\nlength```\n bytes to the connection. Returns the number of bytes actually written, which may be less than ```\nlength```\n, or a (negative) PDNetErr value on error.\n"]
	pub write: unsafe extern "C" fn(conn: *mut TcpConnection,
	                                buffer: *const core::ffi::c_void,
	                                length: usize) -> core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
