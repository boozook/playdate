use cargo::CargoResult;

use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::proc::cargo_proxy_cmd;
use crate::proc::read_cargo_json;
use super::format::TargetKindWild;
use self::format::UnitGraph;


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
			format::TargetKind::Lib(ref ct) => TK::Lib(ct.to_owned()),
			format::TargetKind::Bin => TK::Bin,
			format::TargetKind::Test => TK::Test,
			format::TargetKind::Bench => TK::Bench,
			format::TargetKind::Example => {
				if &self.crate_types == &[CT::Bin] {
					TK::ExampleBin
				} else {
					TK::ExampleLib(self.crate_types.to_owned())
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


	pub fn cargo_target(&self) -> cargo::core::manifest::Target {
		use cargo::core::manifest::Target;

		match self.kind {
			format::TargetKind::Lib(_) => {
				Target::lib_target(
				                   &self.name,
				                   self.crate_types.to_owned(),
				                   self.src_path.as_str().into(),
				                   self.edition,
				)
			},
			format::TargetKind::Bin => {
				Target::bin_target(
				                   &self.name,
				                   None,
				                   self.src_path.as_str().into(),
				                   None,
				                   self.edition,
				)
			},
			format::TargetKind::Example => {
				Target::example_target(
				                       &self.name,
				                       self.crate_types.to_owned(),
				                       self.src_path.as_str().into(),
				                       None,
				                       self.edition,
				)
			},
			format::TargetKind::Test => unimplemented!("test cargo-target"),
			format::TargetKind::Bench => unimplemented!("bench cargo-target"),
			format::TargetKind::CustomBuild => unimplemented!("custom-build cargo-target"),
		}
	}
}


pub mod format {
	use std::str::FromStr;

	use cargo::core::Edition;
	use cargo::core::PackageId;
	use cargo::util::command_prelude::CompileMode;
	use cargo::core::compiler::CompileKind;
	use cargo::core::compiler::CrateType;

	pub use super::super::format::*;


	#[derive(Debug, Deserialize)]
	pub struct UnitGraph {
		#[allow(dead_code)]
		pub version: usize,
		pub units: Vec<Unit>,
		pub roots: Vec<usize>,
	}

	#[derive(Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
	pub struct Unit {
		#[serde(deserialize_with = "de_package_id_or_spec", alias = "pkg_id")]
		pub package_id: PackageId,
		pub target: UnitTarget,
		#[serde(serialize_with = "CompileKind::serialize")]
		#[serde(deserialize_with = "de_compile_kind")]
		pub platform: CompileKind,
		#[serde(serialize_with = "CompileMode::serialize")]
		#[serde(deserialize_with = "CompileModeProxy::deserialize")]
		pub mode: CompileMode,
		pub dependencies: Vec<UnitDep>,
		// ...
		// pub features: Vec<serde_json::Value>,
		// pub profile: crate::proc::reader::format::ArtifactProfile,
	}

	#[derive(Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
	pub struct UnitTarget {
		pub(crate) kind: TargetKind,
		#[serde(deserialize_with = "de_crate_types")]
		pub crate_types: Vec<CrateType>,
		pub name: String,
		pub src_path: String,
		#[serde(deserialize_with = "de_edition")]
		pub edition: Edition,
		// ... doc, doctest, test
	}

	#[derive(Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
	pub struct UnitDep {
		pub index: usize,
		pub extern_crate_name: String,
		pub public: bool,
		#[serde(alias = "noprelude")]
		pub no_prelude: bool,
	}


	pub fn de_edition<'de, D>(deserializer: D) -> Result<Edition, D::Error>
		where D: Deserializer<'de> {
		let s = <&str>::deserialize(deserializer)?;
		Edition::from_str(s).map_err(serde::de::Error::custom)
	}
}
