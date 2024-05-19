use std::borrow::Cow;

pub use crate::compile::PDX_PKG_MANIFEST_FILENAME;
use crate::metadata::format::PlayDateMetadata;
use self::format::Manifest;


pub mod format;


pub trait ManifestDataSource {
	type Value: crate::value::Value;

	fn name(&self) -> &str;
	fn authors(&self) -> &[String];
	fn version(&self) -> Cow<str>;
	fn description(&self) -> Option<&str>;
	fn metadata(&self) -> Option<&PlayDateMetadata<Self::Value>>;
}


impl<'t, T> TryFrom<SourceRef<'t, T>> for Manifest<T::Value> where T: ManifestDataSource {
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
		                          build_number: metadata.build_number.as_ref().and_then(|v| v.parse().ok()),
		                          image_path: metadata.image_path.to_owned(),
		                          launch_sound_path: metadata.launch_sound_path.to_owned(),
		                          content_warning: metadata.content_warning.to_owned(),
		                          content_warning2: metadata.content_warning2.to_owned(),
		                          extra: metadata.extra.to_owned() };
		Ok(manifest)
	}
}

impl<V> Manifest<V> {
	pub fn try_from_source<T>(source: T) -> Result<Self, &'static str>
		where T: ManifestDataSource<Value = V> {
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
	use std::collections::HashMap;
	use std::ops::Deref;
	use crate::metadata::format::PlayDateMetadata;
	use super::*;

	#[cfg(feature = "serde_json")]
	use serde_json::Value;
	#[cfg(all(feature = "toml", not(feature = "serde_json")))]
	use toml::Value;
	#[cfg(all(not(feature = "toml"), not(feature = "serde_json")))]
	use crate::value::default::Value;


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


	fn metadata_minimal() -> PlayDateMetadata<Value> {
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
		                   dev_assets: Default::default(),
		                   options: Default::default(),
		                   support: Default::default(),
		                   extra: Default::default() }
	}
	fn metadata_maximal() -> PlayDateMetadata<Value> {
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
		                   dev_assets: Default::default(),
		                   options: Default::default(),
		                   support: Default::default(),
		                   extra: Default::default() }
	}
	fn metadata_extra() -> PlayDateMetadata<Value> {
		let mut data = metadata_minimal();
		data.extra = HashMap::new();
		data.extra.insert("key".to_owned(), "value".to_string().into());
		data
	}


	#[test]
	fn from_data_source_minimal() {
		let source = ManifestSource { name: "name",
		                              authors: vec!["author".to_owned()],
		                              version: "0.42.0",
		                              description: Some("description"),
		                              metadata: Some(metadata_minimal()) };
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
	fn from_data_source_maximal() {
		let source = ManifestSource { name: "crate-name",
		                              authors: vec!["crate-author".to_owned()],
		                              version: "0.0.0",
		                              description: Some("crate-description"),
		                              metadata: Some(metadata_maximal()) };
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

	#[test]
	fn from_data_source_extra() {
		let source = ManifestSource { name: "-",
		                              version: "0.0.0",
		                              description: Some("-"),
		                              authors: vec!["-".to_owned()],
		                              metadata: Some(metadata_extra()) };
		let manifest = Manifest::try_from_source(source).expect("manifest");

		assert_eq!(Some(&Value::from("value".to_string())), manifest.extra.get("key"));
		assert_eq!(1, manifest.extra.len());
	}
}
