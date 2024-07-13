use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use cargo::core::Workspace;
use cargo::core::compiler::{CompileTarget, CompileKind, TargetInfo};
use cargo::ops::CompileOptions;
use playdate::toolchain::gcc::{ArmToolchain, Gcc};
use playdate::toolchain::sdk::Sdk;
use try_lazy_init::Lazy;

use cargo::util::{CargoResult, Rustc};

use crate::build::rustflags::Rustflags;
use crate::cli::cmd::Cmd;
use crate::cli::deps::Dependency;
use crate::cli::ide::Ide;
use crate::cli::opts::Mount;


pub struct Config<'cfg> {
	/// Requested command
	pub cmd: Cmd,
	/// Cleaned args for_cargo - args excluding our exclusive args
	pub args: Vec<OsString>,

	pub verbose: u32,
	pub quiet: bool,

	pub dry_run: bool,
	pub skip_unknown: bool,
	pub skip_prebuild: bool,

	pub no_sdk: bool,
	pub no_gcc: bool,

	pub sdk_path: Option<PathBuf>,
	pub gcc_path: Option<PathBuf>,

	pub mounting: Option<Mount>,
	pub no_read: bool,

	pub zip: bool,
	pub no_info_meta: bool,

	pub prevent_unwinding: bool,

	// init & new
	pub create_path: Option<PathBuf>,
	pub create_full_config: bool,
	pub create_local_schema: bool,
	pub create_full_metadata: bool,
	pub create_deps_sys_only: bool,
	pub create_deps: Vec<Dependency<'static>>,
	pub ide: Ide,

	pub workspace: Workspace<'cfg>,
	pub host_target: CompileTarget,
	pub compile_options: CompileOptions,

	sdk: Lazy<Sdk>,
	gcc: Lazy<ArmToolchain>,
	rustflags: Lazy<Rustflags>,
	unit_graph: Lazy<crate::utils::cargo::unit_graph::format::UnitGraph>,
	ws_metadata: Lazy<crate::utils::cargo::metadata::CargoMetadataPd>,
	target_infos: Lazy<HashMap<CompileKind, Lazy<TargetInfo>>>,

	pub rustc: Rustc,
	pub rustup: RustupToolchain,
}


impl<'cfg> Config<'cfg> {
	pub fn new(cmd: Cmd,
	           args: Vec<OsString>,
	           verbose: u32,
	           quiet: bool,
	           dry_run: bool,
	           skip_unknown: bool,
	           skip_prebuild: bool,
	           no_sdk: bool,
	           no_gcc: bool,
	           sdk_path: Option<PathBuf>,
	           gcc_path: Option<PathBuf>,
	           mounting: Option<Mount>,
	           no_read: bool,
	           zip: bool,
	           no_info_meta: bool,
	           prevent_unwinding: bool,
	           create_path: Option<PathBuf>,
	           create_full_config: bool,
	           create_local_schema: bool,
	           create_full_metadata: bool,
	           create_deps_sys_only: bool,
	           create_deps: Vec<Dependency<'static>>,
	           ide: Ide,
	           workspace: Workspace<'cfg>,
	           host_target: CompileTarget,
	           compile_options: CompileOptions,
	           rustc: Rustc)
	           -> Self {
		Self { cmd,
		       args,
		       verbose,
		       quiet,
		       dry_run,
		       skip_unknown,
		       skip_prebuild,
		       no_sdk,
		       no_gcc,
		       sdk_path,
		       gcc_path,
		       mounting,
		       no_read,
		       zip,
		       no_info_meta,
		       prevent_unwinding,
		       create_path,
		       create_full_config,
		       create_local_schema,
		       create_full_metadata,
		       create_deps_sys_only,
		       create_deps,
		       ide,
		       workspace,
		       host_target,
		       rustc,
		       compile_options,
		       sdk: Lazy::new(),
		       gcc: Lazy::new(),
		       rustflags: Lazy::new(),
		       unit_graph: Lazy::new(),
		       ws_metadata: Lazy::new(),
		       target_infos: Lazy::new(),
		       rustup: Default::default() }
	}


	pub fn rustflags(&self) -> CargoResult<&Rustflags> {
		self.rustflags.try_get_or_create(|| Rustflags::try_default(self))
	}

	pub fn sdk(&self) -> CargoResult<&Sdk> {
		self.sdk.try_get_or_create(|| {
			        let sdk = (!self.no_sdk).then(|| {
				                                self.sdk_path
				                                    .as_ref()
				                                    .map_or_else(Sdk::try_new, Sdk::try_new_exact)
			                                })
			                                .ok_or_else(|| anyhow::anyhow!("Attempt to get sdk but 'no-sdk' is set"))??;
			        Ok(sdk)
		        })
	}

	pub fn gcc(&self) -> CargoResult<&ArmToolchain> {
		self.gcc.try_get_or_create(|| {
			        let res =
				        (!self.no_gcc).then(|| {
					                      self.gcc_path.as_ref().map_or_else(ArmToolchain::try_new, |p| {
						                                            Gcc::try_new_exact_path(p).and_then(ArmToolchain::try_new_with)
					                                            })
				                      })
				                      .ok_or_else(|| anyhow::anyhow!("Attempt to get arm-gcc but 'no-gcc' is set"))??;
			        Ok(res)
		        })
	}


	pub fn unit_graph(&self) -> CargoResult<&crate::utils::cargo::unit_graph::format::UnitGraph> {
		self.unit_graph
		    .try_get_or_create(|| crate::utils::cargo::unit_graph::unit_graph(self))
	}

	pub fn metadata(&self) -> CargoResult<&crate::utils::cargo::metadata::CargoMetadataPd> {
		self.ws_metadata
		    .try_get_or_create(|| crate::utils::cargo::metadata::metadata(self))
	}

	pub fn target_info_for(&self, kind: CompileKind) -> CargoResult<&TargetInfo> {
		let map = self.target_infos.try_get_or_create(|| -> anyhow::Result<_> {
			                            Ok(self.possible_compile_kinds()?
			                                   .into_iter()
			                                   .map(|k| (k, Lazy::new()))
			                                   .collect())
		                            })?;
		map.get(&kind)
		   .map(|v| v.try_get_or_create(|| self.target_info(kind)))
		   .ok_or_else(|| anyhow::anyhow!("Target-info for unexpected {kind:?}, not prepared."))?
	}
}


#[derive(Debug)]
pub enum RustupToolchain {
	None,
	Some(OsString),
}

impl RustupToolchain {
	pub fn as_os_str(&self) -> Option<&OsStr> {
		match self {
			RustupToolchain::None => None,
			RustupToolchain::Some(s) => Some(s),
		}
	}
}
impl Default for RustupToolchain {
	fn default() -> Self {
		if let Some(value) = std::env::var_os("RUSTUP_TOOLCHAIN") {
			Self::Some(value)
		} else {
			Self::None
		}
	}
}
