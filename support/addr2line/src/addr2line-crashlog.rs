#[macro_use]
extern crate log;
extern crate playdate_symbolize as pd;

use std::collections::HashMap;
use std::collections::HashSet;

use futures_util::StreamExt;
use futures_util::TryStreamExt;

use pd::elf::map_result_fallback_os;
use pd::fmt::crashlog::CrashLog;
use pd::fmt::report::WriteReport as _;
use pd::*;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let cfg = cli_ext::parse();
	cli::env_logger_init();

	let os_db = cli::init_db_resolver(&cfg.base.symbols).await?;

	let docs = fmt::crashlog::parse_file(&cfg.input)?;
	let stream = futures_util::stream::iter(docs);

	let (tx, elf_res) = elf::Resolver::new(&cfg.elf.path, cfg.base.flags.functions, cfg.base.flags.inlinees)?.into_stream_lazy(None::<[_; 0]>)?;
	let mut elf_res = Box::pin(elf_res);

	let mut addrs = HashSet::with_capacity(20);
	let docs: Vec<_> = {
		let addrs = &mut addrs;
		stream.map(move |doc| -> Result<_, anyhow::Error> {
			      let count = doc.ptrs
			                     .values()
			                     .filter(|v| addrs.insert(**v))
			                     .map_while(|v| tx.send(*v).map_err(|err| error!("{err}")).ok())
			                     .count();
			      let date = doc.date.as_deref().unwrap_or("n/a");
			      debug!("sent to resolve {count} addrs for doc by {date}");
			      Ok(doc)
		      })
		      .inspect_err(|err| error!("{err}"))
		      .filter_map(|res| async { res.ok() })
		      .collect()
		      .await
	};
	debug!("Unique addresses: {}", addrs.len());


	let mut resolved_addrs = HashMap::new();
	while let Some(res) = elf_res.next().await {
		let rep = map_result_fallback_os(res, &os_db).await?;
		trace!("Report for {:#08x?}, syms: {}", rep.addr, rep.symbols.len());
		if !resolved_addrs.contains_key(&rep.addr.as_value()) {
			debug!(
			       "Store report for {:#08x?} as {:#08x?}",
			       rep.addr,
			       rep.addr.as_value()
			);
			resolved_addrs.insert(rep.addr.as_value(), rep);
		}
	}
	debug!("Resolved addresses: {}", resolved_addrs.len());

	os_db.close().await;

	let docs: Vec<_> = docs.into_iter()
	                       .map(|doc| {
		                       let CrashLog { date,
		                                      build,
		                                      heap,
		                                      ptrs, } = doc;
		                       let ptrs = ptrs.into_iter()
		                                      .map(|(k, v)| {
			                                      match resolved_addrs.get(&v) {
				                                      Some(rep) => (k, rep),
			                                         None => unreachable!("Key not found: {v:#08x?}"),
			                                      }
		                                      })
		                                      .collect();
		                       CrashLog { date,
		                                  build,
		                                  heap,
		                                  ptrs }
	                       })
	                       .collect();

	for doc in docs {
		doc.default_print(
		                  std::io::stdout(),
		                  true,
		                  0,
		                  cfg.base.flags.functions,
		                  cfg.base.flags.basenames,
		                  cfg.base.flags.ranges,
		                  cfg.base.flags.demangle,
		)?;
	}

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
The tool translates addresses in the given crashlog into file names and line numbers.
Given an crashlog text, it uses symbols database and debugging information to figure out which file name and line number are associated with each address in it."#;

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

		/// Path to crashlog file.
		#[clap(name = "crashlog.txt", value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
		pub input: PathBuf,
	}
}
