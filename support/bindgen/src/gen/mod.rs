#![cfg(feature = "extra-codegen")]
use std::io::Write;
use std::path::Path;
use utils::toolchain::sdk::Sdk;
use quote::ToTokens;
use proc_macro2::TokenStream;

use crate::Result;

pub mod docs;


#[allow(unused_variables)]
pub fn engage(source: &bindgen::Bindings,
              features: &crate::cfg::Features,
              sdk: &Sdk,
              root: Option<&str>)
              -> Result<Bindings> {
	let root_struct_name = root.unwrap_or("PlaydateAPI");

	#[allow(unused_mut)]
	let mut bindings = syn::parse_file(&source.to_string())?;

	#[allow(unused_assignments)]
	#[cfg(feature = "documentation")]
	let docset = if features.documentation {
		let docset_new = docs::parser::parse(sdk)?;
		docs::gen::engage(&mut bindings, &root_struct_name, &docset_new)?;
		Some(docset_new)
	} else {
		None
	};

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
				println!("cargo:warning=Rustfmt error: {err}");

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
