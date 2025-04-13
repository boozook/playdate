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
	cfg.features.nice = feature_bindings_extra();
	cfg
}


pub const fn feature_derive_default() -> bool { true }
pub const fn feature_derive_eq() -> bool { true }
pub const fn feature_derive_copy() -> bool { true }
pub const fn feature_derive_debug() -> bool { true }
pub const fn feature_derive_hash() -> bool { true }
pub const fn feature_derive_ord() -> bool { true }
pub const fn feature_derive_partialeq() -> bool { true }
pub const fn feature_derive_partialord() -> bool { true }
pub const fn feature_derive_constparamty() -> bool { true }
pub const fn feature_bindings_documentation() -> bool { cfg!(feature = "bindings-documentation") }
pub const fn feature_bindings_extra() -> bool { true }
