use std::ffi::OsStr;

use cargo::util::interning::InternedString;
use cargo::CargoResult;

use crate::config::Config;
use crate::proc::cargo_proxy_with;
use crate::proc::read_cargo_json;


pub type CargoMetadataPd = format::Report<format::CrateMetadata<InternedString>, format::WorkspaceMetadata>;


pub fn metadata(cfg: &Config) -> CargoResult<CargoMetadataPd> {
	let mut cargo = cargo_proxy_with(cfg, "metadata", false)?;

	cargo.arg("--format-version=1");

	let kinds = &cfg.compile_options.build_config.requested_kinds[..];
	if kinds.len() == 1 &&
	   let Some(kind) = kinds.first()
	{
		match kind {
			cargo::core::compiler::CompileKind::Target(target) if target != &cfg.host_target => {
				cargo.args(["--filter-platform", &target.rustc_target()]);
			},
			_ => (),
		}
	}

	// add manifest options:
	{
		const MANIFEST_PATH: &str = "--manifest-path";
		let expected = &[
		                 OsStr::new("--locked"),
		                 OsStr::new("--offline"),
		                 OsStr::new("--frozen"),
		];
		let args = cfg.args.iter().enumerate().filter(|(_, arg)| {
			                                      expected.contains(&arg.as_os_str()) ||
			                                      arg.as_os_str() == MANIFEST_PATH ||
			                                      arg.to_string_lossy().starts_with(MANIFEST_PATH)
		                                      });

		args.for_each(|(i, arg)| {
			    cargo.arg(arg);
			    if arg.as_os_str() == MANIFEST_PATH {
				    cargo.arg(&cfg.args[i + 1]);
			    }
		    });

		// if !cfg.workspace.ignore_lock() && !cargo.get_args().any(|arg| arg == "--locked") {
		// 	cargo.arg("--locked");
		// }
	}

	read_cargo_json::<CargoMetadataPd>(cfg, cargo)
}


pub mod format {
	#![allow(dead_code)]
	use std::path::PathBuf;

	use cargo::core::dependency::DepKind;
	use cargo::core::PackageId;
	use cargo::core::PackageIdSpec;
	use cargo::core::SourceId;
	use serde::Deserialize;
	use serde::Deserializer;

	use crate::utils::cargo::unit_graph::format::UnitTarget;

	pub use super::super::format::*;

	pub use playdate::metadata::format::CrateMetadata;
	pub use playdate::metadata::format::ws::WorkspaceMetadata;


	/// `cargo metadata` output __v1__,
	/// just necessary fields.
	#[derive(Debug, Deserialize)]
	#[serde(bound(deserialize = "Meta: Deserialize<'de>, WsMeta: Deserialize<'de>"))]
	pub struct Report<Meta = serde_json::Value, WsMeta = serde_json::Value> {
		pub version: usize,
		pub packages: Vec<Package<Meta>>,
		pub target_directory: PathBuf,

		pub workspace_members: Vec<SourceId>,
		pub workspace_default_members: Vec<SourceId>,
		pub workspace_root: PathBuf,
		#[serde(alias = "metadata")]
		pub workspace_metadata: Option<WsMeta>,

		pub resolve: Resolve,
	}

	#[derive(Deserialize, Debug)]
	pub struct Resolve {
		pub nodes: Vec<ResolveNode>,
		pub root: Option<SourceId>,
	}

	#[derive(Deserialize, Debug)]
	pub struct ResolveNode {
		pub id: PackageIdSpec,
		pub dependencies: Vec<PackageIdSpec>,
		pub deps: Vec<NodeDep>,
	}


	#[derive(Deserialize, Debug)]
	#[serde(bound(deserialize = "Metadata: Deserialize<'de>"))]
	pub struct Package<Metadata> {
		pub id: PackageIdSpec,
		pub source: Option<SourceId>,
		pub dependencies: Vec<PackageDep>,

		pub name: String,
		pub authors: Vec<String>,
		pub version: String,
		pub description: Option<String>,
		pub manifest_path: PathBuf,
		pub targets: Vec<UnitTarget>,
		pub metadata: Option<Metadata>,
	}

	#[derive(Deserialize, Debug)]
	pub struct PackageDep {
		pub name: String,
		pub rename: Option<String>,

		pub source: Option<SourceId>,
		pub req: semver::VersionReq,
		#[serde(serialize_with = "DepKind::serialize")]
		#[serde(deserialize_with = "deserialize_dep_kind")]
		pub kind: DepKind,

		pub optional: bool,
		// ... features, target, registry
		// pub target: Option<serde_json::Value>,
	}

	#[derive(Deserialize, Debug)]
	pub struct NodeDep {
		pub name: String,
		pub pkg: PackageIdSpec,
		pub dep_kinds: serde_json::Value,
	}

	pub fn deserialize_dep_kind<'de, D>(deserializer: D) -> Result<DepKind, D::Error>
		where D: Deserializer<'de> {
		let kind = Option::<&str>::deserialize(deserializer)?;
		let kind = match kind {
			Some("dev") => DepKind::Development,
			Some("build") => DepKind::Build,
			None => DepKind::Normal,
			kind => {
				log::error!("Unknown dep kind: {kind:?}");
				DepKind::Normal
			},
		};
		Ok(kind)
	}
}
