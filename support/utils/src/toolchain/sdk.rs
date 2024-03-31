//! Playdate SDK

use std::borrow::Cow;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use crate::consts::SDK_ENV_VAR;


pub struct Sdk {
	path: PathBuf,
}


// Paths:
impl Sdk {
	pub fn path(&self) -> &Path { &self.path }
	pub fn c_api(&self) -> PathBuf { self.path.join("C_API") }
	pub fn build_support(&self) -> BuildSupport { BuildSupport { path: self.c_api().join("buildsupport").into() } }
	pub fn version_file(&self) -> PathBuf { self.path.join("VERSION.txt") }

	pub fn bin(&self) -> PathBuf { self.path.join("bin") }

	pub fn pdc(&self) -> PathBuf {
		#[cfg(unix)]
		const PDC: &'static str = "pdc";
		#[cfg(windows)]
		const PDC: &'static str = "PDC.EXE";
		self.bin().join(PDC)
	}

	pub fn pdutil(&self) -> PathBuf {
		#[cfg(unix)]
		const PDUTIL: &'static str = "pdutil";
		#[cfg(windows)]
		const PDUTIL: &'static str = "PDUTIL.EXE";
		self.bin().join(PDUTIL)
	}
}


// Constructors:
impl Sdk {
	/// Create new `Sdk` with auto-determining the SDK path
	pub fn try_new() -> Result<Self, Error> {
		let try_with = move |f: fn() -> Result<Self, Error>| {
			let result = f();
			match result {
				Err(ref result_error) => crate::error!("{result_error}"),
				Ok(ref sdk) => {
					crate::info!("Found SDK in {}", sdk.path().display())
				},
			}
			result
		};

		try_with(Self::try_from_default_env).or_else(|_| try_with(Self::try_from_default_config))
		                                    .or_else(|_| try_with(Self::try_from_default_path))
	}

	/// Create new `Sdk` with exact passed SDK path
	pub fn try_new_exact<P: Into<PathBuf>>(root: P) -> Result<Self, Error> {
		let path = root.into();
		let err = |p: &Path| {
			Error::new(
			           ErrorKind::InvalidInput,
			           format!("Invalid SDK path '{}'", p.display()),
			)
		};

		if path.exists() && path.is_dir() {
			let sdk = Self { path };
			if sdk.build_support().link_map().exists() {
				Ok(sdk)
			} else {
				Err(err(&sdk.path))
			}
		} else {
			Err(err(&path))
		}
	}

	/// Create new `Sdk` with default env var
	pub fn try_from_default_env() -> Result<Self, Error> {
		let sdk = std::env::var_os(SDK_ENV_VAR).map(PathBuf::from)
		                                       .map(Self::try_new_exact);
		sdk.ok_or(Error::new(ErrorKind::NotFound, format!("Missed env {SDK_ENV_VAR}")))?
	}

	/// Create new `Sdk` with default env var
	pub fn try_from_default_config() -> Result<Self, Error> {
		let cfg = config::Cfg::try_default()?;
		let path = cfg.sdk_path()
		              .ok_or(Error::new(ErrorKind::InvalidInput, "SDK path is not set"))?;
		Self::try_new_exact(path)
	}

	/// Create new `Sdk` with default env var
	pub fn try_from_default_path() -> Result<Self, Error> {
		#[cfg(unix)]
		const SDK_HOME_DIR: &'static str = "Developer";
		#[cfg(windows)]
		const SDK_HOME_DIR: &'static str = "Documents";

		let home = utils::home_dir()?;
		Self::try_new_exact(home.join(SDK_HOME_DIR).join("PlaydateSDK"))
	}
}


// Read:
impl Sdk {
	pub fn read_version(&self) -> Result<String, Error> {
		let value = std::fs::read_to_string(self.version_file())?;
		Ok(value)
	}
}


pub struct BuildSupport<'t> {
	path: Cow<'t, Path>,
}


impl<'t> BuildSupport<'t> {
	pub fn setup(&self) -> Cow<'t, Path> { self.path.join("setup.c").into() }
	pub fn link_map(&self) -> Cow<'t, Path> { self.path.join("link_map.ld").into() }
}


mod utils {
	use super::Error;
	use super::ErrorKind;
	use std::path::PathBuf;

	pub fn home_dir() -> Result<PathBuf, Error> {
		dirs::home_dir().ok_or(Error::new(ErrorKind::InvalidInput, "Could not find home dir"))
	}
}


mod config {
	use super::Error;
	use super::utils;
	use std::collections::HashMap;
	use std::path::PathBuf;
	use std::str::FromStr;

	const CFG_DIR: &'static str = ".Playdate";
	const CFG_FILENAME: &'static str = "config";
	const CFG_KEY_SDK_ROOT: &'static str = "SDKRoot";

	pub(super) struct Cfg(HashMap<String, String>);

	impl Cfg {
		pub fn try_default() -> Result<Self, Error> {
			let cfg_path = utils::home_dir()?.join(CFG_DIR).join(CFG_FILENAME);
			std::fs::read_to_string(cfg_path)?.parse()
		}

		pub fn sdk_path(&self) -> Option<PathBuf> { self.0.get(CFG_KEY_SDK_ROOT).map(PathBuf::from) }
	}

	impl FromStr for Cfg {
		type Err = Error;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			Ok(Self(
			        s.trim()
			         .lines()
			         .filter_map(|line| line.split_once("\t").map(|(k, v)| (k.to_owned(), v.to_owned())))
			         .collect(),
			))
		}
	}


	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn parse() {
			let path = "/path/PlaydateSDK-dir";
			let cfg: Cfg = format!("{k}\t{v}\n", k = CFG_KEY_SDK_ROOT, v = path).parse()
			                                                                    .unwrap();
			assert_eq!(cfg.sdk_path(), Some(PathBuf::from(path)));
		}
	}
}


// TODO: Move this tests to integration tests dir "tests" and run if sdk exists only.
#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn sdk() { Sdk::try_new().unwrap(); }
}
