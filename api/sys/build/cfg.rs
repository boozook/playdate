use bindgen_cfg::Cfg;


pub fn create() -> Cfg {
	let mut cfg = Cfg::default();
	cfg.derive.default = feature_derive_default();
	cfg.derive.eq = feature_derive_eq();
	cfg.derive.copy = feature_derive_copy();
	cfg.derive.debug = feature_derive_debug();
	cfg.derive.hash = feature_derive_hash();
	cfg.derive.ord = feature_derive_ord();
	cfg.derive.partialeq = feature_derive_partialeq();
	cfg.derive.partialord = feature_derive_partialord();
	cfg.derive.constparamty = feature_derive_constparamty();
	cfg.features.documentation = feature_bindings_documentation();
	cfg.features.rustify = feature_bindings_rustify();
	cfg
}


pub const fn feature_derive_default() -> bool { cfg!(feature = "bindings-derive-default") }
pub const fn feature_derive_eq() -> bool { cfg!(feature = "bindings-derive-eq") }
pub const fn feature_derive_copy() -> bool { cfg!(feature = "bindings-derive-copy") }
pub const fn feature_derive_debug() -> bool { cfg!(feature = "bindings-derive-debug") }
pub const fn feature_derive_hash() -> bool { cfg!(feature = "bindings-derive-hash") }
pub const fn feature_derive_ord() -> bool { cfg!(feature = "bindings-derive-ord") }
pub const fn feature_derive_partialeq() -> bool { cfg!(feature = "bindings-derive-partialeq") }
pub const fn feature_derive_partialord() -> bool { cfg!(feature = "bindings-derive-partialord") }
pub const fn feature_derive_constparamty() -> bool { cfg!(feature = "bindings-derive-constparamty") }
pub const fn feature_bindings_documentation() -> bool { cfg!(feature = "bindings-documentation") }
pub const fn feature_bindings_rustify() -> bool { false }
