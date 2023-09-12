#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]
#![feature(const_trait_impl)]

#[macro_use]
extern crate sys;
extern crate alloc;

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use alloc::string::String;
use alloc::vec::Vec;

use error::Error;
use options::FileOptionsExt;
use options::OpenOptions;
use seek::SeekFrom;
use seek::Whence;
use sys::error::OkOrNullFnErr;
pub use sys::ffi::FileStat;
pub use sys::ffi::FileOptions;
use sys::ffi::CString;
use sys::ffi::CStr;

use file::File;
use error::ApiError;


pub mod cache;
pub mod file;
pub mod seek;
pub mod options;
pub mod error;


pub type Path = str;


/// Read the entire contents of a file into a bytes vector.
/// > Works similarly to [`std::fs::read`].
fn read<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<Vec<u8>, ApiError> {
	let fs = Fs::new()?;
	let mut file = File::open(&path, data_dir)?;

	// determine size of file:
	let size = fs.metadata(path).map(|m| m.size).ok().unwrap_or(0);

	// prepare prefilled buffer:
	let mut buf = Vec::<u8>::with_capacity(size as usize);
	buf.resize(size as usize, 0);

	fs.read(&mut file, &mut buf, size)?;
	Ok(buf)
}


/// Read the entire contents of a file into a string.
/// > Works similarly to [`std::fs::read_to_string`].
pub fn read_to_string<P: AsRef<Path>>(path: P, data_partition: bool) -> Result<String, ApiError> {
	let buf = read(path, data_partition)?;
	alloc::string::String::from_utf8(buf).map_err(Into::into)
}


/// Write a slice as the entire contents of a file.
///
/// This function will create a file if it does not exist,
/// and will entirely replace its contents if it does.
///
/// > Works similarly to [`std::fs::write`].
///
/// Uses [`sys::ffi::playdate_file::open`] and [`sys::ffi::playdate_file::write`].
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), ApiError> {
	let fs = Fs::new()?;
	let mut file = File::options().write(true).append(false).open(&path)?;
	fs.write(&mut file, contents.as_ref())?;
	Ok(())
}


#[inline(always)]
/// Removes a file from the filesystem. Directory is a file too.
///
/// > Works similarly to [`std::fs::remove_file`] and [`std::fs::remove_dir`].
///
/// Uses [`sys::ffi::playdate_file::unlink`].
pub fn remove<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::new()?.remove(path) }


// TODO: metadata
// /// Given a path, query the file system to get information about a file,
// /// directory, etc.
// pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, SysError> { todo!() }

#[inline(always)]
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), ApiError> {
	Fs::new()?.rename(from, to)
}

// TODO: copy
// pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64>

#[inline(always)]
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::new()?.create_dir(path) }

// TODO: create_dir_all
// pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()>


// #[inline(always)]
// pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::new()?.remove(path) }

#[inline(always)]
/// Removes a directory and all of its contents recursively.
/// > Works similarly to [`std::fs::remove_file`], [`std::fs::remove_dir`] and [`std::fs::remove_dir_all`].
///
/// Caution: it also can delete file without any error, if `path` is a file.
///
/// Calls [`sys::ffi::playdate_file::unlink`] with `recursive`.
// XXX: TODO: Should we validate that `path` is a directory?
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::new()?.remove_dir_all(path) }

// TODO: read_dir -> iter ReadDir
// pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>


pub trait FileSystemRaw {
	#![doc(hidden)]
	type AccessError: core::error::Error;

	/// Get [`sys::ffi::playdate_file::open`].
	fn _open(&self) -> Result<cache::FnOpen, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::close`].
	fn _close(&self) -> Result<cache::FnClose, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::flush`].
	fn _flush(&self) -> Result<cache::FnFlush, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::geterr`].
	fn _geterr(&self) -> Result<cache::FnGeterr, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::listfiles`].
	fn _listfiles(&self) -> Result<cache::FnListfiles, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::mkdir`].
	fn _mkdir(&self) -> Result<cache::FnMkdir, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::read`].
	fn _read(&self) -> Result<cache::FnRead, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::rename`].
	fn _rename(&self) -> Result<cache::FnRename, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::seek`].
	fn _seek(&self) -> Result<cache::FnSeek, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::stat`].
	fn _stat(&self) -> Result<cache::FnStat, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::tell`].
	fn _tell(&self) -> Result<cache::FnTell, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::unlink`].
	fn _unlink(&self) -> Result<cache::FnUnlink, Self::AccessError>;
	/// Get [`sys::ffi::playdate_file::write`].
	fn _write(&self) -> Result<cache::FnWrite, Self::AccessError>;
}


pub trait FileSystem: FileSystemRaw
	where <Self as FileSystem>::Error: From<Self::AccessError> {
	type Error: core::error::Error + From<Self::AccessError>;

	/// Open file for given `options`.
	fn open<P: AsRef<Path>, Opts: OpenOptions>(&self, path: P, options: Opts) -> Result<File, Self::Error>;
	/// Close the `file`.
	fn close(&self, file: File) -> Result<(), Self::Error>;

	/// Get current cursor position in the `file`, in bytes.
	fn tell(&self, file: &mut File) -> Result<c_uint, Self::Error>;
	/// Sets the read/write offset in the given file handle to the `pos`, in bytes.
	/// > Works similarly to [`std::io::Seek::seek`].
	fn seek(&self, file: &mut File, pos: SeekFrom) -> Result<c_uint, Self::Error> {
		let (whence, pos) = pos.into_parts();
		self.seek_raw(file, pos, whence)
	}
	/// Seek to an offset, in bytes.
	/// > Works similarly to [`std::io::Seek::seek`].
	fn seek_raw(&self, file: &mut File, pos: c_int, whence: Whence) -> Result<c_uint, Self::Error>;

	/// Read the `len` bytes of a file into the `to` vector.
	/// > Works similarly to [`std::io::Read::read`].
	fn read(&self, file: &mut File, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, Self::Error>;

	/// Write a buffer into the `file`, returning how many bytes were written.
	/// > Works similarly to [`std::io::Write::write`].
	fn write(&self, file: &mut File, from: &[u8]) -> Result<c_uint, Self::Error>;
	/// Flush the `file`, ensuring that all intermediately buffered
	/// contents reach their destination.
	/// > Works similarly to [`std::io::Write::flush`].
	fn flush(&self, file: &mut File) -> Result<c_uint, Self::Error>;

	fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileStat, Self::Error>;
	fn metadata_to<P: AsRef<Path>>(&self, path: P, metadata: &mut FileStat) -> Result<(), Self::Error>;

	fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
	fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
	fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
	fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), Self::Error>;

	fn read_dir<P, Fn>(&self, path: P, callback: Fn, include_hidden: bool) -> Result<(), Self::Error>
		where P: AsRef<Path>,
		      Fn: FnMut(String);
}


// TODO: Ideally this should be default specialized implementation, discuss it.
impl<T, E> FileSystem for T
	where T: FileSystemRaw<AccessError = E>,
	      ApiError: From<E>
{
	type Error = ApiError;

	// TODO: Doc for this concrete implementation
	fn open<P: AsRef<Path>, Opts: OpenOptions>(&self, path: P, options: Opts) -> Result<File, ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self._open()?;
		let ptr = unsafe { f(path.as_ptr() as _, options.into()) };
		Ok(File(ptr as _))
	}

	// TODO: Doc for this concrete implementation
	fn close(&self, mut file: File) -> Result<(), ApiError> {
		let f = self._close()?;
		let result = unsafe { f(file.0 as _) };
		file.0 = core::ptr::null_mut();
		Error::ok_from_code(result)?;
		Ok(())
	}


	// read / write //

	/// Returns the current read/write offset in the given file handle.
	fn tell(&self, file: &mut File) -> Result<c_uint, ApiError> {
		let f = self._tell()?;
		let result = unsafe { f(file.0) };
		Error::ok_from_code(result)
	}

	/// Sets the read/write offset in the given file handle to pos, relative to the `whence`.
	fn seek_raw(&self, file: &mut File, pos: c_int, whence: Whence) -> Result<c_uint, ApiError> {
		let f = self._seek()?;
		let result = unsafe { f(file.0, pos, whence as _) };
		Error::ok_from_code(result)
	}


	// read //

	/// Reads up to `len` bytes from the file into the buffer `to`.
	///
	/// Returns the number of bytes read (0 indicating end of file).
	///
	/// Caution: Vector must be prefilled with `0`s.
	/// ```no_run
	/// let mut buf = Vec::<u8>::with_capacity(size as usize);
	/// buf.resize(size as usize, 0);
	/// fs.read(&mut file, &mut buf, size)?;
	/// ```
	fn read(&self, file: &mut File, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, ApiError> {
		let f = self._read()?;
		let result = unsafe { f(file.0, to.as_mut_ptr() as *mut _, len) };
		Error::ok_from_code(result).into()
	}


	// write //

	/// Writes the buffer of bytes buf to the file. Returns the number of bytes written.
	fn write(&self, file: &mut File, from: &[u8]) -> Result<c_uint, ApiError> {
		let f = self._write()?;
		let result = unsafe { f(file.0, from.as_ptr() as *mut _, from.len() as _) };
		Error::ok_from_code(result)
	}

	/// Flushes the output buffer of file immediately. Returns the number of bytes written.
	fn flush(&self, file: &mut File) -> Result<c_uint, ApiError> {
		let f = self._flush()?;
		let result = unsafe { f(file.0) };
		Error::ok_from_code(result)
	}


	// metadata //

	fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileStat, ApiError> {
		let mut stat = FileStat { isdir: 0,
		                          size: 0,
		                          m_year: 0,
		                          m_month: 0,
		                          m_day: 0,
		                          m_hour: 0,
		                          m_minute: 0,
		                          m_second: 0 };
		self.metadata_to(path, &mut stat).map(|_| stat)
	}

	fn metadata_to<P: AsRef<Path>>(&self, path: P, metadata: &mut FileStat) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self._stat()?;
		let result = unsafe { f(path.as_ptr() as _, metadata as *mut _) };
		Error::ok_from_code(result).map(|_| ())
	}


	// path- fs operations //

	/// Creates the given `path` in the `Data/<gameid>` folder. It does not create intermediate folders.
	fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self._mkdir()?;
		let result = unsafe { f(path.as_ptr() as _) };
		Error::ok_from_code(result).map(|_| ())
	}

	/// Deletes the file at path.
	/// Directory is a file too definitely.
	fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self._unlink()?;
		let result = unsafe { f(path.as_ptr() as _, 0) };
		Error::ok_from_code(result).map(|_| ())
	}

	/// Deletes the file at path.
	/// If the `path` is a folder, this deletes everything inside the folder
	/// (including folders, folders inside those, and so on) as well as the folder itself.
	fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self._unlink()?;
		let result = unsafe { f(path.as_ptr() as _, 1) };
		Error::ok_from_code(result).map(|_| ())
	}

	/// Renames the file at `from` to `to`.
	///
	/// It will overwrite the file at `to`.
	///
	/// It does not create intermediate folders.
	fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), ApiError> {
		let from = CString::new(from.as_ref())?;
		let to = CString::new(to.as_ref())?;
		let f = self._rename()?;
		let result = unsafe { f(from.as_ptr() as _, to.as_ptr() as _) };
		Error::ok_from_code(result).map(|_| ())
	}


	// read dir //

	fn read_dir<P, Fn>(&self, path: P, mut callback: Fn, include_hidden: bool) -> Result<(), ApiError>
		where P: AsRef<Path>,
		      Fn: FnMut(String) {
		let path = CString::new(path.as_ref())?;

		unsafe extern "C" fn proxy<Fn: FnMut(String)>(filename: *const c_char, userdata: *mut c_void) {
			// TODO: are we really needs `into_owned` for storing in the temp vec?
			let filename = CStr::from_ptr(filename as _).to_string_lossy().into_owned();

			if let Some(callback) = (userdata as *mut _ as *mut Fn).as_mut() {
				callback(filename);
			} else {
				panic!("Fs.read_dir: missed callback");
			}
		}

		// NOTE: that's safe because ref dies after internal listfiles() returns.
		let callback_ref = (&mut callback) as *mut Fn as *mut _;

		let f = self._listfiles()?;
		let result = unsafe {
			f(
			  path.as_ptr() as _,
			  Some(proxy::<Fn>),
			  callback_ref,
			  include_hidden as _,
			)
		};
		Error::ok_from_code(result).map(|_| ())
	}
}


/// File-system API cached end-point.
/// Useful if you're making many operations on fs.
pub struct Fs(&'static sys::ffi::playdate_file);

impl Fs {
	pub fn new() -> Result<Fs, sys::error::NullPtrError> { Ok(Self(sys::api_ok!(file)?)) }
}

impl FileSystemRaw for Fs {
	#![doc(hidden)]
	type AccessError = sys::error::NullPtrError;

	#[inline(always)]
	fn _close(&self) -> Result<cache::FnClose, Self::AccessError> { self.0.close.ok_or_null() }

	#[inline(always)]
	fn _flush(&self) -> Result<cache::FnFlush, Self::AccessError> { self.0.flush.ok_or_null() }

	#[inline(always)]
	fn _geterr(&self) -> Result<cache::FnGeterr, Self::AccessError> { self.0.geterr.ok_or_null() }

	#[inline(always)]
	fn _listfiles(&self) -> Result<cache::FnListfiles, Self::AccessError> { self.0.listfiles.ok_or_null() }

	#[inline(always)]
	fn _mkdir(&self) -> Result<cache::FnMkdir, Self::AccessError> { self.0.mkdir.ok_or_null() }

	#[inline(always)]
	fn _open(&self) -> Result<cache::FnOpen, Self::AccessError> { self.0.open.ok_or_null() }

	#[inline(always)]
	fn _read(&self) -> Result<cache::FnRead, Self::AccessError> { self.0.read.ok_or_null() }

	#[inline(always)]
	fn _rename(&self) -> Result<cache::FnRename, Self::AccessError> { self.0.rename.ok_or_null() }

	#[inline(always)]
	fn _seek(&self) -> Result<cache::FnSeek, Self::AccessError> { self.0.seek.ok_or_null() }

	#[inline(always)]
	fn _stat(&self) -> Result<cache::FnStat, Self::AccessError> { self.0.stat.ok_or_null() }

	#[inline(always)]
	fn _tell(&self) -> Result<cache::FnTell, Self::AccessError> { self.0.tell.ok_or_null() }

	#[inline(always)]
	fn _unlink(&self) -> Result<cache::FnUnlink, Self::AccessError> { self.0.unlink.ok_or_null() }

	#[inline(always)]
	fn _write(&self) -> Result<cache::FnWrite, Self::AccessError> { self.0.write.ok_or_null() }
}


pub mod prelude {
	pub use sys::ffi::FileStat;
	pub use sys::ffi::FileOptions;
	pub use crate::error::ApiError as FsApiError;
	pub use crate::error::Error as FsError;
	pub use crate::Path;
	pub use crate::Fs;
	pub use crate::file::*;
	pub use crate::options::*;
	pub use crate::seek::SeekFrom;
}
