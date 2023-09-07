use std::path::Path;

use crate::config::Config;


pub trait AsRelativeTo {
	fn as_relative_to<P: AsRef<Path>>(&self, other: &P) -> &Path;
	fn as_relative_to_root(&self, config: &Config) -> &Path;
}

impl<T> AsRelativeTo for T where T: AsRef<Path> {
	fn as_relative_to<P: AsRef<Path>>(&self, root: &P) -> &Path {
		self.as_ref().strip_prefix(root.as_ref()).unwrap_or(self.as_ref())
	}

	fn as_relative_to_root(&self, config: &Config) -> &Path { self.as_relative_to(&config.workspace.root()) }
}
