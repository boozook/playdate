use std::fmt::Display;
use anyhow::bail;


pub enum Cmd {
	Build,
	Run,

	New,
	Init,
	Migrate,

	/// Build Playdate package (pdx)
	Package,

	/// Prepare, collect assets for target
	Assets,

	/// Publish Playdate package
	Publish,
}

impl Cmd {
	#[allow(dead_code)]
	pub const ALL: &[Cmd] = &[
	                          Self::Build,
	                          Self::Run,
	                          Self::New,
	                          Self::Init,
	                          Self::Migrate,
	                          Self::Package,
	                          Self::Assets,
	                          Self::Publish,
	];

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Build => "build",
			Self::Run => "run",

			Self::New => "new",
			Self::Init => "init",

			Self::Migrate => "migrate",
			Self::Package => "package",
			Self::Assets => "assets",
			Self::Publish => "publish",
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

			"new" => Self::New,
			"init" => Self::Init,

			"migrate" => Self::Migrate,
			"package" => Self::Package,
			"assets" => Self::Assets,
			"publish" => Self::Publish,

			other => bail!("Unknown command '{other}'."),
		};
		Ok(this)
	}
}

impl Display for Cmd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}
