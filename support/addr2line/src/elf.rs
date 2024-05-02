use std::borrow::Cow;
use std::path::Path;

use symbolic::common::{ByteView, Language, Name, NameMangling};
use symbolic::debuginfo::{Function, LineInfo, Object, ObjectError, Symbol};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::Stream;
use anyhow::{Context, Result};

use crate::fmt::addr::Addr;
use crate::fmt::addr::DEF;
use crate::fmt::report::{self, Report};


pub struct Resolver<'elf> {
	data: ByteView<'elf>,
	functions: bool,
	inlinees: bool,
}

impl<'t> Resolver<'t> {
	pub fn new(path: &Path, functions: bool, inlinees: bool) -> Result<Self> {
		let data = ByteView::open(path).context("failed to open file")?;
		Ok(Self { data,
		          functions,
		          inlinees })
	}


	// TODO: Methods with other receivers.
	// `pub fn into_stream_lazy_with(self, mut rx: tokio::sync::broadcast::Receiver<u32>)...`
	// For example:
	// ```
	// let (tx, rx) = tokio::sync::broadcast::channel(16);
	// let resolver = Box::pin(resolver.into_stream_lazy_with(rx.clone())?);
	//
	// let rx2 = tx.subscribe();
	// let resolver2 = SomeAnotherResolver::new(rx2);
	//
	// // Send addresses to both resolvers:
	// let send_addrs = tokio::spawn(async move {
	// 	for addr in addrs {
	// 		rx.send(addr)?;
	// 	}
	// 	Ok::<_, tokio::sync::mpsc::error::SendError<u32>>(())
	// });
	//
	// // Receive reports from resolver:
	// while let Some(res) = resolver.next().await {
	// 	res?.default_print(std::io::stdout(), ...)?;
	// }
	// send_addrs.await??;
	// ```

	/// Example:
	/// ```ignore
	/// let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
	/// let resolver = Box::pin(resolver.into_stream_lazy_from(rx, None::<[_;0]>)?);
	///
	/// // Send addresses to resolver:
	/// let send_addrs = tokio::spawn(async move {
	/// 	for addr in addrs {
	/// 		tx.send(addr)?;
	/// 	}
	/// 	Ok::<_, tokio::sync::mpsc::error::SendError<u32>>(())
	/// });
	///
	/// // Receive reports from resolver:
	/// while let Some(res) = resolver.next().await {
	/// 	res?.default_print(
	/// 	                   std::io::stdout(),
	/// 	                   0,
	/// 	                   cfg.flags.functions,
	/// 	                   cfg.flags.basenames,
	/// 	                   cfg.flags.ranges,
	/// 	                   cfg.flags.demangle,
	/// 	)?;
	/// }
	/// send_addrs.await??;
	/// ```
	pub fn into_stream_lazy_from<'a: 't, T: 'a + IntoIterator<Item = Addr<DEF>>>(
		self,
		mut rx: UnboundedReceiver<Addr>,
		addrs: Option<T>)
		-> Result<impl Stream<Item = Result<Report, ObjectError>> + 't> {
		let stream = async_stream::try_stream! {
			let data = self.data;
			let object = Object::parse(&data)?;
			let session = object.debug_session()?;
			let symbols = object.symbol_map();

			let load_address = object.load_address();
			debug!("Elf's load-address: {load_address}");

			let doit = |mut addr: Addr| {
				trace!(" Resolving {addr:#08x?}");
				{
					// use crate::fmt::addr::FLASH_MEM_REV1;
					use crate::fmt::addr::USER_HEAP_REV1;
					if USER_HEAP_REV1.contains(&addr.value()) && USER_HEAP_REV1.start >= load_address  {
						debug!("Seems to {addr:#08x?} is in the user-mem.");
						let diff = USER_HEAP_REV1.start - load_address;
						if addr.value() >= diff {
							addr.set_fixed(addr.value() - diff);
							trace!(" Now resolving {addr:#08x?}");
						}
					}
				}

				let mut result = None;
				'funcs: for function in session.functions() {
					match function.context("failed to read function") {
						Ok(function) => {
							let res = if self.inlinees {
								resolve_with_inlinees(&function, addr).filter(|v| !v.is_empty())
							} else {
								resolve(&function, addr).map(|res| vec![res])
							};

							if let Some(resolved) = res {
								debug_assert!(!resolved.is_empty());
								let rep = report::Report::from_vec(addr, resolved);
								trace!("Result of resolving fn: {addr:#08x?}, report addr: {:#08x?}", rep.addr);
								result = Some(rep);
								break 'funcs;
							}
						},
						Err(err) => error!("{err}"),
					}
				}

				if let Some(res) = result {
					res
				} else if self.functions {
					trace!("Not found fn for {addr:#08x?}, symbols lookup...");
					let resolved = symbols.lookup(addr.fixed()).or_else(||symbols.lookup(addr.value())).map(ResolvedAddrRef::Sym);

					if let Some(resolved) = resolved {
						trace!("Found sym for {addr:#08x?}");
						report::Report::from_one(addr, resolved)
					} else {
						trace!("Not found sym for {addr:#08x?}");
						report::Report{ addr, symbols: vec![] }
					}
				} else {
					trace!("Not found fn for {addr:#08x?}");
					report::Report{ addr: addr, symbols: vec![] }
				}
			};

			if let Some(addrs) = addrs {
				for addr in addrs {
					yield doit(addr);
				}
			}

			while let Some(addr) = rx.recv().await {
				yield doit(addr);
			}
		};

		Ok(stream)
	}


	/// Example:
	/// ```ignore
	/// let resolver = Box::pin(resolver.into_stream_resolve(rx, &addrs)?);
	///
	/// // Receive reports from resolver:
	/// while let Some(res) = resolver.next().await {
	/// 	res?.default_print(
	/// 	                   std::io::stdout(),
	/// 	                   0,
	/// 	                   cfg.flags.functions,
	/// 	                   cfg.flags.basenames,
	/// 	                   cfg.flags.ranges,
	/// 	                   cfg.flags.demangle,
	/// 	)?;
	/// }
	/// ```
	pub fn into_stream_resolve<'a: 't, T>(self,
	                                      addrs: &'a [T])
	                                      -> Result<impl Stream<Item = Result<Report, ObjectError>> + 't>
		where T: Copy + Into<Addr<u64>>
	{
		let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
		drop(tx);
		self.into_stream_lazy_from(
		                           rx,
		                           Some(addrs.into_iter().map(|v| (*v).into()).collect::<Vec<_>>()),
		)
	}


	pub fn into_stream_lazy<'a: 't, T: 'a + IntoIterator<Item = Addr<DEF>>>(
		self,
		addrs: Option<T>)
		-> Result<(UnboundedSender<Addr>, impl Stream<Item = Result<Report, ObjectError>> + 't)> {
		let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Addr>();
		self.into_stream_lazy_from(rx, addrs).map(|stream| (tx, stream))
	}


	// TODO: Methods with rev- iteration order. Also index.
	// 1. Create index <== iterate over `session.functions()`
	// 2. Iterate over addresses, search in index
	// 3. Search in symbols-map as fallback.
}


pub async fn map_result_fallback_os(res: Result<Report, ObjectError>,
                                    db: &crate::db::Resolver)
                                    -> Result<Report, anyhow::Error> {
	match res {
		Ok(rep) => {
			if rep.symbols.is_empty() {
				db.resolve(rep.addr.value() as _).await
			} else {
				Ok(rep)
			}
		},
		Err(err) => Err(err.into()),
	}
}


pub fn resolve_with_inlinees<'t, 'elf>(f: &'t Function<'elf>,
                                       addr: Addr)
                                       -> Option<Vec<ResolvedAddrRef<'t, 'elf>>> {
	if f.address > addr || f.address + f.size <= addr {
		return None;
	}

	let main = resolve(f, addr)?;
	let mut results = Vec::with_capacity(3); // usually 3 is statistically normal-high inline-chain len, so enough.

	for il in &f.inlinees {
		if let Some(mut res) = resolve_with_inlinees(il, addr) {
			results.append(&mut res);
		}
	}

	results.push(main);
	Some(results)
}

pub fn resolve<'t, 'elf>(f: &'t Function<'elf>, addr: Addr) -> Option<ResolvedAddrRef<'t, 'elf>> {
	if f.address > addr || f.address + f.size <= addr {
		return None;
	}

	let mut indices = Vec::new();

	for (i, line) in f.lines.iter().enumerate() {
		if line.address + line.size.unwrap_or(1) <= addr {
			// not yet there
			continue;
		} else if line.address > addr {
			// already not there
			break;
		}
		trace!("{addr:#08x}: found line {i}: ({})", line.file.name_str());
		indices.push(i);
	}

	let result = ResolvedAddrRef::Fn { function: f,
	                                   lines: indices };
	Some(result)
}


pub enum ResolvedAddrRef<'t, 'elf> {
	Fn {
		function: &'t Function<'elf>,
		/// Line indices
		lines: Vec<usize>,
	},
	Sym(&'t Symbol<'elf>),
}

#[derive(Debug)]
pub enum ResolvedAddr<'elf> {
	Fn {
		function: Function<'elf>,
		/// Line indices
		lines: Vec<usize>,
	},
	Sym(&'elf Symbol<'elf>),
}

impl<'t> From<ResolvedAddrRef<'t, 't>> for ResolvedAddr<'t> {
	fn from(value: ResolvedAddrRef<'t, 't>) -> Self {
		match value {
			ResolvedAddrRef::Fn { function, lines } => {
				Self::Fn { function: function.to_owned(),
				           lines }
			},
			ResolvedAddrRef::Sym(sym) => Self::Sym(sym),
		}
	}
}


impl Report {
	pub fn from_vec<T: Into<report::Symbol>>(addr: Addr, vec: Vec<T>) -> Self {
		Self { addr,
		       symbols: vec.into_iter().map(Into::into).collect() }
	}
	pub fn from_slice<T: Into<report::Symbol>>(addr: Addr, slice: &[T]) -> Self
		where for<'t> &'t T: Into<report::Symbol> {
		Self { addr,
		       symbols: slice.into_iter().map(Into::into).collect() }
	}
	pub fn from_one<T: Into<report::Symbol>>(addr: Addr, result: T) -> Self {
		Self { addr,
		       symbols: vec![result.into()] }
	}
}


impl Into<report::Symbol> for ResolvedAddrRef<'_, '_> {
	fn into(self) -> report::Symbol {
		let owned: ResolvedAddr = self.into();
		owned.into()
	}
}
impl Into<report::Symbol> for ResolvedAddr<'_> {
	fn into(self) -> report::Symbol { (&self).into() }
}

impl Into<report::Symbol> for &ResolvedAddr<'_> {
	fn into(self) -> report::Symbol {
		match self {
			ResolvedAddr::Fn { function, lines } => {
				let name = Name::new(
				                     function.name.as_str().to_string(),
				                     function.name.mangling(),
				                     function.name.language(),
				);
				let lines = lines.into_iter()
				                 .map(|l| &function.lines[*l])
				                 .map(Into::into)
				                 .collect::<Vec<_>>();
				report::Symbol { hw_id: None,
				                 name: Some(name),
				                 address: function.address,
				                 size: Some(function.size),
				                 lines }
			},

			ResolvedAddr::Sym(sym) => {
				let name = sym.name
				              .as_ref()
				              .map(|s| Cow::<'static, str>::Owned(s.to_string()))
				              .map(|name| Name::new(name, NameMangling::Unknown, Language::Rust));
				let size = (sym.size != 0).then_some(sym.size);
				report::Symbol { hw_id: None,
				                 name,
				                 address: sym.address,
				                 size,
				                 lines: Vec::with_capacity(0) }
			},
		}
	}
}


impl<'elf> Into<report::Span> for &LineInfo<'elf> {
	fn into(self) -> report::Span {
		let line = (self.line != 0).then_some(self.line);
		report::Span { hw_id: None,
		               address: self.address,
		               size: self.size,
		               file: self.file.path_str().into(),
		               line }
	}
}
