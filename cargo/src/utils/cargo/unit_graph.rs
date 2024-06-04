use cargo::CargoResult;

use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::proc::cargo_proxy_cmd;
use crate::proc::read_cargo_json;
use self::format::UnitGraph;


pub fn unit_graph(cfg: &Config) -> CargoResult<UnitGraph> {
	let mut cargo = cargo_proxy_cmd(cfg, &Cmd::Build)?;

	cargo.args(["--unit-graph", "-Zunstable-options"]);

	let value: UnitGraph = read_cargo_json(cfg, cargo)?;
	Ok(value)
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
		pub kind: TargetKind,
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
