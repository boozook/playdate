use std::fmt::Display;
use anyhow::bail;
use clap::ValueEnum;
use clap::builder::PossibleValue;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ide {
	None,
	Vscode,
}

impl Default for Ide {
	fn default() -> Self { Self::None }
}


impl Ide {
	#[allow(dead_code)]
	pub const ALL: &'static [Ide] = &[Self::None, Self::Vscode];

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::None => "none",
			Self::Vscode => "vscode",
		}
	}
}

impl AsRef<str> for Ide {
	fn as_ref(&self) -> &'static str { self.as_str() }
}

impl std::ops::Deref for Ide {
	type Target = str;
	fn deref(&self) -> &'static Self::Target { self.as_str() }
}

impl TryFrom<&str> for Ide {
	type Error = anyhow::Error;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let this = match s {
			"none" | "no" | "false" => Self::None,
			"vsc" | "vscode" => Self::Vscode,

			other => bail!("Unknown IDE '{other}'."),
		};
		Ok(this)
	}
}

impl Display for Ide {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}


impl ValueEnum for Ide {
	fn value_variants<'a>() -> &'a [Self] { Self::ALL }

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		match self {
			Ide::None => PossibleValue::new(self.to_string()).aliases(["no"]).into(),
			Ide::Vscode => PossibleValue::new(self.to_string()).aliases(["vsc"]).into(),
		}
	}
}
