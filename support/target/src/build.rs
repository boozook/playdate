fn main() {
	#[cfg(any(feature = "spec", all(feature = "serde_json", feature = "toml")))]
	spec::main();
}


#[path = "built.rs"]
mod consts;

#[cfg(any(feature = "spec", all(feature = "serde_json", feature = "toml")))]
mod spec {
	use std::env::var_os;
	use std::error::Error;
	use std::fmt::Display;
	use std::fs::File;
	use std::io::Write;
	use std::path::PathBuf;
	use super::consts;


	static SPEC: &str = include_str!("spec/thumbv7em-panic-playdate-eabihf.toml");
	static LD: &str = include_str!("spec/link-map.ld");


	type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;


	pub fn main() {
		const ENV_JSON: &str = "cargo::rustc-env=PD_SPEC_PATH";
		const ENV_TOML: &str = "cargo::rustc-env=PD_SPECSRC_PATH";
		const ENV_TOML_PRETTY: &str = "cargo::rustc-env=PD_SPECSRC_PRETTY_PATH";

		println!("cargo::rerun-if-changed=spec/thumbv7em-panic-playdate-eabihf.toml");
		println!("cargo::rerun-if-changed=spec/link-map.ld");

		let out_dir = var_os("OUT_DIR").ok_or(std::env::VarError::NotPresent)
		                               .ok_or_tell_cargo()
		                               .map(PathBuf::from);
		let out_dir = out_dir.and_then(|out| {
			                     let p = out.join(consts::FILENAME);
			                     File::create(&p).ok_or_tell_cargo()
			                                     .and_then(|f| write_json(f, false).ok_or_tell_cargo())
			                                     .and_then(|_| Some(println!("{ENV_JSON}={}", p.display())))
			                                     .map(|_| out)
		                     })
		                     .and_then(|out| {
			                     let mut p = out.join(consts::FILENAME);
			                     p.set_extension("toml");
			                     File::create(&p).ok_or_tell_cargo()
			                                     .and_then(|f| write_toml(f, false).ok_or_tell_cargo())
			                                     .and_then(|_| Some(println!("{ENV_TOML}={}", p.display())))
			                                     .map(|_| out)
		                     });

		#[cfg(feature = "pretty")]
		out_dir.and_then(|out| {
			       let mut p = out.join(consts::FILENAME);
			       p.set_extension("pretty.toml");
			       File::create(&p).ok_or_tell_cargo()
			                       .and_then(|f| write_toml_pretty(f).ok_or_tell_cargo())
			                       .and_then(|_| Some(println!("{ENV_TOML_PRETTY}={}", p.display())))
		       });
	}


	mod build;


	pub fn build_toml() -> Result<toml::Value> { build::build_toml(SPEC, Some(LD)) }

	/// Build using toml-edit
	#[cfg(feature = "pretty")]
	pub fn build_toml_pretty() -> Result<toml_edit::DocumentMut> { build::build_toml_edit(SPEC, Some(LD)) }


	pub fn write_json(to: impl Write, pretty: bool) -> Result<()> {
		build::write_json_from_toml(SPEC, Some(LD), to, pretty)
	}

	pub fn write_toml(mut to: impl Write, pretty: bool) -> Result<()> {
		let spec = build_toml()?;

		let s = if pretty {
			        toml::to_string_pretty(&spec)
		        } else {
			        toml::to_string(&spec)
		        }.map(fix_esc_for_multiline_str)?;

		to.write_all(s.as_bytes()).map_err(Into::into)
	}

	#[cfg(feature = "pretty")]
	pub fn write_toml_pretty(mut to: impl Write) -> Result<()> {
		// also get and write path to schema:
		{
			let mut rustc = {
				let mut cmd = std::process::Command::new("rustc");
				if std::env::var_os("RUSTUP_HOME").is_some() {
					cmd.arg("+nightly");
				}
				cmd.args(["--print", "sysroot"]);
				cmd
			};
			let sysroot: Result<_> = rustc.output()
			                              .map_err(Box::from)
			                              .and_then(|out| out.status.success().then_some(out).ok_or_else(|| "".into()))
			                              .and_then(|out| String::from_utf8(out.stdout).map_err(Box::from));
			let sh_rel = PathBuf::from("etc/target-spec-json-schema.json");
			let schema = sysroot.as_ref()
			                    .map(|sysroot| PathBuf::from(sysroot.trim()).join(sh_rel));
			if let Ok(schema) = schema {
				let bang = format!("#:schema file://{}\n", schema.display());
				to.write_all(bang.as_bytes()).ok();
			}
		}

		// export:
		let spec = fix_esc_for_multiline_str(build_toml_pretty()?.to_string());
		to.write_all(spec.as_bytes()).map_err(Into::into)
	}

	/// fix issue https://github.com/blinxen/tomli/issues/7
	fn fix_esc_for_multiline_str(src: impl AsRef<str>) -> String { src.as_ref().replace("\\t", "\t") }


	trait TellCargo<Ok, Err: Display> {
		fn err_tell_cargo(self) -> Self;
		fn ok_or_tell_cargo(self) -> Option<Ok>;
	}

	impl<Ok, Err: Display> TellCargo<Ok, Err> for std::result::Result<Ok, Err> {
		fn err_tell_cargo(self) -> Self { self.inspect_err(|err| println!("cargo::error={err}")) }
		fn ok_or_tell_cargo(self) -> Option<Ok> { self.err_tell_cargo().ok() }
	}
}
