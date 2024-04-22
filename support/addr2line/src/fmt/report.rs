use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Debug;
use std::io::prelude::*;
use std::io::Result;
use std::path::PathBuf;

use symbolic::common::Language;
use symbolic::common::Name;
use symbolic::demangle::Demangle;
use symbolic::demangle::DemangleOptions;

use super::addr::Addr;


pub const UNKNOWN: &str = "<unknown>";

pub const INDENT: &str = "\t";
pub const INLINE: &str = "  ";

pub trait WriteReport {
	fn default_print<W: Write>(&self,
	                           out: W,
	                           title: bool,
	                           indent: usize,
	                           functions: bool,
	                           basenames: bool,
	                           ranges: bool,
	                           demangle: bool)
	                           -> Result<()>;
}


#[derive(Debug)]
pub struct Report {
	/// Address asked to resolve.
	pub addr: Addr<u64>,

	/// Resolved symbol(s).
	///
	/// In case of inline chain there is many resolved symbols in ordering "into deep".
	/// Otherwise there is only one resolved symbol.
	pub symbols: Vec<Symbol>,
}

impl WriteReport for Report {
	fn default_print<W: Write>(&self,
	                           mut out: W,
	                           title: bool,
	                           indent: usize,
	                           functions: bool,
	                           basenames: bool,
	                           ranges: bool,
	                           demangle: bool)
	                           -> Result<()> {
		let indent = indent;
		let mut indented = INDENT.repeat(indent);
		let addr = self.addr.as_value();
		let local: Cow<_> = if self.addr.is_fixed() {
			format!(" [=>{:#08x}]", self.addr.fixed()).into()
		} else {
			"".into()
		};

		match (title, self.symbols.is_empty()) {
			(true, true) => return write!(out, "{indented}{addr:#08x}{local}:??\n"),
			(true, false) => write!(out, "{indented}{addr:#08x}{local}:\n")?,
			(false, true) => write!(out, "{indented}??\n")?,
			(false, false) => (),
		}

		// Inlined symbols - inner indent:
		let mut inline = 0;
		for sym in self.symbols.iter() {
			indented = INDENT.repeat(indent) + &INLINE.repeat(inline);
			write!(out, "{indented}")?;

			if functions {
				write_name(&mut out, sym.name.as_ref(), demangle)?;
				if ranges {
					write_range(&mut out, sym.address, sym.size)?;
				}

				if let Some(id) = sym.hw_id {
					write!(out, " [hw:{id}]")?;
				}
			}

			for line in &sym.lines {
				write!(out, "\n{indented}  at ")?;

				let file = if basenames {
					line.file.file_name().unwrap_or_else(|| line.file.as_os_str())
				} else {
					line.file.as_os_str()
				};

				let hw_id = line.hw_id
				                .map(|v| Cow::from(format!(" [hw:{v}]")))
				                .unwrap_or_default();

				let line_rendered = line.line
				                        .as_ref()
				                        .map(|v| v.to_string())
				                        .unwrap_or_else(|| "??".to_string());
				write!(out, "{}:{line_rendered}", file.to_string_lossy().as_ref())?;
				if ranges {
					write_range(&mut out, line.address, line.size)?;
				}
				write!(out, "{hw_id}")?;
			}
			writeln!(out)?;
			inline += 1;
		}

		out.flush()
	}
}


pub fn write_name<'a, N: Borrow<Name<'a>>, W: Write>(mut out: W, name: Option<N>, demangle: bool) -> Result<()> {
	match name.as_ref().map(Borrow::borrow) {
		None => write!(out, "??"),
		Some(name) if name.as_str().is_empty() => write!(out, "??"),
		Some(name) if demangle => {
			let opts = DemangleOptions::name_only();
			if let Some(name) = name.demangle(opts).map(Cow::from) {
				write!(out, "{}", name)
			} else {
				let mut name = name.clone();
				name.set_language(Language::Unknown);
				let name = name.try_demangle(opts);
				write!(out, "{}", name)
			}
		},
		Some(name) => write!(out, "{name}"),
	}
}

pub fn write_range<W: Write>(mut out: W, start: u64, len: Option<u64>) -> Result<()> {
	write!(out, " ({start:#x} - ")?;
	match len {
		Some(len) => write!(out, "{:#x})", start + len),
		None => write!(out, "??)"),
	}
}


#[derive(Debug, Clone)]
pub struct Symbol {
	/// Hardware ID where this symbol was found at the `address`.
	pub hw_id: Option<i64>,

	/// Symbol name.
	pub name: Option<Name<'static>>,

	/// Relative symbol address.
	pub address: u64,

	/// In case of symbol is known function,
	/// this is a total code size covered by the function body, including inlined functions.
	///
	/// Otherwise this is a symbol size and can be unknown.
	pub size: Option<u64>,

	pub lines: Vec<Span>,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
	pub hw_id: Option<i64>,

	/// The instruction address relative to the image base (load address).
	pub address: u64,
	/// Total code size covered by this line record.
	pub size: Option<u64>,
	/// File path.
	pub file: PathBuf,
	/// Absolute line number starting at 1. Zero means no line number.
	pub line: Option<u64>,
}
