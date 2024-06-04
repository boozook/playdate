use ::cargo::CargoResult;
use ::cargo::core::Workspace;
use ::cargo::core::compiler::BuildContext;
use ::cargo::core::compiler::UnitInterner;
use ::cargo::ops::CompileOptions;
use ::cargo::ops::create_bcx;
use ::cargo::util::command_prelude::CompileMode;
use try_lazy_init::Lazy;

use crate::config::Config;

pub mod rustc;
pub mod cargo;
pub mod workspace;
pub mod path;
pub mod logging;


#[deprecated(since = "0.5",
             note = "TODO: use crate::utils::cargo:: unit_graph with metadata instead")]
pub struct LazyBuildContext<'a, 'cfg> {
	workspace: &'cfg Workspace<'cfg>,
	bcx: Lazy<BuildContext<'a, 'cfg>>,
	interner: UnitInterner,
	options: CompileOptions,
}

impl<'a, 'cfg> LazyBuildContext<'a, 'cfg> {
	pub fn new(config: &'cfg Config) -> CargoResult<Self> {
		let options = CompileOptions::new(config.workspace.config(), CompileMode::Check { test: false })?;
		Ok(Self { bcx: Lazy::new(),
		          interner: UnitInterner::new(),
		          workspace: &config.workspace,
		          options })
	}

	pub fn get(&'a self) -> CargoResult<&BuildContext<'a, 'cfg>> {
		self.bcx
		    .try_get_or_create(move || create_bcx(self.workspace, &self.options, &self.interner))
	}
}
