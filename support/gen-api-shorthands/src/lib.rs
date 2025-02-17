#![feature(assert_matches)]
#![feature(proc_macro_expand)]

use std::assert_matches::assert_matches;

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, Token, Visibility};

#[derive(Parse)]
struct AttrValue {
	mod_vis: Visibility,
	mod_token: Token![mod],
	mod_name: Ident,
}

#[proc_macro_attribute]
pub fn gen_shorthands(attr: TokenStream, item: TokenStream) -> TokenStream {
	let source_impl: proc_macro2::TokenStream = item.clone().into();
	let (api, impl_items) = parse_api_impl(parse_macro_input!(item as ItemImpl));

	let shorthands = impl_items.into_iter()
		.map(parse_method)
		.map(|method| into_shorthand(&api, method))
		.collect::<Vec<_>>();

	let shorthands = if attr.is_empty() {
		quote! { #(#shorthands)* }
	} else {
		let AttrValue { mod_vis, mod_token, mod_name } = parse_macro_input!(attr as AttrValue);
		quote! {
			#mod_vis #mod_token #mod_name {
				use super::*;
				#(#shorthands)*
			}
		}
	};

	TokenStream::from(quote! {
		#source_impl
		#shorthands
	})
}

fn parse_api_impl(impl_: ItemImpl) -> (Ident, Vec<ImplItem>) {
	let api = match *impl_.self_ty {
		syn::Type::Path(syn::TypePath { qself: None, path }) => path
			.segments.last().expect("empty path")
			.ident.clone(),
		_ => panic!("only simple paths are supported"),
	};

	assert!(impl_.trait_.is_none(), "trait impls are not supported");

	return (api, impl_.items);
}

fn parse_method(item: ImplItem) -> ImplItemFn {
	let ImplItem::Fn(method) = item else { panic!("only methods are supported"); };

	assert!(method.defaultness.is_none(), "default methods are not supported");
	assert!(method.sig.constness.is_none(), "const methods are not supported");
	assert!(method.sig.asyncness.is_none(), "async methods are not supported");
	assert!(method.sig.unsafety.is_none(), "unsafe methods are not supported");
	assert!(method.sig.abi.is_none(), "extern methods are not supported");
	assert!(method.sig.variadic.is_none(), "variadic methods are not supported");
	assert_matches!(method.sig.inputs.first(), Some(FnArg::Receiver(_)), "only methods are supported");

	return method;
}

fn into_shorthand(api: &Ident, mut method: ImplItemFn) -> ImplItemFn {
	let method_name = method.sig.ident.clone();

	// Remove the receiver from the method signature
	method.sig.inputs = method.sig.inputs.into_iter().filter(|input| matches!(input, FnArg::Typed(_))).collect();


	// All shorthand functions are public
	method.vis = Visibility::Public(Token![pub](method.span()));


	// All shorthand functions should inline
	method.attrs.append({
		let shorthand_doc_msg = quote! { concat!(" This function is shorthand for [`", stringify!(#api), "::", stringify!(#method_name), "`], using default ZST end-point.") };
		let shorthand_doc_msg: proc_macro2::TokenStream = TokenStream::from(shorthand_doc_msg).expand_expr().unwrap().into();

		&mut vec![
			syn::parse_quote! { #[doc = ""] },
			syn::parse_quote! { #[doc = #shorthand_doc_msg] },
			syn::parse_quote! { #[inline(always)] },
		]
	});


	// Just call the method from the default end-point
	method.block = {
		let args = method.sig.inputs.iter().filter_map(|input| match input {
			FnArg::Receiver(_) => None,
			FnArg::Typed(pat) => Some(&pat.pat),
		}).collect::<Vec<_>>();

		syn::parse_quote! { { #api::Default().#method_name( #(#args),* ) } }
	};

	return method;
}
