use std::cell::Cell;
use std::collections::HashMap;
use syn::Item;
use syn::Type;
use syn::ItemStruct;

use crate::Result;
use super::DocsMap;


pub fn engage(bindings: &mut syn::File, root: &str, docs: &DocsMap) -> Result<()> {
	let items = Cell::from_mut(&mut bindings.items[..]);
	let items_cells = items.as_slice_of_cells();
	if let Some(root) = find_struct(items_cells, root) {
		walk_struct(items_cells, None, root, docs);
	}

	Ok(())
}


fn find_struct<'t>(items: &'t [Cell<Item>], name: &str) -> Option<&'t mut ItemStruct> {
	items.iter().find_map(|item| {
		            match unsafe { item.as_ptr().as_mut() }.expect("cell is null, impossible") {
			            syn::Item::Struct(entry) if entry.ident == name => Some(entry),
		               _ => None,
		            }
	            })
}


fn walk_struct(items: &[Cell<Item>],
               this: Option<&str>,
               structure: &mut ItemStruct,
               docs: &HashMap<String, String>) {
	let prefix = this.map(|s| format!("{s}.")).unwrap_or("".to_owned());
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
				} else if let Some(ty) = extract_type_from_option(&field.ty) {
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

			ty => {
				println!(
				         "unknown: {prefix}{}: {}",
				         field.ident.as_ref().expect("field.ident"),
				         quote::quote!(#ty)
				);
			},
		}
	}
}


// This is the really bad solution to extract the type from an Option<T>.
fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
	use syn::{GenericArgument, Path, PathArguments, PathSegment};

	fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
		match *ty {
			syn::Type::Path(ref tp) if tp.qself.is_none() => Some(&tp.path),
			_ => None,
		}
	}

	fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
		let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
			                                         acc.push_str(&v.ident.to_string());
			                                         acc.push('|');
			                                         acc
		                                         });
		vec!["Option|", "std|option|Option|", "core|option|Option|"].into_iter()
		                                                            .find(|s| idents_of_path == *s)
		                                                            .and_then(|_| path.segments.last())
	}

	extract_type_path(ty).and_then(|path| extract_option_segment(path))
	                     .and_then(|path_seg| {
		                     let type_params = &path_seg.arguments;
		                     match *type_params {
			                     PathArguments::AngleBracketed(ref params) => params.args.first(),
		                        _ => None,
		                     }
	                     })
	                     .and_then(|generic_arg| {
		                     match *generic_arg {
			                     GenericArgument::Type(ref ty) => Some(ty),
		                        _ => None,
		                     }
	                     })
}
