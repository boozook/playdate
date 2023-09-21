use std::env::VarError;
use std::io::Error as IoError;

use bindgen::BindgenError;
use semver::Error as SemverError;
use utils::toolchain::gcc::err::Error as GccError;


#[derive(Debug)]
pub enum Error {
	Bindgen(BindgenError),
	Io(IoError),
	Env {
		err: VarError,
		ctx: &'static str,
	},
	Semver(SemverError),

	Gcc(GccError),

	#[cfg(feature = "extra-codegen")]
	Syn(syn::Error),
}


impl From<BindgenError> for Error {
	fn from(err: BindgenError) -> Self { Self::Bindgen(err) }
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self { Self::Io(err) }
}

impl From<SemverError> for Error {
	fn from(err: SemverError) -> Self { Self::Semver(err) }
}

impl From<GccError> for Error {
	fn from(err: GccError) -> Self { Self::Gcc(err) }
}

#[cfg(feature = "extra-codegen")]
impl From<syn::Error> for Error {
	fn from(err: syn::Error) -> Self { Self::Syn(err) }
}


impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Bindgen(err) => err.fmt(f),
			Error::Io(err) => err.fmt(f),
			Error::Env { err, ctx } => write!(f, "{err}: {ctx}"),
			Error::Semver(err) => err.fmt(f),
			Error::Gcc(err) => err.fmt(f),
			#[cfg(feature = "extra-codegen")]
			Error::Syn(err) => err.fmt(f),
		}
	}
}


impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Bindgen(err) => Some(err),
			Error::Io(err) => Some(err),
			Error::Env { err, .. } => Some(err),
			Error::Semver(err) => Some(err),
			Error::Gcc(err) => Some(err),
			#[cfg(feature = "extra-codegen")]
			Error::Syn(err) => Some(err),
		}
	}
}
