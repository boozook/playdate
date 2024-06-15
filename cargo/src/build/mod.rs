use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use anyhow::Context;
use anyhow::anyhow;
use anyhow::ensure;
use cargo::core::PackageId;
use cargo::CargoResult;
use cargo::core::Package;
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
use crate::utils::cargo::CompileKindExt;
use crate::utils::path::AsRelativeTo;
use crate::utils::workspace::PossibleTargets;


pub mod rustflags;


pub fn build<'cfg>(config: &'_ Config<'cfg>) -> CargoResult<Vec<BuildProduct>> {
	if config.dry_run {
		return Ok(Default::default());
	}

	// get plan _before_ cargo invocation:
	let plan = config.build_plan()?;

	// call cargo:
	let mut cargo = cargo_proxy_cmd(config, &Cmd::Build)?;

	// add build-std:
	// https://github.com/rust-lang/cargo/issues/8733
	// https://stackoverflow.com/a/76614919/829264
	// We cannot specify build-std for one target, so for all if needed for one:
	if !has_only_nopd_targets(config)? {
		cargo.args(["-Zbuild-std=core,alloc", "-Zunstable-options"]);
		if config.prevent_unwinding {
			cargo.arg("-Zbuild-std-features=panic_immediate_abort");
		}
	}
	config.log().verbose(|mut log| {
		            log.status("Cargo", args_line_for_proc(&cargo));
	            });


	// read cargo output:
	let artifacts = read_cargo_output(config, cargo)?;

	let mut products = Vec::new();

	for (package, target, art) in artifacts {
		log::debug!(
		            "cargo: rustc: artifact: {}:{} ({:?}) {:#?}",
		            art.package_id.name(),
		            art.target.name,
		            art.target.kind(),
		            art.filenames.iter().map(|p| p.as_relative_to_root(config))
		);

		// That's easier and faster then build bcx and inspecting unit-graph
		let invocations = plan.build_package_invocations(&package.package_id())
		                      .collect::<Vec<_>>();

		let mapping = art.filenames
		                 .iter()
		                 .flat_map(|path| {
			                 let invocation =
				                 invocations.iter()
				                            .find(|inv| inv.links.contains_key(path) || inv.outputs.contains(path));
			                 debug_assert!(invocation.is_some());
			                 log::trace!("{:?} <== {}", invocation.map(|i| i.kind), path.display());
			                 invocation.map(|i| (path, i))
		                 })
		                 .map(|(p, i)| {
			                 let ct = if art.executable.as_deref() == Some(p.as_path()) {
				                 CrateType::Bin
			                 } else {
				                 let ct = config.rustc_outputs(CompileMode::Build, &art.target.kind(), &i.kind)
				                                .log_err_cargo(config)
				                                .into_iter()
				                                .flat_map(|out| out.0.into_iter())
				                                .find(|ft| {
					                                // compare with final filename that should be:
					                                let name = ft.uplift_filename(target);
					                                p.file_name() == Some(OsStr::new(&name))
				                                })
				                                .and_then(|fty| fty.crate_type)
				                                .unwrap_or_else(|| {
					                                // We're probably never goes to this fallback, but it should be there anyway:
					                                guess_crate_type(config, p, art.target.kind(), i.kind)
				                                });
				                 log::debug!(
				                             "crate type: {ct} - {:?}",
				                             p.file_name().map(|s| s.to_string_lossy())
				);
				                 ct
			                 };
			                 (p, ct, i)
		                 });


		for (path, ct, inv) in mapping {
			let layout = config.layout_for(inv.kind)?.lock(config.workspace.gctx())?;
			let artifact = CargoArtifact { package_id: package.package_id(),
			                               path,
			                               name: art.target.name,
			                               ct: ct.clone(),
			                               ck: inv.kind,
			                               profile: art.profile.to_owned(),
			                               example: target.is_example() };
			let product = match ct {
				CrateType::Bin if artifact.ck.is_playdate() => build_binary(config, layout, artifact),
				CrateType::Dylib | CrateType::Cdylib if artifact.ck.is_simulator() => {
					build_library(config, layout, artifact)
				},
				CrateType::Staticlib if artifact.ck.is_playdate() => build_library(config, layout, artifact),
				_ => {
					let package_crate_name = package.name().replace('-', "_");
					let name = if package_crate_name == *art.target.name {
						Cow::Borrowed(art.target.name.as_str())
					} else {
						format!("{}, crate {}", package.name(), art.target.name).into()
					};
					let target = if artifact.ck.is_playdate() {
						"playdate"
					} else {
						"simulator"
					};
					config.log().status_with_color(
					                               "Skip",
					                               format!(
						"drop {name}, can't build {} targeting {target}",
						artifact.ct
					),
					                               Color::Yellow,
					);
					continue;
				},
			};


			if !config.compile_options.build_config.keep_going {
				products.push(product?);
			} else if let Ok(product) = product.log_err_cargo(config) {
				products.push(product);
			}

			config.log()
			      .status("Finished", format!("{} of {}", ct, inv.package_name));
		}
	}


	config.log().verbose(|mut log| {
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


fn read_cargo_output<'cc>(
	config: &'cc Config,
	cargo: Command)
	-> CargoResult<Vec<(&'cc Package, &'cc Target, crate::proc::reader::format::Artifact)>> {
	use crate::proc::reader::format::CargoMessage;

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


	let possible_targets = config.possible_targets()?;

	// Add completion to iterator with asking & logging process status.
	// It's looks a little bit ugly with map to `Option` then `flat_map`, but
	// after optimization there is no these perturbations.
	// Also we don't need to fail entire process if one target fails and so status will not ok.
	let artifacts =
		map_artifacts(possible_targets, artifacts).map(Some)
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


fn map_artifacts<'cargo, 'cc>(
	targets: impl Iterator<Item = PossibleTargets<'cc>>,
	artifacts: impl Iterator<Item = crate::proc::reader::format::Artifact> + 'cargo)
	-> impl Iterator<Item = (&'cc Package, &'cc Target, crate::proc::reader::format::Artifact)> {
	let targets = targets.collect::<Vec<_>>();
	artifacts.filter_map(move |art| {
		         targets.iter()
		                .find(|(p, ..)| p.package_id() == art.package_id)
		                .and_then(|(package, targets, ..)| {
			                targets.iter()
			                       .find(|t| {
				                       let crate_name = t.crate_name();
				                       (crate_name == *art.target.name ||
				                        crate_name == art.target.name.replace('-', "_")) &&
				                       t.kind() == &art.target.kind()
			                       })
			                       .map(|target| (*package, *target, art))
		                })
	         })
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


fn build_binary<'cfg, Layout, S>(config: &'cfg Config,
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


fn build_library<'cfg, Layout, S>(config: &'cfg Config,
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
fn guess_crate_type(config: &Config, path: &Path, tk: cargo::core::TargetKind, ck: CompileKind) -> CrateType {
	let mut crate_types = tk.rustc_crate_types();
	let unknown = || CrateType::Other("UNKNOWN".into());

	if crate_types.len() == 1 {
		return crate_types.pop().unwrap_or_else(unknown);
	}

	let ext = path.extension();
	if ext.is_none() && crate_types.contains(&CrateType::Bin) {
		return CrateType::Bin;
	}

	if let Some(ext) = ext {
		if ext == static_lib_suffix() && crate_types.contains(&CrateType::Staticlib) {
			CrateType::Staticlib
		} else {
			let dylib_suffix = dylib_suffix_for_target(config, ck);
			if Some(ext) == dylib_suffix.as_deref().map(OsStr::new) &&
			   (crate_types.contains(&CrateType::Dylib) || crate_types.contains(&CrateType::Cdylib))
			{
				// For us it doesn't matter if it's dylib or cdylib
				CrateType::Dylib
			} else if ext == OsStr::new("rlib") {
				CrateType::Rlib
			} else {
				unknown()
			}
		}
	} else {
		unknown()
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
