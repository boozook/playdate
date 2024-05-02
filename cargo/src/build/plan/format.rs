use std::collections::BTreeMap;
use std::path::PathBuf;
use cargo::core::compiler::CompileTarget;
use cargo::core::compiler::CrateType;
use cargo::util::command_prelude::CompileMode;
use cargo::core::compiler::CompileKind;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;


#[derive(Debug, Serialize, Deserialize)]
pub struct BuildPlan {
	/// Program invocations needed to build the target (along with dependency information).
	pub invocations: Vec<Invocation>,
	/// List of Cargo manifests involved in the build.
	pub inputs: Vec<PathBuf>,
}

/// A tool invocation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Invocation {
	/// The package this invocation is building a part of.
	pub package_name: String,
	/// Version of the package that is being built.
	pub package_version: semver::Version,
	/// The kind of artifact this invocation creates.
	pub target_kind: TargetKind,
	/// Whether the files created by this invocation are for the host or target system.
	#[serde(serialize_with = "CompileKind::serialize")]
	#[serde(deserialize_with = "deserialize_compile_kind")]
	pub kind: CompileKind,
	#[serde(serialize_with = "CompileMode::serialize")]
	#[serde(deserialize_with = "CompileModeProxy::deserialize")]
	pub compile_mode: CompileMode,
	/// List of invocations this invocation depends on.
	///
	/// The vector contains indices into the [`BuildPlan::invocations`] list.
	///
	/// [`BuildPlan::invocations`]: struct.BuildPlan.html#structfield.invocations
	pub deps: Vec<usize>,
	/// List of output artifacts (binaries/libraries) created by this invocation.
	pub outputs: Vec<PathBuf>,
	/// Hardlinks of output files that should be placed.
	pub links: BTreeMap<PathBuf, PathBuf>,
	/// The program to invoke.
	pub program: String,
	/// Arguments to pass to the program.
	pub args: Vec<String>,
	/// Map of environment variables.
	pub env: BTreeMap<String, String>,
	/// The working directory in which to execute the program.
	pub cwd: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
#[serde(remote = "CompileMode")]
pub enum CompileModeProxy {
	/// A target being built for a test.
	Test,
	/// Building a target with `rustc` (lib or bin).
	Build,
	/// Building a target with `rustc` to emit `rmeta` metadata only. If
	/// `test` is true, then it is also compiled with `--test` to check it like
	/// a test.
	Check { test: bool },
	/// Used to indicate benchmarks should be built. This is not used in
	/// `Unit`, because it is essentially the same as `Test` (indicating
	/// `--test` should be passed to rustc) and by using `Test` instead it
	/// allows some de-duping of Units to occur.
	Bench,
	/// A target that will be documented with `rustdoc`.
	/// If `deps` is true, then it will also document all dependencies.
	Doc { deps: bool, json: bool },
	/// A target that will be tested with `rustdoc`.
	Doctest,
	/// An example or library that will be scraped for function calls by `rustdoc`.
	Docscrape,
	/// A marker for Units that represent the execution of a `build.rs` script.
	RunCustomBuild,
}


fn deserialize_compile_kind<'de, D>(deserializer: D) -> Result<CompileKind, D::Error>
	where D: Deserializer<'de> {
	let res = if let Some(s) = Option::<&str>::deserialize(deserializer)? {
		let target = CompileTarget::new(s).map_err(serde::de::Error::custom)?;
		CompileKind::Target(target)
	} else {
		CompileKind::Host
	};
	Ok(res)
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
// #[serde(remote = "cargo::core::TargetKind")]
pub enum TargetKind {
	Lib(Vec<CrateType>),
	Bin,
	Test,
	Bench,
	Example,
	CustomBuild,
}

impl<'de> Deserialize<'de> for TargetKind {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where D: Deserializer<'de> {
		use self::TargetKind::*;

		let raw = Vec::<&str>::deserialize(deserializer)?;
		Ok(match *raw {
			[] => return Err(serde::de::Error::invalid_length(0, &"at least one target kind")),
			["bin"] => Bin,
			["example"] => Example,
			["test"] => Test,
			["custom-build"] => CustomBuild,
			["bench"] => Bench,
			ref lib_kinds => {
				Lib(lib_kinds.iter()
				             .cloned()
				             .map(|s| CrateType::from(&s.to_owned()))
				             .collect())
			},
		})
	}
}

impl Serialize for TargetKind {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		use self::TargetKind::*;
		match self {
			Lib(kinds) => s.collect_seq(kinds.iter().map(|t| t.to_string())),
			Bin => ["bin"].serialize(s),
			Example => ["example"].serialize(s),
			Test => ["test"].serialize(s),
			CustomBuild => ["custom-build"].serialize(s),
			Bench => ["bench"].serialize(s),
		}
	}
}
