use core::convert::Infallible;
use core::ffi::c_void;

use sys::error::OkOrNullFnErr;
use sys::ffi::FileOptions;
use sys::ffi::FileStat;

use crate::FileSystemRaw;


pub type FnClose = unsafe extern "C" fn(*mut c_void) -> i32;
pub type FnFlush = unsafe extern "C" fn(*mut c_void) -> i32;
pub type FnGeterr = unsafe extern "C" fn() -> *const i8;
pub type FnListfiles =
	unsafe extern "C" fn(*const i8, Option<unsafe extern "C" fn(*const i8, *mut c_void)>, *mut c_void, i32) -> i32;
pub type FnMkdir = unsafe extern "C" fn(*const i8) -> i32;
pub type FnOpen = unsafe extern "C" fn(*const i8, FileOptions) -> *mut c_void;
pub type FnRead = unsafe extern "C" fn(*mut c_void, *mut c_void, u32) -> i32;
pub type FnRename = unsafe extern "C" fn(*const i8, *const i8) -> i32;
pub type FnSeek = unsafe extern "C" fn(*mut c_void, i32, i32) -> i32;
pub type FnStat = unsafe extern "C" fn(*const i8, *mut FileStat) -> i32;
pub type FnTell = unsafe extern "C" fn(*mut c_void) -> i32;
pub type FnUnlink = unsafe extern "C" fn(*const i8, i32) -> i32;
pub type FnWrite = unsafe extern "C" fn(*mut c_void, *const c_void, u32) -> i32;


/// File-system API cached end-point.
/// Useful if you're making many operations on fs.
///
// TODO: describe benefits of this cached version.
pub struct Fs {
	close: FnClose,
	flush: FnFlush,
	geterr: FnGeterr,
	listfiles: FnListfiles,
	mkdir: FnMkdir,
	open: FnOpen,
	read: FnRead,
	rename: FnRename,
	seek: FnSeek,
	stat: FnStat,
	tell: FnTell,
	unlink: FnUnlink,
	write: FnWrite,
}


impl FileSystemRaw for Fs {
	#![doc(hidden)]
	type AccessError = Infallible;

	#[inline(always)]
	fn _close(&self) -> Result<FnClose, Self::AccessError> { Ok(self.close) }

	#[inline(always)]
	fn _flush(&self) -> Result<FnFlush, Self::AccessError> { Ok(self.flush) }

	#[inline(always)]
	fn _geterr(&self) -> Result<FnGeterr, Self::AccessError> { Ok(self.geterr) }

	#[inline(always)]
	fn _listfiles(&self) -> Result<FnListfiles, Self::AccessError> { Ok(self.listfiles) }

	#[inline(always)]
	fn _mkdir(&self) -> Result<FnMkdir, Self::AccessError> { Ok(self.mkdir) }

	#[inline(always)]
	fn _open(&self) -> Result<FnOpen, Self::AccessError> { Ok(self.open) }

	#[inline(always)]
	fn _read(&self) -> Result<FnRead, Self::AccessError> { Ok(self.read) }

	#[inline(always)]
	fn _rename(&self) -> Result<FnRename, Self::AccessError> { Ok(self.rename) }

	#[inline(always)]
	fn _seek(&self) -> Result<FnSeek, Self::AccessError> { Ok(self.seek) }

	#[inline(always)]
	fn _stat(&self) -> Result<FnStat, Self::AccessError> { Ok(self.stat) }

	#[inline(always)]
	fn _tell(&self) -> Result<FnTell, Self::AccessError> { Ok(self.tell) }

	#[inline(always)]
	fn _unlink(&self) -> Result<FnUnlink, Self::AccessError> { Ok(self.unlink) }

	#[inline(always)]
	fn _write(&self) -> Result<FnWrite, Self::AccessError> { Ok(self.write) }
}


impl Fs {
	/// Caches all fs-api endpoints in a new `Fs` instance.
	///
	/// Can fail with error if there somewhere is a null pointer.
	pub fn new() -> Result<Fs, sys::error::NullPtrError> {
		let api = sys::api_ok!(file)?;

		let close = api.close.ok_or_null()?;
		let flush = api.flush.ok_or_null()?;
		let geterr = api.geterr.ok_or_null()?;
		let listfiles = api.listfiles.ok_or_null()?;
		let mkdir = api.mkdir.ok_or_null()?;
		let open = api.open.ok_or_null()?;
		let read = api.read.ok_or_null()?;
		let rename = api.rename.ok_or_null()?;
		let seek = api.seek.ok_or_null()?;
		let stat = api.stat.ok_or_null()?;
		let tell = api.tell.ok_or_null()?;
		let unlink = api.unlink.ok_or_null()?;
		let write = api.write.ok_or_null()?;

		Ok(Self { close,
		          flush,
		          geterr,
		          listfiles,
		          mkdir,
		          open,
		          read,
		          rename,
		          seek,
		          stat,
		          tell,
		          unlink,
		          write })
	}
}
