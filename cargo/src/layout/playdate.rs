use std::borrow::Cow;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::hash::{Hash, Hasher};

use cargo::util::StableHasher;
use cargo_util::paths;
use cargo::CargoResult;
use cargo::core::Package;
use cargo::core::profiles::Profiles;
pub use playdate::layout::Name as TargetName;
use crate::config::Config;
use crate::layout::Layout;


/*** PER TARGET:
  target/$TRIPLE
  target/$TRIPLE/debug
  target/$TRIPLE/debug/deps/							: binaries (by toolchain)

	target/$TRIPLE/debug/playdate/	: root
		$NAME.assets/						: collected assets + hash files
		$NAME.build/						: build-package-dir with all files -> pdc
			pdex.elf							: by cargo (+gcc) (link)
			pdex.dylib						: by cargo (link)
			pdxinfo							: manifest
			*									: linked assets
		$NAME.pdx/							: package - result by pdc
	target/$TRIPLE/debug/$NAME.pdx	: zipped package (finally link to playdate/.pdx)

	link: debug/deps/$NAME.{bin|lib}	<- $NAME.build/{pdex.elf|pdex.dylib|..}	: linked binaries
	link: playdate/$NAME.assets|*		<- $NAME.build|*									: linked assets
	link: playdate/$NAME.pdx			<- target/$TRIPLE/debug/$NAME.pdx			: final package
*/


#[derive(Debug, Clone)]
pub struct ForTargetLayout<Path> {
	/// Target name (e.g. `game-example-name`)
	name: TargetName,

	/// The root directory: `/path/to/target/profile/playdate`.
	/// If per-target: `/path/to/target/$TRIPLE/profile/playdate`.
	/// If build-script: `/path/to/target(/$TRIPLE)/profile/build/**/out` ($OUT_DIR or `export-dir`).
	root: Path,
}

impl ForTargetLayout<PathBuf> {
	pub fn new<Main: Layout, S1, S2>(main: &Main, package_name: S1, crate_name: Option<S2>) -> Self
		where S1: Into<OsString>,
		      S2: Into<OsString>,
		      S1: PartialEq<S2> {
		let root = main.dest().join("playdate");
		let name = TargetName::with_names(package_name, crate_name);
		Self { name, root }
	}
}


#[derive(Debug, Clone)]
pub struct CrossTargetLayout<'cfg> {
	/// Target name (e.g. `game-example-name`)
	name: TargetName,
	package: &'cfg Package,

	/// The root directory: `/$target-dir/playdate`.
	root: PathBuf,
	/// `/$target-dir/playdate/$profile`.
	dest: PathBuf,
	/// `/$target-dir/playdate/$profile/$name/`.
	target: PathBuf,

	/// Cross-profile assets, `/$target-dir/playdate/assets/`.
	/// There is root for `PlaydateAssets`.
	assets: PathBuf,
}

impl<'cfg> CrossTargetLayout<'cfg> {
	pub fn new(config: &'cfg Config, package: &'cfg Package, name: Option<TargetName>) -> CargoResult<Self> {
		let profiles = Profiles::new(
		                             &config.workspace,
		                             config.compile_options.build_config.requested_profile,
		)?;
		let profile = profiles.get_dir_name().as_str();
		let name = name.unwrap_or_else(|| TargetName::with_package(package.name().as_str()));
		let root = config.workspace.target_dir().as_path_unlocked().join("playdate");
		let dest = root.join(profile);
		let target = dest.join(name.as_path());
		let assets = root.join("assets");
		Ok(Self { name,
		          package,
		          root,
		          dest,
		          target,
		          assets })
	}

	/// `/$target-dir/playdate/$profile/$name/`.
	pub fn target(&self) -> &Path { &self.target }

	/// Global assets layout for cross-target & cross-profile assets build.
	pub fn assets_layout(&self, config: &Config) -> PlaydateAssets<PathBuf> {
		PlaydateAssets::global(&self, config, self.package)
	}
}

impl Layout for CrossTargetLayout<'_> {
	fn root(&self) -> &Path { self.root.as_ref() }
	fn dest(&self) -> Cow<Path> { self.dest.as_path().into() }

	fn create_dest_dir(&mut self) -> anyhow::Result<()> {
		if !self.root.try_exists()? {
			paths::create_dir_all_excluded_from_backups_atomic(&self.root)?;
		}
		if !self.dest.try_exists()? {
			paths::create_dir_all(&self.dest)?;
		}
		Ok(())
	}

	fn prepare(&mut self) -> anyhow::Result<()> {
		self.create_dest_dir()?;
		paths::create_dir_all(&self.target)?;
		playdate::layout::Layout::prepare(self)?;
		Ok(())
	}

	fn clean(&mut self) -> anyhow::Result<()> { paths::remove_dir_all(self.dest().as_ref()) }
}


impl playdate::layout::Layout for CrossTargetLayout<'_> {
	fn name(&self) -> &TargetName { &self.name }
	fn root(&self) -> &Path { self.dest.as_path() }
	fn dest(&self) -> Cow<Path> { self.target.as_path().into() }
	fn assets(&self) -> Cow<Path> { self.target.join("assets").into() }
	fn assets_hash(&self) -> Cow<Path> { unimplemented!() }
	fn assets_plan(&self) -> Cow<Path> { unimplemented!() }

	fn build(&self) -> Cow<Path> { self.target.join("build").into() }

	fn manifest(&self) -> Cow<Path> {
		self.build()
		    .join(playdate::manifest::PDX_PKG_MANIFEST_FILENAME)
		    .into()
	}

	fn prepare(&mut self) -> std::io::Result<()> {
		use std::fs::create_dir_all;
		create_dir_all(self.assets())?;
		create_dir_all(self.build())?;
		Ok(())
	}
}

impl crate::layout::LayoutLockable for CrossTargetLayout<'_> {
	/// The lockfile filename for a build
	fn lockfilename(&self) -> Cow<'static, str> { ".playdate-lock".into() }
}


#[derive(Debug, Clone)]
pub struct PlaydateAssets<Path> {
	/// Target name (e.g. `game-example-name`)
	name: TargetName,

	/// The root directory: `/path/to/target/playdate.assets`.
	root: Path,
}

impl PlaydateAssets<PathBuf> {
	fn global(root: &CrossTargetLayout<'_>, config: &Config, package: &Package) -> Self {
		let name = Self::name_for_package(config, package);
		Self { name,
		       root: root.assets.to_owned() }
	}

	pub fn assets_plan_for(&self, config: &Config, package: &Package) -> PathBuf {
		use playdate::layout::Layout;
		let name = Self::name_for_package(config, package);
		self.assets_plan().with_file_name(name).with_extension("json")
	}

	pub fn assets_plan_for_dev(&self, config: &Config, package: &Package) -> PathBuf {
		let mut path = self.assets_plan_for(config, package);
		let mut name = String::new();
		path.file_stem().map(|stem| {
			                name.push_str(stem.to_string_lossy().as_ref());
		                });
		name.push_str("-dev");
		path.extension().map(|ext| {
			                name.push_str(".");
			                name.push_str(ext.to_string_lossy().as_ref());
		                });
		path.set_file_name(name);
		path
	}

	fn name_for_package(config: &Config, package: &Package) -> TargetName {
		let mut hasher = StableHasher::new();
		let stable = package.package_id().stable_hash(config.workspace.root());
		stable.hash(&mut hasher);
		let hash = hasher.finish();
		TargetName::with_package(format!("{}-{hash:016x}", package.name()))
	}
}


mod support {
	use std::borrow::Cow;
	use std::io::Error;
	use std::io::ErrorKind;
	use std::path::Path;
	use cargo_util::paths;
	pub use playdate::layout::Layout;
	pub use playdate::layout::Name;
	use super::ForTargetLayout;
	use super::PlaydateAssets;

	impl<P: AsRef<Path>> Layout for ForTargetLayout<P> {
		fn name(&self) -> &Name { &self.name }
		fn root(&self) -> &Path { self.root.as_ref() }

		fn assets(&self) -> Cow<Path> { unimplemented!() }

		fn prepare(&mut self) -> std::io::Result<()> {
			use std::fs::create_dir_all;

			if !self.root().try_exists()? {
				paths::create_dir_all_excluded_from_backups_atomic(&self.root).map_err(|err| {
					                                                              Error::new(ErrorKind::Other, err)
				                                                              })?;
			}
			create_dir_all(self.dest())?;
			// create_dir_all(self.assets())?;
			create_dir_all(self.build())?;
			Ok(())
		}
	}

	impl<P: AsRef<Path>> Layout for PlaydateAssets<P> {
		fn name(&self) -> &Name { &self.name }
		fn root(&self) -> &Path { self.root.as_ref() }

		/// cargo-target-dir/playdate.assets/$name/assets/
		fn assets(&self) -> Cow<Path> { self.dest().join("assets").into() }

		fn prepare(&mut self) -> std::io::Result<()> {
			use std::fs::create_dir_all;

			if !self.root().try_exists()? {
				paths::create_dir_all_excluded_from_backups_atomic(&self.root).map_err(|err| {
					                                                              Error::new(ErrorKind::Other, err)
				                                                              })?;
			}
			create_dir_all(self.dest())?;
			create_dir_all(self.assets())?;
			create_dir_all(self.build())?;
			Ok(())
		}
	}

	impl<P: AsRef<Path>> PlaydateAssets<P> {
		fn dev(&self) -> Cow<Path> { self.assets().parent().unwrap().join("dev").into() }

		/// cargo-target-dir/playdate.assets/$name/dev/assets/
		pub fn assets_dev(&self) -> Cow<Path> { self.dev().join(self.assets().file_name().unwrap()).into() }

		/// cargo-target-dir/playdate.assets/$name/dev/build/
		pub fn build_dev(&self) -> Cow<Path> { self.dev().join(self.build().file_name().unwrap()).into() }
	}
}


impl<P: AsRef<Path>> Layout for ForTargetLayout<P> {
	fn root(&self) -> &Path { self.root.as_ref() }
	fn dest(&self) -> Cow<Path> { <Self as support::Layout>::dest(self) }

	fn create_dest_dir(&mut self) -> anyhow::Result<()> {
		if !self.root.as_ref().try_exists()? {
			std::fs::create_dir_all(&self.root)?;
		}
		if !self.dest().try_exists()? {
			std::fs::create_dir_all(self.dest())?;
		}
		Ok(())
	}

	fn prepare(&mut self) -> anyhow::Result<()> {
		<Self as support::Layout>::prepare(self)?;
		Ok(())
	}
}


impl<P: AsRef<Path>> Layout for PlaydateAssets<P> {
	fn root(&self) -> &Path { self.root.as_ref() }
	fn dest(&self) -> Cow<Path> { <Self as support::Layout>::dest(self) }

	fn create_dest_dir(&mut self) -> anyhow::Result<()> {
		if !self.root.as_ref().try_exists()? {
			std::fs::create_dir_all(&self.root)?;
		}
		if !self.dest().try_exists()? {
			std::fs::create_dir_all(self.dest())?;
		}
		Ok(())
	}

	fn prepare(&mut self) -> anyhow::Result<()> {
		<Self as support::Layout>::prepare(self)?;
		Ok(())
	}

	fn clean(&mut self) -> anyhow::Result<()> {
		use playdate::layout::Layout;

		if self.assets().try_exists()? {
			paths::remove_dir_all(self.assets())?;
		}
		if self.build().try_exists()? {
			paths::remove_dir_all(self.build())?;
		}
		if self.assets_plan().try_exists()? {
			paths::remove_file(self.assets_plan())?;
		}
		if self.assets_hash().try_exists()? {
			paths::remove_file(self.assets_hash())?;
		}

		Ok(())
	}
}


const PD_LOCKFILE_NAME: &str = ".playdate-lock";


impl<P: AsRef<Path>> crate::layout::LayoutLockable for ForTargetLayout<P> {
	/// The lockfile filename for a build
	fn lockfilename(&self) -> Cow<'static, str> { PD_LOCKFILE_NAME.into() }
}


impl<P: AsRef<Path>> crate::layout::LayoutLockable for PlaydateAssets<P> {
	/// The lockfile filename
	fn lockfilename(&self) -> Cow<'static, str> { PD_LOCKFILE_NAME.into() }
}
