use std::path::Path;

use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CompileMode;
use cargo::core::compiler::CompileTarget;
use cargo::core::compiler::CrateType;
use cargo::core::PackageId;
use cargo::core::SourceId;
pub use serde::{Serialize, Deserialize};
pub use serde::{Serializer, Deserializer};


#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
#[serde(remote = "CompileMode")]
pub enum CompileModeProxy {
	/// A target being built for a test.
	Test,
	/// Building a target with `rustc` (lib or bin).
	Build,
	/// Building a target with `rustc` to emit `rmeta` metadata only. If
	/// `test` is true, then it is also compiled with `--test` to check it like
	/// a test.
	Check { test: bool },
	/// A target that will be documented with `rustdoc`.
	/// If `deps` is true, then it will also document all dependencies.
	Doc,
	/// A target that will be tested with `rustdoc`.
	Doctest,
	/// An example or library that will be scraped for function calls by `rustdoc`.
	Docscrape,
	/// A marker for Units that represent the execution of a `build.rs` script.
	RunCustomBuild,
}


/// Remote-type for [`CompileMode`](cargo::core::TargetKind).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetKind {
	Lib(Vec<CrateType>),
	Bin,
	Test,
	Bench,
	Example,
	CustomBuild,
}

impl<'de> Deserialize<'de> for TargetKind {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where D: Deserializer<'de> {
		use self::TargetKind::*;

		let raw = Vec::<&str>::deserialize(deserializer)?;
		Ok(match *raw {
			[] => return Err(serde::de::Error::invalid_length(0, &"at least one target kind")),
			["bin"] => Bin,
			["example"] => Example,
			["test"] => Test,
			["custom-build"] => CustomBuild,
			["bench"] => Bench,
			ref lib_kinds => {
				Lib(lib_kinds.iter()
				             .cloned()
				             .map(|s| CrateType::from(&s.to_owned()))
				             .collect())
			},
		})
	}
}

impl Serialize for TargetKind {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		use self::TargetKind::*;
		match self {
			Lib(kinds) => s.collect_seq(kinds.iter().map(|t| t.to_string())),
			Bin => ["bin"].serialize(s),
			Example => ["example"].serialize(s),
			Test => ["test"].serialize(s),
			CustomBuild => ["custom-build"].serialize(s),
			Bench => ["bench"].serialize(s),
		}
	}
}


#[derive(Debug, Clone, Copy)]
pub enum TargetKindWild {
	Lib,
	Bin,
	Test,
	Bench,
	ExampleLib,
	ExampleBin,
	CustomBuild,
}

impl PartialEq<TargetKind> for TargetKindWild {
	fn eq(&self, other: &TargetKind) -> bool {
		match self {
			TargetKindWild::Lib => matches!(other, TargetKind::Lib(_)),
			TargetKindWild::Bin => matches!(other, TargetKind::Bin),
			TargetKindWild::Test => matches!(other, TargetKind::Test),
			TargetKindWild::Bench => matches!(other, TargetKind::Bench),
			TargetKindWild::ExampleLib => matches!(other, TargetKind::Example),
			TargetKindWild::ExampleBin => matches!(other, TargetKind::Example),
			TargetKindWild::CustomBuild => matches!(other, TargetKind::CustomBuild),
		}
	}
}


pub fn de_package_id_or_specs<'de, D>(deserializer: D) -> Result<Vec<PackageId>, D::Error>
	where D: Deserializer<'de> {
	let items = Vec::<String>::deserialize(deserializer)?;
	let mut ids = Vec::with_capacity(items.len());
	for item in items {
		ids.push(string_to_package_id::<D::Error>(item)?);
	}
	Ok(ids)
}

pub fn de_package_id_or_spec<'de, D>(deserializer: D) -> Result<PackageId, D::Error>
	where D: Deserializer<'de> {
	string_to_package_id(String::deserialize(deserializer)?)
}

/// Try deserialize using actual deserializer.
/// If fails, try to deserialize as old format.
/// Fixes breaking change between old and new format in cargo ~0.78.1.
pub fn string_to_package_id<Error: serde::de::Error>(mut line: String) -> Result<PackageId, Error> {
	// wrap into quotes for deserializer:
	line.insert(0, '\"');
	line.push('\"');
	// preserve original value:
	let value = &line[1..(line.len() - 1)];

	// try actual format first:
	let res = serde_json::from_str::<PackageId>(&line).map_err(Error::custom);


	res.or_else(|err| {
		   // it also can be PackageIdSpec
		   use cargo::core::PackageIdSpec;

		   let spec = PackageIdSpec::parse(&line).map_err(Error::custom)?;
		   spec.url()
		       .map(|url| url.as_str())
		       .map(SourceId::from_url)
		       .and_then(|res| res.ok())
		       .map(|src| (spec, src))
		       .and_then(|(spec, src)| spec.version().map(|ver| (spec, src, ver)))
		       .map(|(spec, src, ver)| PackageId::new(spec.name().into(), ver, src))
		       .ok_or(err)
	   })
	   .or_else(move |err| {
		   // otherwise try old formats:
		   if let Some((uri, name_ver)) = value.split_once('#') {
			   let sid = SourceId::from_url(uri).map_err(Error::custom)?;

			   if let Some((name, ver)) = name_ver.split_once('@') {
				   let ver = ver.parse().map_err(Error::custom)?;
				   let id = PackageId::new(name.into(), ver, sid);
				   return Ok(id);
			   } else {
				   let sid_temp = SourceId::from_url(value).map_err(Error::custom)?;
				   let url = sid_temp.url();
				   if let Some(ver) = url.fragment() {
					   let ver = ver.parse().map_err(Error::custom)?;
					   let name = Path::new(url.path()).file_name()
					                                   .ok_or_else(|| Error::custom("Package name missed"))?
					                                   .to_string_lossy();
					   let id = PackageId::new(name.as_ref().into(), ver, sid);
					   return Ok(id);
				   }
			   }
		   }
		   Err(err)
	   })
}


pub fn de_crate_types<'de, D>(deserializer: D) -> Result<Vec<CrateType>, D::Error>
	where D: Deserializer<'de> {
	let kinds = Vec::<&str>::deserialize(deserializer)?;
	let kinds = kinds.into_iter()
	                 .map(|s| CrateType::from(&s.to_owned()))
	                 .collect();
	Ok(kinds)
}


pub fn de_compile_kind<'de, D>(deserializer: D) -> Result<CompileKind, D::Error>
	where D: Deserializer<'de> {
	let res = if let Some(s) = Option::<&str>::deserialize(deserializer)? {
		let target = CompileTarget::new(s).map_err(serde::de::Error::custom)?;
		CompileKind::Target(target)
	} else {
		CompileKind::Host
	};
	Ok(res)
}


#[cfg(test)]
mod tests {
	use super::*;


	#[derive(Debug, Serialize, Deserialize)]
	pub struct PackageIdWrapped {
		#[serde(deserialize_with = "super::de_package_id_or_spec")]
		pub package_id: PackageId,
	}


	/// Before cargo 0.78
	#[test]
	fn package_spec_a() {
		let msg = r#"{"package_id": "path+file:///Users/U/Developer/Projects/Playdate/playdate-rs/api/sys#playdate-sys@0.3.3"}"#;
		serde_json::from_str::<PackageIdWrapped>(msg).unwrap();
	}

	/// Before cargo 0.78
	#[test]
	fn package_spec_b() {
		let msg = r#"{"package_id": "path+file:///Users/U/Developer/Projects/Playdate/playdate-rs/cargo/tests/crates/simple/with-cfg#0.1.0"}"#;
		serde_json::from_str::<PackageIdWrapped>(msg).unwrap();
	}


	/// From cargo 0.78
	#[test]
	fn package_id() {
		let msg = r#"{"package_id": "playdate-sys 0.3.3 (path+file:///Users/U/Developer/Projects/Playdate/playdate-rs/api/sys)"}"#;
		serde_json::from_str::<PackageIdWrapped>(msg).unwrap();
	}
}
