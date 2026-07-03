use alloc::borrow::ToOwned;
use sys::ffi::FileOptions;

use crate::error::Owned;
use crate::Api;
use crate::Path;


/// Extension for [`sys::ffi::FileOptions`] make it looks like [`std::fs::OpenOptions`].
pub const trait FileOptionsExt: Into<FileOptions> {
	/// Creates new empty file options.
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
	/// Open file with this options.
	fn open<P: AsRef<Path>>(&self, path: P) -> Result<crate::file::File, Owned>;

	/// Open file with this options, using given `api`.
	fn open_with<P: AsRef<Path>>(&self, api: Api, path: P) -> Result<crate::file::File, Owned>;
}

impl OpenOptions for FileOptions {
	#[inline(always)]
	fn open<P: AsRef<Path>>(&self, path: P) -> Result<crate::file::File, Owned> {
		self.open_with(api!(file), path)
	}

	#[inline(always)]
	fn open_with<P: AsRef<Path>>(&self, api: Api, path: P) -> Result<crate::file::File, Owned> {
		crate::op::open(api, path, self.to_owned()).map_err(Owned::from)
	}
}


impl const FileOptionsExt for FileOptions {
	fn new() -> Self { FileOptions(0) }

	/// Read access to Game’s package (bundle) directory.
	fn read(mut self, read: bool) -> Self {
		if read {
			self.0 |= FileOptions::Read.0;
		} else {
			self.0 &= 255 - FileOptions::Read.0;
		}
		self
	}

	/// Read access to Game’s data directory.
	fn read_data(mut self, read: bool) -> Self {
		if read {
			self.0 |= FileOptions::ReadData.0;
		} else {
			self.0 &= 255 - FileOptions::ReadData.0;
		}
		self
	}

	/// Write access to Game’s data directory.
	fn write(mut self, write: bool) -> Self {
		if write {
			self.0 |= FileOptions::Write.0;
		} else {
			self.0 &= 255 - FileOptions::Write.0;
		}
		self
	}

	/// Write access to Game’s data directory.
	fn append(mut self, append: bool) -> Self {
		if append {
			self.0 |= FileOptions::Append.0;
		} else {
			self.0 &= 255 - FileOptions::Append.0;
		}
		self
	}


	fn is_empty(&self) -> bool { self.0 == 0 }
	fn is_read(&self) -> bool { (self.0 & FileOptions::Read.0) != 0 }
	fn is_read_data(&self) -> bool { (self.0 & FileOptions::ReadData.0) != 0 }
	fn is_write(&self) -> bool { (self.0 & FileOptions::Write.0) != 0 }
	fn is_append(&self) -> bool { (self.0 & FileOptions::Append.0) != 0 }

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
		assert_eq!(fo.0, FO::Read.0);
		assert!(fo.is_read());
		assert!(!fo.is_read_data());
		assert!(!fo.is_write());
		assert!(!fo.is_append());
		assert!(!fo.is_empty());
	}

	#[test]
	fn fo_read_data() {
		let fo = FO::new().read_data(true);
		assert_ne!(fo.0, FO::Read.0);
		assert_eq!(fo.0, FO::ReadData.0);
		assert!(!fo.is_read());
		assert!(fo.is_read_data());
		assert!(!fo.is_write());
		assert!(!fo.is_append());
		assert!(!fo.is_empty());

		let fo = FO::new().read(true).read_data(true);
		assert_ne!(fo.0, FO::Read.0);
		assert_ne!(fo.0, FO::ReadData.0);
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
