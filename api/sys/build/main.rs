#![feature(str_from_raw_parts)]
use std::borrow::Cow;
use std::path::{PathBuf, Path};
use bindgen_cfg::*;


/// Existing pre-built bindings used for "no-sdk" environment like docs.rs.
const BINDINGS_PATH_ENV: &str = "PD_BINDINGS_PATH"; // used in source - include-path.
const BINDINGS_VER_ENV: &str = "PD_SDK_VERSION"; // used in source - doc for ffi mod.
const BINDINGS_NAME_ENV: &str = "PD_BINDINGS_FILENAME"; // can be used in source.

const BINDINGS_PATCH_ENV: &str = "PD_BINDINGS_PATCH";
const BINDINGS_RENAME_ENV: &str = "PD_BINDINGS_RENAME";

/// Magic variable to allow save generated bindings to $crate-root/gen/.
const BINDINGS_BUILD_BUNDLED: &str = "PD_BUILD_PREBUILT";

/// Only for dev purposes - to imitate docs.rs env without sdk.
const NO_SDK: &str = "IGNORE_EXISTING_PLAYDATE_SDK";
const DOCS_RS: &str = "DOCS_RS";

/// Cache-ctrl for OUT_DIR.
/// If set and bindings with same filename are exists in OUT_DIR, it will not be generated again.
const IGNORE_BINDINGS_CACHE: &str = "PD_IGNORE_BINDINGS_CACHE";


mod cfg;
mod lint;
mod builtin;


fn output(filename: &Filename, path: Option<&Path>) -> ! {
	let sdk = filename.sdk.as_str();
	println!("cargo::rustc-env={BINDINGS_VER_ENV}={sdk}");
	println!("cargo::rustc-env={BINDINGS_NAME_ENV}={filename}");

	let path = path.map(Cow::Borrowed)
	               .unwrap_or_else(|| builtin::path(filename).into());
	println!("cargo::rustc-env={BINDINGS_PATH_ENV}={}", path.display());

	debug_assert!(path.exists(), "bindings not found");

	if !path.exists() {
		cargo::err(format_args!("output bindings not found at {path:?}"));
	}

	// open_in_editor(&path);
	std::process::exit(0);
}


fn main() {
	cargo_env_watch();

	let target = Target::from_env_target().inspect_err(|err| cargo::warn(err)).ok();

	// target -> cfg:
	if matches!(target, Some(Target::Playdate)) {
		println!("cargo::rustc-cfg=playdate")
	}

	let mut cfg = cfg::default();
	// TODO: configure layout_tests

	// set patch override:
	{
		let patch_env_use = env::var_os(BINDINGS_PATCH_ENV).map(|p| cfg.patch = Some(p.into()));
		let rename_env_use = env::var_os(BINDINGS_RENAME_ENV).map(|p| cfg.rename = Some(p.into()));

		if let Some(path) = cfg.patch.as_deref() &&
		   !path.exists()
		{
			let src = patch_env_use.map(|_| format!("from env {BINDINGS_PATCH_ENV}"))
			                       .unwrap_or_default();
			cargo::err(format_args!("missed patch at {path:?} {src}"));
		}
		if let Some(path) = cfg.rename.as_deref() &&
		   !path.exists()
		{
			let src = rename_env_use.map(|_| format!("from env {BINDINGS_RENAME_ENV}"))
			                        .unwrap_or_default();
			cargo::err(format_args!("missed rename at {path:?} {src}"));
		}
	}


	// Docs.rs-like environment,
	// With mock the bindings are replaced so it doesn’t matter what to use.
	if is_env_without_sdk() || cfg!(any(mockrt, mockrt = "alloc", mockrt = "std")) {
		println!("docs.rs or mock detected");
		return use_existing_bundled(&cfg);
	}

	let sdk_version = Runner::find_sdk_version(&cfg);
	if sdk_version.is_none() {
		cargo::warn("Unable to find Playdate SDK and read its version.");
	}
	let sdk_version = sdk_version.inspect(|ver| println!("Found SDK version: {ver}"))
	                             .unwrap_or_else(|| builtin::highest_version(false));
	println!("Finally using SDK version: {sdk_version}");


	let filename = Filename::new(&sdk_version, &cfg.derive).unwrap();
	println!("Looking for builtin bindings: {filename}");

	// builtin, exactly same as requested:
	let bundled = builtin::path(&filename);


	if bundled.exists() && !is_bundled_rebuild_requested() {
		lint::check_bindgen_unnecessary_inner();

		println!("Found exact match");
		output(&filename, Some(&bundled));
	} else {
		println!("Exact match not found, fallback...");

		if cfg!(feature = "bindgen") {
			lint::check_bindgen_inner_and_external(&cfg.bin);
			#[cfg(feature = "bindgen")]
			return with_builtin_bindgen(cfg);
		} else if let Some((pdbindgen, ver)) = Runner::find_tool(&cfg.bin) {
			println!("Using external bindgen {ver} ({pdbindgen:?})");
			with_external_bindgen(&mut cfg, &filename);
		} else {
			// well, not feature bindgen & pdbindgen not installed.
			// search for some prebuilt that covers requested
			println!("Looking for some bundled pre-built that covers requested");
			if let Some(applicable) = builtin::nearest_applicable(&filename).unwrap() {
				let a = &filename.mask;
				let b = &applicable.mask;
				println!("Found nearest applicable match: {b} instead of {a}.");
				output(&applicable, None);
			} else {
				println!("Nothing that covers requested.");
				lint::panic_recover_hints_no_builtin(&sdk_version);
			}
		}
	}
}


fn with_external_bindgen(cfg: &mut Cfg, filename: &Filename) {
	// determine output path (bundled or OUT_DIR)
	let out_path = out_path_or_cache(filename);

	// set output path to cfg
	cfg.output = Some(out_path);

	// execute bindgen
	match Runner::gen_cmd(&cfg).expect("pd-bindgen cmd").status() {
		Ok(exit) => {
			println!("Playdate bindgen exited with status {exit}");
			output(filename, cfg.output.as_deref());
		},
		Err(err) => {
			cargo::err(format_args!("Playdate bindgen exited with error and feature 'bindgen' is off, so can't generate bindings. {err}"));
		},
	}
}


#[cfg(feature = "bindgen")]
fn with_builtin_bindgen(cfg: Cfg) {
	// prepare generator:
	let generator = bindgen::Generator::new(cfg).expect("Couldn't create bindings generator.");
	let filename = generator.filename.to_owned();

	// determine output path, also check cache/bundled:
	let out_path = out_path_or_cache(&generator.filename);

	// generate bindings:
	let bindings = generator.generate().expect("Couldn't generate bindings.");
	bindings.write_to_file(&out_path)
	        .expect("Couldn't write bindings.");

	output(&filename, Some(&out_path))
}


/// Use builtin unconditionally,
/// don't search SDK,
/// forget about linking.
fn use_existing_bundled(cfg: &Cfg) {
	let version = &builtin::highest_version(false);
	println!("Using pre-built {version}");
	let filename = Filename::new(version, &cfg.derive).expect("filename");
	output(&filename, None);
}


fn out_file_bounded(filename: &Filename) -> PathBuf {
	env::var_os("OUT_DIR").map(PathBuf::from)
	                      .map(|p| p.join(&filename.to_string()))
	                      .expect("OUT_DIR")
}


/// Determine output path, also check cache.
/// Returns bundled path if rebuild is requested.
fn out_path_or_cache(filename: &Filename) -> PathBuf {
	if is_bundled_rebuild_requested() {
		println!("rebuild pre-built bindings requested");
		let out_dir = builtin::root();
		let out_path = out_dir.join(filename.to_string());
		cargo::watch_path(&out_path);
		cargo::warn("Rebuilding `pre-built` bindings");
		if !out_dir.exists() {
			std::fs::create_dir_all(&out_dir).unwrap();
			println!(
			         "cargo::warning=OUT_DIR for `pre-built` bindings created: {}",
			         out_dir.display()
			);
		}
		out_path
	} else {
		// cache-miss:
		let out_path = out_file_bounded(filename);

		let out_dir_reuse_allowed = {
			let var = env::is_true(IGNORE_BINDINGS_CACHE);
			let dis = var.then_some("dis").unwrap_or_default();
			println!("Reusing of previous build is {dis}allowed");
			!var
		};


		// cache-hit:
		if out_dir_reuse_allowed && out_path.exists() {
			println!("Cache-hit in build directory");
			cargo::watch_path(&out_path);
			output(filename, Some(&out_path));
		} else if out_dir_reuse_allowed {
			println!("Cache-miss");
		}

		out_path
	}
}


/// Are we in environment like docs.rs (without SDK)
fn is_env_without_sdk() -> bool {
	#![allow(unexpected_cfgs)]
	cfg!(docsrs) || env::is_set(DOCS_RS) || env::is_true(NO_SDK)
}

fn is_bundled_rebuild_requested() -> bool {
	// TODO: Probably replace with cfg
	cargo::watch_env(BINDINGS_BUILD_BUNDLED);
	env::is_set(BINDINGS_BUILD_BUNDLED)
}


fn cargo_env_watch() {
	let env = [
	           BINDINGS_BUILD_BUNDLED,
	           NO_SDK,
	           DOCS_RS,
	           IGNORE_BINDINGS_CACHE,
	           Cfg::ENV_BIN_PATH,
	           Cfg::ENV_SDK_PATH,
	           Cfg::ENV_ARM_GCC_PATH,
	];

	for var in env {
		cargo::watch_env(var);
	}
}


mod env {
	pub use std::env::*;
	use std::ffi::OsStr;


	/// True if var is set.
	pub fn is_set(var: impl AsRef<OsStr>) -> bool { var_os(var).is_some() }

	/// True if var is set and value means `true`.
	pub fn is_true(var: impl AsRef<OsStr>) -> bool { var_os(var).filter(|s| as_true(s)).is_some() }

	#[allow(dead_code)]
	/// True if var is set and value means `false`.
	pub fn is_false(var: impl AsRef<OsStr>) -> bool { var_os(var).filter(|s| as_false(s)).is_some() }

	fn as_true(s: impl AsRef<OsStr>) -> bool {
		let s = s.as_ref();
		s == "1" || s == "true"
	}
	fn as_false(s: impl AsRef<OsStr>) -> bool {
		let s = s.as_ref();
		s == "0" || s == "false"
	}
}


mod cargo {
	use std::fmt::Display;
	use std::path::Path;

	pub fn warn(s: impl Display) { println!("cargo::warning={s}") }
	pub fn err(s: impl Display) { println!("cargo::error={s}") }

	pub fn watch_path(p: impl AsRef<Path>) { println!("cargo::rerun-if-changed={}", p.as_ref().display()) }
	pub fn watch_env(var: impl Display) { println!("cargo::rerun-if-env-changed={var}") }
}


/// Open `path` in `$EDITOR`.
/// Used only for bindings generation debug purposes
#[allow(unused)]
fn open_in_editor(path: &Path) -> bool {
	std::env::var("EDITOR").map(|var| {
		                       let mut cmd = var.split_once(" ")
		                                        .map(|(cmd, arg)| {
			                                        let mut cmd = std::process::Command::new(cmd);
			                                        if !arg.contains("wait") {
				                                        cmd.arg(arg);
			                                        }
			                                        cmd
		                                        })
		                                        .unwrap_or_else(|| std::process::Command::new(var));
		                       cmd.arg(path.as_os_str());
		                       cmd
	                       })
	                       .inspect_err(|err| {
		                       cargo::err(format_args!("getting env $EDITOR: {err:?}"));
	                       })
	                       .ok()
	                       .and_then(|mut cmd| {
		                       cmd.output()
		                          .inspect_err(|err| {
			                          cargo::err(format_args!("open with $EDITOR: {err:?}"));
		                          })
		                          .ok()
		                          .filter(|s| s.status.success())
	                       })
	                       .is_some()
}
