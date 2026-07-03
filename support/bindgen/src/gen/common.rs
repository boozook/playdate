use core::cell::Cell;

use syn::{GenericArgument, Item, ItemStruct, PathArguments, PathSegment, Type, TypeBareFn};


pub fn extract_ty_from_opt(ty: &Type) -> Option<&Type> {
	extract_ty_from_opt_mut(unsafe { (ty as *const _ as *mut Type).as_mut_unchecked() }).map(|r| &*r)
}

// This is the really bad solution to extract the type from an Option<T>.
pub fn extract_ty_from_opt_mut(ty: &mut Type) -> Option<&mut Type> {
	extract_type_path(ty).and_then(|path| extract_option_segment(path))
	                     .and_then(|path_seg| {
		                     let type_params = &mut path_seg.arguments;
		                     match type_params {
			                     PathArguments::AngleBracketed(params) => params.args.first_mut(),
		                        _ => None,
		                     }
	                     })
	                     .and_then(|generic_arg| {
		                     match generic_arg {
			                     GenericArgument::Type(ty) => Some(ty),
		                        _ => None,
		                     }
	                     })
}

fn extract_type_path(ty: &mut Type) -> Option<&mut syn::Path> {
	match ty {
		Type::Path(tp) if tp.qself.is_none() => Some(&mut tp.path),
		_ => None,
	}
}

fn extract_option_segment(path: &mut syn::Path) -> Option<&mut PathSegment> {
	let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
		                                         acc.push_str(&v.ident.to_string());
		                                         acc.push('|');
		                                         acc
	                                         });
	let expected = ["Option|", "std|option|Option|", "core|option|Option|"];
	if expected.into_iter().any(|s| idents_of_path == *s) {
		path.segments.last_mut()
	} else {
		None
	}
}


#[allow(clippy::mut_from_ref)]
pub fn find_struct<'t>(items: &'t [Cell<Item>], name: &str) -> Option<&'t mut ItemStruct> {
	items.iter().find_map(|item| {
		            match unsafe { item.as_ptr().as_mut() }.expect("cell is null, impossible") {
			            syn::Item::Struct(entry) if entry.ident == name => Some(entry),
		               _ => None,
		            }
	            })
}


pub fn opt_ty_get_mut<'t>(ty: &'t mut Type) -> &'t mut Type {
	let p = ty as *mut Type;
	extract_ty_from_opt_mut(ty).unwrap_or_else(|| unsafe { p.as_mut().unwrap_unchecked() })
}

pub fn safe_opt_fn(ty: &mut Type) -> bool {
	if let Type::BareFn(TypeBareFn { unsafety, .. }) = opt_ty_get_mut(ty) {
		unsafety.take();
		true
	} else {
		false
	}
}
