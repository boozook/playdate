#![feature(assert_matches)]
#![feature(proc_macro_expand)]

use std::assert_matches::assert_matches;

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, Signature, Token, Visibility};

#[derive(Parse)]
struct AttrValue {
  mod_vis: Visibility,
  mod_token: Token![mod],
  mod_name: Ident,
}

#[proc_macro_attribute]
pub fn gen_shorthands_mod(attr: TokenStream, item: TokenStream) -> TokenStream {
  let source_impl: proc_macro2::TokenStream = item.clone().into();

  let AttrValue { mod_vis, mod_token, mod_name } = parse_macro_input!(attr as AttrValue);
  let ItemImpl { trait_, items, self_ty, .. } = parse_macro_input!(item as ItemImpl);

  let self_ty = match *self_ty {
    syn::Type::Path(syn::TypePath { qself: None, path }) => path
      .segments.last().expect("empty path")
      .ident.clone(),
    _ => panic!("only simple paths are supported"),
  };

  assert!(trait_.is_none(), "trait impls are not supported");

  let shorthands = items.into_iter().map(|item| {
    let ImplItem::Fn(ImplItemFn { attrs, defaultness, sig, vis: _, block: _ }) = item else {
      panic!("only methods are supported");
    };

    let Signature { constness, asyncness, unsafety, abi, fn_token: _, ident, generics, paren_token: _, inputs, variadic, output } = sig;
    let where_clause = &generics.where_clause;

    assert!(defaultness.is_none(), "default methods are not supported");
    assert!(constness.is_none(), "const methods are not supported");
    assert!(asyncness.is_none(), "async methods are not supported");
    assert!(unsafety.is_none(), "unsafe methods are not supported");
    assert!(abi.is_none(), "extern methods are not supported");
    assert!(variadic.is_none(), "variadic methods are not supported");

    assert_matches!(inputs.first(), Some(FnArg::Receiver(_)), "only methods are supported");

    let inputs = inputs.iter()
      .filter_map(|input| match input {
        FnArg::Receiver(_) => None,
        FnArg::Typed(arg) => Some(arg),
      })
      .collect::<Vec<_>>();

    let inputs_args = inputs.iter()
      .map(|pat| &pat.pat)
      .collect::<Vec<_>>();

    let shorthand_doc = quote! { concat!(" This function is shorthand for [`", stringify!(#self_ty), "::", stringify!(#ident), "`], using default ZST end-point.") };
    let shorthand_doc: proc_macro2::TokenStream = TokenStream::from(shorthand_doc).expand_expr().unwrap().into();

    quote! {
      #(#attrs)*
      #[doc = ""]
      #[doc = #shorthand_doc]
      #[inline(always)]
      pub fn #ident #generics ( #(#inputs),* ) #output #where_clause {
        #self_ty::Default().#ident( #(#inputs_args),* )
      }
    }
  }).collect::<Vec<_>>();

  TokenStream::from(quote! {
    #source_impl

    #mod_vis #mod_token #mod_name {
      use super::*;

      #(#shorthands)*
    }
  })
}
