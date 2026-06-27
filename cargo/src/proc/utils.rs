use std::borrow::Cow;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::*;
use std::env;
use std::process::Command;

use build::consts::SDK_ENV_VAR;
use cargo::CargoResult;
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
	let mut proc = cargo(Some(cfg))?;
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


pub fn cargo(cfg: Option<&Config>) -> CargoResult<std::process::Command> {
	let mut proc = cargo_cmd(cfg);

	if let Some(cfg) = cfg.map(|cfg| cfg.workspace.gctx()) {
		// transparent env:
		cfg.env_config()?.iter().for_each(|(k, v)| {
			                        let value = v; // resolve(cfg)?
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


pub fn cargo_cmd(cfg: Option<&Config>) -> std::process::Command {
	fn cargo_path<'t>(cfg: Option<&'t Config<'t>>) -> (Cow<'t, Path>, Option<&str>) {
		if let Some(cfg) = cfg {
			let path = cfg.workspace.gctx().cargo_exe().log_err().ok().map(Cow::from);
			if path.is_some() && path == std::env::current_exe().log_err().ok().map(Into::into) {
				// Seems to we're in standalone mode.
				cargo_path(None)
			} else if let Some(path) = path {
				(path, None)
			} else {
				cargo_path(None)
			}
		} else if let Some(path) = env::var_os("CARGO") {
			(PathBuf::from(path).into(), None)
		} else {
			let arg = cfg.and_then(|cfg| cfg.rustup.as_os_str()).map(|_| "+nightly");
			(Path::new("cargo").into(), arg)
		}
	}

	let (bin, arg) = cargo_path(cfg);

	let mut proc = std::process::Command::new(bin.as_ref());
	if let Some(arg) = arg {
		proc.arg(arg);
	}

	proc
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
		cfg.workspace.gctx().shell().err().write_all(&output.stderr)?;
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
