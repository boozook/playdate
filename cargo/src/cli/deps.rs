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
		match self.source {
			DependencySource::CratesIo => None,
			DependencySource::Git => {
				match self.name {
					DependencyName::Sys => Some("https://github.com/boozook/playdate.git"),
					DependencyName::Playdate => Some("https://github.com/boozook/playdate.git"),
					DependencyName::Controls => Some("https://github.com/boozook/playdate.git"),
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

		&[
		  Self { name: Name::Playdate,
		         source: Src::CratesIo, },
		  Self { name: Name::Playdate,
		         source: Src::Git, },
		  Self { name: Name::Sys,
		         source: Src::CratesIo, },
		  Self { name: Name::Sys,
		         source: Src::Git, },
		  Self { name: Name::Controls,
		         source: Src::CratesIo, },
		  Self { name: Name::Controls,
		         source: Src::Git, },
		  Self { name: Name::Other(Cow::Borrowed("any-other")),
		         source: Src::CratesIo, },
		]
	}

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		use DependencyName as Name;
		use DependencySource as Src;

		match (&self.name, &self.source) {
			(Name::Sys, Src::CratesIo) => {
				PossibleValue::new("playdate-sys").alias("sys")
				                                  .help("Low-level Playdate API")
				                                  .into()
			},
			(Name::Sys, Src::Git) => {
				PossibleValue::new("playdate-sys:git").alias("sys:git")
				                                      .help("Low-level Playdate API (git)")
				                                      .into()
			},
			(Name::Playdate, Src::CratesIo) => PossibleValue::new("playdate").help("Playdate API").into(),
			(Name::Playdate, Src::Git) => {
				PossibleValue::new("playdate:git").help("Playdate API (git)")
				                                  .into()
			},
			(Name::Controls, Src::CratesIo) => {
				PossibleValue::new("playdate-controls").alias("controls")
				                                       .help("Playdate Controls API")
				                                       .into()
			},
			(Name::Controls, Src::Git) => {
				PossibleValue::new("playdate-controls:git").alias("controls:git")
				                                           .help("Playdate Controls API (git)")
				                                           .into()
			},
			(Name::Other(s), Src::CratesIo) => {
				PossibleValue::new(s.as_ref().to_owned()).help("Any other package (crates.io only)")
				                                         .into()
			},
			(Name::Other(_), _) => None,
		}
	}
}


impl FromStr for Dependency<'_> {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (name, source) = if let Some((name, source)) = s.trim().split_once(":") {
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
	Sys,
	Playdate,
	Controls,
	// Other(String),
	Other(Cow<'t, str>),
}

impl std::fmt::Display for DependencyName<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			DependencyName::Sys => "playdate-sys",
			DependencyName::Playdate => "playdate",
			DependencyName::Controls => "playdate-controls",
			DependencyName::Other(s) => s,
		};
		write!(f, "{s}")
	}
}

impl FromStr for DependencyName<'_> {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let this = match s.trim().to_lowercase().as_str() {
			"playdate" => DependencyName::Playdate,
			"playdate-sys" | "sys" | "" => DependencyName::Sys,
			"playdate-controls" | "controls" => DependencyName::Controls,
			other => DependencyName::Other(other.to_owned().into()),
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
		                        Name::Playdate,
		                        Name::Controls,
		                        Name::Other(Cow::Borrowed("any-other")),
		];
		ALL
	}

	pub fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::Sys => {
				PossibleValue::new("playdate-sys").help("Low-level Playdate API")
				                                  .into()
			},
			Self::Playdate => PossibleValue::new("playdate").help("Playdate API").into(),
			Self::Controls => {
				PossibleValue::new("playdate-controls").help("Playdate Controls API")
				                                       .into()
			},
			Self::Other(s) => {
				PossibleValue::new(s.as_ref().to_owned()).help("Any other package")
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
