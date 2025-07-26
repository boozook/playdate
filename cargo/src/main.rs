#![feature(never_type)]
#![feature(iter_intersperse)]
#![feature(exit_status_error)]
#![feature(btree_extract_if)]
#![feature(const_trait_impl)]
#![feature(debug_closure_helpers)]

extern crate build as playdate;

use std::borrow::Cow;

use anyhow::bail;
use cargo::core::Verbosity;
use cargo::core::compiler::{CrateType, CompileKind};
use cargo::util::{CargoResult, GlobalContext as CargoConfig};
use config::Config;
use anstyle::AnsiColor as Color;

use crate::utils::cargo::CompileKindExt;

mod cli;
mod proc;
mod logger;
mod config;
mod build;
mod assets;
mod package;
mod layout;
mod utils;


fn main() -> CargoResult<()> {
	let mut config = CargoConfig::default().unwrap_or_else(|err: anyhow::Error| {
		                                       let mut shell = cargo::core::Shell::new();
		                                       cargo::exit_with_error(err.into(), &mut shell);
	                                       });
	let config = cli::initialize(&mut config)?;
	config.workspace.emit_warnings()?;
	execute(&config).map_or_else(
	                             |err| {
		                             #[cfg(debug_assertions)]
		                             eprintln!("Error: {err:?}");
		                             let config = config.workspace.gctx();
		                             config.shell().set_verbosity(Verbosity::Normal);
		                             config.shell().error(err).ok();
		                             std::process::exit(1);
	                             },
	                             |v| {
		                             //  let config = workspace.config();
		                             //  if v > 0 || !inspection_errors.errors().is_empty() {
		                             //     let _ = inspection_errors.report_summary(&config);
		                             //     std::process::exit(v.max(1));
		                             //  } else {
		                             //     std::process::exit(0);
		                             //  }
		                             Ok(v)
	                             },
	)
}


fn execute(config: &Config) -> CargoResult<()> {
	match config.cmd {
		cli::cmd::Cmd::Assets => {
			let deps_tree = crate::utils::cargo::meta_deps::meta_deps(config)?;
			assets::build_all(config, &deps_tree)?;
		},

		cli::cmd::Cmd::Build => {
			if config.compile_options.build_config.build_plan {
				// TODO: wrap result to our own build-plan?
				// let plan = config.build_plan()?;
				// TODO: return the plan
				// config.compile_options.build_config.emit_json()
				return Err(anyhow::anyhow!("build-plan in not implemented yet"));
			}

			let deps_tree = crate::utils::cargo::meta_deps::meta_deps(config)?;
			build::build(config, &deps_tree)?;
		},

		cli::cmd::Cmd::Package => {
			let deps_tree = crate::utils::cargo::meta_deps::meta_deps(config)?;
			let assets = assets::build_all(config, &deps_tree)?;
			let products = build::build(config, &deps_tree)?;

			log::debug!("assets artifacts: {}", assets.len());
			log::debug!("build  artifacts: {}", products.len());

			package::build_all(config, assets, products)?;
		},

		cli::cmd::Cmd::Run => {
			let ck = config.compile_options.build_config.single_requested_kind()?;
			if !ck.is_host() && !ck.is_playdate() {
				let name = match ck {
					CompileKind::Target(ct) => ct.short_name().to_owned(),
					CompileKind::Host => unreachable!(),
				};
				// XXX: This is totally wrong for custom host-targets as json-spec:
				bail!("Only host and playdate targets can be run, but '{name}' requested.");
			}


			// filter packages-targets compatible with compile-kind:
			let mut expected = config.possible_targets_ext()?;
			expected.iter_mut().for_each(|(_, targets)| {
				                   let dropped =
					                   targets.extract_if(.., |target| {
						                          let cts = target.rustc_crate_types();
						                          !if ck.is_playdate() {
							                          cts.contains(&CrateType::Bin) ||
							                          cts.contains(&CrateType::Staticlib)
						                          } else if ck.is_host() {
							                          cts.contains(&CrateType::Dylib) || cts.contains(&CrateType::Cdylib)
						                          } else {
							                          unreachable!("Currently unreachable, but who knows future…")
						                          }
					                          })
					                          .inspect(|target| {
						                          let cks = format!("as incompatible with {}", match ck {
							                          CompileKind::Target(ct) => ct.short_name().to_owned(),
						                             CompileKind::Host => config.host_target.short_name().to_owned(),
						                          });
						                          let reason = if ck.is_playdate() {
							                          "to run on device"
						                          } else {
							                          "to run on simulator"
						                          };
						                          let msg = format!(
						                                            "target {} {:?} dropped {cks} {reason}.",
						                                            target.name(),
						                                            target.kind()
						);
						                          config.log().status_with_color("Skip", msg, Color::Yellow);
					                          })
					                          .count();
				                   if dropped > 0 {
					                   config.log().warn(format!("Dropped targets: {dropped}"))
				                   }
			                   });

			expected.extract_if(.., |(_, targets)| targets.is_empty()).count();
			if expected.is_empty() {
				bail!("Nothing found to run");
			}

			let deps_tree = crate::utils::cargo::meta_deps::meta_deps(config)?;

			// build requested package(s):
			let assets = assets::build_all(config, &deps_tree)?;
			let mut products = build::build(config, &deps_tree)?;

			// filter products with expected:
			products.extract_if(.., |product| {
				        match product {
					        build::BuildProduct::Success { package_id,
				                                          name,
				                                          src_ct,
				                                          .. } => {
					           !expected.iter().any(|(p, targets)| {
						                           p.package_id() == *package_id &&
						                           targets.iter().any(|t| {
							                                         let crate_name = t.crate_name();
							                                         (name == &crate_name ||
							                                          name.replace('-', "_") == crate_name) &&
							                                         t.kind().rustc_crate_types().contains(src_ct)
						                                         })
					                           })
				           },
				           _ => true,
				        }
			        })
			        .count();

			let packages = package::build_all(config, assets, products)?;
			match packages.len() {
				1 => (),
				0 => bail!("No packages have been produced, nothing to run."),
				n => bail!("Produced {n} packages but should be 1, can't choose one."),
			}
			let package = packages.first().unwrap();

			config.log().build_finished(true, Some(package.package_id));


			{
				let sim_or_dev: Cow<str> = if ck.is_playdate() {
					if let Some(query) = config.mounting.as_ref().and_then(|m| m.device.value.as_ref()) {
						format!("on the '{}'", query.to_value_string()).into()
					} else {
						"on a device".into()
					}
				} else {
					"in simulator".into()
				};
				let msg = format!("{} {sim_or_dev}", package.name);
				config.log().status("Running", msg);
			}


			// run:
			{
				use device::run::run as run_dev;
				use simulator::run::run as run_sim;

				if ck.is_playdate() {
					let query = config.mounting
					                  .as_ref()
					                  .map(ToOwned::to_owned)
					                  .unwrap_or_default()
					                  .device;
					let pdx = package.path.to_owned();
					let no_install = false;
					let no_read = config.no_read;
					let force = true;
					let fut = run_dev(query, pdx, no_install, no_read, force);
					async_std::task::block_on(fut)?;
				} else {
					let fut = run_sim(&package.path, config.sdk_path.as_deref());
					futures_lite::future::block_on(fut)?;
				};
			}

			std::process::exit(0)
		},
	}

	config.log().build_finished(true, Some("finally 🎉"));

	Ok(())
}
