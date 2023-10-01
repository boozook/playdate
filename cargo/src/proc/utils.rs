use std::borrow::Cow;
use std::ffi::OsString;
use std::path::*;
use std::env;
use std::process::Command;

use build::consts::SDK_ENV_VAR;
use cargo::CargoResult;
use cargo::Config as CargoConfig;

use crate::cli::cmd::Cmd;
use crate::config::Config;


pub fn cargo_proxy_cmd(cfg: &Config, cmd: &Cmd) -> CargoResult<std::process::Command> {
	let rustflags = cfg.rustflags()?.rustflags_to_args_from(cfg);
	let mut proc = cargo(Some(cfg.workspace.config()))?;
	proc.arg(cmd.as_ref());
	proc.args(&cfg.args);
	proc.args(&rustflags);

	if let Some(path) = cfg.sdk_path.as_deref() {
		proc.env(SDK_ENV_VAR, path);
	}

	Ok(proc)
}


pub fn cargo(config: Option<&CargoConfig>) -> CargoResult<std::process::Command> {
	let cargo: Cow<Path> =
		config.map_or_else(
		                   || Some(PathBuf::from(env::var_os("CARGO").unwrap_or("cargo".into())).into()),
		                   |cfg| cfg.cargo_exe().ok().map(Cow::from),
		)
		      .expect("Unable to get cargo bin from config");
	let mut proc = std::process::Command::new(cargo.as_ref());

	if let Some(cfg) = &config {
		// transparent env:
		cfg.env_config()?.iter().for_each(|(k, v)| {
			                        let value = v.resolve(&cfg);
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
		proc.arg(format!("--color={color}"));
	}

	// disable progress bar:
	proc.env("CARGO_TERM_PROGRESS_WHEN", "never");
	Ok(proc)
}


pub fn args_line_for_proc(proc: &Command) -> String {
	proc.get_args()
	    .collect::<Vec<_>>()
	    .join(&OsString::from(" "))
	    .to_string_lossy()
	    .to_string()
}
