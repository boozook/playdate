use std::path::Path;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};

use cargo::core::PackageId;
use cargo::util::CargoResult;

use playdate::assets::plan::build_plan;
use playdate::assets::plan::BuildPlan;
use playdate::consts::SDK_ENV_VAR;
use playdate::manifest::PackageSource as _;
use playdate::metadata::format::AssetsOptions;
use playdate::metadata::source::MetadataSource as _;

use crate::utils::cargo::meta_deps::MetaDeps;
use crate::utils::cargo::meta_deps::{Node, RootNode};
use crate::utils::path::AsRelativeTo;
use crate::config::Config;


pub struct AssetsPlans<'cfg> {
	pub plans: Vec<BuildPlan<'cfg, 'cfg>>,
	/// per-dep ?dev plan
	pub index: BTreeMap<Key, usize>,
	/// per-root plans to merge
	pub targets: BTreeMap<RootKey, Vec<usize>>,
}

/// Target-agnostic package key.
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
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


/// Target-agnostic root-package key with all dependencies.
#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct RootKey {
	/// Dependencies
	id: Vec<PackageId>,
	/// Primary target is dev
	dev: bool,
}
impl From<&'_ RootNode<'_>> for RootKey {
	fn from(root: &'_ RootNode<'_>) -> Self {
		Self { dev: root.node().target().is_dev(),
		       id: root.deps().iter().map(|d| d.package_id().to_owned()).collect() }
	}
}
impl RootKey {
	pub fn dev(&self) -> bool { self.dev }

	pub fn is_for(&self, root: &'_ RootNode<'_>) -> bool {
		root.node().target().is_dev() == self.dev &&
		root.deps()
		    .iter()
		    .enumerate()
		    .all(|(i, d)| self.id.get(i).filter(|k| *k == d.package_id()).is_some())
	}
}


pub fn plan_all<'cfg>(cfg: &Config<'cfg>, tree: &MetaDeps<'cfg>) -> CargoResult<AssetsPlans<'cfg>> {
	// results:
	let mut plans = AssetsPlans { plans: Vec::with_capacity(tree.roots().len() * 2),
	                              index: BTreeMap::new(),
	                              targets: BTreeMap::new() };

	// prepare env:
	let global_env: BTreeMap<_, _> =
		std::env::vars().chain({
			                cfg.sdk()
			                   .map(|sdk| sdk.path())
			                   .ok()
			                   .or(cfg.sdk_path.as_deref())
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
		                     .chain(vars);

		Env::try_from_iter(iter).map_err(|err| anyhow::anyhow!("{err}"))
	};


	// use target-agnostic selection of roots:
	for root in tree.roots_compile_target_agnostic() {
		let meta_source = root.as_source();

		let options = meta_source.assets_options();

		let root_is_dev = root.node().target().is_dev();

		log::debug!(
		            "planning for {} ({}) +dev:{root_is_dev}",
		            root.package_id(),
		            root.node().target().kind().description()
		);
		log::debug!("  dependencies are allowed: {}", options.dependencies());


		let plan_key = RootKey::from(root);
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

			// dep_key is dev only if this it a primary target (root unit) and dev is requested:
			let with_dev = root_is_dev && dep.package_id() == root.package_id();
			let dep_key = Key::from(dep).with_dev(with_dev);


			let plan_for = |plans: &mut AssetsPlans, indices: &mut Vec<usize>, key: Key| -> anyhow::Result<()> {
				let source = dep.as_source();
				let dev_prefix = key.dev.then_some("dev-").unwrap_or_default();
				if let Some(assets) = source.metadata()
				                            .map(|m| if key.dev { m.dev_assets() } else { m.assets() }) &&
				   !assets.is_empty()
				{
					match build_plan(&env, assets, &options, Some(crate_root.into())) {
						Ok(plan) => {
							let pid = key.id;
							let is_dev = key.dev;
							let dev_index = plans.plans.len();
							let compile_target_agnostic = plan.compile_target_agnostic();
							plans.index.insert(key, dev_index);
							plans.plans.push(plan);
							indices.push(dev_index);

							log::debug!("    done: +#{dev_index} (dev:{is_dev})");
							cfg.log().verbose(|mut log| {
								         log.status("Plan", format_args!("{dev_prefix}assets for {pid} planned"))
							         });

							if !compile_target_agnostic {
								cfg.log()
								   .error("Assets is not compile-target-agnostic, this is not supported");
							}
						},
						Err(err) => {
							cfg.log()
							   .error(format_args!("{err}, caused when planning {dev_prefix}assets for {}", key.id));
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
							"{dev_prefix}assets for {} without plan, reason: empty",
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
				log::debug!("    done (~#{base_index}) (dev:{})", false);

				plan_for(&mut plans, &mut indices, dep_key.with_dev(true))?;
			} else {
				// else just build a plan
				plan_for(&mut plans, &mut indices, dep_key.with_dev(false))?;

				// also for dev targets if needed
				if with_dev {
					plan_for(&mut plans, &mut indices, dep_key.with_dev(true))?;
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
		let key = RootKey::from(root);
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
	let mut root_package: HashMap<&RootKey, HashSet<&PackageId>> = HashMap::with_capacity(tree.roots().len());
	let mut root_options: HashMap<&PackageId, AssetsOptions> = HashMap::with_capacity(tree.roots().len());

	plans.targets
	     .iter()
	     .flat_map(|(key, _)| {
		     tree.roots()
		         .iter()
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
						let id = past.iter()
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
							                  other.iter()
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
						let msg = format!("{name}'s `{dev}assets.{target:?}` overrides {others}");

						if options.overwrite() {
							cfg.log().warn(msg)
						} else {
							let why = format!("but that's not allowed by the top-level crate {root_name}");
							cfg.log().error(format_args!("{msg}, {why}"));
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
