use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Context;
use anyhow::anyhow;
use anyhow::ensure;
use cargo::core::PackageId;
use cargo::CargoResult;
use cargo::core::Target;
use cargo::core::Verbosity;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CrateType;
use cargo::util::command_prelude::CompileMode;
use playdate::compile::dylib_suffix_for_host_opt;
use playdate::compile::static_lib_suffix;
use playdate::fs::soft_link_checked;
use playdate::toolchain::gcc::ArmToolchain;
use playdate::toolchain::sdk::Sdk;
use anstyle::AnsiColor as Color;

use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::layout::CargoLayout;
use crate::layout::Layout;
use crate::layout::LayoutLockable;
use crate::layout::ForTargetLayout;
use crate::logger::LogErr;
use crate::proc::args_line_for_proc;
use crate::proc::cargo_proxy_cmd;
use crate::proc::reader::CargoJsonReader;
use crate::proc::reader::format::ArtifactProfile;
use crate::proc::reader::format::Artifact;
use crate::proc::reader::format::CargoMessage;
use crate::utils::cargo::meta_deps::MetaDeps;
use crate::utils::cargo::meta_deps::RootNode;
use crate::utils::cargo::CompileKindExt;
use crate::utils::path::AsRelativeTo;


pub mod rustflags;


pub fn build(cfg: &Config, tree: &MetaDeps) -> CargoResult<Vec<BuildProduct>> {
	if cfg.dry_run {
		return Ok(Default::default());
	}

	// call cargo:
	let mut cargo = cargo_proxy_cmd(cfg, &Cmd::Build)?;

	// add build-std:
	// https://github.com/rust-lang/cargo/issues/8733
	// https://stackoverflow.com/a/76614919/829264
	// We cannot specify build-std for one target, so for all if needed for one:
	if !has_only_nopd_targets(cfg)? {
		cargo.args(["-Zbuild-std=core,alloc", "-Zunstable-options"]);
		if cfg.prevent_unwinding {
			cargo.arg("-Zbuild-std-features=panic_immediate_abort");
		}
	}
	cfg.log().verbose(|mut log| {
		         log.status("Cargo", args_line_for_proc(&cargo));
	         });

	// read cargo output:
	let mut artifacts = read_cargo_output(cfg, cargo, tree)?;
	let artifacts_len = artifacts.len();
	log::trace!("artifacts by cargo: {}", artifacts.len());


	// Now we have many artifacts associated with multiple roots which have various `CompileKind`s.
	// We should reduce number of associated to one guessing the target for artifact, each of them.


	// Prepare targets associated with roots:
	let root_targets = cfg.possible_targets()?
	                      .flat_map(|(p, targets)| {
		                      let package_id = p.package_id();
		                      tree.roots()
		                          .iter()
		                          .filter(move |r| r.node().unit().package_id == package_id)
		                          .filter_map(move |r| {
			                          let unit = r.node().unit();
			                          targets.iter()
			                                 .find(|t| {
				                                 &unit.target.kind() == t.kind() &&
				                                 (unit.target.name == t.name() ||
				                                  unit.target.name == t.crate_name()) &&
				                                 unit.target.crate_types == t.rustc_crate_types()
			                                 })
			                                 .map(|t| (r, *t))
		                          })
	                      })
	                      .collect::<Vec<_>>();

	let target_for_root = |root: &RootNode| {
		root_targets.iter()
		            .find_map(|(r, t)| (*r == root).then_some(*t).cloned())
		            .unwrap_or_else(|| root.node().target().cargo_target())
	};


	struct ArtifactGraph<'t, 'cfg> {
		/// Targets of root+artifact
		trg: Vec<Target>,
		/// Artifacts of root+target
		art: Vec<Artifact>,
		/// [`CrateType`]s of `artifact.filename`s
		cty: BTreeMap<PathBuf, CrateType>,
		/// Value: (target index, artifact index)
		index: HashMap<&'t RootNode<'cfg>, (usize, usize)>,
	}


	let mut graph = ArtifactGraph { trg: Vec::with_capacity(tree.roots().len()),
	                                art: Vec::with_capacity(artifacts.len()),
	                                cty: Default::default(),
	                                index: Default::default() };


	fn add_to_graph<'t, 'cfg>(root: &'t RootNode<'cfg>,
	                          target: Target,
	                          art: Artifact,
	                          arts: &mut Vec<Artifact>,
	                          targs: &mut Vec<Target>,
	                          index: &mut HashMap<&'t RootNode<'cfg>, (usize, usize)>) {
		let art_index = arts.len();
		let trg_index = arts.len();
		arts.push(art);
		targs.push(target);
		index.insert(root, (trg_index, art_index));

		log::debug!(
		            "add to build: {}::{} {:?} for {}",
		            root.package_id().name(),
		            root.node().target().name,
		            root.node().target().kind,
		            match root.node().unit().platform {
			            CompileKind::Host => "host",
		               CompileKind::Target(ref kind) => kind.short_name(),
		            }
		);
	}


	fn determine_crate_types<'t, 'a: 'trg, 'trg, 'cfg>(
		cfg: &'t Config<'cfg>,
		art: &'a Artifact,
		target: &'trg Target,
		tk: cargo::core::TargetKind,
		ck: CompileKind)
		-> impl Iterator<Item = (&'a PathBuf, Option<CrateType>)> + 'trg {
		cfg.rustc_outputs(CompileMode::Build, &tk, &ck)
		   .log_err_cargo(cfg)
		   .into_iter()
		   .map(|(fts, _)| fts)
		   .flat_map(move |fts| {
			   art.filenames.iter().map(move |f| {
				                       // compare with expected filename that should be:
				                       let name = f.file_name().expect("artifact filename").to_string_lossy();
				                       let ct = fts.iter()
				                                   .find(|ft| name == ft.uplift_filename(target))
				                                   .and_then(|ft| ft.crate_type.clone());
				                       (f, ct)
			                       })
		   })
	}


	// Reduce a number of associated root to one, determining CrateType and CompileKind for each file of artifact:
	const CYCLE_MAX: u8 = 10; // Usually no more then 1-2 times
	let mut reserved_roots = HashSet::<&RootNode>::new();
	let mut anti_recursion = CYCLE_MAX;
	let mut something_changed = false;
	while anti_recursion == CYCLE_MAX ||
	      (something_changed && anti_recursion > 0 && artifacts.iter().any(|(_, roots)| roots.len() > 1))
	{
		anti_recursion -= 1;
		something_changed = false;

		let mut targets_cache = HashMap::new();

		artifacts.extract_if(|(art, roots)| {
			         let executable = art.executable.is_some();
			         let need_remove_roots = roots.len() > 1;


			         log::trace!(
			                     "Choosing one root of {} for artifact {}::{} {:?}{}",
			                     roots.len(),
			                     art.package_id.name(),
			                     art.target.name,
			                     art.target.kind(),
			                     executable.then_some(", executable").unwrap_or_default()
			);
			         cfg.log_extra_verbose(|_| {
				            let f = art.filenames.iter().map(|p| p.as_relative_to_root(cfg));
				            log::trace!("  with files: {f:?}",);
			            });

			         let mut crate_types_cache = BTreeMap::new();

			         let removed = roots.extract_if(|root| {
				                            let tk = root.node().target().kind();
				                            let ck = root.node().unit().platform;

				                            log::trace!("  check {} {}", tk.description(), match ck {
					                            CompileKind::Host => "host",
				                               CompileKind::Target(ref kind) => kind.short_name(),
				                            });

				                            if reserved_roots.contains(root) {
					                            log::trace!("    excluding, reserved root");
					                            return true;
				                            }

				                            {
					                            use cargo::core::TargetKind as Tk;
					                            if executable && !matches!(tk, Tk::Bin | Tk::ExampleBin) {
						                            log::trace!("    excluding, is not executable");
						                            return true;
					                            }
				                            }

				                            let target = targets_cache.entry(*root)
				                                                      .or_insert_with(|| target_for_root(root));

				                            let ct = determine_crate_types(cfg, art, target, tk.clone(), ck).collect::<Vec<_>>();

				                            let is_good = art.filenames.len() == ct.len() &&
				                                          !ct.iter().any(|(_, ct)| ct.is_none());
				                            if is_good {
					                            // save resolved crate-types:
					                            let ct = ct.into_iter()
					                                       .filter_map(|(p, ct)| ct.map(|ct| (p.to_owned(), ct)));
					                            crate_types_cache.extend(ct);
				                            }

				                            need_remove_roots && !is_good
			                            })
			                            .inspect(|r| {
				                            let p = r.package_id().name();
				                            let t = r.node().target().name.as_str();
				                            log::trace!("    excluded: {p}::{t} {:?}", match r.node().unit().platform
				                            {
					                            CompileKind::Host => "host",
				                               CompileKind::Target(ref kind) => kind.short_name(),
				                            })
			                            })
			                            .count();

			         if removed > 0 {
				         log::trace!("  excluded: {removed}, now roots: {}", roots.len());
			         }

			         if removed > 0 || roots.len() == 1 {
				         something_changed = true;
			         }


			         if roots.len() == 1 {
				         reserved_roots.insert(roots[0]);
				         graph.cty.append(&mut crate_types_cache);
				         true
			         } else {
				         false
			         }
		         })
		         .for_each(|(art, roots)| {
			         let r = roots[0];
			         let trg = target_for_root(r);
			         add_to_graph(r, trg, art, &mut graph.art, &mut graph.trg, &mut graph.index);
		         });
	}


	// Now we have artifacts with associated roots and all of them are kinda ok for thus artifacts.
	// We only have to pick one root, so using fallback method now determining by path.
	if !artifacts.is_empty() {
		log::trace!("using fallback by path 😱");
		let export_dir = cfg.compile_options
		                    .build_config
		                    .export_dir
		                    .clone()
		                    .unwrap_or_else(|| cfg.workspace.target_dir().into_path_unlocked());
		artifacts.extract_if(|(art, roots)| {
			         let various_ck: Vec<_> = {
				         let mut ck: Vec<_> = roots.iter().map(|r| &r.node().unit().platform).collect();
				         ck.sort();
				         ck.dedup();
				         ck
			         };
			         let is_various_ck = various_ck.len() > 1;
			         let has_host = roots.iter()
			                             .map(|r| &r.node().unit().platform)
			                             .any(|ck| matches!(ck, CompileKind::Host));
			         log::trace!("has host: {has_host}, various ck: {is_various_ck}");


			         // num of path components for target/profile/this with filename:
			         let _num_comps = match art.target.kind() {
				         cargo::core::TargetKind::ExampleBin => 4,
			            cargo::core::TargetKind::ExampleLib(_) => 4,
			            _ => 3,
			         };

			         // possible variants:
			         let ck_name: Vec<_> = various_ck.iter()
			                                         .filter_map(|ck| {
				                                         match ck {
					                                         CompileKind::Host => None,
				                                            CompileKind::Target(ct) => Some(ct.short_name()),
				                                         }
			                                         })
			                                         .collect();
			         log::trace!("  possible ck: {ck_name:#?}");

			         let comps = art.filenames
			                        .iter()
			                        .filter_map(|f| f.strip_prefix(&export_dir).log_err_cargo(cfg).ok())
			                        .filter(|f| !f.as_os_str().is_empty())
			                        .map(|f| {
				                        let comps = f.components().count();
				                        let first = f.components().next().map(|c| c.as_os_str());
				                        (comps, first)
			                        })
			                        .collect::<BTreeSet<_>>();

			         let ck_exact = ck_name.iter().copied().find(|ck| {
				                                                  comps.iter()
				                                                       .all(|(_num, first)| matches!(first, Some(s) if *s == *ck))
			                                                  });

			         let removed = if let Some(ck) = ck_exact {
				         roots.extract_if(|root| {
					              match root.node().unit().platform {
						              CompileKind::Host => false,
					                 CompileKind::Target(ct) => ct.short_name() != ck,
					              }
				              })
				              .count()
			         } else if has_host {
				         roots.extract_if(|root| !matches!(root.node().unit().platform, CompileKind::Host))
				              .count()
			         } else {
				         0
			         };

			         if removed > 0 {
				         log::trace!("  excluded: {removed}, now roots: {}", roots.len());
			         }

			         roots.len() == 1
		         })
		         .for_each(|(art, roots)| {
			         let r = roots[0];
			         let trg = target_for_root(r);
			         let tk = r.node().target().kind();
			         let ck = r.node().unit().platform;
			         let ct = determine_crate_types(cfg, &art, &trg, tk, ck).filter_map(|(p, ct)| {
				                                                                ct.map(|ct| (p.to_owned(), ct))
			                                                                });
			         graph.cty.extend(ct);
			         add_to_graph(r, trg, art, &mut graph.art, &mut graph.trg, &mut graph.index);
		         });
	}


	// log missed-mapped artifacts:
	if artifacts_len != graph.art.len() {
		cfg.log().verbose(|mut log| {
			         log.status_err(format_args!("Incomprehensible roots for {} artifacts:", artifacts.len()));

			         for (art, roots) in &artifacts {
				         let files = art.filenames
				                        .iter()
				                        .map(|p| p.as_relative_to_root(cfg))
				                        .map(Path::to_string_lossy)
				                        .intersperse(", ".into())
				                        .collect::<String>();
				         let p = art.package_id.name();
				         let t = art.target.name;
				         let tk = art.target.kind().description();
				         log.status_with_color("Artifact", format_args!("{p}::{t} {tk} with {files}"), Color::Red);
				         log.status("", format_args!(" potential roots: {}", roots.len()));

				         for root in roots {
					         let tk = root.node().target().kind();
					         let ck = root.node().unit().platform;
					         let p = root.package_id().name();
					         let t = root.node().target().name.as_str();
					         log.status("", format_args!("  {p}::{t} {tk:?}, {ck:?}"));
				         }
			         }
		         });
	} else {
		log::trace!("Nothing is missed or incomprehensible 🎉");
	}

	debug_assert!(artifacts.is_empty(), "Missed-mapped artifacts: {artifacts:#?}");
	drop(artifacts);


	// Finally, build artifacts:
	let mut products = Vec::new();
	let mapped = graph.index
	                  .iter()
	                  .map(|(r, (t, a))| (*r, &graph.trg[*t], &graph.art[*a]));

	for (root, target, artifact) in mapped {
		let tk = root.node().target().kind();
		let ck = root.node().unit().platform;
		let package_id = root.package_id();
		let layout = cfg.layout_for(ck)?.lock(cfg.workspace.gctx())?;
		log::debug!("Building {} {}", package_id.name(), target.description_named(),);

		let printable_target_name = || {
			let package_crate_name = package_id.name().replace('-', "_");
			if package_crate_name == *artifact.target.name {
				Cow::Borrowed(package_id.name().as_str())
			} else {
				format!("{}, crate {}", package_id.name(), artifact.target.name).into()
			}
		};

		for file in artifact.filenames.iter() {
			let ct = if let Some(ct) = graph.cty.get(file) {
				ct.clone()
			} else if let Some(ct) = guess_cty(cfg, file, tk.clone(), ck) {
				// We're probably never goes to this fallback, but it should be there anyway
				ct
			} else {
				let name = printable_target_name();
				let trg = target.description_named();
				cfg.log().status_with_color(
				                            "Skip",
				                            format_args!("drop {name} {trg}, unable to determine crate-type"),
				                            Color::Red,
				);
				continue;
			};
			let layout = layout.as_inner();

			let temp = CargoArtifact { package_id: package_id.to_owned(),
			                           path: file,
			                           name: artifact.target.name,
			                           ct: ct.to_owned(),
			                           ck,
			                           profile: artifact.profile.to_owned(),
			                           example: target.is_example() };

			let product = match ct {
				CrateType::Bin if ck.is_playdate() => build_binary(cfg, layout, temp),
				CrateType::Dylib | CrateType::Cdylib if ck.is_simulator() => build_library(cfg, layout, temp),
				CrateType::Staticlib if ck.is_playdate() => build_library(cfg, layout, temp),
				_ => {
					let name = printable_target_name();
					let target = if ck.is_playdate() { "playdate" } else { "simulator" };
					cfg.log().status_with_color(
					                            "Skip",
					                            format_args!("drop {name}, can't build {ct} targeting {target}"),
					                            Color::Yellow,
					);
					continue;
				},
			};

			if !cfg.compile_options.build_config.keep_going {
				products.push(product?);
			} else if let Ok(product) = product.log_err_cargo(cfg) {
				products.push(product);
			}

			cfg.log().status(
			                 "Finished",
			                 format_args!("{} of {} {}", ct, package_id.name(), target.description_named()),
			);
		}
	}

	cfg.log().verbose(|mut log| {
		         log.status("Finished", "cargo execution, got products:");
		         for (i, p) in products.iter().enumerate() {
			         let (head, msg) = match p {
				         BuildProduct::Success { name, src_ct, dst_ct, .. } => {
				            let ct = if dst_ct == src_ct {
					            format!("{dst_ct}")
				            } else {
					            format!("{src_ct} -> {dst_ct}")
				            };
				            ("Build", format!("[{i}] {name} ({ct})"))
			            },
			            BuildProduct::Skip { reason, ct, .. } => {
				            ("Skip", format!("[{i}] ({ct}) with reason: {reason}"))
			            },
			         };
			         log.status_with_color(head, msg, Color::White);
		         }
	         });
	Ok(products)
}


fn read_cargo_output<'cfg, 't>(config: &'cfg Config,
                               cargo: Command,
                               tree: &'t MetaDeps<'cfg>)
                               -> CargoResult<Vec<(Artifact, Vec<&'t RootNode<'cfg>>)>> {
	let mut reader = CargoJsonReader::new(cargo)?;
	let mut build_finished_success = None;

	let artifacts = {
		let mut_ref = &mut build_finished_success;
		reader.read()?.filter_map(|m| {
			              // don't print BuildFinished(true) yet:
			              if !matches!(m, CargoMessage::BuildFinished { success: true }) {
				              config.log().print_cargo_message(&m);
			              }
			              match m {
				              CargoMessage::CompilerArtifact(art) => Some(art),
			                 CargoMessage::BuildFinished { success } => {
				                 *mut_ref = Some(success);
				                 None
			                 },

			                 CargoMessage::CompilerMessage { message } => {
				                 if !config.compile_options.build_config.emit_json() {
					                 match message.level.as_str() {
						                 level if level == "warning" || "error" == level => {
						                    let message = message.rendered
						                                         .trim()
						                                         .strip_prefix(&format!("{level}: "))
						                                         .map(Cow::Borrowed)
						                                         .unwrap_or(message.rendered.as_str().into());

						                    if level == "warning" {
							                    config.log().warn(message);
						                    } else {
							                    config.log().error(message);
						                    }
					                    },
					                    _ => (),
					                 }
				                 }
				                 None
			                 },
			                 _ => None,
			              }
		              })
	};


	// Add completion to iterator with asking & logging process status.
	// It's looks a little bit ugly with map to `Option` then `flat_map`, but
	// after optimization there is no these perturbations.
	// Also we don't need to fail entire process if one target fails and so status will not ok.
	let artifacts = map_artifacts(tree, artifacts).map(Some)
	                                              .chain([reader].into_iter()
	                                                             .flat_map(|mut r| {
		                                                             r.status()
		                                                              .log_err_cargo(config)
		                                                              .ok()
		                                                              .and_then(|status| {
			                                                              status.exit_ok().log_err_cargo(config).ok()
		                                                              })
	                                                             })
	                                                             .map(|_| None))
	                                              .flatten()
	                                              .collect::<Vec<_>>();
	let success = build_finished_success.filter(|v| *v)
	                                    .ok_or_else(|| anyhow!("build not successful"));
	if !config.compile_options.build_config.keep_going {
		success?;
	} else {
		success.log_err_cargo(config).ok();
	}

	Ok(artifacts)
}


fn map_artifacts<'cargo, 'cfg, 't>(tree: &'t MetaDeps<'cfg>,
                                   artifacts: impl Iterator<Item = Artifact> + 'cargo)
                                   -> impl Iterator<Item = (Artifact, Vec<&'t RootNode<'cfg>>)> {
	use crate::utils::cargo::format::TargetKind as TK;

	artifacts.filter(|art| matches!(art.target.kind, TK::Lib(_) | TK::Bin | TK::Example))
	         .filter_map(move |art| {
		         let findings = tree.roots()
		                            .iter()
		                            .filter(|r| {
			                            let unit = r.node().unit();
			                            unit.package_id == art.package_id &&
			                            unit.target.name.as_str() == art.target.name.as_str() &&
			                            unit.target.kind() == art.target.kind() &&
			                            unit.target.crate_types == art.target.crate_types &&
			                            Some(unit.target.src_path.as_str().into()) == art.target.src_path
		                            })
		                            .inspect(|r| log::trace!("root for artifact found: {:?} {r}", art.target.crate_types))
		                            .collect::<Vec<_>>();
		         (!findings.is_empty()).then_some((art, findings))
	         })
	         .filter(|(_, roots)| !roots.is_empty())
}


#[derive(Debug)]
struct CargoArtifact<'cr, Name: AsRef<str> + Debug> {
	package_id: PackageId,
	path: &'cr Path,
	/// Crate name
	name: Name,

	ct: CrateType,
	ck: CompileKind,
	profile: ArtifactProfile,

	example: bool,
}


#[derive(Debug)]
pub enum BuildProduct {
	Success {
		package_id: PackageId,

		/// Crate-target ID
		name: String,

		src_ct: CrateType,
		dst_ct: CrateType,
		ck: CompileKind,
		profile: ArtifactProfile,

		path: PathBuf,
		layout: ForTargetLayout<PathBuf>,

		example: bool,
	},
	Skip {
		reason: String,

		package_id: PackageId,
		ct: CrateType,
		ck: CompileKind,
	},
}

impl BuildProduct {
	fn skip_as_unsupported<S: AsRef<str> + Debug>(artifact: CargoArtifact<'_, S>) -> Self {
		let reason = format!(
		                     "{} ({}) {:?} unsupported target {:?}",
		                     artifact.package_id.name(),
		                     artifact.name.as_ref(),
		                     artifact.ct,
		                     artifact.ck
		);
		Self::Skip { reason,
		             ct: artifact.ct,
		             ck: artifact.ck,
		             package_id: artifact.package_id }
	}
}


impl<'t> Config<'t> {
	fn get_sdk_for(&self, ct: &CrateType, ck: &CompileKind) -> anyhow::Result<&Sdk> {
		let msg = format!("Linking without SDK is not supported yet (target: {ct}, compile-kind: {ck:?})");
		self.sdk().with_context(|| msg).log_err()
	}

	fn get_gcc_for(&self, ct: &CrateType, ck: &CompileKind) -> anyhow::Result<&ArmToolchain> {
		let msg =
			format!("Linking without ARM GNU toolchain is not supported yet (target: {ct}, compile-kind: {ck:?})");
		self.gcc().with_context(|| msg).log_err()
	}
}


fn build_binary<Layout, S>(config: &Config,
                           layout: Layout,
                           artifact: CargoArtifact<'_, S>)
                           -> anyhow::Result<BuildProduct>
	where Layout: AsRef<CargoLayout>,
	      S: AsRef<str> + Debug
{
	ensure!(
	        matches!(artifact.path.try_exists(), Ok(true)),
	        "artifact {} not found at {}",
	        artifact.name.as_ref(),
	        artifact.path.display()
	);

	let package_crate_name = artifact.package_id.name().replace('-', "_");
	let mut pdl = ForTargetLayout::new(
	                                   layout.as_ref(),
	                                   package_crate_name,
	                                   Some(artifact.name.as_ref()),
	).lock(config.workspace.gctx())?;
	pdl.as_mut().prepare()?;

	let product = if artifact.ck.is_playdate() {
		ensure!(artifact.ct == CrateType::Bin, "executable binary expected");

		// product should be in pd-layout:
		let pdl_ref = pdl.as_inner();
		let product = {
			use playdate::layout::Layout as _;
			pdl_ref.binary()
		};
		let linked = soft_link_checked(artifact.path, &product, true, pdl_ref.dest())?;
		log::debug!(
		            "{} linked (overwritten: {linked}) {}",
		            artifact.ct,
		            product.as_relative_to_root(config).display()
		);

		BuildProduct::Success { ck: artifact.ck,
		                        src_ct: artifact.ct,
		                        dst_ct: CrateType::Bin,
		                        profile: artifact.profile,
		                        layout: pdl_ref.to_owned(),
		                        path: product.to_path_buf(),
		                        package_id: artifact.package_id,
		                        name: artifact.name.as_ref().to_owned(),
		                        example: artifact.example }
	} else {
		// Currently this case is usually unreachable, but who knows future…
		BuildProduct::skip_as_unsupported(artifact)
	};

	Ok(product)
}


fn build_library<Layout, S>(config: &Config,
                            layout: Layout,
                            artifact: CargoArtifact<'_, S>)
                            -> anyhow::Result<BuildProduct>
	where Layout: AsRef<CargoLayout>,
	      S: AsRef<str> + Debug
{
	config.log().status(
	                    "Compiling",
	                    format!(
		"{} of {}{}",
		artifact.ct,
		artifact.package_id,
		if config.workspace.gctx().extra_verbose() {
			format!(" from {}", artifact.path.as_relative_to_root(config).display())
		} else {
			Default::default()
		}
	),
	);
	ensure!(
	        matches!(artifact.path.try_exists(), Ok(true)),
	        "artifact {} not found at {}",
	        artifact.name.as_ref(),
	        artifact.path.display()
	);

	let package_crate_name = artifact.package_id.name().replace('-', "_");
	let mut pdl = ForTargetLayout::new(
	                                   layout.as_ref(),
	                                   package_crate_name,
	                                   Some(artifact.name.as_ref()),
	).lock(config.workspace.gctx())?;
	pdl.as_mut().prepare()?;

	let product = if artifact.ck.is_playdate() {
		ensure!(artifact.ct == CrateType::Staticlib, "static lib expected");

		let sdk = config.get_sdk_for(&artifact.ct, &artifact.ck)?;
		let arm = config.get_gcc_for(&artifact.ct, &artifact.ck)?;
		let gcc = arm.gcc();

		// product should be in pd-layout:
		let pdl_ref = pdl.as_inner();
		let product = {
			use playdate::layout::Layout as _;
			pdl_ref.binary()
		};
		let link_map = format!("-T{}", sdk.build_support().link_map().display());
		let d = None::<&Path>;

		let mut gcc = Command::new(gcc.path());
		gcc.arg(artifact.path);
		// TODO: #feature=compat & --with-setup => gcc.arg(setup.c) and -l artifact.path
		// TODO: use const `GCC_ARGS_LIB` from support::compile
		gcc.args([
			"-nostartfiles",
			"-mthumb",
			"-mcpu=cortex-m7",
			"-mfloat-abi=hard",
			"-mfpu=fpv5-sp-d16",
			"-D__FPU_USED=1",
			"-Wl,--cref,--gc-sections,--no-warn-mismatch,--emit-relocs",
			"-fno-exceptions",
			"-mword-relocations",
			"-fno-common",
		]);
		gcc.arg(&link_map);
		if let Some(d) = d {
			gcc.arg(format!("-MF{}", d.display()));
		}
		gcc.args([OsStr::new("-o"), product.as_os_str()]);
		gcc.args(["--entry", "eventHandlerShim"]);

		config.log().verbose(|mut log| {
			            log.status("Gcc", args_line_for_proc(&gcc));
		            });

		// Print gcc's output in verbose mode only!
		let quiet = config.workspace.gctx().shell().verbosity() == Verbosity::Quiet;
		let extra_verbose = config.workspace.gctx().extra_verbose();
		gcc.stderr(if quiet { Stdio::null() } else { Stdio::inherit() });
		gcc.stdout(if extra_verbose {
			   Stdio::inherit()
		   } else {
			   Stdio::null()
		   });
		gcc.status()?.exit_ok()?;
		BuildProduct::Success { ck: artifact.ck,
		                        src_ct: artifact.ct,
		                        dst_ct: CrateType::Bin,
		                        profile: artifact.profile,
		                        layout: pdl_ref.to_owned(),
		                        path: product.to_path_buf(),
		                        package_id: artifact.package_id,
		                        name: artifact.name.as_ref().to_owned(),
		                        example: artifact.example }
	} else if artifact.ck.is_simulator() {
		ensure!(artifact.ct.is_dynamic(), "dynamic lib expected");
		let pdl = pdl.as_ref();
		let product = {
			use playdate::layout::Layout as _;
			pdl.library()
			   .with_extension(artifact.path.extension().unwrap_or_default())
		};
		let linked = soft_link_checked(artifact.path, &product, true, pdl.dest())?;
		log::debug!(
		            "{} linked (overwritten: {linked}) {}",
		            artifact.ct,
		            product.as_relative_to_root(config).display()
		);
		BuildProduct::Success { ck: artifact.ck,
		                        src_ct: artifact.ct.to_owned(),
		                        dst_ct: artifact.ct,
		                        profile: artifact.profile,
		                        layout: pdl.to_owned(),
		                        path: product,
		                        package_id: artifact.package_id,
		                        name: artifact.name.as_ref().to_owned(),
		                        example: artifact.example }
	} else {
		// Currently this case is usually unreachable, but who knows future…
		BuildProduct::skip_as_unsupported(artifact)
	};

	Ok(product)
}


/// Guesses a `CrateType` by `path`.ext and artifact crate types (target kind).
///
/// `crate_types` should not be empty, otherwise `None` will be returned.
fn guess_cty(cfg: &Config, path: &Path, tk: cargo::core::TargetKind, ck: CompileKind) -> Option<CrateType> {
	let mut crate_types = tk.rustc_crate_types();

	if crate_types.len() == 1 {
		return crate_types.pop();
	}

	let ext = path.extension();
	if ext.is_none() && crate_types.contains(&CrateType::Bin) {
		return Some(CrateType::Bin);
	}

	if let Some(ext) = ext {
		if ext == static_lib_suffix() && crate_types.contains(&CrateType::Staticlib) {
			Some(CrateType::Staticlib)
		} else {
			let dylib_suffix = dylib_suffix_for_target(cfg, ck);
			if Some(ext) == dylib_suffix.as_deref().map(OsStr::new) &&
			   (crate_types.contains(&CrateType::Dylib) || crate_types.contains(&CrateType::Cdylib))
			{
				// For us it doesn't matter if it's dylib or cdylib
				Some(CrateType::Dylib)
			} else if ext == OsStr::new("rlib") {
				Some(CrateType::Rlib)
			} else {
				None
			}
		}
	} else {
		None
	}
}


pub fn dylib_suffix_for_target<'c>(config: &'c Config, kind: CompileKind) -> Option<Cow<'c, str>> {
	use crate::utils::rustc::TargetSpec;

	let for_target = |kind| {
		config.rustc
		      .target_spec(kind)
		      .log_err_cargo(config)
		      .ok()
		      .and_then(|spec| spec.dll_suffix)
	};
	let for_host = || for_target(&CompileKind::Host);

	match kind {
		CompileKind::Host => {
			for_host().map(Cow::from)
			          .or_else(|| dylib_suffix_for_host_opt().map(Into::into))
		},
		CompileKind::Target(target) if target == config.host_target => {
			for_host().map(Cow::from)
			          .or_else(|| dylib_suffix_for_host_opt().map(Into::into))
		},
		CompileKind::Target(_) => for_target(&kind).map(Into::into),
	}
}


fn has_only_nopd_targets(config: &Config) -> CargoResult<bool> {
	let pd = CompileKind::playdate();
	let kinds = &config.compile_options.build_config.requested_kinds[..];
	let contains_nopd = kinds.iter().any(|kind| *kind != pd);
	let contains_pd = kinds.contains(&pd);
	if !contains_pd && contains_nopd {
		return Ok(true);
	}

	// kinds of interest:
	let mut kinds: Vec<CompileKind> = Vec::new();
	kinds.extend(&config.compile_options.build_config.requested_kinds);

	let specs = config.compile_options
	                  .spec
	                  .to_package_id_specs(&config.workspace)?;
	let cli_features = &config.compile_options.cli_features;
	let members = config.workspace.members_with_features(&specs, cli_features)?;

	for (package, ..) in members {
		if contains_nopd {
			kinds.extend(package.manifest().default_kind());
		}
		kinds.extend(package.manifest().forced_kind());
	}

	let contains_nopd = kinds.iter().any(|kind| kind != &pd);
	let contains_pd = kinds.contains(&pd);

	Ok(!contains_pd && contains_nopd)
}
