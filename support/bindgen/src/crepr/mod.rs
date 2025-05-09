use bindgen::BindgenError;
use bindgen::EnumVariation;
use bindgen::RustTarget;
use syn::File;
use syn::Ident;
use syn::Item;


/// Usage: `enum_repr("i686-pc-win32-msvc")`
pub fn enum_repr(target: &str) -> Option<(Ident, bool)> {
	let bindings = bindings_for_enum(target).ok()?;
	let source = bindings.to_string();
	let module = syn::parse_file(&source.to_string()).ok()?;
	repr_of("MyEnum", &module).map(|ident| {
		                          let signed = ident.to_string().starts_with('i');
		                          println!("REPR TY: {ident}, signed: {signed}");
		                          (ident, signed)
	                          })
}


pub fn bindings_for_enum(target: &str) -> Result<bindgen::Bindings, BindgenError> {
	const ENUM: &str = include_str!("enum.h");

	builder_for(ENUM).default_enum_style(EnumVariation::Rust { non_exhaustive: false })
	                 .layout_tests(true)
	                 .rust_target(RustTarget::nightly())
	                 .clang_arg(format!("--target={target}"))
	                 .generate()
}


pub fn builder_for(src: &str) -> bindgen::Builder {
	bindgen::builder().header_contents("test.h", src)
	                  .rust_target(RustTarget::nightly())
	                  .use_core()
	                  .ctypes_prefix("core::ffi")
	                  .size_t_is_usize(true)
	                  .translate_enum_integer_types(true)
	                  .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
	                  .layout_tests(true)
}


fn repr_of(name: &str, module: &File) -> Option<Ident> {
	module.items.iter().find_map(|item| {
		                   if let Item::Enum(item) = item {
			                   if item.ident == name {
				                   item.attrs.iter().find_map(|attr| {
					                                    if let syn::Meta::List(ref list) = attr.meta {
						                                    if let Ok(ident) = list.path.require_ident() {
							                                    if ident == "repr" {
								                                    syn::parse2::<Ident>(list.tokens.to_owned()).ok()
							                                    } else {
								                                    None
							                                    }
						                                    } else {
							                                    None
						                                    }
					                                    } else {
						                                    None
					                                    }
				                                    })
			                   } else {
				                   None
			                   }
		                   } else {
			                   None
		                   }
	                   })
}
