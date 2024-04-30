use std::borrow::Cow;
use std::convert::Infallible;
use std::str::FromStr;

use clap::ValueEnum;
use clap::builder::PossibleValue;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency<'t> {
	pub name: DependencyName<'t>,
	pub source: DependencySource,
}

impl Dependency<'_> {
	pub const fn git(&self) -> Option<&'static str> {
		const GIT: &str = env!("CARGO_PKG_REPOSITORY");
		match self.source {
			DependencySource::CratesIo => None,
			DependencySource::Git => {
				match self.name {
					DependencyName::Sys => Some(GIT),
					DependencyName::System => Some(GIT),
					DependencyName::Menu => Some(GIT),
					DependencyName::Controls => Some(GIT),
					DependencyName::Fs => Some(GIT),
					DependencyName::Sound => Some(GIT),
					DependencyName::Display => Some(GIT),
					DependencyName::Graphics => Some(GIT),
					DependencyName::Sprite => Some(GIT),
					DependencyName::Color => Some(GIT),
					DependencyName::Playdate => Some(GIT),
					DependencyName::Other(_) => None,
				}
			},
		}
	}
}

impl ValueEnum for Dependency<'_> {
	fn value_variants<'a>() -> &'a [Self] {
		use DependencyName as Name;
		use DependencySource as Src;

		#[rustfmt::skip]
		let res = &[
		            Self { name: Name::Playdate, source: Src::CratesIo, },
		            Self { name: Name::Playdate, source: Src::Git, },

						Self { name: Name::Sys, source: Src::CratesIo, },
		            Self { name: Name::Sys, source: Src::Git, },

						Self { name: Name::System, source: Src::CratesIo, },
						Self { name: Name::System, source: Src::Git, },

						Self { name: Name::Controls, source: Src::CratesIo, },
		            Self { name: Name::Controls, source: Src::Git, },

						Self { name: Name::Menu, source: Src::CratesIo, },
						Self { name: Name::Menu, source: Src::Git, },

						Self { name: Name::Fs, source: Src::CratesIo, },
						Self { name: Name::Fs, source: Src::Git, },

						Self { name: Name::Sound, source: Src::CratesIo, },
						Self { name: Name::Sound, source: Src::Git, },

						Self { name: Name::Display, source: Src::CratesIo, },
						Self { name: Name::Display, source: Src::Git, },

						Self { name: Name::Graphics, source: Src::CratesIo, },
						Self { name: Name::Graphics, source: Src::Git, },

						Self { name: Name::Sprite, source: Src::CratesIo, },
						Self { name: Name::Sprite, source: Src::Git, },

						Self { name: Name::Color, source: Src::CratesIo, },
						Self { name: Name::Color, source: Src::Git, },

		            Self { name: Name::Other(Cow::Borrowed("any-other")), source: Src::CratesIo, },
		];

		#[cfg(debug_assertions)]
		{
			let missed: Vec<_> = DependencyName::value_variants().iter()
			                                                     .filter(|name| !res.iter().any(|dep| dep.name == **name))
			                                                     .collect();
			debug_assert_eq!(0, missed.len(), "Missing dependencies: {:?}", missed);
		}

		res
	}

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		use DependencyName as Name;
		use DependencySource as Src;

		match (&self.name, &self.source) {
			(Name::Other(s), Src::CratesIo) => {
				PossibleValue::new(s.as_ref().to_owned()).help("Any other package (crates.io only)")
				                                         .into()
			},
			(Name::Other(_), _) => None,

			(name, Src::CratesIo) => {
				PossibleValue::new(name.to_string()).aliases(name.aliases())
				                                    .help(name.description().to_string())
				                                    .into()
			},
			(name, Src::Git) => {
				let help = format!("{} (git)", name.description());
				PossibleValue::new(name.to_string()).aliases(name.aliases())
				                                    .help(help)
				                                    .into()
			},
		}
	}
}


impl FromStr for Dependency<'_> {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (name, source) = if let Some((name, source)) = s.trim().split_once(':') {
			let name = DependencyName::from_str(name)?;
			let source = DependencySource::from_str(source)?;
			(name, source)
		} else {
			let name = DependencyName::from_str(s)?;
			let source = DependencySource::default();
			(name, source)
		};

		Ok(Self { name, source })
	}
}

impl std::fmt::Display for Dependency<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.name, self.source)
	}
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencyName<'t> {
	Color,
	Controls,
	Display,
	Fs,
	Graphics,
	Menu,
	Sound,
	Sprite,
	Sys,
	System,
	Playdate,
	Other(Cow<'t, str>),
}

impl<'t> DependencyName<'t> {
	pub fn as_str(&self) -> Cow<'t, str> {
		match self {
			DependencyName::Sys => "playdate-sys".into(),
			DependencyName::System => "playdate-system".into(),
			DependencyName::Controls => "playdate-controls".into(),
			DependencyName::Menu => "playdate-menu".into(),
			DependencyName::Fs => "playdate-fs".into(),
			DependencyName::Sound => "playdate-sound".into(),
			DependencyName::Display => "playdate-display".into(),
			DependencyName::Graphics => "playdate-graphics".into(),
			DependencyName::Sprite => "playdate-sprite".into(),
			DependencyName::Color => "playdate-color".into(),
			DependencyName::Playdate => "playdate".into(),
			DependencyName::Other(s) => s.clone(),
		}
	}

	pub fn description(&self) -> Cow<'t, str> {
		match self {
			DependencyName::Sys => "Low-level Playdate API".into(),
			DependencyName::System => "Playdate system API".into(),
			DependencyName::Controls => "Playdate controls API".into(),
			DependencyName::Menu => "Playdate menu API".into(),
			DependencyName::Fs => "Playdate file-system API".into(),
			DependencyName::Sound => "Playdate sound API".into(),
			DependencyName::Display => "Playdate display API".into(),
			DependencyName::Graphics => "Playdate graphics API".into(),
			DependencyName::Sprite => "Playdate sprite API".into(),
			DependencyName::Color => "Playdate color API".into(),
			DependencyName::Playdate => "High-level Playdate API".into(),
			DependencyName::Other(s) => s.clone(),
		}
	}

	pub fn aliases(&self) -> impl Iterator<Item = &'static str> {
		match self {
			DependencyName::Sys => &["sys"][..],
			DependencyName::System => &["system"],
			DependencyName::Controls => &["controls", "ctrl"],
			DependencyName::Menu => &["menu"],
			DependencyName::Fs => &["fs"],
			DependencyName::Sound => &["sound"],
			DependencyName::Display => &["display"],
			DependencyName::Graphics => &["graphics"],
			DependencyName::Sprite => &["sprite"],
			DependencyName::Color => &["color"],
			DependencyName::Playdate => &["pd"],
			DependencyName::Other(_) => &[],
		}.iter()
		.copied()
	}
}

impl std::fmt::Display for DependencyName<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = self.as_str();
		write!(f, "{s}")
	}
}

impl FromStr for DependencyName<'_> {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use DependencyName::*;

		let this = match s.trim().to_lowercase().as_str() {
			"" => Sys, // default empty case
			n if n == Sys.as_str() || Sys.aliases().any(|a| a == n) => Sys,
			n if n == System.as_str() || System.aliases().any(|a| a == n) => System,
			n if n == Controls.as_str() || Controls.aliases().any(|a| a == n) => Controls,
			n if n == Menu.as_str() || Menu.aliases().any(|a| a == n) => Menu,
			n if n == Fs.as_str() || Fs.aliases().any(|a| a == n) => Fs,
			n if n == Sound.as_str() || Sound.aliases().any(|a| a == n) => Sound,
			n if n == Graphics.as_str() || Graphics.aliases().any(|a| a == n) => Graphics,
			n if n == Color.as_str() || Color.aliases().any(|a| a == n) => Color,
			n if n == Playdate.as_str() || Playdate.aliases().any(|a| a == n) => Playdate,
			other => Other(other.to_owned().into()),
		};
		Ok(this)
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DependencySource {
	CratesIo,
	Git,
}

impl std::fmt::Display for DependencySource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			DependencySource::CratesIo => "crates",
			DependencySource::Git => "git",
		};
		write!(f, "{s}")
	}
}

impl FromStr for DependencySource {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.trim().to_lowercase().as_str() {
			"crates" | "" => Ok(DependencySource::CratesIo),
			"git" => Ok(DependencySource::Git),
			err => Err(anyhow::anyhow!("Invalid dependency source: {err}")),
		}
	}
}

impl Default for DependencySource {
	fn default() -> Self { Self::CratesIo }
}


impl DependencyName<'_> {
	#![allow(dead_code)]

	pub fn value_variants() -> &'static [Self] {
		use DependencyName as Name;
		static ALL: &[Name] = &[
		                        Name::Sys,
		                        Name::System,
		                        Name::Controls,
		                        Name::Menu,
		                        Name::Fs,
		                        Name::Sound,
		                        Name::Graphics,
		                        Name::Color,
		                        Name::Playdate,
		                        Name::Other(Cow::Borrowed("any-other")),
		];
		ALL
	}

	pub fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::Other(s) => {
				PossibleValue::new(s.as_ref().to_owned()).help("Any other package")
				                                         .into()
			},
			name => {
				PossibleValue::new(name.to_string()).aliases(name.aliases())
				                                    .help(name.description().to_string())
				                                    .into()
			},
		}
	}
}


impl DependencySource {
	#![allow(dead_code)]

	pub fn value_variants<'a>() -> &'a [Self] { &[Self::CratesIo, Self::Git] }

	pub fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::CratesIo => PossibleValue::new("crates").help("crates.io").into(),
			Self::Git => PossibleValue::new("git").help("Only for known sources.").into(),
		}
	}
}
