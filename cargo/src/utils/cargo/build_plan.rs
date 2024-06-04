use cargo::CargoResult;
use cargo::core::PackageId;
use cargo::util::command_prelude::CompileMode;
use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::proc::cargo_proxy_cmd;
use crate::proc::read_cargo_json;
use self::format::TargetKind;


pub fn build_plan(cfg: &Config) -> CargoResult<format::BuildPlan> {
	let mut cargo = cargo_proxy_cmd(cfg, &Cmd::Build)?;

	if !cfg.compile_options.build_config.build_plan {
		cargo.args(["--build-plan", "-Zunstable-options"]);
	}

	read_cargo_json(cfg, cargo)
}


impl format::BuildPlan {
	pub fn build_package_invocations<'plan: 'i, 'p: 'i, 'i>(
		&'plan self,
		package: &'p PackageId)
		-> impl Iterator<Item = &'plan format::Invocation> + 'i {
		self.invocations
		    .iter()
		    .filter(move |item| {
			    item.package_name == package.name().as_str() && package.version() == &item.package_version
		    })
		    .filter(|item| item.compile_mode == CompileMode::Build)
	}
}


#[allow(dead_code)]
pub enum TargetKindWild {
	Lib,
	Bin,
	Test,
	Bench,
	ExampleLib,
	ExampleBin,
	CustomBuild,
}

impl PartialEq<TargetKind> for TargetKindWild {
	fn eq(&self, other: &TargetKind) -> bool {
		match self {
			TargetKindWild::Lib => matches!(other, TargetKind::Lib(_)),
			TargetKindWild::Bin => matches!(other, TargetKind::Bin),
			TargetKindWild::Test => matches!(other, TargetKind::Test),
			TargetKindWild::Bench => matches!(other, TargetKind::Bench),
			TargetKindWild::ExampleLib => matches!(other, TargetKind::Example),
			TargetKindWild::ExampleBin => matches!(other, TargetKind::Example),
			TargetKindWild::CustomBuild => matches!(other, TargetKind::CustomBuild),
		}
	}
}


pub mod format {
	use std::collections::BTreeMap;
	use std::path::PathBuf;
	use cargo::util::command_prelude::CompileMode;
	use cargo::core::compiler::CompileKind;
	use serde::{Serialize, Deserialize};

	pub use super::super::format::*;


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
}
