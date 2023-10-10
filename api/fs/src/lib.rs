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
use file::AnyFile;
use options::FileOptionsExt;
use options::OpenOptions;
use seek::Whence;
pub use sys::ffi::FileStat;
pub use sys::ffi::FileOptions;
use sys::ffi::CString;
use sys::ffi::CStr;

use file::File;
use error::ApiError;


pub mod api;
pub mod file;
pub mod seek;
pub mod options;
pub mod error;


pub type Path = str;


/// Read the entire contents of a file into a bytes vector.
/// > Works similarly to [`std::fs::read`].
pub fn read<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<Vec<u8>, ApiError> {
	let fs = Fs::Cached();
	let opts = FileOptions::new().read(true).read_data(data_dir);
	let mut file = fs.open_with(api::Default, &path, opts)?;

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
pub fn read_to_string<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<String, ApiError> {
	let buf = read(path, data_dir)?;
	alloc::string::String::from_utf8(buf).map_err(Into::into)
}


/// Write a bytes of the entire `contents` of a file.
///
/// This function will create a file if it does not exist,
/// and will entirely replace its contents if it does.
///
/// > Works similarly to [`std::fs::write`].
///
/// Uses [`sys::ffi::playdate_file::open`] and [`sys::ffi::playdate_file::write`].
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), ApiError> {
	let mut file = File::options().write(true).append(false).open(&path)?;
	file.write(contents.as_ref())?;
	Ok(())
}


#[inline(always)]
/// Removes a file from the filesystem. Directory is a file too.
///
/// > Works similarly to [`std::fs::remove_file`] and [`std::fs::remove_dir`].
///
/// Uses [`sys::ffi::playdate_file::unlink`].
pub fn remove<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::Default().remove(path) }


// TODO: metadata
/// Given a path, query the file system to get information about a file,
/// directory, etc.
#[inline(always)]
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<FileStat, ApiError> { Fs::Default().metadata(path) }


/// Renames the file at `from` to `to`.
///
/// It will overwrite the file at `to`.
///
/// It does not create intermediate folders.
#[inline(always)]
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), ApiError> {
	Fs::Default().rename(from, to)
}

/// Creates the given `path` in the `Data/<gameid>` folder.
///
/// It does not create intermediate folders.
#[inline(always)]
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::Default().create_dir(path) }

// TODO: create_dir_all
// pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()>


#[inline(always)]
/// Removes a directory and all of its contents recursively.
/// > Works similarly to [`std::fs::remove_file`], [`std::fs::remove_dir`] and [`std::fs::remove_dir_all`].
///
/// Caution: it also can delete file without any error, if `path` is a file.
///
/// Calls [`sys::ffi::playdate_file::unlink`] with `recursive`.
// XXX: TODO: Should we validate that `path` is a directory?
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ApiError> { Fs::Default().remove_dir_all(path) }

// TODO: read_dir -> iter ReadDir
// pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>


/// Playdate File-system API.
///
/// Uses inner api end-point for all operations.
#[derive(Debug, Clone, Copy)]
pub struct Fs<Api = api::Default>(Api);

impl Fs<api::Default> {
	/// Creates default [`Fs`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Fs<api::Cache> {
	/// Creates [`Fs`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Fs<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Fs<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Fs<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}


mod ops {
	use super::*;

	pub fn open<Api: api::Api, P: AsRef<Path>, Opts: OpenOptions>(api: Api,
	                                                              path: P,
	                                                              options: Opts)
	                                                              -> Result<File<Api>, ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = api.open();
		let ptr = unsafe { f(path.as_ptr() as _, options.into()) };
		Ok(File(ptr as _, api))
	}

	pub fn open_with<UApi: api::Api, FApi: api::Api, P: AsRef<Path>, Opts: OpenOptions>(
		using: UApi,
		api: FApi,
		path: P,
		options: Opts)
		-> Result<File<FApi>, ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = using.open();
		let ptr = unsafe { f(path.as_ptr() as _, options.into()) };
		Ok(File(ptr as _, api))
	}

	pub fn close<Api: api::Api>(mut file: File<Api>) -> Result<(), Error> {
		let f = file.1.close();
		let result = unsafe { f(file.0 as _) };
		file.0 = core::ptr::null_mut();
		Error::ok_from_code(result)?;
		Ok(())
	}

	pub fn close_with<Api: api::Api, FApi: api::Api>(api: Api, mut file: File<FApi>) -> Result<(), Error> {
		let f = api.close();
		let result = unsafe { f(file.0) };
		file.0 = core::ptr::null_mut();
		Error::ok_from_code(result)?;
		Ok(())
	}

	pub fn seek<Api: api::Api>(file: &mut File<Api>, pos: c_int, whence: Whence) -> Result<(), Error> {
		let f = file.1.seek();
		let result = unsafe { f(file.0, pos, whence as _) };
		Error::ok_from_code(result)?;
		Ok(())
	}

	pub fn seek_with<Api: api::Api>(api: Api,
	                                file: &mut impl AnyFile,
	                                pos: c_int,
	                                whence: Whence)
	                                -> Result<(), Error> {
		let f = api.seek();
		let result = unsafe { f(file.as_raw(), pos, whence as _) };
		Error::ok_from_code(result)?;
		Ok(())
	}

	pub fn tell<Api: api::Api>(file: &mut File<Api>) -> Result<c_uint, Error> {
		let f = file.1.tell();
		let result = unsafe { f(file.0) };
		Error::ok_from_code(result)
	}

	pub fn tell_with<Api: api::Api>(api: Api, file: &mut impl AnyFile) -> Result<c_uint, Error> {
		let f = api.tell();
		let result = unsafe { f(file.as_raw()) };
		Error::ok_from_code(result)
	}


	pub fn read<Api: api::Api>(file: &mut File<Api>, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, Error> {
		let f = file.1.read();
		let result = unsafe { f(file.0, to.as_mut_ptr() as *mut _, len) };
		Error::ok_from_code(result)
	}

	pub fn write<Api: api::Api>(file: &mut File<Api>, from: &[u8]) -> Result<c_uint, Error> {
		let f = file.1.write();
		let result = unsafe { f(file.0, from.as_ptr() as *mut _, from.len() as _) };
		Error::ok_from_code(result)
	}

	pub fn flush<Api: api::Api>(file: &mut File<Api>) -> Result<c_uint, Error> {
		let f = file.1.flush();
		let result = unsafe { f(file.0) };
		Error::ok_from_code(result)
	}
}


impl<Api: api::Api> Fs<Api> {
	/// Open file for given `options`.
	///
	/// Creates new [`File`] instance with copy of inner api end-point.
	///
	/// Equivalent to [`sys::ffi::playdate_file::open`]
	#[doc(alias = "sys::ffi::playdate_file::open")]
	#[inline(always)]
	pub fn open<P: AsRef<Path>, Opts: OpenOptions>(&self, path: P, options: Opts) -> Result<File<Api>, ApiError>
		where Api: Copy {
		ops::open(self.0, path, options)
	}

	/// Open file for given `options`.
	///
	/// Creates new [`File`] instance with given `api`.
	///
	/// Equivalent to [`sys::ffi::playdate_file::open`]
	#[doc(alias = "sys::ffi::playdate_file::open")]
	#[inline(always)]
	pub fn open_with<T: api::Api, P: AsRef<Path>, Opts: OpenOptions>(&self,
	                                                                 api: T,
	                                                                 path: P,
	                                                                 options: Opts)
	                                                                 -> Result<File<T>, ApiError> {
		ops::open_with(&self.0, api, path, options)
	}

	/// Closes the given file.
	///
	/// Equivalent to [`sys::ffi::playdate_file::close`]
	#[doc(alias = "sys::ffi::playdate_file::close")]
	#[inline(always)]
	pub fn close<T: api::Api>(&self, file: File<T>) -> Result<(), Error> { ops::close_with(&self.0, file) }


	/// Returns the current read/write offset in the given file.
	///
	/// Equivalent to [`sys::ffi::playdate_file::tell`]
	#[doc(alias = "sys::ffi::playdate_file::tell")]
	#[inline(always)]
	pub fn tell(&self, file: &mut impl AnyFile) -> Result<c_uint, Error> { crate::ops::tell_with(&self.0, file) }

	/// Sets the read/write offset in the given file to pos, relative to the `whence`.
	/// - [`Whence::Start`] is relative to the beginning of the file,
	/// - [`Whence::Current`] is relative to the current position of the file,
	/// - [`Whence::End`] is relative to the end of the file.
	///
	/// Equivalent to [`sys::ffi::playdate_file::seek`]
	#[doc(alias = "sys::ffi::playdate_file::seek")]
	#[inline(always)]
	pub fn seek_raw(&self, file: &mut impl AnyFile, pos: c_int, whence: Whence) -> Result<(), Error> {
		crate::ops::seek_with(&self.0, file, pos, whence)
	}


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
	///
	/// Equivalent to [`sys::ffi::playdate_file::read`]
	#[doc(alias = "sys::ffi::playdate_file::read")]
	pub fn read(&self, file: &mut impl AnyFile, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, Error> {
		let f = self.0.read();
		let result = unsafe { f(file.as_raw(), to.as_mut_ptr() as *mut _, len) };
		Error::ok_from_code(result)
	}


	/// Writes the buffer of bytes buf to the file.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::playdate_file::write`]
	#[doc(alias = "sys::ffi::playdate_file::write")]
	pub fn write(&self, file: &mut impl AnyFile, from: &[u8]) -> Result<c_uint, Error> {
		let f = self.0.write();
		let result = unsafe { f(file.as_raw(), from.as_ptr() as *mut _, from.len() as _) };
		Error::ok_from_code(result)
	}

	/// Flushes the output buffer of file immediately.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::playdate_file::flush`]
	#[doc(alias = "sys::ffi::playdate_file::flush")]
	pub fn flush(&self, file: &mut impl AnyFile) -> Result<c_uint, Error> {
		let f = self.0.flush();
		let result = unsafe { f(file.as_raw()) };
		Error::ok_from_code(result)
	}


	/// Populates the [`FileStat`] stat with information about the file at `path`.
	///
	/// Equivalent to [`sys::ffi::playdate_file::stat`]
	#[doc(alias = "sys::ffi::playdate_file::stat")]
	pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileStat, ApiError> {
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

	/// Writes into the given `metadata` information about the file at `path`.
	///
	/// Equivalent to [`sys::ffi::playdate_file::stat`]
	#[doc(alias = "sys::ffi::playdate_file::stat")]
	pub fn metadata_to<P: AsRef<Path>>(&self, path: P, metadata: &mut FileStat) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self.0.stat();
		let result = unsafe { f(path.as_ptr() as _, metadata as *mut _) };
		Error::ok_from_code(result)?;
		Ok(())
	}


	// path- fs operations //

	/// Creates the given `path` in the `Data/<gameid>` folder.
	///
	/// It does not create intermediate folders.
	///
	/// Equivalent to [`sys::ffi::playdate_file::mkdir`]
	#[doc(alias = "sys::ffi::playdate_file::mkdir")]
	pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self.0.mkdir();
		let result = unsafe { f(path.as_ptr() as _) };
		Error::ok_from_code(result)?;
		Ok(())
	}

	/// Deletes the file at path.
	/// Directory is a file too, definitely.
	///
	/// See also [`remove_dir_all`](Fs::remove_dir_all).
	///
	/// Equivalent to [`sys::ffi::playdate_file::unlink`]
	#[doc(alias = "sys::ffi::playdate_file::unlink")]
	pub fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self.0.unlink();
		let result = unsafe { f(path.as_ptr() as _, 0) };
		Error::ok_from_code(result)?;
		Ok(())
	}

	/// Deletes the file at path.
	///
	/// If the `path` is a folder, this deletes everything inside the folder
	/// (including folders, folders inside those, and so on) as well as the folder itself.
	///
	/// Equivalent to [`sys::ffi::playdate_file::unlink`]
	#[doc(alias = "sys::ffi::playdate_file::unlink")]
	pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path = CString::new(path.as_ref())?;
		let f = self.0.unlink();
		let result = unsafe { f(path.as_ptr() as _, 1) };
		Error::ok_from_code(result)?;
		Ok(())
	}

	/// Renames the file at `from` to `to`.
	///
	/// It will overwrite the file at `to`.
	///
	/// It does not create intermediate folders.
	///
	/// Equivalent to [`sys::ffi::playdate_file::rename`]
	#[doc(alias = "sys::ffi::playdate_file::rename")]
	pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), ApiError> {
		let from = CString::new(from.as_ref())?;
		let to = CString::new(to.as_ref())?;
		let f = self.0.rename();
		let result = unsafe { f(from.as_ptr() as _, to.as_ptr() as _) };
		Error::ok_from_code(result)?;
		Ok(())
	}


	// read dir //

	/// Calls the given callback function for every file at `path`.
	///
	/// Subfolders are indicated by a trailing slash '/' in filename.
	///
	/// This method does not recurse into subfolders.
	///
	/// If `include_hidden` is set, files beginning with a period will be included;
	/// otherwise, they are skipped.
	///
	/// Returns error if no folder exists at path or it can’t be opened.
	///
	/// Equivalent to [`sys::ffi::playdate_file::listfiles`]
	#[doc(alias = "sys::ffi::playdate_file::listfiles")]
	pub fn read_dir<P, Fn>(&self, path: P, mut callback: Fn, include_hidden: bool) -> Result<(), ApiError>
		where P: AsRef<Path>,
		      Fn: FnMut(String) {
		unsafe extern "C" fn proxy<Fn: FnMut(String)>(filename: *const c_char, userdata: *mut c_void) {
			// TODO: are we really need `into_owned` for storing in the temp vec?
			let filename = CStr::from_ptr(filename as _).to_string_lossy().into_owned();

			if let Some(callback) = (userdata as *mut _ as *mut Fn).as_mut() {
				callback(filename);
			} else {
				panic!("Fs.read_dir: missed callback");
			}
		}

		let path = CString::new(path.as_ref())?;

		// NOTE: that's safe because ref dies after internal listfiles() returns.
		let callback_ref = (&mut callback) as *mut Fn as *mut _;

		let f = self.0.listfiles();
		let result = unsafe {
			f(
			  path.as_ptr() as _,
			  Some(proxy::<Fn>),
			  callback_ref,
			  include_hidden as _,
			)
		};
		Error::ok_from_code(result)?;
		Ok(())
	}
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
