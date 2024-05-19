use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
	Io(IoError),
	Err(&'static str),
	#[cfg(feature = "serde_json")]
	Json(serde_json::error::Error),
	#[cfg(feature = "toml")]
	Toml(toml::de::Error),
}

impl From<&'static str> for Error {
	fn from(value: &'static str) -> Self { Self::Err(value) }
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self { Self::Io(err) }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::error::Error> for Error {
	fn from(err: serde_json::error::Error) -> Self { Self::Json(err) }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
	fn from(err: toml::de::Error) -> Self { Self::Toml(err) }
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Io(err) => err.fmt(f),
			Error::Err(err) => err.fmt(f),
			#[cfg(feature = "serde_json")]
			Error::Json(err) => err.fmt(f),
			#[cfg(feature = "toml")]
			Error::Toml(err) => err.fmt(f),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Err(_) => None,
			Error::Io(err) => Some(err),
			#[cfg(feature = "serde_json")]
			Error::Json(err) => Some(err),
			#[cfg(feature = "toml")]
			Error::Toml(err) => Some(err),
		}
	}
}
