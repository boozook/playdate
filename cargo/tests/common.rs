#![allow(unused_imports)]
#![allow(dead_code)]
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Once;
use std::ffi::OsStr;
use std::process::Output;
use std::process::Command;
use std::fmt::Display;
use anyhow::Result;

pub use ::build::compile::dylib_suffix_for_host;

#[path = "./shared.rs"]
mod shared;
pub use shared::*;


pub struct Tool<'p>(Cow<'p, Path>);

#[cfg(test)]
impl Default for Tool<'static> {
	fn default() -> Self { Self(Self::path().into()) }
}

#[cfg(test)]
impl Tool<'_> {
	pub fn path() -> &'static Path {
		const BIN_PATH: &str = env!("CARGO_BIN_EXE_cargo-playdate");
		Path::new(BIN_PATH)
	}
}


impl Tool<'_> {
	#[track_caller]
	pub fn execute<I, S>(pwd: &Path, args: I) -> Result<Output>
		where I: IntoIterator<Item = S>,
		      S: AsRef<OsStr> {
		let output = Self::command().current_dir(pwd).args(args).output()?;
		Ok(output)
	}

	pub fn command() -> Command {
		let mut cmd = Command::new(Self::path());
		cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
		cmd
	}


	#[track_caller]
	pub fn build<S>(pwd: &Path, args: impl IntoIterator<Item = S>) -> Result<Output>
		where S: AsRef<OsStr> + From<&'static OsStr> {
		Self::execute(pwd, [OsStr::new("build").into()].into_iter().chain(args))
	}


	#[track_caller]
	pub fn build_with<S>(pwd: &Path,
	                     args_before_cmd: Option<impl IntoIterator<Item = S>>,
	                     args: impl IntoIterator<Item = S>)
	                     -> Result<Output>
		where S: AsRef<OsStr> + From<&'static OsStr>
	{
		Self::execute(
		              pwd,
		              args_before_cmd.into_iter()
		                             .flatten()
		                             .chain([OsStr::new("build").into()])
		                             .chain(args),
		)
	}

	#[track_caller]
	pub fn init<S>(pwd: &Path, args: impl IntoIterator<Item = S>) -> Result<Output>
		where S: AsRef<OsStr> + From<&'static OsStr> {
		Self::execute(pwd, [OsStr::new("init").into()].into_iter().chain(args))
	}

	#[track_caller]
	pub fn new<S>(pwd: &Path, args: impl IntoIterator<Item = S>) -> Result<Output>
		where S: AsRef<OsStr> + From<&'static OsStr> {
		Self::execute(pwd, [OsStr::new("new").into()].into_iter().chain(args))
	}
}


pub fn simple_crates() -> Result<impl Iterator<Item = PathBuf>> {
	let root = Path::new("tests/crates/simple");
	assert!(root.exists());

	let crates = std::fs::read_dir(root)?.filter_map(|entry| entry.ok())
	                                     .filter(|entry| entry.path().is_dir())
	                                     .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
	                                     .map(|entry| entry.path());
	Ok(crates)
}

pub fn auto_target() -> Result<&'static Path> {
	let root = Path::new("tests/crates/auto-target");
	assert!(root.exists());
	Ok(root)
}

pub fn workspace() -> Result<&'static Path> {
	let root = Path::new("tests/crates/workspace");
	assert!(root.exists());
	Ok(root)
}

pub fn metadata_workspace() -> Result<&'static Path> {
	let root = Path::new("tests/crates/metadata");
	assert!(root.exists());
	Ok(root)
}


pub fn to_dyn_lib_name<S: Display>(crate_name: S) -> String {
	let prefix = if cfg!(windows) { "" } else { "lib" };
	let suffix = dylib_suffix_for_host();
	format!("{prefix}{crate_name}.{suffix}")
}


pub fn target_dir() -> &'static Path {
	let tmp = Path::new(env!("CARGO_TARGET_TMPDIR"));
	if !tmp.exists() {
		std::fs::create_dir_all(tmp).expect("can't create tmp dir");
	}

	tmp
}

pub fn target_dir_rand() -> PathBuf {
	use rand::RngCore;

	// add random:
	let mut values = [0u8; 4];
	rand::thread_rng().fill_bytes(&mut values);
	let rand = values.into_iter()
	                 .map(|v| v.to_string())
	                 .collect::<Vec<_>>()
	                 .join("");

	target_dir().join(rand)
}

pub fn target_triple() -> String {
	if cfg!(target_os = "macos") {
		format!("{}-{}-darwin", target::arch(), target::vendor())
	} else {
		format!(
		        "{}-{}-{}-{}",
		        target::arch(),
		        target::vendor(),
		        target::os(),
		        target::env()
		)
	}
}


// (issue: #315) Convert dir-name to package-name, then to crate_name
pub fn to_cargo_package_crate_name(path: &Path) -> Option<String> {
	Some(path.file_name()?.to_str()?.replace('-', "_"))
}
