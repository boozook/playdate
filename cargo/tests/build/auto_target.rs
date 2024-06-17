use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Output;
use anyhow::Result;

use ::build::consts::DEVICE_TARGET;
use crate::common::*;


fn run_build(crate_path: &Path,
             args: impl IntoIterator<Item = impl Into<OsString>>)
             -> Result<(Output, &'static Path)> {
	println!("crate: {}", crate_path.display());

	let target_dir = target_dir();
	let target_dir_arg = format!("--target-dir={}", target_dir.display());
	let args = args.into_iter()
	               .map(Into::into)
	               .chain([OsString::from(target_dir_arg)]);
	let output = Tool::build(crate_path, args)?;
	assert!(
	        output.status.success(),
	        "Tool failed with stderr:\n{}",
	        std::str::from_utf8(&output.stderr).unwrap()
	);
	Ok((output, target_dir))
}

fn test_artifact_dev(path: &Path, _debug: bool) {
	println!("validating: {}", path.display());
	assert!(path.exists(), "path does not exist: {}", path.display());
}


fn export_dir(target_dir: &Path, target: &str, profile: &str) -> PathBuf { target_dir.join(target).join(profile) }


#[test]
fn bins_examples() -> Result<()> {
	let target = DEVICE_TARGET;
	let args = ["--device", "--bins", "--examples"].into_iter().map(OsStr::new);

	let path = auto_target()?;
	let (_, target_dir) = run_build(path, args.clone())?;
	let export_dir = export_dir(target_dir, target, "debug");

	// check expectations:
	let package_name = "test-auto-target";
	let cargo_package_name = to_cargo_package_crate_name(Path::new(package_name)).expect("package_crate_name");
	for trg in ["test-auto-target", "auto-bin", "example-bin"] {
		let cargo_target_fullname = format!("{cargo_package_name}-{trg}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.elf"));
		println!("should be there: {artifact:?}");
		test_artifact_dev(&artifact, false);
	}

	Ok(())
}


#[test]
#[cfg_attr(windows, ignore = "Off until paths on Windows fixed.")]
// Error on Windows:
// failed to parse value from --config argument `target.thumbv7em-none-eabihf.rustflags=["-Ctarget-cpu=cortex-m7", "-Clink-args=--emit-relocs", "-Crelocation-model=pic", "-Csoft-float=no", "-Clink-arg=--cref", "-Clink-arg=--gc-sections", "-Clink-arg=--entry=eventHandlerShim", "-Clink-arg=-T\\?\\D:\\a\playdate\\playdate\\target\\tmp\\pd.x"]` as a dotted key expression
fn bins_examples_no_gcc() -> Result<()> {
	let target = DEVICE_TARGET;
	let args = ["--device", "--bins", "--examples", "--no-gcc"].into_iter()
	                                                           .map(OsStr::new);

	let path = auto_target()?;
	let (_, target_dir) = run_build(path, args.clone())?;
	let export_dir = export_dir(target_dir, target, "debug");

	// check expectations:
	let package_name = "test-auto-target";
	let cargo_package_name = to_cargo_package_crate_name(Path::new(package_name)).expect("package_crate_name");
	for trg in ["test-auto-target", "auto-bin", "example-bin"] {
		let cargo_target_fullname = format!("{cargo_package_name}-{trg}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.elf"));
		println!("should be there: {artifact:?}");
		test_artifact_dev(&artifact, false);
	}

	Ok(())
}
