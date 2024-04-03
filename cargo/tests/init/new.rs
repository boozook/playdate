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

	let crate_path = target_dir().join(format!("create-new--{crate_name}"));

	if crate_path.try_exists()? {
		std::fs::remove_dir_all(&crate_path)?;
	}

	let crate_parent = crate_path.parent().expect("parent");
	if !crate_parent.try_exists()? {
		std::fs::create_dir_all(&crate_parent)?;
	}

	let mut extra = vec![OsString::from(&crate_path)];
	if let Some(cty_arg) = lib.map(|v| v.then_some("--lib").or(Some("--bin"))).flatten() {
		extra.push(OsString::from(cty_arg));
	}

	let args = extra.into_iter().chain(args.into_iter().map(Into::into));
	let output = Tool::new(&crate_parent, args)?;
	assert!(output.status.success());
	Ok((output, crate_path))
}


#[test]
#[cfg_attr(not(init_tests), ignore = "set RUSTFLAGS='--cfg init_tests' to enable.")]
fn create_lib() -> Result<()> {
	let args = ["--full-config", "--full-metadata"].into_iter().map(OsStr::new);

	let (_, crate_dir) = run("new-lib", Some(true), args)?;

	assert!(crate_dir.join("Cargo.toml").exists());
	assert!(crate_dir.join("src").join("lib.rs").exists());
	assert!(!crate_dir.join("src").join("main.rs").exists());

	Ok(())
}


#[test]
#[cfg_attr(not(init_tests), ignore = "set RUSTFLAGS='--cfg init_tests' to enable.")]
fn create_bin() -> Result<()> {
	let args = ["--full-config", "--full-metadata"].into_iter().map(OsStr::new);

	let (_, crate_dir) = run("new-bin", Some(false), args)?;

	assert!(crate_dir.join("Cargo.toml").exists());
	assert!(crate_dir.join("src").join("lib.rs").exists());
	assert!(crate_dir.join("src").join("main.rs").exists());

	Ok(())
}

#[test]
#[cfg_attr(not(init_tests), ignore = "set RUSTFLAGS='--cfg init_tests' to enable.")]
fn create_default() -> Result<()> {
	let args = ["--full-config", "--full-metadata"].into_iter().map(OsStr::new);

	let (_, crate_dir) = run("new-default", None, args)?;

	assert!(crate_dir.join("Cargo.toml").exists());
	assert!(crate_dir.join("src").join("lib.rs").exists());
	assert!(!crate_dir.join("src").join("main.rs").exists());

	Ok(())
}
