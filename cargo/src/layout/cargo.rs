use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

use cargo::CargoResult;
use cargo::core::Workspace;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CompileTarget;
use cargo::core::profiles::Profiles;
use cargo_util::paths;

use crate::config::Config;
use super::Layout;
use super::LayoutLockable;


/// Mimics to cargo's [`Layout`](cargo::core::compiler::Layout) but without lock.
/// Contains the paths of all target output locations.
#[allow(dead_code)]
pub struct CargoLayout {
	/// The root directory: `/path/to/target`.
	/// If cross compiling: `/path/to/target/$TRIPLE`.
	root: PathBuf,
	/// The final artifact destination: `$root/debug` (or `release`).
	dest: PathBuf,
	/// The directory with rustc artifacts: `$dest/deps`
	deps: PathBuf,
	/// The directory for build scripts: `$dest/build`
	build: PathBuf,
	/// The directory for artifacts, i.e. binaries, cdylibs, staticlibs: `$dest/deps/artifact`
	artifact: PathBuf,
	/// The directory for incremental files: `$dest/incremental`
	incremental: PathBuf,
	/// The directory for fingerprints: `$dest/.fingerprint`
	fingerprint: PathBuf,
	/// The directory for examples: `$dest/examples`
	examples: PathBuf,
	/// The directory for rustdoc output: `$root/doc`
	doc: PathBuf,
	/// The directory for temporary data of integration tests and benches: `$dest/tmp`
	tmp: PathBuf,
}

impl AsRef<CargoLayout> for &'_ CargoLayout {
	fn as_ref(&self) -> &CargoLayout { *self }
}

impl CargoLayout {
	/// Calculate the paths for build output and return as a Layout.
	///
	/// `dest` should be the final artifact directory name. Currently either
	/// "debug" or "release".
	pub fn new(ws: &Workspace<'_>, target: Option<CompileTarget>, dest: &str) -> CargoResult<CargoLayout> {
		let mut root = ws.target_dir();
		if let Some(target) = target {
			root.push(target.short_name());
		}
		let dest = root.join(dest);
		let root = root.into_path_unlocked();
		let dest = dest.into_path_unlocked();
		let deps = dest.join("deps");
		let artifact = deps.join("artifact");

		Ok(CargoLayout { deps,
		                 build: dest.join("build"),
		                 artifact,
		                 incremental: dest.join("incremental"),
		                 fingerprint: dest.join(".fingerprint"),
		                 examples: dest.join("examples"),
		                 doc: root.join("doc"),
		                 tmp: root.join("tmp"),
		                 root,
		                 dest })
	}
}

impl Layout for CargoLayout {
	fn root(&self) -> &Path { &self.root }
	fn dest(&self) -> Cow<Path> { self.dest.as_path().into() }

	fn create_dest_dir(&mut self) -> anyhow::Result<()> {
		if !self.root.try_exists()? {
			// If the root directory doesn't already exist go ahead and create it
			// here. Use this opportunity to exclude it from backups as well if the
			// system supports it since this is a freshly created folder.
			paths::create_dir_all_excluded_from_backups_atomic(&self.root)?;
		}
		if !self.dest.try_exists()? {
			// Now that the excluded from backups target root is created we can create the
			// actual destination (sub)subdirectory.
			paths::create_dir_all(&self.dest)?;
		}
		Ok(())
	}

	fn prepare(&mut self) -> anyhow::Result<()> {
		paths::create_dir_all(&self.deps)?;
		paths::create_dir_all(&self.incremental)?;
		paths::create_dir_all(&self.fingerprint)?;
		paths::create_dir_all(&self.examples)?;
		paths::create_dir_all(&self.build)?;
		Ok(())
	}
}

impl LayoutLockable for CargoLayout {
	/// The lockfile filename for a build (`.cargo-lock`).
	fn lockfilename(&self) -> Cow<'static, str> { ".cargo-lock".into() }
}

impl<'cfg> Config<'cfg> {
	pub fn layout(&self, target: Option<CompileTarget>) -> anyhow::Result<CargoLayout> {
		let profiles = Profiles::new(
		                             &self.workspace,
		                             self.compile_options.build_config.requested_profile,
		)?;
		let layout = CargoLayout::new(&self.workspace, target, profiles.get_dir_name().as_str())?;
		if let Some(export_dir) = self.compile_options.build_config.export_dir.as_deref() {
			debug_assert_eq!(layout.dest(), export_dir)
		}
		Ok(layout)
	}

	pub fn layout_for(&self, kind: CompileKind) -> anyhow::Result<CargoLayout> {
		let target = match kind {
			CompileKind::Host => None,
			CompileKind::Target(target) => Some(target),
		};
		self.layout(target)
	}
}
