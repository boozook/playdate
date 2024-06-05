use cargo::CargoResult;

use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::proc::cargo_proxy_cmd;
use crate::proc::read_cargo_json;
use self::format::UnitGraph;

use super::build_plan::TargetKindWild;


pub fn unit_graph(cfg: &Config) -> CargoResult<UnitGraph> {
	let mut cargo = cargo_proxy_cmd(cfg, &Cmd::Build)?;

	cargo.args(["--unit-graph", "-Zunstable-options"]);

	let value: UnitGraph = read_cargo_json(cfg, cargo)?;
	Ok(value)
}


impl format::UnitTarget {
	pub fn is_dev(&self) -> bool {
		// This is not so correct because `format::TargetKind::CustomBuild` isn't dev-target,
		// bu we're working with libs, bins and examples only.
		// Also in meta-tree roots are already filtered out, so we have no custom-build here anyway.
		!matches!(self.kind, format::TargetKind::Lib(_) | format::TargetKind::Bin)
	}


	pub fn kind(&self) -> cargo::core::TargetKind {
		use cargo::core::TargetKind as TK;
		use cargo::core::compiler::CrateType as CT;

		match self.kind {
			format::TargetKind::Lib(ref ct) => TK::Lib(ct.clone()),
			format::TargetKind::Bin => TK::Bin,
			format::TargetKind::Test => TK::Test,
			format::TargetKind::Bench => TK::Bench,
			format::TargetKind::Example => {
				if &self.crate_types == &[CT::Bin] {
					TK::ExampleBin
				} else {
					TK::ExampleLib(self.crate_types.clone())
				}
			},
			format::TargetKind::CustomBuild => TK::CustomBuild,
		}
	}


	pub fn kind_wild(&self) -> TargetKindWild {
		use cargo::core::compiler::CrateType as CT;

		match self.kind {
			format::TargetKind::Lib(_) => TargetKindWild::Lib,
			format::TargetKind::Bin => TargetKindWild::Bin,
			format::TargetKind::Test => TargetKindWild::Test,
			format::TargetKind::Bench => TargetKindWild::Bench,
			format::TargetKind::Example => {
				if &self.crate_types == &[CT::Bin] {
					TargetKindWild::ExampleBin
				} else {
					TargetKindWild::ExampleLib
				}
			},
			format::TargetKind::CustomBuild => TargetKindWild::CustomBuild,
		}
	}
}


pub mod format {
	#![allow(dead_code)]
	use cargo::core::PackageId;
	use cargo::util::command_prelude::CompileMode;
	use cargo::core::compiler::CompileKind;
	use cargo::core::compiler::CrateType;
	use serde::Deserialize;

	pub use super::super::format::*;


	#[derive(Debug, Deserialize)]
	pub struct UnitGraph {
		pub version: usize,
		pub units: Vec<Unit>,
		pub roots: Vec<usize>,
	}

	#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
	pub struct Unit {
		#[serde(deserialize_with = "deserialize_package_id", alias = "pkg_id")]
		pub package_id: PackageId,
		pub target: UnitTarget,
		#[serde(serialize_with = "CompileKind::serialize")]
		#[serde(deserialize_with = "deserialize_compile_kind")]
		pub platform: CompileKind,
		#[serde(serialize_with = "CompileMode::serialize")]
		#[serde(deserialize_with = "CompileModeProxy::deserialize")]
		pub mode: CompileMode,
		pub dependencies: Vec<UnitDep>,
		// ...
		// pub features: Vec<serde_json::Value>,
		// pub profile: crate::proc::reader::format::ArtifactProfile,
	}

	#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
	pub struct UnitTarget {
		pub(crate) kind: TargetKind,
		#[serde(deserialize_with = "deserialize_crate_types")]
		pub crate_types: Vec<CrateType>,
		pub name: String,
		pub src_path: String,
		// ...
	}

	#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
	pub struct UnitDep {
		pub index: usize,
		pub extern_crate_name: String,
		pub public: bool,
		#[serde(alias = "noprelude")]
		pub no_prelude: bool,
	}
}
