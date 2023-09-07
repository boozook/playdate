use std::borrow::Cow;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use crate::compile::PDX_BIN_NAME_ELF;
use crate::compile::PDX_BIN_NAME_STEM;
use crate::compile::PDX_PKG_EXT;
use crate::compile::PDX_PKG_MANIFEST_FILENAME;
use crate::compile::dylib_suffix_for_host;
use crate::config::Env;


pub trait Layout {
	fn name(&self) -> &Name;


	/// The root directory: `/path/to/target/profile/playdate`.
	/// If per-target: `/path/to/target/$TRIPLE/profile/playdate`.
	/// If build-script: `/path/to/target(/$TRIPLE)/profile/build/**/out` ($OUT_DIR or `export-dir`).
	fn root(&self) -> &Path;

	/// The intermediate artifact destination: `$root/$NAME/`
	///
	/// Crate can have multiple _targets_ (e.g. bins, lib, examples),
	/// so we're have to specify the name,
	/// so `dest` will be `$root/$NAME/`,
	/// where `$NAME` is the name of the target.
	fn dest(&self) -> Cow<Path> { self.root().join(self.name().as_ref()).into() }

	/// Collected assets
	fn assets(&self) -> Cow<Path> { self.build().clone() }
	/// Hash of collected assets: `$dest/.assets.hash`
	fn assets_hash(&self) -> Cow<Path> { self.dest().join(".assets.hash").into() }
	fn assets_plan(&self) -> Cow<Path> { self.dest().join("plan.json").into() }

	/// The directory for build package: `$dest/build`
	///
	/// Directory with all files prepared to build with pdc.
	///
	/// Contains:
	/// - pdex.elf    : by cargo (+gcc) (link)
	/// - pdex.dylib  : by cargo (link)
	/// - pdxinfo     : manifest
	/// - * files     : linked assets
	fn build(&self) -> Cow<Path> { self.dest().join("build").into() }

	/// Playdate package manifest: `$build/pdxinfo`
	fn manifest(&self) -> Cow<Path> { self.build().join(PDX_PKG_MANIFEST_FILENAME).into() }

	/// Playdate (hw) executable: `$build/pdex.elf`
	fn binary(&self) -> Cow<Path> { self.build().join(PDX_BIN_NAME_ELF).into() }

	/// Playdate (sim) library: `$build/pdex.(dylib|dll)`
	///
	/// Type of library depends on the current (HOST) target.
	fn library(&self) -> Cow<Path> {
		self.build()
		    .join(PDX_BIN_NAME_STEM)
		    .with_extension(dylib_suffix_for_host())
		    .into()
	}


	/// The final package: `$root/$NAME.pdx`
	fn artifact(&self) -> Cow<Path> {
		self.root()
		    .join(self.name().as_ref())
		    .with_extension(PDX_PKG_EXT)
		    .into()
	}


	/// Create all directories.
	fn prepare(&mut self) -> std::io::Result<()> {
		use std::fs::create_dir_all;
		create_dir_all(self.root())?;
		create_dir_all(self.dest())?;
		create_dir_all(self.assets())?;
		create_dir_all(self.build())?;
		Ok(())
	}
}


/// Default layout, usually for build-script.
pub struct DefaultLayout {
	name: Name,
	root: PathBuf,
	dest: PathBuf,
	build: PathBuf,
}


impl DefaultLayout {
	pub fn new(name: Name, root: PathBuf) -> Self {
		let dest = root.join(name.as_path());
		let build = dest.join("build");
		Self { name,
		       root,
		       dest,
		       build }
	}
}


impl Layout for DefaultLayout {
	fn name(&self) -> &Name { &self.name }
	fn root(&self) -> &Path { self.root.as_path() }
	fn dest(&self) -> Cow<Path> { self.dest.as_path().into() }
	fn build(&self) -> Cow<Path> { self.build.as_path().into() }
}


#[derive(Clone)]
pub struct Name(OsString);


impl Name {
	/// `crate_name` is a name of target, such as name of lib, bin or example.
	pub fn with_names<S1, S2>(package_name: S1, crate_name: Option<S2>) -> Self
		where S1: Into<OsString>,
		      S1: PartialEq<S2>,
		      S2: Into<OsString> {
		let mut name: OsString = package_name.into();

		if let Some(crate_name) = crate_name.map(Into::into) {
			if name != crate_name {
				name.write_fmt(format_args!("-{}", crate_name.to_string_lossy()))
				    .unwrap();
			}
		}

		Name(name)
	}

	/// `crate_name` is a name of target, such as name of lib, bin or example.
	pub fn with_package<S>(package_name: S) -> Self
		where S: Into<OsString> {
		Name(package_name.into())
	}


	pub fn from_env(env: &Env) -> Self {
		const UNKNOWN_CARGO_PKG_NAME: &str = "unknown";
		let name = env.vars
		              .get("CARGO_BIN_NAME")
		              .or_else(|| env.vars.get("CARGO_CRATE_NAME"))
		              .or_else(|| env.vars.get("CARGO_PKG_NAME"))
		              .map(|s| s.as_str())
		              .unwrap_or(UNKNOWN_CARGO_PKG_NAME);
		Self(name.into())
	}
}


impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string_lossy())
	}
}

impl Debug for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.0) }
}


impl AsRef<OsStr> for Name {
	fn as_ref(&self) -> &OsStr { self.0.as_ref() }
}

impl Name {
	pub fn as_path(&self) -> &Path { Path::new(&self.0) }
}

impl Deref for Name {
	type Target = Path;
	fn deref(&self) -> &Self::Target { self.as_path() }
}
