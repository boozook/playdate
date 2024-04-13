use crate::value::Value;


pub const METADATA_FIELD: &str = "playdate";


#[cfg(feature = "crate-metadata")]
pub use self::cargo::*;

mod cargo {
	#![cfg(feature = "crate-metadata")]
	use std::path::PathBuf;
	pub use crate_metadata::Package;
	use super::format::Metadata;
	use crate::config::Env;
	use super::Value;
	use super::error::Error;


	#[derive(Debug)]
	pub struct PackageInfo<T> {
		pub package: Package<Metadata<T>>,
		pub target_directory: PathBuf,
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
}


pub mod error {
	use std::io::Error as IoError;

	#[derive(Debug)]
	pub enum Error {
		Io(IoError),
		Err(&'static str),
		#[cfg(feature = "serde_json")]
		Json(serde_json::error::Error),
		#[cfg(feature = "toml")]
		Toml(toml::de::Error),
	}

	impl From<&'static str> for Error {
		fn from(value: &'static str) -> Self { Self::Err(value) }
	}

	impl From<IoError> for Error {
		fn from(err: IoError) -> Self { Self::Io(err) }
	}

	#[cfg(feature = "serde_json")]
	impl From<serde_json::error::Error> for Error {
		fn from(err: serde_json::error::Error) -> Self { Self::Json(err) }
	}

	#[cfg(feature = "toml")]
	impl From<toml::de::Error> for Error {
		fn from(err: toml::de::Error) -> Self { Self::Toml(err) }
	}

	impl std::fmt::Display for Error {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Error::Io(err) => err.fmt(f),
				Error::Err(err) => err.fmt(f),
				#[cfg(feature = "serde_json")]
				Error::Json(err) => err.fmt(f),
				#[cfg(feature = "toml")]
				Error::Toml(err) => err.fmt(f),
			}
		}
	}

	impl std::error::Error for Error {
		fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
			match self {
				Error::Err(_) => None,
				Error::Io(err) => Some(err),
				#[cfg(feature = "serde_json")]
				Error::Json(err) => Some(err),
				#[cfg(feature = "toml")]
				Error::Toml(err) => Some(err),
			}
		}
	}
}


pub mod format {
	use super::Value;
	use super::error::Error;
	use std::borrow::Cow;
	use std::collections::HashMap;
	use serde::Deserialize;


	#[derive(Deserialize, Debug)]
	#[serde(bound(deserialize = "T: Deserialize<'de>"))]
	#[serde(deny_unknown_fields)]
	pub struct Metadata<T> {
		// TODO: test deserialization with `crate::metadata::METADATA_FIELD` for field name.
		pub playdate: Option<PlayDateMetadata<T>>,
	}


	#[derive(Deserialize, Debug, Clone)]
	#[serde(bound(deserialize = "T: Deserialize<'de>"))]
	#[serde(deny_unknown_fields)]
	pub struct PlayDateMetadata<T> {
		pub name: Option<String>,
		pub version: Option<String>,
		pub author: Option<String>,
		#[serde(alias = "bundle-id", rename = "bundle-id")]
		pub bundle_id: String,
		pub description: Option<String>,
		#[serde(alias = "image-path", rename = "image-path")]
		pub image_path: Option<String>,
		#[serde(alias = "launch-sound-path", rename = "launch-sound-path")]
		pub launch_sound_path: Option<String>,
		#[serde(alias = "content-warning", rename = "content-warning")]
		pub content_warning: Option<String>,
		#[serde(alias = "content-warning2", rename = "content-warning2")]
		pub content_warning2: Option<String>,
		#[serde(alias = "build-number", rename = "build-number")]
		pub build_number: Option<String>,
		#[serde(default = "PlayDateMetadataAssets::<T>::default")]
		pub assets: PlayDateMetadataAssets<T>,
		#[serde(alias = "dev-assets", rename = "dev-assets")]
		pub dev_assets: Option<PlayDateMetadataAssets<T>>,
		#[serde(default)]
		pub options: Options,
		#[serde(default)]
		pub support: Support,
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


	#[derive(Deserialize, Debug, Clone, Default)]
	#[serde(deny_unknown_fields)]
	pub struct Options {
		pub assets: Option<AssetsOptions>,
		// This is temporary removed:
		// #[serde(alias = "dry-run", rename = "dry-run", default)]
		// pub dry_run: bool,
		// #[serde(alias = "cross-target", rename = "cross-target", default)]
		// pub cross_target: bool,
	}


	#[derive(Deserialize, Debug, Clone, Default)]
	#[serde(deny_unknown_fields)]
	pub struct AssetsOptions {
		#[serde(alias = "override", default = "bool::<true>")]
		pub overwrite: bool,

		#[serde(
		        alias = "follow-symlinks",
		        rename = "follow-symlinks",
		        default = "bool::<true>"
		)]
		pub follow_symlinks: bool,

		#[serde(alias = "build-method", default)]
		pub method: AssetsBuildMethod,

		/// Allow building assets for dependencies
		#[serde(default = "bool::<false>")]
		pub dependencies: bool,
	}

	#[derive(Deserialize, Debug, Clone, Copy)]
	#[serde(rename_all = "kebab-case")]
	pub enum AssetsBuildMethod {
		Copy,
		Link,
	}

	impl Default for AssetsBuildMethod {
		fn default() -> Self { Self::Link }
	}


	#[derive(Deserialize, Debug, Clone)]
	#[serde(bound(deserialize = "T: Deserialize<'de>"))]
	#[serde(untagged)]
	#[serde(deny_unknown_fields)]
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


	#[derive(Deserialize, Debug, Clone, Default)]
	pub struct Support {
		// #[serde(rename = "crank-manifest")]
		// #[serde(alias = "crank-manifest")]
		// pub crank_manifest: Option<CrankManifest>,
		// pub crank_manifest_rules: Option<Rules>,
	}

	const fn bool<const V: bool>() -> bool { V }
}
