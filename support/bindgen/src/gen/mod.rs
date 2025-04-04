#![cfg(feature = "extra-codegen")]
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use utils::toolchain::sdk::Sdk;
use quote::ToTokens;
use proc_macro2::TokenStream;

use crate::Result;
use crate::error::Error;
use crate::rustify::rename::{self, Kind, SharedRenamed};

pub mod docs;
pub mod fixes;


#[allow(unused_variables)]
pub fn engage(source: &bindgen::Bindings,
              renamed: SharedRenamed,
              features: &crate::cfg::Features,
              target: &crate::cfg::Target,
              sdk: &Sdk,
              root: Option<&str>)
              -> Result<Bindings> {
	if features.rustify {
		rename::reduce(Arc::clone(&renamed));
	}

	let root_struct_name = {
		let orig = root.as_ref().map(AsRef::as_ref).unwrap_or("PlaydateAPI");

		// find the renamed root:
		let key = Kind::Struct(orig.to_owned());
		renamed.read()
		       .map_err(|err| {
			       let s = Box::leak(Box::new(format!("renamed set is locked: {err}"))).as_str();
			       Error::Internal(s)
		       })?
		       .get(&key)
		       .map(ToOwned::to_owned)
		       .map(Cow::from)
		       .unwrap_or_else(|| Cow::from(orig))
	};


	// rename::print_as_md_table(Arc::clone(&renamed));


	#[allow(unused_mut)]
	let mut bindings = syn::parse_file(&source.to_string())?;

	#[allow(unused_assignments)]
	#[cfg(feature = "documentation")]
	let docset = if features.documentation {
		let docset = docs::parser::parse(sdk)?;
		docs::gen::engage(&mut bindings, &root_struct_name, &docset)?;
		Some(docset)
	} else {
		None
	};


	#[cfg(feature = "extra-codegen")]
	if features.rustify {
		// let fixes = if target.is_playdate() {
		let mut fixes = HashMap::new();
		fixes.insert("system.error".to_owned(), fixes::Fix::ReturnNever);
		// fixes
		// } else {
		// 	Default::default()
		// };
		fixes::engage(&mut bindings, &root_struct_name, target, &fixes)?;
	}

	let mut module = TokenStream::new();
	bindings.to_tokens(&mut module);

	Ok(Bindings { module,
	              #[cfg(feature = "documentation")]
	              docset })
}


/// Engaged bindings with doc-comments.
#[derive(Debug)]
pub struct Bindings {
	module: TokenStream,

	// for cache:
	#[cfg(feature = "documentation")]
	docset: Option<docs::DocsMap>,
	// TODO: minimal cfg from parent such as formatter & rustfmt-cfg.
}

impl crate::Bindings {
	#[cfg(feature = "documentation")]
	pub fn docset(&self) -> Option<&docs::DocsMap> {
		match self {
			crate::Bindings::Bindgen(_) => None,
			crate::Bindings::Engaged(this) => this.docset.as_ref(),
		}
	}
}

impl Bindings {
	/// Write these bindings as source text to a file.
	pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
		let file = std::fs::OpenOptions::new().write(true)
		                                      .truncate(true)
		                                      .create(true)
		                                      .open(path.as_ref())?;
		self.write(Box::new(file))?;
		Ok(())
	}

	/// Write these bindings as source text to the given `Write`able.
	pub fn write<'a>(&self, mut writer: Box<dyn Write + 'a>) -> std::io::Result<()> {
		// formatting:
		let source = self.module.to_string();
		let output = match crate::rustfmt(None, source.clone(), None) {
			Ok(output) => output,
			Err(err) => {
				println!("cargo::warning=Rustfmt error: {err}");

				let output: String;
				#[cfg(feature = "pretty-please")]
				{
					let tokens = &self.module;
					output = prettyplease::unparse(&syn::parse_quote!(#tokens));
				}
				#[cfg(not(feature = "prettyplease"))]
				{
					output = source;
				}
				output
			},
		};

		writer.write_all(output.as_bytes())?;
		Ok(())
	}
}


impl std::fmt::Display for Bindings {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut bytes = vec![];
		self.write(Box::new(&mut bytes) as Box<dyn Write>)
		    .expect("writing to a vec cannot fail");
		f.write_str(std::str::from_utf8(&bytes).expect("we should only write bindings that are valid utf-8"))
	}
}
