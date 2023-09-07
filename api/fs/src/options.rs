use alloc::borrow::ToOwned;
use sys::ffi::FileOptions;

use crate::FileSystem;
use crate::Fs;
use crate::Path;
use crate::error::ApiError;


/// Extension for [`sys::ffi::FileOptions`] make it looks like [`std::fs::OpenOptions`].
#[const_trait]
pub trait FileOptionsExt: Into<FileOptions> {
	fn new() -> Self;

	fn read(self, read: bool) -> Self;
	fn read_data(self, read: bool) -> Self;

	fn write(self, write: bool) -> Self;
	fn append(self, append: bool) -> Self;

	fn is_empty(&self) -> bool;
	fn is_read(&self) -> bool;
	fn is_read_data(&self) -> bool;
	fn is_write(&self) -> bool;
	fn is_append(&self) -> bool;

	fn is_read_any(&self) -> bool;
	fn is_write_any(&self) -> bool;
}

pub trait OpenOptions: Into<FileOptions> {
	fn open<P: AsRef<Path>>(&self, path: P) -> Result<crate::file::File, ApiError>;
	fn open_using<P: AsRef<Path>>(&self, path: P, fs: &Fs) -> Result<crate::file::File, ApiError>;
}

impl OpenOptions for FileOptions {
	#[inline(always)]
	fn open<P: AsRef<Path>>(&self, path: P) -> Result<crate::file::File, ApiError> {
		Fs::new()?.open(path, self.to_owned())
	}

	#[inline(always)]
	fn open_using<P: AsRef<Path>>(&self, path: P, fs: &Fs) -> Result<crate::file::File, ApiError> {
		fs.open(path, self.to_owned())
	}
}


impl const FileOptionsExt for FileOptions {
	fn new() -> Self { FileOptions(0) }

	/// Read access to Game’s package (bundle) directory.
	fn read(mut self, read: bool) -> Self {
		if read {
			self.0 |= FileOptions::kFileRead.0;
		} else {
			self.0 &= 255 - FileOptions::kFileRead.0;
		}
		self
	}

	/// Read access to Game’s data directory.
	fn read_data(mut self, read: bool) -> Self {
		if read {
			self.0 |= FileOptions::kFileReadData.0;
		} else {
			self.0 &= 255 - FileOptions::kFileReadData.0;
		}
		self
	}

	/// Write access to Game’s data directory.
	fn write(mut self, write: bool) -> Self {
		if write {
			self.0 |= FileOptions::kFileWrite.0;
		} else {
			self.0 &= 255 - FileOptions::kFileWrite.0;
		}
		self
	}

	/// Write access to Game’s data directory.
	fn append(mut self, append: bool) -> Self {
		if append {
			self.0 |= FileOptions::kFileAppend.0;
		} else {
			self.0 &= 255 - FileOptions::kFileAppend.0;
		}
		self
	}


	fn is_empty(&self) -> bool { self.0 == 0 }
	fn is_read(&self) -> bool { (self.0 & FileOptions::kFileRead.0) != 0 }
	fn is_read_data(&self) -> bool { (self.0 & FileOptions::kFileReadData.0) != 0 }
	fn is_write(&self) -> bool { (self.0 & FileOptions::kFileWrite.0) != 0 }
	fn is_append(&self) -> bool { (self.0 & FileOptions::kFileAppend.0) != 0 }

	fn is_read_any(&self) -> bool { self.is_read() || self.is_read_data() }
	fn is_write_any(&self) -> bool { self.is_write() || self.is_append() }
}

#[cfg(test)]
mod tests {
	use super::{FileOptionsExt, FileOptions};
	use FileOptions as FO;

	#[test]
	fn fo_empty() {
		let fo = FO::new();
		assert!(fo.is_empty());
		assert_eq!(fo.0, 0);
		assert!(!fo.is_read());
		assert!(!fo.is_read_data());
		assert!(!fo.is_write());
		assert!(!fo.is_append());
	}

	#[test]
	fn fo_read() {
		let fo = FO::new().read(true);
		assert_eq!(fo.0, FO::kFileRead.0);
		assert!(fo.is_read());
		assert!(!fo.is_read_data());
		assert!(!fo.is_write());
		assert!(!fo.is_append());
		assert!(!fo.is_empty());
	}

	#[test]
	fn fo_read_data() {
		let fo = FO::new().read_data(true);
		assert_ne!(fo.0, FO::kFileRead.0);
		assert_eq!(fo.0, FO::kFileReadData.0);
		assert!(!fo.is_read());
		assert!(fo.is_read_data());
		assert!(!fo.is_write());
		assert!(!fo.is_append());
		assert!(!fo.is_empty());

		let fo = FO::new().read(true).read_data(true);
		assert_ne!(fo.0, FO::kFileRead.0);
		assert_ne!(fo.0, FO::kFileReadData.0);
		assert!(fo.is_read());
		assert!(fo.is_read_data());
		assert!(!fo.is_write());
		assert!(!fo.is_append());
		assert!(!fo.is_empty());
	}

	#[test]
	fn fo_many() {
		let fo = FO::new().read(true)
		                  .write(true)
		                  .read_data(true)
		                  .append(true)
		                  .read(false)
		                  .append(false);
		assert!(!fo.is_read());
		assert!(fo.is_read_data());
		assert!(fo.is_write());
		assert!(!fo.is_append());
		assert!(!fo.is_empty());
	}
}
