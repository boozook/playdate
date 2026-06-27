use bindgen::callbacks::AttributeInfo;
use bindgen::callbacks::ParseCallbacks;
use bindgen::callbacks::TypeKind;

use super::rename::SharedIdents;


/// Derives `ConstParamTy` to simple structs and enums.
#[derive(Debug)]
pub struct DeriveConstParamTy(SharedIdents);

impl DeriveConstParamTy {
	pub fn new(idents: SharedIdents) -> Self { Self(idents) }
}


impl ParseCallbacks for DeriveConstParamTy {
	fn add_attributes(&self, info: &AttributeInfo<'_>) -> Vec<String> {
		const TYPES: &[&str] = &["PDButtons", "FileOptions", "LCDPattern"];
		const ATTR: &str = r#"#[cfg_attr(feature="const-types", derive(::core::marker::ConstParamTy))]"#;

		match info.kind {
			TypeKind::Enum => vec![ATTR.to_string()],
			TypeKind::Struct if type_matches(info.name, info.kind, TYPES, &self.0) => {
				vec![ATTR.to_string()]
			},
			_ => vec![],
		}
	}
}


pub fn type_matches(name: &str, _kind: TypeKind, with: &[&str], names: &SharedIdents) -> bool {
	// matched as-is:
	with.contains(&name) || {
		use super::rename::Kind;
		names.read().expect("names is locked").iter().any(|(k, v)| {
			                    let vmatch = v == name;
			                    // matched orig key:
			                    matches!(k, Kind::Item(name) | Kind::Struct(name) if with.contains(&name.as_str())) &&
			                    vmatch
		                    })
	}
}
