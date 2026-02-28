#![feature(exit_status_error)]
#![allow(unused_imports)]
mod common;
use common::*;


#[test]
#[cfg(feature = "pretty")]
fn schema_actual_bang() -> anyhow::Result<()> {
	use playdate_target::spec::SPEC_TOML_PRETTY;


	let schema = spec_schema()?;
	assert!(SPEC_TOML_PRETTY.contains(&*schema.to_string_lossy()));

	Ok(())
}


#[test]
#[cfg(all(feature = "serde_json", feature = "toml"))]
fn build_json() {
	use target::spec::{LINKER_SCRIPT, SPEC_TOML};
	use target::spec::build::build_json;


	let json = build_json(SPEC_TOML, Some(LINKER_SCRIPT)).unwrap();
	assert!(json.get("link-script").is_some());

	let json = build_json(SPEC_TOML, Some("foobar")).unwrap();
	let ls = json.get("link-script").and_then(|v| v.as_str());
	assert_eq!(Some("foobar"), ls);
}

#[test]
#[cfg(all(feature = "serde_json", feature = "toml"))]
fn build_json_ls_untouched() {
	use target::spec::SPEC_TOML;
	use target::spec::build::build_json;


	let json = build_json(SPEC_TOML, None::<&str>).unwrap();
	assert!(json.get("link-script").is_some());
}

#[test]
#[cfg(all(feature = "serde_json", feature = "toml_edit"))]
fn build_json_ls_untouched_pretty() {
	use target::spec::SPEC_TOML_PRETTY;
	use target::spec::build::build_json;


	let json = build_json(SPEC_TOML_PRETTY, None::<&str>).unwrap();
	assert!(json.get("link-script").is_some());
}
