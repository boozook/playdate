#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(const_deref)]

#[macro_use]
extern crate sys;
extern crate alloc;

use core::ffi::c_int;
use core::ffi::c_uint;
use alloc::string::String;
use alloc::vec::Vec;

use error::Owned;
use error::ReadUtf8Error;
use options::FileOptionsExt;
use options::OpenOptions;
use seek::Whence;
pub use sys::ffi::FileStat;
pub use sys::ffi::FileOptions;

use file::File;
use path::Path;


mod op;
pub mod path;
pub mod file;
pub mod seek;
pub mod options;
pub mod error;


pub mod prelude {
	pub use sys::ffi::FileStat;
	pub use sys::ffi::FileOptions;
	pub use super::Fs;
	pub use super::path::*;
	pub use super::file::*;
	pub use super::seek::*;
	pub use super::options::*;
	pub use super::error::Owned as FsError;
}


type Api = &'static sys::ffi::PlaydateFile;


/// Read the entire contents of a file into a bytes vector.
/// Works similarly to [`std::fs::read`].
pub fn read<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<Vec<u8>, Owned> {
	let fs = Fs::default();

	let opts = FileOptions::new().read(true).read_data(data_dir);
	let mut file = fs.open(&path, opts)?;

	// determine size of file:
	let size = fs.metadata(path).map(|m| m.size).ok().unwrap_or(0);

	// prepare prefilled buffer:
	let mut buf = alloc::vec![0; size as usize];

	fs.read(&mut file, &mut buf, size)?;
	Ok(buf)
}


/// Read the entire contents of a file into a string.
/// Works similarly to [`std::fs::read_to_string`].
pub fn read_to_string<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<String, ReadUtf8Error> {
	let buf = read(path, data_dir).map_err(ReadUtf8Error::Fs)?;
	alloc::string::String::from_utf8(buf).map_err(ReadUtf8Error::Utf8)
}


/// Write a bytes of the entire `contents` of a file.
///
/// This function will create a file if it does not exist,
/// and will entirely replace its contents if it does.
///
/// Works similarly to [`std::fs::write`].
///
/// Uses [`sys::ffi::PlaydateFile::open`] and [`sys::ffi::PlaydateFile::write`].
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), Owned> {
	let mut file = File::options().write(true).append(false).open(&path)?;
	file.write(contents.as_ref())?;
	Ok(())
}


#[inline(always)]
/// Removes a file from the filesystem. Directory is a file too.
///
/// Works similarly to [`std::fs::remove_file`] and [`std::fs::remove_dir`].
///
/// Uses [`sys::ffi::PlaydateFile::unlink`].
pub fn remove<P: AsRef<Path>>(path: P) -> Result<(), Owned> { Fs::default().remove(path) }


/// Given a path, query the file system to get information about a file,
/// directory, etc.
#[inline(always)]
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<FileStat, Owned> { Fs::default().metadata(path) }


/// Renames the file at `from` to `to`.
///
/// It will overwrite the file at `to`.
///
/// It does not create intermediate folders.
#[inline(always)]
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), Owned> {
	Fs::default().rename(from, to)
}

/// Creates the given `path` in the `Data/<gameid>` folder.
///
/// It does not create intermediate folders.
#[inline(always)]
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), Owned> { Fs::default().create_dir(path) }


/// Removes a directory and all of its contents recursively.
/// Works similarly to [`std::fs::remove_file`], [`std::fs::remove_dir`] and [`std::fs::remove_dir_all`].
///
/// Caution: it also can delete file without any error, if `path` is a file.
///
/// Calls [`sys::ffi::PlaydateFile::unlink`] with `recursive`.
#[inline(always)]
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), Owned> { Fs::default().remove_dir_all(path) }


/// Playdate File-system API.
///
/// Uses inner api end-point for all operations.
///
/// Each method returns result with owned (cloned, reallocated) error.
///
/// For scoped version of this, that returns borrowed errors, see [`Fs`](scoped::Fs) and [`Fs::scoped()`].
#[derive(Clone, Copy)]
pub struct Fs(Api);

impl Default for Fs {
	fn default() -> Self { Self(api!(file)) }
}

impl Fs {
	pub const fn new(api: Api) -> Self { Self(api) }
}


impl Fs {
	/// Open file for given `options`.
	///
	/// Creates new [`File`] instance with copy of inner api end-point.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::open`]
	#[doc(alias = "sys::ffi::PlaydateFile::open")]
	#[inline(always)]
	pub fn open<P: AsRef<Path>, Opts: OpenOptions>(&self, path: P, options: Opts) -> Result<File, Owned> {
		op::open(self.0, path, options).map_err(Owned::from)
	}

	/// Closes the given file.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::close`]
	#[doc(alias = "sys::ffi::PlaydateFile::close")]
	#[inline(always)]
	pub fn close(&self, file: File) -> Result<(), Owned> { op::close(self.0, file).map_err(Owned::from) }


	/// Returns the current read/write offset in the given file.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::tell`]
	#[doc(alias = "sys::ffi::PlaydateFile::tell")]
	#[inline(always)]
	pub fn tell(&self, file: &mut File) -> Result<c_uint, Owned> {
		crate::op::tell(self.0, file).map_err(Owned::from)
	}

	/// Sets the read/write offset in the given file to pos, relative to the `whence`.
	/// - [`Whence::Start`] is relative to the beginning of the file,
	/// - [`Whence::Current`] is relative to the current position of the file,
	/// - [`Whence::End`] is relative to the end of the file.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::seek`]
	#[doc(alias = "sys::ffi::PlaydateFile::seek")]
	#[inline(always)]
	pub fn seek_raw(&self, file: &mut File, pos: c_int, whence: Whence) -> Result<(), Owned> {
		crate::op::seek(self.0, file, pos, whence).map_err(Owned::from)
	}


	/// Reads up to `len` bytes from the file into the buffer `to`.
	///
	/// Returns the number of bytes read (0 indicating end of file).
	///
	/// Caution: Vector must be prefilled with `0`s.
	/// ```no_run
	/// let mut buf = Vec::<u8>::with_capacity(size);
	/// buf.resize(size, 0);
	/// fs.read(&mut file, &mut buf, size)?;
	/// ```
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::read`]
	#[doc(alias = "sys::ffi::PlaydateFile::read")]
	#[inline(always)]
	pub fn read(&self, file: &mut File, to: &mut [u8], len: c_uint) -> Result<c_uint, Owned> {
		op::read(self.0, file, to, len).map_err(Owned::from)
	}


	/// Writes the buffer of bytes buf to the file.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::write`]
	#[doc(alias = "sys::ffi::PlaydateFile::write")]
	#[inline(always)]
	pub fn write(&self, file: &mut File, from: &[u8]) -> Result<c_uint, Owned> {
		op::write(self.0, file, from).map_err(Owned::from)
	}

	/// Flushes the output buffer of file immediately.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::flush`]
	#[doc(alias = "sys::ffi::PlaydateFile::flush")]
	#[inline(always)]
	pub fn flush(&self, file: &mut File) -> Result<c_uint, Owned> { op::flush(self.0, file).map_err(Owned::from) }


	/// Populates the [`FileStat`] stat with information about the file at `path`.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::stat`]
	#[doc(alias = "sys::ffi::PlaydateFile::stat")]
	#[inline(always)]
	pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileStat, Owned> {
		let mut stat = FileStat::default();
		self.metadata_to(path, &mut stat).map_err(Owned::from)?;
		Ok(stat)
	}

	/// Writes into the given `metadata` information about the file at `path`.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::stat`]
	#[doc(alias = "sys::ffi::PlaydateFile::stat")]
	#[inline(always)]
	pub fn metadata_to<P: AsRef<Path>>(&self, path: P, metadata: &mut FileStat) -> Result<(), Owned> {
		op::metadata(self.0, path, metadata).map_err(Owned::from)
	}


	/// Creates the given `path` in the `Data/<gameid>` folder.
	///
	/// It does not create intermediate folders.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::mkdir`]
	#[doc(alias = "sys::ffi::PlaydateFile::mkdir")]
	#[inline(always)]
	pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Owned> {
		op::create_dir(self.0, path).map_err(Owned::from)
	}

	/// Deletes the file at path.
	/// Directory is a file too, definitely.
	///
	/// See also [`remove_dir_all`](Fs::remove_dir_all).
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::unlink`]
	#[doc(alias = "sys::ffi::PlaydateFile::unlink")]
	#[inline(always)]
	pub fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), Owned> {
		op::remove(self.0, path).map_err(Owned::from)
	}

	/// Deletes the file at path.
	///
	/// If the `path` is a folder, this deletes everything inside the folder
	/// (including folders, folders inside those, and so on) as well as the folder itself.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::unlink`]
	#[doc(alias = "sys::ffi::PlaydateFile::unlink")]
	#[inline(always)]
	pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Owned> {
		op::remove_dir_all(self.0, path).map_err(Owned::from)
	}

	/// Renames the file at `from` to `to`.
	///
	/// It will overwrite the file at `to`.
	///
	/// It does not create intermediate folders.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::rename`]
	#[doc(alias = "sys::ffi::PlaydateFile::rename")]
	#[inline(always)]
	pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), Owned> {
		op::rename(self.0, from, to).map_err(Owned::from)
	}


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
	///
	/// Argument passed to the callback is a path of a file, for each file.
	/// It is borrowed from C-side and should be cloned if you want to keep it,
	/// e.g.: `path.to_owned()` or `path.to_string_lossy().into_owned()`.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::listfiles`]
	#[doc(alias = "sys::ffi::PlaydateFile::listfiles")]
	#[inline(always)]
	pub fn read_dir<P, Fn>(&self, path: P, callback: Fn, include_hidden: bool) -> Result<(), Owned>
		where P: AsRef<Path>,
		      Fn: FnMut(&Path) {
		op::read_dir(self.0, path, callback, include_hidden).map_err(Owned::from)
	}
}


pub mod scoped {
	use core::ffi::c_int;
	use core::ffi::c_uint;


	use sys::ffi::FileStat;
	use crate::error::Borrowed;
	use crate::file::File;
	use crate::op;
	use crate::options::OpenOptions;
	use crate::seek::Whence;
	use crate::Api;
	use crate::Path;


	impl super::Fs {
		/// Creates "scope" with [scoped `Fs`](Fs) with isolated results which are borrowed from C-side.
		pub fn scoped<F, T, E>(&self, mut op: F) -> Result<T, E>
			where F: FnMut(&mut Fs) -> Result<T, E> {
			op(&mut Fs(self.0))
		}

		pub fn new_scoped() -> Fs { Fs::default() }
	}


	/// Scoped version of [`Fs`](super::Fs).
	///
	/// Requires mutability for each call, but as benefit, returns results with borrowed error.
	pub struct Fs(Api);

	impl Fs {
		pub const fn new(api: Api) -> Self { Self(api) }

		/// Converts to [`Fs`](crate::Fs), that returns owned results and don't requires mutability.
		pub const fn unscope(self) -> crate::Fs { crate::Fs(self.0) }
	}

	impl Default for Fs {
		fn default() -> Self { Self::new(api!(file)) }
	}


	/// This deref impls gives ability to use `crate::Fs` as `scoped::Fs` hopefully safely.
	/// ```no_run
	/// use playdate_fs as fs;
	/// use fs::prelude::*;
	///
	/// const FILE: &Path = c"pdxinfo";
	///
	/// fn meta_if_exists(fs: &mut fs::scoped::Fs) -> Option<FileStat> {
	/// 	fs.metadata(FILE).ok() // does not clones error if it happens
	/// }
	///
	/// let mut fs = fs::Fs::default();
	/// let meta = meta_if_exists(&mut fs);   // call fs::scoped::Fs::metadata
	/// let meta = (&mut *fs).metadata(FILE); // call fs::scoped::Fs::metadata
	/// let meta = fs.metadata(FILE);         // call fs::Fs::metadata, clones error if it caused
	/// ```
	mod deref {
		use core::ops::{Deref, DerefMut};
		use crate::scoped;


		#[allow(clippy::unnecessary_operation)]
		const _: () = const {
			["Size of scoped Fs"][core::mem::size_of::<crate::Fs>() - core::mem::size_of::<scoped::Fs>()];
			["Size of relaxed Fs"][core::mem::size_of::<scoped::Fs>() - core::mem::size_of::<crate::Fs>()];
		};

		/// Down-conversion from one which returns owned results to this the [scoped](scoped::Fs).
		impl const Deref for crate::Fs {
			type Target = scoped::Fs;
			fn deref(&self) -> &Self::Target { unsafe { core::mem::transmute(self) } }
		}

		impl const DerefMut for crate::Fs {
			fn deref_mut(&mut self) -> &mut scoped::Fs { unsafe { core::mem::transmute(self) } }
		}
	}


	impl Fs {
		/// Open file for given `options`.
		///
		/// Creates new [`File`] instance with copy of inner api end-point.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::open`]
		#[doc(alias = "sys::ffi::PlaydateFile::open")]
		#[inline(always)]
		pub fn open<'t, P: AsRef<Path>, Opts: OpenOptions>(&'t mut self,
		                                                   path: P,
		                                                   options: Opts)
		                                                   -> Result<File, Borrowed<'t>> {
			op::open(self.0, path, options)
		}

		/// Closes the given file.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::close`]
		#[doc(alias = "sys::ffi::PlaydateFile::close")]
		#[inline(always)]
		pub fn close<'t>(&'t mut self, file: File) -> Result<(), Borrowed<'t>> { op::close(self.0, file) }


		/// Returns the current read/write offset in the given file.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::tell`]
		#[doc(alias = "sys::ffi::PlaydateFile::tell")]
		#[inline(always)]
		pub fn tell<'t>(&'t mut self, file: &mut File) -> Result<c_uint, Borrowed<'t>> {
			crate::op::tell(self.0, file)
		}

		/// Sets the read/write offset in the given file to pos, relative to the `whence`.
		/// - [`Whence::Start`] is relative to the beginning of the file,
		/// - [`Whence::Current`] is relative to the current position of the file,
		/// - [`Whence::End`] is relative to the end of the file.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::seek`]
		#[doc(alias = "sys::ffi::PlaydateFile::seek")]
		#[inline(always)]
		pub fn seek_raw<'t>(&'t mut self, file: &mut File, pos: c_int, whence: Whence) -> Result<(), Borrowed<'t>> {
			crate::op::seek(self.0, file, pos, whence)
		}


		/// Reads up to `len` bytes from the file into the buffer `to`.
		///
		/// Returns the number of bytes read (0 indicating end of file).
		///
		/// Caution: Vector must be prefilled with `0`s.
		/// ```no_run
		/// let mut buf = Vec::<u8>::with_capacity(size);
		/// buf.resize(size, 0);
		/// fs.read(&mut file, &mut buf, size)?;
		/// ```
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::read`]
		#[doc(alias = "sys::ffi::PlaydateFile::read")]
		#[inline(always)]
		pub fn read<'t>(&'t mut self, file: &mut File, to: &mut [u8], len: c_uint) -> Result<c_uint, Borrowed<'t>> {
			op::read(self.0, file, to, len)
		}


		/// Writes the buffer of bytes buf to the file.
		///
		/// Returns the number of bytes written.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::write`]
		#[doc(alias = "sys::ffi::PlaydateFile::write")]
		#[inline(always)]
		pub fn write<'t>(&'t mut self, file: &mut File, from: &[u8]) -> Result<c_uint, Borrowed<'t>> {
			op::write(self.0, file, from)
		}

		/// Flushes the output buffer of file immediately.
		///
		/// Returns the number of bytes written.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::flush`]
		#[doc(alias = "sys::ffi::PlaydateFile::flush")]
		#[inline(always)]
		pub fn flush<'t>(&'t mut self, file: &mut File) -> Result<c_uint, Borrowed<'t>> { op::flush(self.0, file) }


		/// Populates the [`FileStat`] stat with information about the file at `path`.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::stat`]
		#[doc(alias = "sys::ffi::PlaydateFile::stat")]
		#[inline(always)]
		pub fn metadata<'t, P: AsRef<Path>>(&'t mut self, path: P) -> Result<FileStat, Borrowed<'t>> {
			let mut stat = FileStat::default();
			self.metadata_to(path, &mut stat)?;
			Ok(stat)
		}

		/// Writes into the given `metadata` information about the file at `path`.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::stat`]
		#[doc(alias = "sys::ffi::PlaydateFile::stat")]
		#[inline(always)]
		pub fn metadata_to<'t, P: AsRef<Path>>(&'t mut self,
		                                       path: P,
		                                       metadata: &mut FileStat)
		                                       -> Result<(), Borrowed<'t>> {
			let path = path.as_ref();
			let result = unsafe { (self.0.stat)(path.as_ptr(), metadata) };
			Borrowed::from_code(result, self.0).map(|_| ())
		}


		/// Creates the given `path` in the `Data/<gameid>` folder.
		///
		/// It does not create intermediate folders.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::mkdir`]
		#[doc(alias = "sys::ffi::PlaydateFile::mkdir")]
		#[inline(always)]
		pub fn create_dir<'t, P: AsRef<Path>>(&'t mut self, path: P) -> Result<(), Borrowed<'t>> {
			op::create_dir(self.0, path)
		}

		/// Deletes the file at path.
		/// Directory is a file too, definitely.
		///
		/// See also [`remove_dir_all`](Fs::remove_dir_all).
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::unlink`]
		#[doc(alias = "sys::ffi::PlaydateFile::unlink")]
		#[inline(always)]
		pub fn remove<'t, P: AsRef<Path>>(&'t mut self, path: P) -> Result<(), Borrowed<'t>> {
			op::remove(self.0, path)
		}

		/// Deletes the file at path.
		///
		/// If the `path` is a folder, this deletes everything inside the folder
		/// (including folders, folders inside those, and so on) as well as the folder itself.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::unlink`]
		#[doc(alias = "sys::ffi::PlaydateFile::unlink")]
		#[inline(always)]
		pub fn remove_dir_all<'t, P: AsRef<Path>>(&'t mut self, path: P) -> Result<(), Borrowed<'t>> {
			op::remove_dir_all(self.0, path)
		}

		/// Renames the file at `from` to `to`.
		///
		/// It will overwrite the file at `to`.
		///
		/// It does not create intermediate folders.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::rename`]
		#[doc(alias = "sys::ffi::PlaydateFile::rename")]
		#[inline(always)]
		pub fn rename<'t, P: AsRef<Path>, Q: AsRef<Path>>(&'t mut self,
		                                                  from: P,
		                                                  to: Q)
		                                                  -> Result<(), Borrowed<'t>> {
			op::rename(self.0, from, to)
		}


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
		///
		/// Argument passed to the callback is a path of a file, for each file.
		/// It is borrowed from C-side and should be cloned if you want to keep it,
		/// e.g.: `path.to_owned()` or `path.to_string_lossy().into_owned()`.
		///
		/// Equivalent to [`sys::ffi::PlaydateFile::listfiles`]
		#[doc(alias = "sys::ffi::PlaydateFile::listfiles")]
		#[inline(always)]
		pub fn read_dir<'t, P, Fn>(&'t mut self,
		                           path: P,
		                           callback: Fn,
		                           include_hidden: bool)
		                           -> Result<(), Borrowed<'t>>
			where P: AsRef<Path>,
			      Fn: FnMut(&Path)
		{
			op::read_dir(self.0, path, callback, include_hidden)
		}
	}
}
