//! This args needed for:
//! - to fill [[CargoConfig]],
//! - get our-needed options,
//! - filter out our opt and later call cargo/rustc


use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use clap::error::{ErrorKind, ContextKind, ContextValue};
use clap::{Arg, ArgMatches, FromArgMatches};
use cargo::core::{Workspace, VirtualManifest, WorkspaceConfig, WorkspaceRootConfig};
use cargo::core::compiler::{CompileTarget, CompileKind};
use cargo::ops::CompileOptions;
use cargo::util::command_prelude::{ArgMatchesExt, CompileMode, ProfileChecking};
use cargo::util::GlobalContext as CargoConfig;
use cargo::util::CargoResult;
use clap_lex::SeekFrom;

use crate::config::Config;
use crate::logger::LogErr;
use self::opts::*;
use self::cmd::Cmd;
use self::target::{PlaydateTarget, NeedToReplace};


pub mod target;
pub mod opts;
pub mod deps;
pub mod ide;
pub mod cmd;


const BIN_NAME: &str = env!("CARGO_BIN_NAME");
const CMD_NAME: &str = "playdate";


pub fn initialize(config: &'_ mut CargoConfig) -> CargoResult<Config<'_>> {
	initialize_from(env::args_os(), config)
}


pub fn initialize_from(args: impl IntoIterator<Item = impl Into<OsString> + AsRef<OsStr> + Clone>,
                       config: &'_ mut CargoConfig)
                       -> CargoResult<Config<'_>> {
	let args = args.into_iter().collect::<Vec<_>>();

	if cfg!(debug_assertions) {
		println!("bin: {}", std::env::current_exe()?.display());
		println!("args: {:?}", args.iter().map(|s| s.as_ref()).collect::<Vec<_>>());
	}

	let aliases: Option<HashMap<String, String>> = config.get("alias")?;
	let mut cli = tool(aliases.as_ref());
	let matches = cli.try_get_matches_from_mut(args.clone())
		   .or_else(|err| {
			   match err.kind() {
				   ErrorKind::InvalidSubcommand => {
				      match err.context().next() {
					      Some((ContextKind::InvalidSubcommand, ContextValue::String(cmd))) if Cmd::try_from(cmd.as_ref()).is_ok() => {
					         // create root command with globals without cargo-wrapper:
					         main(aliases.as_ref())
					         .bin_name(
					            env::var("CARGO_BIN_NAME")
					               .or_else(|_|env::var("CARGO_CRATE_NAME"))
					               .map_err(anyhow::Error::from)
					            .or_else(|err| {
					               match env::current_exe().map(|p|p.file_name().map(|s| s.to_string_lossy().to_string())) {
					                  Ok(Some(result)) => Ok::<_, anyhow::Error>(result),
					                  Ok(None) => Err(err),
					                  Err(err) => Err(err.into()),
					               }
					            }
					         ).unwrap_or(CMD_NAME.into())
							)
								.name(CMD_NAME)
								.try_get_matches_from(args.clone())
				         },
							//                        .no_binary_name(true)?
				         _ => cli.bin_name("cargo").try_get_matches_from(args.clone()),
				      }
			      },
			      _ => err.exit(),
			   }.map_err(|sub| sub.exit())
		   })
		   .unwrap(/* already infallible */);

	if let Some(cwd) = matches._value_of_os("directory") {
		env::set_current_dir(cwd)?;
		config.reload_rooted_at(cwd)?;
	}
	config.nightly_features_allowed = true;

	// Standalone mode compatibility.
	// If subcommand name is allowed command, skip root command (tool name / cargo command).
	let matches = if matches.subcommand_name()
	                        .and_then(|s| Cmd::try_from(s).ok())
	                        .is_some()
	{
		// skip root command
		matches.subcommand()
	} else {
		// get root, then subcommand
		matches.subcommand_matches(CMD_NAME).and_then(|m| m.subcommand())
	};


	if let Some((name, matches)) = matches {
		let cfg: Vec<String> = matches.get_many("config").unwrap_or_default().cloned().collect();
		let target_dir = matches.get_one::<String>("target-dir").map(PathBuf::from);
		let unstable_flags: Vec<String> = matches.get_many("unstable-features")
		                                         .unwrap_or_default()
		                                         .cloned()
		                                         .collect();
		// disallow any changes in workspace:
		let unstable_flags_ext: Vec<_> =
			unstable_flags.into_iter()
			              .chain(["unstable-options".to_owned(), "no-index-update".to_owned()])
			              .collect();

		let verbose = matches.verbose();
		let quiet = matches.get_flag("quiet");
		crate::logger::init(verbose).map_err(|err| eprintln!("{err}"))
		                            .ok();

		config.configure(
		                 verbose,
		                 quiet,
		                 matches.get_one::<&str>("color").copied(),
		                 matches.get_flag("frozen"),
		                 matches.get_flag("locked"), // TODO: || true, // disallow any changes in workspace
		                 matches.get_flag("offline"),
		                 &target_dir,
		                 &unstable_flags_ext,
		                 &cfg,
		)?;


		let cmd = Cmd::try_from(name)?;

		let workspace = if !matches!(cmd, Cmd::New | Cmd::Init) {
			matches.workspace(config)?
		} else {
			// We should not create real ws for place where potentially nothing.
			// So, lets create virtual one.
			let cwd = env::current_dir()?;
			let fake = cwd.join("Cargo.toml");
			let ws_cfg = WorkspaceConfig::Root(WorkspaceRootConfig::new(
				&cwd,
				&Default::default(),
				&Default::default(),
				&Default::default(),
				&Default::default(),
				&Default::default(),
			));
			let fake_manifest = VirtualManifest::new(
			                     Rc::default(),
			                     Rc::new(toml_edit::ImDocument::parse("".to_owned()).expect("empty is valid TOML")),
			                     Rc::default(),
			                     Rc::default(),
			                     Vec::new(),
			                     Default::default(),
			                     ws_cfg,
			                     Default::default(),
			                     Default::default(),
			);

			Workspace::new_virtual(cwd, fake, fake_manifest, config)?
		};

		log::debug!("ws target_dir: {:?}", workspace.target_dir().as_path_unlocked());

		let mut compile_options = compile_options(&cmd, matches, &workspace)?;

		// add extra targets by shorthands:
		let extra_targets = mapped_flags_playdate_target().into_iter()
		                                                  .filter(|(k, _)| {
			                                                  matches.try_get_one::<bool>(k.as_str())
			                                                         .ok()
			                                                         .flatten()
			                                                         .filter(|v| **v)
			                                                         .is_some()
		                                                  })
		                                                  .collect::<HashMap<_, _>>();
		for (k, v) in &extra_targets {
			log::debug!("target added: {v:?} because of '--{k}'");
			compile_options.build_config.requested_kinds.push(*v);
		}
		if !extra_targets.is_empty() {
			compile_options.build_config.requested_kinds.sort();
			compile_options.build_config.requested_kinds.dedup();
		}


		if matches!(cmd, Cmd::Build | Cmd::Assets | Cmd::Package | Cmd::Run) {
			matches.check_optional_opts(&workspace, &compile_options)?;
		}

		let rustc = workspace.gctx().load_global_rustc(Some(&workspace))?;
		let host_target = CompileTarget::new(&rustc.host)?;


		// toolchains:
		let no_sdk = matches.get_flag("no-sdk");
		let no_gcc = matches.get_flag("no-gcc");
		let sdk_path = matches.get_one::<PathBuf>("sdk").cloned();
		let gcc_path = matches.get_one::<PathBuf>("gcc").cloned();

		{
			// debug:
			let extra = matches.try_get_many::<OsString>("EXTRA")
			                   .unwrap_or_default()
			                   .into_iter()
			                   .flatten()
			                   .collect::<Vec<_>>();
			log::debug!("extra args: {extra:?}");
		}

		let no_read = matches.flag("no-read");
		let mounting = matches!(cmd, Cmd::Run).then(|| Mount::from_arg_matches(matches).ok())
		                                      .flatten();

		// zip flag for package:
		let zip = matches.flag("zip");
		let no_info_meta = matches.flag("no-info-file");

		// shorthand for panic behavior:
		let prevent_unwinding = matches.flag("no-unwinding");

		// path positional arg for new & init:
		let create_path = matches._contains("path")
		                         .then(|| matches.get_one::<PathBuf>("path").cloned())
		                         .flatten();
		let create_full_config = matches.flag("full-config");
		let create_local_schema = matches.flag("local-schema");
		let create_full_metadata = matches.flag("full-metadata");
		let create_deps_sys_only = matches.flag("sys-only");
		let create_deps = {
			let mut create_deps: Vec<_> = matches._values_of("deps")
			                                     .into_iter()
			                                     .flat_map(|s| {
				                                     s.replace(',', " ")
				                                      .split(' ')
				                                      .filter(|s| !s.trim().is_empty())
				                                      .map(|s| deps::Dependency::from_str(s).log_err().unwrap())
				                                      .collect::<Vec<_>>()
			                                     })
			                                     .collect();
			create_deps.sort();
			create_deps.dedup();
			create_deps
		};
		let ide = matches._contains("ide")
		                 .then(|| matches.get_one::<ide::Ide>("ide"))
		                 .flatten()
		                 .cloned()
		                 .unwrap_or_default();

		// TODO: mb. decrease verbosity for underlying cargo by 1.

		// args for future invocation:
		let args = adapt_args_for_underlying_cargo(
		                                           &cmd,
		                                           aliases.as_ref(),
		                                           args.into_iter(),
		                                           matches,
		                                           &compile_options.build_config.requested_kinds,
		                                           &host_target,
		);

		let dry_run = matches.dry_run();
		let skip_unknown = matches.flag("skip-unknown");
		let skip_prebuild = matches.flag("no-pre-build");

		return Ok(Config::new(
		                      cmd,
		                      args,
		                      verbose,
		                      quiet,
		                      dry_run,
		                      skip_unknown,
		                      skip_prebuild,
		                      no_sdk,
		                      no_gcc,
		                      sdk_path,
		                      gcc_path,
		                      mounting,
		                      no_read,
		                      zip,
		                      no_info_meta,
		                      prevent_unwinding,
		                      create_path,
		                      create_full_config,
		                      create_local_schema,
		                      create_full_metadata,
		                      create_deps_sys_only,
		                      create_deps,
		                      ide,
		                      workspace,
		                      host_target,
		                      compile_options,
		                      rustc,
		));
	}

	unreachable!("clap should have caught this")
}


/// Aliases for the `command`
fn command_aliases<'s, 'c: 's>(cmd: &'c Cmd,
                               aliases: Option<&'s HashMap<String, String>>)
                               -> impl Iterator<Item = &'s OsStr> {
	aliases.as_ref()
	       .map(|m| {
		       m.iter()
		        .filter(|(_, v)| v == &cmd.as_ref())
		        .map(|(k, ..)| OsStr::new(k.as_str()))
		        .collect::<Vec<_>>()
	       })
	       .unwrap_or_default()
	       .into_iter()
	       .chain([OsStr::new(cmd.as_ref())])
}

/// Get all aliases for the `arg`, returns tuple: `(short, long)`.
fn arg_all_aliases(arg: &Arg) -> (impl Iterator<Item = char>, impl Iterator<Item = Cow<str>>) {
	let shorts = arg.get_short().into_iter().chain(arg.get_all_short_aliases()
	                                                  .into_iter()
	                                                  .flat_map(|a| a.into_iter()));
	let longs = [Cow::from(arg.get_id().as_str())].into_iter()
	                                              .chain(arg.get_all_aliases().into_iter().flatten().map(Cow::from));
	(shorts, longs)
}

/// List global args that accepts values which equal the current command
fn presented_dangerous_global_args_values<'m>(matches: &'m ArgMatches,
                                              cmd_aliases: &[&OsStr])
                                              -> Vec<(Arg, Vec<&'m OsStr>)> {
	let globals = globals();
	let globals: Vec<_> = globals.get_arguments().collect();

	matches.ids()
	       .filter_map(|id| globals.iter().find(|arg| arg.get_id() == id))
	       .filter_map(|arg| {
		       let id = arg.get_id();
		       matches._contains(id.as_str())
		              .then(|| {
			              let values: Vec<_> = matches.try_get_raw(id.as_str())
			                                          .unwrap()
			                                          .unwrap_or_default()
			                                          .filter(|value| cmd_aliases.contains(value))
			                                          .collect();
			              if !values.is_empty() {
				              // there is double-ref to arg:
				              Some::<(Arg, _)>(((*arg).to_owned(), values))
			              } else {
				              None
			              }
		              })
		              .flatten()
	       })
	       .collect()
}


fn compile_options(cmd: &Cmd, matches: &ArgMatches, ws: &Workspace<'_>) -> CargoResult<CompileOptions> {
	let cfg = ws.gctx();
	let mut compile_options = match cmd {
		// allow multiple crates:
		Cmd::Build | Cmd::Package | Cmd::Migrate | Cmd::Assets => {
			matches.compile_options(cfg, CompileMode::Build, Some(ws), ProfileChecking::Custom)?
		},

		Cmd::New | Cmd::Init => {
			let mut opts = CompileOptions::new(ws.gctx(), CompileMode::Check { test: false })?;
			opts.build_config
			    .requested_kinds
			    .push(PlaydateTarget::Device.into());
			opts
		},

		// allow only one crate:
		Cmd::Run => {
			matches.compile_options_for_single_package(cfg, CompileMode::Build, Some(ws), ProfileChecking::Custom)?
		},

		// allow only one crate?
		Cmd::Publish => {
			matches.compile_options_for_single_package(cfg, CompileMode::Build, Some(ws), ProfileChecking::Custom)?
		},
	};

	let kinds = &mut compile_options.build_config.requested_kinds;

	// add shorthands:
	let device = if !matches!(cmd, Cmd::Run) {
		matches.flag(&PlaydateTarget::Device)
	} else {
		matches._contains(&PlaydateTarget::Device)
	};
	if device {
		if kinds.len() == 1 && matches!(kinds.first(), Some(CompileKind::Host)) {
			kinds[0] = PlaydateTarget::Device.into();
		} else {
			kinds.push(PlaydateTarget::Device.into());
		}
	}

	// replace our exclusive targets for cargo/rustc:
	kinds.iter_mut().for_each(|kind| {
		                if kind.need_to_replace() {
			                if let Ok(ref pd) = PlaydateTarget::try_from(&*kind) {
				                *kind = pd.into();
			                }
		                }
	                });
	kinds.sort();
	kinds.dedup();

	Ok(compile_options)
}


/// - Find `cmd` and replace to `Cmd::Build`
/// - Replace all `--target(=)PlaydateTarget::*` to normalized `CompileKind`
/// - Strip first args such as `cargo playdate cmd`.
///
/// Note: this doesn't supports flag-series with values like `-abc="c-value"`.
fn adapt_args_for_underlying_cargo<S, I>(cmd: &Cmd,
                                         aliases: Option<&HashMap<String, String>>,
                                         args: I,
                                         matches: &ArgMatches,
                                         kinds: &[CompileKind],
                                         host: &CompileTarget)
                                         -> Vec<OsString>
	where I: Iterator<Item = S>,
	      S: Into<OsString> + AsRef<OsStr> + AsRef<OsStr>
{
	let mut result = Vec::new();
	let raw_args = clap_lex::RawArgs::new(args);
	let mut cursor = raw_args.cursor();

	let cmd_aliases = command_aliases(cmd, aliases).collect::<Vec<_>>();
	let dangerous = presented_dangerous_global_args_values(matches, &cmd_aliases);
	let dangerous = dangerous.iter().collect::<Vec<_>>();

	// context:
	let mut meet_tool = false;
	let mut meet_cmd = false;
	let mut meet_escape = false;

	let _ = raw_args.next(&mut cursor); // Skip the bin

	// prepare special args for exclusion:
	let specials_args: Vec<_> = special_args_for(cmd).into_iter()
	                                                 .chain(special_args_global().into_iter())
	                                                 .collect();
	let special_cases = matches!(cmd, Cmd::Run) && matches.get_raw("device").is_some();
	let is_special_case = |id| id == "device" && matches!(cmd, Cmd::Run);
	let specials_vals =
		specials_args.iter()
		             .flat_map(|arg| {
			             let max_values = if let Some(range) = arg.get_num_args() {
				             assert!(
				                     arg.get_short().map(|_| range.max_values()).unwrap_or_default() == 0,
				                     "special shorts should no require values, otherwise not implemented, {arg:?}"
				);
				             assert!(
				                     range.min_values() == range.max_values() ||
				                     is_special_case(arg.get_id().as_str()),
				                     "special args should require exact values num, otherwise not implemented"
				);
				             assert!(
				                     range.max_values() <= 1,
				                     "special args should have 0..1 values, otherwise not implemented"
				);
				             range.max_values()
			             } else {
				             0
			             };
			             assert!(
			                     !arg.is_allow_hyphen_values_set(),
			                     "special args should no allow hyphen values"
			);
			             let mut all: Vec<_> = arg.get_all_aliases()
			                                      .unwrap_or_default()
			                                      .into_iter()
			                                      .map(ToOwned::to_owned)
			                                      .collect();
			             all.extend(arg.get_long().map(ToOwned::to_owned).into_iter());
			             all.into_iter().map(move |k| (k, max_values))
		             })
		             .collect::<HashMap<String, usize>>();


	// insert targets:
	let replace_host = kinds.len() > 1;
	for target in kinds.iter() {
		let render = |kind: &CompileKind| {
			let target = match kind {
				CompileKind::Host => replace_host.then_some(host),
				CompileKind::Target(target) => Some(target),
			};
			target.map(|target| format!("--target={}", target.rustc_target().as_str()))
		};
		let s = match PlaydateTarget::try_from(target) {
			Ok(pd) => render(&pd.into()),
			Err(_) => render(target),
		};

		if !matches!(cmd, Cmd::Init | Cmd::New) {
			if let Some(s) = s {
				log::debug!("+arg: {s}");
				result.push(OsString::from(s))
			}
		}
	}


	while let Some(arg) = raw_args.next(&mut cursor) {
		let raw_arg = arg.to_value_os();
		let is_escape = arg.is_escape();
		let maybe_cmd = !meet_cmd &&
		                !(arg.is_empty() ||
		                  is_escape ||
		                  arg.is_long() ||
		                  arg.is_short() ||
		                  arg.is_negative_number() ||
		                  arg.is_stdio());

		if maybe_cmd {
			if !meet_tool && raw_arg == OsStr::new(CMD_NAME) {
				meet_tool = true;
				continue;
			} else if !meet_cmd && cmd_aliases.contains(&raw_arg) {
				// TODO: ensure that is arg is not onw of globals!
				// - get ALL globals with values,
				// - check: (prev raw_arg is rendered (any of arg's aliases)) && (this `raw_arg` is v)
				// => so this is NOT a command!
				let mut is_value = false;
				for conflicting in dangerous.iter().filter(|(_, values)| values.contains(&raw_arg)) {
					let mut get_prev_arg = || {
						raw_args.seek(&mut cursor, clap_lex::SeekFrom::Current(-2));
						let res = raw_args.next(&mut cursor);
						raw_args.next(&mut cursor);
						res
					};

					is_value = if let Some(prev) = get_prev_arg() {
						let (mut shorts, mut longs) = arg_all_aliases(&conflicting.0);
						let mut last_flag = prev.to_short()
						                        .into_iter()
						                        .filter_map(|k| k.last().and_then(|k| k.ok()))
						                        .filter(|flag| shorts.any(|ref key| key == flag));
						let mut long = prev.to_long()
						                   .and_then(|(key, value)| key.ok().map(|key| (key, value)))
						                   .filter(|(arg, _)| longs.any(|ref key| key == arg))
						                   .into_iter();
						last_flag.next().is_some() || matches!(long.next(), Some((_, None)))
					} else {
						false
					};
					// stop the loop if have found that's value for prev arg:
					if is_value {
						break;
					}
				}

				if !is_value {
					meet_cmd = true;
					meet_tool = true; // that's true for standalone case, otherwise impossible

					// skip the command
					// raw_arg = OsStr::new(Cmd::Build.as_str());
					continue;
				}
			}
		} else if !meet_tool {
			continue;
		}

		// skip first escape:
		if !meet_escape && is_escape {
			meet_escape = true;
			continue;
		}


		// replace targets:
		// NOTE: args already validated by clap, so we know that after target must be value.
		let target_arg = arg.to_long()
		                    .map(|(res, val)| {
			                    let s = match res.map(OsStr::new) {
				                    Ok(s) | Err(s) => s,
			                    };
			                    (s, val)
		                    })
		                    .filter(|(arg, _)| arg == &OsStr::new("target"))
		                    .map(|(s, val)| (s, val.or_else(|| raw_args.next_os(&mut cursor))))
		                    .and_then(|(s, val)| val.map(|val| (s, val)));
		if let Some((..)) = target_arg {
			continue;
		} else if let Some(arg) = arg.to_long()
		                             .map(|(res, _)| {
			                             match res.map(OsStr::new) {
				                             Ok(s) | Err(s) => s,
			                             }
		                             })
		                             .filter(|arg| arg == &OsStr::new(PlaydateTarget::Device.as_str()))
		{
			// skip special cases:
			if special_cases {
				if let Some((_, num)) = specials_vals.iter().find(|(s, _)| OsStr::new(s) == arg) {
					if *num > 0 {
						raw_args.seek(&mut cursor, SeekFrom::Current(*num as _));
					}
				}
			}
			continue;
		}
		// skip specials:
		if let Some((Ok(arg), value)) = arg.to_long() {
			if let Some((_, num)) = specials_vals.iter().find(|(s, _)| s.as_str() == arg) {
				if *num > 0 {
					raw_args.seek(&mut cursor, SeekFrom::Current(*num as _));
				} else if value.is_some() {
					raw_args.next_os(&mut cursor);
				}
				continue;
			}
		}


		// save the arg:
		if meet_tool || meet_cmd {
			result.push(raw_arg.to_owned());
		}
	}

	result
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::ffi::OsStr;
	use std::path::Path;


	#[test]
	fn args_commands_not_missed() -> CargoResult<()> {
		let tool = tool(Some(&aliases()));
		let subs: Vec<_> = tool.get_subcommands()
		                       .filter(|cmd| cmd.get_name() == CMD_NAME)
		                       .flat_map(|cmd| cmd.get_subcommands())
		                       .map(|cmd| cmd.get_name())
		                       .collect();

		for expected in Cmd::ALL {
			let expected = expected.as_str();
			assert!(
			        subs.contains(&expected),
			        "Subcommand '{}' doesn't found",
			        &expected
			);
		}
		Ok(())
	}


	#[test]
	fn args_global() -> CargoResult<()> {
		let globals = opts::globals();
		let args = [CMD_NAME, "-vvq", "--frozen", "--locked", "--offline"].into_iter()
		                                                                  .map(|s| OsStr::new(s));
		globals.try_get_matches_from(args)?;
		Ok(())
	}


	#[test]
	fn args() -> CargoResult<()> {
		for cmd in Cmd::ALL {
			let args = ["cargo", CMD_NAME, &cmd].into_iter()
			                                    .map(|s| OsStr::new(s))
			                                    .chain(
			                                           // add required args:
			                                           if matches!(cmd, Cmd::New) {
				                                           vec![OsStr::new("path")]
			                                           } else {
				                                           vec![]
			                                           }.into_iter(),
			);
			let matches = tool(Some(&aliases())).try_get_matches_from(args)?;
			let main = matches.subcommand_matches(CMD_NAME)
			                  .expect(&format!("{CMD_NAME} not matched"));
			assert_eq!(main.subcommand_name(), Some(cmd.as_str()), "{}", cmd.as_str());
		}
		Ok(())
	}


	#[test]
	fn args_build() -> CargoResult<()> {
		let args = [
		            "cargo",
		            CMD_NAME,
		            "build",
		            "-vv",
		            "--target-dir=./TARGET-DIR/",
		            "--bin=ololo",
		            "--release",
		            "--features=foo,bar",
		            "--target=thumbv7em-none-eabihf",
		            "--all",
		].into_iter()
		           .map(|s| OsStr::new(s));

		let matches = tool(Some(&aliases())).try_get_matches_from(args.clone())?;
		assert!(matches.args_present());

		Ok(())
	}

	#[test]
	fn args_new() -> CargoResult<()> {
		let args = ["cargo", CMD_NAME, &Cmd::New, "path", "-vv"].into_iter()
		                                                        .map(|s| OsStr::new(s));
		let matches = tool(Some(&aliases())).try_get_matches_from(args.clone())?;
		assert!(matches.args_present());

		Ok(())
	}

	#[test]
	fn init_cargo() -> CargoResult<()> {
		let mut cwd = std::env::current_dir()?;
		if !cwd.ends_with("tests/crates/simple/no-cfg") {
			// this check needed because tests can run in one process
			cwd.push(PathBuf::from("tests/crates/simple/no-cfg"))
		}
		println!("PWD: {}", cwd.display());
		std::env::set_current_dir(cwd)?;

		let args = [
		            "cargo",
		            CMD_NAME,
		            "-v",
		            "build",
		            "--sdk=ololo",
		            "-vv",
		            "-Zunstable-options",
		            "--target-dir=./TARGET-DIR/",
		            "--release",
		            "--features=foo,bar",
		            "--simulator",
		            "--device",
		            "--all",
		            "--",
		            "--unknown-arg",
		].into_iter()
		           .map(|s| OsStr::new(s));

		let mut config = CargoConfig::default()?;
		let result = initialize_from(args, &mut config)?;

		assert_eq!(
		           &result.workspace.target_dir().display().to_string(),
		           "./TARGET-DIR/"
		);
		assert!(result.compile_options
		              .build_config
		              .requested_kinds
		              .contains(&PlaydateTarget::Simulator.into()));
		assert!(result.compile_options
		              .build_config
		              .requested_kinds
		              .contains(&PlaydateTarget::Device.into()));
		assert_eq!(2, result.compile_options.build_config.requested_kinds.len());

		assert_eq!(result.sdk_path.as_deref(), Some(Path::new("ololo")));
		assert!(matches!(result.cmd, Cmd::Build));

		Ok(())
	}

	#[test]
	fn init_standalone() -> CargoResult<()> {
		let mut cwd = std::env::current_dir()?;
		{
			let crate_path = PathBuf::from("tests/crates/simple/no-cfg");
			if !cwd.ends_with(&crate_path) {
				// this check needed because tests can run in one process
				cwd.push(crate_path)
			}
		}
		println!("PWD: {}", cwd.display());
		std::env::set_current_dir(cwd)?;

		let args = [
		            CMD_NAME,
		            "build",
		            "--sdk=ololo",
		            "-vv",
		            "--target-dir=./TARGET-DIR/",
		            "--release",
		            "--features=foo,bar",
		            "--target=thumbv7em-none-eabihf",
		            "--simulator",
		            "--device",
		            "--all",
		            "--",
		            "--unknown-arg",
		].into_iter()
		           .map(|s| OsStr::new(s));

		let mut config = CargoConfig::default()?;
		let result = initialize_from(args, &mut config)?;

		assert_eq!(
		           &result.workspace.target_dir().display().to_string(),
		           "./TARGET-DIR/"
		);
		assert_eq!(2, result.compile_options.build_config.requested_kinds.len());
		assert_eq!(result.sdk_path.as_deref(), Some(Path::new("ololo")));
		assert!(matches!(result.cmd, Cmd::Build));
		Ok(())
	}


	#[test]
	fn init_args_before_cmd() -> CargoResult<()> {
		let mut cwd = std::env::current_dir()?;
		cwd.extend(["tests", "crates", "simple", "no-cfg"].iter());
		println!("PWD: {}", cwd.display());
		std::env::set_current_dir(cwd)?;

		let args = [
		            "cargo",
		            CMD_NAME,
		            "--sdk",
		            Cmd::Assets.as_ref(), // conflicting with command
		            "-vC",
		            Cmd::Run.as_ref(), // conflicting with command
		            "-Zunstable-options",
		            "-v",
		            Cmd::Build.as_ref(), // command
		            "-vv",
		            "--target-dir",
		            "./TARGET-DIR/",
		            "--release",
		            "--features=foo,bar",
		            "--target",
		            "thumbv7em-none-eabihf",
		            "--target=thumbv7em-none-eabihf",
		            "--simulator",
		            "--device",
		            "--all",
		            "--",
		            "--unknown-arg",
		].into_iter()
		           .map(|s| OsStr::new(s));

		let mut config = CargoConfig::default()?;
		let result = initialize_from(args, &mut config)?;

		assert_eq!(
		           &result.workspace.target_dir().display().to_string(),
		           "./TARGET-DIR/"
		);
		assert_eq!(Some(Path::new(Cmd::Assets.as_ref())), result.sdk_path.as_deref());
		assert_eq!(2, result.compile_options.build_config.requested_kinds.len());
		assert!(matches!(result.cmd, Cmd::Build));
		Ok(())
	}


	fn aliases() -> HashMap<&'static str, &'static str> { [("b", "build")].into_iter().collect() }
}
