#![cfg(feature = "extra-codegen")]
use std::borrow::Cow;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use patch::PatchCfg;
use syn::spanned::Spanned;
use utils::toolchain::sdk::Sdk;
use quote::ToTokens;
use proc_macro2::TokenStream;

use crate::Result;
use crate::error::Error;
use crate::nice::rename::{self, Kind, SharedIdents};

pub mod docs;
pub mod patch;
pub mod common;


#[allow(unused_variables)]
pub fn engage(source: &bindgen::Bindings,
              renamed: SharedIdents,
              patches: PatchCfg,
              features: &crate::cfg::Features,
              target: &crate::cfg::Target,
              sdk: &Sdk,
              root: Option<&str>)
              -> Result<Bindings> {
	if features.patch {
		rename::reduce(Arc::clone(&renamed));
	}

	let root_struct_name = {
		let orig = root.as_deref().unwrap_or("PlaydateAPI");

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


	#[allow(unused_mut)]
	let mut bindings = syn::parse_file(&source.to_string())?;

	#[allow(unused_assignments)]
	#[cfg(feature = "documentation")]
	let docset = if features.documentation {
		let docset = docs::parser::parse(sdk)?;
		docs::r#gen::engage(&mut bindings, &root_struct_name, &docset)?;
		Some(docset)
	} else {
		None
	};


	#[cfg(feature = "extra-codegen")]
	if features.patch {
		let renamed = Arc::clone(&renamed);
		patch::engage(&mut bindings, &root_struct_name, target, patches, renamed)?;
	} else {
		panic!("features.patch is OFF: {features:#?}");
	}

	// add compat- module with original names:
	#[cfg(feature = "extra-codegen")]
	insert_compat_aliases(&mut bindings, renamed);

	#[cfg(all(not(feature = "extra-codegen"), feature = "syn"))]
	bindings.items.push(syn::parse_quote! { pub mod compat { /*! Original names of renamed types. */ pub use super::*; } });

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
		let output;
		#[cfg(feature = "pretty")]
		{
			let tokens = &self.module;
			output = prettyplease::unparse(&syn::parse_quote!(#tokens));
		}
		#[cfg(not(feature = "prettyplease"))]
		{
			output = self.module.to_string();
		}
		let output = match crate::rustfmt(None, output.as_str().into(), None) {
			Ok(output) => output,
			Err(err) => {
				println!("cargo::warning=rustfmt error: {err}");
				output.into()
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


#[cfg(feature = "extra-codegen")]
fn insert_compat_aliases(bindings: &mut syn::File, renamed: SharedIdents) {
	let mut items = TokenStream::new();
	let common_span = items.span();
	let idents = renamed.read().expect("ident-map set is locked");
	let var = idents.iter()
	                .filter_map(|(orig, new)| {
		                let old = orig.item_name();
		                (!matches!(orig, Kind::EnumVariant(..)) && new != old).then_some((old, new.as_str()))
	                })
	                .filter(|&(_, new)| {
		                // get rid off unexisting things:
		                bindings.items.iter().any(|existing| {
			                                     use syn::*;
			                                     if let Item::Const(ItemConst { ident, .. }) |
			                                     Item::Enum(ItemEnum { ident, .. }) |
			                                     Item::Mod(ItemMod { ident, .. }) |
			                                     Item::Static(ItemStatic { ident, .. }) |
			                                     Item::Struct(ItemStruct { ident, .. }) |
			                                     Item::Type(ItemType { ident, .. }) |
			                                     Item::Union(ItemUnion { ident, .. }) |
			                                     Item::Fn(ItemFn { sig: Signature { ident, .. },
			                                                              .. }) = existing
			                                     {
				                                     ident.eq(new)
			                                     } else {
				                                     false
			                                     }
		                                     })
	                })
	                .map(|(old, new)| (syn::Ident::new(old, common_span), syn::Ident::new(new, common_span)))
	                .map(|(old, new)| quote::quote!(pub use super::#new as #old;));
	items.extend(var);

	let module: syn::Item = syn::parse_quote! {
		pub mod compat { /*! Original names of renamed types. */ #items }
	};
	bindings.items.push(module);
}
