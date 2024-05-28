use std::borrow::Cow;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::*;
use std::env;
use std::process::Command;

use build::consts::SDK_ENV_VAR;
use cargo::CargoResult;
use cargo::Config as CargoConfig;
use serde::de::DeserializeOwned;

use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::logger::LogErr;


pub fn cargo_proxy_cmd(cfg: &Config, cmd: &Cmd) -> CargoResult<std::process::Command> {
	cargo_proxy_with(cfg, cmd.as_str(), true)
}


pub fn cargo_proxy_with<S: AsRef<OsStr>>(cfg: &Config,
                                         cmd: S,
                                         cfg_args: bool)
                                         -> CargoResult<std::process::Command> {
	let rustflags = cfg.rustflags()?.rustflags_to_args_from(cfg);
	let mut proc = cargo(Some(cfg.workspace.config()))?;
	proc.arg(cmd);
	if cfg_args {
		proc.args(&cfg.args);
	}
	proc.args(&rustflags);

	if let Some(path) = cfg.sdk_path.as_deref() {
		proc.env(SDK_ENV_VAR, path);
	}

	Ok(proc)
}


pub fn cargo(config: Option<&CargoConfig>) -> CargoResult<std::process::Command> {
	let cargo = cargo_bin_path(config);
	let mut proc = std::process::Command::new(cargo.as_ref());

	if let Some(cfg) = &config {
		// transparent env:
		cfg.env_config()?.iter().for_each(|(k, v)| {
			                        let value = v.resolve(cfg);
			                        proc.env(k, value);
		                        });
		// explicitly set colors:
		use cargo::core::shell::ColorChoice;
		let color_choice = cfg.shell().color_choice();
		let color = if match color_choice {
			ColorChoice::Always => true,
			ColorChoice::Never => false,
			ColorChoice::CargoAuto => cfg.shell().err_supports_color() || cfg.shell().out_supports_color(),
		} {
			"always"
		} else {
			"never"
		};
		proc.env("CARGO_TERM_COLOR", color);
	}

	// disable progress bar:
	proc.env("CARGO_TERM_PROGRESS_WHEN", "never");
	Ok(proc)
}


pub fn cargo_bin_path(config: Option<&CargoConfig>) -> Cow<Path> {
	if let Some(cfg) = config {
		let path = cfg.cargo_exe().log_err().ok().map(Cow::from);
		if path.is_some() && path == std::env::current_exe().log_err().ok().map(Into::into) {
			// Seems to we're in standalone mode.
			cargo_bin_path(None)
		} else if let Some(path) = path {
			path
		} else {
			cargo_bin_path(None)
		}
	} else {
		PathBuf::from(env::var_os("CARGO").unwrap_or("cargo".into())).into()
	}
}


pub fn args_line_for_proc(proc: &Command) -> String {
	proc.get_args()
	    .collect::<Vec<_>>()
	    .join(&OsString::from(" "))
	    .to_string_lossy()
	    .to_string()
}


pub fn read_cargo_json<T: DeserializeOwned>(cfg: &Config, mut cmd: Command) -> CargoResult<T> {
	cfg.log()
	   .verbose(|mut log| log.status("Cargo", args_line_for_proc(&cmd)));

	let output = cmd.output()?;
	if !output.status.success() {
		cfg.workspace.config().shell().err().write_all(&output.stderr)?;
		output.status.exit_ok()?;
	}

	let stdout = std::str::from_utf8(&output.stdout)?;

	// parse only last line of output:
	let line = stdout.lines()
	                 .find(|s| {
		                 let s = s.trim();
		                 !s.is_empty() && s.starts_with('{')
	                 })
	                 .unwrap_or("{}");

	Ok(serde_json::de::from_str::<T>(line)?)
}
