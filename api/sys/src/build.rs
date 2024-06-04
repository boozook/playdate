use std::env;
use std::borrow::Cow;
use std::process::exit;
use std::path::{PathBuf, Path};

use bindgen_cfg::*;


/// Existing pre-built bindings used for "no-sdk" environment like docs.rs.
const SDK_VER_EXISTING: &str = "2.5.0";
const BINDINGS_PATH_ENV: &str = "PD_BINDINGS_PATH"; // used in source - include-path.
const BINDINGS_NAME_ENV: &str = "PD_BINDINGS_FILENAME";
const BINDINGS_VER_ENV: &str = "PD_SDK_VERSION"; // used in source - doc for ffi mod.
/// Magic variable to allow save generated bindings to $crate-root/gen/.
const BINDINGS_BUILD_PREBUILT: &str = "PD_BUILD_PREBUILT";
/// Only for dev purposes - to imitate docs.rs env without sdk.
const NO_SDK_ANYWAY: &str = "IGNORE_EXISTING_PLAYDATE_SDK";
/// Cache-ctrl for OUT_DIR.
/// If set and bindings with same filename are exists in OUT_DIR, it will not be generated again.
const USE_BUILT_BINDINGS: &str = "PD_BUILD_BINDINGS_ONCE";


// Using pre-build by default.
// Switching to generate new if:
// - cache-miss (pre-built not found)
// - or re-build requested
//   - also switch output path.


fn main() {
	println!("cargo::rerun-if-env-changed={SDK_PATH_ENV_VAR}");

	println!("cargo::rustc-check-cfg=cfg(playdate)");
	if matches!(Target::from_env_target(), Ok(Target::Playdate)) {
		println!("cargo::rustc-cfg=playdate")
	}

	let mut cfg = Cfg::default();
	cfg.derive.default = feature_derive_default();
	cfg.derive.eq = feature_derive_eq();
	cfg.derive.copy = feature_derive_copy();
	cfg.derive.debug = feature_derive_debug();
	cfg.derive.hash = feature_derive_hash();
	cfg.derive.ord = feature_derive_ord();
	cfg.derive.partialeq = feature_derive_partialeq();
	cfg.derive.partialord = feature_derive_partialord();
	cfg.derive.constparamty = feature_derive_constparamty();
	// Because firstly we'll try to use pre-installed bindgen, enable all features:
	cfg.features.documentation = true;

	let pkg_name = env!("CARGO_PKG_NAME");

	// Docs.rs-like environment:
	if is_env_without_sdk() {
		println!("docs.rs detected");
		return use_existing_prebuilt(&cfg);
	}


	let pdbindgen_found = Runner::find_tool(&cfg);
	if cfg!(feature = "bindgen") && pdbindgen_found.is_some() {
		println!(
		         "cargo::warning=Playdate bindgen found but also built as dependency of the {} by enabled feature 'bindgen'. You might want to disable that feature to significantly decrease build time.",
		         pkg_name
		);
		// better wording: significantly speed up the compilation process?
	}

	if let Some((path, ver)) = pdbindgen_found.as_ref() {
		println!("found {BIN_NAME}: {ver} at {}", path.display());
	} else {
		#[cfg(feature = "bindgen")]
		{
			println!("{BIN_NAME} not found, continue with builtin bindgen.");
			return with_builtin_bindgen(cfg);
		}
		#[cfg(not(feature = "bindgen"))]
		{
			let rec = format!("Install it or enable feature 'bindgen' for {pkg_name}.");
			println!("cargo::warning=Playdate bindgen executable not found. {rec}");
			panic!("Unable to find Playdate bindgen executable and feature 'bindgen' disabled, so can't generate bindings.");
		}
	}


	let sdk_version = Runner::find_sdk_version(&cfg);

	if sdk_version.is_none() {
		// That is fail.
		// But probably it can do this's crate dep.
		// So we have to continue:
		// Jump to bindgen if feature enabled.
		// Otherwise panic.
		#[cfg(feature = "bindgen")]
		return with_builtin_bindgen(cfg);
		#[cfg(not(feature = "bindgen"))]
		panic!("Unable to find Playdate SDK and read its version.");
	}
	let sdk_version = sdk_version.expect("SDK version");
	println!("cargo::rustc-env={BINDINGS_VER_ENV}={}", sdk_version);


	// get filename
	let filename = Filename::new(sdk_version, &cfg.derive).expect("output filename");
	println!("cargo::rustc-env={BINDINGS_NAME_ENV}={}", filename.to_string());


	// determine output path (prebuilt or OUT_DIR)
	let out_path = out_path_or_finish_with_prebuilt(&filename);

	// set output path to cfg
	cfg.output = Some(out_path);

	// execute bindgen
	let result = Runner::gen_cmd(&cfg).and_then(|mut cmd| cmd.status().map_err(|err| eprintln!("{err}")).ok());

	if result.is_some() {
		// for dev purposes:
		// open_bindings(cfg.output.as_ref().unwrap());
	} else {
		#[cfg(feature = "bindgen")]
		{
			println!("cargo::warning=Playdate bindgen exited with error. Trying to build without it.");
			return with_builtin_bindgen(cfg);
		}

		#[cfg(not(feature = "bindgen"))]
		panic!("Playdate bindgen exited with error and feature 'bindgen' disabled, so can't generate bindings.");
	}
}


#[cfg(feature = "bindgen")]
fn with_builtin_bindgen(mut cfg: Cfg) {
	// override features by this crate features
	cfg.features.documentation = cfg!(feature = "bindings-documentation");

	// prepare generator:
	let generator = bindgen::Generator::new(cfg).expect("Couldn't create bindings generator.");

	println!(
	         "cargo::rustc-env={BINDINGS_NAME_ENV}={}",
	         generator.filename.to_string()
	);
	println!("cargo::rustc-env={BINDINGS_VER_ENV}={}", generator.filename.sdk);

	// determine output path, also check cache/prebuilt:
	let out_path = out_path_or_finish_with_prebuilt(&generator.filename);

	// generate bindings:
	let bindings = generator.generate().expect("Couldn't generate bindings.");
	bindings.write_to_file(&out_path)
	        .expect("Couldn't write bindings.");

	// for dev purposes:
	// open_bindings(&out_path);
}


/// Needed for dev purposes.
/// Opens bindings in a `$EDITOR` or `code` if first doesn't set.
/// This is useful for reading, validating & debugging codegen results.
#[allow(dead_code)]
fn open_bindings(path: &Path) {
	let editor = env::var("EDITOR").map(Cow::from)
	                               .unwrap_or_else(|_| "code".into());
	let mut editor = editor.split(" ");
	std::process::Command::new(editor.next().unwrap()).args(editor)
	                                                  .arg(path)
	                                                  .envs(env::vars())
	                                                  .current_dir(env::current_dir().expect("PWD"))
	                                                  .spawn()
	                                                  .ok();
}


fn use_existing_prebuilt(cfg: &Cfg) {
	let version = SDK_VER_EXISTING;
	println!("using pre-built {version}");
	let filename = Filename::new(version, &cfg.derive).expect("filename");

	println!("cargo::rustc-env={BINDINGS_VER_ENV}={version}");
	println!("cargo::rustc-env={BINDINGS_NAME_ENV}={}", filename.to_string());

	let out_path = out_file_prebuilt(&filename);
	println!("cargo::rerun-if-changed={}", out_path.display());
	println!("cargo::rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());
	assert!(out_path.exists(), "pre-built bindings not found.");
}


fn out_dir_prebuilt() -> PathBuf {
	env::var("CARGO_MANIFEST_DIR").map(PathBuf::from)
	                              .map(|p| p.join("gen"))
	                              .expect("CARGO_MANIFEST_DIR")
}

fn out_file_prebuilt(filename: &Filename) -> PathBuf { out_dir_prebuilt().join(&filename.to_string()) }

fn out_file_bounded(filename: &Filename) -> PathBuf {
	env::var("OUT_DIR").map(PathBuf::from)
	                   .map(|p| p.join(&filename.to_string()))
	                   .expect("OUT_DIR")
}


/// Determine output path, also check cache/prebuilt
fn out_path_or_finish_with_prebuilt(filename: &Filename) -> PathBuf {
	let out_path = if is_rebuild_prebuilt_requested() {
		println!("rebuild pre-built bindings requested");
		let out_dir = out_dir_prebuilt();
		let out_path = out_dir.join(filename.to_string());
		println!("cargo::rerun-if-changed={}", out_path.display());
		println!("cargo::warning=Rebuilding `pre-built` bindings");
		if !out_dir.exists() {
			std::fs::create_dir_all(&out_dir).unwrap();
			println!(
			         "cargo::warning=OUT_DIR for `pre-built` bindings created: {}",
			         out_dir.display()
			);
		}
		out_path
	} else {
		// check if pre-built exists
		let out_path = out_file_prebuilt(filename);

		// cache-hit:
		if out_path.exists() {
			println!("cargo::rerun-if-changed={}", out_path.display());
			println!("cargo::rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());
			println!("bindings: cache-hit in pre-built directory");
			exit(0);
		}

		// cache-miss:
		let out_path = out_file_bounded(filename);

		// cache-hit:
		let out_dir_reuse_allowed = env::var_os(USE_BUILT_BINDINGS).filter(|s| s == "1" || s == "true")
		                                                           .is_some();
		if out_path.exists() && out_dir_reuse_allowed {
			println!("cargo::rerun-if-changed={}", out_path.display());
			println!("cargo::rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());
			println!("bindings: cache-hit in build directory");
			exit(0);
		}

		println!("bindings: cache-miss, continuing");
		out_path
	};
	println!("cargo::rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());


	out_path
}


/// Are we in environment like docs.rs (without SDK)
fn is_env_without_sdk() -> bool {
	#![allow(unexpected_cfgs)]
	cfg!(docsrs) ||
	env::var_os("DOCS_RS").is_some() ||
	env::var_os(NO_SDK_ANYWAY).filter(|s| s == "1" || s == "true")
	                          .is_some()
}

fn is_rebuild_prebuilt_requested() -> bool {
	println!("cargo::rerun-if-env-changed={BINDINGS_BUILD_PREBUILT}");
	env::var_os(BINDINGS_BUILD_PREBUILT).is_some()
}


const fn feature_derive_default() -> bool { cfg!(feature = "bindings-derive-default") }
const fn feature_derive_eq() -> bool { cfg!(feature = "bindings-derive-eq") }
const fn feature_derive_copy() -> bool { cfg!(feature = "bindings-derive-copy") }
const fn feature_derive_debug() -> bool { cfg!(feature = "bindings-derive-debug") }
const fn feature_derive_hash() -> bool { cfg!(feature = "bindings-derive-hash") }
const fn feature_derive_ord() -> bool { cfg!(feature = "bindings-derive-ord") }
const fn feature_derive_partialeq() -> bool { cfg!(feature = "bindings-derive-partialeq") }
const fn feature_derive_partialord() -> bool { cfg!(feature = "bindings-derive-partialord") }
const fn feature_derive_constparamty() -> bool { cfg!(feature = "bindings-derive-constparamty") }
// const fn feature_derive_cache() -> bool { cfg!(feature = "bindings-derive-cache") }
