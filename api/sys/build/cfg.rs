use std::path::Path;
use bindgen_cfg::Cfg;


pub fn default() -> Cfg {
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
	cfg.features.patch = true;

	let root = {
		let parent = Path::new(file!()).parent()
		                               .and_then(|parent| parent.file_name())
		                               .expect("src build dir path");
		Path::new(env!("CARGO_MANIFEST_DIR")).join(parent)
	};
	let patch = root.join("patch.yml");
	let rename = root.join("rename.yml");
	println!("cargo::rerun-if-changed={}", patch.display());
	println!("cargo::rerun-if-changed={}", rename.display());
	cfg.patch = Some(patch);
	cfg.rename = Some(rename);

	cfg
}
