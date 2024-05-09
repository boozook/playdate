use std::collections::HashMap;
use std::path::{PathBuf, Path};
use anyhow::anyhow;
use cargo::core::Shell;
use cargo::core::{Package, PackageId};
use playdate::assets::BuildReport;
use playdate::assets::apply_build_plan;
use playdate::config::Env;
use playdate::metadata::format::AssetsOptions;
use crate::config::Config;
use crate::utils::path::AsRelativeTo;
use playdate::consts::SDK_ENV_VAR;
use cargo::util::CargoResult;
use playdate::metadata::format::PlayDateMetadata;
use playdate::assets::plan::BuildPlan as AssetsPlan;
use playdate::assets::plan::build_plan as assets_build_plan;
use try_lazy_init::Lazy;

use crate::layout::{PlaydateAssets, LayoutLock};


pub type TomlMetadata = PlayDateMetadata<toml::Value>;


pub struct LazyEnvBuilder<'a, 'cfg> {
	config: &'a Config<'cfg>,
	package: &'cfg Package,
	env: Lazy<Env>,
}

impl<'a, 'cfg> LazyEnvBuilder<'a, 'cfg> {
	pub fn new(config: &'cfg Config, package: &'cfg Package) -> Self {
		Self { env: Lazy::new(),
		       package,
		       config }
	}

	pub fn get(&'a self) -> CargoResult<&'a Env> {
		self.env.try_get_or_create(move || {
			        let root = self.package.root().display().to_string();
			        let vars = vec![
			                        ("CARGO_PKG_NAME", self.package.name().to_string()),
			                        ("CARGO_MANIFEST_DIR", root.to_string()),
			];

			        let mut env = Env::try_from_iter(vars.into_iter()).map_err(|err| anyhow::anyhow!("{err}"))?;

			        // add global environment:
			        for (k, v) in std::env::vars() {
				        if !env.vars.contains_key(&k) {
					        env.vars.insert(k, v);
				        }
			        }

			        if let Some(path) = self.config.sdk_path.as_ref() {
				        env.vars.insert(SDK_ENV_VAR.into(), path.display().to_string());
			        }

			        Ok::<_, anyhow::Error>(env)
		        })
	}
}


pub type LockedLayout<'t> = LayoutLock<&'t mut PlaydateAssets<PathBuf>>;


/// Returns `None` if there is no `assets` metadata.
pub fn plan_for<'cfg, 'env, 'l>(config: &'cfg Config,
                                package: &'cfg Package,
                                metadata: &TomlMetadata,
                                env: &'cfg LazyEnvBuilder<'env, 'cfg>,
                                layout: &'l LockedLayout<'l>,
                                with_dev: bool)
                                -> CargoResult<PackageAssetsPlan<'env, 'cfg>> {
	let opts = metadata.assets_options();

	let has_dev_assets = with_dev && metadata.dev_assets.iter().any(|t| !t.is_empty());
	let is_empty = metadata.assets.is_empty() && !has_dev_assets;

	if is_empty {
		return Ok(PackageAssetsPlan { main: None,
		                              dev: None });
	}

	let env = env.get()?;
	let root = package.manifest_path()
	                  .parent()
	                  .ok_or(anyhow!("No parent of manifest-path"))?;

	let main = if !metadata.assets.is_empty() {
		let plan = assets_build_plan(env, &metadata.assets, opts.as_ref(), Some(root))?;

		// main-assets plan:
		let path = layout.as_inner().assets_plan_for(config, package);
		let mut cached = CachedPlan::new(path, plan)?;
		if config.compile_options.build_config.force_rebuild {
			cached.difference = Difference::Missing;
		}

		Some(cached)
	} else {
		None
	};


	// dev-assets plan:
	let dev = if has_dev_assets && metadata.dev_assets.is_some() {
		let assets = metadata.dev_assets.as_ref().unwrap();
		let dev_plan = assets_build_plan(env, assets, opts.as_ref(), Some(root))?;

		let path = layout.as_inner().assets_plan_for_dev(config, package);
		let mut dev_cached = CachedPlan::new(path, dev_plan)?;

		// Inheritance, if main is stale or missing - this one is too:
		if let Some(main) = main.as_ref() {
			if !matches!(main.difference, Difference::Same) {
				dev_cached.difference = main.difference;
			}
		}

		dev_cached.into()
	} else {
		None
	};

	Ok(PackageAssetsPlan { main, dev })
}


#[derive(Debug)]
pub struct PackageAssetsPlan<'t, 'cfg> {
	/// Main build-plan.
	///
	/// Can be empty, so `None`.
	pub main: Option<CachedPlan<'t, 'cfg>>,

	/// Dev-assets build-plan.
	///
	/// Inherited by main `plan`.
	///
	/// Can be empty, so `None`.
	pub dev: Option<CachedPlan<'t, 'cfg>>,
}

impl<'t, 'cfg> PackageAssetsPlan<'t, 'cfg> {
	pub fn is_empty(&self) -> bool { self.main.is_none() && self.dev.is_none() }
}


#[derive(Debug)]
pub struct CachedPlan<'t, 'cfg> {
	/// Inner build-plan
	pub plan: AssetsPlan<'t, 'cfg>,

	/// Path to the cache file
	pub path: PathBuf,

	/// State of the cache
	pub difference: Difference,

	serialized: Option<String>,
}


impl<'t, 'cfg> CachedPlan<'t, 'cfg> {
	#[must_use = "Cached plan must be used"]
	fn new(path: PathBuf, plan: AssetsPlan<'t, 'cfg>) -> CargoResult<Self> {
		let mut serializable = plan.iter_flatten().collect::<Vec<_>>();
		serializable.sort_by_key(|(_, _, (p, _))| p.to_string_lossy().to_string());
		let json = serde_json::to_string(&serializable)?;

		let difference = if path.try_exists()? {
			if std::fs::read_to_string(&path)? == json {
				log::debug!("Cached plan is the same");
				Difference::Same
			} else {
				log::debug!("Cache mismatch, need diff & rebuild");
				Difference::Different
			}
		} else {
			log::debug!("Cache mismatch, full rebuilding");
			Difference::Missing
		};

		let serialized = (!difference.is_same()).then_some(json);

		Ok(Self { plan,
		          path,
		          difference,
		          serialized })
	}


	pub fn apply(self,
	             dest: &Path,
	             options: &AssetsOptions,
	             config: &Config)
	             -> CargoResult<BuildReport<'t, 'cfg>> {
		let cache = self.serialized;
		let report = apply_build_plan(self.plan, dest, options)?;
		// and finally save cache of just successfully applied plan:
		// only if there is no errors
		if !report.has_errors() {
			if let Some(data) = &cache {
				std::fs::write(&self.path, data)?;
				config.log().verbose(|mut log| {
					            let path = self.path.as_relative_to_root(config);
					            log.status("Cache", format!("saved to {}", path.display()));
				            });
			} else {
				config.log().verbose(|mut log| {
					            log.status("Cache", "nothing to save");
				            });
			}
		} else {
			config.log().verbose(|mut log| {
				            let message = "build has errors, so cache was not saved";
				            log.status("Cache", message);
			            });
		}

		Ok(report)
	}


	pub fn printable_serializable(&self, source: &Package, kind: AssetKind) -> SerializablePlan<'_, 't, 'cfg> {
		SerializablePlan { package: source.package_id(),
		                   plan: &self.plan,
		                   difference: &self.difference,
		                   path: &self.path,
		                   kind }
	}


	pub fn as_inner(&self) -> &AssetsPlan<'t, 'cfg> { &self.plan }


	pub fn pretty_print(&self, shell: &mut Shell, root: &Path) -> CargoResult<()> {
		use playdate::assets::plan::*;
		use playdate::assets::resolver::*;

		let title = |(left, right): &(Expr, Expr)| format!("rule: {} = {}", left.original(), right.original());
		let row_columns = |target: &Path, source: &Path| {
			(format!("  {}", target.as_relative_to(&root).display()),
			 source.as_relative_to(&root).display().to_string())
		};

		let mut sections = HashMap::new();
		for mapping in self.as_inner().as_inner().iter() {
			match mapping {
				Mapping::AsIs(inc, exprs) => {
					sections.insert(title(exprs), vec![row_columns(&inc.target(), &inc.source())]);
				},
				Mapping::Into(inc, exprs) => {
					sections.insert(title(exprs), vec![row_columns(&inc.target(), &inc.source())]);
				},
				Mapping::ManyInto { sources,
				                    target,
				                    exprs,
				                    .. } => {
					let mut rows = Vec::new();
					for inc in sources.iter() {
						rows.push(row_columns(&target.join(inc.target()), &inc.source()));
					}
					sections.insert(title(exprs), rows);
				},
			}
		}

		// calc max len for left column:
		let mut max_len = 0;
		for (_, rows) in sections.iter() {
			for (left, _) in rows.iter() {
				max_len = left.len().max(max_len);
			}
		}

		// add padding to left column:
		for (_, rows) in sections.iter_mut() {
			for (left, _) in rows.iter_mut() {
				let diff = max_len - left.len();
				left.push_str(&" ".repeat(diff));
			}
		}

		// print:
		for (title, rows) in sections.iter_mut() {
			shell.status("", title)?;
			for (left, right) in rows.iter_mut() {
				shell.status("", format!("{left} <- {right}"))?;
			}
		}

		Ok(())
	}
}


#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum Difference {
	Same,
	Different,
	/// There is not cache file.
	Missing,
}

impl Difference {
	/// Needs rebuild
	pub fn is_same(&self) -> bool { matches!(self, Self::Same) }
}


#[derive(Debug, serde::Serialize)]
pub struct SerializablePlan<'p, 't, 'cfg> {
	package: PackageId,

	kind: AssetKind,

	#[serde(rename = "assets")]
	plan: &'p AssetsPlan<'t, 'cfg>,

	#[serde(rename = "cache")]
	difference: &'p Difference,

	#[serde(rename = "plan")]
	path: &'p Path,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum AssetKind {
	Package,
	Dev,
}
