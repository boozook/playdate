use utils::toolchain::gcc::ArmToolchain;
use utils::toolchain::sdk::Sdk;


pub struct Config {
	pub sdk: Option<Sdk>,
	pub gcc: Option<ArmToolchain>,

	pub derive: Derive,
}

impl Default for Config {
	fn default() -> Self {
		Self { sdk: None,
		       gcc: None,
		       derive: Default::default() }
	}
}


#[derive(Debug, Default, Clone, Copy)]
pub struct Derive {
	pub default: bool,
	pub eq: bool,
	pub copy: bool,
	pub debug: bool,
	pub hash: bool,
	pub ord: bool,
	pub partialeq: bool,
	pub partialord: bool,
}
