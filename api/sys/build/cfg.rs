use bindgen_cfg::Cfg;


pub fn create() -> Cfg {
	let mut cfg = Cfg::default();
	cfg.derive.default = true;
	cfg.derive.eq = false;
	cfg.derive.copy = true;
	cfg.derive.debug = true;
	cfg.derive.hash = true;
	cfg.derive.ord = false;
	cfg.derive.partialeq = true;
	cfg.derive.partialord = true;
	cfg.derive.constparamty = true;
	cfg.features.documentation = cfg!(feature = "bindings-documentation");
	cfg.features.nice = true;
	cfg
}
