use std::ffi::OsStr;
use std::process::{Command, Child, Stdio, ExitStatus, Output};
use std::io::{prelude::*, BufReader};

use anyhow::Result;
use anyhow::ensure;

use clap_lex::OsStrExt;

use crate::logger::LogErr;

use self::format::{SerializedTarget, CargoMessage};


const MESSAGE_FORMAT_ARG: &str = "--message-format";


/// Blocking reader for Cargo JSON standard output,
/// `stderr` is inherit, piped parent process.
pub struct CargoJsonReader {
	handle: Child,
}


impl CargoJsonReader {
	pub fn new(mut cargo: Command) -> Result<Self> {
		if !check_message_format(&cargo)? {
			cargo.arg("--message-format=json");
		}
		cargo.stderr(Stdio::inherit());
		cargo.stdout(Stdio::piped());
		Ok(Self { handle: cargo.spawn()? })
	}


	pub fn read(&mut self) -> Result<impl Iterator<Item = CargoMessage>> {
		let is_json = |s: &String| {
			if s.starts_with('{') {
				true
			} else {
				println!("{s}");
				false
			}
		};
		let iter = self.handle
		               .stdout
		               .take()
		               .into_iter()
		               .flat_map(|out| BufReader::new(out).lines().map_while(|line| line.log_err().ok()))
		               .filter(is_json)
		               .filter_map(|line| serde_json::from_str::<CargoMessage>(&line).log_err().ok());
		Ok(iter)
	}


	pub fn status(&mut self) -> Result<ExitStatus> { self.handle.wait().map_err(anyhow::Error::from) }

	#[allow(dead_code)]
	pub fn output(self) -> Result<Output> { self.handle.wait_with_output().map_err(anyhow::Error::from) }
}


/// check if the cargo command has the --message-format argument with json value
fn check_message_format(cargo: &Command) -> Result<bool> {
	let args = cargo.get_args();
	let mut next_arg_should_be_value = false;
	let mut has_message_format = false;

	let expected = OsStr::new("json");
	for arg in args {
		if !next_arg_should_be_value {
			if arg.starts_with(MESSAGE_FORMAT_ARG) {
				if let Some(value) = arg.split("=").nth(1) {
					ensure!(value == expected, "message format should be json");
					has_message_format = true;
				} else {
					next_arg_should_be_value = true;
				}
			}
		} else {
			ensure!(arg == expected, "message format should be json");
			has_message_format = true;
			next_arg_should_be_value = false;
		}
	}

	Ok(has_message_format)
}


impl SerializedTarget {
	pub fn kind(&self) -> cargo::core::TargetKind {
		use cargo::core::compiler::CrateType;
		use self::format::TargetKind as STK;
		use cargo::core::TargetKind as TK;

		match &self.kind {
			STK::Lib(types) => TK::Lib(types.to_owned()),
			STK::Bin => TK::Bin,
			STK::Test => TK::Test,
			STK::Bench => TK::Bench,
			STK::CustomBuild => TK::CustomBuild,
			STK::Example => {
				if matches!(self.crate_types.first(), Some(CrateType::Bin)) {
					TK::ExampleBin
				} else {
					debug_assert!(
					              self.crate_types.contains(&CrateType::Lib) ||
					              self.crate_types.contains(&CrateType::Dylib) ||
					              self.crate_types.contains(&CrateType::Cdylib)
					);
					TK::ExampleLib(self.crate_types.to_owned())
				}
			},
		}
	}
}


pub mod format {
	#![allow(dead_code)]
	use std::path::PathBuf;
	use cargo::core::compiler::CrateType;
	use cargo::util::interning::InternedString;
	use cargo::util::machine_message::Message;
	use serde::Serialize;
	use serde::Deserialize;
	use cargo::core::PackageId;
	use crate::utils::cargo::build_plan::format::deserialize_crate_types;
	use crate::utils::cargo::format::deserialize_package_id;
	pub use crate::utils::cargo::format::TargetKind;


	#[derive(Serialize, Deserialize)]
	#[serde(tag = "reason", rename_all = "kebab-case")]
	pub enum CargoMessage {
		CompilerArtifact(Artifact),
		BuildScriptExecuted(serde_json::Value),
		CompilerMessage { message: CompilerMessage },
		BuildFinished { success: bool },
	}

	impl Message for CargoMessage {
		fn reason(&self) -> &str { unreachable!() }
		fn to_json_string(&self) -> String {
			let json = serde_json::to_string(self).unwrap();
			assert!(json.starts_with("{\""));
			json
		}
	}

	impl<'t> Message for &'t CargoMessage {
		fn reason(&self) -> &str { unreachable!() }
		fn to_json_string(&self) -> String {
			let json = serde_json::to_string(self).unwrap();
			assert!(json.starts_with("{\""));
			json
		}
	}


	#[derive(Debug, Serialize, Deserialize)]
	pub struct Artifact {
		#[serde(deserialize_with = "deserialize_package_id")]
		pub package_id: PackageId,
		pub manifest_path: PathBuf,
		pub target: SerializedTarget,
		pub profile: ArtifactProfile,
		pub features: Vec<String>,
		pub filenames: Vec<PathBuf>,
		pub executable: Option<PathBuf>,
		pub fresh: bool,
	}


	impl Message for Artifact {
		fn reason(&self) -> &str { "compiler-artifact" }
	}


	#[derive(Debug, Serialize, Deserialize)]
	pub struct SerializedTarget {
		/// Is this a `--bin bin`, `--lib`, `--example ex`?
		/// Serialized as a list of strings for historical reasons.
		pub kind: TargetKind,
		/// Corresponds to `--crate-type` compiler attribute.
		/// See <https://doc.rust-lang.org/reference/linkage.html>
		#[serde(deserialize_with = "deserialize_crate_types")]
		pub crate_types: Vec<CrateType>,
		pub name: InternedString,
		pub src_path: Option<PathBuf>,
		pub edition: InternedString,
		pub required_features: Option<Vec<String>>,
		/// Whether docs should be built for the target via `cargo doc`
		/// See <https://doc.rust-lang.org/cargo/commands/cargo-doc.html#target-selection>
		pub doc: bool,
		pub doctest: bool,
		/// Whether tests should be run for the target (`test` field in `Cargo.toml`)
		pub test: bool,
	}


	#[derive(Clone, Debug, Serialize, Deserialize)]
	pub struct ArtifactProfile {
		pub opt_level: InternedString,
		pub debuginfo: Option<u32>,
		pub debug_assertions: bool,
		pub overflow_checks: bool,
		pub test: bool,
	}


	#[derive(Clone, Debug, Serialize, Deserialize)]
	#[serde(rename_all = "snake_case")]
	pub struct CompilerMessage {
		pub rendered: String,
		pub code: Option<serde_json::Value>,
		pub level: String,
		pub spans: Vec<CompilerMessageSpan>,
	}

	impl Message for CompilerMessage {
		fn reason(&self) -> &str { "compiler-message" }
	}

	#[derive(Clone, Debug, Serialize, Deserialize)]
	#[serde(rename_all = "snake_case")]
	pub struct CompilerMessageSpan {
		column_start: usize,
		column_end: usize,
		file_name: String,
		line_start: usize,
		line_end: usize,
	}


	#[cfg(test)]
	mod tests {
		use super::*;


		/// Before cargo 0.78
		#[test]
		fn msg_message_format_old() {
			let msg = r#"{"reason":"compiler-artifact","package_id":"path+file:///Users/U/Dev/playdate-rs/api/sys#playdate-sys@0.3.3","manifest_path":"/Users/U/Dev/playdate-rs/api/sys/Cargo.toml","target":{"kind":["example"],"crate_types":["dylib","staticlib"],"name":"hello-world","src_path":"/Users/U/Dev/playdate-rs/api/sys/examples/hello-world.rs","edition":"2021","required-features":["lang-items"],"doc":false,"doctest":false,"test":false},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":["allocator","arrayvec","bindgen","bindgen-runtime","bindings-derive-debug","default","eh-personality","lang-items","panic-handler"],"filenames":["/Users/U/Dev/playdate-rs/target/aarch64-apple-darwin/debug/examples/libhello_world.dylib","/Users/U/Dev/playdate-rs/target/aarch64-apple-darwin/debug/examples/libhello_world.a"],"executable":null,"fresh":false}"#;
			serde_json::from_str::<Artifact>(msg).unwrap();
		}
	}
}
