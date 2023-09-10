extern crate bindgen;

use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::path::{PathBuf, Path};
use bindgen::SDK_PATH_ENV_VAR;
use bindgen::bindgen::callbacks::DeriveInfo;


const SDK_VER_SUPPORTED: &str = "~2.0.1"; // used for version validation and if no sdk to choose pre-gen.
const BINDINGS_PATH_ENV: &str = "PD_BINDINGS_PATH"; // used in source - include-path.
const BINDINGS_NAME_ENV: &str = "PD_BINDINGS_FILENAME";
const BINDINGS_VER_ENV: &str = "PD_SDK_VERSION"; // used in source - doc for ffi mod.
/// Magic variable to allow save generated bindings to $crate-root/gen/.
const BINDINGS_BUILD_PREBUILT: &str = "PD_BUILD_PREBUILT";
/// Only for dev purposes - to imitate docs.rs env without sdk.
const NO_SDK_ANYWAY: &str = "IGNORE_EXISTING_PLAYDATE_SDK";


// Using pre-build by default.
// Switching to generate new if:
// - cache-miss (pre-built not found)
// - or re-build requested
//   - also switch output path.


fn main() {
	println!("cargo:rerun-if-env-changed={SDK_PATH_ENV_VAR}");


	let mut cfg = bindgen::cfg::Config::default();
	cfg.derive.default = feature_derive_default();
	cfg.derive.eq = feature_derive_eq();
	cfg.derive.copy = feature_derive_copy();
	cfg.derive.debug = feature_derive_debug();
	cfg.derive.hash = feature_derive_hash();
	cfg.derive.ord = feature_derive_ord();
	cfg.derive.partialeq = feature_derive_partialeq();
	cfg.derive.partialord = feature_derive_partialord();


	if is_env_without_sdk() {
		println!("docs.rs detected");
		return use_existing_prebuilt(&cfg);
	}


	// prepare generator:
	let mut generator = bindgen::Generator::new(cfg).expect("Couldn't create bindings generator.");

	// add features:
	add_features_to_filename(&mut generator.filename);
	if cfg!(feature = "bindings-derive-constparamty") {
		generator.builder = generator.builder.parse_callbacks(Box::new(DeriveConstParamTy));
	}
	// if cfg!(feature = "bindings-derive-cache") {
	// 	generator.builder = generator.builder.parse_callbacks(Box::new(DeriveCaches));
	// }

	println!(
	         "cargo:rustc-env={BINDINGS_NAME_ENV}={}",
	         generator.filename.to_string()
	);
	println!("cargo:rustc-env={BINDINGS_VER_ENV}={}", generator.filename.sdk);


	// determine output path, also check cache/prebuilt:
	let out_path = if is_rebuild_prebuilt_requested() {
		println!("rebuild pre-built bindings requested");
		let out_dir = out_dir_prebuilt();
		let out_path = out_dir.join(&generator.filename.to_string());
		println!("cargo:rerun-if-changed={}", out_path.display());
		println!("cargo:warning=Rebuilding `pre-built` bindings");
		if !out_dir.exists() {
			std::fs::create_dir_all(&out_dir).unwrap();
			println!(
			         "cargo:warning=OUT_DIR for `pre-built` bindings created: {}",
			         out_dir.display()
			);
		}
		out_path
	} else {
		// check if pre-built exists
		let out_path = out_file_prebuilt(&generator.filename);

		// cache-hit:
		if out_path.exists() {
			println!("cargo:rerun-if-changed={}", out_path.display());
			println!("cargo:rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());
			return;
		}

		// cache-miss:
		out_file_bounded(&generator.filename)
	};
	println!("cargo:rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());


	// TODO: same cache-check for docset? #[cfg(feature = "bindings-documentation")]


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
	let editor = std::env::var("EDITOR").map(Cow::from)
	                                    .unwrap_or_else(|_| "code".into());
	let mut editor = editor.split(" ");
	std::process::Command::new(editor.next().unwrap()).args(editor)
	                                                  .arg(path)
	                                                  .envs(env::vars())
	                                                  .current_dir(env::current_dir().expect("PWD"))
	                                                  .spawn()
	                                                  .ok();
}


fn use_existing_prebuilt(cfg: &bindgen::cfg::Config) {
	let version = get_supported_version().unwrap();
	let version_raw = version.to_string();
	println!("using pre-built {version_raw}");
	let filename = filename(version, &cfg);

	println!("cargo:rustc-env={BINDINGS_VER_ENV}={version_raw}");
	println!("cargo:rustc-env={BINDINGS_NAME_ENV}={}", filename.to_string());

	let out_path = out_file_prebuilt(&filename);
	println!("cargo:rerun-if-changed={}", out_path.display());
	println!("cargo:rustc-env={BINDINGS_PATH_ENV}={}", out_path.display());
	assert!(out_path.exists(), "pre-built bindings not found.");
}


fn filename(ver: semver::Version, cfg: &bindgen::cfg::Config) -> bindgen::Filename {
	let mut filename = bindgen::Filename::new(ver, &cfg.derive).unwrap();
	add_features_to_filename(&mut filename);
	filename
}

fn add_features_to_filename(filename: &mut bindgen::Filename) {
	filename.mask.push(feature_derive_constparamty());
	// filename.mask.push(feature_derive_cache());
}


fn out_dir_prebuilt() -> PathBuf {
	bindgen::env_var("CARGO_MANIFEST_DIR").map(PathBuf::from)
	                                      .map(|p| p.join("gen"))
	                                      .unwrap()
}

fn out_file_prebuilt(filename: &bindgen::Filename) -> PathBuf { out_dir_prebuilt().join(&filename.to_string()) }

fn out_file_bounded(filename: &bindgen::Filename) -> PathBuf {
	bindgen::env_var("OUT_DIR").map(PathBuf::from)
	                           .map(|p| p.join(&filename.to_string()))
	                           .unwrap()
}


/// Are we in environment like docs.rs (without SDK)
fn is_env_without_sdk() -> bool {
	#![allow(unexpected_cfgs)]
	cfg!(docsrs) ||
	std::env::var_os("DOCS_RS").is_some() ||
	env::var_os(NO_SDK_ANYWAY).filter(|s| s == "1" || s == "true")
	                          .is_some()
}

fn is_rebuild_prebuilt_requested() -> bool {
	println!("cargo:rerun-if-env-changed={BINDINGS_BUILD_PREBUILT}");
	std::env::var_os(BINDINGS_BUILD_PREBUILT).is_some()
}


fn get_supported_version() -> Result<semver::Version, Box<dyn Error>> {
	let requirement = semver::VersionReq::parse(SDK_VER_SUPPORTED).unwrap();

	let mut major = None;
	let mut minor = None;
	let mut patch = None;

	requirement.comparators
	           .iter()
	           .inspect(|c| {
		           major = Some(c.major);
		           minor = c.minor;
		           patch = c.patch;
	           })
	           .count();

	let s = match (major, minor, patch) {
		(Some(a), Some(b), Some(c)) => format!("{a}.{b}.{c}"),
		(Some(a), Some(b), None) => format!("{a}.{b}"),
		_ => return Err("Can't determine version by required semver.".into()),
	};

	let version = semver::Version::parse(&s)?;
	Ok(version)
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


#[derive(Debug)]
/// Derives `Copy` to simple structs and enums.
struct DeriveConstParamTy;

impl bindgen::bindgen::callbacks::ParseCallbacks for DeriveConstParamTy {
	fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
		const TYPES: &[&str] = &[
		                         "PDButtons",
		                         "FileOptions",
		                         "LCDBitmapDrawMode",
		                         "LCDBitmapFlip",
		                         "LCDSolidColor",
		                         "LCDLineCapStyle",
		                         "PDStringEncoding",
		                         "LCDPolygonFillRule",
		                         "PDLanguage",
		                         "PDPeripherals",
		                         "l_valtype",
		                         "LuaType",
		                         "json_value_type",
		                         "SpriteCollisionResponseType",
		                         "SoundFormat",
		                         "LFOType",
		                         "SoundWaveform",
		                         "TwoPoleFilterType",
		                         "PDSystemEvent",
		];

		if TYPES.contains(&info.name) {
			vec!["::core::marker::ConstParamTy".to_string()]
		} else {
			vec![]
		}
	}
}
