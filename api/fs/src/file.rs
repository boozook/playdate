use core::ffi::c_uint;
use alloc::vec::Vec;

use sys::error::OkOrNullFnErr;
use sys::ffi::FileOptions;
use sys::ffi::SDFile;

use crate::FileSystem;
use crate::Fs;
use crate::Path;
use crate::ApiError;
use crate::error::Error;
use crate::options::FileOptionsExt;
use crate::options::OpenOptions;
use crate::seek::SeekFrom;


#[must_use = "File will closed on drop"]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct File(pub(crate) *mut SDFile);

impl File {
	// Ensure the inner pointer is valid (not null).
	pub fn valid(&self) -> bool { !self.0.is_null() }

	/// Attempts to open a file in read-only mode.
	///
	/// See the [`OpenOptions::open`] method for more details.
	///
	/// https://sdk.play.date/2.0.1/Inside%20Playdate%20with%20C.html#f-file.open
	#[must_use]
	pub fn open<P: AsRef<Path>>(path: P, data_dir: bool) -> Result<File, ApiError> {
		Fs::new()?.open(path, FileOptions::new().read(true).read_data(data_dir))
	}

	/// Reads up to `len` bytes from the file into the buffer `buf`.
	/// Returns the number of bytes read (0 indicating end of file).
	pub fn read(&mut self, to: &mut Vec<u8>, len: c_uint) -> Result<c_uint, ApiError> {
		Fs::new()?.read(self, to, len)
	}

	/// Writes the buffer of bytes buf to the file. Returns the number of bytes written.
	pub fn write(&mut self, from: &[u8]) -> Result<c_uint, ApiError> { Fs::new()?.write(self, from) }

	/// Flushes the output buffer of file immediately. Returns the number of bytes written.
	pub fn flush(&mut self) -> Result<c_uint, ApiError> { Fs::new()?.flush(self) }

	/// Returns the current read/write offset in the given file handle.
	#[must_use]
	pub fn tell(&mut self) -> Result<c_uint, ApiError> { Fs::new()?.tell(self) }

	/// Sets the read/write offset in the given file handle to pos, relative to the `whence`.
	/// `Whence::Set` is relative to the beginning of the file,
	/// `Whence::Cur` is relative to the current position of the file pointer, and
	/// `Whence::End` is relative to the end of the file.
	/// Returns 0 on success.
	pub fn seek(&mut self, pos: SeekFrom) -> Result<c_uint, ApiError> { Fs::new()?.seek(self, pos) }

	/// Closes the this file handle.
	pub fn close(self) -> Result<(), ApiError> { Fs::new()?.close(self) }


	/// Creates a blank new set of options ready for configuration.
	/// All options are initially set to false.
	///
	/// It is equivalent to `FileOptions::new()`.
	#[must_use]
	pub fn options() -> impl OpenOptions + FileOptionsExt { FileOptions::new() }
}

impl Drop for File {
	fn drop(&mut self) {
		if self.valid() {
			fn print_err<E: core::fmt::Display>(err: E) { println!("Err on file-drop: {err}") }
			let _ = Fs::new().map(|fs| {
				                 let _ = fs.0
				                           .close
				                           .ok_or_null()
				                           .map(|f| {
					                           let result = unsafe { f(self.0) };
					                           self.0 = core::ptr::null_mut();
					                           match Error::ok_from_code(result) {
						                           Ok(_) => (),
					                              Err(err) => print_err(err),
					                           }
				                           })
				                           .map_err(|err| print_err(err))
				                           .ok();
			                 })
			                 .map_err(|err| print_err(err))
			                 .ok();
		}
	}
}
