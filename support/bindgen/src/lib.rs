#![cfg_attr(feature = "documentation", feature(get_mut_unchecked))]
pub extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use bindgen::callbacks::DeriveInfo;
use bindgen::{EnumVariation, RustTarget, Builder, MacroTypeVariation};
use utils::consts::*;
use utils::toolchain::gcc::{ArmToolchain, Gcc};
use utils::toolchain::sdk::Sdk;
pub use bindgen_cfg as cfg;
use rustify::rename::SharedRenamed;


pub mod error;
pub mod gen;
pub mod rustify {
	pub mod rename;
}


type Result<T, E = error::Error> = std::result::Result<T, E>;


pub const SDK_VER_SUPPORTED: &str = ">=2.1.0, <3.0.0";


/// Generated Rust bindings.
pub enum Bindings {
	Bindgen(Box<bindgen::Bindings>),
	#[cfg(feature = "extra-codegen")]
	Engaged(gen::Bindings),
}


impl Bindings {
	#[inline(always)]
	/// Write these bindings as source text to a file.
	pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
		match self {
			Bindings::Bindgen(this) => this.write_to_file(path),
			#[cfg(feature = "extra-codegen")]
			Bindings::Engaged(this) => this.write_to_file(path),
		}
	}

	#[inline(always)]
	/// Write these bindings as source text to the given `Write`able.
	pub fn write<'a>(&self, writer: Box<dyn std::io::Write + 'a>) -> std::io::Result<()> {
		match self {
			Bindings::Bindgen(this) => this.write(writer),
			#[cfg(feature = "extra-codegen")]
			Bindings::Engaged(this) => this.write(writer),
		}
	}
}

impl std::fmt::Display for Bindings {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Bindings::Bindgen(this) => std::fmt::Display::fmt(this, f),
			#[cfg(feature = "extra-codegen")]
			Bindings::Engaged(this) => std::fmt::Display::fmt(this, f),
		}
	}
}


pub struct Generator {
	/// Playdate SDK.
	pub sdk: Sdk,
	/// Version of the Playdate SDK.
	pub version: semver::Version,

	/// ARM GCC.
	pub gcc: ArmToolchain,

	/// Suggested filename for export bindings.
	pub filename: cfg::Filename,
	/// Configured [`bindgen::Builder`].
	pub builder: Builder,

	/// Renamed symbols, if `rustify` is enabled.
	renamed: SharedRenamed,

	// configuration
	pub derives: cfg::Derive,
	pub features: cfg::Features,
}


impl Generator {
	pub fn new(cfg: cfg::Cfg) -> Result<Self> { create_generator(cfg) }

	pub fn generate(mut self) -> Result<Bindings> {
		// disable formatting if we gonna extra work:
		if cfg!(feature = "extra-codegen") {
			self.builder = self.builder.formatter(bindgen::Formatter::None);
		}

		// generate:
		let bindings = self.builder.generate()?;

		#[cfg(not(feature = "extra-codegen"))]
		return Ok(Bindings::Bindgen(bindings.into()));


		#[cfg(feature = "extra-codegen")]
		gen::engage(
		            &bindings,
		            self.renamed,
		            &self.features,
		            &self.filename.target,
		            &self.sdk,
		            None,
		).map(Bindings::Engaged)
	}
}


fn create_generator(cfg: cfg::Cfg) -> Result<Generator, error::Error> {
	println!("cargo::rerun-if-env-changed=TARGET");
	let cargo_target_triple = env::var("TARGET").expect("TARGET cargo env var");

	println!("cargo::rerun-if-env-changed=PROFILE");
	let cargo_profile = env::var("PROFILE").expect("PROFILE cargo env var");
	let is_debug = cargo_profile == "debug" || env_cargo_feature("DEBUG");

	let sdk = cfg.sdk
	             .map(|p| Sdk::try_new_exact(p).or_else(|_| Sdk::try_new()))
	             .unwrap_or_else(Sdk::try_new)?;
	let version_path = sdk.version_file();
	let version_raw = sdk.read_version()?;
	let version = check_sdk_version(&version_raw)?;
	println!("cargo::rerun-if-changed={}", version_path.display());
	let sdk_c_api = sdk.c_api();

	let main_header = sdk_c_api.join("pd_api.h");
	println!("cargo::rerun-if-changed={}", main_header.display());
	println!("cargo::rerun-if-env-changed={SDK_ENV_VAR}");
	println!("cargo::metadata=include={}", sdk_c_api.display());


	// builder:
	let gcc = cfg.gcc
	             .map(|p| {
		             Gcc::try_from_path(p).and_then(ArmToolchain::try_new_with)
		                                  .or_else(|_| ArmToolchain::try_new())
	             })
	             .unwrap_or_else(ArmToolchain::try_new)?;
	let (mut builder, renamed) = create_builder(
	                                            &cargo_target_triple,
	                                            &sdk_c_api,
	                                            &main_header,
	                                            &cfg.derive,
	                                            &cfg.features,
	);
	builder = apply_profile(builder, is_debug);
	builder = apply_target(builder, &cargo_target_triple, &gcc);


	let filename = cfg::Filename::new(version.to_owned(), cfg.derive)?;

	Ok(Generator { sdk,
	               gcc,
	               version,
	               filename,
	               builder,
	               renamed,
	               derives: cfg.derive,
	               features: cfg.features })
}


fn check_sdk_version(version: &str) -> Result<semver::Version, error::Error> {
	is_version_matches(version)
	.map(|(ver, res, req)| {
		if res {
			const PKG: &str = env!("CARGO_PKG_NAME");
			const VER: &str = env!("CARGO_PKG_VERSION");
			println!("cargo:warning=Playdate SDK v{ver} may not be compatible with {PKG} v{VER} which hasn't been tested with it. Supported '{req}' does not matches current '{ver}'.")
		}
		ver
	})
}

fn is_version_matches(version: &str) -> Result<(semver::Version, bool, semver::VersionReq), error::Error> {
	let requirement =
		semver::VersionReq::parse(SDK_VER_SUPPORTED).expect("Builtin supported version requirement is invalid.");
	let version = semver::Version::parse(version.trim())?;
	let matches = requirement.matches(&version);
	Ok((version, matches, requirement))
}


pub fn env_var(name: &'static str) -> Result<String> {
	env::var(name).map_err(|err| error::Error::Env { err, ctx: name })
}

pub fn env_cargo_feature(feature: &str) -> bool { env::var(format!("CARGO_FEATURE_{feature}")).is_ok() }


fn create_builder(_target: &str,
                  capi: &Path,
                  header: &Path,
                  derive: &cfg::Derive,
                  features: &cfg::Features)
                  -> (Builder, SharedRenamed) {
	let mut builder = bindgen::builder()
	.header(format!("{}", header.display()))
	.rust_target(RustTarget::nightly())

	// allow types:
	.allowlist_recursively(true)
	.allowlist_type("PlaydateAPI")
	.allowlist_type("PDSystemEvent")
	.allowlist_type("LCDSolidColor")
	.allowlist_type("LCDColor")
	.allowlist_type("LCDPattern")
	.allowlist_type("PDEventHandler")

	.allowlist_var("LCD_COLUMNS")
	.allowlist_var("LCD_ROWS")
	.allowlist_var("LCD_ROWSIZE")
	.allowlist_var("LCD_SCREEN_RECT")
	.allowlist_var("SEEK_SET")
	.allowlist_var("SEEK_CUR")
	.allowlist_var("SEEK_END")
	.allowlist_var("AUDIO_FRAMES_PER_CYCLE")
	.allowlist_var("NOTE_C4")

	// experimental:
	.default_macro_constant_type(MacroTypeVariation::Unsigned)
	.allowlist_var("LCDMakePattern")
	.allowlist_type("LCDMakePattern")
	.allowlist_var("LCDOpaquePattern")
	.allowlist_type("LCDOpaquePattern")
	.allowlist_type("LCDFontLanguage")

	.bitfield_enum("FileOptions")
	.bitfield_enum("PDButtons")

	// types:
	.use_core()
	.ctypes_prefix("core::ffi")
	.size_t_is_usize(true)
	.no_convert_floats()
	.translate_enum_integer_types(true)
	.array_pointers_in_arguments(true)
	.explicit_padding(false)

	.default_enum_style(EnumVariation::Rust { non_exhaustive: false })

	.layout_tests(true)
	.enable_function_attribute_detection()
	.detect_include_paths(true)

	.clang_args(&["--include-directory", &capi.display().to_string()])
	.clang_arg("-DTARGET_EXTENSION=1")

	.dynamic_link_require_all(true)

	// derives:
	.derive_default(derive.default)
	.derive_eq(derive.eq)
	.derive_copy(derive.copy)
	.derive_debug(derive.debug)
	.derive_hash(derive.hash)
	.derive_ord(derive.ord)
	.derive_partialeq(derive.partialeq)
	.derive_partialord(derive.partialord)

	.must_use_type("playdate_*")
	.must_use_type(".*")
	.generate_comments(true);


	builder = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
	if !derive.copy {
		builder = builder.parse_callbacks(Box::new(DeriveCopyToPrimitives));
	}
	if derive.constparamty {
		builder = builder.parse_callbacks(Box::new(DeriveConstParamTy));
	}

	let renamed = if features.rustify {
		let hook = rustify::rename::RenameMap::new();
		let renamed = hook.renamed.clone();
		builder = builder.parse_callbacks(Box::new(hook));
		renamed
	} else {
		Default::default()
	};


	// explicitly set "do not derive":
	if !derive.default {
		builder = builder.no_default(".*");
	}
	if !derive.copy {
		builder = builder.no_copy(".*");
	}
	if !derive.debug {
		builder = builder.no_debug(".*");
	}
	if !derive.hash {
		builder = builder.no_hash(".*");
	}
	if !derive.partialeq {
		builder = builder.no_partialeq(".*");
	}

	(builder, renamed)
}


fn apply_profile(mut builder: Builder, debug: bool) -> Builder {
	// extra code-gen for `debug` feature:
	if debug {
		builder = builder.clang_arg("-D_DEBUG=1").derive_debug(true);
	} else {
		// should we set "-D_DEBUG=0"?
		// builder = builder.derive_debug(false).no_debug(".*");
	}
	builder
}


// This is for build with ARM toolchain.
// TODO: impl build with just LLVM.
fn apply_target(mut builder: Builder, target: &str, gcc: &ArmToolchain) -> Builder {
	builder = if DEVICE_TARGET == target {
		let arm_eabi_include = gcc.include();
		// println!("cargo::rustc-link-search={}", arm_eabi.join("lib").display()); // for executable
		println!("cargo::metadata=include={}", arm_eabi_include.display());

		// TODO: prevent build this for other targets:
		// builder = builder.raw_line(format!("#![cfg(target = \"{DEVICE_TARGET}\")]\n\n"));

		builder.clang_arg("-DTARGET_PLAYDATE=1")
		       .blocklist_file("stdlib.h")
		       .clang_args(&["-target", DEVICE_TARGET])
		       .clang_arg("-fshort-enums")
		       .clang_args(&["--include-directory", &arm_eabi_include.display().to_string()])
		       .clang_arg(format!("-I{}", arm_eabi_include.display()))
	} else {
		builder.clang_arg("-DTARGET_SIMULATOR=1")
	};
	builder
}


/// Derives `Copy` to simple structs and enums.
#[derive(Debug)]
struct DeriveCopyToPrimitives;
impl bindgen::callbacks::ParseCallbacks for DeriveCopyToPrimitives {
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
			vec!["Copy".to_string()]
		} else {
			vec![]
		}
	}
}


#[derive(Debug)]
/// Derives `Copy` to simple structs and enums.
struct DeriveConstParamTy;

impl bindgen::callbacks::ParseCallbacks for DeriveConstParamTy {
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


pub fn rustfmt(mut rustfmt_path: Option<PathBuf>,
               source: String,
               config_path: Option<&Path>)
               -> std::io::Result<String> {
	use std::io::Write;
	use std::process::{Command, Stdio};

	rustfmt_path = rustfmt_path.or_else(|| std::env::var("RUSTFMT").map(PathBuf::from).ok());
	#[cfg(feature = "which-rustfmt")]
	{
		rustfmt_path = rustfmt_path.or_else(|| which::which("rustfmt").ok());
	}
	let rustfmt = rustfmt_path.as_deref().unwrap_or(Path::new("rustfmt"));


	let mut cmd = Command::new(rustfmt);

	cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

	if let Some(path) = config_path {
		cmd.arg("--config-path");
		cmd.arg(path);
	}

	let mut child = cmd.spawn()?;
	let mut child_stdin = child.stdin.take().unwrap();
	let mut child_stdout = child.stdout.take().unwrap();

	// Write to stdin in a new thread, so that we can read from stdout on this
	// thread. This keeps the child from blocking on writing to its stdout which
	// might block us from writing to its stdin.
	let stdin_handle = std::thread::spawn(move || {
		let _ = child_stdin.write_all(source.as_bytes());
		source
	});

	let mut output = vec![];
	std::io::copy(&mut child_stdout, &mut output)?;

	let status = child.wait()?;
	let source = stdin_handle.join()
	                         .expect("The thread writing to rustfmt's stdin doesn't do anything that could panic");

	match String::from_utf8(output) {
		Ok(bindings) => {
			match status.code() {
				Some(0) => Ok(bindings),
				Some(2) => Err(std::io::Error::other("Rustfmt parsing errors.".to_string())),
				Some(3) => {
					println!("cargo:warning=Rustfmt could not format some lines.");
					Ok(bindings)
				},
				_ => Err(std::io::Error::other("Internal rustfmt error".to_string())),
			}
		},
		_ => Ok(source),
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn same_env_var() {
		assert_eq!(utils::consts::SDK_ENV_VAR, bindgen_cfg::Cfg::ENV_SDK_PATH);
	}

	#[test]
	fn version_matches() {
		use super::is_version_matches as check;

		let map = |(_, res, _)| res;

		assert!(check("0.0").map(map).is_err());
		assert!(!check("0.0.0").map(map).unwrap());
		assert!(check("2.1.0").map(map).unwrap());
		assert!(check("2.7.0").map(map).unwrap());
		assert!(!check("2.7.0-beta.3").map(map).unwrap());
		assert!(!check("3.1.0").map(map).unwrap());
	}
}
