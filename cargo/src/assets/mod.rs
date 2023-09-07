use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::bail;
use cargo::CargoResult;
use cargo::core::{Package, Verbosity};
use playdate::metadata::METADATA_FIELD;
use playdate::layout::Layout;

use crate::config::Config;
use crate::layout::{PlaydateAssets, LayoutLockable, Layout as _, CrossTargetLayout};
use crate::logger::LogErr;
use crate::utils::LazyBuildContext;
use crate::utils::path::AsRelativeTo;
use self::plan::TomlMetadata;


mod plan;
mod pdc;


#[derive(Debug)]
pub struct AssetsArtifact<'cfg> {
	pub package: &'cfg Package,
	pub layout: PlaydateAssets<PathBuf>,
	/// Cached metadata
	pub metadata: Option<TomlMetadata>,
}

/// One artifact per package.
pub type AssetsArtifacts<'cfg> = HashMap<&'cfg Package, AssetsArtifact<'cfg>>;


pub fn build<'cfg>(config: &'cfg Config) -> CargoResult<AssetsArtifacts<'cfg>> {
	let bcx = LazyBuildContext::new(&config)?;
	let mut artifacts = AssetsArtifacts::new();

	for (package, ..) in config.possible_targets()? {
		let env = plan::LazyEnvBuilder::new(config, package);
		let mut plans: HashMap<&Package, _> = Default::default();
		let global_layout = CrossTargetLayout::new(config, package, None)?;
		let mut layout = global_layout.assets_layout(config);
		let mut options = HashMap::new();

		if !config.dry_run && config.compile_options.build_config.force_rebuild {
			if !matches!(config.workspace.config().shell().verbosity(), Verbosity::Quiet) {
				config.log()
				      .status("Clean", format!("assets for {}", package.package_id()));
			}
			layout.clean()?;
		}


		log::debug!("Inspecting dependencies tree for {}", package.package_id());
		let packages = deps_tree_metadata(package, &bcx, config)?;

		// TODO: mb. instead of this, merge metadata into one?

		for (package, metadata) in packages {
			let locked = layout.lock_mut(config.workspace.config())?;

			if let Some(plan) = plan::plan_for(config, package, &metadata, &env, &locked)? {
				options.insert(package, metadata);
				plans.insert(package, plan);
			} else {
				config.log()
				      .verbose(|mut log| log.status("Skip", format!("{} without plan", package.package_id())));
			}
		}

		// report if needed:
		if config.compile_options.build_config.emit_json() || config.compile_options.build_config.build_plan {
			for (package, plan) in plans.iter() {
				let message = plan.printable_serializable(&package);
				config.workspace.config().shell().print_json(&message)?;
			}
		} else {
			config.workspace
			      .config()
			      .shell()
			      .verbose(|shell| {
				      for (package, plan) in plans.iter() {
					      shell.status("Assets", format!("build plan for {}", package.package_id()))?;
					      plan.pretty_print(shell, &config.workspace.root())?;
				      }
				      Ok(())
			      })
			      .log_err()
			      .ok();
		}

		/* TODO: how to resolve conflicts:
			- merge all plans, where
			- resolve conflicts as it happening in the `build_plan()::re-mapping`:
				e.g.: Mapping::* -> Mapping::ManyInto
		*/

		{
			// validate plans:
			let mut has_errors = false;
			let mut targets = HashMap::new();
			for (package, plan) in plans.iter() {
				for target in plan.as_inner().targets() {
					if let Some(pid) = targets.get(&target) {
						has_errors = true;
						let message = format!(
						                      "Duplicate asset destination: {}, found in {:#?}",
						                      target.as_relative_to_root(config).display(),
						                      [pid, &package.package_id()]
						);
						config.log().error(message);
					} else {
						targets.insert(target, package.package_id());
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

			// TODO: also check sources duplicates, but only warn.
		}


		// finally apply plans:
		if !config.dry_run && !config.compile_options.build_config.build_plan && !plans.is_empty() {
			let mut locked = layout.lock_mut(config.workspace.config())?;
			locked.prepare()?;
			let dest = locked.as_inner().assets();

			for (dependency, plan) in plans.into_iter() {
				if plan.difference.is_same() {
					config.log().status(
					                    "Skip",
					                    format!(
						"{}, cache state is {:?}",
						dependency.package_id(),
						&plan.difference
					),
					);
					continue;
				}


				config.log()
				      .status("Build", format!("assets for {}", dependency.package_id()));
				config.log().verbose(|mut log| {
					            let s = format!("destination: {}", dest.as_relative_to_root(config).display());
					            log.status("", s)
				            });


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

								config.log()
								      .status_with_color("Error", message, termcolor::Color::Red)
							},
						};
					}
				}

				if report.has_errors() {
					if !config.compile_options.build_config.keep_going {
						bail!("Assets build failed.");
					}
				}


				// finally build with pdc:
				match pdc::build(config, dependency, locked.as_inner()) {
					Ok(_) => {
						config.log()
						      .status("Finished", format!("assets for {}", dependency.package_id()));
					},
					Err(err) => {
						let message = format!("build with pdc failed: {err}");
						config.log()
						      .status_with_color("Error", message, termcolor::Color::Red);
						if !config.compile_options.build_config.keep_going {
							bail!("Assets build failed.");
						}
					},
				}
			}

			locked.unlock();

			// TODO: if has no errors
			log::debug!(
			            "Assets artifact for {} at {}",
			            package.package_id(),
			            crate::layout::Layout::dest(&layout).as_relative_to_root(config)
			                                                .display()
			);
			let metadata = options.remove(package);
			artifacts.insert(
			                 package,
			                 AssetsArtifact { package,
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
                                            -> CargoResult<HashMap<&'r Package, TomlMetadata>> {
	let mut packages = HashMap::new();
	if let Some(metadata) = playdate_metadata(package) {
		// if explicitly allowed collect deps => scan deps-tree
		if metadata.assets_options().dependencies {
			log::debug!("inspecting deps-tree of {}", package.package_id());

			packages.insert(package, metadata);

			let bcx = bcx.get()?;

			// TODO: cache hash of bcx.unit_graph?

			// find this package in roots:
			let root = bcx.unit_graph
			              .keys()
			              .find(|u| u.pkg.package_id() == package.package_id());
			let mut dependencies = HashSet::new();

			if let Some(deps) = root.and_then(|root| bcx.unit_graph.get(root)) {
				// find all dependencies:
				dependencies.extend(
				                    deps.into_iter()
				                        .flat_map(|ud| bcx.unit_graph.get_key_value(&ud.unit).map(|(u, _)| u)),
				);


				let mut last_length = 0;
				while last_length != dependencies.len() {
					let pre_last_length = dependencies.len();

					let next = dependencies.iter()
					                       .flat_map(|u| {
						                       bcx.unit_graph.get(u).into_iter().flat_map(|deps| {
							                                                        deps.into_iter().flat_map(|ud| {
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
					                  .map(|vec| vec.last())
					                  .flatten()
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


pub fn playdate_metadata(package: &Package) -> Option<TomlMetadata> {
	package.manifest()
	       .custom_metadata()
	       .map(|m| m.as_table().map(|t| t.get(METADATA_FIELD)))
	       .flatten()
	       .flatten()
	       .map(|v| v.to_owned().try_into::<TomlMetadata>().log_err().ok())
	       .flatten()
	       .map(|mut m| m.merge_opts().map(|_| m).log_err().ok())
	       .flatten()
}
