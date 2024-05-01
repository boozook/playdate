use std::ffi::OsStr;
use std::fmt::Display;

use anyhow::bail;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CompileTarget;
use playdate::consts::DEVICE_TARGET;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaydateTarget {
	Device,
	Simulator,
}

impl PlaydateTarget {
	pub const ALL: &'static [Self] = &[Self::Device, Self::Simulator];

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Device => "device",
			Self::Simulator => "simulator",
		}
	}
}

impl AsRef<str> for PlaydateTarget {
	fn as_ref(&self) -> &'static str { self.as_str() }
}

impl std::ops::Deref for PlaydateTarget {
	type Target = str;
	fn deref(&self) -> &'static Self::Target { self.as_str() }
}

impl TryFrom<&str> for PlaydateTarget {
	type Error = anyhow::Error;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"device" | "playdate" => Ok(Self::Device),
			"simulator" => Ok(Self::Simulator),
			other => bail!("Unknown Playdate target '{other}'."),
		}
	}
}

impl TryFrom<&OsStr> for PlaydateTarget {
	type Error = anyhow::Error;
	fn try_from(s: &OsStr) -> Result<Self, Self::Error> {
		match s.to_string_lossy().as_ref() {
			"device" | "playdate" => Ok(Self::Device),
			"simulator" => Ok(Self::Simulator),
			other => bail!("Unknown Playdate target '{other}'."),
		}
	}
}

impl Display for PlaydateTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}

impl From<&'_ PlaydateTarget> for CompileKind {
	fn from(value: &'_ PlaydateTarget) -> Self {
		match value {
			PlaydateTarget::Device => CompileKind::Target(CompileTarget::new(DEVICE_TARGET).unwrap_or_else(|_| panic!("Target {DEVICE_TARGET} must be valid CompileTarget"))),
			PlaydateTarget::Simulator => CompileKind::Host,
		}
	}
}

impl From<PlaydateTarget> for CompileKind {
	fn from(value: PlaydateTarget) -> Self { (&value).into() }
}


impl PlaydateTarget {
	pub fn device_compile_target() -> CompileTarget {
		CompileTarget::new(DEVICE_TARGET).expect(&format!("Target {DEVICE_TARGET} must be valid CompileTarget"))
	}
}


impl TryFrom<&CompileKind> for PlaydateTarget {
	type Error = anyhow::Error;

	fn try_from(kind: &CompileKind) -> Result<Self, Self::Error> {
		match kind {
			CompileKind::Host => Ok(Self::Simulator),
			CompileKind::Target(target) => {
				match target.rustc_target().as_str() {
					"device" | "playdate" => Ok(Self::Device),
					"simulator" => Ok(Self::Simulator),
					other => bail!("Unknown Playdate target '{other}'."),
				}
			},
		}
	}
}

impl TryFrom<&mut CompileKind> for PlaydateTarget {
	type Error = anyhow::Error;

	fn try_from(kind: &mut CompileKind) -> Result<Self, Self::Error> {
		match kind {
			CompileKind::Host => Ok(Self::Simulator),
			CompileKind::Target(target) => {
				match target.rustc_target().as_str() {
					"device" | "playdate" => Ok(Self::Device),
					"simulator" => Ok(Self::Simulator),
					other => bail!("Unknown Playdate target '{other}'."),
				}
			},
		}
	}
}


impl PartialEq<CompileKind> for PlaydateTarget {
	fn eq(&self, other: &CompileKind) -> bool {
		match (self, other) {
			(PlaydateTarget::Simulator, CompileKind::Host) => true,
			(PlaydateTarget::Device, CompileKind::Target(target)) => {
				target.rustc_target() == DEVICE_TARGET ||
				Self::try_from(target.rustc_target().as_str()).ok()
				                                              .filter(|target| target == &Self::Device)
				                                              .is_some()
			},
			_ => false,
		}
	}
}


pub trait NeedToReplace {
	fn need_to_replace(&self) -> bool;
}

impl NeedToReplace for CompileKind {
	fn need_to_replace(&self) -> bool {
		match self {
			CompileKind::Host => false,
			CompileKind::Target(target) => {
				match target.rustc_target().as_str() {
					"device" | "playdate" | "simulator" => true,
					_ => false,
				}
			},
		}
	}
}
