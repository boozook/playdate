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
pub struct FileOptions(pub u32);
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_file { # [doc = "`const char* playdate->file->geterr(void);`\n\nReturns human-readable text describing the most recent error (usually indicated by a -1 return from a filesystem function)."] pub geterr : :: core :: option :: Option < unsafe extern "C" fn () -> * const core :: ffi :: c_char > , # [doc = "`int playdate->file->listfiles(const char* path, void (*callback)(const char* filename, void* userdata), void* userdata, int showhidden);`\n\nCalls the given callback function for every file at *path*. Subfolders are indicated by a trailing slash '/' in *filename*. *listfiles()* does not recurse into subfolders. If *showhidden* is set, files beginning with a period will be included; otherwise, they are skipped. Returns 0 on success, -1 if no folder exists at *path* or it can’t be opened.\n\nEquivalent to [`playdate.file.listFiles()`](./Inside%20Playdate.html#f-file.listFiles) in the Lua API."] pub listfiles : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , callback : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , userdata : * mut core :: ffi :: c_void) > , userdata : * mut core :: ffi :: c_void , showhidden : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->stat(const char* path, FileStat* stat);`\n\nPopulates the FileStat *stat* with information about the file at *path*. Returns 0 on success, or -1 in case of error.\n\nFileStat\n\n```cpp\ntypedef struct\n{\n\tint isdir;\n\tunsigned int size;\n\tint m_year;\n\tint m_month;\n\tint m_day;\n\tint m_hour;\n\tint m_minute;\n\tint m_second;\n} FileStat;\n```"] pub stat : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char , stat : * mut FileStat) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->mkdir(const char* path);`\n\nCreates the given *path* in the Data/\\<gameid\\> folder. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\n\nEquivalent to [`playdate.file.mkdir()`](./Inside%20Playdate.html#f-file.mkdir) in the Lua API."] pub mkdir : :: core :: option :: Option < unsafe extern "C" fn (path : * const core :: ffi :: c_char) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->unlink(const char* path, int recursive);`\n\nDeletes the file at *path*. Returns 0 on success, or -1 in case of error. If recursive is 1 and the target path is a folder, this deletes everything inside the folder (including folders, folders inside those, and so on) as well as the folder itself."] pub unlink : :: core :: option :: Option < unsafe extern "C" fn (name : * const core :: ffi :: c_char , recursive : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->rename(const char* from, const char* to);`\n\nRenames the file at *from* to *to*. It will overwrite the file at *to* without confirmation. It does not create intermediate folders. Returns 0 on success, or -1 in case of error.\n\nEquivalent to [`playdate.file.rename()`](./Inside%20Playdate.html#f-file.rename) in the Lua API."] pub rename : :: core :: option :: Option < unsafe extern "C" fn (from : * const core :: ffi :: c_char , to : * const core :: ffi :: c_char) -> core :: ffi :: c_int > , # [doc = "`SDFile* playdate->file->open(const char* path, FileOptions mode);`\n\nOpens a handle for the file at *path*. The *kFileRead* mode opens a file in the game pdx, while *kFileReadData* searches the game’s data folder; to search the data folder first then fall back on the game pdx, use the bitwise combination *kFileRead|kFileReadData*.*kFileWrite* and *kFileAppend* always write to the data folder. The function returns NULL if a file at *path* cannot be opened, and [playdate-\\>file-\\>geterr()](#f-file.geterr) will describe the error. The filesystem has a limit of 64 simultaneous open files. The returned file handle should be [closed](#f-file.close), not freed, when it is no longer in use.\n\nFileOptions\n\n```cpp\ntypedef enum\n{\n\tkFileRead,\n\tkFileReadData,\n\tkFileWrite,\n\tkFileAppend\n} FileOptions;\n```\n\nEquivalent to [`playdate.file.open()`](./Inside%20Playdate.html#f-file.open) in the Lua API."] pub open : :: core :: option :: Option < unsafe extern "C" fn (name : * const core :: ffi :: c_char , mode : FileOptions) -> * mut SDFile > , # [doc = "`int playdate->file->close(SDFile* file);`\n\nCloses the given *file* handle. Returns 0 on success, or -1 in case of error.\n\nEquivalent to [`playdate.file.close()`](./Inside%20Playdate.html#f-file.close) in the Lua API."] pub close : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->read(SDFile* file, void* buf, unsigned int len);`\n\nReads up to *len* bytes from the *file* into the buffer *buf*. Returns the number of bytes read (0 indicating end of file), or -1 in case of error.\n\nEquivalent to [`playdate.file.file:read()`](./Inside%20Playdate.html#m-file.read) in the Lua API."] pub read : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile , buf : * mut core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->write(SDFile* file, const void* buf, unsigned int len);`\n\nWrites the buffer of bytes *buf* to the *file*. Returns the number of bytes written, or -1 in case of error.\n\nEquivalent to [`playdate.file.file:write()`](./Inside%20Playdate.html#m-file.write) in the Lua API."] pub write : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile , buf : * const core :: ffi :: c_void , len : core :: ffi :: c_uint) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->flush(SDFile* file);`\n\nFlushes the output buffer of *file* immediately. Returns the number of bytes written, or -1 in case of error.\n\nEquivalent to [`playdate.file.flush()`](./Inside%20Playdate.html#f-file.flush) in the Lua API."] pub flush : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->tell(SDFile* file);`\n\nReturns the current read/write offset in the given *file* handle, or -1 on error.\n\nEquivalent to [`playdate.file.file:tell()`](./Inside%20Playdate.html#m-file.tell) in the Lua API."] pub tell : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile) -> core :: ffi :: c_int > , # [doc = "`int playdate->file->seek(SDFile* file, int pos, int whence);`\n\nSets the read/write offset in the given *file* handle to *pos*, relative to the *whence* macro. SEEK\\_SET is relative to the beginning of the file, SEEK\\_CUR is relative to the current position of the file pointer, and SEEK\\_END is relative to the end of the file. Returns 0 on success, -1 on error.\n\nEquivalent to [`playdate.file.file:seek()`](./Inside%20Playdate.html#m-file.seek) in the Lua API."] pub seek : :: core :: option :: Option < unsafe extern "C" fn (file : * mut SDFile , pos : core :: ffi :: c_int , whence : core :: ffi :: c_int) -> core :: ffi :: c_int > , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_file"][::core::mem::size_of::<playdate_file>() - 104usize];
	["Alignment of playdate_file"][::core::mem::align_of::<playdate_file>() - 8usize];
	["Offset of field: playdate_file::geterr"][::core::mem::offset_of!(playdate_file, geterr) - 0usize];
	["Offset of field: playdate_file::listfiles"][::core::mem::offset_of!(playdate_file, listfiles) - 8usize];
	["Offset of field: playdate_file::stat"][::core::mem::offset_of!(playdate_file, stat) - 16usize];
	["Offset of field: playdate_file::mkdir"][::core::mem::offset_of!(playdate_file, mkdir) - 24usize];
	["Offset of field: playdate_file::unlink"][::core::mem::offset_of!(playdate_file, unlink) - 32usize];
	["Offset of field: playdate_file::rename"][::core::mem::offset_of!(playdate_file, rename) - 40usize];
	["Offset of field: playdate_file::open"][::core::mem::offset_of!(playdate_file, open) - 48usize];
	["Offset of field: playdate_file::close"][::core::mem::offset_of!(playdate_file, close) - 56usize];
	["Offset of field: playdate_file::read"][::core::mem::offset_of!(playdate_file, read) - 64usize];
	["Offset of field: playdate_file::write"][::core::mem::offset_of!(playdate_file, write) - 72usize];
	["Offset of field: playdate_file::flush"][::core::mem::offset_of!(playdate_file, flush) - 80usize];
	["Offset of field: playdate_file::tell"][::core::mem::offset_of!(playdate_file, tell) - 88usize];
	["Offset of field: playdate_file::seek"][::core::mem::offset_of!(playdate_file, seek) - 96usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct LCDRect {
	pub left: core::ffi::c_int,
	pub right: core::ffi::c_int,
	pub top: core::ffi::c_int,
	pub bottom: core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of LCDRect"][::core::mem::size_of::<LCDRect>() - 16usize];
	["Alignment of LCDRect"][::core::mem::align_of::<LCDRect>() - 4usize];
	["Offset of field: LCDRect::left"][::core::mem::offset_of!(LCDRect, left) - 0usize];
	["Offset of field: LCDRect::right"][::core::mem::offset_of!(LCDRect, right) - 4usize];
	["Offset of field: LCDRect::top"][::core::mem::offset_of!(LCDRect, top) - 8usize];
	["Offset of field: LCDRect::bottom"][::core::mem::offset_of!(LCDRect, bottom) - 12usize];
};
#[repr(u32)]
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
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDBitmapFlip {
	kBitmapUnflipped = 0,
	kBitmapFlippedX = 1,
	kBitmapFlippedY = 2,
	kBitmapFlippedXY = 3,
}
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDSolidColor {
	kColorBlack = 0,
	kColorWhite = 1,
	kColorClear = 2,
	kColorXOR = 3,
}
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDLineCapStyle {
	kLineCapStyleButt = 0,
	kLineCapStyleSquare = 1,
	kLineCapStyleRound = 2,
}
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDStringEncoding {
	kASCIIEncoding = 0,
	kUTF8Encoding = 1,
	k16BitLEEncoding = 2,
}
pub type LCDPattern = [u8; 16usize];
pub type LCDColor = usize;
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum LCDPolygonFillRule {
	kPolygonFillNonZero = 0,
	kPolygonFillEvenOdd = 1,
}
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDTextWrappingMode {
	kWrapClip = 0,
	kWrapCharacter = 1,
	kWrapWord = 2,
}
#[repr(u32)]
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
pub struct LCDTileMap {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDVideoPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct LCDStreamPlayer {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct HTTPConnection {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct TCPConnection {
	_unused: [u8; 0],
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_video"][::core::mem::size_of::<playdate_video>() - 64usize];
	["Alignment of playdate_video"][::core::mem::align_of::<playdate_video>() - 8usize];
	["Offset of field: playdate_video::loadVideo"][::core::mem::offset_of!(playdate_video, loadVideo) - 0usize];
	["Offset of field: playdate_video::freePlayer"][::core::mem::offset_of!(playdate_video, freePlayer) - 8usize];
	["Offset of field: playdate_video::setContext"][::core::mem::offset_of!(playdate_video, setContext) - 16usize];
	["Offset of field: playdate_video::useScreenContext"]
		[::core::mem::offset_of!(playdate_video, useScreenContext) - 24usize];
	["Offset of field: playdate_video::renderFrame"]
		[::core::mem::offset_of!(playdate_video, renderFrame) - 32usize];
	["Offset of field: playdate_video::getError"][::core::mem::offset_of!(playdate_video, getError) - 40usize];
	["Offset of field: playdate_video::getInfo"][::core::mem::offset_of!(playdate_video, getInfo) - 48usize];
	["Offset of field: playdate_video::getContext"][::core::mem::offset_of!(playdate_video, getContext) - 56usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_videostream {
	pub newPlayer: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDStreamPlayer>,
	pub freePlayer: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer)>,
	pub setBufferSize: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer,
	                                                               video: core::ffi::c_int,
	                                                               audio: core::ffi::c_int)>,
	pub setFile: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer, file: *mut SDFile)>,
	pub setHTTPConnection:
		::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer, conn: *mut HTTPConnection)>,
	pub getFilePlayer: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer) -> *mut FilePlayer>,
	pub getVideoPlayer:
		::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer) -> *mut LCDVideoPlayer>,
	pub update: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer) -> bool>,
	pub getBufferedFrameCount:
		::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer) -> core::ffi::c_int>,
	pub getBytesRead: ::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer) -> u32>,
	pub setTCPConnection:
		::core::option::Option<unsafe extern "C" fn(p: *mut LCDStreamPlayer, conn: *mut TCPConnection)>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_videostream"][::core::mem::size_of::<playdate_videostream>() - 88usize];
	["Alignment of playdate_videostream"][::core::mem::align_of::<playdate_videostream>() - 8usize];
	["Offset of field: playdate_videostream::newPlayer"]
		[::core::mem::offset_of!(playdate_videostream, newPlayer) - 0usize];
	["Offset of field: playdate_videostream::freePlayer"]
		[::core::mem::offset_of!(playdate_videostream, freePlayer) - 8usize];
	["Offset of field: playdate_videostream::setBufferSize"]
		[::core::mem::offset_of!(playdate_videostream, setBufferSize) - 16usize];
	["Offset of field: playdate_videostream::setFile"]
		[::core::mem::offset_of!(playdate_videostream, setFile) - 24usize];
	["Offset of field: playdate_videostream::setHTTPConnection"]
		[::core::mem::offset_of!(playdate_videostream, setHTTPConnection) - 32usize];
	["Offset of field: playdate_videostream::getFilePlayer"]
		[::core::mem::offset_of!(playdate_videostream, getFilePlayer) - 40usize];
	["Offset of field: playdate_videostream::getVideoPlayer"]
		[::core::mem::offset_of!(playdate_videostream, getVideoPlayer) - 48usize];
	["Offset of field: playdate_videostream::update"]
		[::core::mem::offset_of!(playdate_videostream, update) - 56usize];
	["Offset of field: playdate_videostream::getBufferedFrameCount"]
		[::core::mem::offset_of!(playdate_videostream, getBufferedFrameCount) - 64usize];
	["Offset of field: playdate_videostream::getBytesRead"]
		[::core::mem::offset_of!(playdate_videostream, getBytesRead) - 72usize];
	["Offset of field: playdate_videostream::setTCPConnection"]
		[::core::mem::offset_of!(playdate_videostream, setTCPConnection) - 80usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_tilemap {
	#[doc = "`LCDTileMap* playdate->graphics->tilemap->newTilemap(void)`\n\nCreates a new, empty LCDTileMap object."]
	pub newTilemap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDTileMap>,
	#[doc = "`void playdate->graphics->tilemap->freeTilemap(LCDTileMap* tilemap)`\n\nFrees an LCDTileMap object previously allocated with `playdate→graphics→tilemap→newTilemap()`."]
	pub freeTilemap: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap)>,
	#[doc = "`void playdate->graphics->tilemap->setImageTable(LCDTileMap* tilemap, LCDBitmapTable* table)`\n\nSets the image table to use for the tilemap’s tiles."]
	pub setImageTable:
		::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap, table: *mut LCDBitmapTable)>,
	#[doc = "`LCDBitmapTable* playdate->graphics->tilemap->getImageTable(LCDTileMap* tilemap)`\n\nReturns the LCDBitmapTable used for the tilemap’s tiles."]
	pub getImageTable: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap) -> *mut LCDBitmapTable>,
	#[doc = "`void playdate->graphics->tilemap->setSize(LCDTileMap* tilemap, int tilesWide, int tilesHigh)`\n\nSets the tilemap’s width and height, in number of tiles."]
	pub setSize: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap,
	                                                         tilesWide: core::ffi::c_int,
	                                                         tilesHigh: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->tilemap->getSize(LCDTileMap* tilemap, int* outwidth, int* outheight)`\n\nReturns the size of the tile map, in tiles."]
	pub getSize: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap,
	                                                         tilesWide: *mut core::ffi::c_int,
	                                                         tilesHigh: *mut core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->tilemap->getPixelSize(LCDTileMap* tilemap, uint32_t* outwidth, uint32_t* outheight)`\n\nReturns the size of the tilemap in pixels; that is, the size of the tile image multiplied by the number of rows and columns in the tilemap."]
	pub getPixelSize:
		::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap, outWidth: *mut u32, outHeight: *mut u32)>,
	#[doc = "`void playdate->graphics->tilemap->setTiles(LCDTileMap* tilemap, uint16_t* indexes, int count, int rowwidth)`\n\nSets the tilemap’s width to *rowwidth* and height to *count/rowwidth* (*count* must be evenly divisible by *rowwidth*), then sets the tiles' indexes to the given list."]
	pub setTiles: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap,
	                                                          indexes: *mut u16,
	                                                          count: core::ffi::c_int,
	                                                          rowwidth: core::ffi::c_int)>,
	#[doc = "`void playdate->graphics->tilemap->setTileAtPosition(LCDTileMap* tilemap, int x, int y, uint16_t idx)`\n\nSets the index of the tile at tilemap position (*x*, *y*). *index* is the (0-based) index of the cell in the tilemap’s image table."]
	pub setTileAtPosition: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap,
	                                                                   x: core::ffi::c_int,
	                                                                   y: core::ffi::c_int,
	                                                                   idx: u16)>,
	#[doc = "`int playdate->graphics->tilemap->getTileAtPosition(LCDTileMap* tilemap, int x, int y)`\n\nReturns the image index of the tile at the given *x* and *y* coordinate. If *x* or *y* is out of bounds, returns -1."]
	pub getTileAtPosition: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap,
	                                                                   x: core::ffi::c_int,
	                                                                   y: core::ffi::c_int)
	                                                                   -> core::ffi::c_int>,
	#[doc = "`void playdate->graphics->tilemap->drawAtPoint(LCDTileMap* m, float x, float y, LCDBitmapDrawMode mode)`\n\nDraws the tile map at coordinate (*x*, *y*)."]
	pub drawAtPoint: ::core::option::Option<unsafe extern "C" fn(m: *mut LCDTileMap,
	                                                             x: core::ffi::c_float,
	                                                             y: core::ffi::c_float)>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_tilemap"][::core::mem::size_of::<playdate_tilemap>() - 88usize];
	["Alignment of playdate_tilemap"][::core::mem::align_of::<playdate_tilemap>() - 8usize];
	["Offset of field: playdate_tilemap::newTilemap"]
		[::core::mem::offset_of!(playdate_tilemap, newTilemap) - 0usize];
	["Offset of field: playdate_tilemap::freeTilemap"]
		[::core::mem::offset_of!(playdate_tilemap, freeTilemap) - 8usize];
	["Offset of field: playdate_tilemap::setImageTable"]
		[::core::mem::offset_of!(playdate_tilemap, setImageTable) - 16usize];
	["Offset of field: playdate_tilemap::getImageTable"]
		[::core::mem::offset_of!(playdate_tilemap, getImageTable) - 24usize];
	["Offset of field: playdate_tilemap::setSize"][::core::mem::offset_of!(playdate_tilemap, setSize) - 32usize];
	["Offset of field: playdate_tilemap::getSize"][::core::mem::offset_of!(playdate_tilemap, getSize) - 40usize];
	["Offset of field: playdate_tilemap::getPixelSize"]
		[::core::mem::offset_of!(playdate_tilemap, getPixelSize) - 48usize];
	["Offset of field: playdate_tilemap::setTiles"][::core::mem::offset_of!(playdate_tilemap, setTiles) - 56usize];
	["Offset of field: playdate_tilemap::setTileAtPosition"]
		[::core::mem::offset_of!(playdate_tilemap, setTileAtPosition) - 64usize];
	["Offset of field: playdate_tilemap::getTileAtPosition"]
		[::core::mem::offset_of!(playdate_tilemap, getTileAtPosition) - 72usize];
	["Offset of field: playdate_tilemap::drawAtPoint"]
		[::core::mem::offset_of!(playdate_tilemap, drawAtPoint) - 80usize];
};
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
	#[doc = "`void playdate->graphics->drawRotatedBitmap(LCDBitmap* bitmap, int x, int y, float degrees, float centerx, float centery, float xscale, float yscale);`\n\nDraws the *bitmap* scaled to *xscale* and *yscale* then rotated by *degrees* with its center as given by proportions *centerx* and *centery* at *x*, *y*; that is: if *centerx* and *centery* are both 0.5 the center of the image is at (*x*,*y*), if *centerx* and *centery* are both 0 the top left corner of the image (before rotation) is at (*x*,*y*), etc."]
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
	#[doc = "`int playdate->graphics->getTextHeightForMaxWidth(LCDFont* font, const void* text, size_t len, int maxwidth, PDStringEncoding encoding, PDTextWrappingMode wrap, int tracking, int extraLeading);`\n\nReturns the height of *text* in the given *font*, when wrapped to *maxwidth* using the wrapping mode *wrap*. See the [note above](#f-graphics.drawText) about the `len` argument."]
	pub getTextHeightForMaxWidth: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont,
	                                                                          text: *const core::ffi::c_void,
	                                                                          len: usize,
	                                                                          maxwidth: core::ffi::c_int,
	                                                                          encoding: PDStringEncoding,
	                                                                          wrap: PDTextWrappingMode,
	                                                                          tracking: core::ffi::c_int,
	                                                                          extraLeading: core::ffi::c_int)
	                                                                          -> core::ffi::c_int>,
	#[doc = "`void playdate->graphics->drawRoundRect(int x, int y, int width, int height, int radius, int lineWidth, LCDColor color);`\n\nDraws a rectangle with rounded corners inside the rect with origin (*x*, *y*) and size (*width*, *height*) using the given *lineWidth*, *color*, and corner *radius*.\n\nEquivalent to [`playdate.graphics.drawRoundRect()`](./Inside%20Playdate.html#f-graphics.drawRoundRect) in the Lua API."]
	pub drawRoundRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                               y: core::ffi::c_int,
	                                                               width: core::ffi::c_int,
	                                                               height: core::ffi::c_int,
	                                                               radius: core::ffi::c_int,
	                                                               lineWidth: core::ffi::c_int,
	                                                               color: LCDColor)>,
	#[doc = "`void playdate->graphics->fillRoundRect(int x, int y, int width, int height, int radius, LCDColor color);`\n\nDraws a filled rectangle with rounded corners in the rect with origin (*x*, *y*) and size (*width*, *height*) using the given *color*, and corner *radius*.\n\nEquivalent to [`playdate.graphics.fillRoundRect()`](./Inside%20Playdate.html#f-graphics.fillRoundRect) in the Lua API."]
	pub fillRoundRect: ::core::option::Option<unsafe extern "C" fn(x: core::ffi::c_int,
	                                                               y: core::ffi::c_int,
	                                                               width: core::ffi::c_int,
	                                                               height: core::ffi::c_int,
	                                                               radius: core::ffi::c_int,
	                                                               color: LCDColor)>,
	pub tilemap: *const playdate_tilemap,
	pub videostream: *const playdate_videostream,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_graphics"][::core::mem::size_of::<playdate_graphics>() - 552usize];
	["Alignment of playdate_graphics"][::core::mem::align_of::<playdate_graphics>() - 8usize];
	["Offset of field: playdate_graphics::video"][::core::mem::offset_of!(playdate_graphics, video) - 0usize];
	["Offset of field: playdate_graphics::clear"][::core::mem::offset_of!(playdate_graphics, clear) - 8usize];
	["Offset of field: playdate_graphics::setBackgroundColor"]
		[::core::mem::offset_of!(playdate_graphics, setBackgroundColor) - 16usize];
	["Offset of field: playdate_graphics::setStencil"]
		[::core::mem::offset_of!(playdate_graphics, setStencil) - 24usize];
	["Offset of field: playdate_graphics::setDrawMode"]
		[::core::mem::offset_of!(playdate_graphics, setDrawMode) - 32usize];
	["Offset of field: playdate_graphics::setDrawOffset"]
		[::core::mem::offset_of!(playdate_graphics, setDrawOffset) - 40usize];
	["Offset of field: playdate_graphics::setClipRect"]
		[::core::mem::offset_of!(playdate_graphics, setClipRect) - 48usize];
	["Offset of field: playdate_graphics::clearClipRect"]
		[::core::mem::offset_of!(playdate_graphics, clearClipRect) - 56usize];
	["Offset of field: playdate_graphics::setLineCapStyle"]
		[::core::mem::offset_of!(playdate_graphics, setLineCapStyle) - 64usize];
	["Offset of field: playdate_graphics::setFont"][::core::mem::offset_of!(playdate_graphics, setFont) - 72usize];
	["Offset of field: playdate_graphics::setTextTracking"]
		[::core::mem::offset_of!(playdate_graphics, setTextTracking) - 80usize];
	["Offset of field: playdate_graphics::pushContext"]
		[::core::mem::offset_of!(playdate_graphics, pushContext) - 88usize];
	["Offset of field: playdate_graphics::popContext"]
		[::core::mem::offset_of!(playdate_graphics, popContext) - 96usize];
	["Offset of field: playdate_graphics::drawBitmap"]
		[::core::mem::offset_of!(playdate_graphics, drawBitmap) - 104usize];
	["Offset of field: playdate_graphics::tileBitmap"]
		[::core::mem::offset_of!(playdate_graphics, tileBitmap) - 112usize];
	["Offset of field: playdate_graphics::drawLine"]
		[::core::mem::offset_of!(playdate_graphics, drawLine) - 120usize];
	["Offset of field: playdate_graphics::fillTriangle"]
		[::core::mem::offset_of!(playdate_graphics, fillTriangle) - 128usize];
	["Offset of field: playdate_graphics::drawRect"]
		[::core::mem::offset_of!(playdate_graphics, drawRect) - 136usize];
	["Offset of field: playdate_graphics::fillRect"]
		[::core::mem::offset_of!(playdate_graphics, fillRect) - 144usize];
	["Offset of field: playdate_graphics::drawEllipse"]
		[::core::mem::offset_of!(playdate_graphics, drawEllipse) - 152usize];
	["Offset of field: playdate_graphics::fillEllipse"]
		[::core::mem::offset_of!(playdate_graphics, fillEllipse) - 160usize];
	["Offset of field: playdate_graphics::drawScaledBitmap"]
		[::core::mem::offset_of!(playdate_graphics, drawScaledBitmap) - 168usize];
	["Offset of field: playdate_graphics::drawText"]
		[::core::mem::offset_of!(playdate_graphics, drawText) - 176usize];
	["Offset of field: playdate_graphics::newBitmap"]
		[::core::mem::offset_of!(playdate_graphics, newBitmap) - 184usize];
	["Offset of field: playdate_graphics::freeBitmap"]
		[::core::mem::offset_of!(playdate_graphics, freeBitmap) - 192usize];
	["Offset of field: playdate_graphics::loadBitmap"]
		[::core::mem::offset_of!(playdate_graphics, loadBitmap) - 200usize];
	["Offset of field: playdate_graphics::copyBitmap"]
		[::core::mem::offset_of!(playdate_graphics, copyBitmap) - 208usize];
	["Offset of field: playdate_graphics::loadIntoBitmap"]
		[::core::mem::offset_of!(playdate_graphics, loadIntoBitmap) - 216usize];
	["Offset of field: playdate_graphics::getBitmapData"]
		[::core::mem::offset_of!(playdate_graphics, getBitmapData) - 224usize];
	["Offset of field: playdate_graphics::clearBitmap"]
		[::core::mem::offset_of!(playdate_graphics, clearBitmap) - 232usize];
	["Offset of field: playdate_graphics::rotatedBitmap"]
		[::core::mem::offset_of!(playdate_graphics, rotatedBitmap) - 240usize];
	["Offset of field: playdate_graphics::newBitmapTable"]
		[::core::mem::offset_of!(playdate_graphics, newBitmapTable) - 248usize];
	["Offset of field: playdate_graphics::freeBitmapTable"]
		[::core::mem::offset_of!(playdate_graphics, freeBitmapTable) - 256usize];
	["Offset of field: playdate_graphics::loadBitmapTable"]
		[::core::mem::offset_of!(playdate_graphics, loadBitmapTable) - 264usize];
	["Offset of field: playdate_graphics::loadIntoBitmapTable"]
		[::core::mem::offset_of!(playdate_graphics, loadIntoBitmapTable) - 272usize];
	["Offset of field: playdate_graphics::getTableBitmap"]
		[::core::mem::offset_of!(playdate_graphics, getTableBitmap) - 280usize];
	["Offset of field: playdate_graphics::loadFont"]
		[::core::mem::offset_of!(playdate_graphics, loadFont) - 288usize];
	["Offset of field: playdate_graphics::getFontPage"]
		[::core::mem::offset_of!(playdate_graphics, getFontPage) - 296usize];
	["Offset of field: playdate_graphics::getPageGlyph"]
		[::core::mem::offset_of!(playdate_graphics, getPageGlyph) - 304usize];
	["Offset of field: playdate_graphics::getGlyphKerning"]
		[::core::mem::offset_of!(playdate_graphics, getGlyphKerning) - 312usize];
	["Offset of field: playdate_graphics::getTextWidth"]
		[::core::mem::offset_of!(playdate_graphics, getTextWidth) - 320usize];
	["Offset of field: playdate_graphics::getFrame"]
		[::core::mem::offset_of!(playdate_graphics, getFrame) - 328usize];
	["Offset of field: playdate_graphics::getDisplayFrame"]
		[::core::mem::offset_of!(playdate_graphics, getDisplayFrame) - 336usize];
	["Offset of field: playdate_graphics::getDebugBitmap"]
		[::core::mem::offset_of!(playdate_graphics, getDebugBitmap) - 344usize];
	["Offset of field: playdate_graphics::copyFrameBufferBitmap"]
		[::core::mem::offset_of!(playdate_graphics, copyFrameBufferBitmap) - 352usize];
	["Offset of field: playdate_graphics::markUpdatedRows"]
		[::core::mem::offset_of!(playdate_graphics, markUpdatedRows) - 360usize];
	["Offset of field: playdate_graphics::display"]
		[::core::mem::offset_of!(playdate_graphics, display) - 368usize];
	["Offset of field: playdate_graphics::setColorToPattern"]
		[::core::mem::offset_of!(playdate_graphics, setColorToPattern) - 376usize];
	["Offset of field: playdate_graphics::checkMaskCollision"]
		[::core::mem::offset_of!(playdate_graphics, checkMaskCollision) - 384usize];
	["Offset of field: playdate_graphics::setScreenClipRect"]
		[::core::mem::offset_of!(playdate_graphics, setScreenClipRect) - 392usize];
	["Offset of field: playdate_graphics::fillPolygon"]
		[::core::mem::offset_of!(playdate_graphics, fillPolygon) - 400usize];
	["Offset of field: playdate_graphics::getFontHeight"]
		[::core::mem::offset_of!(playdate_graphics, getFontHeight) - 408usize];
	["Offset of field: playdate_graphics::getDisplayBufferBitmap"]
		[::core::mem::offset_of!(playdate_graphics, getDisplayBufferBitmap) - 416usize];
	["Offset of field: playdate_graphics::drawRotatedBitmap"]
		[::core::mem::offset_of!(playdate_graphics, drawRotatedBitmap) - 424usize];
	["Offset of field: playdate_graphics::setTextLeading"]
		[::core::mem::offset_of!(playdate_graphics, setTextLeading) - 432usize];
	["Offset of field: playdate_graphics::setBitmapMask"]
		[::core::mem::offset_of!(playdate_graphics, setBitmapMask) - 440usize];
	["Offset of field: playdate_graphics::getBitmapMask"]
		[::core::mem::offset_of!(playdate_graphics, getBitmapMask) - 448usize];
	["Offset of field: playdate_graphics::setStencilImage"]
		[::core::mem::offset_of!(playdate_graphics, setStencilImage) - 456usize];
	["Offset of field: playdate_graphics::makeFontFromData"]
		[::core::mem::offset_of!(playdate_graphics, makeFontFromData) - 464usize];
	["Offset of field: playdate_graphics::getTextTracking"]
		[::core::mem::offset_of!(playdate_graphics, getTextTracking) - 472usize];
	["Offset of field: playdate_graphics::setPixel"]
		[::core::mem::offset_of!(playdate_graphics, setPixel) - 480usize];
	["Offset of field: playdate_graphics::getBitmapPixel"]
		[::core::mem::offset_of!(playdate_graphics, getBitmapPixel) - 488usize];
	["Offset of field: playdate_graphics::getBitmapTableInfo"]
		[::core::mem::offset_of!(playdate_graphics, getBitmapTableInfo) - 496usize];
	["Offset of field: playdate_graphics::drawTextInRect"]
		[::core::mem::offset_of!(playdate_graphics, drawTextInRect) - 504usize];
	["Offset of field: playdate_graphics::getTextHeightForMaxWidth"]
		[::core::mem::offset_of!(playdate_graphics, getTextHeightForMaxWidth) - 512usize];
	["Offset of field: playdate_graphics::drawRoundRect"]
		[::core::mem::offset_of!(playdate_graphics, drawRoundRect) - 520usize];
	["Offset of field: playdate_graphics::fillRoundRect"]
		[::core::mem::offset_of!(playdate_graphics, fillRoundRect) - 528usize];
	["Offset of field: playdate_graphics::tilemap"]
		[::core::mem::offset_of!(playdate_graphics, tilemap) - 536usize];
	["Offset of field: playdate_graphics::videostream"]
		[::core::mem::offset_of!(playdate_graphics, videostream) - 544usize];
};
impl Default for playdate_graphics {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
pub type va_list = __builtin_va_list;
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
pub struct PDButtons(pub u32);
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDLanguage {
	kPDLanguageEnglish = 0,
	kPDLanguageJapanese = 1,
	kPDLanguageUnknown = 2,
}
pub type AccessRequestCallback =
	::core::option::Option<unsafe extern "C" fn(allowed: bool, userdata: *mut core::ffi::c_void)>;
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum accessReply {
	kAccessAsk = 0,
	kAccessDeny = 1,
	kAccessAllow = 2,
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PDDateTime"][::core::mem::size_of::<PDDateTime>() - 8usize];
	["Alignment of PDDateTime"][::core::mem::align_of::<PDDateTime>() - 2usize];
	["Offset of field: PDDateTime::year"][::core::mem::offset_of!(PDDateTime, year) - 0usize];
	["Offset of field: PDDateTime::month"][::core::mem::offset_of!(PDDateTime, month) - 2usize];
	["Offset of field: PDDateTime::day"][::core::mem::offset_of!(PDDateTime, day) - 3usize];
	["Offset of field: PDDateTime::weekday"][::core::mem::offset_of!(PDDateTime, weekday) - 4usize];
	["Offset of field: PDDateTime::hour"][::core::mem::offset_of!(PDDateTime, hour) - 5usize];
	["Offset of field: PDDateTime::minute"][::core::mem::offset_of!(PDDateTime, minute) - 6usize];
	["Offset of field: PDDateTime::second"][::core::mem::offset_of!(PDDateTime, second) - 7usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct PDMenuItem {
	_unused: [u8; 0],
}
#[repr(u32)]
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
pub struct playdate_sys { # [doc = "`void* playdate->system->realloc(void* ptr, size_t size)`\n\nAllocates heap space if *ptr* is NULL, else reallocates the given pointer. If *size* is zero, frees the given pointer."] pub realloc : :: core :: option :: Option < unsafe extern "C" fn (ptr : * mut core :: ffi :: c_void , size : usize) -> * mut core :: ffi :: c_void > , # [doc = "`int playdate->system->formatString(char **outstring, const char *format, ...)`\n\nCreates a formatted string and returns it via the *outstring* argument. The arguments and return value match libc’s `asprintf()`: the format string is standard `printf()` style, the string returned in *outstring* should be freed by the caller when it’s no longer in use, and the return value is the length of the formatted string."] pub formatString : :: core :: option :: Option < unsafe extern "C" fn (ret : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int > , # [doc = "`void playdate->system->logToConsole(const char* format, ...)`\n\nCalls the log function.\n\nEquivalent to [`print()`](./Inside%20Playdate.html#f-print) in the Lua API."] pub logToConsole : :: core :: option :: Option < unsafe extern "C" fn (fmt : * const core :: ffi :: c_char , ...) > , # [doc = "`void playdate->system->error(const char* format, ...)`\n\nCalls the log function, outputting an error in red to the console, then pauses execution."] pub error : :: core :: option :: Option < unsafe extern "C" fn (fmt : * const core :: ffi :: c_char , ...) > , # [doc = "`PDLanguage playdate->system->getLanguage(void);`\n\nReturns the current language of the system."] pub getLanguage : :: core :: option :: Option < unsafe extern "C" fn () -> PDLanguage > , # [doc = "`unsigned int playdate->system->getCurrentTimeMilliseconds(void)`\n\nReturns the number of milliseconds since…\u{200b}some arbitrary point in time. This should present a consistent timebase while a game is running, but the counter will be disabled when the device is sleeping."] pub getCurrentTimeMilliseconds : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_uint > , # [doc = "`unsigned int playdate->system->getSecondsSinceEpoch(unsigned int *milliseconds)`\n\nReturns the number of seconds (and sets *milliseconds* if not NULL) elapsed since midnight (hour 0), January 1, 2000."] pub getSecondsSinceEpoch : :: core :: option :: Option < unsafe extern "C" fn (milliseconds : * mut core :: ffi :: c_uint) -> core :: ffi :: c_uint > , # [doc = "`void playdate->system->drawFPS(int x, int y)`\n\nCalculates the current frames per second and draws that value at *x, y*."] pub drawFPS : :: core :: option :: Option < unsafe extern "C" fn (x : core :: ffi :: c_int , y : core :: ffi :: c_int) > , # [doc = "`void playdate->system->setUpdateCallback(PDCallbackFunction* update, void* userdata)`\n\nPDCallbackFunction\n\n```cpp\nint PDCallbackFunction(void* userdata);\n```\n\nReplaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed."] pub setUpdateCallback : :: core :: option :: Option < unsafe extern "C" fn (update : PDCallbackFunction , userdata : * mut core :: ffi :: c_void) > , # [doc = "`void playdate->system->getButtonState(PDButtons* current, PDButtons* pushed, PDButtons* released)`\n\nSets the value pointed to by *current* to a bitmask indicating which buttons are currently down. *pushed* and *released* reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.\n\nPDButton\n\n```cpp\nkButtonLeft\nkButtonRight\nkButtonUp\nkButtonDown\nkButtonB\nkButtonA\n```"] pub getButtonState : :: core :: option :: Option < unsafe extern "C" fn (current : * mut PDButtons , pushed : * mut PDButtons , released : * mut PDButtons) > , # [doc = "`void playdate->system->setPeripheralsEnabled(PDPeripherals mask)`\n\nBy default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.\n\nPDPeripherals\n\n```cpp\nkNone\nkAccelerometer\n```"] pub setPeripheralsEnabled : :: core :: option :: Option < unsafe extern "C" fn (mask : PDPeripherals) > , # [doc = "`void playdate->system->getAccelerometer(float* outx, float* outy, float* outz)`\n\nReturns the last-read accelerometer data."] pub getAccelerometer : :: core :: option :: Option < unsafe extern "C" fn (outx : * mut core :: ffi :: c_float , outy : * mut core :: ffi :: c_float , outz : * mut core :: ffi :: c_float) > , # [doc = "`float playdate->system->getCrankChange(void)`\n\nReturns the angle change of the crank since the last time this function was called. Negative values are anti-clockwise."] pub getCrankChange : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`float playdate->system->getCrankAngle(void)`\n\nReturns the current position of the crank, in the range 0-360. Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device."] pub getCrankAngle : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`int playdate->system->isCrankDocked(void)`\n\nReturns 1 or 0 indicating whether or not the crank is folded into the unit."] pub isCrankDocked : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`int playdate->system->setCrankSoundsDisabled(int disable)`\n\nThe function returns the previous value for this setting."] pub setCrankSoundsDisabled : :: core :: option :: Option < unsafe extern "C" fn (flag : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->system->getFlipped()`\n\nReturns 1 if the global \"flipped\" system setting is set, otherwise 0."] pub getFlipped : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`void playdate->system->setAutoLockDisabled(int disable)`\n\nDisables or enables the 3 minute auto lock feature. When called, the timer is reset to 3 minutes."] pub setAutoLockDisabled : :: core :: option :: Option < unsafe extern "C" fn (disable : core :: ffi :: c_int) > , # [doc = "`void playdate->system->setMenuImage(LCDBitmap* bitmap, int xOffset);`\n\nA game can optionally provide an image to be displayed alongside the system menu. *bitmap* must be a 400x240 LCDBitmap. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.\n\nOptionally, a non-zero *xoffset*, can be provided. This must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by xoffset pixels as the menu is animated in.\n\nThis function could be called in response to the kEventPause *event* in your implementation of [eventHandler()](#_eventHandler)."] pub setMenuImage : :: core :: option :: Option < unsafe extern "C" fn (bitmap : * mut LCDBitmap , xOffset : core :: ffi :: c_int) > , # [doc = "`PDMenuItem* playdate->system->addMenuItem(const char* title, PDMenuItemCallbackFunction* callback, void* userdata)`\n\n*title* will be the title displayed by the menu item.\n\nAdds a new menu item to the System Menu. When invoked by the user, this menu item will:\n\n1. Invoke your *callback* function.\n\n2. Hide the System Menu.\n\n3. Unpause your game and call [eventHandler()](#_eventHandler) with the kEventResume *event*.\n\nYour game can then present an options interface to the player, or take other action, in whatever manner you choose.\n\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually."] pub addMenuItem : :: core :: option :: Option < unsafe extern "C" fn (title : * const core :: ffi :: c_char , callback : PDMenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut PDMenuItem > , # [doc = "`PDMenuItem* playdate->system->addCheckmarkMenuItem(const char* title, int value, PDMenuItemCallbackFunction* callback, void* userdata)`\n\nAdds a new menu item that can be checked or unchecked by the player.\n\n*title* will be the title displayed by the menu item.\n\n*value* should be 0 for unchecked, 1 for checked.\n\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\n\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually."] pub addCheckmarkMenuItem : :: core :: option :: Option < unsafe extern "C" fn (title : * const core :: ffi :: c_char , value : core :: ffi :: c_int , callback : PDMenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut PDMenuItem > , # [doc = "`PDMenuItem* playdate->system->addOptionsMenuItem(const char* title, const char** options, int optionsCount, PDMenuItemCallbackFunction* callback, void* userdata)`\n\nAdds a new menu item that allows the player to cycle through a set of options.\n\n*title* will be the title displayed by the menu item.\n\n*options* should be an array of strings representing the states this menu item can cycle through. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.\n\n*optionsCount* should be the number of items contained in *options*.\n\nIf this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed.\n\nThe returned menu item is freed when removed from the menu; it does not need to be freed manually."] pub addOptionsMenuItem : :: core :: option :: Option < unsafe extern "C" fn (title : * const core :: ffi :: c_char , optionTitles : * mut * const core :: ffi :: c_char , optionsCount : core :: ffi :: c_int , f : PDMenuItemCallbackFunction , userdata : * mut core :: ffi :: c_void) -> * mut PDMenuItem > , # [doc = "`void playdate->system->removeAllMenuItems()`\n\nRemoves all custom menu items from the system menu."] pub removeAllMenuItems : :: core :: option :: Option < unsafe extern "C" fn () > , # [doc = "`void playdate->system->removeMenuItem(PDMenuItem *menuItem)`\n\nRemoves the menu item from the system menu."] pub removeMenuItem : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) > , # [doc = "`int playdate->system->getMenuItemValue(PDMenuItem *menuItem)`"] pub getMenuItemValue : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) -> core :: ffi :: c_int > , # [doc = "`void playdate->system->setMenuItemValue(PDMenuItem *menuItem, int value)`\n\nGets or sets the integer value of the menu item.\n\nFor checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option."] pub setMenuItemValue : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem , value : core :: ffi :: c_int) > , # [doc = "`const char* playdate->system->getMenuItemTitle(PDMenuItem *menuItem)`"] pub getMenuItemTitle : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) -> * const core :: ffi :: c_char > , # [doc = "`void playdate->system->setMenuItemTitle(PDMenuItem *menuItem, const char* title)`\n\nGets or sets the display title of the menu item."] pub setMenuItemTitle : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem , title : * const core :: ffi :: c_char) > , # [doc = "`void* playdate->system->getMenuItemUserdata(PDMenuItem *menuItem)`"] pub getMenuItemUserdata : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem) -> * mut core :: ffi :: c_void > , # [doc = "`void playdate->system->setMenuItemUserdata(PDMenuItem *menuItem, void* userdata)`\n\nGets or sets the userdata value associated with this menu item."] pub setMenuItemUserdata : :: core :: option :: Option < unsafe extern "C" fn (menuItem : * mut PDMenuItem , ud : * mut core :: ffi :: c_void) > , # [doc = "`int playdate->system->getReduceFlashing()`\n\nReturns 1 if the global \"reduce flashing\" system setting is set, otherwise 0."] pub getReduceFlashing : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`float playdate->system->getElapsedTime()`\n\nReturns the number of seconds since `playdate.resetElapsedTime()` was called. The value is a floating-point number with microsecond accuracy."] pub getElapsedTime : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`void playdate->system->resetElapsedTime(void)`\n\nResets the high-resolution timer."] pub resetElapsedTime : :: core :: option :: Option < unsafe extern "C" fn () > , # [doc = "`float playdate->system->getBatteryPercentage()`\n\nReturns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full."] pub getBatteryPercentage : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`float playdate->system->getBatteryVoltage()`\n\nReturns the battery’s current voltage level."] pub getBatteryVoltage : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_float > , # [doc = "`int32_t playdate->system->getTimezoneOffset()`\n\nReturns the system timezone offset from GMT, in seconds."] pub getTimezoneOffset : :: core :: option :: Option < unsafe extern "C" fn () -> i32 > , # [doc = "`int playdate->system->shouldDisplay24HourTime()`\n\nReturns 1 if the user has set the 24-Hour Time preference in the Settings program."] pub shouldDisplay24HourTime : :: core :: option :: Option < unsafe extern "C" fn () -> core :: ffi :: c_int > , # [doc = "`void playdate->system->convertEpochToDateTime(uint32_t epoch, struct PDDateTime* datetime)`\n\nConverts the given epoch time to a PDDateTime."] pub convertEpochToDateTime : :: core :: option :: Option < unsafe extern "C" fn (epoch : u32 , datetime : * mut PDDateTime) > , # [doc = "`uint32_t playdate->system->convertDateTimeToEpoch(struct PDDateTime* datetime)`\n\nConverts the given PDDateTime to an epoch time."] pub convertDateTimeToEpoch : :: core :: option :: Option < unsafe extern "C" fn (datetime : * mut PDDateTime) -> u32 > , # [doc = "`float playdate->system->clearICache()`\n\nFlush the CPU instruction cache, on the very unlikely chance you’re modifying instruction code on the fly. (If you don’t know what I’m talking about, you don’t need this. :smile:)"] pub clearICache : :: core :: option :: Option < unsafe extern "C" fn () > , # [doc = "`void playdate->system->setButtonCallback(PDButtonCallbackFunction* cb, void* userdata, int queuesize)`\n\nAs an alternative to polling for button presses using `getButtonState()`, this function allows a callback function to be set. The function is called for each button up/down event (possibly multiple events on the same button) that occurred during the previous update cycle. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. The function should return 0 on success or a non-zero value to signal an error.\n\nPDButtonCallbackFunction\n\n```cpp\ntypedef int PDButtonCallbackFunction(PDButtons button, int down, uint32_t when, void* userdata);\n```"] pub setButtonCallback : :: core :: option :: Option < unsafe extern "C" fn (cb : PDButtonCallbackFunction , buttonud : * mut core :: ffi :: c_void , queuesize : core :: ffi :: c_int) > , # [doc = "`void playdate->system->setSerialMessageCallback(void (*callback)(const char* data));`\n\nProvides a callback to receive messages sent to the device over the serial port using the `msg` command. If no device is connected, you can send these messages to a game in the simulator by entering `!msg <message>` in the Lua console."] pub setSerialMessageCallback : :: core :: option :: Option < unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (data : * const core :: ffi :: c_char) >) > , # [doc = "`int playdate->system->vaFormatString(char **ret, const char *format, va_list args)`\n\nAllocates and formats a string using a variadic `va_list` argument, in the style of `vasprintf()`. The string returned via *ret* should be freed by the caller when it is no longer in use. The return value from the function is the length of the formatted string."] pub vaFormatString : :: core :: option :: Option < unsafe extern "C" fn (outstr : * mut * mut core :: ffi :: c_char , fmt : * const core :: ffi :: c_char , args : * mut va_list) -> core :: ffi :: c_int > , # [doc = "`int playdate->system->parseString(const char *str, const char *format, ...)`\n\nLike libc `sscanf()`, parses a string according to a format string and places the values into pointers passed in after the format. The return value is the number of items matched."] pub parseString : :: core :: option :: Option < unsafe extern "C" fn (str_ : * const core :: ffi :: c_char , format : * const core :: ffi :: c_char , ...) -> core :: ffi :: c_int > , # [doc = "`float playdate->system->delay(uint32_t milliseconds)`\n\nPauses execution for the given number of milliseconds."] pub delay : :: core :: option :: Option < unsafe extern "C" fn (milliseconds : u32) > , # [doc = "`void playdate->system->getServerTime(void (*callback)(const char* time, const char* err))`\n\nQueries the Playdate server for the current time, in seconds elapsed since midnight (hour 0), January 1 2000 UTC. This provides games with a reliable clock source, since the internal clock can be set by the user. The function is asynchronous, returning the server time to a callback function passed in. If an error occurred fetching the time, the `error` argument is set instead."] pub getServerTime : :: core :: option :: Option < unsafe extern "C" fn (callback : :: core :: option :: Option < unsafe extern "C" fn (time : * const core :: ffi :: c_char , err : * const core :: ffi :: c_char) >) > , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sys"][::core::mem::size_of::<playdate_sys>() - 368usize];
	["Alignment of playdate_sys"][::core::mem::align_of::<playdate_sys>() - 8usize];
	["Offset of field: playdate_sys::realloc"][::core::mem::offset_of!(playdate_sys, realloc) - 0usize];
	["Offset of field: playdate_sys::formatString"][::core::mem::offset_of!(playdate_sys, formatString) - 8usize];
	["Offset of field: playdate_sys::logToConsole"][::core::mem::offset_of!(playdate_sys, logToConsole) - 16usize];
	["Offset of field: playdate_sys::error"][::core::mem::offset_of!(playdate_sys, error) - 24usize];
	["Offset of field: playdate_sys::getLanguage"][::core::mem::offset_of!(playdate_sys, getLanguage) - 32usize];
	["Offset of field: playdate_sys::getCurrentTimeMilliseconds"]
		[::core::mem::offset_of!(playdate_sys, getCurrentTimeMilliseconds) - 40usize];
	["Offset of field: playdate_sys::getSecondsSinceEpoch"]
		[::core::mem::offset_of!(playdate_sys, getSecondsSinceEpoch) - 48usize];
	["Offset of field: playdate_sys::drawFPS"][::core::mem::offset_of!(playdate_sys, drawFPS) - 56usize];
	["Offset of field: playdate_sys::setUpdateCallback"]
		[::core::mem::offset_of!(playdate_sys, setUpdateCallback) - 64usize];
	["Offset of field: playdate_sys::getButtonState"]
		[::core::mem::offset_of!(playdate_sys, getButtonState) - 72usize];
	["Offset of field: playdate_sys::setPeripheralsEnabled"]
		[::core::mem::offset_of!(playdate_sys, setPeripheralsEnabled) - 80usize];
	["Offset of field: playdate_sys::getAccelerometer"]
		[::core::mem::offset_of!(playdate_sys, getAccelerometer) - 88usize];
	["Offset of field: playdate_sys::getCrankChange"]
		[::core::mem::offset_of!(playdate_sys, getCrankChange) - 96usize];
	["Offset of field: playdate_sys::getCrankAngle"]
		[::core::mem::offset_of!(playdate_sys, getCrankAngle) - 104usize];
	["Offset of field: playdate_sys::isCrankDocked"]
		[::core::mem::offset_of!(playdate_sys, isCrankDocked) - 112usize];
	["Offset of field: playdate_sys::setCrankSoundsDisabled"]
		[::core::mem::offset_of!(playdate_sys, setCrankSoundsDisabled) - 120usize];
	["Offset of field: playdate_sys::getFlipped"][::core::mem::offset_of!(playdate_sys, getFlipped) - 128usize];
	["Offset of field: playdate_sys::setAutoLockDisabled"]
		[::core::mem::offset_of!(playdate_sys, setAutoLockDisabled) - 136usize];
	["Offset of field: playdate_sys::setMenuImage"]
		[::core::mem::offset_of!(playdate_sys, setMenuImage) - 144usize];
	["Offset of field: playdate_sys::addMenuItem"][::core::mem::offset_of!(playdate_sys, addMenuItem) - 152usize];
	["Offset of field: playdate_sys::addCheckmarkMenuItem"]
		[::core::mem::offset_of!(playdate_sys, addCheckmarkMenuItem) - 160usize];
	["Offset of field: playdate_sys::addOptionsMenuItem"]
		[::core::mem::offset_of!(playdate_sys, addOptionsMenuItem) - 168usize];
	["Offset of field: playdate_sys::removeAllMenuItems"]
		[::core::mem::offset_of!(playdate_sys, removeAllMenuItems) - 176usize];
	["Offset of field: playdate_sys::removeMenuItem"]
		[::core::mem::offset_of!(playdate_sys, removeMenuItem) - 184usize];
	["Offset of field: playdate_sys::getMenuItemValue"]
		[::core::mem::offset_of!(playdate_sys, getMenuItemValue) - 192usize];
	["Offset of field: playdate_sys::setMenuItemValue"]
		[::core::mem::offset_of!(playdate_sys, setMenuItemValue) - 200usize];
	["Offset of field: playdate_sys::getMenuItemTitle"]
		[::core::mem::offset_of!(playdate_sys, getMenuItemTitle) - 208usize];
	["Offset of field: playdate_sys::setMenuItemTitle"]
		[::core::mem::offset_of!(playdate_sys, setMenuItemTitle) - 216usize];
	["Offset of field: playdate_sys::getMenuItemUserdata"]
		[::core::mem::offset_of!(playdate_sys, getMenuItemUserdata) - 224usize];
	["Offset of field: playdate_sys::setMenuItemUserdata"]
		[::core::mem::offset_of!(playdate_sys, setMenuItemUserdata) - 232usize];
	["Offset of field: playdate_sys::getReduceFlashing"]
		[::core::mem::offset_of!(playdate_sys, getReduceFlashing) - 240usize];
	["Offset of field: playdate_sys::getElapsedTime"]
		[::core::mem::offset_of!(playdate_sys, getElapsedTime) - 248usize];
	["Offset of field: playdate_sys::resetElapsedTime"]
		[::core::mem::offset_of!(playdate_sys, resetElapsedTime) - 256usize];
	["Offset of field: playdate_sys::getBatteryPercentage"]
		[::core::mem::offset_of!(playdate_sys, getBatteryPercentage) - 264usize];
	["Offset of field: playdate_sys::getBatteryVoltage"]
		[::core::mem::offset_of!(playdate_sys, getBatteryVoltage) - 272usize];
	["Offset of field: playdate_sys::getTimezoneOffset"]
		[::core::mem::offset_of!(playdate_sys, getTimezoneOffset) - 280usize];
	["Offset of field: playdate_sys::shouldDisplay24HourTime"]
		[::core::mem::offset_of!(playdate_sys, shouldDisplay24HourTime) - 288usize];
	["Offset of field: playdate_sys::convertEpochToDateTime"]
		[::core::mem::offset_of!(playdate_sys, convertEpochToDateTime) - 296usize];
	["Offset of field: playdate_sys::convertDateTimeToEpoch"]
		[::core::mem::offset_of!(playdate_sys, convertDateTimeToEpoch) - 304usize];
	["Offset of field: playdate_sys::clearICache"][::core::mem::offset_of!(playdate_sys, clearICache) - 312usize];
	["Offset of field: playdate_sys::setButtonCallback"]
		[::core::mem::offset_of!(playdate_sys, setButtonCallback) - 320usize];
	["Offset of field: playdate_sys::setSerialMessageCallback"]
		[::core::mem::offset_of!(playdate_sys, setSerialMessageCallback) - 328usize];
	["Offset of field: playdate_sys::vaFormatString"]
		[::core::mem::offset_of!(playdate_sys, vaFormatString) - 336usize];
	["Offset of field: playdate_sys::parseString"][::core::mem::offset_of!(playdate_sys, parseString) - 344usize];
	["Offset of field: playdate_sys::delay"][::core::mem::offset_of!(playdate_sys, delay) - 352usize];
	["Offset of field: playdate_sys::getServerTime"]
		[::core::mem::offset_of!(playdate_sys, getServerTime) - 360usize];
};
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
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of lua_reg"][::core::mem::size_of::<lua_reg>() - 16usize];
	["Alignment of lua_reg"][::core::mem::align_of::<lua_reg>() - 8usize];
	["Offset of field: lua_reg::name"][::core::mem::offset_of!(lua_reg, name) - 0usize];
	["Offset of field: lua_reg::func"][::core::mem::offset_of!(lua_reg, func) - 8usize];
};
impl Default for lua_reg {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of lua_val__bindgen_ty_1"][::core::mem::size_of::<lua_val__bindgen_ty_1>() - 8usize];
	["Alignment of lua_val__bindgen_ty_1"][::core::mem::align_of::<lua_val__bindgen_ty_1>() - 8usize];
	["Offset of field: lua_val__bindgen_ty_1::intval"]
		[::core::mem::offset_of!(lua_val__bindgen_ty_1, intval) - 0usize];
	["Offset of field: lua_val__bindgen_ty_1::floatval"]
		[::core::mem::offset_of!(lua_val__bindgen_ty_1, floatval) - 0usize];
	["Offset of field: lua_val__bindgen_ty_1::strval"]
		[::core::mem::offset_of!(lua_val__bindgen_ty_1, strval) - 0usize];
};
impl Default for lua_val__bindgen_ty_1 {
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
	["Size of lua_val"][::core::mem::size_of::<lua_val>() - 24usize];
	["Alignment of lua_val"][::core::mem::align_of::<lua_val>() - 8usize];
	["Offset of field: lua_val::name"][::core::mem::offset_of!(lua_val, name) - 0usize];
	["Offset of field: lua_val::type_"][::core::mem::offset_of!(lua_val, type_) - 8usize];
	["Offset of field: lua_val::v"][::core::mem::offset_of!(lua_val, v) - 16usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_lua"][::core::mem::size_of::<playdate_lua>() - 256usize];
	["Alignment of playdate_lua"][::core::mem::align_of::<playdate_lua>() - 8usize];
	["Offset of field: playdate_lua::addFunction"][::core::mem::offset_of!(playdate_lua, addFunction) - 0usize];
	["Offset of field: playdate_lua::registerClass"]
		[::core::mem::offset_of!(playdate_lua, registerClass) - 8usize];
	["Offset of field: playdate_lua::pushFunction"][::core::mem::offset_of!(playdate_lua, pushFunction) - 16usize];
	["Offset of field: playdate_lua::indexMetatable"]
		[::core::mem::offset_of!(playdate_lua, indexMetatable) - 24usize];
	["Offset of field: playdate_lua::stop"][::core::mem::offset_of!(playdate_lua, stop) - 32usize];
	["Offset of field: playdate_lua::start"][::core::mem::offset_of!(playdate_lua, start) - 40usize];
	["Offset of field: playdate_lua::getArgCount"][::core::mem::offset_of!(playdate_lua, getArgCount) - 48usize];
	["Offset of field: playdate_lua::getArgType"][::core::mem::offset_of!(playdate_lua, getArgType) - 56usize];
	["Offset of field: playdate_lua::argIsNil"][::core::mem::offset_of!(playdate_lua, argIsNil) - 64usize];
	["Offset of field: playdate_lua::getArgBool"][::core::mem::offset_of!(playdate_lua, getArgBool) - 72usize];
	["Offset of field: playdate_lua::getArgInt"][::core::mem::offset_of!(playdate_lua, getArgInt) - 80usize];
	["Offset of field: playdate_lua::getArgFloat"][::core::mem::offset_of!(playdate_lua, getArgFloat) - 88usize];
	["Offset of field: playdate_lua::getArgString"][::core::mem::offset_of!(playdate_lua, getArgString) - 96usize];
	["Offset of field: playdate_lua::getArgBytes"][::core::mem::offset_of!(playdate_lua, getArgBytes) - 104usize];
	["Offset of field: playdate_lua::getArgObject"]
		[::core::mem::offset_of!(playdate_lua, getArgObject) - 112usize];
	["Offset of field: playdate_lua::getBitmap"][::core::mem::offset_of!(playdate_lua, getBitmap) - 120usize];
	["Offset of field: playdate_lua::getSprite"][::core::mem::offset_of!(playdate_lua, getSprite) - 128usize];
	["Offset of field: playdate_lua::pushNil"][::core::mem::offset_of!(playdate_lua, pushNil) - 136usize];
	["Offset of field: playdate_lua::pushBool"][::core::mem::offset_of!(playdate_lua, pushBool) - 144usize];
	["Offset of field: playdate_lua::pushInt"][::core::mem::offset_of!(playdate_lua, pushInt) - 152usize];
	["Offset of field: playdate_lua::pushFloat"][::core::mem::offset_of!(playdate_lua, pushFloat) - 160usize];
	["Offset of field: playdate_lua::pushString"][::core::mem::offset_of!(playdate_lua, pushString) - 168usize];
	["Offset of field: playdate_lua::pushBytes"][::core::mem::offset_of!(playdate_lua, pushBytes) - 176usize];
	["Offset of field: playdate_lua::pushBitmap"][::core::mem::offset_of!(playdate_lua, pushBitmap) - 184usize];
	["Offset of field: playdate_lua::pushSprite"][::core::mem::offset_of!(playdate_lua, pushSprite) - 192usize];
	["Offset of field: playdate_lua::pushObject"][::core::mem::offset_of!(playdate_lua, pushObject) - 200usize];
	["Offset of field: playdate_lua::retainObject"]
		[::core::mem::offset_of!(playdate_lua, retainObject) - 208usize];
	["Offset of field: playdate_lua::releaseObject"]
		[::core::mem::offset_of!(playdate_lua, releaseObject) - 216usize];
	["Offset of field: playdate_lua::setUserValue"]
		[::core::mem::offset_of!(playdate_lua, setUserValue) - 224usize];
	["Offset of field: playdate_lua::getUserValue"]
		[::core::mem::offset_of!(playdate_lua, getUserValue) - 232usize];
	["Offset of field: playdate_lua::callFunction_deprecated"]
		[::core::mem::offset_of!(playdate_lua, callFunction_deprecated) - 240usize];
	["Offset of field: playdate_lua::callFunction"]
		[::core::mem::offset_of!(playdate_lua, callFunction) - 248usize];
};
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of json_value__bindgen_ty_1"][::core::mem::size_of::<json_value__bindgen_ty_1>() - 8usize];
	["Alignment of json_value__bindgen_ty_1"][::core::mem::align_of::<json_value__bindgen_ty_1>() - 8usize];
	["Offset of field: json_value__bindgen_ty_1::intval"]
		[::core::mem::offset_of!(json_value__bindgen_ty_1, intval) - 0usize];
	["Offset of field: json_value__bindgen_ty_1::floatval"]
		[::core::mem::offset_of!(json_value__bindgen_ty_1, floatval) - 0usize];
	["Offset of field: json_value__bindgen_ty_1::stringval"]
		[::core::mem::offset_of!(json_value__bindgen_ty_1, stringval) - 0usize];
	["Offset of field: json_value__bindgen_ty_1::arrayval"]
		[::core::mem::offset_of!(json_value__bindgen_ty_1, arrayval) - 0usize];
	["Offset of field: json_value__bindgen_ty_1::tableval"]
		[::core::mem::offset_of!(json_value__bindgen_ty_1, tableval) - 0usize];
};
impl Default for json_value__bindgen_ty_1 {
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
	["Size of json_value"][::core::mem::size_of::<json_value>() - 16usize];
	["Alignment of json_value"][::core::mem::align_of::<json_value>() - 8usize];
	["Offset of field: json_value::type_"][::core::mem::offset_of!(json_value, type_) - 0usize];
	["Offset of field: json_value::data"][::core::mem::offset_of!(json_value, data) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of json_decoder"][::core::mem::size_of::<json_decoder>() - 80usize];
	["Alignment of json_decoder"][::core::mem::align_of::<json_decoder>() - 8usize];
	["Offset of field: json_decoder::decodeError"][::core::mem::offset_of!(json_decoder, decodeError) - 0usize];
	["Offset of field: json_decoder::willDecodeSublist"]
		[::core::mem::offset_of!(json_decoder, willDecodeSublist) - 8usize];
	["Offset of field: json_decoder::shouldDecodeTableValueForKey"]
		[::core::mem::offset_of!(json_decoder, shouldDecodeTableValueForKey) - 16usize];
	["Offset of field: json_decoder::didDecodeTableValue"]
		[::core::mem::offset_of!(json_decoder, didDecodeTableValue) - 24usize];
	["Offset of field: json_decoder::shouldDecodeArrayValueAtIndex"]
		[::core::mem::offset_of!(json_decoder, shouldDecodeArrayValueAtIndex) - 32usize];
	["Offset of field: json_decoder::didDecodeArrayValue"]
		[::core::mem::offset_of!(json_decoder, didDecodeArrayValue) - 40usize];
	["Offset of field: json_decoder::didDecodeSublist"]
		[::core::mem::offset_of!(json_decoder, didDecodeSublist) - 48usize];
	["Offset of field: json_decoder::userdata"][::core::mem::offset_of!(json_decoder, userdata) - 56usize];
	["Offset of field: json_decoder::returnString"][::core::mem::offset_of!(json_decoder, returnString) - 64usize];
	["Offset of field: json_decoder::path"][::core::mem::offset_of!(json_decoder, path) - 72usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of json_reader"][::core::mem::size_of::<json_reader>() - 16usize];
	["Alignment of json_reader"][::core::mem::align_of::<json_reader>() - 8usize];
	["Offset of field: json_reader::read"][::core::mem::offset_of!(json_reader, read) - 0usize];
	["Offset of field: json_reader::userdata"][::core::mem::offset_of!(json_reader, userdata) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of json_encoder"][::core::mem::size_of::<json_encoder>() - 120usize];
	["Alignment of json_encoder"][::core::mem::align_of::<json_encoder>() - 8usize];
	["Offset of field: json_encoder::writeStringFunc"]
		[::core::mem::offset_of!(json_encoder, writeStringFunc) - 0usize];
	["Offset of field: json_encoder::userdata"][::core::mem::offset_of!(json_encoder, userdata) - 8usize];
	["Offset of field: json_encoder::startArray"][::core::mem::offset_of!(json_encoder, startArray) - 24usize];
	["Offset of field: json_encoder::addArrayMember"]
		[::core::mem::offset_of!(json_encoder, addArrayMember) - 32usize];
	["Offset of field: json_encoder::endArray"][::core::mem::offset_of!(json_encoder, endArray) - 40usize];
	["Offset of field: json_encoder::startTable"][::core::mem::offset_of!(json_encoder, startTable) - 48usize];
	["Offset of field: json_encoder::addTableMember"]
		[::core::mem::offset_of!(json_encoder, addTableMember) - 56usize];
	["Offset of field: json_encoder::endTable"][::core::mem::offset_of!(json_encoder, endTable) - 64usize];
	["Offset of field: json_encoder::writeNull"][::core::mem::offset_of!(json_encoder, writeNull) - 72usize];
	["Offset of field: json_encoder::writeFalse"][::core::mem::offset_of!(json_encoder, writeFalse) - 80usize];
	["Offset of field: json_encoder::writeTrue"][::core::mem::offset_of!(json_encoder, writeTrue) - 88usize];
	["Offset of field: json_encoder::writeInt"][::core::mem::offset_of!(json_encoder, writeInt) - 96usize];
	["Offset of field: json_encoder::writeDouble"][::core::mem::offset_of!(json_encoder, writeDouble) - 104usize];
	["Offset of field: json_encoder::writeString"][::core::mem::offset_of!(json_encoder, writeString) - 112usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_json"][::core::mem::size_of::<playdate_json>() - 24usize];
	["Alignment of playdate_json"][::core::mem::align_of::<playdate_json>() - 8usize];
	["Offset of field: playdate_json::initEncoder"][::core::mem::offset_of!(playdate_json, initEncoder) - 0usize];
	["Offset of field: playdate_json::decode"][::core::mem::offset_of!(playdate_json, decode) - 8usize];
	["Offset of field: playdate_json::decodeString"]
		[::core::mem::offset_of!(playdate_json, decodeString) - 16usize];
};
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PDRect"][::core::mem::size_of::<PDRect>() - 16usize];
	["Alignment of PDRect"][::core::mem::align_of::<PDRect>() - 4usize];
	["Offset of field: PDRect::x"][::core::mem::offset_of!(PDRect, x) - 0usize];
	["Offset of field: PDRect::y"][::core::mem::offset_of!(PDRect, y) - 4usize];
	["Offset of field: PDRect::width"][::core::mem::offset_of!(PDRect, width) - 8usize];
	["Offset of field: PDRect::height"][::core::mem::offset_of!(PDRect, height) - 12usize];
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of SpriteCollisionInfo"][::core::mem::size_of::<SpriteCollisionInfo>() - 88usize];
	["Alignment of SpriteCollisionInfo"][::core::mem::align_of::<SpriteCollisionInfo>() - 8usize];
	["Offset of field: SpriteCollisionInfo::sprite"]
		[::core::mem::offset_of!(SpriteCollisionInfo, sprite) - 0usize];
	["Offset of field: SpriteCollisionInfo::other"][::core::mem::offset_of!(SpriteCollisionInfo, other) - 8usize];
	["Offset of field: SpriteCollisionInfo::responseType"]
		[::core::mem::offset_of!(SpriteCollisionInfo, responseType) - 16usize];
	["Offset of field: SpriteCollisionInfo::overlaps"]
		[::core::mem::offset_of!(SpriteCollisionInfo, overlaps) - 20usize];
	["Offset of field: SpriteCollisionInfo::ti"][::core::mem::offset_of!(SpriteCollisionInfo, ti) - 24usize];
	["Offset of field: SpriteCollisionInfo::move_"][::core::mem::offset_of!(SpriteCollisionInfo, move_) - 28usize];
	["Offset of field: SpriteCollisionInfo::normal"]
		[::core::mem::offset_of!(SpriteCollisionInfo, normal) - 36usize];
	["Offset of field: SpriteCollisionInfo::touch"][::core::mem::offset_of!(SpriteCollisionInfo, touch) - 44usize];
	["Offset of field: SpriteCollisionInfo::spriteRect"]
		[::core::mem::offset_of!(SpriteCollisionInfo, spriteRect) - 52usize];
	["Offset of field: SpriteCollisionInfo::otherRect"]
		[::core::mem::offset_of!(SpriteCollisionInfo, otherRect) - 68usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of SpriteQueryInfo"][::core::mem::size_of::<SpriteQueryInfo>() - 32usize];
	["Alignment of SpriteQueryInfo"][::core::mem::align_of::<SpriteQueryInfo>() - 8usize];
	["Offset of field: SpriteQueryInfo::sprite"][::core::mem::offset_of!(SpriteQueryInfo, sprite) - 0usize];
	["Offset of field: SpriteQueryInfo::ti1"][::core::mem::offset_of!(SpriteQueryInfo, ti1) - 8usize];
	["Offset of field: SpriteQueryInfo::ti2"][::core::mem::offset_of!(SpriteQueryInfo, ti2) - 12usize];
	["Offset of field: SpriteQueryInfo::entryPoint"]
		[::core::mem::offset_of!(SpriteQueryInfo, entryPoint) - 16usize];
	["Offset of field: SpriteQueryInfo::exitPoint"][::core::mem::offset_of!(SpriteQueryInfo, exitPoint) - 24usize];
};
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
	#[doc = "`void playdate->sprite->setTilemap(LCDSprite* sprite, LCDTileMap* tilemap);`\n\nSets the given *sprite*'s image to the given tilemap\\_."]
	pub setTilemap: ::core::option::Option<unsafe extern "C" fn(s: *mut LCDSprite, tilemap: *mut LCDTileMap)>,
	#[doc = "`LCDTileMap* playdate->sprite->getTilemap(LCDSprite *sprite);`\n\nReturns the LCDTileMap currently assigned to the given *sprite*."]
	pub getTilemap: ::core::option::Option<unsafe extern "C" fn(s: *mut LCDSprite) -> *mut LCDTileMap>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sprite"][::core::mem::size_of::<playdate_sprite>() - 520usize];
	["Alignment of playdate_sprite"][::core::mem::align_of::<playdate_sprite>() - 8usize];
	["Offset of field: playdate_sprite::setAlwaysRedraw"]
		[::core::mem::offset_of!(playdate_sprite, setAlwaysRedraw) - 0usize];
	["Offset of field: playdate_sprite::addDirtyRect"]
		[::core::mem::offset_of!(playdate_sprite, addDirtyRect) - 8usize];
	["Offset of field: playdate_sprite::drawSprites"]
		[::core::mem::offset_of!(playdate_sprite, drawSprites) - 16usize];
	["Offset of field: playdate_sprite::updateAndDrawSprites"]
		[::core::mem::offset_of!(playdate_sprite, updateAndDrawSprites) - 24usize];
	["Offset of field: playdate_sprite::newSprite"][::core::mem::offset_of!(playdate_sprite, newSprite) - 32usize];
	["Offset of field: playdate_sprite::freeSprite"]
		[::core::mem::offset_of!(playdate_sprite, freeSprite) - 40usize];
	["Offset of field: playdate_sprite::copy"][::core::mem::offset_of!(playdate_sprite, copy) - 48usize];
	["Offset of field: playdate_sprite::addSprite"][::core::mem::offset_of!(playdate_sprite, addSprite) - 56usize];
	["Offset of field: playdate_sprite::removeSprite"]
		[::core::mem::offset_of!(playdate_sprite, removeSprite) - 64usize];
	["Offset of field: playdate_sprite::removeSprites"]
		[::core::mem::offset_of!(playdate_sprite, removeSprites) - 72usize];
	["Offset of field: playdate_sprite::removeAllSprites"]
		[::core::mem::offset_of!(playdate_sprite, removeAllSprites) - 80usize];
	["Offset of field: playdate_sprite::getSpriteCount"]
		[::core::mem::offset_of!(playdate_sprite, getSpriteCount) - 88usize];
	["Offset of field: playdate_sprite::setBounds"][::core::mem::offset_of!(playdate_sprite, setBounds) - 96usize];
	["Offset of field: playdate_sprite::getBounds"]
		[::core::mem::offset_of!(playdate_sprite, getBounds) - 104usize];
	["Offset of field: playdate_sprite::moveTo"][::core::mem::offset_of!(playdate_sprite, moveTo) - 112usize];
	["Offset of field: playdate_sprite::moveBy"][::core::mem::offset_of!(playdate_sprite, moveBy) - 120usize];
	["Offset of field: playdate_sprite::setImage"][::core::mem::offset_of!(playdate_sprite, setImage) - 128usize];
	["Offset of field: playdate_sprite::getImage"][::core::mem::offset_of!(playdate_sprite, getImage) - 136usize];
	["Offset of field: playdate_sprite::setSize"][::core::mem::offset_of!(playdate_sprite, setSize) - 144usize];
	["Offset of field: playdate_sprite::setZIndex"]
		[::core::mem::offset_of!(playdate_sprite, setZIndex) - 152usize];
	["Offset of field: playdate_sprite::getZIndex"]
		[::core::mem::offset_of!(playdate_sprite, getZIndex) - 160usize];
	["Offset of field: playdate_sprite::setDrawMode"]
		[::core::mem::offset_of!(playdate_sprite, setDrawMode) - 168usize];
	["Offset of field: playdate_sprite::setImageFlip"]
		[::core::mem::offset_of!(playdate_sprite, setImageFlip) - 176usize];
	["Offset of field: playdate_sprite::getImageFlip"]
		[::core::mem::offset_of!(playdate_sprite, getImageFlip) - 184usize];
	["Offset of field: playdate_sprite::setStencil"]
		[::core::mem::offset_of!(playdate_sprite, setStencil) - 192usize];
	["Offset of field: playdate_sprite::setClipRect"]
		[::core::mem::offset_of!(playdate_sprite, setClipRect) - 200usize];
	["Offset of field: playdate_sprite::clearClipRect"]
		[::core::mem::offset_of!(playdate_sprite, clearClipRect) - 208usize];
	["Offset of field: playdate_sprite::setClipRectsInRange"]
		[::core::mem::offset_of!(playdate_sprite, setClipRectsInRange) - 216usize];
	["Offset of field: playdate_sprite::clearClipRectsInRange"]
		[::core::mem::offset_of!(playdate_sprite, clearClipRectsInRange) - 224usize];
	["Offset of field: playdate_sprite::setUpdatesEnabled"]
		[::core::mem::offset_of!(playdate_sprite, setUpdatesEnabled) - 232usize];
	["Offset of field: playdate_sprite::updatesEnabled"]
		[::core::mem::offset_of!(playdate_sprite, updatesEnabled) - 240usize];
	["Offset of field: playdate_sprite::setCollisionsEnabled"]
		[::core::mem::offset_of!(playdate_sprite, setCollisionsEnabled) - 248usize];
	["Offset of field: playdate_sprite::collisionsEnabled"]
		[::core::mem::offset_of!(playdate_sprite, collisionsEnabled) - 256usize];
	["Offset of field: playdate_sprite::setVisible"]
		[::core::mem::offset_of!(playdate_sprite, setVisible) - 264usize];
	["Offset of field: playdate_sprite::isVisible"]
		[::core::mem::offset_of!(playdate_sprite, isVisible) - 272usize];
	["Offset of field: playdate_sprite::setOpaque"]
		[::core::mem::offset_of!(playdate_sprite, setOpaque) - 280usize];
	["Offset of field: playdate_sprite::markDirty"]
		[::core::mem::offset_of!(playdate_sprite, markDirty) - 288usize];
	["Offset of field: playdate_sprite::setTag"][::core::mem::offset_of!(playdate_sprite, setTag) - 296usize];
	["Offset of field: playdate_sprite::getTag"][::core::mem::offset_of!(playdate_sprite, getTag) - 304usize];
	["Offset of field: playdate_sprite::setIgnoresDrawOffset"]
		[::core::mem::offset_of!(playdate_sprite, setIgnoresDrawOffset) - 312usize];
	["Offset of field: playdate_sprite::setUpdateFunction"]
		[::core::mem::offset_of!(playdate_sprite, setUpdateFunction) - 320usize];
	["Offset of field: playdate_sprite::setDrawFunction"]
		[::core::mem::offset_of!(playdate_sprite, setDrawFunction) - 328usize];
	["Offset of field: playdate_sprite::getPosition"]
		[::core::mem::offset_of!(playdate_sprite, getPosition) - 336usize];
	["Offset of field: playdate_sprite::resetCollisionWorld"]
		[::core::mem::offset_of!(playdate_sprite, resetCollisionWorld) - 344usize];
	["Offset of field: playdate_sprite::setCollideRect"]
		[::core::mem::offset_of!(playdate_sprite, setCollideRect) - 352usize];
	["Offset of field: playdate_sprite::getCollideRect"]
		[::core::mem::offset_of!(playdate_sprite, getCollideRect) - 360usize];
	["Offset of field: playdate_sprite::clearCollideRect"]
		[::core::mem::offset_of!(playdate_sprite, clearCollideRect) - 368usize];
	["Offset of field: playdate_sprite::setCollisionResponseFunction"]
		[::core::mem::offset_of!(playdate_sprite, setCollisionResponseFunction) - 376usize];
	["Offset of field: playdate_sprite::checkCollisions"]
		[::core::mem::offset_of!(playdate_sprite, checkCollisions) - 384usize];
	["Offset of field: playdate_sprite::moveWithCollisions"]
		[::core::mem::offset_of!(playdate_sprite, moveWithCollisions) - 392usize];
	["Offset of field: playdate_sprite::querySpritesAtPoint"]
		[::core::mem::offset_of!(playdate_sprite, querySpritesAtPoint) - 400usize];
	["Offset of field: playdate_sprite::querySpritesInRect"]
		[::core::mem::offset_of!(playdate_sprite, querySpritesInRect) - 408usize];
	["Offset of field: playdate_sprite::querySpritesAlongLine"]
		[::core::mem::offset_of!(playdate_sprite, querySpritesAlongLine) - 416usize];
	["Offset of field: playdate_sprite::querySpriteInfoAlongLine"]
		[::core::mem::offset_of!(playdate_sprite, querySpriteInfoAlongLine) - 424usize];
	["Offset of field: playdate_sprite::overlappingSprites"]
		[::core::mem::offset_of!(playdate_sprite, overlappingSprites) - 432usize];
	["Offset of field: playdate_sprite::allOverlappingSprites"]
		[::core::mem::offset_of!(playdate_sprite, allOverlappingSprites) - 440usize];
	["Offset of field: playdate_sprite::setStencilPattern"]
		[::core::mem::offset_of!(playdate_sprite, setStencilPattern) - 448usize];
	["Offset of field: playdate_sprite::clearStencil"]
		[::core::mem::offset_of!(playdate_sprite, clearStencil) - 456usize];
	["Offset of field: playdate_sprite::setUserdata"]
		[::core::mem::offset_of!(playdate_sprite, setUserdata) - 464usize];
	["Offset of field: playdate_sprite::getUserdata"]
		[::core::mem::offset_of!(playdate_sprite, getUserdata) - 472usize];
	["Offset of field: playdate_sprite::setStencilImage"]
		[::core::mem::offset_of!(playdate_sprite, setStencilImage) - 480usize];
	["Offset of field: playdate_sprite::setCenter"]
		[::core::mem::offset_of!(playdate_sprite, setCenter) - 488usize];
	["Offset of field: playdate_sprite::getCenter"]
		[::core::mem::offset_of!(playdate_sprite, getCenter) - 496usize];
	["Offset of field: playdate_sprite::setTilemap"]
		[::core::mem::offset_of!(playdate_sprite, setTilemap) - 504usize];
	["Offset of field: playdate_sprite::getTilemap"]
		[::core::mem::offset_of!(playdate_sprite, getTilemap) - 512usize];
};
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_source"][::core::mem::size_of::<playdate_sound_source>() - 32usize];
	["Alignment of playdate_sound_source"][::core::mem::align_of::<playdate_sound_source>() - 8usize];
	["Offset of field: playdate_sound_source::setVolume"]
		[::core::mem::offset_of!(playdate_sound_source, setVolume) - 0usize];
	["Offset of field: playdate_sound_source::getVolume"]
		[::core::mem::offset_of!(playdate_sound_source, getVolume) - 8usize];
	["Offset of field: playdate_sound_source::isPlaying"]
		[::core::mem::offset_of!(playdate_sound_source, isPlaying) - 16usize];
	["Offset of field: playdate_sound_source::setFinishCallback"]
		[::core::mem::offset_of!(playdate_sound_source, setFinishCallback) - 24usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_fileplayer { # [doc = "`FilePlayer* playdate->sound->fileplayer->newPlayer(void);`\n\nAllocates a new FilePlayer."] pub newPlayer : :: core :: option :: Option < unsafe extern "C" fn () -> * mut FilePlayer > , # [doc = "`void playdate->sound->fileplayer->freePlayer(FilePlayer* player);`\n\nFrees the given *player*."] pub freePlayer : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) > , # [doc = "`int playdate->sound->fileplayer->loadIntoPlayer(FilePlayer* player, const char* path);`\n\nPrepares *player* to stream the file at *path*. Returns 1 if the file exists, otherwise 0."] pub loadIntoPlayer : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , path : * const core :: ffi :: c_char) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->fileplayer->setBufferLength(FilePlayer* player, float bufferLen);`\n\nSets the buffer length of *player* to *bufferLen* seconds;"] pub setBufferLength : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , bufferLen : core :: ffi :: c_float) > , # [doc = "`int playdate->sound->fileplayer->play(FilePlayer* player, int repeat);`\n\nStarts playing the file *player*. If *repeat* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with [playdate-\\>sound-\\>fileplayer-\\>stop()](#f-sound.fileplayer.stop). Returns 1 on success, 0 if buffer allocation failed."] pub play : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , repeat : core :: ffi :: c_int) -> core :: ffi :: c_int > , # [doc = "`int playdate->sound->fileplayer->isPlaying(FilePlayer* player);`\n\nReturns one if *player* is playing, zero if not."] pub isPlaying : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->fileplayer->pause(FilePlayer* player);`\n\nPauses the file *player*."] pub pause : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) > , # [doc = "`void playdate->sound->fileplayer->stop(FilePlayer* player);`\n\nStops playing the file."] pub stop : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) > , # [doc = "`void playdate->sound->fileplayer->setVolume(FilePlayer* player, float left, float right);`\n\nSets the playback volume for left and right channels of *player*."] pub setVolume : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->fileplayer->getVolume(FilePlayer* player, float* outleft, float* outright);`\n\nGets the left and right channel playback volume for *player*."] pub getVolume : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , left : * mut core :: ffi :: c_float , right : * mut core :: ffi :: c_float) > , # [doc = "`float playdate->sound->fileplayer->getLength(FilePlayer* player);`\n\nReturns the length, in seconds, of the file loaded into *player*."] pub getLength : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float > , # [doc = "`void playdate->sound->fileplayer->setOffset(FilePlayer* player, float offset);`\n\nSets the current *offset* in seconds."] pub setOffset : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , offset : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->fileplayer->setRate(FilePlayer* player, float rate)`\n\nSets the playback *rate* for the *player*. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate \\< 0)."] pub setRate : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , rate : core :: ffi :: c_float) > , # [doc = "`void playdate->sound->fileplayer->setLoopRange(FilePlayer* player, float start, float end);`\n\nSets the *start* and *end* of the loop region for playback, in seconds. If *end* is omitted, the end of the file is used."] pub setLoopRange : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , start : core :: ffi :: c_float , end : core :: ffi :: c_float) > , # [doc = "`int playdate->sound->fileplayer->didUnderrun(FilePlayer* player);`\n\nReturns one if *player* has underrun, zero if not."] pub didUnderrun : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_int > , # [doc = "`void playdate->sound->fileplayer->setFinishCallback(FilePlayer* player, sndCallbackProc callback, void* userdata);`\n\nSets a function to be called when playback has completed. This is an alias for [playdate→sound→source→setFinishCallback()](#f-sound.source.setFinishCallback).\n\nsndCallbackProc\n\n```cpp\ntypedef void sndCallbackProc(SoundSource* c, void* userdata);\n```"] pub setFinishCallback : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , callback : sndCallbackProc , userdata : * mut core :: ffi :: c_void) > , pub setLoopCallback : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , callback : sndCallbackProc , userdata : * mut core :: ffi :: c_void) > , # [doc = "`float playdate->sound->fileplayer->getOffset(FilePlayer* player);`\n\nReturns the current offset in seconds for *player*."] pub getOffset : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float > , # [doc = "`float playdate->sound->fileplayer->getRate(FilePlayer* player)`\n\nReturns the playback rate for *player*."] pub getRate : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer) -> core :: ffi :: c_float > , # [doc = "`void playdate->sound->fileplayer->setStopOnUnderrun(FilePlayer* player, int flag)`\n\nIf *flag* evaluates to true, the *player* will restart playback (after an audible stutter) as soon as data is available."] pub setStopOnUnderrun : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , flag : core :: ffi :: c_int) > , # [doc = "`void playdate->sound->fileplayer->fadeVolume(FilePlayer* player, float left, float right, int32_t len, sndCallbackProc finishCallback, void* userdata);`\n\nChanges the volume of the fileplayer to *left* and *right* over a length of *len* sample frames, then calls the provided callback (if set)."] pub fadeVolume : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , left : core :: ffi :: c_float , right : core :: ffi :: c_float , len : i32 , finishCallback : sndCallbackProc , userdata : * mut core :: ffi :: c_void) > , pub setMP3StreamSource : :: core :: option :: Option < unsafe extern "C" fn (player : * mut FilePlayer , dataSource : :: core :: option :: Option < unsafe extern "C" fn (data : * mut u8 , bytes : core :: ffi :: c_int , userdata : * mut core :: ffi :: c_void) -> core :: ffi :: c_int > , userdata : * mut core :: ffi :: c_void , bufferLen : core :: ffi :: c_float) > , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_fileplayer"][::core::mem::size_of::<playdate_sound_fileplayer>() - 176usize];
	["Alignment of playdate_sound_fileplayer"][::core::mem::align_of::<playdate_sound_fileplayer>() - 8usize];
	["Offset of field: playdate_sound_fileplayer::newPlayer"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, newPlayer) - 0usize];
	["Offset of field: playdate_sound_fileplayer::freePlayer"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, freePlayer) - 8usize];
	["Offset of field: playdate_sound_fileplayer::loadIntoPlayer"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, loadIntoPlayer) - 16usize];
	["Offset of field: playdate_sound_fileplayer::setBufferLength"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setBufferLength) - 24usize];
	["Offset of field: playdate_sound_fileplayer::play"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, play) - 32usize];
	["Offset of field: playdate_sound_fileplayer::isPlaying"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, isPlaying) - 40usize];
	["Offset of field: playdate_sound_fileplayer::pause"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, pause) - 48usize];
	["Offset of field: playdate_sound_fileplayer::stop"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, stop) - 56usize];
	["Offset of field: playdate_sound_fileplayer::setVolume"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setVolume) - 64usize];
	["Offset of field: playdate_sound_fileplayer::getVolume"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, getVolume) - 72usize];
	["Offset of field: playdate_sound_fileplayer::getLength"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, getLength) - 80usize];
	["Offset of field: playdate_sound_fileplayer::setOffset"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setOffset) - 88usize];
	["Offset of field: playdate_sound_fileplayer::setRate"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setRate) - 96usize];
	["Offset of field: playdate_sound_fileplayer::setLoopRange"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setLoopRange) - 104usize];
	["Offset of field: playdate_sound_fileplayer::didUnderrun"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, didUnderrun) - 112usize];
	["Offset of field: playdate_sound_fileplayer::setFinishCallback"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setFinishCallback) - 120usize];
	["Offset of field: playdate_sound_fileplayer::setLoopCallback"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setLoopCallback) - 128usize];
	["Offset of field: playdate_sound_fileplayer::getOffset"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, getOffset) - 136usize];
	["Offset of field: playdate_sound_fileplayer::getRate"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, getRate) - 144usize];
	["Offset of field: playdate_sound_fileplayer::setStopOnUnderrun"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setStopOnUnderrun) - 152usize];
	["Offset of field: playdate_sound_fileplayer::fadeVolume"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, fadeVolume) - 160usize];
	["Offset of field: playdate_sound_fileplayer::setMP3StreamSource"]
		[::core::mem::offset_of!(playdate_sound_fileplayer, setMP3StreamSource) - 168usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_sample"][::core::mem::size_of::<playdate_sound_sample>() - 64usize];
	["Alignment of playdate_sound_sample"][::core::mem::align_of::<playdate_sound_sample>() - 8usize];
	["Offset of field: playdate_sound_sample::newSampleBuffer"]
		[::core::mem::offset_of!(playdate_sound_sample, newSampleBuffer) - 0usize];
	["Offset of field: playdate_sound_sample::loadIntoSample"]
		[::core::mem::offset_of!(playdate_sound_sample, loadIntoSample) - 8usize];
	["Offset of field: playdate_sound_sample::load"]
		[::core::mem::offset_of!(playdate_sound_sample, load) - 16usize];
	["Offset of field: playdate_sound_sample::newSampleFromData"]
		[::core::mem::offset_of!(playdate_sound_sample, newSampleFromData) - 24usize];
	["Offset of field: playdate_sound_sample::getData"]
		[::core::mem::offset_of!(playdate_sound_sample, getData) - 32usize];
	["Offset of field: playdate_sound_sample::freeSample"]
		[::core::mem::offset_of!(playdate_sound_sample, freeSample) - 40usize];
	["Offset of field: playdate_sound_sample::getLength"]
		[::core::mem::offset_of!(playdate_sound_sample, getLength) - 48usize];
	["Offset of field: playdate_sound_sample::decompress"]
		[::core::mem::offset_of!(playdate_sound_sample, decompress) - 56usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_sampleplayer"][::core::mem::size_of::<playdate_sound_sampleplayer>() - 136usize];
	["Alignment of playdate_sound_sampleplayer"][::core::mem::align_of::<playdate_sound_sampleplayer>() - 8usize];
	["Offset of field: playdate_sound_sampleplayer::newPlayer"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, newPlayer) - 0usize];
	["Offset of field: playdate_sound_sampleplayer::freePlayer"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, freePlayer) - 8usize];
	["Offset of field: playdate_sound_sampleplayer::setSample"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setSample) - 16usize];
	["Offset of field: playdate_sound_sampleplayer::play"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, play) - 24usize];
	["Offset of field: playdate_sound_sampleplayer::isPlaying"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, isPlaying) - 32usize];
	["Offset of field: playdate_sound_sampleplayer::stop"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, stop) - 40usize];
	["Offset of field: playdate_sound_sampleplayer::setVolume"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setVolume) - 48usize];
	["Offset of field: playdate_sound_sampleplayer::getVolume"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, getVolume) - 56usize];
	["Offset of field: playdate_sound_sampleplayer::getLength"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, getLength) - 64usize];
	["Offset of field: playdate_sound_sampleplayer::setOffset"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setOffset) - 72usize];
	["Offset of field: playdate_sound_sampleplayer::setRate"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setRate) - 80usize];
	["Offset of field: playdate_sound_sampleplayer::setPlayRange"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setPlayRange) - 88usize];
	["Offset of field: playdate_sound_sampleplayer::setFinishCallback"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setFinishCallback) - 96usize];
	["Offset of field: playdate_sound_sampleplayer::setLoopCallback"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setLoopCallback) - 104usize];
	["Offset of field: playdate_sound_sampleplayer::getOffset"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, getOffset) - 112usize];
	["Offset of field: playdate_sound_sampleplayer::getRate"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, getRate) - 120usize];
	["Offset of field: playdate_sound_sampleplayer::setPaused"]
		[::core::mem::offset_of!(playdate_sound_sampleplayer, setPaused) - 128usize];
};
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
                                                                         stopped: core::ffi::c_int,
                                                                         offset: core::ffi::c_int)>;
pub type signalDeallocFunc = ::core::option::Option<unsafe extern "C" fn(userdata: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_sound_signal {
	#[doc = "`PDSynthSignal* playdate->sound->signal->newSignal(signalStepFunc step, signalNoteOnFunc noteOn, signalNoteOffFunc noteOff, signalDeallocFunc dealloc, void* userdata)`\n\nSignalCallbacks\n\n```cpp\ntypedef float (*signalStepFunc)(void* userdata, int* iosamples, float* ifval);\ntypedef void (*signalNoteOnFunc)(void* userdata, MIDINote note, float vel, float len); // len = -1 for indefinite\ntypedef void (*signalNoteOffFunc)(void* userdata, int stopped, int offset); // stopped = 0 on note release, = 1 when note actually stops playing; offset is # of frames into the current cycle\ntypedef void (*signalDeallocFunc)(void* userdata);\n```\n\nProvides a custom implementation for the signal. *signalStepFunc step* is the only required function, returning the value at the end of the current frame. When called, the *ioframes* pointer contains the number of samples until the end of the frame. If the signal needs to provide a value in the middle of the frame (e.g. an LFO that needs to be sample-accurate) it should return the \"interframe\" value in *ifval* and set *iosamples* to the sample offset of the value. The functions are called on the audio render thread, so they should return as quickly as possible."]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_signal"][::core::mem::size_of::<playdate_sound_signal>() - 48usize];
	["Alignment of playdate_sound_signal"][::core::mem::align_of::<playdate_sound_signal>() - 8usize];
	["Offset of field: playdate_sound_signal::newSignal"]
		[::core::mem::offset_of!(playdate_sound_signal, newSignal) - 0usize];
	["Offset of field: playdate_sound_signal::freeSignal"]
		[::core::mem::offset_of!(playdate_sound_signal, freeSignal) - 8usize];
	["Offset of field: playdate_sound_signal::getValue"]
		[::core::mem::offset_of!(playdate_sound_signal, getValue) - 16usize];
	["Offset of field: playdate_sound_signal::setValueScale"]
		[::core::mem::offset_of!(playdate_sound_signal, setValueScale) - 24usize];
	["Offset of field: playdate_sound_signal::setValueOffset"]
		[::core::mem::offset_of!(playdate_sound_signal, setValueOffset) - 32usize];
	["Offset of field: playdate_sound_signal::newSignalForValue"]
		[::core::mem::offset_of!(playdate_sound_signal, newSignalForValue) - 40usize];
};
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_lfo"][::core::mem::size_of::<playdate_sound_lfo>() - 112usize];
	["Alignment of playdate_sound_lfo"][::core::mem::align_of::<playdate_sound_lfo>() - 8usize];
	["Offset of field: playdate_sound_lfo::newLFO"][::core::mem::offset_of!(playdate_sound_lfo, newLFO) - 0usize];
	["Offset of field: playdate_sound_lfo::freeLFO"]
		[::core::mem::offset_of!(playdate_sound_lfo, freeLFO) - 8usize];
	["Offset of field: playdate_sound_lfo::setType"]
		[::core::mem::offset_of!(playdate_sound_lfo, setType) - 16usize];
	["Offset of field: playdate_sound_lfo::setRate"]
		[::core::mem::offset_of!(playdate_sound_lfo, setRate) - 24usize];
	["Offset of field: playdate_sound_lfo::setPhase"]
		[::core::mem::offset_of!(playdate_sound_lfo, setPhase) - 32usize];
	["Offset of field: playdate_sound_lfo::setCenter"]
		[::core::mem::offset_of!(playdate_sound_lfo, setCenter) - 40usize];
	["Offset of field: playdate_sound_lfo::setDepth"]
		[::core::mem::offset_of!(playdate_sound_lfo, setDepth) - 48usize];
	["Offset of field: playdate_sound_lfo::setArpeggiation"]
		[::core::mem::offset_of!(playdate_sound_lfo, setArpeggiation) - 56usize];
	["Offset of field: playdate_sound_lfo::setFunction"]
		[::core::mem::offset_of!(playdate_sound_lfo, setFunction) - 64usize];
	["Offset of field: playdate_sound_lfo::setDelay"]
		[::core::mem::offset_of!(playdate_sound_lfo, setDelay) - 72usize];
	["Offset of field: playdate_sound_lfo::setRetrigger"]
		[::core::mem::offset_of!(playdate_sound_lfo, setRetrigger) - 80usize];
	["Offset of field: playdate_sound_lfo::getValue"]
		[::core::mem::offset_of!(playdate_sound_lfo, getValue) - 88usize];
	["Offset of field: playdate_sound_lfo::setGlobal"]
		[::core::mem::offset_of!(playdate_sound_lfo, setGlobal) - 96usize];
	["Offset of field: playdate_sound_lfo::setStartPhase"]
		[::core::mem::offset_of!(playdate_sound_lfo, setStartPhase) - 104usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_envelope"][::core::mem::size_of::<playdate_sound_envelope>() - 96usize];
	["Alignment of playdate_sound_envelope"][::core::mem::align_of::<playdate_sound_envelope>() - 8usize];
	["Offset of field: playdate_sound_envelope::newEnvelope"]
		[::core::mem::offset_of!(playdate_sound_envelope, newEnvelope) - 0usize];
	["Offset of field: playdate_sound_envelope::freeEnvelope"]
		[::core::mem::offset_of!(playdate_sound_envelope, freeEnvelope) - 8usize];
	["Offset of field: playdate_sound_envelope::setAttack"]
		[::core::mem::offset_of!(playdate_sound_envelope, setAttack) - 16usize];
	["Offset of field: playdate_sound_envelope::setDecay"]
		[::core::mem::offset_of!(playdate_sound_envelope, setDecay) - 24usize];
	["Offset of field: playdate_sound_envelope::setSustain"]
		[::core::mem::offset_of!(playdate_sound_envelope, setSustain) - 32usize];
	["Offset of field: playdate_sound_envelope::setRelease"]
		[::core::mem::offset_of!(playdate_sound_envelope, setRelease) - 40usize];
	["Offset of field: playdate_sound_envelope::setLegato"]
		[::core::mem::offset_of!(playdate_sound_envelope, setLegato) - 48usize];
	["Offset of field: playdate_sound_envelope::setRetrigger"]
		[::core::mem::offset_of!(playdate_sound_envelope, setRetrigger) - 56usize];
	["Offset of field: playdate_sound_envelope::getValue"]
		[::core::mem::offset_of!(playdate_sound_envelope, getValue) - 64usize];
	["Offset of field: playdate_sound_envelope::setCurvature"]
		[::core::mem::offset_of!(playdate_sound_envelope, setCurvature) - 72usize];
	["Offset of field: playdate_sound_envelope::setVelocitySensitivity"]
		[::core::mem::offset_of!(playdate_sound_envelope, setVelocitySensitivity) - 80usize];
	["Offset of field: playdate_sound_envelope::setRateScaling"]
		[::core::mem::offset_of!(playdate_sound_envelope, setRateScaling) - 88usize];
};
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_synth"][::core::mem::size_of::<playdate_sound_synth>() - 240usize];
	["Alignment of playdate_sound_synth"][::core::mem::align_of::<playdate_sound_synth>() - 8usize];
	["Offset of field: playdate_sound_synth::newSynth"]
		[::core::mem::offset_of!(playdate_sound_synth, newSynth) - 0usize];
	["Offset of field: playdate_sound_synth::freeSynth"]
		[::core::mem::offset_of!(playdate_sound_synth, freeSynth) - 8usize];
	["Offset of field: playdate_sound_synth::setWaveform"]
		[::core::mem::offset_of!(playdate_sound_synth, setWaveform) - 16usize];
	["Offset of field: playdate_sound_synth::setGenerator_deprecated"]
		[::core::mem::offset_of!(playdate_sound_synth, setGenerator_deprecated) - 24usize];
	["Offset of field: playdate_sound_synth::setSample"]
		[::core::mem::offset_of!(playdate_sound_synth, setSample) - 32usize];
	["Offset of field: playdate_sound_synth::setAttackTime"]
		[::core::mem::offset_of!(playdate_sound_synth, setAttackTime) - 40usize];
	["Offset of field: playdate_sound_synth::setDecayTime"]
		[::core::mem::offset_of!(playdate_sound_synth, setDecayTime) - 48usize];
	["Offset of field: playdate_sound_synth::setSustainLevel"]
		[::core::mem::offset_of!(playdate_sound_synth, setSustainLevel) - 56usize];
	["Offset of field: playdate_sound_synth::setReleaseTime"]
		[::core::mem::offset_of!(playdate_sound_synth, setReleaseTime) - 64usize];
	["Offset of field: playdate_sound_synth::setTranspose"]
		[::core::mem::offset_of!(playdate_sound_synth, setTranspose) - 72usize];
	["Offset of field: playdate_sound_synth::setFrequencyModulator"]
		[::core::mem::offset_of!(playdate_sound_synth, setFrequencyModulator) - 80usize];
	["Offset of field: playdate_sound_synth::getFrequencyModulator"]
		[::core::mem::offset_of!(playdate_sound_synth, getFrequencyModulator) - 88usize];
	["Offset of field: playdate_sound_synth::setAmplitudeModulator"]
		[::core::mem::offset_of!(playdate_sound_synth, setAmplitudeModulator) - 96usize];
	["Offset of field: playdate_sound_synth::getAmplitudeModulator"]
		[::core::mem::offset_of!(playdate_sound_synth, getAmplitudeModulator) - 104usize];
	["Offset of field: playdate_sound_synth::getParameterCount"]
		[::core::mem::offset_of!(playdate_sound_synth, getParameterCount) - 112usize];
	["Offset of field: playdate_sound_synth::setParameter"]
		[::core::mem::offset_of!(playdate_sound_synth, setParameter) - 120usize];
	["Offset of field: playdate_sound_synth::setParameterModulator"]
		[::core::mem::offset_of!(playdate_sound_synth, setParameterModulator) - 128usize];
	["Offset of field: playdate_sound_synth::getParameterModulator"]
		[::core::mem::offset_of!(playdate_sound_synth, getParameterModulator) - 136usize];
	["Offset of field: playdate_sound_synth::playNote"]
		[::core::mem::offset_of!(playdate_sound_synth, playNote) - 144usize];
	["Offset of field: playdate_sound_synth::playMIDINote"]
		[::core::mem::offset_of!(playdate_sound_synth, playMIDINote) - 152usize];
	["Offset of field: playdate_sound_synth::noteOff"]
		[::core::mem::offset_of!(playdate_sound_synth, noteOff) - 160usize];
	["Offset of field: playdate_sound_synth::stop"]
		[::core::mem::offset_of!(playdate_sound_synth, stop) - 168usize];
	["Offset of field: playdate_sound_synth::setVolume"]
		[::core::mem::offset_of!(playdate_sound_synth, setVolume) - 176usize];
	["Offset of field: playdate_sound_synth::getVolume"]
		[::core::mem::offset_of!(playdate_sound_synth, getVolume) - 184usize];
	["Offset of field: playdate_sound_synth::isPlaying"]
		[::core::mem::offset_of!(playdate_sound_synth, isPlaying) - 192usize];
	["Offset of field: playdate_sound_synth::getEnvelope"]
		[::core::mem::offset_of!(playdate_sound_synth, getEnvelope) - 200usize];
	["Offset of field: playdate_sound_synth::setWavetable"]
		[::core::mem::offset_of!(playdate_sound_synth, setWavetable) - 208usize];
	["Offset of field: playdate_sound_synth::setGenerator"]
		[::core::mem::offset_of!(playdate_sound_synth, setGenerator) - 216usize];
	["Offset of field: playdate_sound_synth::copy"]
		[::core::mem::offset_of!(playdate_sound_synth, copy) - 224usize];
	["Offset of field: playdate_sound_synth::clearEnvelope"]
		[::core::mem::offset_of!(playdate_sound_synth, clearEnvelope) - 232usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_control_signal"][::core::mem::size_of::<playdate_control_signal>() - 48usize];
	["Alignment of playdate_control_signal"][::core::mem::align_of::<playdate_control_signal>() - 8usize];
	["Offset of field: playdate_control_signal::newSignal"]
		[::core::mem::offset_of!(playdate_control_signal, newSignal) - 0usize];
	["Offset of field: playdate_control_signal::freeSignal"]
		[::core::mem::offset_of!(playdate_control_signal, freeSignal) - 8usize];
	["Offset of field: playdate_control_signal::clearEvents"]
		[::core::mem::offset_of!(playdate_control_signal, clearEvents) - 16usize];
	["Offset of field: playdate_control_signal::addEvent"]
		[::core::mem::offset_of!(playdate_control_signal, addEvent) - 24usize];
	["Offset of field: playdate_control_signal::removeEvent"]
		[::core::mem::offset_of!(playdate_control_signal, removeEvent) - 32usize];
	["Offset of field: playdate_control_signal::getMIDIControllerNumber"]
		[::core::mem::offset_of!(playdate_control_signal, getMIDIControllerNumber) - 40usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_instrument"][::core::mem::size_of::<playdate_sound_instrument>() - 104usize];
	["Alignment of playdate_sound_instrument"][::core::mem::align_of::<playdate_sound_instrument>() - 8usize];
	["Offset of field: playdate_sound_instrument::newInstrument"]
		[::core::mem::offset_of!(playdate_sound_instrument, newInstrument) - 0usize];
	["Offset of field: playdate_sound_instrument::freeInstrument"]
		[::core::mem::offset_of!(playdate_sound_instrument, freeInstrument) - 8usize];
	["Offset of field: playdate_sound_instrument::addVoice"]
		[::core::mem::offset_of!(playdate_sound_instrument, addVoice) - 16usize];
	["Offset of field: playdate_sound_instrument::playNote"]
		[::core::mem::offset_of!(playdate_sound_instrument, playNote) - 24usize];
	["Offset of field: playdate_sound_instrument::playMIDINote"]
		[::core::mem::offset_of!(playdate_sound_instrument, playMIDINote) - 32usize];
	["Offset of field: playdate_sound_instrument::setPitchBend"]
		[::core::mem::offset_of!(playdate_sound_instrument, setPitchBend) - 40usize];
	["Offset of field: playdate_sound_instrument::setPitchBendRange"]
		[::core::mem::offset_of!(playdate_sound_instrument, setPitchBendRange) - 48usize];
	["Offset of field: playdate_sound_instrument::setTranspose"]
		[::core::mem::offset_of!(playdate_sound_instrument, setTranspose) - 56usize];
	["Offset of field: playdate_sound_instrument::noteOff"]
		[::core::mem::offset_of!(playdate_sound_instrument, noteOff) - 64usize];
	["Offset of field: playdate_sound_instrument::allNotesOff"]
		[::core::mem::offset_of!(playdate_sound_instrument, allNotesOff) - 72usize];
	["Offset of field: playdate_sound_instrument::setVolume"]
		[::core::mem::offset_of!(playdate_sound_instrument, setVolume) - 80usize];
	["Offset of field: playdate_sound_instrument::getVolume"]
		[::core::mem::offset_of!(playdate_sound_instrument, getVolume) - 88usize];
	["Offset of field: playdate_sound_instrument::activeVoiceCount"]
		[::core::mem::offset_of!(playdate_sound_instrument, activeVoiceCount) - 96usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_track"][::core::mem::size_of::<playdate_sound_track>() - 136usize];
	["Alignment of playdate_sound_track"][::core::mem::align_of::<playdate_sound_track>() - 8usize];
	["Offset of field: playdate_sound_track::newTrack"]
		[::core::mem::offset_of!(playdate_sound_track, newTrack) - 0usize];
	["Offset of field: playdate_sound_track::freeTrack"]
		[::core::mem::offset_of!(playdate_sound_track, freeTrack) - 8usize];
	["Offset of field: playdate_sound_track::setInstrument"]
		[::core::mem::offset_of!(playdate_sound_track, setInstrument) - 16usize];
	["Offset of field: playdate_sound_track::getInstrument"]
		[::core::mem::offset_of!(playdate_sound_track, getInstrument) - 24usize];
	["Offset of field: playdate_sound_track::addNoteEvent"]
		[::core::mem::offset_of!(playdate_sound_track, addNoteEvent) - 32usize];
	["Offset of field: playdate_sound_track::removeNoteEvent"]
		[::core::mem::offset_of!(playdate_sound_track, removeNoteEvent) - 40usize];
	["Offset of field: playdate_sound_track::clearNotes"]
		[::core::mem::offset_of!(playdate_sound_track, clearNotes) - 48usize];
	["Offset of field: playdate_sound_track::getControlSignalCount"]
		[::core::mem::offset_of!(playdate_sound_track, getControlSignalCount) - 56usize];
	["Offset of field: playdate_sound_track::getControlSignal"]
		[::core::mem::offset_of!(playdate_sound_track, getControlSignal) - 64usize];
	["Offset of field: playdate_sound_track::clearControlEvents"]
		[::core::mem::offset_of!(playdate_sound_track, clearControlEvents) - 72usize];
	["Offset of field: playdate_sound_track::getPolyphony"]
		[::core::mem::offset_of!(playdate_sound_track, getPolyphony) - 80usize];
	["Offset of field: playdate_sound_track::activeVoiceCount"]
		[::core::mem::offset_of!(playdate_sound_track, activeVoiceCount) - 88usize];
	["Offset of field: playdate_sound_track::setMuted"]
		[::core::mem::offset_of!(playdate_sound_track, setMuted) - 96usize];
	["Offset of field: playdate_sound_track::getLength"]
		[::core::mem::offset_of!(playdate_sound_track, getLength) - 104usize];
	["Offset of field: playdate_sound_track::getIndexForStep"]
		[::core::mem::offset_of!(playdate_sound_track, getIndexForStep) - 112usize];
	["Offset of field: playdate_sound_track::getNoteAtIndex"]
		[::core::mem::offset_of!(playdate_sound_track, getNoteAtIndex) - 120usize];
	["Offset of field: playdate_sound_track::getSignalForController"]
		[::core::mem::offset_of!(playdate_sound_track, getSignalForController) - 128usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_sequence"][::core::mem::size_of::<playdate_sound_sequence>() - 160usize];
	["Alignment of playdate_sound_sequence"][::core::mem::align_of::<playdate_sound_sequence>() - 8usize];
	["Offset of field: playdate_sound_sequence::newSequence"]
		[::core::mem::offset_of!(playdate_sound_sequence, newSequence) - 0usize];
	["Offset of field: playdate_sound_sequence::freeSequence"]
		[::core::mem::offset_of!(playdate_sound_sequence, freeSequence) - 8usize];
	["Offset of field: playdate_sound_sequence::loadMIDIFile"]
		[::core::mem::offset_of!(playdate_sound_sequence, loadMIDIFile) - 16usize];
	["Offset of field: playdate_sound_sequence::getTime"]
		[::core::mem::offset_of!(playdate_sound_sequence, getTime) - 24usize];
	["Offset of field: playdate_sound_sequence::setTime"]
		[::core::mem::offset_of!(playdate_sound_sequence, setTime) - 32usize];
	["Offset of field: playdate_sound_sequence::setLoops"]
		[::core::mem::offset_of!(playdate_sound_sequence, setLoops) - 40usize];
	["Offset of field: playdate_sound_sequence::getTempo_deprecated"]
		[::core::mem::offset_of!(playdate_sound_sequence, getTempo_deprecated) - 48usize];
	["Offset of field: playdate_sound_sequence::setTempo"]
		[::core::mem::offset_of!(playdate_sound_sequence, setTempo) - 56usize];
	["Offset of field: playdate_sound_sequence::getTrackCount"]
		[::core::mem::offset_of!(playdate_sound_sequence, getTrackCount) - 64usize];
	["Offset of field: playdate_sound_sequence::addTrack"]
		[::core::mem::offset_of!(playdate_sound_sequence, addTrack) - 72usize];
	["Offset of field: playdate_sound_sequence::getTrackAtIndex"]
		[::core::mem::offset_of!(playdate_sound_sequence, getTrackAtIndex) - 80usize];
	["Offset of field: playdate_sound_sequence::setTrackAtIndex"]
		[::core::mem::offset_of!(playdate_sound_sequence, setTrackAtIndex) - 88usize];
	["Offset of field: playdate_sound_sequence::allNotesOff"]
		[::core::mem::offset_of!(playdate_sound_sequence, allNotesOff) - 96usize];
	["Offset of field: playdate_sound_sequence::isPlaying"]
		[::core::mem::offset_of!(playdate_sound_sequence, isPlaying) - 104usize];
	["Offset of field: playdate_sound_sequence::getLength"]
		[::core::mem::offset_of!(playdate_sound_sequence, getLength) - 112usize];
	["Offset of field: playdate_sound_sequence::play"]
		[::core::mem::offset_of!(playdate_sound_sequence, play) - 120usize];
	["Offset of field: playdate_sound_sequence::stop"]
		[::core::mem::offset_of!(playdate_sound_sequence, stop) - 128usize];
	["Offset of field: playdate_sound_sequence::getCurrentStep"]
		[::core::mem::offset_of!(playdate_sound_sequence, getCurrentStep) - 136usize];
	["Offset of field: playdate_sound_sequence::setCurrentStep"]
		[::core::mem::offset_of!(playdate_sound_sequence, setCurrentStep) - 144usize];
	["Offset of field: playdate_sound_sequence::getTempo"]
		[::core::mem::offset_of!(playdate_sound_sequence, getTempo) - 152usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct TwoPoleFilter {
	_unused: [u8; 0],
}
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect_twopolefilter"]
		[::core::mem::size_of::<playdate_sound_effect_twopolefilter>() - 80usize];
	["Alignment of playdate_sound_effect_twopolefilter"]
		[::core::mem::align_of::<playdate_sound_effect_twopolefilter>() - 8usize];
	["Offset of field: playdate_sound_effect_twopolefilter::newFilter"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, newFilter) - 0usize];
	["Offset of field: playdate_sound_effect_twopolefilter::freeFilter"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, freeFilter) - 8usize];
	["Offset of field: playdate_sound_effect_twopolefilter::setType"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, setType) - 16usize];
	["Offset of field: playdate_sound_effect_twopolefilter::setFrequency"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, setFrequency) - 24usize];
	["Offset of field: playdate_sound_effect_twopolefilter::setFrequencyModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, setFrequencyModulator) - 32usize];
	["Offset of field: playdate_sound_effect_twopolefilter::getFrequencyModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, getFrequencyModulator) - 40usize];
	["Offset of field: playdate_sound_effect_twopolefilter::setGain"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, setGain) - 48usize];
	["Offset of field: playdate_sound_effect_twopolefilter::setResonance"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, setResonance) - 56usize];
	["Offset of field: playdate_sound_effect_twopolefilter::setResonanceModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, setResonanceModulator) - 64usize];
	["Offset of field: playdate_sound_effect_twopolefilter::getResonanceModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_twopolefilter, getResonanceModulator) - 72usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect_onepolefilter"]
		[::core::mem::size_of::<playdate_sound_effect_onepolefilter>() - 40usize];
	["Alignment of playdate_sound_effect_onepolefilter"]
		[::core::mem::align_of::<playdate_sound_effect_onepolefilter>() - 8usize];
	["Offset of field: playdate_sound_effect_onepolefilter::newFilter"]
		[::core::mem::offset_of!(playdate_sound_effect_onepolefilter, newFilter) - 0usize];
	["Offset of field: playdate_sound_effect_onepolefilter::freeFilter"]
		[::core::mem::offset_of!(playdate_sound_effect_onepolefilter, freeFilter) - 8usize];
	["Offset of field: playdate_sound_effect_onepolefilter::setParameter"]
		[::core::mem::offset_of!(playdate_sound_effect_onepolefilter, setParameter) - 16usize];
	["Offset of field: playdate_sound_effect_onepolefilter::setParameterModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_onepolefilter, setParameterModulator) - 24usize];
	["Offset of field: playdate_sound_effect_onepolefilter::getParameterModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_onepolefilter, getParameterModulator) - 32usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect_bitcrusher"]
		[::core::mem::size_of::<playdate_sound_effect_bitcrusher>() - 64usize];
	["Alignment of playdate_sound_effect_bitcrusher"]
		[::core::mem::align_of::<playdate_sound_effect_bitcrusher>() - 8usize];
	["Offset of field: playdate_sound_effect_bitcrusher::newBitCrusher"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, newBitCrusher) - 0usize];
	["Offset of field: playdate_sound_effect_bitcrusher::freeBitCrusher"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, freeBitCrusher) - 8usize];
	["Offset of field: playdate_sound_effect_bitcrusher::setAmount"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, setAmount) - 16usize];
	["Offset of field: playdate_sound_effect_bitcrusher::setAmountModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, setAmountModulator) - 24usize];
	["Offset of field: playdate_sound_effect_bitcrusher::getAmountModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, getAmountModulator) - 32usize];
	["Offset of field: playdate_sound_effect_bitcrusher::setUndersampling"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, setUndersampling) - 40usize];
	["Offset of field: playdate_sound_effect_bitcrusher::setUndersampleModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, setUndersampleModulator) - 48usize];
	["Offset of field: playdate_sound_effect_bitcrusher::getUndersampleModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_bitcrusher, getUndersampleModulator) - 56usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect_ringmodulator"]
		[::core::mem::size_of::<playdate_sound_effect_ringmodulator>() - 40usize];
	["Alignment of playdate_sound_effect_ringmodulator"]
		[::core::mem::align_of::<playdate_sound_effect_ringmodulator>() - 8usize];
	["Offset of field: playdate_sound_effect_ringmodulator::newRingmod"]
		[::core::mem::offset_of!(playdate_sound_effect_ringmodulator, newRingmod) - 0usize];
	["Offset of field: playdate_sound_effect_ringmodulator::freeRingmod"]
		[::core::mem::offset_of!(playdate_sound_effect_ringmodulator, freeRingmod) - 8usize];
	["Offset of field: playdate_sound_effect_ringmodulator::setFrequency"]
		[::core::mem::offset_of!(playdate_sound_effect_ringmodulator, setFrequency) - 16usize];
	["Offset of field: playdate_sound_effect_ringmodulator::setFrequencyModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_ringmodulator, setFrequencyModulator) - 24usize];
	["Offset of field: playdate_sound_effect_ringmodulator::getFrequencyModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_ringmodulator, getFrequencyModulator) - 32usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect_delayline"]
		[::core::mem::size_of::<playdate_sound_effect_delayline>() - 80usize];
	["Alignment of playdate_sound_effect_delayline"]
		[::core::mem::align_of::<playdate_sound_effect_delayline>() - 8usize];
	["Offset of field: playdate_sound_effect_delayline::newDelayLine"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, newDelayLine) - 0usize];
	["Offset of field: playdate_sound_effect_delayline::freeDelayLine"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, freeDelayLine) - 8usize];
	["Offset of field: playdate_sound_effect_delayline::setLength"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, setLength) - 16usize];
	["Offset of field: playdate_sound_effect_delayline::setFeedback"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, setFeedback) - 24usize];
	["Offset of field: playdate_sound_effect_delayline::addTap"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, addTap) - 32usize];
	["Offset of field: playdate_sound_effect_delayline::freeTap"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, freeTap) - 40usize];
	["Offset of field: playdate_sound_effect_delayline::setTapDelay"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, setTapDelay) - 48usize];
	["Offset of field: playdate_sound_effect_delayline::setTapDelayModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, setTapDelayModulator) - 56usize];
	["Offset of field: playdate_sound_effect_delayline::getTapDelayModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, getTapDelayModulator) - 64usize];
	["Offset of field: playdate_sound_effect_delayline::setTapChannelsFlipped"]
		[::core::mem::offset_of!(playdate_sound_effect_delayline, setTapChannelsFlipped) - 72usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect_overdrive"]
		[::core::mem::size_of::<playdate_sound_effect_overdrive>() - 72usize];
	["Alignment of playdate_sound_effect_overdrive"]
		[::core::mem::align_of::<playdate_sound_effect_overdrive>() - 8usize];
	["Offset of field: playdate_sound_effect_overdrive::newOverdrive"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, newOverdrive) - 0usize];
	["Offset of field: playdate_sound_effect_overdrive::freeOverdrive"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, freeOverdrive) - 8usize];
	["Offset of field: playdate_sound_effect_overdrive::setGain"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, setGain) - 16usize];
	["Offset of field: playdate_sound_effect_overdrive::setLimit"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, setLimit) - 24usize];
	["Offset of field: playdate_sound_effect_overdrive::setLimitModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, setLimitModulator) - 32usize];
	["Offset of field: playdate_sound_effect_overdrive::getLimitModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, getLimitModulator) - 40usize];
	["Offset of field: playdate_sound_effect_overdrive::setOffset"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, setOffset) - 48usize];
	["Offset of field: playdate_sound_effect_overdrive::setOffsetModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, setOffsetModulator) - 56usize];
	["Offset of field: playdate_sound_effect_overdrive::getOffsetModulator"]
		[::core::mem::offset_of!(playdate_sound_effect_overdrive, getOffsetModulator) - 64usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_effect"][::core::mem::size_of::<playdate_sound_effect>() - 104usize];
	["Alignment of playdate_sound_effect"][::core::mem::align_of::<playdate_sound_effect>() - 8usize];
	["Offset of field: playdate_sound_effect::newEffect"]
		[::core::mem::offset_of!(playdate_sound_effect, newEffect) - 0usize];
	["Offset of field: playdate_sound_effect::freeEffect"]
		[::core::mem::offset_of!(playdate_sound_effect, freeEffect) - 8usize];
	["Offset of field: playdate_sound_effect::setMix"]
		[::core::mem::offset_of!(playdate_sound_effect, setMix) - 16usize];
	["Offset of field: playdate_sound_effect::setMixModulator"]
		[::core::mem::offset_of!(playdate_sound_effect, setMixModulator) - 24usize];
	["Offset of field: playdate_sound_effect::getMixModulator"]
		[::core::mem::offset_of!(playdate_sound_effect, getMixModulator) - 32usize];
	["Offset of field: playdate_sound_effect::setUserdata"]
		[::core::mem::offset_of!(playdate_sound_effect, setUserdata) - 40usize];
	["Offset of field: playdate_sound_effect::getUserdata"]
		[::core::mem::offset_of!(playdate_sound_effect, getUserdata) - 48usize];
	["Offset of field: playdate_sound_effect::twopolefilter"]
		[::core::mem::offset_of!(playdate_sound_effect, twopolefilter) - 56usize];
	["Offset of field: playdate_sound_effect::onepolefilter"]
		[::core::mem::offset_of!(playdate_sound_effect, onepolefilter) - 64usize];
	["Offset of field: playdate_sound_effect::bitcrusher"]
		[::core::mem::offset_of!(playdate_sound_effect, bitcrusher) - 72usize];
	["Offset of field: playdate_sound_effect::ringmodulator"]
		[::core::mem::offset_of!(playdate_sound_effect, ringmodulator) - 80usize];
	["Offset of field: playdate_sound_effect::delayline"]
		[::core::mem::offset_of!(playdate_sound_effect, delayline) - 88usize];
	["Offset of field: playdate_sound_effect::overdrive"]
		[::core::mem::offset_of!(playdate_sound_effect, overdrive) - 96usize];
};
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
	#[doc = "`int playdate->sound->channel->addEffect(SoundChannel* channel, SoundEffect* effect)`\n\nAdds a [SoundEffect](#f-sound.effect) to the channel. Returns 1 if successful, 0 if the effect is already in use."]
	pub addEffect: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel,
	                                                           effect: *mut SoundEffect)
	                                                           -> core::ffi::c_int>,
	#[doc = "`int playdate->sound->channel->removeEffect(SoundChannel* channel, SoundEffect* effect)`\n\nRemoves a [SoundEffect](#f-sound.effect) from the channel. Returns 1 if the effect was in the channel and removed, otherwise 0."]
	pub removeEffect: ::core::option::Option<unsafe extern "C" fn(channel: *mut SoundChannel,
	                                                              effect: *mut SoundEffect)
	                                                              -> core::ffi::c_int>,
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound_channel"][::core::mem::size_of::<playdate_sound_channel>() - 128usize];
	["Alignment of playdate_sound_channel"][::core::mem::align_of::<playdate_sound_channel>() - 8usize];
	["Offset of field: playdate_sound_channel::newChannel"]
		[::core::mem::offset_of!(playdate_sound_channel, newChannel) - 0usize];
	["Offset of field: playdate_sound_channel::freeChannel"]
		[::core::mem::offset_of!(playdate_sound_channel, freeChannel) - 8usize];
	["Offset of field: playdate_sound_channel::addSource"]
		[::core::mem::offset_of!(playdate_sound_channel, addSource) - 16usize];
	["Offset of field: playdate_sound_channel::removeSource"]
		[::core::mem::offset_of!(playdate_sound_channel, removeSource) - 24usize];
	["Offset of field: playdate_sound_channel::addCallbackSource"]
		[::core::mem::offset_of!(playdate_sound_channel, addCallbackSource) - 32usize];
	["Offset of field: playdate_sound_channel::addEffect"]
		[::core::mem::offset_of!(playdate_sound_channel, addEffect) - 40usize];
	["Offset of field: playdate_sound_channel::removeEffect"]
		[::core::mem::offset_of!(playdate_sound_channel, removeEffect) - 48usize];
	["Offset of field: playdate_sound_channel::setVolume"]
		[::core::mem::offset_of!(playdate_sound_channel, setVolume) - 56usize];
	["Offset of field: playdate_sound_channel::getVolume"]
		[::core::mem::offset_of!(playdate_sound_channel, getVolume) - 64usize];
	["Offset of field: playdate_sound_channel::setVolumeModulator"]
		[::core::mem::offset_of!(playdate_sound_channel, setVolumeModulator) - 72usize];
	["Offset of field: playdate_sound_channel::getVolumeModulator"]
		[::core::mem::offset_of!(playdate_sound_channel, getVolumeModulator) - 80usize];
	["Offset of field: playdate_sound_channel::setPan"]
		[::core::mem::offset_of!(playdate_sound_channel, setPan) - 88usize];
	["Offset of field: playdate_sound_channel::setPanModulator"]
		[::core::mem::offset_of!(playdate_sound_channel, setPanModulator) - 96usize];
	["Offset of field: playdate_sound_channel::getPanModulator"]
		[::core::mem::offset_of!(playdate_sound_channel, getPanModulator) - 104usize];
	["Offset of field: playdate_sound_channel::getDryLevelSignal"]
		[::core::mem::offset_of!(playdate_sound_channel, getDryLevelSignal) - 112usize];
	["Offset of field: playdate_sound_channel::getWetLevelSignal"]
		[::core::mem::offset_of!(playdate_sound_channel, getWetLevelSignal) - 120usize];
};
pub type RecordCallback = ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void,
                                                                      buffer: *mut i16,
                                                                      length: core::ffi::c_int)
                                                                      -> core::ffi::c_int>;
#[repr(u32)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_sound"][::core::mem::size_of::<playdate_sound>() - 192usize];
	["Alignment of playdate_sound"][::core::mem::align_of::<playdate_sound>() - 8usize];
	["Offset of field: playdate_sound::channel"][::core::mem::offset_of!(playdate_sound, channel) - 0usize];
	["Offset of field: playdate_sound::fileplayer"][::core::mem::offset_of!(playdate_sound, fileplayer) - 8usize];
	["Offset of field: playdate_sound::sample"][::core::mem::offset_of!(playdate_sound, sample) - 16usize];
	["Offset of field: playdate_sound::sampleplayer"]
		[::core::mem::offset_of!(playdate_sound, sampleplayer) - 24usize];
	["Offset of field: playdate_sound::synth"][::core::mem::offset_of!(playdate_sound, synth) - 32usize];
	["Offset of field: playdate_sound::sequence"][::core::mem::offset_of!(playdate_sound, sequence) - 40usize];
	["Offset of field: playdate_sound::effect"][::core::mem::offset_of!(playdate_sound, effect) - 48usize];
	["Offset of field: playdate_sound::lfo"][::core::mem::offset_of!(playdate_sound, lfo) - 56usize];
	["Offset of field: playdate_sound::envelope"][::core::mem::offset_of!(playdate_sound, envelope) - 64usize];
	["Offset of field: playdate_sound::source"][::core::mem::offset_of!(playdate_sound, source) - 72usize];
	["Offset of field: playdate_sound::controlsignal"]
		[::core::mem::offset_of!(playdate_sound, controlsignal) - 80usize];
	["Offset of field: playdate_sound::track"][::core::mem::offset_of!(playdate_sound, track) - 88usize];
	["Offset of field: playdate_sound::instrument"][::core::mem::offset_of!(playdate_sound, instrument) - 96usize];
	["Offset of field: playdate_sound::getCurrentTime"]
		[::core::mem::offset_of!(playdate_sound, getCurrentTime) - 104usize];
	["Offset of field: playdate_sound::addSource"][::core::mem::offset_of!(playdate_sound, addSource) - 112usize];
	["Offset of field: playdate_sound::getDefaultChannel"]
		[::core::mem::offset_of!(playdate_sound, getDefaultChannel) - 120usize];
	["Offset of field: playdate_sound::addChannel"]
		[::core::mem::offset_of!(playdate_sound, addChannel) - 128usize];
	["Offset of field: playdate_sound::removeChannel"]
		[::core::mem::offset_of!(playdate_sound, removeChannel) - 136usize];
	["Offset of field: playdate_sound::setMicCallback"]
		[::core::mem::offset_of!(playdate_sound, setMicCallback) - 144usize];
	["Offset of field: playdate_sound::getHeadphoneState"]
		[::core::mem::offset_of!(playdate_sound, getHeadphoneState) - 152usize];
	["Offset of field: playdate_sound::setOutputsActive"]
		[::core::mem::offset_of!(playdate_sound, setOutputsActive) - 160usize];
	["Offset of field: playdate_sound::removeSource"]
		[::core::mem::offset_of!(playdate_sound, removeSource) - 168usize];
	["Offset of field: playdate_sound::signal"][::core::mem::offset_of!(playdate_sound, signal) - 176usize];
	["Offset of field: playdate_sound::getError"][::core::mem::offset_of!(playdate_sound, getError) - 184usize];
};
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
	#[doc = "`float playdate->display->getRefreshRate()`\n\nReturns the current nominal display refresh rate. This is the frame rate the device is targeting, and does not account for lag due to (for example) code running too slow. To get the real time frame rate, use [playdate→display→getFPS()](#f-display.getFPS).\n\nEquivalent to [`playdate.display.getRefreshRate()`](./Inside%20Playdate.html#f-display.getRefreshRate) in the Lua API."]
	pub getRefreshRate: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_float>,
	#[doc = "`float playdate->display->getFPS()`\n\nReturns the *measured, actual* refresh rate in frames per second. This value may be different from the *specified* refresh rate (see [playdate→display→getRefreshRate()](#f-display.getRefreshRate)) by a little or a lot depending upon how much calculation is being done per frame.\n\nEquivalent to [`playdate.display.getFPS()`](./Inside%20Playdate.html#f-display.getFPS) in the Lua API."]
	pub getFPS: ::core::option::Option<unsafe extern "C" fn() -> core::ffi::c_float>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_display"][::core::mem::size_of::<playdate_display>() - 80usize];
	["Alignment of playdate_display"][::core::mem::align_of::<playdate_display>() - 8usize];
	["Offset of field: playdate_display::getWidth"][::core::mem::offset_of!(playdate_display, getWidth) - 0usize];
	["Offset of field: playdate_display::getHeight"]
		[::core::mem::offset_of!(playdate_display, getHeight) - 8usize];
	["Offset of field: playdate_display::setRefreshRate"]
		[::core::mem::offset_of!(playdate_display, setRefreshRate) - 16usize];
	["Offset of field: playdate_display::setInverted"]
		[::core::mem::offset_of!(playdate_display, setInverted) - 24usize];
	["Offset of field: playdate_display::setScale"][::core::mem::offset_of!(playdate_display, setScale) - 32usize];
	["Offset of field: playdate_display::setMosaic"]
		[::core::mem::offset_of!(playdate_display, setMosaic) - 40usize];
	["Offset of field: playdate_display::setFlipped"]
		[::core::mem::offset_of!(playdate_display, setFlipped) - 48usize];
	["Offset of field: playdate_display::setOffset"]
		[::core::mem::offset_of!(playdate_display, setOffset) - 56usize];
	["Offset of field: playdate_display::getRefreshRate"]
		[::core::mem::offset_of!(playdate_display, getRefreshRate) - 64usize];
	["Offset of field: playdate_display::getFPS"][::core::mem::offset_of!(playdate_display, getFPS) - 72usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct PDScore {
	pub rank: u32,
	pub value: u32,
	pub player: *mut core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PDScore"][::core::mem::size_of::<PDScore>() - 16usize];
	["Alignment of PDScore"][::core::mem::align_of::<PDScore>() - 8usize];
	["Offset of field: PDScore::rank"][::core::mem::offset_of!(PDScore, rank) - 0usize];
	["Offset of field: PDScore::value"][::core::mem::offset_of!(PDScore, value) - 4usize];
	["Offset of field: PDScore::player"][::core::mem::offset_of!(PDScore, player) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PDScoresList"][::core::mem::size_of::<PDScoresList>() - 32usize];
	["Alignment of PDScoresList"][::core::mem::align_of::<PDScoresList>() - 8usize];
	["Offset of field: PDScoresList::boardID"][::core::mem::offset_of!(PDScoresList, boardID) - 0usize];
	["Offset of field: PDScoresList::count"][::core::mem::offset_of!(PDScoresList, count) - 8usize];
	["Offset of field: PDScoresList::lastUpdated"][::core::mem::offset_of!(PDScoresList, lastUpdated) - 12usize];
	["Offset of field: PDScoresList::playerIncluded"]
		[::core::mem::offset_of!(PDScoresList, playerIncluded) - 16usize];
	["Offset of field: PDScoresList::limit"][::core::mem::offset_of!(PDScoresList, limit) - 20usize];
	["Offset of field: PDScoresList::scores"][::core::mem::offset_of!(PDScoresList, scores) - 24usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PDBoard"][::core::mem::size_of::<PDBoard>() - 16usize];
	["Alignment of PDBoard"][::core::mem::align_of::<PDBoard>() - 8usize];
	["Offset of field: PDBoard::boardID"][::core::mem::offset_of!(PDBoard, boardID) - 0usize];
	["Offset of field: PDBoard::name"][::core::mem::offset_of!(PDBoard, name) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PDBoardsList"][::core::mem::size_of::<PDBoardsList>() - 16usize];
	["Alignment of PDBoardsList"][::core::mem::align_of::<PDBoardsList>() - 8usize];
	["Offset of field: PDBoardsList::count"][::core::mem::offset_of!(PDBoardsList, count) - 0usize];
	["Offset of field: PDBoardsList::lastUpdated"][::core::mem::offset_of!(PDBoardsList, lastUpdated) - 4usize];
	["Offset of field: PDBoardsList::boards"][::core::mem::offset_of!(PDBoardsList, boards) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_scoreboards"][::core::mem::size_of::<playdate_scoreboards>() - 56usize];
	["Alignment of playdate_scoreboards"][::core::mem::align_of::<playdate_scoreboards>() - 8usize];
	["Offset of field: playdate_scoreboards::addScore"]
		[::core::mem::offset_of!(playdate_scoreboards, addScore) - 0usize];
	["Offset of field: playdate_scoreboards::getPersonalBest"]
		[::core::mem::offset_of!(playdate_scoreboards, getPersonalBest) - 8usize];
	["Offset of field: playdate_scoreboards::freeScore"]
		[::core::mem::offset_of!(playdate_scoreboards, freeScore) - 16usize];
	["Offset of field: playdate_scoreboards::getScoreboards"]
		[::core::mem::offset_of!(playdate_scoreboards, getScoreboards) - 24usize];
	["Offset of field: playdate_scoreboards::freeBoardsList"]
		[::core::mem::offset_of!(playdate_scoreboards, freeBoardsList) - 32usize];
	["Offset of field: playdate_scoreboards::getScores"]
		[::core::mem::offset_of!(playdate_scoreboards, getScores) - 40usize];
	["Offset of field: playdate_scoreboards::freeScoresList"]
		[::core::mem::offset_of!(playdate_scoreboards, freeScoresList) - 48usize];
};
#[repr(i32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum PDNetErr {
	NET_OK = 0,
	NET_NO_DEVICE = -1,
	NET_BUSY = -2,
	NET_WRITE_ERROR = -3,
	NET_WRITE_BUSY = -4,
	NET_WRITE_TIMEOUT = -5,
	NET_READ_ERROR = -6,
	NET_READ_BUSY = -7,
	NET_READ_TIMEOUT = -8,
	NET_READ_OVERFLOW = -9,
	NET_FRAME_ERROR = -10,
	NET_BAD_RESPONSE = -11,
	NET_ERROR_RESPONSE = -12,
	NET_RESET_TIMEOUT = -13,
	NET_BUFFER_TOO_SMALL = -14,
	NET_UNEXPECTED_RESPONSE = -15,
	NET_NOT_CONNECTED_TO_AP = -16,
	NET_NOT_IMPLEMENTED = -17,
	NET_CONNECTION_CLOSED = -18,
}
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum WifiStatus {
	#[doc = "!< Not connected to an AP"]
	kWifiNotConnected = 0,
	#[doc = "!< Device is connected to an AP"]
	kWifiConnected = 1,
	#[doc = "!< A connection has been attempted and no configured AP was available"]
	kWifiNotAvailable = 2,
}
pub type HTTPConnectionCallback = ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection)>;
pub type HTTPHeaderCallback = ::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection,
                                                                          key: *const core::ffi::c_char,
                                                                          value: *const core::ffi::c_char)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_http {
	#[doc = "`enum accessReply playdate->network->http->requestAccess(const char* server, int port, bool usessl, const char* purpose, AccessRequestCallback* requestCallback, void* userdata);`\n\n```cpp\ntypedef void AccessRequestCallback(bool allowed, void* userdata);\n```\n\nBefore connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling http→newConnection(). `server` can be a parent domain of the connections opened, or NULL to request access to any HTTP server. `purpose` is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, `requestCallback` is called with the given `userdata` argument. The return value is one of the following:\n\n```cpp\nenum accessReply\n{\n\tkAccessAsk,\n\tkAccessDeny,\n\tkAccessAllow\n};\n```"]
	pub requestAccess: ::core::option::Option<unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                                               port: core::ffi::c_int,
	                                                               usessl: bool,
	                                                               purpose: *const core::ffi::c_char,
	                                                               requestCallback: AccessRequestCallback,
	                                                               userdata: *mut core::ffi::c_void)
	                                                               -> accessReply>,
	#[doc = "`HTTPConnection* playdate->network->http->newConnection(const char* server, int port, bool usessl);`\n\nReturns an `HTTPConnection` object for connecting to the given server, or NULL if permission has been denied or not yet granted. If `port` is 0, the connection will use port 80 if `usessl` is false, otherwise 443. No connection is attempted until [get()](#f-network.http.get) or [post()](#f-network.http.post) are called."]
	pub newConnection: ::core::option::Option<unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                                               port: core::ffi::c_int,
	                                                               usessl: bool)
	                                                               -> *mut HTTPConnection>,
	#[doc = "`HTTPConnection* playdate->network->http->retain(HTTPConnection* connection);`\n\nAdds 1 to the connection’s retain count, so that it won’t be freed when it scopes out of another context. This is used primarily so we can pass a connection created in Lua into C and not have to worry about the Lua wrapper’s lifespan."]
	pub retain: ::core::option::Option<unsafe extern "C" fn(http: *mut HTTPConnection) -> *mut HTTPConnection>,
	#[doc = "`void playdate->network->http->release(HTTPConnection* connection);`\n\nReleases a previous retain on the connection."]
	pub release: ::core::option::Option<unsafe extern "C" fn(http: *mut HTTPConnection)>,
	#[doc = "`void playdate->network->http->setConnectTimeout(HTTPConnection* connection, int ms);`\n\nSets the length of time (in milliseconds) to wait for the connection to the server to be made."]
	pub setConnectTimeout:
		::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection, ms: core::ffi::c_int)>,
	#[doc = "`void playdate->network->http->setKeepAlive(HTTPConnection* connection, bool keepalive);`\n\nIf `keepalive` is true, this causes the HTTP request to include a *Connection: keep-alive* header."]
	pub setKeepAlive:
		::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection, keepalive: bool)>,
	#[doc = "`void playdate->network->http->setByteRange(HTTPConnection* connection, int start, int end);`\n\nAdds a `Range: bytes=<start>-<end>` header to the HTTP request."]
	pub setByteRange: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection,
	                                                              start: core::ffi::c_int,
	                                                              end: core::ffi::c_int)>,
	#[doc = "`void playdate->network->http->setUserdata(HTTPConnection* connection, void* userdata);`\n\nSets a custom userdata on the connection."]
	pub setUserdata: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection,
	                                                             userdata: *mut core::ffi::c_void)>,
	#[doc = "`void* playdate->network->http->getUserdata(HTTPConnection* connection);`\n\nReturns the userdata previously set on the connection."]
	pub getUserdata:
		::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection) -> *mut core::ffi::c_void>,
	#[doc = "`PDNetErr playdate->network->http->get(HTTPConnection* conn, const char* path, const char* headers, size_t headerlen);`\n\nOpens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a GET request with the given path and additional *headers* if specified."]
	pub get: ::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection,
	                                                     path: *const core::ffi::c_char,
	                                                     headers: *const core::ffi::c_char,
	                                                     headerlen: usize)
	                                                     -> PDNetErr>,
	#[doc = "`PDNetErr playdate->network->http->post(HTTPConnection* conn, const char* path, const char* headers, size_t headerlen, const char* body, size_t bodylen);`\n\nOpens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a POST request with the given path, additional *headers* if specified, and the provided *data*."]
	pub post: ::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection,
	                                                      path: *const core::ffi::c_char,
	                                                      headers: *const core::ffi::c_char,
	                                                      headerlen: usize,
	                                                      body: *const core::ffi::c_char,
	                                                      bodylen: usize)
	                                                      -> PDNetErr>,
	#[doc = "`PDNetErr playdate->network->http->getError(HTTPConnection* connection);`\n\nReturns a code for the last error on the connection, or NET\\_OK if none occurred."]
	pub getError: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection) -> PDNetErr>,
	#[doc = "`void playdate->network->http->getProgress(HTTPConnection* conn, int* read, int* total);`\n\nReturns the number of bytes already read from the connection and the total bytes the server plans to send, if known."]
	pub getProgress: ::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection,
	                                                             read: *mut core::ffi::c_int,
	                                                             total: *mut core::ffi::c_int)>,
	#[doc = "`int playdate->network->http->getResponseStatus(HTTPConnection* connection);`\n\nReturns the HTTP status response code, if the request response headers have been received and parsed."]
	pub getResponseStatus:
		::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection) -> core::ffi::c_int>,
	#[doc = "`size_t playdate->network->http->getBytesAvailable(HTTPConnection* connection);`\n\nReturns the number of bytes currently available for reading from the connection."]
	pub getBytesAvailable: ::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection) -> usize>,
	#[doc = "`void playdate->network->http->setReadTimeout(HTTPConnection* connection, int ms);`\n\nSets the length of time, in milliseconds, the [read()](#f-network.http.read) function will wait for incoming data before returning. The default value is 1000, or one second."]
	pub setReadTimeout:
		::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection, ms: core::ffi::c_int)>,
	#[doc = "`void playdate->network->http->setReadBufferSize(HTTPConnection* connection, int bytes);`\n\nSets the size of the connection’s read buffer. The default buffer size is 64 KB."]
	pub setReadBufferSize:
		::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection, bytes: core::ffi::c_int)>,
	#[doc = "`int playdate->network->http->read(HTTPConnection* conn, void* buf, unsigned int buflen);`\n\nOn success, returns up to `length` bytes (limited by the size of the read buffer) from the connection. If `length` is more than the number of bytes available the function will wait for more data up to the length of time set by [setReadTimeout()](#f-network.http.setReadTimeout) (default one second)."]
	pub read: ::core::option::Option<unsafe extern "C" fn(conn: *mut HTTPConnection,
	                                                      buf: *mut core::ffi::c_void,
	                                                      buflen: core::ffi::c_uint)
	                                                      -> core::ffi::c_int>,
	#[doc = "`void playdate->network->http->close(HTTPConnection* connection);`\n\nCloses the HTTP connection. The connection may be used again for another request."]
	pub close: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection)>,
	#[doc = "`void playdate->network->http->setHeaderReceivedCallback(HTTPConnection* connection, HTTPHeaderCallback* header);`\n\nSets a callback to be called when the HTTP parser reads a header line from the connection"]
	pub setHeaderReceivedCallback:
		::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection, headercb: HTTPHeaderCallback)>,
	#[doc = "`void playdate->network->http->setHeadersReadCallback(HTTPConnection* connection, HTTPConnectionCallback* header);`\n\nSets a function to be called after the connection has parsed the headers from the server response. At this point, [getResponseStatus()](#f-network.http.getResponseStatus) and [getProgress()](#f-network.http.getProgress) can be used to query the status and size of the response, and [get()](#f-network.http.get)/[post()](#f-network.http.post) can queue another request if `connection:setKeepAlive(true)` was set and the connection is still open."]
	pub setHeadersReadCallback: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection,
	                                                                        callback: HTTPConnectionCallback)>,
	#[doc = "`void playdate->network->http->setResponseCallback(HTTPConnection* connection, HTTPConnectionCallback* header);`\n\nSets a function to be called when data is available for reading."]
	pub setResponseCallback: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection,
	                                                                     callback: HTTPConnectionCallback)>,
	#[doc = "`void playdate->network->http->setRequestCompleteCallback(HTTPConnection* connection, HTTPConnectionCallback* header);`\n\nSets a function to be called when all data for the request has been received (if the response contained a Content-Length header and the size is known) or the request times out."]
	pub setRequestCompleteCallback: ::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection,
	                                                                            callback: HTTPConnectionCallback)>,
	#[doc = "`void playdate->network->http->setConnectionClosedCallback(HTTPConnection* connection, HTTPConnectionCallback* header);`\n\nSets a function to be called when the server has closed the connection."]
	pub setConnectionClosedCallback:
		::core::option::Option<unsafe extern "C" fn(connection: *mut HTTPConnection,
		                                            callback: HTTPConnectionCallback)>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_http"][::core::mem::size_of::<playdate_http>() - 192usize];
	["Alignment of playdate_http"][::core::mem::align_of::<playdate_http>() - 8usize];
	["Offset of field: playdate_http::requestAccess"]
		[::core::mem::offset_of!(playdate_http, requestAccess) - 0usize];
	["Offset of field: playdate_http::newConnection"]
		[::core::mem::offset_of!(playdate_http, newConnection) - 8usize];
	["Offset of field: playdate_http::retain"][::core::mem::offset_of!(playdate_http, retain) - 16usize];
	["Offset of field: playdate_http::release"][::core::mem::offset_of!(playdate_http, release) - 24usize];
	["Offset of field: playdate_http::setConnectTimeout"]
		[::core::mem::offset_of!(playdate_http, setConnectTimeout) - 32usize];
	["Offset of field: playdate_http::setKeepAlive"]
		[::core::mem::offset_of!(playdate_http, setKeepAlive) - 40usize];
	["Offset of field: playdate_http::setByteRange"]
		[::core::mem::offset_of!(playdate_http, setByteRange) - 48usize];
	["Offset of field: playdate_http::setUserdata"][::core::mem::offset_of!(playdate_http, setUserdata) - 56usize];
	["Offset of field: playdate_http::getUserdata"][::core::mem::offset_of!(playdate_http, getUserdata) - 64usize];
	["Offset of field: playdate_http::get"][::core::mem::offset_of!(playdate_http, get) - 72usize];
	["Offset of field: playdate_http::post"][::core::mem::offset_of!(playdate_http, post) - 80usize];
	["Offset of field: playdate_http::getError"][::core::mem::offset_of!(playdate_http, getError) - 88usize];
	["Offset of field: playdate_http::getProgress"][::core::mem::offset_of!(playdate_http, getProgress) - 96usize];
	["Offset of field: playdate_http::getResponseStatus"]
		[::core::mem::offset_of!(playdate_http, getResponseStatus) - 104usize];
	["Offset of field: playdate_http::getBytesAvailable"]
		[::core::mem::offset_of!(playdate_http, getBytesAvailable) - 112usize];
	["Offset of field: playdate_http::setReadTimeout"]
		[::core::mem::offset_of!(playdate_http, setReadTimeout) - 120usize];
	["Offset of field: playdate_http::setReadBufferSize"]
		[::core::mem::offset_of!(playdate_http, setReadBufferSize) - 128usize];
	["Offset of field: playdate_http::read"][::core::mem::offset_of!(playdate_http, read) - 136usize];
	["Offset of field: playdate_http::close"][::core::mem::offset_of!(playdate_http, close) - 144usize];
	["Offset of field: playdate_http::setHeaderReceivedCallback"]
		[::core::mem::offset_of!(playdate_http, setHeaderReceivedCallback) - 152usize];
	["Offset of field: playdate_http::setHeadersReadCallback"]
		[::core::mem::offset_of!(playdate_http, setHeadersReadCallback) - 160usize];
	["Offset of field: playdate_http::setResponseCallback"]
		[::core::mem::offset_of!(playdate_http, setResponseCallback) - 168usize];
	["Offset of field: playdate_http::setRequestCompleteCallback"]
		[::core::mem::offset_of!(playdate_http, setRequestCompleteCallback) - 176usize];
	["Offset of field: playdate_http::setConnectionClosedCallback"]
		[::core::mem::offset_of!(playdate_http, setConnectionClosedCallback) - 184usize];
};
pub type TCPConnectionCallback =
	::core::option::Option<unsafe extern "C" fn(connection: *mut TCPConnection, err: PDNetErr)>;
pub type TCPOpenCallback = ::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection,
                                                                       err: PDNetErr,
                                                                       ud: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_tcp {
	#[doc = "`void playdate->network->tcp->requestAccess(const char* server, int port, bool usessl, const char* purpose, AccessRequestCallback* requestCallback, void* userdata);`\n\nBefore connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling [newConnection()](#f-network.tcp.newConnection()). `server` can be a parent domain of the connections opened, or NULL to request access to any HTTP server. Similarly, if `port` is zero, this requests access to all ports on the target server(s). `purpose` is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, `requestCallback` is called with the given `userdata` argument. The return value is one of the following:\n\n```cpp\nenum accessReply\n{\n\tkAccessAsk,\n\tkAccessDeny,\n\tkAccessAllow\n};\n```"]
	pub requestAccess: ::core::option::Option<unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                                               port: core::ffi::c_int,
	                                                               usessl: bool,
	                                                               purpose: *const core::ffi::c_char,
	                                                               requestCallback: AccessRequestCallback,
	                                                               userdata: *mut core::ffi::c_void)
	                                                               -> accessReply>,
	#[doc = "`TCPConnection* playdate->network->tcp->newConnection(const char* server, int port, bool usessl);`\n\nReturns a `playdate.network.tcp` object for connecting to the given server, or NULL if permission has been denied or not yet granted. No connection is attempted until [open()](#f-network.tcp.open) is called."]
	pub newConnection: ::core::option::Option<unsafe extern "C" fn(server: *const core::ffi::c_char,
	                                                               port: core::ffi::c_int,
	                                                               usessl: bool)
	                                                               -> *mut TCPConnection>,
	#[doc = "`HTTPConnection* playdate->network->tcp->retain(TCPConnection* connection);`\n\nAdds 1 to the connection’s retain count, so that it won’t be freed when it scopes out of another context. This is used primarily so we can pass a connection created in Lua into C and not have to worry about the Lua wrapper’s lifespan."]
	pub retain: ::core::option::Option<unsafe extern "C" fn(http: *mut TCPConnection) -> *mut TCPConnection>,
	#[doc = "`void playdate->network->tcp->release(TCPConnection* connection);`\n\nReleases a previous retain on the connection."]
	pub release: ::core::option::Option<unsafe extern "C" fn(http: *mut TCPConnection)>,
	#[doc = "`PDNetErr playdate->network->tcp->getError(TCPConnection* connection);`\n\nReturns a code for the last error on the connection, or NET\\_OK if none occurred."]
	pub getError: ::core::option::Option<unsafe extern "C" fn(connection: *mut TCPConnection) -> PDNetErr>,
	#[doc = "`void playdate->network->tcp->setConnectTimeout(TCPConnection* connection, int ms);`\n\nSets the length of time (in milliseconds) to wait for the connection to the server to be made."]
	pub setConnectTimeout:
		::core::option::Option<unsafe extern "C" fn(connection: *mut TCPConnection, ms: core::ffi::c_int)>,
	#[doc = "`void playdate->network->tcp->setUserdata(TCPConnection* connection, void* userdata);`\n\nSets a custom userdata on the connection."]
	pub setUserdata: ::core::option::Option<unsafe extern "C" fn(connection: *mut TCPConnection,
	                                                             userdata: *mut core::ffi::c_void)>,
	#[doc = "`void* playdate->network->tcp->getUserdata(TCPConnection* connection);`\n\nReturns the userdata previously set on the connection."]
	pub getUserdata:
		::core::option::Option<unsafe extern "C" fn(connection: *mut TCPConnection) -> *mut core::ffi::c_void>,
	#[doc = "`PDNetErr playdate->network->tcp->open(TCPConnection* connection, TCPOpenCallback cb, void* ud);`\n\n```cpp\ntypedef void TCPOpenCallback(TCPConnection* conn, PDNetErr err, void* ud);\n```\n\nAttempts to open the connection to the server. Note that an error may be returned immediately, or in the open callback depending on where it occurs."]
	pub open: ::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection,
	                                                      cb: TCPOpenCallback,
	                                                      ud: *mut core::ffi::c_void)
	                                                      -> PDNetErr>,
	#[doc = "`PDNetErr playdate->network->tcp->close(TCPConnection* connection);`\n\nCloses the connection. The connection may be used again for another request."]
	pub close: ::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection) -> PDNetErr>,
	#[doc = "`void playdate->network->tcp->setConnectionClosedCallback(TCPConnection* connection, TCPConnectionCallback* header);`\n\n```cpp\ntypedef void TCPConnectionCallback(TCPConnection* connection, PDNetErr err);\n```\n\nSets a callback to be called when the connection is closed."]
	pub setConnectionClosedCallback:
		::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection, callback: TCPConnectionCallback)>,
	#[doc = "`void playdate->network->tcp->setReadTimeout(TCPConnection* connection, int ms);`\n\nSets the length of time, in milliseconds, [read()](#f-network.tcp.read) will wait for incoming data before returning. The default value is 1000, or one second."]
	pub setReadTimeout:
		::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection, ms: core::ffi::c_int)>,
	#[doc = "`void playdate->network->tcp->setReadBufferSize(TCPConnection* connection, int bytes);`\n\nSets the size of the connection’s read buffer. The default buffer size is 64 KB."]
	pub setReadBufferSize:
		::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection, bytes: core::ffi::c_int)>,
	#[doc = "`size_t playdate->network->tcp->getBytesAvailable(TCPConnection* connection);`\n\nReturns the number of bytes currently available for reading from the connection."]
	pub getBytesAvailable: ::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection) -> usize>,
	#[doc = "`int playdate->network->tcp->read(TCPConnection* conn, void *buffer, size_t length);`\n\nAttempts to read up to `length` bytes from the connection into `buffer`. If `length` is more than the number of bytes available on the connection the function will wait for more data, up to the length of time set by [setReadTimeout()](#f-network.tcp.setReadTimeout) (default one second). Returns the number of bytes actually read, or a (negative) PDNetErr value on error."]
	pub read: ::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection,
	                                                      buffer: *mut core::ffi::c_void,
	                                                      length: usize)
	                                                      -> core::ffi::c_int>,
	#[doc = "`size_t playdate->network->tcp->write(TCPConnection* conn, const void *buffer, size_t length);`\n\nAttempts to write up to `length` bytes to the connection. Returns the number of bytes actually written, which may be less than `length`, or a (negative) PDNetErr value on error."]
	pub write: ::core::option::Option<unsafe extern "C" fn(conn: *mut TCPConnection,
	                                                       buffer: *const core::ffi::c_void,
	                                                       length: usize)
	                                                       -> core::ffi::c_int>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_tcp"][::core::mem::size_of::<playdate_tcp>() - 128usize];
	["Alignment of playdate_tcp"][::core::mem::align_of::<playdate_tcp>() - 8usize];
	["Offset of field: playdate_tcp::requestAccess"]
		[::core::mem::offset_of!(playdate_tcp, requestAccess) - 0usize];
	["Offset of field: playdate_tcp::newConnection"]
		[::core::mem::offset_of!(playdate_tcp, newConnection) - 8usize];
	["Offset of field: playdate_tcp::retain"][::core::mem::offset_of!(playdate_tcp, retain) - 16usize];
	["Offset of field: playdate_tcp::release"][::core::mem::offset_of!(playdate_tcp, release) - 24usize];
	["Offset of field: playdate_tcp::getError"][::core::mem::offset_of!(playdate_tcp, getError) - 32usize];
	["Offset of field: playdate_tcp::setConnectTimeout"]
		[::core::mem::offset_of!(playdate_tcp, setConnectTimeout) - 40usize];
	["Offset of field: playdate_tcp::setUserdata"][::core::mem::offset_of!(playdate_tcp, setUserdata) - 48usize];
	["Offset of field: playdate_tcp::getUserdata"][::core::mem::offset_of!(playdate_tcp, getUserdata) - 56usize];
	["Offset of field: playdate_tcp::open"][::core::mem::offset_of!(playdate_tcp, open) - 64usize];
	["Offset of field: playdate_tcp::close"][::core::mem::offset_of!(playdate_tcp, close) - 72usize];
	["Offset of field: playdate_tcp::setConnectionClosedCallback"]
		[::core::mem::offset_of!(playdate_tcp, setConnectionClosedCallback) - 80usize];
	["Offset of field: playdate_tcp::setReadTimeout"]
		[::core::mem::offset_of!(playdate_tcp, setReadTimeout) - 88usize];
	["Offset of field: playdate_tcp::setReadBufferSize"]
		[::core::mem::offset_of!(playdate_tcp, setReadBufferSize) - 96usize];
	["Offset of field: playdate_tcp::getBytesAvailable"]
		[::core::mem::offset_of!(playdate_tcp, getBytesAvailable) - 104usize];
	["Offset of field: playdate_tcp::read"][::core::mem::offset_of!(playdate_tcp, read) - 112usize];
	["Offset of field: playdate_tcp::write"][::core::mem::offset_of!(playdate_tcp, write) - 120usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct playdate_network { pub http : * const playdate_http , pub tcp : * const playdate_tcp , # [doc = "`WifiStatus playdate->network->getStatus();`\n\nReturns a value from the following:"] pub getStatus : :: core :: option :: Option < unsafe extern "C" fn () -> WifiStatus > , # [doc = "`void playdate->network->setEnabled(bool flag, void (*callback)(PDNetErr err));`\n\nPlaydate will connect to the configured access point automatically as needed and turn off the wifi radio after a 30 second idle timeout. This function allows a game to start connecting to the access point sooner, since that can take upwards of 10 seconds, or turn off wifi as soon as it’s no longer needed instead of waiting 30 seconds. If `flag` is true, a callback function can be provided to check for an error connecting to the access point.\n\n```cpp\ntypedef enum {\n\tNET_OK = 0,\n\tNET_NO_DEVICE = -1,\n\tNET_BUSY = -2,\n\tNET_WRITE_ERROR = -3,\n\tNET_WRITE_BUSY = -4,\n\tNET_WRITE_TIMEOUT = -5,\n\tNET_READ_ERROR = -6,\n\tNET_READ_BUSY = -7,\n\tNET_READ_TIMEOUT = -8,\n\tNET_READ_OVERFLOW = -9,\n\tNET_FRAME_ERROR = -10,\n\tNET_BAD_RESPONSE = -11,\n\tNET_ERROR_RESPONSE = -12,\n\tNET_RESET_TIMEOUT = -13,\n\tNET_BUFFER_TOO_SMALL = -14,\n\tNET_UNEXPECTED_RESPONSE = -15,\n\tNET_NOT_CONNECTED_TO_AP = -16,\n\tNET_NOT_IMPLEMENTED = -17,\n\tNET_CONNECT_TIMEOUT = -18,\n\tNET_CONNECTION_CLOSED = -19,\n\tNET_PERMISSION_DENIED = -20,\n} PDNetErr;\n```"] pub setEnabled : :: core :: option :: Option < unsafe extern "C" fn (flag : bool , callback : :: core :: option :: Option < unsafe extern "C" fn (err : PDNetErr) >) > , pub reserved : [usize ; 3usize] , }
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of playdate_network"][::core::mem::size_of::<playdate_network>() - 56usize];
	["Alignment of playdate_network"][::core::mem::align_of::<playdate_network>() - 8usize];
	["Offset of field: playdate_network::http"][::core::mem::offset_of!(playdate_network, http) - 0usize];
	["Offset of field: playdate_network::tcp"][::core::mem::offset_of!(playdate_network, tcp) - 8usize];
	["Offset of field: playdate_network::getStatus"]
		[::core::mem::offset_of!(playdate_network, getStatus) - 16usize];
	["Offset of field: playdate_network::setEnabled"]
		[::core::mem::offset_of!(playdate_network, setEnabled) - 24usize];
	["Offset of field: playdate_network::reserved"][::core::mem::offset_of!(playdate_network, reserved) - 32usize];
};
impl Default for playdate_network {
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
	pub network: *const playdate_network,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of PlaydateAPI"][::core::mem::size_of::<PlaydateAPI>() - 80usize];
	["Alignment of PlaydateAPI"][::core::mem::align_of::<PlaydateAPI>() - 8usize];
	["Offset of field: PlaydateAPI::system"][::core::mem::offset_of!(PlaydateAPI, system) - 0usize];
	["Offset of field: PlaydateAPI::file"][::core::mem::offset_of!(PlaydateAPI, file) - 8usize];
	["Offset of field: PlaydateAPI::graphics"][::core::mem::offset_of!(PlaydateAPI, graphics) - 16usize];
	["Offset of field: PlaydateAPI::sprite"][::core::mem::offset_of!(PlaydateAPI, sprite) - 24usize];
	["Offset of field: PlaydateAPI::display"][::core::mem::offset_of!(PlaydateAPI, display) - 32usize];
	["Offset of field: PlaydateAPI::sound"][::core::mem::offset_of!(PlaydateAPI, sound) - 40usize];
	["Offset of field: PlaydateAPI::lua"][::core::mem::offset_of!(PlaydateAPI, lua) - 48usize];
	["Offset of field: PlaydateAPI::json"][::core::mem::offset_of!(PlaydateAPI, json) - 56usize];
	["Offset of field: PlaydateAPI::scoreboards"][::core::mem::offset_of!(PlaydateAPI, scoreboards) - 64usize];
	["Offset of field: PlaydateAPI::network"][::core::mem::offset_of!(PlaydateAPI, network) - 72usize];
};
impl Default for PlaydateAPI {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
#[repr(u32)]
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
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[must_use]
pub struct __va_list_tag {
	pub gp_offset: core::ffi::c_uint,
	pub fp_offset: core::ffi::c_uint,
	pub overflow_arg_area: *mut core::ffi::c_void,
	pub reg_save_area: *mut core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
	["Size of __va_list_tag"][::core::mem::size_of::<__va_list_tag>() - 24usize];
	["Alignment of __va_list_tag"][::core::mem::align_of::<__va_list_tag>() - 8usize];
	["Offset of field: __va_list_tag::gp_offset"][::core::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
	["Offset of field: __va_list_tag::fp_offset"][::core::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
	["Offset of field: __va_list_tag::overflow_arg_area"]
		[::core::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
	["Offset of field: __va_list_tag::reg_save_area"]
		[::core::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
};
impl Default for __va_list_tag {
	fn default() -> Self {
		let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
		unsafe {
			::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
			s.assume_init()
		}
	}
}
