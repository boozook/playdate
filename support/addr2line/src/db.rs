use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use anyhow::Result;
use rusqlite::Connection;
use symbolic::common::{Language, Name, NameMangling};
use utils::toolchain::sdk::Sdk;

use crate::fmt::report::{self, UNKNOWN};


const QUERY_FN: &str = include_str!("query-fn.sql");
const QUERY_LN: &str = include_str!("query-ln.sql");


pub struct Resolver {
	conn: Connection,
}

impl Resolver {
	pub async fn new(sdk: Option<&Path>) -> Result<Self> {
		let sdk = sdk.map_or_else(Sdk::try_new, Sdk::try_new_exact)?;
		let path = if cfg!(target_os = "macos") {
			"Playdate Simulator.app/Contents/Resources/symbols.db"
		} else if cfg!(unix) {
			"symbols.db"
		} else if cfg!(windows) {
			"symbols.db"
		} else {
			const MSG: &str = "Unsupported platform, can't find symbols.db";
			return Err(IoError::new(IoErrorKind::Unsupported, MSG).into());
		};

		let path = sdk.bin().join(path).canonicalize()?;
		Self::with_exact(&path).await
	}

	pub async fn with_exact(db_path: &Path) -> Result<Self> {
		let conn = Connection::open(db_path)?;
		Ok(Self { conn })
	}

	pub async fn close(self) -> rusqlite::Result<()> {
		self.conn.close().map_err(|(conn, err)| {
			                 conn.close().ok();
			                 err
		                 })
	}


	pub async fn resolve(&self, addr: u32) -> anyhow::Result<report::Report> {
		let fun = self.resolve_fn(addr).await;
		let ln = self.resolve_ln(addr).await;

		// merge ln -> fn:
		match fun {
			Ok(mut fun) => {
				match ln {
					Ok(mut ln) => {
						let mut to_add = Vec::new();
						for b in ln.symbols.drain(..) {
							let mut exists = false;
							'fun: for a in fun.symbols.iter() {
								if a.lines == b.lines {
									exists = true;
									break 'fun;
								}
							}
							if !exists {
								to_add.push(b);
							}
						}
						fun.symbols.extend(to_add);
					},
					Err(err) => error!("{err}"),
				}
				Ok(fun)
			},
			Err(err_fn) => {
				match ln {
					Err(err_ln) => Err(err_fn.context(err_ln)),
					ok => ok,
				}
			},
		}
	}

	pub async fn resolve_fn(&self, addr: u32) -> anyhow::Result<report::Report> {
		struct Record {
			name: Option<String>,
			low: Option<i64>,
			size: Option<i64>,
			fn_hw_id: Option<i64>,
			ln_low: Option<i64>,
			ln_hw_id: Option<i64>,
			lineno: Option<i64>,
			path: Option<String>,
		}

		let mut stmt = self.conn.prepare(QUERY_FN)?;
		let records = stmt.query_map([addr], |row| {
			                  Ok(Record { name: row.get(0)?,
			                              low: row.get(1)?,
			                              size: row.get(2)?,
			                              fn_hw_id: row.get(3)?,
			                              ln_low: row.get(4)?,
			                              ln_hw_id: row.get(4)?,
			                              lineno: row.get(6)?,
			                              path: row.get(7)? })
		                  })?;

		let mut results = HashMap::new();

		for record in records {
			let func = record?;
			let fn_name = func.name.as_deref().map(Cow::from).unwrap_or(UNKNOWN.into());
			let fn_id = format_args!("{fn_name}{}", func.fn_hw_id.unwrap_or_default()).to_string();
			let name = Name::new(fn_name.to_string(), NameMangling::Unmangled, Language::C);

			let line = report::Span { hw_id: func.ln_hw_id,
			                          address: func.ln_low.map(|p| p as _).unwrap_or_default(),
			                          size: None,
			                          line: func.lineno.map(|p| p as _),
			                          file: func.path.unwrap_or(UNKNOWN.into()).into() };

			if let Some(item) = results.get_mut(&fn_id) {
				let report::Symbol { lines, .. } = item;
				lines.push(line);
				lines.sort();
			} else {
				let res = report::Symbol { hw_id: func.fn_hw_id,
				                           name: Some(name),
				                           address: func.low.map(|p| p as _).unwrap_or_default(),
				                           size: func.size.map(|p| p as _),
				                           lines: vec![line] };
				results.insert(fn_id, res);
			}
		}

		let result = report::Report { addr: (addr as u64).into(),
		                              symbols: results.into_values().collect() };

		Ok(result)
	}


	pub async fn resolve_ln(&self, addr: u32) -> anyhow::Result<report::Report> {
		struct Record {
			low: Option<i64>,
			hw_id: Option<i64>,
			lineno: Option<i64>,
			path: Option<String>,
		}

		let mut stmt = self.conn.prepare(QUERY_LN)?;
		let records = stmt.query_map([addr], |row| {
			                  Ok(Record { low: row.get(0)?,
			                              hw_id: row.get(1)?,
			                              lineno: row.get(2)?,
			                              path: row.get(3)? })
		                  })?;

		let mut lines = records.into_iter()
		                       .try_fold(Vec::new(), |mut acc, res| -> rusqlite::Result<_> {
			                       let ln = res?;
			                       let span = report::Span { hw_id: ln.hw_id,
			                                                 address: ln.low.map(|p| p as _).unwrap_or_default(),
			                                                 size: None,
			                                                 line: ln.lineno.map(|p| p as _),
			                                                 file: ln.path.unwrap_or(UNKNOWN.into()).into() };
			                       acc.push(span);
			                       Ok(acc)
		                       })?;

		let result = if lines.is_empty() {
			report::Report { addr: (addr as u64).into(),
			                 symbols: vec![] }
		} else {
			let exact = lines.iter()
			                 .enumerate()
			                 .find_map(|(i, ln)| (ln.address == addr as u64).then_some(i));

			let mut lines = if let Some(exact) = exact {
				vec![lines.remove(exact)]
			} else {
				lines
			};
			lines.sort();

			let sym = report::Symbol { hw_id: None,
			                           name: None,
			                           address: addr as _,
			                           size: None,
			                           lines };
			report::Report { addr: (addr as u64).into(),
			                 symbols: vec![sym] }
		};

		Ok(result)
	}
}


pub struct SystemDbResult<'t> {
	pub hw_id: i64,

	pub name: Name<'t>,
	pub low: i64,
	pub size: i64,
	pub lines: Vec<SystemDbSpan>,
}

pub struct SystemDbSpan {
	pub hw_id: i64,
	pub low: i64,
	pub number: i64,
	pub file: PathBuf,
}
