use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{PathBuf, Path};

use anstyle::AnsiColor as Color;
use anyhow::bail;
use cargo::CargoResult;
use cargo::core::{Package, PackageId, Verbosity};
use playdate::manifest::ManifestSourceOpt as _;
use playdate::metadata::source::MetadataSource as _;
use playdate::metadata::METADATA_FIELD;
use playdate::layout::Layout;

use crate::assets::plan::{AssetKind, CachedPlan};
use crate::config::Config;
use crate::layout::{PlaydateAssets, LayoutLockable, Layout as _, CrossTargetLayout};
use crate::logger::LogErr;
use crate::utils::LazyBuildContext;
use crate::utils::path::AsRelativeTo;
use self::plan::Metadata;


mod plan;
mod pdc;


#[derive(Debug)]
pub struct AssetsArtifact {
	pub package_id: PackageId,
	pub layout: PlaydateAssets<PathBuf>,
	/// Cached metadata
	pub metadata: Option<Metadata>,
}

/// One artifact per package.
pub type AssetsArtifacts<'cfg> = HashMap<&'cfg Package, AssetsArtifact>;


pub fn build<'cfg>(config: &'cfg Config) -> CargoResult<AssetsArtifacts<'cfg>> {
	let bcx = LazyBuildContext::new(config)?;
	let mut artifacts = AssetsArtifacts::new();

	for (package, targets, ..) in config.possible_targets()? {
		let env = plan::LazyEnvBuilder::new(config, package);
		let mut plans: HashMap<&Package, _> = Default::default();
		let global_layout = CrossTargetLayout::new(config, package.package_id(), None)?;
		let mut layout = global_layout.assets_layout(config);
		let mut options = HashMap::new();

		if !config.dry_run && config.compile_options.build_config.force_rebuild {
			if !matches!(config.workspace.config().shell().verbosity(), Verbosity::Quiet) {
				config.log()
				      .status("Clean", format!("assets for {}", package.package_id()));
			}
			layout.clean()?;
		}

		// primary top-level package
		let target_pid = package.package_id();
		let has_dev = targets.iter()
		                     .any(|t| t.is_example() || t.is_test() || t.is_bench());


		log::debug!("Inspecting dependencies tree for {}", package.package_id());
		let packages = deps_tree_metadata(package, &bcx, config)?;


		// TODO: list deps in the plan

		for (package, metadata) in packages {
			let locked = layout.lock_mut(config.workspace.config())?;
			let dev = has_dev && package.package_id() == target_pid;
			let err_msg = |err| format!("{err}, caused when planning assets for {}.", package.package_id());

			match plan::plan_for(config, package, &metadata, &env, &locked, dev) {
				// nothing to pack:
				Ok(plan) if plan.is_empty() => {
					config.log()
					      .verbose(|mut log| log.status("Skip", format!("{} without plan", package.package_id())))
					// TODO: add clean assets task for `package`/`kind`
					// Also remove old build-plan.
					// Here and below for error case.
				},

				// report and continue:
				Err(err) if config.compile_options.build_config.keep_going => {
					let msg = format!("{} Continuing because `keep-going` is set.", err_msg(&err));
					config.log().error(msg)
				},

				// abort:
				Err(err) => {
					config.log().error(err_msg(&err));
					return Err(err);
				},

				// add plan to pack:
				Ok(plan) => {
					// TODO: Check main/dev is empty and add clean assets task for `package`/`kind`
					// Also remove old build-plan.

					options.insert(package, metadata);
					plans.insert(package, plan);
				},
			}
		}

		// report if needed:
		if config.compile_options.build_config.emit_json() || config.compile_options.build_config.build_plan {
			for (package, plan) in plans.iter() {
				for (plan, kind) in plan.main
				                        .as_ref()
				                        .into_iter()
				                        .map(|plan| (plan, AssetKind::Package))
				                        .chain(plan.dev.as_ref().into_iter().map(|plan| (plan, AssetKind::Dev)))
				{
					let message = plan.printable_serializable(package.package_id(), kind);
					config.workspace.config().shell().print_json(&message)?;
				}
			}
		} else {
			config.workspace
			      .config()
			      .shell()
			      .verbose(|shell| {
				      for (package, plan) in plans.iter() {
					      for plan in plan.main
					                      .as_ref()
					                      .into_iter()
					                      .chain(plan.dev.as_ref().into_iter())
					      {
						      shell.status("Assets", format!("build plan for {}", package.package_id()))?;
						      plan.pretty_print(shell, config.workspace.root())?;
					      }
				      }
				      Ok(())
			      })
			      .log_err()
			      .ok();
		}

		/* NOTE for future: how to resolve conflicts better:
			- merge all plans, where
			- resolve conflicts as it happening in the `build_plan()::re-mapping`:
				e.g.: Mapping::* -> Mapping::ManyInto
		*/

		{
			// validate plans:
			let mut has_errors = false;
			let mut targets = HashMap::new();

			let mut check_duplicates = |package_id: PackageId, target_kind: AssetKind, plan| {
				for target in plan {
					if let Some((pid, kind)) = targets.get::<Cow<Path>>(&target) {
						has_errors = true;
						let err_msg = |pid, kind| {
							match kind {
								AssetKind::Package => format!("{pid} in [assets]"),
								AssetKind::Dev => format!("{pid} in [dev-assets]"),
							}
						};
						let a = err_msg(pid, *kind);
						let b = err_msg(&package_id, target_kind);
						let message = format!(
						                      "Duplicate dev-asset destination: '{}':\n\t{a}\n\t{b}",
						                      target.as_relative_to_root(config).display(),
						);

						config.log().error(message);
					} else {
						targets.insert(target, (package_id, target_kind));
					}
				}
			};


			for (package, plan) in plans.iter() {
				let package_id = package.package_id();
				if let Some(plan) = plan.main.as_ref() {
					check_duplicates(package_id, AssetKind::Package, plan.as_inner().targets());
				}
				if package_id == target_pid {
					if let Some(plan) = plan.dev.as_ref() {
						check_duplicates(package_id, AssetKind::Dev, plan.as_inner().targets());
					}
				}
			}

			if has_errors {
				if config.compile_options.build_config.keep_going {
					config.log()
					      .status("Drop", format!("assets for {}", package.package_id()));
					continue;
				} else {
					bail!("Duplicated asset destination, stopping compilation.");
				}
			}

			// TODO: Also check sources duplicates, but only warn.
		}


		// finally apply plans:
		if !config.dry_run && !config.compile_options.build_config.build_plan && !plans.is_empty() {
			let mut locked = layout.lock_mut(config.workspace.config())?;
			locked.prepare()?;

			for (dependency, mut plan) in plans.into_iter() {
				let dep_pkg_id = dependency.package_id();

				let apply = |plan: CachedPlan, kind| -> CargoResult<()> {
					let dest = match kind {
						AssetKind::Package => locked.as_inner().assets(),
						AssetKind::Dev => locked.as_inner().assets_dev(),
					};
					let kind_prefix = match kind {
						AssetKind::Package => "",
						AssetKind::Dev => "dev-",
					};

					let dep_root = dependency.manifest_path().parent().unwrap();

					config.log()
					      .status("Build", format!("{kind_prefix}assets for {}", dep_pkg_id));
					config.log().verbose(|mut log| {
						            let dest = format!("destination: {}", dest.as_relative_to_root(config).display());
						            log.status("", dest);
						            let src = format!("root {}", dep_root.as_relative_to_root(config).display());
						            log.status("", src);
						            if dep_root != plan.path {
							            let path = plan.plan.crate_root();
							            let src = format!("root (plan) {}", path.as_relative_to_root(config).display());
							            log.status("", src);
						            }
					            });


					// FIXME: use primary (top-level) assets-options, but not options of dependency!
					let metadata = options.get(dependency).expect("Metadata is gone, impossible!");
					let report = plan.apply(&dest, &metadata.assets_options(), config)?;


					// print report:
					for (x, (m, results)) in report.results.iter().enumerate() {
						let results = results.iter().enumerate();
						let expr = m.exprs();
						let incs = m.sources();

						for (y, res) in results {
							let path = incs[y].target();
							let path = path.as_relative_to_root(config);
							match res {
								Ok(op) => {
									config.log().verbose(|mut log| {
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

									config.log().status_with_color("Error", message, Color::Red)
								},
							};
						}
					}

					if report.has_errors() && !config.compile_options.build_config.keep_going {
						bail!("Assets build failed.");
					}


					// finally build with pdc:
					// if not disallowed explicitly
					if config.skip_prebuild {
						const REASON: &str = "as requested";
						let msg = format!("{kind_prefix}assets pre-build for {}, {REASON}.", dep_pkg_id);
						config.log().status("Skip", msg);
					} else {
						match pdc::build(config, dependency, locked.as_inner(), kind) {
							Ok(_) => {
								let msg = format!("{kind_prefix}assets for {}", dep_pkg_id);
								config.log().status("Finished", msg);
							},
							Err(err) => {
								let msg = format!("build {kind_prefix}assets with pdc failed: {err}");
								config.log().status_with_color("Error", msg, Color::Red);
								if !config.compile_options.build_config.keep_going {
									bail!("Assets build failed.");
								}
							},
						}
					}

					Ok(())
				};

				// main:
				let mut main_cache_hit = false;
				if dep_pkg_id == target_pid {
					if let Some(plan) = plan.main.take() {
						if plan.difference.is_same() {
							config.log().status(
							                    "Skip",
							                    format!("{}, cache state is {:?}", dep_pkg_id, &plan.difference),
							);
							main_cache_hit = true;
							// continue;
						}

						apply(plan, AssetKind::Package)?;
					}
				}

				// dev:
				if dep_pkg_id == target_pid {
					if let Some(plan) = plan.dev.take() {
						if main_cache_hit && plan.difference.is_same() {
							config.log().status(
							                    "Skip",
							                    format!("{} (dev), cache state is {:?}", dep_pkg_id, &plan.difference),
							);
							continue;
						}

						apply(plan, AssetKind::Dev)?;
					}
				}
			}

			locked.unlock();

			log::debug!(
			            "Assets artifact for {} at {}",
			            package.package_id(),
			            crate::layout::Layout::dest(&layout).as_relative_to_root(config)
			                                                .display()
			);
			let metadata = options.remove(package);
			artifacts.insert(
			                 package,
			                 AssetsArtifact { package_id: package.package_id(),
			                                  layout,
			                                  metadata, },
			);
		}
	}


	if config.compile_options.build_config.build_plan {
		config.workspace.config().shell().out().flush()?;
		config.workspace.config().shell().err().flush()?;
		std::process::exit(0);
	}

	Ok(artifacts)
}


fn deps_tree_metadata<'cfg: 'r, 't: 'r, 'r>(package: &'cfg Package,
                                            bcx: &'t LazyBuildContext<'t, 'cfg>,
                                            config: &Config<'_>)
                                            -> CargoResult<HashMap<&'r Package, Metadata>> {
	let mut packages = HashMap::new();
	if let Some(metadata) = playdate_metadata(package) {
		// if explicitly allowed collect deps => scan deps-tree
		if metadata.assets_options().dependencies {
			log::debug!("inspecting deps-tree of {}", package.package_id());

			packages.insert(package, metadata);

			let bcx = bcx.get()?;

			// TODO: Cache hash of bcx.unit_graph in the assets-build-plan

			// find this package in roots:
			let root = bcx.unit_graph
			              .keys()
			              .find(|u| u.pkg.package_id() == package.package_id());
			let mut dependencies = HashSet::new();

			if let Some(deps) = root.and_then(|root| bcx.unit_graph.get(root)) {
				// find all dependencies:
				dependencies.extend(
				                    deps.iter()
				                        .flat_map(|ud| bcx.unit_graph.get_key_value(&ud.unit).map(|(u, _)| u)),
				);


				let mut last_length = 0;
				while last_length != dependencies.len() {
					let pre_last_length = dependencies.len();

					let next = dependencies.iter()
					                       .flat_map(|u| {
						                       bcx.unit_graph.get(u).into_iter().flat_map(|deps| {
							                                                        deps.iter().flat_map(|ud| {
								                                                                   bcx.unit_graph
								                                                                           .get_key_value(&ud.unit)
								                                                                           .map(|(u, _)| u)
							                                                                   })
						                                                        })
					                       })
					                       .filter(|u| !dependencies.contains(u))
					                       .collect::<Vec<_>>();
					dependencies.extend(next);
					last_length = pre_last_length;
				}

				// dedup dependencies, choose only highest versions
				let mut id_ver = dependencies.iter()
				                             .filter(|u| u.pkg.package_id() != package.package_id())
				                             .filter(|u| u.pkg.manifest().custom_metadata().is_some())
				                             .map(|u| {
					                             let name = u.pkg.name();
					                             let source_id = u.pkg.package_id().source_id();
					                             let versions = dependencies.iter()
					                                                        .filter(|u| {
						                                                        u.pkg.name() == name &&
						                                                        u.pkg.package_id().source_id() ==
						                                                        source_id
					                                                        })
					                                                        .map(|u| (u.pkg.version(), *u))
					                                                        .collect::<Vec<_>>();
					                             ((name, source_id), versions)
				                             })
				                             .collect::<HashMap<_, _>>();

				id_ver.values_mut().for_each(|vers| {
					                   vers.sort_by_key(|(v, _)| *v);
					                   vers.dedup_by_key(|(v, _)| *v);
				                   });


				dependencies.retain(|u| {
					            let key = (u.pkg.name(), u.pkg.package_id().source_id());
					            id_ver.get(&key)
					                  .and_then(|vec| vec.last())
					                  .map(|(_, u)| *u)
					                  .filter(|u1| u == u1)
					                  .is_some()
				            });
			}


			let with_meta = dependencies.into_iter()
			                            .inspect(|u| {
				                            config.log().verbose(|mut log| {
					                                        log.status("Check", format!("{}", u.pkg.package_id()))
				                                        })
			                            })
			                            .filter_map(|u| playdate_metadata(&u.pkg).map(|m| (&u.pkg, m)));


			packages.extend(with_meta);
		} else {
			packages.insert(package, metadata);
		}
	}
	Ok(packages)
}


pub fn playdate_metadata(package: &Package) -> Option<Metadata> {
	package.manifest()
	       .custom_metadata()
	       .and_then(|m| m.as_table().map(|t| t.get(METADATA_FIELD)))
	       .flatten()
	       .and_then(|v| v.to_owned().try_into::<Metadata>().log_err().ok())
}
