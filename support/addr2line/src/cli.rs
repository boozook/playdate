//! CLI-related things for all bins.

use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum};
use utils::consts::SDK_ENV_VAR;

use ::playdate_symbolize::db;


pub fn env_logger_init() {
	env_logger::Builder::from_env(env_logger::Env::default()).format_indent(Some(3))
	                                                         .format_module_path(false)
	                                                         .format_target(true)
	                                                         .format_timestamp(None)
	                                                         .init();
}


#[derive(Parser, Debug)]
pub struct Base {
	/// Standard output format.
	#[clap(long, global = true, default_value_t = Format::Human)]
	#[clap(hide = true)] // Hidden because it's not implemented yet.
	pub format: Format,

	/// Device hardware revision: name or number.
	#[clap(long, global = true, default_value_t = Revision::Auto)]
	pub rev: Revision,

	#[clap(flatten)]
	pub flags: Flags,

	#[clap(flatten)]
	pub symbols: SdkPath,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Format {
	Human,
	#[clap(hide = true)]
	Json,
}

impl std::fmt::Display for Format {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Human => "human".fmt(f),
			Self::Json => "json".fmt(f),
		}
	}
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Revision {
	#[clap(name = "first", alias = "1")]
	First = 1,
	#[clap(hide = true)] // Hidden because it's not implemented yet.
	#[clap(name = "second", alias = "2")]
	Second = 2,
	Auto = 999,
}

impl std::fmt::Display for Revision {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Auto => "auto".fmt(f),
			Self::First => "1".fmt(f),
			Self::Second => "2".fmt(f),
		}
	}
}

#[derive(Parser, Debug)]
pub struct ElfPath {
	/// Path of the executable for which addresses should be translated.
	#[clap(long = "exe", short = 'e', number_of_values = 1)]
	pub path: PathBuf,
}


#[derive(Parser, Debug)]
pub struct SdkPath {
	/// Playdate OS symbols database.
	#[clap(long, name = "symbols.db", visible_alias = "db", value_name = "FILE", conflicts_with = "sdk", value_hint = clap::ValueHint::FilePath)]
	pub symbols: Option<PathBuf>,

	/// Playdate SDK path.
	#[arg(long, env = SDK_ENV_VAR, value_name = "DIRECTORY", conflicts_with = "symbols.db", value_hint = clap::ValueHint::DirPath)]
	pub sdk: Option<PathBuf>,
}


#[derive(Args, Debug)]
pub struct Flags {
	/// Decode (demangle) low-level symbol names into user-level names.
	/// Besides removing any initial underscore prepended by the system, this makes Rust or C++ function names readable.
	#[clap(long, short = 'C')]
	pub demangle: bool,

	/// Display function names as well as file and line number information.
	#[clap(long, short = 'f')]
	pub functions: bool,

	/// Display function address ranges in addition to function names.
	#[clap(long, short = 'r')]
	pub ranges: bool,

	/// Display only the base of each file name.
	#[clap(long, short = 's')]
	pub basenames: bool,

	/// If the address belongs to a function that was inlined,
	/// the source information for all enclosing scopes back to the first non-inlined function will also be printed.
	///
	/// For example, if "main" inlines "callee1" which inlines "callee2", and address is from "callee2",
	/// the source information for "callee1" and "main" will also be printed.
	#[clap(long, short = 'i')]
	pub inlinees: bool,
}


pub async fn init_db_resolver(cfg: &SdkPath) -> Result<db::Resolver, anyhow::Error> {
	let os_bd = if let Some(path) = cfg.symbols.as_deref() {
		match db::Resolver::with_exact(path).await {
			Ok(res) => res,
			Err(err) => {
				error!("{err}");
				db::Resolver::new(cfg.sdk.as_deref()).await?
			},
		}
	} else {
		db::Resolver::new(cfg.sdk.as_deref()).await?
	};
	Ok(os_bd)
}
