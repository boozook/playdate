use core::fmt;


pub type ApiError = sys::error::Error<self::Error>;


#[derive(Debug)]
pub enum Error {
	/// The file does not exist.
	FileNotExist,

	/// Causes when allocation failed and/or null-ptr returned.
	Alloc,

	/// Error caused by the file system.
	Fs(fs::error::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Error::Alloc => write!(f, "Snd: Allocation failed"),
			Error::FileNotExist => write!(f, "Snd: File doesn't exist"),
			Error::Fs(err) => err.fmt(f),
		}
	}
}


impl Into<ApiError> for Error {
	fn into(self) -> ApiError { ApiError::Api(self) }
}

impl From<fs::error::Error> for Error {
	fn from(err: fs::error::Error) -> Self { Self::Fs(err) }
}


impl core::error::Error for Error {}
