use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::{PathBuf, Path};

use cargo::CargoResult;
use cargo::core::{PackageId, Verbosity};
use anstyle::AnsiColor as Color;
use anyhow::bail;

use playdate::assets::plan::BuildPlan;
use playdate::assets::BuildReport;
use playdate::layout::Layout as _;
use playdate::metadata::format::AssetsOptions;

use crate::config::Config;
use crate::utils::path::AsRelativeTo;
use crate::utils::cargo::meta_deps::{MetaDeps, RootNode};
use crate::layout::{LayoutLockable, CrossTargetLayout};
use crate::layout::{PlaydateAssets, Layout};


mod pdc;
mod plan;
mod cache;


#[derive(Debug)]
pub struct Artifact {
	pub package_id: PackageId,
	pub layout: PlaydateAssets<PathBuf>,
	pub kind: Kind,
}


pub struct Artifacts<'t, 'cfg> {
	artifacts: Vec<Artifact>,
	index: BTreeMap<plan::RootKey, Vec<usize>>,
	pub tree: &'t MetaDeps<'cfg>,
}


impl Artifacts<'_, '_> {
	pub fn len(&self) -> usize { self.artifacts.len() }
	pub fn artifacts(&self) -> &[Artifact] { &self.artifacts }
	pub fn index(&self) -> &BTreeMap<plan::RootKey, Vec<usize>> { &self.index }

	pub fn iter(&self) -> impl Iterator<Item = (&RootNode, impl Iterator<Item = &Artifact>)> {
		self.index
		    .iter()
		    .flat_map(|(key, index)| {
			    self.tree
			        .roots()
			        .iter()
			        .filter(|r| key.is_for(r))
			        .map(|root| (root, index.as_slice()))
		    })
		    .map(|(root, index)| {
			    let arts = index.iter().map(|i| &self.artifacts[*i]);
			    (root, arts)
		    })
	}
}

impl core::fmt::Debug for Artifacts<'_, '_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("AssetsArtifacts")
		 .field_with("artifacts", |f| {
			 self.artifacts
			     .iter()
			     .enumerate()
			     .collect::<BTreeMap<_, _>>()
			     .fmt(f)
		 })
		 .field("index", &self.index)
		 .finish_non_exhaustive()
	}
}


#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum Kind {
	Package,
	Dev,
}

impl Kind {
	pub fn is_dev(&self) -> bool { matches!(self, Self::Dev) }
}


pub fn build_all<'t, 'cfg>(cfg: &Config<'cfg>, tree: &'t MetaDeps<'cfg>) -> CargoResult<Artifacts<'t, 'cfg>> {
	// planning:
	let plans = plan::plan_all(cfg, tree)?;

	// validation:
	if let Err(err) = plan::merge_all_virtually(cfg, tree, &plans) &&
	   !cfg.compile_options.build_config.keep_going
	{
		return Err(err.context("Assets validation failed"));
	}

	// results:
	let mut artifacts = Artifacts { artifacts: Vec::with_capacity(plans.plans.len()),
	                                index: Default::default(),
	                                tree };


	// checking cache, apply each plan:
	for (index, plan) in plans.plans.into_iter().enumerate() {
		let key = plans.index
		               .iter()
		               .find_map(|(k, i)| (*i == index).then_some(k))
		               .ok_or_else(|| anyhow::anyhow!("No assets-plan key in plan #{index}"))?;

		log::debug!("#{index} build (dev:{}) {}", key.dev, key.id);


		let global_layout = CrossTargetLayout::new(cfg, key.id, None)?;
		let mut layout = global_layout.assets_layout(cfg);


		// clean layout if needed:
		if !cfg.dry_run && cfg.compile_options.build_config.force_rebuild {
			if !matches!(cfg.workspace.gctx().shell().verbosity(), Verbosity::Quiet) {
				cfg.log().status("Clean", format!("assets for {}", key.id));
			}
			layout.clean()?;
		}


		let mut locked = layout.lock_mut(cfg.workspace.gctx())?;
		locked.prepare()?;

		// path of build-plan file:
		let path = if key.dev {
			locked.as_inner().assets_plan_for_dev(cfg, &key.id)
		} else {
			locked.as_inner().assets_plan_for(cfg, &key.id)
		};

		let mut cache = cache::plan_cache(path, &plan)?;
		if cfg.compile_options.build_config.force_rebuild {
			cache.difference = cache::Difference::Missing;
		}


		let dest = if key.dev {
			locked.as_inner().assets_dev()
		} else {
			locked.as_inner().assets()
		};


		// kind of assets just for log:
		let kind_prefix = key.dev.then_some("dev-").unwrap_or_default();
		// this one for assets:
		let kind = if key.dev { Kind::Dev } else { Kind::Package };


		// build if needed:
		if cache.difference.is_same() {
			cfg.log().status(
			                 "Skip",
			                 format!(
				"{} {kind_prefix}assets cache state is {:?}",
				key.id, &cache.difference
			),
			);
		} else {
			cfg.log()
			   .status("Build", format!("{kind_prefix}assets for {}", key.id));
			cfg.log().verbose(|mut log| {
				         let dep_root = plan.crate_root();
				         let dest = format!("destination: {:?}", dest.as_relative_to_root(cfg));
				         log.status("", dest);
				         let src = format!("root: {:?}", dep_root.as_relative_to_root(cfg));
				         log.status("", src);
			         });


			// Since we build each plan separately independently, the default options are sufficient.
			// The options are needed further when merging assets into a package.
			let dep_opts = Default::default();
			let report = apply(cache, plan, &dest, &dep_opts, cfg)?;


			// print report:
			for (x, (m, results)) in report.results.iter().enumerate() {
				let results = results.iter().enumerate();
				let expr = m.exprs();
				let incs = m.sources();

				for (y, res) in results {
					let path = incs[y].target();
					let path = path.as_relative_to_root(cfg);
					match res {
						Ok(op) => {
							cfg.log().verbose(|mut log| {
								         let msg = format!("asset [{x}:{y}] {}", path.display());
								         log.status(format!("{op:?}"), msg)
							         })
						},
						Err(err) => {
							use fs_extra::error::ErrorKind as FsExtraErrorKind;

							let error = match &err.kind {
								FsExtraErrorKind::Io(err) => format!("IO: {err}"),
								FsExtraErrorKind::StripPrefix(err) => format!("StripPrefix: {err}"),
								FsExtraErrorKind::OsString(err) => format!("OsString: {err:?}"),
								_ => err.to_string(),
							};
							let message = format!(
							                      "Asset [{x}:{y}], rule: '{} <- {} | {}', {error}",
							                      expr.0.original(),
							                      expr.1.original(),
							                      path.display()
							);

							cfg.log().status_with_color("Error", message, Color::Red)
						},
					};
				}
			}

			// TODO: log report.exclusions

			if report.has_errors() && !cfg.compile_options.build_config.keep_going {
				use anyhow::Error;

				#[derive(Debug)]
				pub struct Mapping(String);
				impl std::error::Error for Mapping {}
				impl std::fmt::Display for Mapping {
					fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
				}

				let err = report.results
				                .into_iter()
				                .filter_map(|(map, res)| {
					                if res.iter().any(|res| res.is_err()) {
						                let err = Mapping(map.pretty_print_compact());
						                let err = res.into_iter()
						                             .filter_map(|res| res.err())
						                             .fold(Error::new(err), Error::context);
						                Some(err)
					                } else {
						                None
					                }
				                })
				                .fold(Error::msg("Assets build failed"), Error::context);
				return Err(err);
			}


			// finally build with pdc:
			// if not disallowed explicitly
			if cfg.skip_prebuild {
				const REASON: &str = "as requested";
				let msg = format!("{kind_prefix}assets pre-build for {}, {REASON}.", key.id);
				cfg.log().status("Skip", msg);
			} else {
				match pdc::build(cfg, &key.id, locked.as_inner(), kind) {
					Ok(_) => {
						let msg = format!("{kind_prefix}assets for {}", key.id);
						cfg.log().status("Finished", msg);
					},
					Err(err) => {
						let msg = format!("build {kind_prefix}assets with pdc failed: {err}");
						cfg.log().status_with_color("Error", msg, Color::Red);
						if !cfg.compile_options.build_config.keep_going {
							bail!("Assets build failed.");
						}
					},
				}
			}
		}


		// Finale:

		locked.unlock();


		let art_index = artifacts.artifacts.len();
		artifacts.artifacts.push(Artifact { kind,
		                                    package_id: key.id,
		                                    layout: layout.clone() });

		log::debug!(
		            "Assets artifact for {} at {:?}",
		            key.id,
		            crate::layout::Layout::dest(&layout).as_relative_to_root(cfg)
		);

		for (r_key, index) in plans.targets.iter().filter(|(_, i)| i.contains(&index)) {
			artifacts.index
			         .entry(r_key.to_owned())
			         .or_insert(Vec::with_capacity(index.len()))
			         .push(art_index);
		}
	}


	cfg.log_extra_verbose(|mut logger| {
		   artifacts.iter().for_each(|(root, arts)| {
			                   use cargo::core::compiler::CompileKind;
			                   let ct: Cow<_> = match root.node().unit().platform {
				                   CompileKind::Host => "host".into(),
			                      CompileKind::Target(kind) => kind.short_name().to_owned().into(),
			                   };

			                   let root = format!(
			                                      "{} {} of {} for {ct}",
			                                      root.node().target().kind().description(),
			                                      root.node().target().name,
			                                      root.node().package_id().name(),
			);
			                   logger.status("Assets", format!("artifacts for {root}:"));
			                   arts.for_each(|art| {
				                       let dest = match art.kind {
					                       Kind::Package => art.layout.assets(),
				                          Kind::Dev => art.layout.assets_dev(),
				                       };
				                       let msg = format!(
				                                         "[{:?}] {} - {:?}",
				                                         art.kind,
				                                         art.package_id.name(),
				                                         dest.as_relative_to_root(cfg)
				);
				                       logger.status("", msg);
			                       });
		                   });
	   });

	Ok(artifacts)
}


fn apply<'l, 'r>(cache: cache::PlanCache,
                 plan: BuildPlan<'l, 'r>,
                 dest: &Path,
                 options: &AssetsOptions,
                 config: &Config)
                 -> CargoResult<BuildReport<'l, 'r>> {
	use crate::playdate::assets::apply_build_plan;

	let report = apply_build_plan(plan, dest, options)?;
	// and finally save cache of just successfully applied plan:
	// only if there is no errors
	if !report.has_errors() {
		if let Some(data) = cache.serialized.as_deref() {
			log::trace!("writing cache to {:?}", cache.path);
			std::fs::write(&cache.path, data)?;
			config.log().verbose(|mut log| {
				            let path = cache.path.as_relative_to_root(config);
				            log.status("Cache", format_args!("saved to {}", path.display()));
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
