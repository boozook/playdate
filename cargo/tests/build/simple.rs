use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Output;
use anyhow::Result;

use playdate::consts::DEVICE_TARGET;
use crate::common::*;


fn run_build(crate_path: &Path,
             args: impl IntoIterator<Item = impl Into<OsString>>)
             -> Result<(Output, PathBuf)> {
	println!("crate: {}", crate_path.display());

	let target_dir = target_dir();
	let target_dir_arg = format!("--target-dir={}", target_dir.display());
	let args = args.into_iter()
	               .map(Into::into)
	               .chain([OsString::from(target_dir_arg)]);
	let output = Tool::build(&crate_path, args)?;
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

fn test_artifact_sim(path: &Path, _debug: bool) {
	println!("validating: {}", path.display());
	assert!(path.exists(), "path does not exist: {}", path.display());
}

fn test_artifact(path: &Path, debug: bool, dev: bool) {
	if dev {
		test_artifact_dev(path, debug)
	} else {
		test_artifact_sim(path, debug)
	}
}


fn export_dir(target_dir: &Path, target: &str, profile: &str) -> PathBuf { target_dir.join(target).join(profile) }
fn export_dir_host(target_dir: &Path, profile: &str) -> PathBuf { target_dir.join(profile) }

const LIB_NAME: &str = "game";
const EXAMPLE_PREFIX: &str = "example";


#[test]
/// target: playdate hardware
fn dev_lib_release() -> Result<()> {
	let target = DEVICE_TARGET;
	let args = ["--device", "--lib", "--release"].into_iter().map(OsStr::new);
	// let args = ["--device", "--no-sdk", "--no-gcc", "--release"].into_iter().map(OsStr::new);

	for path in simple_crates()? {
		let (_, target_dir) = run_build(&path, args.clone())?;
		let export_dir = export_dir(&target_dir, target, "release");

		// check expectations:
		let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
		let cargo_target_fullname = format!("{cargo_package_name}-{LIB_NAME}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.elf"));
		test_artifact_dev(&artifact, false)
	}
	Ok(())
}

#[test]
/// target: playdate hardware
fn dev_lib_debug() -> Result<()> {
	let target = DEVICE_TARGET;
	let args = ["--device", "--lib"].into_iter().map(OsStr::new);

	for path in simple_crates()? {
		let (_, target_dir) = run_build(&path, args.clone())?;
		let export_dir = export_dir(&target_dir, target, "debug");

		// check expectations:
		let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
		let cargo_target_fullname = format!("{cargo_package_name}-{LIB_NAME}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.elf"));
		test_artifact_dev(&artifact, false)
	}
	Ok(())
}


#[test]
/// target: default host
fn sim_host_release() -> Result<()> {
	let args = ["--release"].into_iter().map(OsStr::new);

	for path in simple_crates()? {
		let (_, target_dir) = run_build(&path, args.clone())?;
		let export_dir = export_dir_host(&target_dir, "release");

		// check expectations:
		let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
		let cargo_target_fullname = format!("{cargo_package_name}-{LIB_NAME}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.dylib"));
		test_artifact_sim(&artifact, false)
	}

	Ok(())
}

#[test]
/// target: default host
fn sim_host_debug() -> Result<()> {
	for path in simple_crates()? {
		let (_, target_dir) = run_build(&path, None::<&OsStr>)?;
		let export_dir = export_dir_host(&target_dir, "debug");

		// check expectations:
		let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
		let cargo_target_fullname = format!("{cargo_package_name}-{LIB_NAME}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.dylib"));
		test_artifact_sim(&artifact, false)
	}

	Ok(())
}

#[test]
/// target: default host
fn sim_host_release_exp() -> Result<()> {
	let args = ["--release", "--simulator"].into_iter().map(OsStr::new);

	for path in simple_crates()? {
		let (_, target_dir) = run_build(&path, args.clone())?;
		let export_dir = export_dir_host(&target_dir, "release");

		// check expectations:
		let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
		let cargo_target_fullname = format!("{cargo_package_name}-{LIB_NAME}");
		let artifact = export_dir.join("playdate")
		                         .join(cargo_target_fullname)
		                         .join(PathBuf::from("build/pdex.dylib"));
		test_artifact_sim(&artifact, false)
	}

	Ok(())
}

#[test]
/// target: both device & default host
fn dev_sim_release_exp() -> Result<()> {
	let dev_target = DEVICE_TARGET;
	let host_target = target_triple();
	let dev_target_arg = format!("--target={}", dev_target);
	let host_target_arg = format!("--target={}", host_target);

	let args = ["--release", &dev_target_arg, &host_target_arg].into_iter()
	                                                           .map(OsStr::new);
	let expectations = [
	                    ("pdex.elf", dev_target, true),
	                    ("pdex.dylib", &host_target, false),
	];

	for path in simple_crates()? {
		let package_name = path.file_name().expect("package_name").to_str().unwrap();
		let (_, target_dir) = run_build(&path, args.clone())?;

		// check expectations:
		for (filename, target, dev) in &expectations {
			let export_dir = export_dir(&target_dir, target, "release");
			let cargo_target_fullname = format!("{package_name}-{LIB_NAME}");
			let artifact = export_dir.join("playdate")
			                         .join(cargo_target_fullname)
			                         .join("build")
			                         .join(filename);
			test_artifact(&artifact, false, *dev);
		}
	}

	Ok(())
}


mod examples {
	use super::*;
	use std::ffi::OsStr;


	#[test]
	/// target: playdate hardware
	fn dev_release_lib() -> Result<()> {
		let target = DEVICE_TARGET;
		let args = ["--device", "--release", "--example=example-lib"].into_iter()
		                                                             .map(OsStr::new);

		for path in simple_crates()? {
			let (_, target_dir) = run_build(&path, args.clone())?;
			let export_dir = export_dir(&target_dir, target, "release");

			// check expectations:
			let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
			let cargo_target_fullname = format!("{cargo_package_name}-{EXAMPLE_PREFIX}-lib");
			let artifact = export_dir.join("playdate")
			                         .join(cargo_target_fullname)
			                         .join(PathBuf::from("build/pdex.elf"));
			test_artifact_dev(&artifact, false)
		}
		Ok(())
	}


	#[test]
	/// target: playdate hardware
	fn dev_release_bin() -> Result<()> {
		let target = DEVICE_TARGET;
		let args = ["--device", "--release", "--example=example-bin"].into_iter()
		                                                             .map(OsStr::new);

		for path in simple_crates()? {
			let (_, target_dir) = run_build(&path, args.clone())?;
			let export_dir = export_dir(&target_dir, target, "release");

			// check expectations:
			let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
			let cargo_target_fullname = format!("{cargo_package_name}-{EXAMPLE_PREFIX}-bin");
			let artifact = export_dir.join("playdate")
			                         .join(cargo_target_fullname)
			                         .join(PathBuf::from("build/pdex.elf"));
			test_artifact_dev(&artifact, false)
		}
		Ok(())
	}

	#[test]
	/// target: playdate hardware
	fn dev_debug_bin() -> Result<()> {
		let target = DEVICE_TARGET;
		let args = ["--device", "--example=example-bin"].into_iter().map(OsStr::new);

		for path in simple_crates()? {
			let (_, target_dir) = run_build(&path, args.clone())?;
			let export_dir = export_dir(&target_dir, target, "debug");

			// check expectations:
			let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
			let cargo_target_fullname = format!("{cargo_package_name}-{EXAMPLE_PREFIX}-bin");
			let artifact = export_dir.join("playdate")
			                         .join(cargo_target_fullname)
			                         .join(PathBuf::from("build/pdex.elf"));
			test_artifact_dev(&artifact, false)
		}
		Ok(())
	}

	#[test]
	/// target: playdate hardware+simulator(host)
	fn sim_dev_release_examples() -> Result<()> {
		let dev_target = DEVICE_TARGET;
		let host_target = target_triple();
		let host_target_arg = format!("--target={}", host_target);

		let args = [
		            "--device",
		            &host_target_arg,
		            "--release",
		            "--examples",
		            "--keep-going", // needed because "example-bin" can't be built for HOME target
		].into_iter()
		           .map(OsStr::new);

		let expectations = [
		                    ("pdex.elf", dev_target, true),
		                    ("pdex.dylib", &host_target, false),
		];

		for path in simple_crates()? {
			let (_, target_dir) = run_build(&path, args.clone())?;

			// check expectations:
			for (filename, target, dev) in &expectations {
				let export_dir = export_dir(&target_dir, target, "release");
				let cargo_package_name = path.file_name().unwrap().to_str().unwrap();
				let cargo_target_fullname = format!("{cargo_package_name}-{EXAMPLE_PREFIX}-lib");
				let artifact = export_dir.join("playdate")
				                         .join(cargo_target_fullname)
				                         .join("build")
				                         .join(filename);
				test_artifact(&artifact, false, *dev);


				if *dev {
					let cargo_target_fullname = format!("{cargo_package_name}-{EXAMPLE_PREFIX}-bin");
					let artifact = export_dir.join("playdate")
					                         .join(cargo_target_fullname)
					                         .join("build")
					                         .join(filename);
					test_artifact(&artifact, false, *dev);
				}
			}
		}
		Ok(())
	}
}
