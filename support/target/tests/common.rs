#![allow(unused)]
#![feature(exit_status_error)]

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Error, Result};

pub extern crate playdate_target as target;


pub fn target_dir_tmp() -> &'static Path {
	let tmp = Path::new(env!("CARGO_TARGET_TMPDIR"));
	if !tmp.exists() {
		std::fs::create_dir_all(tmp).expect("can't create tmp dir");
	}
	tmp
}

pub fn spec_dir() -> PathBuf {
	let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("spec");
	assert!(dir.exists(), "specs are gone");
	dir
}


/// Ensure exists and point to
/// `{sysroot}/etc/target-spec-json-schema.json`,
/// or export to `target_dir_tmp` that one printed by rustc.
pub fn spec_schema() -> Result<PathBuf> {
	let sysroot = tools::rustc().args(["--print", "sysroot"])
	                            .output()
	                            .map_err(Error::new)
	                            .and_then(|out| {
		                            out.status.exit_ok().context("calling nightly rustc")?;
		                            String::from_utf8(out.stdout).map_err(Error::new)
	                            })?;

	let sysroot = PathBuf::from(sysroot.trim());
	let schema = sysroot.join(PathBuf::from("etc/target-spec-json-schema.json"));
	assert!(schema.exists(), "schema not found");
	if schema.exists() {
		Ok(schema)
	} else {
		let p = target_dir_tmp().join("target-spec-json-schema.json");
		if !p.exists() {
			let mut f = std::fs::File::create(&p)?;
			tools::rustc().args(["-Zunstable-options", "--print=target-spec-json-schema"])
			              .stdout(f)
			              .status()?
			              .exit_ok()?;
		}
		Ok(p)
	}
}


pub mod tools {
	use core::cell::Cell;
	use core::sync::atomic::{AtomicBool, Ordering};
	use std::path::Path;
	use std::process::Command;


	pub fn is_rustup() -> bool { std::env::var_os("RUSTUP_HOME").is_some() }


	pub fn rustc() -> Command {
		const NAME: &str = "rustc";
		let is_rustup = is_rustup();

		{
			static OK: AtomicBool = AtomicBool::new(false);

			if !OK.load(Ordering::Relaxed) {
				let mut cmd = Command::new("rustc");
				if is_rustup {
					cmd.arg("+nightly");
				}
				cmd.arg("-V");

				if cmd.output().ok().is_some_and(|out| out.status.success()) {
					OK.store(true, Ordering::Relaxed);
				} else {
					panic!("something wrong with rustc");
				}
			}
		}

		let mut cmd = Command::new(NAME);
		if is_rustup {
			cmd.arg("+nightly");
		}
		cmd
	}


	pub fn taplo() -> Command {
		const NAME: &str = "taplo";
		{
			static OK: AtomicBool = AtomicBool::new(false);
			if !OK.load(Ordering::Relaxed) {
				Command::new(NAME).arg("-V")
				                  .output()
				                  .expect(NAME)
				                  .exit_ok()
				                  .expect(NAME);
				OK.store(true, Ordering::Relaxed);
			}
		}
		Command::new(NAME)
	}

	pub fn taplo_check_schema(schema: &Path) -> Command {
		let mut cmd = taplo();
		cmd.args(["check", "--schema"])
		   .arg(format!("file://{}", schema.display()));
		cmd
	}
}
