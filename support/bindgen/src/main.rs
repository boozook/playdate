use std::io::Write;
use std::io::stdout;

use clap::Parser;
use playdate_bindgen::*;
use utils::toolchain::sdk::Sdk;


fn main() {
	let mut args = Args::parse();

	if cfg!(debug_assertions) {
		eprintln!("{args:#?}")
	}

	match args.cmd {
		None => {
			assert!(args.cfg.output.is_some(), "Output file is not specified.");
			let output = args.cfg.output.take().expect("output path");

			// prepare generator:
			let generator = Generator::new(args.cfg).expect("Couldn't create bindings generator.");

			// generate bindings:
			let bindings = generator.generate().expect("Couldn't generate bindings.");
			bindings.write_to_file(&output).expect("Couldn't write bindings.");
		},

		Some(Cmd::FindSDKVersion) => {
			let result = args.cfg
			                 .sdk
			                 .as_ref()
			                 .map(|p| Sdk::try_new_exact(p).or_else(|_| Sdk::try_new()))
			                 .unwrap_or_else(|| Sdk::try_new())
			                 .and_then(|sdk| sdk.read_version());

			match result {
				Ok(version) => {
					let mut out = stdout();
					let exit = out.write_all(version.as_bytes())
					              .and_then(|_| out.flush())
					              .map(|_| 0)
					              .map_err(|err| {
						              eprintln!("{err}");
						              err
					              })
					              .unwrap_or(1);
					std::process::exit(exit);
				},
				Err(err) => {
					eprintln!("{err}");
					std::process::exit(1);
				},
			}
		},
	}
}


/// Playdate-bindgen configuration.
#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about, name = bindgen_cfg::BIN_NAME, verbatim_doc_comment)]
pub struct Args {
	pub cmd: Option<Cmd>,

	#[command(flatten)]
	pub cfg: bindgen_cfg::Cfg,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Cmd {
	/// Determine version for the Playdate SDK.
	#[value(name = bindgen_cfg::FIND_SDK_VERSION_CMD)]
	FindSDKVersion,
}


#[cfg(test)]
mod tests {
	#[test]
	fn same_bin_name() {
		assert_eq!(env!("CARGO_BIN_NAME"), bindgen_cfg::BIN_NAME);
	}

	#[test]
	fn same_env_var() {
		assert_eq!(utils::consts::SDK_ENV_VAR, bindgen_cfg::Cfg::ENV_SDK_PATH);
	}
}
