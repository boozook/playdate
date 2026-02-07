use std::fmt::Display;
use anyhow::bail;


pub enum Cmd {
	Build,
	Run,

	/// Build Playdate package (pdx)
	Package,

	/// Prepare, collect assets for target
	Assets,
}

impl Cmd {
	#[allow(dead_code)]
	pub const ALL: &'static [Cmd] = &[Self::Build, Self::Run, Self::Package, Self::Assets];

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Build => "build",
			Self::Run => "run",
			Self::Package => "package",
			Self::Assets => "assets",
		}
	}
}

impl AsRef<str> for Cmd {
	fn as_ref(&self) -> &'static str { self.as_str() }
}

impl std::ops::Deref for Cmd {
	type Target = str;
	fn deref(&self) -> &'static Self::Target { self.as_str() }
}

impl TryFrom<&str> for Cmd {
	type Error = anyhow::Error;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let this = match s {
			"build" => Self::Build,
			"run" => Self::Run,

			"package" => Self::Package,
			"assets" => Self::Assets,

			other => bail!("Unknown command '{other}'."),
		};
		Ok(this)
	}
}

impl Display for Cmd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}
