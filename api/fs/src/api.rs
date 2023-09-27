use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_char;
use core::ffi::c_void;
use core::ptr::NonNull;

use sys::ffi::playdate_file;
use sys::ffi::FileOptions;
use sys::ffi::FileStat;


/// Default file system api end-point, ZST.
///
/// All calls approximately costs ~3 derefs.
#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;
impl Api for Default {}


/// Cached file system api end-point.
///
/// Useful if you're making many operations on fs.
///
/// Stores one reference, so size on stack is eq `usize`.
///
/// All calls approximately costs ~1 deref.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Cache(&'static playdate_file);

impl core::default::Default for Cache {
	fn default() -> Self { Self(sys::api!(file)) }
}

impl From<*const playdate_file> for Cache {
	#[inline(always)]
	fn from(ptr: *const playdate_file) -> Self { Self(unsafe { ptr.as_ref() }.expect("fs")) }
}

impl From<&'static playdate_file> for Cache {
	#[inline(always)]
	fn from(r: &'static playdate_file) -> Self { Self(r) }
}

impl From<NonNull<playdate_file>> for Cache {
	#[inline(always)]
	fn from(ptr: NonNull<playdate_file>) -> Self { Self(unsafe { ptr.as_ref() }) }
}

impl From<&'_ NonNull<playdate_file>> for Cache {
	#[inline(always)]
	fn from(ptr: &NonNull<playdate_file>) -> Self { Self(unsafe { ptr.as_ref() }) }
}


impl Api for Cache {
	#[inline(always)]
	fn close(&self) -> defs::FnClose { self.0.close.expect("close") }

	#[inline(always)]
	fn flush(&self) -> defs::FnFlush { self.0.flush.expect("flush") }

	#[inline(always)]
	fn geterr(&self) -> defs::FnGeterr { self.0.geterr.expect("geterr") }

	#[inline(always)]
	fn listfiles(&self) -> defs::FnListfiles { self.0.listfiles.expect("listfiles") }

	#[inline(always)]
	fn mkdir(&self) -> defs::FnMkdir { self.0.mkdir.expect("mkdir") }

	#[inline(always)]
	fn open(&self) -> defs::FnOpen { self.0.open.expect("open") }

	#[inline(always)]
	fn read(&self) -> defs::FnRead { self.0.read.expect("read") }

	#[inline(always)]
	fn rename(&self) -> defs::FnRename { self.0.rename.expect("rename") }

	#[inline(always)]
	fn seek(&self) -> defs::FnSeek { self.0.seek.expect("seek") }

	#[inline(always)]
	fn stat(&self) -> defs::FnStat { self.0.stat.expect("stat") }

	#[inline(always)]
	fn tell(&self) -> defs::FnTell { self.0.tell.expect("tell") }

	#[inline(always)]
	fn unlink(&self) -> defs::FnUnlink { self.0.unlink.expect("unlink") }

	#[inline(always)]
	fn write(&self) -> defs::FnWrite { self.0.write.expect("write") }
}


pub trait Api {
	/// Returns [`sys::ffi::playdate_file::close`].
	#[doc(alias = "sys::ffi::playdate_file::close")]
	#[inline(always)]
	fn close(&self) -> defs::FnClose { *sys::api!(file.close) }

	/// Returns [`sys::ffi::playdate_file::flush`].
	#[doc(alias = "sys::ffi::playdate_file::flush")]
	#[inline(always)]
	fn flush(&self) -> defs::FnFlush { *sys::api!(file.flush) }

	/// Returns [`sys::ffi::playdate_file::geterr`].
	#[doc(alias = "sys::ffi::playdate_file::geterr")]
	#[inline(always)]
	fn geterr(&self) -> defs::FnGeterr { *sys::api!(file.geterr) }

	/// Returns [`sys::ffi::playdate_file::listfiles`].
	#[doc(alias = "sys::ffi::playdate_file::listfiles")]
	#[inline(always)]
	fn listfiles(&self) -> defs::FnListfiles { *sys::api!(file.listfiles) }

	/// Returns [`sys::ffi::playdate_file::mkdir`].
	#[doc(alias = "sys::ffi::playdate_file::mkdir")]
	#[inline(always)]
	fn mkdir(&self) -> defs::FnMkdir { *sys::api!(file.mkdir) }

	/// Returns [`sys::ffi::playdate_file::open`].
	#[doc(alias = "sys::ffi::playdate_file::open")]
	#[inline(always)]
	fn open(&self) -> defs::FnOpen { *sys::api!(file.open) }

	/// Returns [`sys::ffi::playdate_file::read`].
	#[doc(alias = "sys::ffi::playdate_file::read")]
	#[inline(always)]
	fn read(&self) -> defs::FnRead { *sys::api!(file.read) }

	/// Returns [`sys::ffi::playdate_file::rename`].
	#[doc(alias = "sys::ffi::playdate_file::rename")]
	#[inline(always)]
	fn rename(&self) -> defs::FnRename { *sys::api!(file.rename) }

	/// Returns [`sys::ffi::playdate_file::seek`].
	#[doc(alias = "sys::ffi::playdate_file::seek")]
	#[inline(always)]
	fn seek(&self) -> defs::FnSeek { *sys::api!(file.seek) }

	/// Returns [`sys::ffi::playdate_file::stat`].
	#[doc(alias = "sys::ffi::playdate_file::stat")]
	#[inline(always)]
	fn stat(&self) -> defs::FnStat { *sys::api!(file.stat) }

	/// Returns [`sys::ffi::playdate_file::tell`].
	#[doc(alias = "sys::ffi::playdate_file::tell")]
	#[inline(always)]
	fn tell(&self) -> defs::FnTell { *sys::api!(file.tell) }

	/// Returns [`sys::ffi::playdate_file::unlink`].
	#[doc(alias = "sys::ffi::playdate_file::unlink")]
	#[inline(always)]
	fn unlink(&self) -> defs::FnUnlink { *sys::api!(file.unlink) }

	/// Returns [`sys::ffi::playdate_file::write`].
	#[doc(alias = "sys::ffi::playdate_file::write")]
	#[inline(always)]
	fn write(&self) -> defs::FnWrite { *sys::api!(file.write) }
}

impl<T: Api> Api for &'_ T {
	#[inline(always)]
	fn close(&self) -> defs::FnClose { (*self).close() }

	#[inline(always)]
	fn flush(&self) -> defs::FnFlush { (*self).flush() }

	#[inline(always)]
	fn geterr(&self) -> defs::FnGeterr { (*self).geterr() }

	#[inline(always)]
	fn listfiles(&self) -> defs::FnListfiles { (*self).listfiles() }

	#[inline(always)]
	fn mkdir(&self) -> defs::FnMkdir { (*self).mkdir() }

	#[inline(always)]
	fn open(&self) -> defs::FnOpen { (*self).open() }

	#[inline(always)]
	fn read(&self) -> defs::FnRead { (*self).read() }

	#[inline(always)]
	fn rename(&self) -> defs::FnRename { (*self).rename() }

	#[inline(always)]
	fn seek(&self) -> defs::FnSeek { (*self).seek() }

	#[inline(always)]
	fn stat(&self) -> defs::FnStat { (*self).stat() }

	#[inline(always)]
	fn tell(&self) -> defs::FnTell { (*self).tell() }

	#[inline(always)]
	fn unlink(&self) -> defs::FnUnlink { (*self).unlink() }

	#[inline(always)]
	fn write(&self) -> defs::FnWrite { (*self).write() }
}


#[doc(hidden)]
mod defs {
	#![doc(hidden)]
	use super::*;

	pub type FnClose = unsafe extern "C" fn(*mut c_void) -> c_int;
	pub type FnFlush = unsafe extern "C" fn(*mut c_void) -> c_int;
	pub type FnGeterr = unsafe extern "C" fn() -> *const c_char;
	pub type FnListfiles = unsafe extern "C" fn(*const c_char,
	                                            Option<unsafe extern "C" fn(*const c_char, *mut c_void)>,
	                                            *mut c_void,
	                                            c_int) -> c_int;
	pub type FnMkdir = unsafe extern "C" fn(*const c_char) -> c_int;
	pub type FnOpen = unsafe extern "C" fn(*const c_char, FileOptions) -> *mut c_void;
	pub type FnRead = unsafe extern "C" fn(*mut c_void, *mut c_void, c_uint) -> c_int;
	pub type FnRename = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;
	pub type FnSeek = unsafe extern "C" fn(*mut c_void, c_int, c_int) -> c_int;
	pub type FnStat = unsafe extern "C" fn(*const c_char, *mut FileStat) -> c_int;
	pub type FnTell = unsafe extern "C" fn(*mut c_void) -> c_int;
	pub type FnUnlink = unsafe extern "C" fn(*const c_char, c_int) -> c_int;
	pub type FnWrite = unsafe extern "C" fn(*mut c_void, *const c_void, c_uint) -> c_int;
}
