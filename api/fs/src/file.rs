use core::ffi::c_uint;
use alloc::vec::Vec;

use sys::ffi::FileOptions;
use sys::ffi::SDFile;
use sys::traits::AsRaw;

use crate::api;
use crate::Path;
use crate::ApiError;
use crate::error::Error;
use crate::options::FileOptionsExt;
use crate::options::OpenOptions;
use crate::seek::SeekFrom;


pub trait AnyFile: AsRaw<Type = SDFile> {}
impl<T: AsRaw<Type = SDFile>> AnyFile for T {}


#[must_use = "File will be closed on drop"]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct File<Api: api::Api = api::Default>(pub(crate) *mut SDFile, pub(crate) Api);

// impl<Api: api::Api> File<Api> {
// 	pub type Default = File<api::Default>;
// 	pub type Cached = File<api::Cached>;
// }

impl<Api: api::Api> AsRaw for File<Api> {
	type Type = SDFile;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0 }
}


impl File<api::Default> {
	/// Creates a blank new set of options ready for configuration.
	/// All options are initially set to false.
	///
	/// It is equivalent to `FileOptions::new()`.
	#[must_use]
	pub fn options() -> impl OpenOptions + FileOptionsExt { FileOptions::new() }
}


impl<Api: api::Api> File<Api> {
	/// Attempts to open a file in read-only mode.
	///
	/// See the [`OpenOptions::open`] method and [official docs][docs] for more details.
	///
	/// [docs]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#f-file.open
	///
	/// Equivalent to [`sys::ffi::playdate_file::open`]
	#[doc(alias = "sys::ffi::playdate_file::open")]
	#[must_use]
	pub fn open<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<File<Api>, ApiError>
		where Api: Default {
		let api = Default::default();
		crate::ops::open(api, path, FileOptions::new().read(true).read_data(data_dir))
	}

	/// Attempts to open a file in read-only mode, using the given `api`.
	///
	/// See the [`OpenOptions::open`] method and [official docs][docs] for more details.
	///
	/// [docs]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#f-file.open
	///
	/// Equivalent to [`sys::ffi::playdate_file::open`]
	#[doc(alias = "sys::ffi::playdate_file::open")]
	#[must_use]
	pub fn open_with<P: AsRef<Path>>(api: Api, path: P, data_dir: bool) -> Result<File<Api>, ApiError> {
		crate::ops::open(api, path, FileOptions::new().read(true).read_data(data_dir))
	}

	/// Reads up to `len` bytes from the file into the buffer `buf`.
	///
	/// Returns the number of bytes read (0 indicating end of file).
	///
	/// Equivalent to [`sys::ffi::playdate_file::read`]
	#[doc(alias = "sys::ffi::playdate_file::read")]
	#[must_use]
	#[inline(always)]
	pub fn read(&mut self, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, Error> {
		crate::ops::read(self, to, len)
	}

	/// Writes the buffer of bytes buf to the file.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::playdate_file::write`]
	#[doc(alias = "sys::ffi::playdate_file::write")]
	#[must_use]
	#[inline(always)]
	pub fn write(&mut self, from: &[u8]) -> Result<c_uint, Error> { crate::ops::write(self, from) }

	/// Flushes the output buffer of file immediately.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::playdate_file::flush`]
	#[doc(alias = "sys::ffi::playdate_file::flush")]
	#[inline(always)]
	pub fn flush(&mut self) -> Result<c_uint, Error> { crate::ops::flush(self) }

	/// Returns the current read/write offset in the given file handle.
	///
	/// Equivalent to [`sys::ffi::playdate_file::tell`]
	#[doc(alias = "sys::ffi::playdate_file::tell")]
	#[must_use]
	#[inline(always)]
	pub fn tell(&mut self) -> Result<c_uint, Error> { crate::ops::tell(self) }

	/// Sets the read/write offset in the file to `pos`.
	///
	/// Equivalent to [`sys::ffi::playdate_file::seek`]
	#[doc(alias = "sys::ffi::playdate_file::seek")]
	#[inline(always)]
	pub fn seek(&mut self, pos: SeekFrom) -> Result<(), Error> {
		let (whence, pos) = pos.into_parts();
		crate::ops::seek(self, pos, whence)
	}

	/// Closes this file.
	///
	/// Equivalent to [`sys::ffi::playdate_file::close`]
	#[doc(alias = "sys::ffi::playdate_file::close")]
	#[inline(always)]
	pub fn close(self) -> Result<(), Error> { crate::ops::close(self) }
}

impl<Api: api::Api> Drop for File<Api> {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let f = self.1.close();
			let result = unsafe { f(self.0) };
			self.0 = core::ptr::null_mut();

			match Error::ok_from_code_with(result, &self.1) {
				Ok(_) => (),
				Err(err) => println!("Err on file-drop: {err}"),
			}
		}
	}
}
