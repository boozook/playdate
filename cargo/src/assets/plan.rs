use std::collections::HashMap;
use std::path::{PathBuf, Path};
use anyhow::anyhow;
use cargo::core::Shell;
use cargo::core::{Package, PackageId};
use playdate::assets::BuildReport;
use playdate::assets::apply_build_plan;
use playdate::config::Env;
use playdate::metadata::format::AssetsOptions;
use playdate::metadata::source::MetadataSource as _;
use crate::config::Config;
use crate::utils::path::AsRelativeTo;
use playdate::consts::SDK_ENV_VAR;
use cargo::util::CargoResult;
pub use playdate::metadata::format::Metadata;
use playdate::assets::plan::BuildPlan as AssetsPlan;
use playdate::assets::plan::build_plan as assets_build_plan;
use try_lazy_init::Lazy;

use crate::layout::{PlaydateAssets, LayoutLock};


pub struct LazyEnvBuilder<'a, 'cfg> {
	config: &'a Config<'cfg>,
	package_id: PackageId,
	root: &'cfg Path,
	env: Lazy<Env>,
}

impl<'a, 'cfg> LazyEnvBuilder<'a, 'cfg> {
	pub fn new(config: &'cfg Config, pkg_id: PackageId, root: &'cfg Path) -> Self {
		Self { env: Lazy::new(),
		       package_id: pkg_id,
		       root,
		       config }
	}

	pub fn new_for(config: &'cfg Config, package: &'cfg Package) -> Self {
		Self::new(config, package.package_id(), package.root())
	}

	pub fn get(&'a self) -> CargoResult<&'a Env> {
		self.env.try_get_or_create(move || {
			        let root = self.root.display().to_string();
			        let vars = vec![
			                        ("CARGO_PKG_NAME", self.package_id.name().to_string()),
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


pub mod proto {
	use std::borrow::Cow;
	use std::collections::{BTreeMap, HashMap, HashSet};
	use std::path::Path;

	use super::{PackageId, Config, CargoResult};
	use super::assets_build_plan;

	use playdate::assets::plan::BuildPlan;
	use playdate::consts::SDK_ENV_VAR;
	use playdate::manifest::PackageSource as _;
	use playdate::metadata::format::{AssetsOptions, Options};
	use playdate::metadata::source::MetadataSource as _;

	use crate::utils::cargo::meta_deps::MetaDeps;
	use crate::utils::cargo::meta_deps::{Node, RootNode};
	use crate::utils::path::AsRelativeTo;


	pub struct AssetsPlans<'cfg> {
		pub plans: Vec<BuildPlan<'cfg, 'cfg>>,
		/// per-dep ?dev plan
		pub index: BTreeMap<Key, usize>,
		/// per-root plans to merge
		pub targets: BTreeMap<MultiKey, Vec<usize>>,
	}

	#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
	pub struct Key {
		pub id: PackageId,
		pub dev: bool,
	}

	impl Key {
		fn with_dev(&self, dev: bool) -> Self {
			Self { id: self.id.to_owned(),
			       dev }
		}
	}
	impl From<&'_ Node<'_>> for Key {
		fn from(node: &'_ Node<'_>) -> Self {
			Key { id: node.package_id().to_owned(),
			      dev: node.target().is_dev() }
		}
	}


	#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
	pub struct MultiKey {
		/// Dependencies
		id: Vec<PackageId>,
		/// Primary target is dev
		dev: bool,
	}
	impl From<&'_ RootNode<'_>> for MultiKey {
		fn from(root: &'_ RootNode<'_>) -> Self {
			Self { dev: root.node().target().is_dev(),
			       id: root.deps()
			               .into_iter()
			               .map(|d| d.package_id().to_owned())
			               .collect() }
		}
	}
	impl MultiKey {
		pub fn dev(&self) -> bool { self.dev }

		pub fn is_for(&self, root: &'_ RootNode<'_>) -> bool {
			root.node().target().is_dev() == self.dev &&
			root.deps()
			    .into_iter()
			    .enumerate()
			    .all(|(i, d)| self.id.get(i).filter(|k| *k == d.package_id()).is_some())
			// root.deps().into_iter().enumerate().all(|(i, d)| d.package_id() == &self.id[i])
		}
	}


	pub fn plan_all<'cfg>(cfg: &Config<'cfg>, tree: &MetaDeps<'cfg>) -> CargoResult<AssetsPlans<'cfg>> {
		// results:
		let mut plans = AssetsPlans { plans: Vec::with_capacity(tree.roots().len() * 2),
		                              index: BTreeMap::new(),
		                              targets: BTreeMap::new() };

		// prepare env:
		let global_env: BTreeMap<_, _> =
			std::env::vars().into_iter()
			                .map(|(k, v)| (k, v))
			                .chain({
				                cfg.sdk()
				                   .map(|sdk| sdk.path())
				                   .ok()
				                   .or_else(|| cfg.sdk_path.as_deref())
				                   .map(|p| (SDK_ENV_VAR.to_string(), p.display().to_string()))
				                   .into_iter()
			                })
			                .collect();
		let env = |id: &PackageId, root: &Path| {
			use playdate::config::Env;

			let vars = [
			            (Cow::from("CARGO_PKG_NAME"), Cow::from(id.name().as_str())),
			            (Cow::from("CARGO_MANIFEST_DIR"), root.display().to_string().into()),
			];

			let iter = global_env.iter()
			                     .map(|(k, v)| (Cow::Borrowed(k.as_str()), Cow::Borrowed(v.as_str())))
			                     .chain(vars.into_iter());

			Env::try_from_iter(iter).map_err(|err| anyhow::anyhow!("{err}"))
		};


		for root in tree.roots() {
			let meta_source = root.as_source();

			let options = meta_source.assets_options();

			let root_is_dev = root.node().target().is_dev();

			log::debug!(
			            "planning for {} ({}) +dev:{root_is_dev}",
			            root.package_id(),
			            root.node().target().kind().description()
			);
			log::debug!("  dependencies are allowed: {}", options.dependencies);


			let plan_key = MultiKey::from(root);
			if plans.targets.contains_key(&plan_key) {
				log::debug!("  skip: already done");
				continue;
			}


			let mut indices = Vec::<usize>::with_capacity(root.deps().len());

			for dep in root.deps().iter().rev() {
				log::debug!("  planning dep: {}", dep.package_id());

				let crate_root = dep.manifest_path()
				                    .and_then(|p| p.parent())
				                    .ok_or_else(|| anyhow::anyhow!("Unable to get crate root"))?;

				let env = env(dep.package_id(), crate_root)?;

				let with_dev = root_is_dev && dep.package_id() == root.package_id();

				let dep_key = Key::from(dep).with_dev(with_dev);


				let plan_for =
					|plans: &mut AssetsPlans, indices: &mut Vec<usize>, key: Key, dev: bool| -> anyhow::Result<()> {
						let source = dep.as_source();
						let name_log = dev.then_some("dev-").unwrap_or_default();
						if let Some(assets) = source.metadata()
						                            .map(|m| if dev { m.dev_assets() } else { m.assets() }) &&
						   !assets.is_empty()
						{
							// let plan =
							match assets_build_plan(&env, assets, &options, Some(crate_root.into())) {
								Ok(plan) => {
									let pid = key.id.clone();
									let is_dev = key.dev;
									let dev_index = plans.plans.len();
									plans.index.insert(key, dev_index);
									plans.plans.push(plan);
									indices.push(dev_index);

									log::debug!("    done: +#{dev_index} (dev:{is_dev})");
									cfg.log().verbose(|mut log| {
										         log.status("Plan", format_args!("{name_log}assets for {pid} planned",))
									         })
								},
								Err(err) => {
									cfg.log()
									   .error(format_args!("{err}, caused when planning {name_log}assets for {}", key.id));
									return cfg.compile_options
									          .build_config
									          .keep_going
									          .then_some(())
									          .ok_or(err.into());
								},
							}
						} else {
							cfg.log().verbose(|mut log| {
								         log.status(
								                    "Skip",
								                    format_args!(
									"{name_log}assets for {} without plan, reason: empty",
									key.id
								),
								)
							         })
						}
						Ok(())
					};

				if let Some(i) = plans.index.get(&dep_key) {
					// we already have plan for this dep
					log::debug!("    done (~#{i}) (dev:{})", dep_key.dev);
					indices.push(*i);
				} else if with_dev && let Some(base_index) = plans.index.get(&dep_key.with_dev(false)).copied() {
					// we already have plan for this dep, but not for dev part
					indices.push(base_index);
					log::debug!("    done (~#{base_index}) (dev:{})", dep_key.dev);

					plan_for(&mut plans, &mut indices, dep_key, true)?;
				} else {
					// else just build a plan
					plan_for(&mut plans, &mut indices, dep_key, false)?;

					// TODO: it must be norm+dev assets, if dev needed - `with_dev`
					if with_dev {
						log::warn!("    TODO: WITH DEV")
					}
				}
			}


			plans.targets
			     .entry(plan_key)
			     .and_modify(|vec| vec.append(&mut indices))
			     .or_insert(indices);
		}


		// report:
		cfg.log()
		   .status("Assets", "planning complete for all requested targets");
		cfg.log_extra_verbose(|mut logger| {
			   for (k, v) in &plans.targets {
				   let dev = k.dev.then_some(" +dev").unwrap_or_default();
				   let key = k.id.iter().map(|p| p.name()).collect::<Vec<_>>().join(", ");
				   logger.status("Plans", format_args!("for{dev} {key}"));
				   for i in v {
					   let plan = &plans.plans[*i];
					   logger.status("Plan", format_args!("#{i}:\n{plan:>10}"));
				   }
			   }
		   });

		// check:
		for root in tree.roots() {
			let key = MultiKey::from(root);
			debug_assert!(plans.targets.contains_key(&key));
		}

		Ok(plans)
	}


	/// Try to merge virtually and validate.
	/// Emits warnings and errors, returns errors-chain.
	pub fn merge_all_virtually<'cfg>(cfg: &Config<'cfg>,
	                                 tree: &MetaDeps<'cfg>,
	                                 plans: &AssetsPlans<'cfg>)
	                                 -> CargoResult<()> {
		// prepare context:
		let mut root_package: HashMap<&MultiKey, HashSet<&PackageId>> = HashMap::with_capacity(tree.roots().len());
		let mut root_options: HashMap<&PackageId, AssetsOptions> = HashMap::with_capacity(tree.roots().len());

		plans.targets
		     .iter()
		     .flat_map(|(key, _)| {
			     tree.roots()
			         .into_iter()
			         .filter(|r| key.is_for(r))
			         .map(move |r| (key, r))
		     })
		     .for_each(|(key, root)| {
			     root_package.entry(key)
			                 .or_insert_with(|| HashSet::with_capacity(tree.roots().len()))
			                 .insert(root.package_id());

			     if !root_options.contains_key(root.package_id()) {
				     let options = root.as_source().assets_options();
				     root_options.insert(root.package_id(), options);
			     }
		     });


		// Buffered errors:
		let mut overrides = Vec::new();


		// merge, analyse:
		for (key, index) in plans.targets.iter() {
			// Note, correct ordering in `index` guaranteed by the planner.

			// Need merge many into one:
			let _many = index.len() > 1;

			for root_id in root_package.get(key).into_iter().flat_map(|set| set.iter()) {
				use playdate::assets::plan::*;

				log::trace!("v-merging for {} (dev:{})", root_id.name(), key.dev);

				let options = &root_options[root_id];

				let mut _plan: Vec<Mapping> = Default::default();


				let mut targets = BTreeMap::new();
				let mut sources = BTreeMap::new();


				for i in index {
					let next = &plans.plans[*i];

					for (kind, dst, src) in next.iter_flatten() {
						let target: Cow<_> = match kind {
							                     MappingKind::AsIs | MappingKind::ManyInto => dst,
						                        MappingKind::Into => dst.join(src.file_name().expect("filename")),
						                     }.into();

						// Save for future check if we already have this source:
						sources.entry(Cow::from(src))
						       .or_insert_with(|| Vec::with_capacity(2))
						       .push(*i);

						// Check if we already have this target:
						targets.entry(target.clone())
						       .or_insert_with(|| Vec::with_capacity(2))
						       .push(*i);

						if let Some(past) = targets.get(&target) &&
						   past.len() > 1
						{
							let id = past.into_iter()
							             .flat_map(|x| plans.index.iter().find_map(|(key, i)| (i == x).then_some(key)))
							             .collect::<Vec<_>>();
							debug_assert!(!id.is_empty());

							let this = id.last().unwrap();
							let other = &id[..id.len() - 1];

							let dev = this.dev.then_some("dev-").unwrap_or_default();
							let others = other.is_empty()
							                  .then_some("by itself")
							                  .map(Cow::from)
							                  .unwrap_or_else(|| {
								                  other.into_iter()
								                       .map(|k| {
									                       let dev = k.dev.then_some("dev-").unwrap_or_default();
									                       format!("{}'s '{dev}assets'", k.id.name())
								                       })
								                       .collect::<Vec<_>>()
								                       .join(", ")
								                       .into()
							                  });

							let name = this.id.name();
							let root_name = root_id.name();
							let why = format!("but that's not allowed by the top-level crate {root_name}");
							let msg = format!("{name}'s `{dev}assets.{target:?}` overrides {others}, {why}");

							if options.overwrite {
								cfg.log().warn(msg)
							} else {
								cfg.log().error(&msg);
								overrides.push(msg);
							}
						}
					}
				}

				// Check if we already have this source:
				for (src, index) in sources {
					if index.len() < 2 {
						continue;
					}

					let id = index.into_iter()
					              .flat_map(|x| plans.index.iter().find_map(|(key, i)| (*i == x).then_some(key)))
					              .collect::<Vec<_>>();
					debug_assert!(!id.is_empty());

					let src_rel = src.as_relative_to_root(cfg);
					let others = id.is_empty()
					               .then_some("itself")
					               .map(Cow::from)
					               .unwrap_or_else(|| {
						               id.into_iter()
						                 .map(|k| {
							                 let dev = k.dev.then_some("dev-").unwrap_or_default();
							                 format!("{}'s '{dev}assets'", k.id.name())
						                 })
						                 .collect::<Vec<_>>()
						                 .join(", ")
						                 .into()
					               });
					let msg = format!("asset {src_rel:?} used multiple times in {others}");
					cfg.log().warn(msg);
				}
			}
		}


		{
			use err::Override;
			use anyhow::Error;
			overrides.is_empty()
			         .then_some(())
			         .ok_or_else(|| overrides.into_iter().fold(Error::new(Override), Error::context))
		}
	}


	mod err {
		#[derive(Debug)]
		pub struct Override;
		impl std::error::Error for Override {}
		impl std::fmt::Display for Override {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { "Not allowed override".fmt(f) }
		}
	}
}


/// Returns `None` if there is no `assets` metadata.
pub fn plan_for<'cfg, 'env, 'l>(config: &'cfg Config,
                                package: &'cfg Package,
                                metadata: &Metadata,
                                env: &'cfg LazyEnvBuilder<'env, 'cfg>,
                                layout: &'l LockedLayout<'l>,
                                with_dev: bool)
                                -> CargoResult<PackageAssetsPlan<'env, 'cfg>> {
	let opts = metadata.assets_options();

	let has_dev_assets = with_dev && !metadata.dev_assets().is_empty();
	let is_empty = metadata.assets().is_empty() && !has_dev_assets;

	if is_empty {
		return Ok(PackageAssetsPlan { main: None,
		                              dev: None });
	}

	let env = env.get()?;
	let root = package.manifest_path()
	                  .parent()
	                  .ok_or(anyhow!("No parent of manifest-path"))?;

	let main = if !metadata.assets().is_empty() {
		let plan = assets_build_plan(env, metadata.assets(), opts.as_ref(), Some(root.into()))?;

		// main-assets plan:
		let path = layout.as_inner().assets_plan_for(config, &package.package_id());
		let mut cached = CachedPlan::new(path, plan)?;
		if config.compile_options.build_config.force_rebuild {
			cached.difference = Difference::Missing;
		}

		Some(cached)
	} else {
		None
	};


	// dev-assets plan:
	let dev = if has_dev_assets && !metadata.dev_assets().is_empty() {
		let assets = metadata.dev_assets();
		let dev_plan = assets_build_plan(env, assets, opts.as_ref(), Some(root.into()))?;

		let path = layout.as_inner()
		                 .assets_plan_for_dev(config, &package.package_id());
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
		let mut serializable = plan.iter_flatten_meta().collect::<Vec<_>>();
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


	pub fn printable_serializable(&self, source: PackageId, kind: AssetKind) -> SerializablePlan<'_, 't, 'cfg> {
		SerializablePlan { package: source,
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
