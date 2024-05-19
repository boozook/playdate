#![cfg(feature = "crate-metadata")]
use std::borrow::Cow;
use std::path::PathBuf;
pub use crate_metadata::Package;
use super::format::PlayDateMetadata;
use crate::config::Env;
use crate::manifest::ManifestDataSource;
use super::Value;
use super::error::Error;


#[derive(Debug)]
pub struct PackageInfo<T> {
	pub package: Package<Metadata<T>>,
	pub target_directory: PathBuf,
}


#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "T: serde::Deserialize<'de>")))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Metadata<T> {
	pub playdate: Option<PlayDateMetadata<T>>,
}


pub fn crate_metadata<T: Value>(env: &Env) -> Result<PackageInfo<T>, Error> {
	let name = env.cargo_pkg_name();
	let manifest = env.manifest_path();
	manifest.try_exists()?
	        .then(|| ())
	        .ok_or("unable to find crate manifest")?;

	let metadata = crate_metadata::crate_metadata::<Metadata<T>>().unwrap();
	let result = metadata.packages
	                     .into_iter()
	                     .find(|p| &p.name == name)
	                     .map(|package| {
		                     PackageInfo { package,
		                                   target_directory: metadata.target_directory }
	                     });
	result.ok_or("package not found".into())
}


impl<T: crate::value::Value> ManifestDataSource for PackageInfo<T> {
	type Value = T;

	fn name(&self) -> &str { &self.package.name }

	fn authors(&self) -> &[String] { &self.package.authors }

	fn version(&self) -> Cow<str> { Cow::Borrowed(&self.package.version) }

	fn description(&self) -> Option<&str> { self.package.description.as_deref() }

	fn metadata(&self) -> Option<&PlayDateMetadata<Self::Value>> {
		self.package.metadata.as_ref().and_then(|m| m.playdate.as_ref())
	}
}
