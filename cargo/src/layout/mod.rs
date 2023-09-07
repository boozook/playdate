use std::borrow::Cow;
use std::path::Path;

mod cargo;
mod playdate;

use ::cargo::CargoResult;
use ::cargo::Config;
use ::cargo::util::FileLock;
use ::cargo::util::Filesystem;

pub use self::cargo::*;
pub use self::playdate::*;


pub trait Layout {
	/// The root path
	fn root(&self) -> &Path;

	/// The destination path for final artifacts
	fn dest(&self) -> Cow<Path>;
	fn create_dest_dir(&mut self) -> anyhow::Result<()>;

	/// Makes sure all directories stored in the Layout exist on the filesystem.
	fn prepare(&mut self) -> anyhow::Result<()>;

	fn clean(&mut self) -> anyhow::Result<()> { unimplemented!() }
}

impl<T: Layout> Layout for &mut T {
	fn root(&self) -> &Path { <T as Layout>::root(&self) }
	fn dest(&self) -> Cow<Path> { <T as Layout>::dest(&self) }
	fn create_dest_dir(&mut self) -> anyhow::Result<()> { <T as Layout>::create_dest_dir(self) }
	fn prepare(&mut self) -> anyhow::Result<()> { <T as Layout>::prepare(self) }
}


pub trait LayoutLockable: Layout {
	/// The lockfile filename for a build (e.g: `.crank-lock`).
	fn lockfilename(&self) -> Cow<str>;

	/// Lock the destination directory of the layout.
	/// This function will block if the directory is already locked.
	fn lock(self, cfg: &Config) -> anyhow::Result<LayoutLock<Self>>
		where Self: Sized {
		LayoutLock::new(self, cfg)
	}

	fn lock_mut(&mut self, cfg: &Config) -> anyhow::Result<LayoutLock<&mut Self>>
		where Self: Sized {
		LayoutLock::new(self, cfg)
	}
}

impl<T: LayoutLockable> LayoutLockable for &mut T where T: Layout {
	fn lockfilename(&self) -> Cow<str> { <T as LayoutLockable>::lockfilename(self) }
}


/// Mimics to cargo's [`Layout`](cargo::core::compiler::Layout) with custom lock.
/// Contains the paths of all target output locations.
pub struct LayoutLock<T> {
	layout: T,

	/// The lockfile. Will be unlocked when this struct is `drop`ped.
	_lock: FileLock,
}


impl<T: LayoutLockable> LayoutLock<T> {
	pub fn new(mut layout: T, cfg: &Config) -> CargoResult<LayoutLock<T>> {
		layout.create_dest_dir()?;

		let dest = Filesystem::new(layout.dest().to_path_buf());
		// For now we don't do any more finer-grained locking on the artifact
		// directory, so just lock the entire thing for the duration of this
		// compile.
		let lock = dest.open_rw(layout.lockfilename().as_ref(), cfg, "build directory")?;

		Ok(LayoutLock { layout, _lock: lock })
	}
}

impl<T: Layout> LayoutLock<T> {
	#[allow(dead_code)]
	pub fn unlock(self) -> T { self.layout }
	pub fn as_inner(&self) -> &T { &self.layout }
}


impl<T: Layout> AsRef<T> for LayoutLock<T> {
	fn as_ref(&self) -> &T { &self.layout }
}

impl<T: Layout> AsMut<T> for LayoutLock<T> {
	fn as_mut(&mut self) -> &mut T { &mut self.layout }
}

impl<T: Layout> Layout for LayoutLock<T> {
	fn root(&self) -> &Path { self.layout.root() }
	fn dest(&self) -> Cow<Path> { self.layout.dest() }
	fn create_dest_dir(&mut self) -> anyhow::Result<()> { self.layout.create_dest_dir() }
	fn prepare(&mut self) -> anyhow::Result<()> { self.layout.prepare() }
	fn clean(&mut self) -> anyhow::Result<()> { self.layout.clean() }
}


mod support {
	use super::LayoutLock;
	pub use playdate::layout::Layout as PlaydateLayout;

	impl<T: PlaydateLayout> PlaydateLayout for LayoutLock<T> {
		fn name(&self) -> &playdate::layout::Name { PlaydateLayout::name(&self.layout) }
		fn root(&self) -> &std::path::Path { PlaydateLayout::root(&self.layout) }
	}
}
