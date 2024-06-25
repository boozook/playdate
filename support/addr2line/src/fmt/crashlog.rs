use std::collections::HashMap;
use std::io::prelude::Write;
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

// impl<T> CrashLog<T> {
// 	pub fn new() -> Self {
// 		//
// 		//
// 		//
// 	}
// }


impl<T> CrashLog<T> {
	pub fn pretty_print_sub_title<W: Write>(&self, mut out: W, indent: &str, title: &str) -> std::io::Result<()> {
		writeln!(out, "{indent}  {title}")
	}

	fn pretty_print_head<W: Write>(&self, mut out: W, title: bool, indent: usize) -> std::io::Result<()> {
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

		Ok(())
	}
}


impl<T: Copy> CrashLog<Addr<T>> where Addr<T>: std::cmp::Eq + std::hash::Hash + WriteReport {
	pub fn pretty_print_regs<'a, 'b, W, I>(&self,
	                                       mut out: W,
	                                       regs: I,
	                                       resolved: &HashMap<Addr<T>, Report>,
	                                       header: &str,
	                                       title: bool,
	                                       indent: usize,
	                                       funcs: bool,
	                                       short: bool,
	                                       ranges: bool,
	                                       demangle: bool)
	                                       -> std::io::Result<()>
		where W: Write,
		      I: IntoIterator<Item = (&'a str, &'b str)>
	{
		let indented = INDENT.repeat(indent);

		// title:
		self.pretty_print_sub_title(&mut out, &indented, header)?;


		for (k, name) in regs {
			let addr = &self.ptrs[k];

			let space = " ".repeat(8 - name.len());
			write!(out, "{indented}  {name}:{space}")?;
			addr.default_print(&mut out, title, indent, funcs, short, ranges, demangle)?;
			write!(out, ":")?;

			if let Some(report) = resolved.get(addr) &&
			   !report.symbols.is_empty()
			{
				write!(out, "\n")?;
				report.default_print(&mut out, false, indent + 2, funcs, short, ranges, demangle)?;
			} else {
				write!(out, " ??\n")?;
			}
		}
		Ok(())
	}

	/// General-purpose registers
	pub fn pretty_print_gpr<W: Write>(&self,
	                                  mut out: W,
	                                  resolved: &HashMap<Addr<T>, Report>,
	                                  title: bool,
	                                  indent: usize,
	                                  funcs: bool,
	                                  short: bool,
	                                  ranges: bool,
	                                  demangle: bool)
	                                  -> std::io::Result<()> {
		let gpr = [
		           ("r0", "0"),
		           ("r1", "1"),
		           ("r2", "2"),
		           ("r3", "3"),
		           ("r12", "12"),
		           ("lr", "14 LR"),
		           ("pc", "15 PC"),
		];
		let header = "General-purpose registers (stack)";
		self.pretty_print_regs(&mut out, gpr, resolved, header, title, indent, funcs, short, ranges, demangle)

		// let indented = INDENT.repeat(indent);

		// // title:
		// self.pretty_print_sub_title(&mut out, &indented, "General-purpose registers (stack)")?;

		// for k in GPR {
		// 	let addr = &self.ptrs[*k];

		// 	let space = " ".repeat(8 - k.len());
		// 	write!(out, "{indented}  {k}:{space}")?;
		// 	addr.default_print(&mut out, title, indent, funcs, short, ranges, demangle)?;
		// 	write!(out, ":")?;

		// 	if let Some(report) = resolved.get(addr) &&
		// 	   !report.symbols.is_empty()
		// 	{
		// 		write!(out, "\n")?;
		// 		report.default_print(&mut out, false, indent + 2, funcs, short, ranges, demangle)?;
		// 	} else {
		// 		write!(out, " ??\n")?;
		// 	}
		// }
		// Ok(())
	}


	/// Special registers
	pub fn pretty_print_sr<W: Write>(&self,
	                                 mut out: W,
	                                 resolved: &HashMap<Addr<T>, Report>,
	                                 title: bool,
	                                 indent: usize,
	                                 funcs: bool,
	                                 short: bool,
	                                 ranges: bool,
	                                 demangle: bool)
	                                 -> std::io::Result<()>
		where crate::ef::PSR: TryFrom<T>,
		      crate::ef::CFSR: TryFrom<T>,
		      crate::ef::HSFR: TryFrom<T>
	{
		use crate::ef::*;


		let gpr = [("mmfar", "MMFAR"), ("bfar", "BFAR"), ("rcccsr", "rcccsr")];
		let header = "Special registers";
		self.pretty_print_regs(&mut out, gpr, resolved, header, title, indent, funcs, short, ranges, demangle)?;

		let indented = INDENT.repeat(indent);


		fn print_reg<W: Write, R: RegTags + std::fmt::Binary>(mut out: W,
		                                                      reg: R,
		                                                      name: &str,
		                                                      doc: &str,
		                                                      indent: &str)
		                                                      -> std::io::Result<()> {
			if !reg.is_empty() {
				let space = " ".repeat(8 - name.len());
				writeln!(out, "{indent} {space}{name}:  {reg:#032b} {doc}")?;

				let mut max_tag_len = 0;
				for (tag, doc) in reg.tags() {
					max_tag_len = max_tag_len.max(tag.len());
					let doc: Vec<_> = doc.trim()
					                     .lines()
					                     .map(|s| s.trim())
					                     .filter(|s| !s.is_empty())
					                     .collect();
					let gap = " ".repeat(max_tag_len - tag.len());
					let pre = format!("{indent} {space}   {tag}");
					write!(out, "{pre}")?;
					if !doc.is_empty() {
						writeln!(out, ":{gap} {}", doc.first().unwrap())?;
						let space = " ".repeat(pre.len());
						for line in doc.into_iter().skip(1) {
							writeln!(out, "{space} {gap} {line}")?;
						}
					}
				}
			}

			Ok(())
		}

		// PSR:
		if let Some(Ok(reg)) = self.ptrs.get("psr").map(|a| a.value()).map(PSR::try_from) {
			let space = " ".repeat(8 - "PSR".len());
			let doc = "(Program Status Register)";
			writeln!(out, "{indented}  PSR:{space} {reg:#032b} {doc}")?;
			let apsr = reg.apsr();
			let ipsr = reg.ipsr();
			let epsr = reg.epsr();


			print_reg(
			          &mut out,
			          apsr,
			          "APSR",
			          "(Application Program Status Register)",
			          &indented,
			)?;
			print_reg(
			          &mut out,
			          ipsr,
			          "IPSR",
			          "(Interrupt Program Status Register)",
			          &indented,
			)?;
			print_reg(
			          &mut out,
			          epsr,
			          "EPSR",
			          "(Execution Program Status Register)",
			          &indented,
			)?;
		}


		// CFSR:
		if let Some(Ok(reg)) = self.ptrs.get("cfsr").map(|a| a.value()).map(CFSR::try_from) {
			let space = " ".repeat(8 - "CFSR".len());
			let doc = "(Configurable Fault Status)";
			writeln!(out, "{indented}  CFSR:{space} {reg:#032b} {doc}")?;
			let ufsr = reg.ufsr();
			let bfsr = reg.bfsr();
			let mmfsr = reg.mmfsr();


			print_reg(&mut out, ufsr, "UFSR", "(UsageFault Status Register)", &indented)?;
			print_reg(&mut out, bfsr, "BFSR", "(BusFault Status Register)", &indented)?;
			print_reg(
			          &mut out,
			          mmfsr,
			          "MMFSR",
			          "(MemManage Fault Status Register)",
			          &indented,
			)?;
		}


		// HSFR:
		if let Some(Ok(reg)) = self.ptrs.get("hsfr").map(|a| a.value()).map(HSFR::try_from) {
			let doc = "(HardFault Status Register)";
			print_reg(&mut out, reg, "HSFR", doc, &indented)?;
		}

		Ok(())
	}


	pub fn pretty_print<W: Write>(&self,
	                              mut out: W,
	                              resolved: &HashMap<Addr<T>, Report>,
	                              title: bool,
	                              indent: usize,
	                              funcs: bool,
	                              short: bool,
	                              ranges: bool,
	                              demangle: bool)
	                              -> std::io::Result<()>
		where crate::ef::PSR: TryFrom<T>,
		      crate::ef::CFSR: TryFrom<T>,
		      crate::ef::HSFR: TryFrom<T>
	{
		// Header:
		self.pretty_print_head(&mut out, title, indent)?;

		// Stack:
		self.pretty_print_gpr(&mut out, resolved, title, indent, funcs, short, ranges, demangle)?;

		// Registers:
		self.pretty_print_sr(&mut out, resolved, title, indent, funcs, short, ranges, demangle)?;

		writeln!(out)
	}
}


impl WriteReport for CrashLog<&'_ Report> {
	fn default_print<W: Write>(&self,
	                           mut out: W,
	                           title: bool,
	                           indent: usize,
	                           funcs: bool,
	                           short: bool,
	                           ranges: bool,
	                           demangle: bool)
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
			 .default_print(&mut out, title, indent, funcs, short, ranges, demangle)?;
			write!(out, ":")?;
			if v.symbols.is_empty() {
				write!(out, " ??\n")?;
			} else {
				write!(out, "\n")?;
				v.default_print(&mut out, false, indent + 2, funcs, short, ranges, demangle)?;
			}
		}

		writeln!(out)?;

		Ok(())
	}
}
