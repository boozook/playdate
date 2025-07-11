use core::ffi::c_uint;
use core::ptr::NonNull;

use sys::ffi::FileOptions;
use sys::ffi::SdFile;
use sys::utils::AsRaw;

use crate::error::Owned;
use crate::{Api, Path};
use crate::seek::SeekFrom;
use crate::options::FileOptionsExt;
use crate::options::OpenOptions;
use crate::error::err_code_on_drop;


#[derive(Debug)]
#[must_use = "File will be closed on drop"]
pub struct File(pub(crate) *mut SdFile);

impl const AsRaw for File {
	type Output = SdFile;
	unsafe fn as_raw(&self) -> NonNull<SdFile> { NonNull::new(self.0).expect("non-null ptr") }
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
	pub fn read(&mut self, to: &mut [u8], len: c_uint) -> Result<c_uint, Owned> {
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


#[cfg(any(feature = "no_std_io2", feature = "portable-io"))]
mod io {
	use super::File;
	use alloc::boxed::Box;

	#[cfg(feature = "portable-io")]
	use portable_io as io;
	#[cfg(all(feature = "no_std_io2", not(feature = "portable-io")))]
	use no_std_io2::io;


	impl io::Read for File {
		fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
			debug_assert!(buf.len() <= core::ffi::c_uint::MAX as usize);

			File::read(self, buf, buf.len() as _).map_err(|e| io::Error::other(Box::new(e)))
			                                     .map(|v| v as usize)
		}

		fn read_exact(&mut self, mut buf: &mut [u8]) -> io::Result<()> {
			debug_assert!(buf.len() <= core::ffi::c_uint::MAX as usize);

			while !buf.is_empty() {
				match self.read(buf, buf.len() as _) {
					Ok(0) => break,
					Ok(n) => {
						buf = &mut buf[n as usize..];
					},
					Err(e) => return Err(io::Error::other(Box::new(e))),
				}
			}
			if !buf.is_empty() {
				Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
			} else {
				Ok(())
			}
		}
	}


	impl io::Write for File {
		fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
			debug_assert!(buf.len() <= core::ffi::c_uint::MAX as usize);

			File::write(self, buf).map_err(|e| io::Error::other(Box::new(e)))
			                      .map(|v| v as usize)
		}

		fn flush(&mut self) -> io::Result<()> {
			File::flush(self).map_err(|e| io::Error::other(Box::new(e)))?;
			Ok(())
		}
	}


	impl io::Seek for File {
		fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
			#[cfg(debug_assertions)]
			match pos {
				io::SeekFrom::Start(v) => assert!(v <= core::ffi::c_uint::MAX as u64),
				io::SeekFrom::End(v) => assert!(v.abs() <= core::ffi::c_int::MAX as i64),
				io::SeekFrom::Current(v) => assert!(v.abs() <= core::ffi::c_int::MAX as i64),
			}

			File::seek(self, pos.into()).and_then(|_| File::tell(self))
			                            .map(|v| v as _)
			                            .map_err(|e| io::Error::other(Box::new(e)))
		}


		#[cfg(feature = "portable-io")]
		fn stream_len(&mut self) -> io::Result<u64> {
			let old_pos = self.stream_position()?;

			self.seek(crate::seek::SeekFrom::End(0))
			    .map_err(|e| io::Error::other(Box::new(e)))?;
			let len = self.stream_position()?;

			if old_pos != len {
				self.seek(crate::seek::SeekFrom::Start(old_pos as _))
				    .map_err(|e| io::Error::other(Box::new(e)))?;
			}

			Ok(len)
		}

		#[cfg(feature = "portable-io")]
		fn stream_position(&mut self) -> io::Result<u64> {
			File::tell(self).map(|v| v as _)
			                .map_err(|e| io::Error::other(Box::new(e)))
		}

		#[cfg(feature = "portable-io")]
		fn rewind(&mut self) -> portable_io::Result<()> {
			self.seek(crate::seek::SeekFrom::Start(0))
			    .map_err(|e| io::Error::other(Box::new(e)))?;
			Ok(())
		}
	}


	impl Into<io::SeekFrom> for crate::seek::SeekFrom {
		fn into(self) -> io::SeekFrom {
			match self {
				crate::seek::SeekFrom::Start(v) => io::SeekFrom::Start(v as _),
				crate::seek::SeekFrom::Current(v) => io::SeekFrom::Current(v as _),
				crate::seek::SeekFrom::End(v) => io::SeekFrom::End(v as _),
			}
		}
	}

	impl From<io::SeekFrom> for crate::seek::SeekFrom {
		fn from(other: io::SeekFrom) -> Self {
			match other {
				io::SeekFrom::Start(v) => Self::Start(v as _),
				io::SeekFrom::End(v) => Self::End(v as _),
				io::SeekFrom::Current(v) => Self::Current(v as _),
			}
		}
	}
}
