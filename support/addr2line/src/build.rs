fn main() {
	#[cfg(query_validation)]
	{
		use std::path::PathBuf;
		let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("symbols.db");
		let root = root.canonicalize()
		               .map_err(|err| println!("cargo:warning={err:#}"))
		               .unwrap_or(root);

		let src = root.join("src");
		std::env::set_var("PD_SYM_QUERY_ROOT", src.as_os_str());

		if std::env::var_os("DATABASE_URL").is_none() {
			let url = format!("sqlite://{}", root.display());
			std::env::set_var("DATABASE_URL", &url);
			println!("env var DATABASE_URL has been set to '{url}'.");
			if !root.exists() {
				println!("cargo:warning=env var DATABASE_URL isn't set and db doesn't exist.");
			}
		}
	}
}
