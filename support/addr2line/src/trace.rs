use std::ops::Range;

use crate::fmt::addr::Addr;


#[derive(Debug)]
pub enum TraceLine<'t> {
	Addr {
		addr: Addr,
		/// System task or thread name 🤷🏻‍♂️
		task: Option<&'t str>,
		/// Indent level (start .. start+len)
		level: Range<usize>,
	},
	Other {
		text: &'t str,
		/// Indent level (start .. start+len)
		level: Range<usize>,
	},
	Error {
		line: &'t str,
		error: anyhow::Error,
	},
}


pub fn parse_trace_line(line: &str) -> TraceLine<'_> {
	let line = line.trim_end();

	// count indent in spaces:
	let i = line.chars().into_iter().take_while(|c| *c == ' ').count();

	let addr_task = line.trim_start().strip_prefix("~tr:").map(str::trim);
	let trimmed = line.trim();

	if let Some(text) = addr_task {
		let addr;
		let task;

		if let Some((addr_, task_)) = text.split_once('\t').or_else(|| text.split_once(' ')) {
			addr = addr_.trim();
			task = Some(task_.trim());
		} else {
			addr = text;
			task = None;
		}

		match crate::fmt::addr::parse_addr(addr) {
			Ok(addr) => {
				TraceLine::Addr { addr: addr as _,
				                  task,
				                  level: i..(trimmed.len() + i) }
			},
			Err(err) => {
				TraceLine::Error { line: trimmed,
				                   error: err }
			},
		}
	} else {
		TraceLine::Other { text: trimmed,
		                   level: i..(trimmed.len() + i) }
	}
}
