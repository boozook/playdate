use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;


const CARGO_MANIFEST_FILENAME: &str = "Cargo.toml";


// TODO: use Cow's instead of strings
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Env {
	/// Cargo.toml by default
	pub cargo_manifest_filename: String,
	pub vars: BTreeMap<String, String>,
}


impl Env {
	pub fn cargo_out_dir(&self) -> &Path { Path::new(&self.vars["OUT_DIR"]) }
	// TODO: use CARGO_BIN_NAME -> CARGO_CRATE_NAME -> CARGO_PKG_NAME
	pub fn cargo_pkg_name(&self) -> &str { &self.vars["CARGO_PKG_NAME"] }
	pub fn cargo_manifest_dir(&self) -> &Path { Path::new(&self.vars["CARGO_MANIFEST_DIR"]) }

	/// Creates a new environment with values by real env by default.
	pub fn try_default() -> Result<Self, env::VarError> {
		Ok(Self { vars: env::vars().collect(),
		          cargo_manifest_filename: CARGO_MANIFEST_FILENAME.to_string() })
	}

	// TODO: Proper error for `Env::try_from_iter`
	pub fn try_from_iter<K, V>(iter: impl Iterator<Item = (K, V)>) -> Result<Self, &'static str>
		where K: ToString,
		      V: ToString {
		let mut env = BTreeMap::new();
		env.extend(iter.map(|(k, v)| (k.to_string(), v.to_string())));

		// validate:
		env.get("CARGO_PKG_NAME").ok_or("Missed env 'CARGO_PKG_NAME'")?;
		env.get("CARGO_MANIFEST_DIR")
		   .ok_or("Missed env 'CARGO_MANIFEST_DIR'")?;

		Ok(Self { vars: env,
		          cargo_manifest_filename: CARGO_MANIFEST_FILENAME.to_string() })
	}


	pub fn manifest_path(&self) -> PathBuf { self.cargo_manifest_dir().join(&self.cargo_manifest_filename) }
}
