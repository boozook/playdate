use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;
use syn::Item;
use syn::Type;
use syn::ItemStruct;

use crate::Result;
use super::DocsMap;
use super::super::common::*;


pub fn engage(bindings: &mut syn::File, root: &str, docs: &DocsMap) -> Result<()> {
	let items = Cell::from_mut(&mut bindings.items[..]);
	let items_cells = items.as_slice_of_cells();
	if let Some(root) = find_struct(items_cells, root) {
		walk_struct(items_cells, None, root, docs);
	}

	Ok(())
}


fn walk_struct(items: &[Cell<Item>],
               this: Option<&str>,
               structure: &mut ItemStruct,
               docs: &HashMap<String, String>) {
	let prefix = this.map(|s| Cow::from(format!("{s}."))).unwrap_or("".into());
	for field in structure.fields.iter_mut() {
		let field_name = field.ident.as_ref().expect("field name");

		match &mut field.ty {
			syn::Type::Ptr(entry) => {
				match entry.elem.as_mut() {
					syn::Type::Path(path) => {
						if let Some(ident) = path.path.get_ident() {
							if let Some(ty) = find_struct(items, &ident.to_string()) {
								let key = format!("{prefix}{field_name}");
								walk_struct(items, Some(&key), ty, docs);
							}
						}
					},
					_ => unimplemented!("unknown ty: {}", quote::quote!(#{{field.ty}})),
				}
			},

			syn::Type::Path(path) => {
				if let Some(ident) = path.path.get_ident() {
					unimplemented!("unexpected struct: '{}'", quote::quote!(#ident))
				} else if let Some(ty) = extract_ty_from_opt(&field.ty) {
					match ty {
						Type::BareFn(_) => {
							let key = format!("{prefix}{field_name}");
							if let Some(doc) = docs.get(&key) {
								let attr: syn::Attribute = syn::parse_quote! { #[doc = #doc] };
								field.attrs.push(attr);
							} else {
								#[cfg(feature = "log")]
								println!("cargo::warning=Doc not found for '{key}'");
							}
						},
						_ => unimplemented!("unexpected ty: '{}'", quote::quote!(#ty)),
					}
				} else {
					unimplemented!("unexpected ty: '{}'", quote::quote!(#&path))
				}
			},

			_ty => {
				#[cfg(feature = "log")]
				println!("unknown: {prefix}{field_name}: {}", quote::quote!(#_ty));
			},
		}
	}
}
