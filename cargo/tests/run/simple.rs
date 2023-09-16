use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;
use anyhow::Result;

use crate::common::*;


fn run(crate_name: &str, crate_path: &Path, args: impl IntoIterator<Item = impl Into<OsString>>) -> Result<()> {
	println!("crate: {}", crate_path.display());

	let value = test_value();
	let target_dir = target_dir().join(crate_name);
	let target_dir_arg = format!("--target-dir={}", target_dir.display());
	let args = [OsString::from("run")].into_iter()
	                                  .chain(args.into_iter().map(Into::into))
	                                  .chain([OsString::from(target_dir_arg)]);
	if !target_dir.exists() {
		std::fs::create_dir_all(&target_dir)?;
	}

	let mut cmd = Tool::command();
	cmd.args(args);
	cmd.current_dir(crate_path);
	cmd.env(CARGO_PLAYDATE_TEST_VALUE_ENV, &value);


	let expected_value = format!("{CARGO_PLAYDATE_TEST_VALUE_PREFIX}{value}");

	let log_path = target_dir.join("stdout.log");
	let file = File::create(&log_path)?;
	cmd.stdout(Stdio::from(file));
	let mut handle = cmd.spawn()?;

	println!("cmd: {:?}", cmd);

	let start = Instant::now();
	let limit = Duration::from_secs(60 * 1);


	while start.elapsed() < limit {
		std::thread::sleep(Duration::from_millis(100))
	}

	println!("timeout reached");

	let pid = handle.id();
	#[cfg(unix)]
	{
		use nix::unistd::Pid;
		use nix::sys::signal::{self, Signal};
		signal::kill(
		             Pid::from_raw(pid.try_into().expect("invalid PID")),
		             Signal::SIGTERM, // mb. send SIGKILL?
		).ok();
	}
	handle.kill()
	      .expect(&format!("Unable to kill child process (pid={pid}"));

	println!("Child process killed");

	// take a time to flush:
	std::thread::sleep(Duration::from_secs(4));

	println!("Reading log");

	let file = File::open(log_path)?;
	let found = BufReader::new(file).lines()
	                                .map_while(|r| r.ok())
	                                .find(|line| line.trim().contains(&expected_value));
	println!("Found expected value: {:?}", found);

	assert!(found.is_some(), "Test value didn't found.");

	Ok(())
}


fn test_value() -> String {
	use rand::RngCore;
	let mut values = [0u8; 8];
	rand::thread_rng().fill_bytes(&mut values);
	let rand = values.into_iter().fold(String::new(), |mut acc, n| {
		                             acc.push_str(&n.to_string());
		                             acc
	                             });
	rand
}


#[test]
fn run_metadata_workspace_root_dev() -> Result<()> {
	let crate_name = "test-workspace-main-crate";
	let args = ["--simulator", "-p", crate_name, "--lib"].into_iter()
	                                                     .map(OsStr::new);
	let ws = metadata_workspace()?;
	run(crate_name, &ws, args)?;

	Ok(())
}

#[test]
fn run_metadata_workspace_root_release() -> Result<()> {
	let crate_name = "test-workspace-main-crate";
	let args = ["--simulator", "-p", crate_name, "--lib", "--release"].into_iter()
	                                                                  .map(OsStr::new);
	let ws = metadata_workspace()?;
	run(crate_name, &ws, args)?;

	Ok(())
}
