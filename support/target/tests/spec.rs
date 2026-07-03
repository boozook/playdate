#![feature(exit_status_error)]

use std::ffi::OsStr;
use std::path::Path;

use anyhow::Context;

mod common;
use common::*;


#[test]
fn vs_schema() -> anyhow::Result<()> {
	let schema = spec_schema()?;
	let specs =
		std::fs::read_dir(spec_dir())?.filter_map(|f| f.ok())
		                              .filter(|f| {
			                              Path::new(&f.file_name()).extension() == Some(OsStr::new("toml"))
		                              });

	for spec in specs {
		tools::taplo_check_schema(&schema).arg(spec.path())
		                                  .output()?
		                                  .exit_ok()
		                                  .with_context(|| format!("invalid spec {:?}", spec.path()))?;
	}

	Ok(())
}


#[test]
#[cfg(any(feature = "spec", all(feature = "serde_json", feature = "toml")))]
fn with_rustc() -> anyhow::Result<()> {
	let tmp = target_dir_tmp().join(target::spec::SPEC_FILENAME_JSON);
	std::fs::write(&tmp, target::spec::SPEC_JSON)?;

	let args = ["-Zunstable-options", "--print=cfg", "--target"];
	let out = tools::rustc().args(args).arg(tmp).output()?;

	if let Err(err) = out.status.exit_ok() {
		let stderr = String::from_utf8(out.stderr).unwrap_or_else(|_| err.to_string());
		panic!("Spec seems to invalid:\n\t{err}\n\t{stderr}");
	}

	Ok(())
}
