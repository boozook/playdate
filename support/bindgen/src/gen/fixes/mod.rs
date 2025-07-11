use std::cell::Cell;
use std::collections::HashMap;
use bindgen_cfg::Target;
use syn::spanned::Spanned;
use syn::token;
use syn::Item;
use syn::Lifetime;
use syn::ReturnType;
use syn::Type;
use syn::ItemStruct;
use syn::TypeNever;
use syn::TypeReference;

use crate::Result;


pub type FixMap = HashMap<String, Fix>;

pub enum Fix {
	ReturnNever,
	Unwrap,
	Deref,
	// /// Remove field with simple ptr-type, replace with `*const void`.
	// RemovePtr,
}


pub fn engage(bindings: &mut syn::File, root: &str, _target: &Target, docs: &FixMap) -> Result<()> {
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
               fixes: &HashMap<String, Fix>) {
	let prefix = this.map(|s| format!("{s}.")).unwrap_or("".to_owned());
	for field in structure.fields.iter_mut() {
		let field_name = field.ident.as_ref().expect("field name");

		// remove/hide/make unuseful:
		{
			let key = format!("{prefix}{field_name}");
			if key == "system.vaFormatString" {
				field.ty = syn::parse_quote! { *const ::core::convert::Infallible };
				let attr: syn::Attribute = syn::parse_quote! { #[doc(hidden)] };
				field.attrs.push(attr);
				continue;
			}
		}

		match &mut field.ty {
			syn::Type::Ptr(entry) => {
				match entry.elem.as_mut() {
					syn::Type::Path(path) => {
						if let Some(ident) = path.path.get_ident() {
							if let Some(ty) = find_struct(items, &ident.to_string()) {
								let key = format!("{prefix}{field_name}");
								walk_struct(items, Some(&key), ty, fixes);
								apply_all(&key, field, fixes, None);
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
							apply_all(&key, field, fixes, Some(ty.to_owned()));
						},
						_ => unimplemented!("unexpected ty: '{}'", quote::quote!(#ty)),
					}
				} else {
					unimplemented!("unexpected ty: '{}'", quote::quote!(#&path))
				}
			},

			_ty => {
				#[cfg(feature = "log")]
				println!(
				         "unknown: {prefix}{}: {}",
				         field.ident.as_ref().expect("field.ident"),
				         quote::quote!(#_ty)
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


fn apply(_key: &str, field: &mut syn::Field, fix: &Fix, _underlying: Option<Type>) {
	match fix {
		Fix::ReturnNever => {
			if let Type::BareFn(ref mut ty) = &mut field.ty {
				ty.output =
					ReturnType::Type(
					                 token::RArrow(ty.output.span()),
					                 Box::new(TypeNever { bang_token: syn::Token![!](ty.output.span()), }.into()),
					);
			}
		},
		Fix::Unwrap => {
			// TODO: Fix::Unwrap
		},
		Fix::Deref => {
			// TODO: Fix::Deref
		},
	}
}


fn apply_all(key: &str, field: &mut syn::Field, fixes: &FixMap, ty: Option<Type>) {
	let ty = ty.as_ref()
	           .or_else(|| extract_type_from_option(&field.ty))
	           .unwrap_or(&field.ty);


	match ty {
		Type::BareFn(_) => {
			// apply default unwrap:
			const UNWRAP_EXCLUDE: &[&str] = &["graphics.getDebugBitmap", "system.vaFormatString"];
			if !UNWRAP_EXCLUDE.contains(&key) {
				field.ty = ty.to_owned();
			}
		},
		Type::Ptr(ty) => {
			// apply default deref:
			match ty.elem.as_ref() {
				Type::Path(path) => {
					let lifetime = Lifetime::new("'static", ty.star_token.span());
					field.ty = Type::Reference(TypeReference { and_token: token::And(ty.star_token.span()),
					                                           lifetime: Some(lifetime),
					                                           mutability: ty.mutability,
					                                           elem: Type::Path(path.to_owned()).into() })
				},
				_ => unimplemented!(),
			}
		},
		_ => unimplemented!(),
	}

	if let Some(fix) = fixes.get(key) {
		apply(key, field, fix, None);
	}
}
