#[macro_use]
extern crate log;
extern crate playdate_symbolize as pd;

use futures_util::StreamExt;
use symbolic::debuginfo::ObjectError;
use tokio::io::{AsyncBufReadExt, BufReader};

use pd::{elf, fmt, db};
use elf::map_result_fallback_os;
use fmt::report::WriteReport;
use fmt::report::Report;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let cfg = cli_ext::parse();
	cli::env_logger_init();

	let os_db = cli::init_db_resolver(&cfg.base.symbols).await?;

	if cfg.input.is_empty() {
		let (tx, elf_res) =
		elf::Resolver::new(&cfg.elf.path, cfg.base.flags.functions, cfg.base.flags.inlinees)?.into_stream_lazy(None::<[_; 0]>)?;
		let mut elf_res = Box::pin(elf_res);


		let mut reader = BufReader::new(tokio::io::stdin()).lines();
		let send_addrs = tokio::spawn(async move {
			while let Ok(Some(line)) = reader.next_line().await {
				match fmt::addr::parse_addr(&line) {
					Ok(v) => tx.send(v)?,
					Err(err) => eprintln!("Error: {err}"),
				}
			}
			Ok::<_, tokio::sync::mpsc::error::SendError<_>>(())
		});

		while let Some(res) = elf_res.next().await {
			process_elf_result(res, &os_db, &cfg.base.flags).await?;
		}

		send_addrs.await??;
	} else {
		let elf_res =
			elf::Resolver::new(&cfg.elf.path, cfg.base.flags.functions, cfg.base.flags.inlinees)?.into_stream_resolve(&cfg.input)?;
		let mut elf_res = Box::pin(elf_res);

		while let Some(res) = elf_res.next().await {
			process_elf_result(res, &os_db, &cfg.base.flags).await?;
		}
	}

	os_db.close().await?;

	Ok(())
}


async fn process_elf_result(res: Result<Report, ObjectError>,
                            db: &db::Resolver,
                            flags: &cli::Flags)
                            -> Result<(), anyhow::Error> {
	match map_result_fallback_os(res, db).await {
		Ok(rep) => {
			rep.default_print(
			                  std::io::stdout(),
			                  true,
			                  0,
			                  flags.functions,
			                  flags.basenames,
			                  flags.ranges,
			                  flags.demangle,
			)?
		},
		Err(err) => error!("{err}"),
	}
	Ok(())
}


mod cli;
mod cli_ext {
	pub use super::cli::*;
	use super::fmt::addr::{parse_addr, Addr};
	use clap::{Parser, builder};


	pub fn parse() -> Cfg { Cfg::parse() }


	const NAME: &str = env!("CARGO_BIN_NAME");
	const ABOUT_TMT: &str = r#"{name} {version}
The tool translates addresses into file names and line numbers.
Given an address in an executable or an offset in a section of a relocatable object, it uses symbols database and debugging information to figure out which file name and line number are associated with it.
{name} has two modes of operation.
  In the first, hexadecimal addresses are specified on the command line, and {name} displays the file name and line number for each address.
  In the second, utility reads hexadecimal addresses from standard input, and prints the file name and line number for each address on standard output. So {name} may be used in a pipe to convert dynamically chosen addresses."#;

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
		pub elf: ElfPath,

		/// Addresses to be translated.
		#[clap(name = "addresses", value_name = "ADDRS", value_parser = builder::ValueParser::new(parse_addr))]
		pub input: Vec<Addr>,
	}
}
