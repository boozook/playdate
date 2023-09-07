use core::ffi::c_int;


#[derive(Debug, Clone, Copy)]
/// Same as [`std::io::SeekFrom`].
pub enum SeekFrom {
	/// Sets the offset to the provided number of bytes.
	///
	/// Same as [`std::io::SeekFrom::Start`].
	Start(c_int),

	/// Sets the offset to the current position plus the specified number of
	/// bytes.
	///
	/// It is possible to seek beyond the end of an object, but it's an error to
	/// seek before byte 0.
	///
	/// Same as [`std::io::SeekFrom::Current`].
	Current(c_int),

	/// Sets the offset to the size of this object plus the specified number of
	/// bytes.
	///
	/// It is possible to seek beyond the end of an object, but it's an error to
	/// seek before byte 0.
	///
	/// Same as [`std::io::SeekFrom::End`].
	End(c_int),
}

impl SeekFrom {
	pub const fn into_parts(self) -> (Whence, c_int) {
		match self {
			SeekFrom::Start(pos) => (Whence::Start, pos),
			SeekFrom::Current(pos) => (Whence::Current, pos),
			SeekFrom::End(pos) => (Whence::End, pos),
		}
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Whence {
	/// Equal to [`sys::ffi::SEEK_SET`].
	Start = sys::ffi::SEEK_SET as _,

	/// Equal to [`sys::ffi::SEEK_CUR`].
	Current = sys::ffi::SEEK_CUR as _,

	/// Equal to [`sys::ffi::SEEK_END`].
	End = sys::ffi::SEEK_END as _,
}
