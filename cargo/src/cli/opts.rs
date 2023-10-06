use std::collections::HashMap;
use std::ffi::OsString;

use cargo::core::compiler::CompileKind;
use cargo::util::command_prelude::{CommandExt, flag, opt, multi_opt};
use clap::builder::{Str, ArgPredicate};
use clap::{Arg, ArgAction, value_parser, Args};
use clap::Command;
use playdate::consts::{SDK_ENV_VAR, DEVICE_TARGET};
use playdate::toolchain::gcc::ARM_GCC_PATH_ENV_VAR;
use tool::cli::run::DeviceDestination;
use tool::cli::mount::Mount;


use super::{Cmd, CMD_NAME, BIN_NAME};
use super::target::PlaydateTarget;


pub fn special_args_global() -> Vec<Arg> {
	vec![
	     Arg::new("sdk").help("Path to Playdate SDK")
	                    .long("sdk")
	                    .env(SDK_ENV_VAR)
	                    .conflicts_with("no-sdk")
	                    .value_name("DIRECTORY")
	                    .value_hint(clap::ValueHint::DirPath)
	                    .value_parser(clap::builder::ValueParser::path_buf())
	                    .num_args(1)
	                    .global(true),
	     Arg::new("gcc").help("Path to GCC in ARM GNU toolchain, usually named 'arm-none-eabi-gcc'.")
	                    .long("gcc")
	                    .env(ARM_GCC_PATH_ENV_VAR)
	                    .conflicts_with("no-gcc")
	                    .value_name("EXECUTABLE")
	                    .value_parser(clap::builder::ValueParser::path_buf())
	                    .num_args(1)
	                    .global(true),
	     // XXX: no-sdk & no-gcc are hidden because not supported yet:
	     flag("no-sdk", "Do not use Playdate SDK").global(true).hide(true),
	     flag("no-gcc", "Do not use ARM GNU toolchain, but Rust & LLVM only.").global(true)
	                                                                          .hide(true)
	]
}

pub fn special_args_for(cmd: &Cmd) -> Vec<Arg> {
	match cmd {
		Cmd::Build => shorthands_for(cmd),
		Cmd::Run => {
			let mut args = mount();
			args.append(&mut shorthands_for(cmd));
			args.push(flag_no_wait());
			args.push(flag_no_info_file());
			args
		},
		Cmd::Package => {
			let mut args = special_args_for(&Cmd::Build);
			args.push(flag_zip_package());
			args.push(flag_no_info_file());
			args
		},
		Cmd::Assets => vec![flag_pdc_skip_unknown()],

		Cmd::New => {
			vec![
			     flag_create_full_config(),
			     flag_create_full_metadata(),
			     flag_create_deps_sys_only(),
			     flag_create_deps_list(),
			     ide_template(),
			]
		},
		Cmd::Init => special_args_for(&Cmd::New),
		Cmd::Migrate => vec![],
		Cmd::Publish => vec![],
	}
}

fn shorthands_for(cmd: &Cmd) -> Vec<Arg> {
	match cmd {
		Cmd::Build => {
			vec![
			     flag_playdate_device(),
			     flag_playdate_simulator(),
			     flag_no_unwinding()
			]
		},
		Cmd::Run => {
			vec![
			     flag_playdate_simulator().conflicts_with_all(["device", "mounting"]),
			     flag_no_unwinding()
			]
		},

		Cmd::New => vec![],
		Cmd::Init => vec![],
		Cmd::Migrate => vec![],
		Cmd::Package => vec![],
		Cmd::Assets => vec![],
		Cmd::Publish => vec![],
	}
}


fn build() -> Command {
	Command::new(Cmd::Build.as_ref()).ignore_errors(true)
	                     .about("Compile a local package and all of its dependencies")
	                     .arg_release("Build artifacts in release mode, with optimizations")
	                     .arg_profile("Build artifacts with the specified profile")
	                     .arg_build_plan()
	                     .arg_unit_graph()
	                     .arg_package_spec(
	                                       "Package to build (see `cargo help pkgid`)",
	                                       "Alias for --workspace (deprecated)",
	                                       "Exclude packages from the build",
	)
	                     .arg_jobs()
	                     .arg_targets_all(
	                                      "Build only this package's library",
	                                      "Build only the specified binary",
	                                      "Build all binaries",
	                                      "Build only the specified example",
	                                      "Build all examples",
	                                      "Build only the specified test target",
	                                      "Build all tests",
	                                      "Build only the specified bench target",
	                                      "Build all benches",
	                                      "Build all targets",
	)
	                     .arg_future_incompat_report()
	                     .arg_timings()
	                     // hide currently not supported targets:
	                     .mut_arg("test", |arg| arg.hide(true))
	                     .mut_arg("tests", |arg| arg.hide(true))
	                     .mut_arg("bench", |arg| arg.hide(true))
	                     .mut_arg("benches", |arg| arg.hide(true))
	                     // add exclusive shorthands:
	                     .args(special_args_for(&Cmd::Build))
								.arg(extra_arg())
}


fn run() -> Command {
	Command::new(Cmd::Run.as_ref()).ignore_errors(true)
	                               .about("Run a binary or example of the local package on a device or simulator")
	                               .arg_release("Build artifacts in release mode, with optimizations")
	                               .arg_profile("Build artifacts with the specified profile")
	                               .arg_build_plan()
	                               .arg_unit_graph()
	                               .arg_package_spec(
	                                                 "Package to build (see `cargo help pkgid`)",
	                                                 "Alias for --workspace (deprecated)",
	                                                 "Exclude packages from the build",
	)
	                               .arg_jobs()
	                               .arg_targets_lib_bin_example(
	                                                            "Build only this package's library",
	                                                            "Build only the specified binary",
	                                                            "Build all binaries",
	                                                            "Build only the specified example",
	                                                            "Build all examples",
	)
	                               .arg_future_incompat_report()
	                               .arg_timings()
	                               // add exclusive shorthands:
	                               .args(special_args_for(&Cmd::Run))
											 .arg(extra_arg())
}

fn package() -> Command {
	// extend `build` command:
	build().name(Cmd::Package.as_ref())
	.arg(flag_zip_package()).about("Compile a local package and all of its dependencies, build assets for them, manifests for local crates and pack it all together.")
}

fn migrate() -> Command {
	Command::new(Cmd::Migrate.as_ref()).ignore_errors(true)
	                                   .about("non implemented yet")
}
fn publish() -> Command {
	Command::new(Cmd::Publish.as_ref()).arg(flag_zip_package().default_value("true"))
	                                   .ignore_errors(true)
	                                   .about("non implemented yet")
}


fn assets() -> Command {
	Command::new(Cmd::Assets.as_ref()).ignore_errors(true)
	                                  .about("Collect assets for a local package and all of its dependencies")
	                                  .arg_build_plan()
	                                  .arg_unit_graph()
	                                  .arg_package_spec(
	                                                    "Package to build (see `cargo help .pkgid`)",
	                                                    "Alias for --workspace (deprecated)",
	                                                    "Exclude packages from the build",
	)
	                                  // add exclusive shorthands:
	                                  .args(special_args_for(&Cmd::Assets))
	                                  .arg(extra_arg())
}


fn new_crate() -> Command {
	Command::new(Cmd::New.as_ref()).ignore_errors(true)
	                               .about("Create a new cargo package at <path>")
	                               .arg_new_opts()
											 .arg(Arg::new("path").num_args(1)
	                                                    .required(true)
	                                                    .value_hint(clap::ValueHint::DirPath)
	                                                    .value_parser(clap::builder::ValueParser::path_buf()))
											 // add exclusive shorthands:
											 .args(special_args_for(&Cmd::New))
}

fn init_crate() -> Command {
	new_crate().name(Cmd::Init.as_ref())
	           .about("Create a new cargo package in an existing directory")
	           .mut_arg("path", |arg| arg.required(false).default_value("."))
}


fn mount() -> Vec<Arg> {
	let mount: Command =
		Mount::augment_args(Command::new("mount")).mut_arg("device", |arg| arg.long("device").num_args(0..=1));
	mount.get_arguments()
	     .cloned()
	     .map(|arg| arg.group("mounting"))
	     .collect()
}

fn flag_no_wait() -> Arg {
	DeviceDestination::augment_args(Command::new("dest")).get_arguments()
	                                                     .find(|arg| arg.get_id().as_str() == "no-wait")
	                                                     .expect("Arg no-wait")
	                                                     .to_owned()
}


/// Extra args for cargo that we are not support.
/// Usage: `matches.get_many::<OsString>("EXTRA").into_iter().flatten()`
fn extra_arg() -> Arg {
	Arg::new("EXTRA").help("Additional arguments for underlying cargo, better starts with `--`, but it's not required.")
	                 .trailing_var_arg(true)
	                 .num_args(1..)
	                 .action(ArgAction::Append)
	                 .value_parser(value_parser!(OsString))
}


pub fn mapped_flags_playdate_target() -> HashMap<PlaydateTarget, CompileKind> {
	let mut map = HashMap::new();
	map.insert(PlaydateTarget::Device, PlaydateTarget::Device.into());
	map.insert(PlaydateTarget::Simulator, PlaydateTarget::Simulator.into());
	map
}


fn flag_playdate_device() -> Arg {
	let name = PlaydateTarget::Device.as_str();
	Arg::new(&name).long(&name)
	               .help(format!("Shorthand for '--target={DEVICE_TARGET}'"))
	               .action(ArgAction::SetTrue)
}

fn flag_playdate_simulator() -> Arg {
	let name = PlaydateTarget::Simulator.as_str();
	Arg::new(&name).long(&name)
	               .help(format!("Shorthand for host target"))
	               .long_help("Shorthand for host target. Default target, so it can be omitted if there's only one target.")
	               .action(ArgAction::SetTrue)
}

fn flag_pdc_skip_unknown() -> Arg {
	let name = "skip-unknown";
	Arg::new(&name).long(&name)
	               .help(format!("Tell pdc to skip unknown files"))
	               .conflicts_with("no-sdk")
	               .action(ArgAction::SetTrue)
}

fn flag_zip_package() -> Arg {
	let name = "zip";
	Arg::new(&name).long(&name)
	               .help(format!("Make an archive with the produced package"))
	               .action(ArgAction::SetTrue)
}

fn flag_no_info_file() -> Arg {
	let name = "no-info-file";
	Arg::new(&name).long(&name)
	               .help(format!("Opt-out inclusion info file with builder version into the produced package"))
	               .action(ArgAction::SetTrue)
}

fn flag_no_unwinding() -> Arg {
	let name = "no-unwinding";
	const SHORT: &str =
		"Prevents unwinding, shorthand for `panic=abort` rustc flag and `panic_immediate_abort` feature.";
	const LONG: &str ="Prevents unwinding. Adds `-Cpanic=abort` to `RUSTFLAGS` so that build profiles do not need to specify `panic = \"abort\"` in the cargo manifest. Also adds `-Zbuild-std-features=panic_immediate_abort` to ensure that there is no `core::panicking` in the product.";
	Arg::new(&name).long(&name)
	               .help(SHORT)
	               .long_help(LONG)
	               .action(ArgAction::SetTrue)
}

fn flag_create_full_config() -> Arg {
	let name = "full-config";
	let help = format!("Create a full cargo config file with hardcoded link-paths that required to compile bin.");
	let long =
		format!("{help} Usually you don't need to use this flag when using {BIN_NAME} because {BIN_NAME} adds them itself when compiling.");
	Arg::new(&name).long(&name)
	               .help(help)
	               .long_help(long)
	               .action(ArgAction::SetTrue)
}

fn flag_create_full_metadata() -> Arg {
	let name = "full-metadata";
	let help = format!("Create a template with complex metadata example.");
	Arg::new(&name).long(&name).help(help).action(ArgAction::SetTrue)
}

fn flag_create_deps_sys_only() -> Arg {
	let name = "sys-only";
	let help = r#"Add only "playdate-sys" dependency, use low-level template, don't add high-level dependencies."#;
	Arg::new(&name).long(&name).help(help).action(ArgAction::SetTrue)
}

fn flag_create_deps_list() -> Arg {
	let name = "deps";
	let help = r#"Space or comma separated list of dependencies to add. Format: 'crate-name[:git|crates]', source is usable for known crates only. "#;

	let arg = Arg::new(&name).long(&name)
	                         .help(help)
	                         .required(false)
	                         .num_args(1)
	                         .value_name("DEPS")
	                         .action(ArgAction::Append)
	                         .default_values(["playdate"])
	                         .default_missing_values(["playdate:git"])
	                         .value_delimiter(',')
	                         .default_value_if("sys-only", ArgPredicate::Equals("true".into()), "sys");

	let possible_values = arg.clone()
	                         .value_parser(value_parser!(super::deps::Dependency))
	                         .get_possible_values();
	let mut long = "Possible values:".to_string();
	for val in possible_values {
		let head = format!(
		                   "\n  {}",
		                   val.get_name_and_aliases().collect::<Vec<_>>().join(", ")
		);
		let tail = val.get_help().map(|s| format!(": {s}")).unwrap_or_default();
		long.push_str(&format!("{head}{tail}"));
	}
	arg.long_help(long)
}

fn ide_template() -> Arg {
	let name = "ide";
	let help = r#"Add configuration files for given IDE."#;
	Arg::new(&name).long(&name)
	               .help(help)
	               .num_args(1)
	               .required(false)
	               .action(ArgAction::Set)
	               .value_hint(clap::ValueHint::Other)
	               .value_parser(value_parser!(super::ide::Ide))
}

fn set_aliases(cmd: Command, aliases: Option<&HashMap<impl Into<Str> + Clone, impl AsRef<str>>>) -> Command {
	if let Some(aliases) = aliases {
		let name = cmd.get_name();
		let aliases = aliases.into_iter()
		                     .filter(|(_, v)| v.as_ref() == name)
		                     .map(|(k, _)| k.clone().into())
		                     .collect::<Vec<Str>>();
		cmd.visible_aliases(aliases)
	} else {
		cmd
	}
}


fn cargo() -> Command {
	let usage = usage();
	clap::command!("cargo").override_usage(&usage)
	                       .propagate_version(true)
	                       .bin_name("cargo")
}

pub fn tool(aliases: Option<&HashMap<impl Into<Str> + Clone, impl AsRef<str>>>) -> Command {
	let mut cargo = cargo().arg_required_else_help(true).subcommand(main(aliases));
	cargo.build();
	cargo
}

pub fn main(aliases: Option<&HashMap<impl Into<Str> + Clone, impl AsRef<str>>>) -> Command {
	let tool = cargo().name(CMD_NAME);
	add_globals(tool).subcommand_required(true)
	                 .subcommand(set_aliases(build(), aliases))
	                 .subcommand(run())
	                 .subcommand(assets())
	                 .subcommand(new_crate())
	                 .subcommand(init_crate())
	                 .subcommand(package())
	                 .subcommand(migrate().hide(true))
	                 .subcommand(publish().hide(true))
}


/// returns fake command with global args
pub fn globals() -> Command { add_globals(Command::new("")) }


fn add_globals(cmd: Command) -> Command {
	let cmd =
		cmd.arg_dry_run("Enable dry-run mode")
		   .arg_quiet()
		   .arg(opt("verbose", "Use verbose output (-vv extra verbose output)").short('v')
		                                                                       .env("CARGO_TERM_VERBOSE")
		                                                                       .action(ArgAction::Count)
		                                                                       .global(true))
		   .arg(opt("color", "Coloring: auto, always, never").value_name("WHEN")
		                                                     .global(true))
		   .arg(Arg::new("directory").global(true)
		                             .help("Change to DIRECTORY before doing anything (nightly-only)")
		                             .short('C')
		                             .value_name("DIRECTORY")
		                             .alias("dir")
		                             .short_alias('D')
		                             .value_hint(clap::ValueHint::DirPath)
		                             .value_parser(clap::builder::ValueParser::path_buf()))
		   .arg(flag("frozen", "Require Cargo.lock and cache are up to date").global(true))
		   .arg(flag("locked", "Require Cargo.lock is up to date").global(true))
		   .arg(flag("offline", "Run without accessing the network").global(true))
		   .arg(multi_opt("config", "KEY=VALUE", "Override a configuration value").global(true))
		   .arg(Arg::new("unstable-features").help("Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details")
		                                     .short('Z')
		                                     .value_name("FLAG")
		                                     .action(ArgAction::Append)
		                                     .global(true))
		   .arg_target_dir()
		   .arg_features()
		   .arg_target_triple("Build for the target triple")
		   .arg_manifest_path()
		   .arg_message_format();

	let cmd = cmd.mut_arg("quiet", |arg| arg.global(true).env("CARGO_TERM_QUIET"))
	             .mut_arg("dry-run", |arg| arg.global(true))
	             .mut_arg("target-dir", |arg| arg.global(true))
	             .mut_arg("features", |arg| arg.global(true))
	             .mut_arg("all-features", |arg| arg.global(true))
	             .mut_arg("no-default-features", |arg| arg.global(true))
	             .mut_arg("target", |arg| arg.global(true))
	             .mut_arg("manifest-path", |arg| arg.global(true))
	             .mut_arg("message-format", |arg| arg.global(true))
	             .mut_arg("target", |arg| arg.num_args(1..=1));

	// Exclusive opts for this tool only, it's not for cargo:
	cmd.args(special_args_global())
}


fn usage() -> String {
	let rustup = std::env::var("RUSTUP_HOME").ok();
	let (rustup_current, is_nightly) = if let Some(toolchain) = std::env::var("RUSTUP_TOOLCHAIN").ok().as_deref() {
		(format!(" Currently using '{toolchain}'."), toolchain.starts_with("nightly-"))
	} else {
		(Default::default(), false)
	};

	let toolchain_help = if !is_nightly {
		format!("Toolchain: {BIN_NAME} requires nightly rust.{rustup_current}\n")
	} else {
		Default::default()
	};
	let toolchain_help_no_rustup = format!("Note: {BIN_NAME} requires nightly rust toolchain.\n");
	format!(
	        include_str!("usage.txt"),
	        NAME = CMD_NAME,
	        BIN_NAME = BIN_NAME,
	        toolchain = {
		        if rustup.is_some() {
			        " [+toolchain]"
		        } else {
			        ""
		        }
	        },
	        toolchain_usage = {
		        if rustup.is_some() {
			        &toolchain_help
		        } else {
			        &toolchain_help_no_rustup
		        }
	        },
	).trim()
	.into()
}
