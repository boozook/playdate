use std::convert::Infallible;
use std::env::VarError;
use std::io::stderr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::str::FromStr;

/// Executable name of the `playdate-bindgen`.
pub const BIN_NAME: &str = "pdbindgen";
pub const FIND_SDK_VERSION_CMD: &str = "find-sdk-version";
pub const SDK_PATH_ENV_VAR: &str = "PLAYDATE_SDK_PATH";


/// Playdate-bindgen configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "clap", command(author, version, about, name = BIN_NAME, verbatim_doc_comment))]
pub struct Cfg {
	/// Path to the playdate-bindgen (pdbindgen) executable.
	#[cfg_attr(feature = "clap", arg(skip))]
	pub bin: PathBuf,

	/// Path to the Playdate SDK.
	#[cfg_attr(feature = "clap", arg(long, value_name = "SDK", env = SDK_PATH_ENV_VAR))]
	pub sdk: Option<PathBuf>,

	/// Path to gnu-arm-gcc executable, usually 'arm-none-eabi-gcc' or 'gcc-arm-none-eabi'.
	#[cfg_attr(feature = "clap", arg(long, value_name = "GCC", env = "ARM_GCC_PATH"))]
	pub gcc: Option<PathBuf>,

	/// Comma separated list of types to derive. Possible values: debug, default, eq, copy, hash, ord, partialeq, partialord, constparamty.
	#[cfg_attr(feature = "clap", arg(long, value_name = "TY[,TY...]", default_value_t = Derive::default(), verbatim_doc_comment))]
	pub derive: Derive,

	/// Comma separated list of features to use. Possible values: documentation.
	#[cfg_attr(feature = "clap", arg(long, value_name = "FEATURE[,FEATURE...]", default_value_t = Features::default(), verbatim_doc_comment))]
	pub features: Features,

	/// Output file path.
	#[cfg_attr(feature = "clap", arg(long, value_name = "FILE"))]
	pub output: Option<PathBuf>,
}


pub struct Runner;

impl Runner {
	/// Returns path and version of the `pdbindgen` executable if found.
	pub fn find_tool(cfg: &Cfg) -> Option<(&Path, String)> {
		Command::new(&cfg.bin).arg("-V")
		                      .stdout(Stdio::piped())
		                      .stderr(Stdio::inherit())
		                      .output()
		                      .ok()
		                      .and_then(|out| {
			                      out.status
			                         .success()
			                         .then(|| {
				                         std::str::from_utf8(&out.stdout).ok()?
				                                                         .strip_prefix(BIN_NAME)
				                                                         .map(|s| s.trim().to_owned())
				                                                         .filter(|s| !s.is_empty())
			                         })
			                         .flatten()
		                      })
		                      .map(|ver| (cfg.bin.as_path(), ver))
	}


	/// Prepare `Command` to run `pdbindgen`,
	/// but without content of the `cfg`.
	pub fn cmd(cfg: &Cfg) -> Option<Command> {
		Self::find_tool(cfg).and_then(|(path, _)| {
			                    let mut proc = Command::new(path);
			                    std::env::current_dir().map(|pwd| proc.current_dir(pwd)).ok();
			                    proc.envs(std::env::vars());
			                    proc.stderr(stderr());
			                    proc.into()
		                    })
	}


	pub fn gen_cmd(cfg: &Cfg) -> Option<Command> {
		Self::cmd(cfg).and_then(|mut proc| {
			              proc.args(cfg.to_cli_args());
			              proc.into()
		              })
	}


	pub fn find_sdk_version(cfg: &Cfg) -> Option<String> {
		// Path of the SDK:
		let path =
			cfg.sdk.clone().or_else(|| {
				               std::env::var(SDK_PATH_ENV_VAR).ok()
				                                              .filter(|s| !s.trim().is_empty())
				                                              .filter(|s| {
					                                              Path::new(s).try_exists().ok().unwrap_or(false)
				                                              })
				                                              .map(PathBuf::from)
			               });

		// Easiest way to get existing SDK version:
		let sdk_version = path.and_then(|path| {
			                      std::fs::read_to_string(path.join("VERSION.txt")).ok()
			                                                                       .map(|s| s.trim().to_string())
		                      })
		                      .filter(|s| !s.is_empty());

		// Alternative way is to execute the tool:
		if sdk_version.is_none() {
			Self::cmd(cfg)?.arg(FIND_SDK_VERSION_CMD)
			               .args(cfg.to_cli_args())
			               .stdout(Stdio::piped())
			               .output()
			               .ok()
			               .and_then(|out| {
				               out.status.success().then(|| {
					                                   std::str::from_utf8(&out.stdout).ok()
					                                                                   .map(|s| s.trim().to_owned())
					                                                                   .filter(|s| !s.is_empty())
				                                   })
			               })
			               .flatten()
		} else {
			sdk_version
		}
	}
}


impl Default for Cfg {
	fn default() -> Self {
		Self { sdk: Default::default(),
		       gcc: Default::default(),
		       derive: Default::default(),
		       features: Default::default(),
		       output: None,
		       bin: std::env::var_os("PDBINDGEN_PATH").map(PathBuf::from)
		                                              .unwrap_or_else(|| PathBuf::from(BIN_NAME)) }
	}
}


impl Cfg {
	pub fn to_cli_args(&self) -> Vec<String> {
		let mut args = vec![];

		if let Some(ref sdk) = self.sdk {
			args.push(format!("--sdk={}", sdk.display()));
		}

		if let Some(ref gcc) = self.gcc {
			args.push(format!("--gcc={}", gcc.display()));
		}

		args.extend(self.derive.to_cli_args());
		args.extend(self.features.to_cli_args());

		if let Some(ref path) = self.output {
			args.push(format!("--output={}", path.display()));
		}

		args
	}
}


#[derive(Debug, Clone, Copy)]
pub struct Derive {
	// standard
	pub default: bool,
	pub eq: bool,
	pub copy: bool,
	pub debug: bool,
	pub hash: bool,
	pub ord: bool,
	pub partialeq: bool,
	pub partialord: bool,
	// extra
	pub constparamty: bool,
}

impl Derive {
	pub const fn empty() -> Self {
		Self { default: false,
		       eq: false,
		       copy: false,
		       debug: false,
		       hash: false,
		       ord: false,
		       partialeq: false,
		       partialord: false,
		       constparamty: false }
	}

	pub fn to_cli_args(&self) -> Vec<String> {
		let words = self.to_feature_list();
		if words.is_empty() {
			vec!["--derive=".to_string()]
		} else {
			vec![format!("--derive={words}")]
		}
	}
}


impl FromStr for Derive {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut this = Derive::empty();

		if s.trim().is_empty() {
			return Ok(this);
		}

		for word in s.to_ascii_lowercase().split(',') {
			match word {
				"default" => this.default = true,
				"eq" => this.eq = true,
				"copy" => this.copy = true,
				"debug" => this.debug = true,
				"hash" => this.hash = true,
				"ord" => this.ord = true,
				"partialeq" => this.partialeq = true,
				"partialord" => this.partialord = true,
				"constparamty" => this.constparamty = true,
				_ => println!("cargo::warning=Unknown derive '{word}'."),
			}
		}

		Ok(this)
	}
}

impl Derive {
	#[rustfmt::skip]
	fn to_feature_list(&self) -> String {
		let mut out = Vec::new();
		if self.default { out.push("default") }
		if self.eq {out.push("eq")}
		if self.copy {out.push("copy")}
		if self.debug {out.push("debug")}
		if self.hash {out.push("hash")}
		if self.ord {out.push("ord")}
		if self.partialeq {out.push("partialeq")}
		if self.partialord {out.push("partialord")}
		if self.constparamty {out.push("constparamty")}
		out.join(",")
	}
}

impl std::fmt::Display for Derive {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_feature_list().fmt(f) }
}

impl Default for Derive {
	fn default() -> Self {
		Self { debug: true,
		       default: Default::default(),
		       eq: Default::default(),
		       copy: Default::default(),
		       hash: Default::default(),
		       ord: Default::default(),
		       partialeq: Default::default(),
		       partialord: Default::default(),
		       constparamty: Default::default() }
	}
}


#[derive(Debug, Clone, Copy)]
pub struct Features {
	pub documentation: bool,
}

impl Features {
	pub const fn empty() -> Self { Self { documentation: false } }

	pub fn to_cli_args(&self) -> Vec<String> {
		let words = self.to_feature_list();
		if words.is_empty() {
			vec!["--features=".to_string()]
		} else {
			vec![format!("--features={words}")]
		}
	}
}


impl FromStr for Features {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut this = Features::empty();

		if s.trim().is_empty() {
			return Ok(this);
		}

		for word in s.to_ascii_lowercase().split(',') {
			match word {
				"documentation" => this.documentation = true,
				_ => println!("cargo::warning=Unknown feature '{word}'."),
			}
		}

		Ok(this)
	}
}

impl Features {
	#[rustfmt::skip]
	fn to_feature_list(&self) -> String {
		let mut out = Vec::new();
		if self.documentation { out.push("documentation") }
		out.join(",")
	}
}

impl std::fmt::Display for Features {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_feature_list().fmt(f) }
}

impl Default for Features {
	fn default() -> Self { Self { documentation: true } }
}


/// Bindings output filename components.
#[derive(Debug, Clone)]
pub struct Filename {
	/// Version of the Playdate SDK.
	pub sdk: String,

	/// String representation of enabled features/derives.
	pub mask: DerivesMask,

	/// Current target.
	pub target: Target,
}

impl Filename {
	pub fn new<T: Into<DerivesMask>>(sdk: impl ToString, derives: T) -> Result<Self, VarError> {
		let target = Target::from_env_target()?;
		Ok(Self::new_for(sdk, derives, target))
	}

	#[inline(never)]
	pub fn new_for<T: Into<DerivesMask>>(sdk: impl ToString, derives: T, target: Target) -> Self {
		Self { sdk: sdk.to_string(),
		       target,
		       mask: derives.into() }
	}
}


impl std::fmt::Display for Filename {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let derives = &self.mask;
		let target = &self.target;
		let sdk = &self.sdk;
		write!(f, "pd{sdk}-{target}-{derives}.rs")
	}
}


/// Target representation.
/// There is not needed abi and arch because rust's arch in the corelib gives great abstraction.
#[derive(Debug, Clone)]
pub enum Target {
	Playdate,
	Other {
		/// Target pointer width in bits.
		ptr: String,

		/// Target arch.
		arch: String,

		/// Target OS.
		os: String,

		/// Target c_int size in bits.
		/// For playdate usually it should be 1 byte if compiled with `-fshort-enums`.
		/// For other targets it should be between size of `16` and `64` bits, usually `32`,
		/// before optimizations but it doesn't matter at all.
		c_int: usize,
	},
}

impl Target {
	/// Retrieve by cargo env vars.
	pub fn from_env_target() -> Result<Self, VarError> {
		use std::env::var;
		if var("TARGET")? == "thumbv7em-none-eabihf" {
			Ok(Self::Playdate)
		} else {
			use core::ffi::c_int;
			let ptr = var("CARGO_CFG_TARGET_POINTER_WIDTH")?;
			let arch = var("CARGO_CFG_TARGET_ARCH")?;
			let os = var("CARGO_CFG_TARGET_OS")?;
			Ok(Self::Other { os,
			                 arch,
			                 ptr,
			                 c_int: c_int::BITS as usize })
		}
	}
}

impl std::fmt::Display for Target {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Target::Playdate => write!(f, "pd"),
			Target::Other { os, ptr, arch, c_int } => write!(f, "{os}-{arch}-{ptr}-i{c_int}"),
		}
	}
}


#[derive(Debug, Clone)]
pub struct DerivesMask {
	inner: Vec<bool>,
}

impl DerivesMask {
	pub fn push(&mut self, value: bool) { self.inner.push(value) }
}


impl From<Derive> for DerivesMask {
	fn from(values: Derive) -> Self {
		// Caution: do not change the order of items!
		Self { inner: vec![
		                   values.default,
		                   values.eq,
		                   values.copy,
		                   values.debug,
		                   values.hash,
		                   values.ord,
		                   values.partialeq,
		                   values.partialord,
		                   values.constparamty,
		] }
	}
}

impl From<&'_ Derive> for DerivesMask {
	fn from(value: &'_ Derive) -> Self { (*value).into() }
}

impl std::fmt::Display for DerivesMask {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let iter = self.inner.iter().map(|v| if *v { "1" } else { "0" });
		write!(f, "{}", iter.collect::<String>())
	}
}
