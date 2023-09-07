use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Output;
use anyhow::Result;

use crate::common::*;


fn run(crate_name: &str,
       lib: Option<bool>,
       args: impl IntoIterator<Item = impl Into<OsString>>)
       -> Result<(Output, PathBuf)> {
	println!("crate: {}", crate_name);

	let crate_path = target_dir().join("create-init-tests").join(&crate_name);

	if crate_path.try_exists()? {
		std::fs::remove_dir_all(&crate_path)?;
		std::fs::create_dir_all(&crate_path)?;
	} else {
		std::fs::create_dir_all(&crate_path)?;
	}

	let mut extra = vec![];
	if let Some(cty_arg) = lib.map(|v| v.then_some("--lib").or(Some("--bin"))).flatten() {
		extra.push(OsString::from(cty_arg));
	}

	let args = extra.into_iter().chain(args.into_iter().map(Into::into));
	let output = Tool::init(&crate_path, args)?;
	assert!(output.status.success());
	Ok((output, crate_path))
}


#[test]
fn create_lib() -> Result<()> {
	let args = ["--full-config", "--full-metadata"].into_iter().map(OsStr::new);

	let (_, crate_dir) = run("init-lib", Some(true), args)?;

	assert!(crate_dir.join("Cargo.toml").exists());
	assert!(crate_dir.join("src").join("lib.rs").exists());
	assert!(!crate_dir.join("src").join("main.rs").exists());

	Ok(())
}


#[test]
fn create_bin() -> Result<()> {
	let args = ["--full-config", "--full-metadata"].into_iter().map(OsStr::new);

	let (_, crate_dir) = run("init-bin", Some(false), args)?;

	assert!(crate_dir.join("Cargo.toml").exists());
	assert!(crate_dir.join("src").join("lib.rs").exists());
	assert!(crate_dir.join("src").join("main.rs").exists());

	Ok(())
}

#[test]
fn create_default() -> Result<()> {
	let args = ["--full-config", "--full-metadata"].into_iter().map(OsStr::new);

	let (_, crate_dir) = run("init-default", None, args)?;

	assert!(crate_dir.join("Cargo.toml").exists());
	assert!(crate_dir.join("src").join("lib.rs").exists());
	assert!(!crate_dir.join("src").join("main.rs").exists());

	Ok(())
}
