use core::ffi::c_uint;
use core::ptr::NonNull;
use alloc::vec::Vec;

use sys::ffi::FileOptions;
use sys::ffi::SdFile;

use crate::error::Owned;
use crate::{Api, Path};
use crate::seek::SeekFrom;
use crate::options::FileOptionsExt;
use crate::options::OpenOptions;
use crate::error::err_code_on_drop;


#[derive(Debug)]
#[must_use = "File will be closed on drop"]
pub struct File(pub(crate) *mut SdFile);

impl From<NonNull<SdFile>> for File {
	fn from(ptr: NonNull<SdFile>) -> Self { Self(ptr.as_ptr()) }
}


impl File {
	/// Creates a blank new set of options ready for configuration.
	/// All options are initially set to false.
	///
	/// It is equivalent to `FileOptions::new()`.
	#[must_use]
	pub fn options() -> impl OpenOptions + FileOptionsExt { FileOptions::new() }
}


impl File {
	/// Attempts to open a file in read-only mode.
	///
	/// See the [`OpenOptions::open`] method and [official docs][docs] for more details.
	///
	/// [docs]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#f-file.open
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::open`]
	#[doc(alias = "sys::ffi::PlaydateFile::open")]
	#[must_use]
	#[inline(always)]
	pub fn open<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<File, Owned> {
		Self::open_with(api!(file), path, data_dir)
	}

	/// Attempts to open a file in read-only mode, using the given `api`.
	///
	/// See the [`OpenOptions::open`] method and [official docs][docs] for more details.
	///
	/// [docs]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#f-file.open
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::open`]
	#[doc(alias = "sys::ffi::PlaydateFile::open")]
	#[must_use]
	pub fn open_with<P: AsRef<Path>>(api: Api, path: P, data_dir: bool) -> Result<File, Owned> {
		crate::op::open(api, path, FileOptions::new().read(true).read_data(data_dir)).map_err(Owned::from)
	}

	/// Reads up to `len` bytes from the file into the buffer `buf`.
	///
	/// Returns the number of bytes read (0 indicating end of file).
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::read`]
	#[doc(alias = "sys::ffi::PlaydateFile::read")]
	#[must_use]
	#[inline(always)]
	pub fn read(&mut self, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, Owned> {
		crate::op::read(api!(file), self, to, len).map_err(Owned::from)
	}

	/// Writes the buffer of bytes buf to the file.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::write`]
	#[doc(alias = "sys::ffi::PlaydateFile::write")]
	#[must_use]
	#[inline(always)]
	pub fn write(&mut self, from: &[u8]) -> Result<c_uint, Owned> {
		crate::op::write(api!(file), self, from).map_err(Owned::from)
	}

	/// Flushes the output buffer of file immediately.
	///
	/// Returns the number of bytes written.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::flush`]
	#[doc(alias = "sys::ffi::PlaydateFile::flush")]
	#[inline(always)]
	pub fn flush(&mut self) -> Result<c_uint, Owned> { crate::op::flush(api!(file), self).map_err(Owned::from) }

	/// Returns the current read/write offset in the given file handle.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::tell`]
	#[doc(alias = "sys::ffi::PlaydateFile::tell")]
	#[must_use]
	#[inline(always)]
	pub fn tell(&mut self) -> Result<c_uint, Owned> { crate::op::tell(api!(file), self).map_err(Owned::from) }

	/// Sets the read/write offset in the file to `pos`.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::seek`]
	#[doc(alias = "sys::ffi::PlaydateFile::seek")]
	#[inline(always)]
	pub fn seek(&mut self, pos: SeekFrom) -> Result<(), Owned> {
		let (whence, pos) = pos.into_parts();
		crate::op::seek(api!(file), self, pos, whence).map_err(Owned::from)
	}

	/// Closes this file.
	///
	/// Equivalent to [`sys::ffi::PlaydateFile::close`]
	#[doc(alias = "sys::ffi::PlaydateFile::close")]
	#[inline(always)]
	pub fn close(self) -> Result<(), Owned> { crate::op::close(api!(file), self).map_err(Owned::from) }
}


impl Drop for File {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let api = api!(file);
			let code = unsafe { (api.close)(self.0) };
			self.0 = core::ptr::null_mut();
			err_code_on_drop(code, api);
		}
	}
}
