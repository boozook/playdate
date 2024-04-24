use std::collections::HashMap;
use std::path::Path;
use std::io::Read;

use regex::Regex;

use crate::fmt::addr::Addr;
use crate::fmt::addr::DEF;

use super::report::Report;
use super::report::WriteReport;
use super::report::INDENT;


pub fn parse_file(path: &Path) -> anyhow::Result<impl Iterator<Item = CrashLog>> {
	let path = path.canonicalize()?;
	let reader = std::fs::File::open(path)?;
	parse(reader)
}


pub fn parse<R: Read>(mut reader: R) -> anyhow::Result<impl Iterator<Item = CrashLog>> {
	let blocks: Vec<_> = {
		let mut log = String::new();
		reader.read_to_string(&mut log)?;
		log.split("\n\n")
		   .filter(|s| !s.is_empty())
		   .map(|s| s.trim().to_owned())
		   .collect()
	};

	pub const DATE: &str = r"^.*crash at\s+([\d:/ ]+).*$";
	pub const BUILD: &str = r"^.*build:\s*(.*)$";
	pub const HEAP: &str = r"^.*heap allocated:\s*(\d+).*$";
	pub const VARS: &str = r"([a-z]+[0-9]*):\s*([0-9a-f]{8})(?:\s|$)";
	let re_date = Regex::new(DATE)?;
	let re_build = Regex::new(BUILD)?;
	let re_heap = Regex::new(HEAP)?;
	let re_vars = Regex::new(VARS)?;

	let res =
		blocks.into_iter().map(move |block| {
			                  let mut date = None;
			                  let mut build = None;
			                  let mut heap = None;
			                  let mut ptrs = HashMap::with_capacity(13);

			                  for line in block.lines() {
				                  if date.is_none() {
					                  date = re_date.captures(&line)
					                                .map(|r| r.get(1))
					                                .flatten()
					                                .map(|r| r.as_str());
				                  }

				                  if build.is_none() {
					                  build = re_build.captures(&line)
					                                  .map(|r| r.get(1))
					                                  .flatten()
					                                  .map(|r| r.as_str());
				                  }

				                  if heap.is_none() {
					                  heap = re_heap.captures(&line)
					                                .map(|r| r.get(1))
					                                .flatten()
					                                .map(|r| r.as_str().parse::<DEF>().map_err(|err| error!("{err}")).ok())
					                                .flatten();
				                  }

				                  for (_, [var, val]) in re_vars.captures_iter(&line).map(|caps| caps.extract()) {
					                  match DEF::from_str_radix(val, 16) {
						                  Ok(val) => {
						                     ptrs.insert(var.to_owned(), Addr::new(val));
					                     },
					                     Err(err) => error!("{err}"),
					                  }
				                  }
			                  }

			                  CrashLog { date: date.map(ToOwned::to_owned),
			                             build: build.map(ToOwned::to_owned),
			                             heap,
			                             ptrs }
		                  });
	Ok(res)
}


#[derive(Debug)]
pub struct CrashLog<T = Addr<DEF>> {
	pub date: Option<String>,
	pub build: Option<String>,
	pub heap: Option<DEF>,
	pub ptrs: HashMap<String, T>,
}


impl WriteReport for CrashLog<&'_ Report> {
	fn default_print<W: std::io::prelude::Write>(&self,
	                                             mut out: W,
	                                             title: bool,
	                                             indent: usize,
	                                             funcs: bool,
	                                             short: bool,
	                                             ranges: bool,
	                                             nice: bool)
	                                             -> std::io::Result<()> {
		let indented = INDENT.repeat(indent);
		if title {
			write!(out, "{indented}Crash")?;
			if let Some(s) = self.date.as_deref() {
				writeln!(out, " at {s}")?;
			} else {
				writeln!(out)?;
			}
		}

		if let Some(s) = self.build.as_deref() {
			writeln!(out, "{indented}  BUILD:   {s}")?;
		}

		if let Some(v) = &self.heap {
			writeln!(out, "{indented}  HEAP:    {v}")?;
		}

		let mut keys: Vec<_> = self.ptrs.keys().collect();
		keys.sort();

		for (k, v) in keys.into_iter().map(|k| (k, &self.ptrs[k])) {
			let space = " ".repeat(8 - k.len());
			write!(out, "{indented}  {k}:{space}")?;
			v.addr
			 .default_print(&mut out, title, indent, funcs, short, ranges, nice)?;
			write!(out, ":")?;
			if v.symbols.is_empty() {
				write!(out, " ??\n")?;
			} else {
				write!(out, "\n")?;
				v.default_print(&mut out, false, indent + 2, funcs, short, ranges, nice)?;
			}
		}

		writeln!(out)?;

		Ok(())
	}
}
