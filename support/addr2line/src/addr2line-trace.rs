#[macro_use]
extern crate log;
extern crate playdate_symbolize as pd;

use std::borrow::Cow;
use std::ops::Range;
use std::path::Path;

use anyhow::Result;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio::io::BufReader;
use tokio_stream::StreamExt;

use pd::{elf, fmt, trace};
use fmt::report::WriteReport;
use elf::map_result_fallback_os;
use trace::{parse_trace_line, TraceLine};


#[tokio::main]
async fn main() -> Result<()> {
	let cfg = cli_ext::parse();
	cli::env_logger_init();

	if let Some(path) = cfg.input {
		process_trace_file(&path, cfg.elf.path.as_deref(), &cfg.base.symbols, &cfg.base.flags).await?;
	} else {
		let reader = tokio::io::stdin();
		process_trace(
		              reader,
		              cfg.elf.path.as_deref(),
		              &cfg.base.symbols,
		              &cfg.base.flags,
		).await?;
	};


	Ok(())
}


async fn process_trace_file(path: &Path,
                            elf: Option<&Path>,
                            db: &cli::SdkPath,
                            flags: &cli::Flags)
                            -> Result<()> {
	let path = path.canonicalize()?;
	let reader = tokio::fs::File::open(path).await?;
	process_trace(reader, elf, db, flags).await
}


async fn process_trace<R: AsyncRead>(reader: R,
                                     elf: Option<&Path>,
                                     db: &cli::SdkPath,
                                     flags: &cli::Flags)
                                     -> Result<()> {
	let reader = Box::pin(BufReader::new(reader));
	let mut lines = reader.lines();


	let os_db = cli::init_db_resolver(db).await?;


	let tx;
	let mut elf_res;
	if let Some(elf) = elf {
		let (tx_, elf_res_) =
			elf::Resolver::new(elf, flags.functions, flags.inlinees)?.into_stream_lazy(None::<[_; 0]>)?;
		tx = Some(tx_);
		elf_res = Some(Box::pin(elf_res_));
	} else {
		tx = None;
		elf_res = None;
	}

	let mut indent = 0;
	let mut level_prev = 0;
	let update_indent = |indent: &mut usize, lprev: &mut usize, level: &Range<usize>| {
		if level.start > *lprev {
			*indent += 1;
		} else if level.start < *lprev {
			*indent = 0;
		}
		*lprev = level.start;
	};
	let indented = |indent| fmt::report::INDENT.repeat(indent);

	// stats:
	let mut missed = 0;
	let mut resolved = 0;

	while let Some(line) = lines.next_line().await? {
		match parse_trace_line(&line) {
			TraceLine::Addr { addr, task, level } => {
				update_indent(&mut indent, &mut level_prev, &level);
				let indented = indented(indent);
				let task: Cow<_> = task.map(|s| format!(" ({s})").into()).unwrap_or_default();
				print!("{indented}{addr:#08x}:{task}");

				if let Some(tx) = tx.as_ref() {
					tx.send(addr)?;
				}

				let res = if let Some(elf_res) = elf_res.as_mut() {
					if let Some(res) = elf_res.next().await {
						map_result_fallback_os(res, &os_db).await
					} else {
						os_db.resolve(addr.value() as _).await
					}
				} else {
					os_db.resolve(addr.value() as _).await
				};

				match res {
					Ok(rep) => {
						if rep.symbols.is_empty() {
							missed += 1;
							print!(" ??\n");
						} else {
							resolved += 1;
							print!("\n");
							rep.default_print(
							                  std::io::stdout(),
							                  false,
							                  indent,
							                  flags.functions,
							                  flags.basenames,
							                  flags.ranges,
							                  flags.demangle,
							)?
						}
					},
					Err(err) => error!("{indented}{err}"),
				}
			},
			TraceLine::Other { text, level } => {
				update_indent(&mut indent, &mut level_prev, &level);
				let indented = indented(indent);
				println!("{indented}{text}");
			},
			TraceLine::Error { line, error } => {
				error!("{error}, line: '{line}'");
			},
		}
	}

	os_db.close().await;

	info!("Resolved addresses: {resolved}");
	info!("Unknown addresses: {missed}");

	Ok(())
}


mod cli;
mod cli_ext {
	pub use super::cli::*;
	use std::path::PathBuf;
	use clap::Parser;


	pub fn parse() -> Cfg { Cfg::parse() }


	const NAME: &str = env!("CARGO_BIN_NAME");
	const ABOUT_TMT: &str = r#"{name} {version}
The tool translates addresses in the given trace-dump into file names and line numbers.
Given an trace, it uses symbols database and debugging information to figure out which file name and line number are associated with each address in it.
{name} has two modes of operation.
  In the first, path of trace-dump are specified on the command line, and {name} displays the entire text adding symbol information.
  In the second, utility reads trace from standard input, and prints it with inserted symbol information on standard output. In this mode, {name} may be used in a pipe to trace dynamically."#;

	const fn about() -> &'static str {
		const A: &str = const_str::replace!(ABOUT_TMT, "utility", NAME);
		const B: &str = const_str::replace!(A, "{name}", NAME);
		const C: &str = const_str::replace!(B, "{version}", env!("CARGO_PKG_VERSION"));
		C
	}

	#[derive(Parser, Debug)]
	#[command(author, version, about = about(), name = NAME)]
	pub struct Cfg {
		#[clap(flatten)]
		pub base: Base,

		#[clap(flatten)]
		pub elf: ElfPathOpt,

		/// Path to raw trace file.
		#[clap(name = "raw trace", value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
		pub input: Option<PathBuf>,
	}

	#[derive(Parser, Debug)]
	pub struct ElfPathOpt {
		/// Path of the executable for which addresses should be translated.
		#[clap(long = "exe", short = 'e', number_of_values = 1, required = false)]
		pub path: Option<PathBuf>,
	}
}
