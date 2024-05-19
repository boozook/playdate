use super::Value;
use super::error::Error;
use std::borrow::Cow;
use std::collections::HashMap;
#[cfg(feature = "serde")]
use serde::Deserialize;


/// Package Metadata, contains:
/// - Package Manifest fields
/// - Assets tables - `assets` & `dev-assets`
/// - Configuration table - `options`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "T: Deserialize<'de>")))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct PlayDateMetadata<T> {
	pub name: Option<String>,
	pub version: Option<String>,
	pub author: Option<String>,
	#[cfg_attr(feature = "serde", serde(alias = "bundle-id"))]
	pub bundle_id: String,
	pub description: Option<String>,
	#[cfg_attr(feature = "serde", serde(alias = "image-path"))]
	pub image_path: Option<String>,
	#[cfg_attr(feature = "serde", serde(alias = "launch-sound-path"))]
	pub launch_sound_path: Option<String>,
	#[cfg_attr(feature = "serde", serde(alias = "content-warning"))]
	pub content_warning: Option<String>,
	#[cfg_attr(feature = "serde", serde(alias = "content-warning2"))]
	pub content_warning2: Option<String>,
	#[cfg_attr(feature = "serde", serde(alias = "build-number"))]
	pub build_number: Option<String>,
	#[cfg_attr(feature = "serde", serde(default = "PlayDateMetadataAssets::<T>::default"))]
	pub assets: PlayDateMetadataAssets<T>,
	#[cfg_attr(feature = "serde", serde(alias = "dev-assets"))]
	pub dev_assets: Option<PlayDateMetadataAssets<T>>,
	#[cfg_attr(feature = "serde", serde(default))]
	pub options: Options,
	#[cfg_attr(feature = "serde", serde(default))]
	pub support: Support,

	/// Package Manifest extra fields.
	// It could be `serde::flatten`, but if so we should remove `deny_unknown_fields` from entire struct
	// and break other fields validation, so it isn't good idea.
	#[cfg_attr(feature = "serde", serde(default))]
	pub extra: HashMap<String, T>,
}


impl<T: Value> PlayDateMetadata<T> where Error: From<<T as TryInto<AssetsOptions>>::Error> {
	pub fn merge_opts(&mut self) -> Result<(), Error> {
		let opts = if let Some(res) = self.assets.extract_options() {
			Some(res?)
		} else {
			Default::default()
		};

		match (self.options.assets.is_some(), opts) {
			(_, None) => Ok(()),
			(true, Some(_)) => {
				Err(concat!(
					"[package.metadata.playdate.assets.options]",
					" conflicts with ",
					"[package.metadata.playdate.options.assets]"
				).into())
			},
			(false, Some(opts)) => {
				let _ = self.options.assets.insert(opts);
				Ok(())
			},
		}
	}
}

impl<T: Value> PlayDateMetadata<T> {
	pub fn assets_options(&self) -> Cow<'_, crate::metadata::format::AssetsOptions> {
		self.options
		    .assets
		    .as_ref()
		    .map_or_else(Default::default, Cow::Borrowed)
	}
}


impl<T: Value> PlayDateMetadataAssets<T> where Error: From<<T as TryInto<AssetsOptions>>::Error> {
	fn extract_options(&mut self) -> Option<Result<AssetsOptions, Error>> {
		match self {
			PlayDateMetadataAssets::Map(map) => {
				// Remove only value that have `table/map` (not bool or str) type:
				if map.get("options")
				      .filter(|v| v.as_str().is_none() && v.as_bool().is_none())
				      .is_some()
				{
					map.remove("options")
					   .map(|v| v.try_into())
					   .map(|res| res.map_err(Into::into))
				} else {
					None
				}
			},
			_ => None,
		}
	}
}

#[cfg(feature = "serde_json")]
impl TryFrom<serde_json::Value> for AssetsOptions {
	type Error = serde_json::error::Error;
	fn try_from(value: serde_json::Value) -> std::result::Result<Self, Self::Error> {
		serde_json::from_value(value)
	}
}

#[cfg(feature = "toml")]
impl TryFrom<toml::Value> for AssetsOptions {
	type Error = toml::de::Error;
	fn try_from(value: toml::Value) -> std::result::Result<Self, Self::Error> {
		toml::Value::try_into::<AssetsOptions>(value)
	}
}


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Options {
	pub assets: Option<AssetsOptions>,
	// Output layout ctrl, temporary removed:
	// #[serde(alias = "cross-target", default)]
	// pub cross_target: bool,
}


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct AssetsOptions {
	#[cfg_attr(feature = "serde", serde(alias = "override", default = "bool::<true>"))]
	pub overwrite: bool,

	#[cfg_attr(feature = "serde", serde(alias = "follow-symlinks", default = "bool::<true>"))]
	pub follow_symlinks: bool,

	#[cfg_attr(feature = "serde", serde(alias = "build-method", default))]
	pub method: AssetsBuildMethod,

	/// Allow building assets for dependencies
	#[cfg_attr(feature = "serde", serde(default = "bool::<false>"))]
	pub dependencies: bool,
}

#[cfg(feature = "serde")]
const fn bool<const V: bool>() -> bool { V }


#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AssetsBuildMethod {
	Copy,
	Link,
}

impl Default for AssetsBuildMethod {
	fn default() -> Self { Self::Link }
}


#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "T: Deserialize<'de>")))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub enum PlayDateMetadataAssets<T> {
	/// List of paths to include.
	List(Vec<String>),
	/// Rules & queries used to resolve paths to include.
	Map(HashMap<String, T>),
}

impl<T> Default for PlayDateMetadataAssets<T> {
	fn default() -> Self { Self::List(Vec::with_capacity(0)) }
}

impl<T> PlayDateMetadataAssets<T> {
	pub fn is_empty(&self) -> bool {
		match self {
			PlayDateMetadataAssets::List(list) => list.is_empty(),
			PlayDateMetadataAssets::Map(map) => map.is_empty(),
		}
	}
}


/// Compatibility options.
/// e.g. Crank manifest path.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Support {
	// #[serde(alias = "crank-manifest")]
	// pub crank_manifest: Option<PathBuf>, // bool
}
