use std::io::BufRead;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use cargo::CargoResult;

use crate::config::Config;
use super::args_line_for_proc;


pub fn cmd_logged(config: &Config, mut cmd: Command) -> CargoResult<Command> {
	cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

	let tool = &Path::new(cmd.get_program());
	let tool = {
		let s = tool.file_name().unwrap_or(tool.as_os_str()).to_string_lossy();
		let mut c = s.chars();
		match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		}
	};

	config.log().verbose(|mut log| {
		            log.status(&tool, args_line_for_proc(&cmd));
	            });

	let output = cmd.output()?;
	if !output.status.success() {
		config.log().verbose(|mut log| {
			            let stdout_utf8 = std::str::from_utf8(&output.stdout);
			            let stdout_debug: &dyn std::fmt::Debug = match stdout_utf8 {
				            Ok(ref str) => str,
			               Err(_) => &output.stdout,
			            };

			            let stderr_utf8 = std::str::from_utf8(&output.stderr);
			            let stderr_debug: &dyn std::fmt::Debug = match stderr_utf8 {
				            Ok(ref str) => str,
			               Err(_) => &output.stderr,
			            };

			            let message = format!(
			                            "{tool} exited with error, code: {}\n\tstdout: {stdout_debug:?}\n\tstderr: {stderr_debug:?}",
			                            output.status
			                                  .code()
			                                  .map(|v| v.to_string())
			                                  .unwrap_or_else(|| "n/a".to_string()),
			);
			            log.status_err(message);
		            });
	} else {
		config.log_extra_verbose(|mut log| {
			      if !output.stdout.trim_ascii().is_empty() {
				      log.status(&tool, "output:");
				      output.stdout.lines().for_each(|line| {
					                           if let Ok(line) = line {
						                           log.status("", line);
					                           }
				                           });
			      }
		      });
	}

	let stderr = output.stderr.trim_ascii();
	if !stderr.is_empty() {
		if let Ok(error) = std::str::from_utf8(stderr) {
			config.log().status_err(format!("{tool} stderr:\n{error}"));
		} else {
			config.workspace.config().shell().status_header(&tool)?;
			config.workspace
			      .config()
			      .shell()
			      .err()
			      .write_all("stderr:\n".as_bytes())?;
			config.workspace.config().shell().err().write_all(stderr)?;
			config.workspace.config().shell().err().write_all(b"\n")?;
		}
	}

	Ok(cmd)
}
