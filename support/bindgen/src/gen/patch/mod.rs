use std::borrow::Cow;
use std::cell::Cell;
use bindgen_cfg::Target;
use syn::TypeBareFn;
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
use crate::nice::rename::SharedIdents;
use super::common::*;


pub fn engage(bindings: &mut syn::File,
              root: &str,
              _target: &Target,
              cfg: PatchCfg,
              idents: SharedIdents)
              -> Result<()> {
	let items = Cell::from_mut(&mut bindings.items[..]);
	let items_cells = items.as_slice_of_cells();


	let root = find_struct(items_cells, root);
	// inner field starting from entry-point
	if let Some(root) = root {
		walk_struct(items_cells, None, root, &cfg.root);
	}

	// orphans, non-root
	let renamed = idents.read().expect("ident-map is locked");
	for item in items_cells {
		let item = unsafe { item.as_ptr().as_mut() }.expect("cell is null, impossible");
		match item {
			Item::Struct(item) => {
				let name = renamed.iter()
				                  .find_map(|(k, v)| item.ident.eq(v).then_some(k.item_name()))
				                  .map(ToOwned::to_owned)
				                  .unwrap_or_else(|| item.ident.to_string());


				for fix in cfg.safe_inner.iter().filter(|item| name.eq(&item.name)) {
					let field = item.fields
					                .iter_mut()
					                .find(|f| f.ident.as_ref().is_some_and(|id| id.eq(&fix.field)));
					if let Some(field) = field {
						let ty = opt_ty_get_mut(&mut field.ty);
						if let Type::BareFn(TypeBareFn { inputs, .. }) = ty {
							let arg = inputs.iter_mut()
							                .find(|arg| arg.name.as_ref().is_some_and(|(id, _)| id.eq(&fix.param)));
							if let Some(arg) = arg {
								safe_opt_fn(&mut arg.ty);
							}
						}
					}
				}
			},

			Item::Type(item) => {
				let name = renamed.iter()
				                  .find_map(|(k, v)| item.ident.eq(v).then_some(k.item_name()))
				                  .map(ToOwned::to_owned)
				                  .unwrap_or_else(|| item.ident.to_string());

				if cfg.safe.include.contains(&name) && !cfg.safe.exclude.contains(&name) {
					safe_opt_fn(item.ty.as_mut());
				}
			},

			_ => {},
		}
	}

	Ok(())
}


fn walk_struct(items: &[Cell<Item>], this: Option<&str>, structure: &mut ItemStruct, cfg: &RootPath) {
	let prefix = this.map(|s| Cow::from(format!("{s}."))).unwrap_or("".into());
	for field in structure.fields.iter_mut() {
		let field_name = field.ident.as_ref().expect("field name");

		// remove/hide/make unuseful:
		let key = format!("{prefix}{field_name}");
		if cfg.remove.contains(&key) {
			field.ty = syn::parse_quote! { *const ::core::convert::Infallible };
			let attr: syn::Attribute = syn::parse_quote! { #[doc(hidden)] };
			field.attrs.push(attr);
			continue;
		}

		match &mut field.ty {
			syn::Type::Ptr(entry) => {
				match entry.elem.as_mut() {
					syn::Type::Path(path) => {
						if let Some(ident) = path.path.get_ident() {
							if let Some(ty) = find_struct(items, &ident.to_string()) {
								walk_struct(items, Some(&key), ty, cfg);
								apply_all(&key, field, cfg, None);
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
						Type::BareFn(_) => apply_all(&key, field, cfg, Some(ty.to_owned())),
						_ => unimplemented!("unexpected ty: '{}'", quote::quote!(#ty)),
					}
				} else {
					unimplemented!("unexpected ty: '{}'", quote::quote!(#&path))
				}
			},

			_ty => {
				#[cfg(feature = "log")]
				println!("unknown: {key}: {}", quote::quote!(#_ty));
			},
		}
	}
}


fn apply_return_never(_key: &str, field: &mut syn::Field) {
	if let Type::BareFn(ty) = &mut field.ty {
		ty.output = ReturnType::Type(
		                             token::RArrow(ty.output.span()),
		                             Box::new(TypeNever { bang_token: syn::Token![!](ty.output.span()), }.into()),
		);
	}
}


fn apply_all(key: &str, field: &mut syn::Field, cfg: &RootPath, ty: Option<Type>) {
	let ty = ty.as_ref()
	           .or_else(|| extract_ty_from_opt(&field.ty))
	           .unwrap_or(&field.ty);

	match ty {
		Type::BareFn(_) => {
			// apply unwrap:
			if !cfg.unwrap.exclude.iter().any(|v| v.eq(key)) {
				field.ty = ty.to_owned();
			}
		},
		Type::Ptr(ty) => {
			// apply deref:
			match ty.elem.as_ref() {
				Type::Path(path) => {
					if !cfg.deref.exclude.iter().any(|v| v.eq(key)) {
						let lifetime = Lifetime::new("'static", ty.star_token.span());
						field.ty = Type::Reference(TypeReference { and_token: token::And(ty.star_token.span()),
						                                           lifetime: Some(lifetime),
						                                           mutability: ty.mutability,
						                                           elem: Type::Path(path.to_owned()).into() })
					}
				},
				_ => unimplemented!(),
			}
		},
		_ => unimplemented!(),
	}

	if cfg.never.iter().any(|v| v.eq(key)) {
		apply_return_never(key, field);
	}
}


#[derive(Debug, Default, serde::Deserialize)]
pub struct PatchCfg {
	#[serde(default = "Default::default")]
	safe: Paths,
	#[serde(default = "Default::default")]
	#[serde(alias = "safe-param", rename = "safe-param")]
	safe_inner: Vec<ParamPath>,
	root: RootPath,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct ParamPath {
	/// Struct ident
	#[serde(alias = "struct", rename = "struct")]
	name: String,
	/// Field ident
	field: String,
	/// param ident
	param: String,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct RootPath {
	#[serde(default = "Default::default")]
	#[serde(alias = "never-return", rename = "never-return")]
	never: Vec<String>,
	#[serde(default = "Default::default")]
	remove: Vec<String>,
	#[serde(default = "Default::default")]
	unwrap: Paths,
	#[serde(default = "Default::default")]
	deref: Paths,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Paths {
	#[serde(default = "Default::default")]
	include: Vec<String>,
	#[serde(default = "Default::default")]
	exclude: Vec<String>,
}
