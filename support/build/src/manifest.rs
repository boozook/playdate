use std::borrow::Cow;
use std::path::PathBuf;

pub use crate::compile::PDX_PKG_MANIFEST_FILENAME;
use crate::metadata::format::PlayDateMetadata;
use self::format::Manifest;


pub struct SavedPlaydateManifest {
	pub manifest: Manifest,
	pub path: PathBuf,
}


pub mod format {
	use serde::Deserialize;
	use serde::Serialize;


	#[derive(Serialize, Deserialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct Manifest {
		// pdxversion=20000
		pub name: String,
		pub author: String,
		pub description: String,
		#[serde(rename = "bundleID")]
		pub bundle_id: String,
		pub version: String,
		pub build_number: Option<usize>,
		pub image_path: Option<String>,
		pub launch_sound_path: Option<String>,
		pub content_warning: Option<String>,
		pub content_warning2: Option<String>,
	}


	impl ToString for Manifest {
		fn to_string(&self) -> String {
			let mut result = String::new();

			fn to_row<K: AsRef<str>, V: AsRef<str>>(key: K, value: V) -> String {
				if !value.as_ref().trim().is_empty() {
					format!("{}={}\n", key.as_ref(), value.as_ref())
				} else {
					String::new()
				}
			}

			result.push_str(&to_row("name", &self.name));
			result.push_str(&to_row("author", &self.author));
			result.push_str(&to_row("description", &self.description));
			result.push_str(&to_row("bundleID", &self.bundle_id));
			result.push_str(&to_row("version", &self.version));
			if let Some(value) = self.build_number {
				result.push_str(&to_row("buildNumber", value.to_string()));
			}
			if let Some(ref value) = self.image_path {
				result.push_str(&to_row("imagePath", value));
			}
			if let Some(ref value) = self.launch_sound_path {
				result.push_str(&to_row("launchSoundPath", value));
			}
			if let Some(ref value) = self.content_warning {
				result.push_str(&to_row("contentWarning", value));
				if let Some(ref value) = self.content_warning2 {
					result.push_str(&to_row("contentWarning2", value));
				}
			}
			result
		}
	}
}


pub trait ManifestDataSource {
	type Value: crate::value::Value;

	fn name(&self) -> &str;
	fn authors(&self) -> &[String];
	fn version(&self) -> Cow<str>;
	fn description(&self) -> Option<&str>;
	fn metadata(&self) -> Option<&PlayDateMetadata<Self::Value>>;
}


impl<'t, T> TryFrom<SourceRef<'t, T>> for Manifest where T: ManifestDataSource {
	type Error = &'static str;

	fn try_from(source: SourceRef<'t, T>) -> Result<Self, &'static str> {
		let metadata = source.metadata()
		                     .ok_or("[package.metadata.playdate] not found in the manifest file Cargo.toml")?;

		let description = metadata.description
		                          .to_owned()
		                          .or(source.description().map(ToOwned::to_owned))
		                          .ok_or("description not found")?;
		let manifest = Manifest { name: metadata.name.to_owned().unwrap_or(source.name().to_owned()),
		                          author: metadata.author.to_owned().unwrap_or(source.authors().join(", ")),
		                          description,
		                          bundle_id: metadata.bundle_id.to_owned(),
		                          version: metadata.version
		                                           .to_owned()
		                                           .unwrap_or(source.version().to_string()),
		                          build_number: metadata.build_number.as_ref().map(|v| v.parse().ok()).flatten(),
		                          image_path: metadata.image_path.to_owned(),
		                          launch_sound_path: metadata.launch_sound_path.to_owned(),
		                          content_warning: metadata.content_warning.to_owned(),
		                          content_warning2: metadata.content_warning2.to_owned() };
		Ok(manifest)
	}
}

impl Manifest {
	pub fn try_from_source<T>(source: T) -> Result<Self, &'static str>
		where T: ManifestDataSource {
		SourceRef(&source).try_into()
	}
}


struct SourceRef<'t, T: ManifestDataSource>(pub &'t T);
impl<'t, T: ManifestDataSource> SourceRef<'t, T> {
	#![allow(dead_code)]
	pub fn inner(&self) -> &'t T { self.0 }
	pub fn into_inner(self) -> &'t T { self.0 }
}

impl<'t, T> From<&'t T> for SourceRef<'t, T> where T: ManifestDataSource {
	fn from(value: &'t T) -> Self { Self(value) }
}

impl<'t, T> ManifestDataSource for SourceRef<'t, T> where T: ManifestDataSource {
	type Value = T::Value;
	fn name(&self) -> &str { self.inner().name() }
	fn authors(&self) -> &[String] { self.inner().authors() }
	fn version(&self) -> Cow<str> { self.inner().version() }
	fn description(&self) -> Option<&str> { self.inner().description() }
	fn metadata(&self) -> Option<&PlayDateMetadata<Self::Value>> { self.inner().metadata() }
}


#[cfg(test)]
mod tests {
	#[cfg(any(feature = "toml", feature = "serde_json"))]
	use std::ops::Deref;
	use crate::metadata::format::PlayDateMetadata;
	use super::*;

	#[cfg(feature = "serde_json")]
	use serde_json::Value;
	#[cfg(all(feature = "toml", not(feature = "serde_json")))]
	use toml::Value;


	struct ManifestSource<Name, Ver, Descr> {
		name: Name,
		authors: Vec<String>,
		version: Ver,
		description: Option<Descr>,
		metadata: Option<PlayDateMetadata<Value>>,
	}

	impl<N, V, D> ManifestDataSource for ManifestSource<N, V, D>
		where N: AsRef<str>,
		      V: AsRef<str>,
		      D: Deref<Target = str>
	{
		type Value = Value;

		fn name(&self) -> &str { self.name.as_ref() }
		fn authors(&self) -> &[String] { &self.authors }
		fn version(&self) -> Cow<str> { self.version.as_ref().into() }
		fn description(&self) -> Option<&str> { self.description.as_deref() }
		fn metadata(&self) -> Option<&PlayDateMetadata<Value>> { self.metadata.as_ref() }
	}


	fn minimal_metadata() -> PlayDateMetadata<Value> {
		PlayDateMetadata { bundle_id: "bundle.id".to_owned(),
		                   name: Default::default(),
		                   version: Default::default(),
		                   author: Default::default(),
		                   description: Default::default(),
		                   image_path: Default::default(),
		                   launch_sound_path: Default::default(),
		                   content_warning: Default::default(),
		                   content_warning2: Default::default(),
		                   build_number: Default::default(),
		                   assets: Default::default(),
		                   options: Default::default(),
		                   support: Default::default() }
	}
	fn maximal_metadata() -> PlayDateMetadata<Value> {
		PlayDateMetadata { bundle_id: "bundle.id".to_owned(),
		                   name: Some("name".to_owned()),
		                   version: Some("0.42.0".to_owned()),
		                   author: Some("author".to_owned()),
		                   description: Some("description".to_owned()),
		                   image_path: Some("image_path".to_owned()),
		                   launch_sound_path: Some("launch_sound_path".to_owned()),
		                   content_warning: Some("content_warning".to_owned()),
		                   content_warning2: Some("content_warning2".to_owned()),
		                   build_number: Some("42".to_owned()),
		                   assets: Default::default(),
		                   options: Default::default(),
		                   support: Default::default() }
	}


	#[test]
	fn manifest_data_source_minimal() {
		let source = ManifestSource { name: "name",
		                              authors: vec!["author".to_owned()],
		                              version: "0.42.0",
		                              description: Some("description"),
		                              metadata: Some(minimal_metadata()) };
		let manifest = Manifest::try_from_source(source).expect("manifest");

		assert_eq!(&manifest.name, "name");
		assert_eq!(&manifest.author, "author");
		assert_eq!(&manifest.version, "0.42.0");
		assert_eq!(&manifest.description, "description");
		assert_eq!(&manifest.bundle_id, "bundle.id");

		assert!(manifest.image_path.is_none());
		assert!(manifest.launch_sound_path.is_none());
		assert!(manifest.content_warning.is_none());
		assert!(manifest.content_warning2.is_none());
		assert!(manifest.build_number.is_none());
	}

	#[test]
	fn manifest_data_source_maximal() {
		let source = ManifestSource { name: "crate-name",
		                              authors: vec!["crate-author".to_owned()],
		                              version: "0.0.0",
		                              description: Some("crate-description"),
		                              metadata: Some(maximal_metadata()) };
		let manifest = Manifest::try_from_source(source).expect("manifest");

		assert_eq!(&manifest.name, "name");
		assert_eq!(&manifest.author, "author");
		assert_eq!(&manifest.version, "0.42.0");
		assert_eq!(&manifest.description, "description");
		assert_eq!(&manifest.bundle_id, "bundle.id");

		assert_eq!(manifest.image_path.as_deref(), Some("image_path"));
		assert_eq!(manifest.launch_sound_path.as_deref(), Some("launch_sound_path"));
		assert_eq!(manifest.content_warning.as_deref(), Some("content_warning"));
		assert_eq!(manifest.content_warning2.as_deref(), Some("content_warning2"));
		assert_eq!(manifest.build_number, Some(42));
	}
}
