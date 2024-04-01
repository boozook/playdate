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
			if s.starts_with("{") {
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
	use cargo::core::SourceId;
	use cargo::util::interning::InternedString;
	use cargo::util::machine_message::Message;
	use serde::Serialize;
	use serde::Deserialize;
	use serde::Deserializer;
	use cargo::core::PackageId;
	pub use crate::build::plan::format::TargetKind;


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

	/// Try deserialize using actual deserializer.
	/// If fails, try to deserialize as old format.
	/// Fixes breaking change between old and new format in cargo ~0.78.1.
	fn deserialize_package_id<'de, D>(deserializer: D) -> Result<PackageId, D::Error>
		where D: Deserializer<'de> {
		use serde::de::Error;

		let mut line = String::deserialize(deserializer)?;
		// wrap into quotes for deserializer:
		line.insert(0, '\"');
		line.push('\"');
		// preserve original value:
		let value = &line[1..(line.len() - 1)];

		// try actual format first:
		let res = serde_json::from_str::<PackageId>(&line).map_err(|err| Error::custom(err));

		// otherwise try old format:
		res.or_else(move |err| {
			   if let Some((uri, name_ver)) = value.split_once('#') {
				   let sid = SourceId::from_url(uri).map_err(|err| Error::custom(err))?;

				   if let Some((name, ver)) = name_ver.split_once('@') {
					   let ver = ver.parse().map_err(|err| Error::custom(err))?;

					   let id = PackageId::new(name.into(), ver, sid);
					   return Ok(id);
				   }
			   }
			   Err(err)
		   })
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
		#[serde(deserialize_with = "deserialize_crate_type_vec")]
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

	fn deserialize_crate_type_vec<'de, D>(deserializer: D) -> Result<Vec<CrateType>, D::Error>
		where D: Deserializer<'de> {
		let strings = Vec::<&str>::deserialize(deserializer)?;
		let res = strings.into_iter()
		                 .map(|s| CrateType::from(&s.to_owned()))
		                 .collect();
		Ok(res)
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


		#[derive(Debug, Serialize, Deserialize)]
		pub struct PackageIdWrapped {
			#[serde(deserialize_with = "super::deserialize_package_id")]
			pub package_id: PackageId,
		}


		/// Before cargo 0.78
		#[test]
		fn message_format_old() {
			let msg = r#"{"package_id": "path+file:///Users/name/Developer/Projects/Playdate/playdate-rs/api/sys#playdate-sys@0.3.3"}"#;
			serde_json::from_str::<PackageIdWrapped>(msg).unwrap();
		}


		/// From cargo 0.78
		#[test]
		fn message_format_new() {
			let msg = r#"{"package_id": "playdate-sys 0.3.3 (path+file:///Users/name/Developer/Projects/Playdate/playdate-rs/api/sys)"}"#;
			serde_json::from_str::<PackageIdWrapped>(msg).unwrap();
		}
	}
}
