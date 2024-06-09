use std::ops::Deref;
use std::cmp::Eq;
use std::hash::Hash;
use std::borrow::Cow;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer};

use super::source::*;


#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde",
           serde(bound(deserialize = "S: Deserialize<'de> + Eq + Hash")))]
pub struct CrateMetadata<S: Eq + Hash = String> {
	#[cfg_attr(feature = "serde", serde(rename = "playdate"))]
	pub inner: Option<Metadata<S>>,
}

/// Just ensure that `METADATA_FIELD` is not changed and something missed.
#[cfg(test)]
#[cfg_attr(test, test)]
fn eq_metadata_field() {
	assert_eq!(super::METADATA_FIELD, "playdate");
}


pub mod ws {
	#[derive(Debug)]
	#[cfg_attr(feature = "serde", derive(super::Deserialize))]
	pub struct WorkspaceMetadata {
		#[cfg_attr(feature = "serde", serde(rename = "playdate"))]
		pub inner: Option<Metadata>,
	}

	#[derive(Debug, Clone, PartialEq)]
	#[cfg_attr(feature = "serde", derive(super::Deserialize))]
	#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
	pub struct Metadata {
		pub options: Option<OptionsDefault>,
		pub support: Option<super::Support>,
	}

	#[derive(Debug, Clone, Default, PartialEq)]
	#[cfg_attr(feature = "serde", derive(super::Deserialize))]
	#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
	pub struct OptionsDefault {
		#[cfg_attr(feature = "serde", serde(default))]
		pub assets: super::AssetsOptions,
	}
}


/// Package Playdate Metadata, contains:
/// - Package Manifest fields
/// - Assets tables - `assets` & `dev-assets`
/// - Configuration table - `options`
#[derive(Debug, Clone, PartialEq)]

pub struct Metadata<S: Eq + Hash = String> {
	pub(super) inner: MetadataInner<S>,
}


#[cfg(feature = "serde")]
impl<'de, S: Deserialize<'de> + Eq + Hash> Deserialize<'de> for Metadata<S> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where D: Deserializer<'de> {
		let meta = MetadataInner::<S>::deserialize(deserializer)?;
		// here is should be some validation
		Ok(Self { inner: meta })
	}
}


impl<S> MetadataSource for Metadata<S>
	where S: Eq + Hash + AsRef<str>,
	      Override<S>: ManifestSourceOptExt,
	      Ext<Manifest<S>>: ManifestSourceOptExt,
	      for<'t> &'t Ext<Manifest<S>>: ManifestSourceOptExt
{
	type S = S;
	type Manifest = Ext<Manifest<S>>;
	type TargetManifest = Override<S>;


	fn manifest(&self) -> &Self::Manifest { &self.inner.manifest }

	fn bins(&self) -> &[Self::TargetManifest] { self.inner.bins.as_slice() }
	fn examples(&self) -> &[Self::TargetManifest] { self.inner.examples.as_slice() }

	fn bin_targets(&self) -> impl IntoIterator<Item = &str> { self.inner.bins.iter().map(|o| o.target.as_ref()) }
	fn example_targets(&self) -> impl IntoIterator<Item = &str> {
		self.inner.examples.iter().map(|o| o.target.as_ref())
	}

	fn assets(&self) -> &AssetsRules<S> { &self.inner.assets }
	fn dev_assets(&self) -> &AssetsRules<S> { &self.inner.dev_assets }


	fn options(&self) -> &Options { &self.inner.options }
	fn assets_options(&self) -> Cow<'_, AssetsOptions> { Cow::Borrowed(&self.options().assets) }

	fn support(&self) -> &Support { &self.inner.support }
}


/// Package Metadata, contains:
/// - Package Manifest fields
/// - Assets tables - `assets` & `dev-assets`
/// - Configuration table - `options`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde",
           serde(bound(deserialize = "S: Deserialize<'de> + Eq + Hash")))]
pub(super) struct MetadataInner<S: Eq + Hash = String> {
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub(super) manifest: Ext<Manifest<S>>,

	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(deserialize_with = "one_of::assets_rules"))]
	pub(super) assets: AssetsRules<S>,
	#[cfg_attr(feature = "serde", serde(default, alias = "dev-assets"))]
	#[cfg_attr(feature = "serde", serde(deserialize_with = "one_of::assets_rules"))]
	pub(super) dev_assets: AssetsRules<S>,

	#[cfg_attr(feature = "serde", serde(default))]
	pub(super) options: Options,
	#[cfg_attr(feature = "serde", serde(default))]
	pub(super) support: Support,

	#[cfg_attr(feature = "serde", serde(default, alias = "bin", rename = "bin"))]
	#[cfg_attr(feature = "serde", serde(deserialize_with = "one_of::targets_overrides"))]
	pub(super) bins: Vec<Override<S>>,
	#[cfg_attr(feature = "serde", serde(default, alias = "example", rename = "example"))]
	#[cfg_attr(feature = "serde", serde(deserialize_with = "one_of::targets_overrides"))]
	pub(super) examples: Vec<Override<S>>,
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "Main: Deserialize<'de>")))]
pub struct Ext<Main> {
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub(super) main: Main,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub(super) extra: ExtraFields<ExtraValue>,
}

impl<T> Ext<T> {
	pub fn new(main: T, extra: ExtraFields<ExtraValue>) -> Self { Self { main, extra } }
}

impl<T> Ext<T> {
	pub fn inner(&self) -> &T { &self.main }
	pub fn extra(&self) -> &ExtraFields<ExtraValue> { &self.extra }
}

impl<S> Ext<Manifest<S>> where S: ToOwned {
	pub fn clone_owned(&self) -> Ext<Manifest<<S as ToOwned>::Owned>> {
		Ext { main: self.main.clone_owned(),
		      extra: self.extra.to_owned() }
	}
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "S: Deserialize<'de>")))]
pub struct Manifest<S> {
	pub name: Option<S>,
	pub version: Option<S>,
	pub author: Option<S>,
	#[cfg_attr(feature = "serde", serde(alias = "bundle-id"))]
	pub bundle_id: Option<S>,
	pub description: Option<S>,
	#[cfg_attr(feature = "serde", serde(alias = "image-path"))]
	pub image_path: Option<S>,
	#[cfg_attr(feature = "serde", serde(alias = "launch-sound-path"))]
	pub launch_sound_path: Option<S>,
	#[cfg_attr(feature = "serde", serde(alias = "content-warning"))]
	pub content_warning: Option<S>,
	#[cfg_attr(feature = "serde", serde(alias = "content-warning2"))]
	pub content_warning2: Option<S>,
	#[cfg_attr(feature = "serde", serde(default, alias = "build-number"))]
	#[cfg_attr(feature = "serde", serde(deserialize_with = "one_of::usize_or_from_str"))]
	pub build_number: Option<usize>,
}


impl<'t, S> Cob<'t> for Manifest<S> where S: Cob<'t> {
	type Output = Manifest<<S as Cob<'t>>::Output>;

	fn as_borrow(&'t self) -> Self::Output {
		Manifest { name: self.name.as_ref().map(Cob::as_borrow),
		           version: self.version.as_ref().map(Cob::as_borrow),
		           author: self.author.as_ref().map(Cob::as_borrow),
		           bundle_id: self.bundle_id.as_ref().map(Cob::as_borrow),
		           description: self.description.as_ref().map(Cob::as_borrow),
		           image_path: self.image_path.as_ref().map(Cob::as_borrow),
		           launch_sound_path: self.launch_sound_path.as_ref().map(Cob::as_borrow),
		           content_warning: self.content_warning.as_ref().map(Cob::as_borrow),
		           content_warning2: self.content_warning2.as_ref().map(Cob::as_borrow),
		           build_number: self.build_number }
	}
}

impl<'t, T> Cob<'t> for Ext<T> where T: Cob<'t> {
	type Output = Ext<<T as Cob<'t>>::Output>;

	fn as_borrow(&'t self) -> Self::Output {
		let main = self.main.as_borrow();
		Ext { main,
		      extra: self.extra
		                 .iter()
		                 .map(|(k, v)| (k.to_owned(), v.to_owned()))
		                 .collect() }
	}
}

impl<'t, T> Cob<'t> for Override<T> where T: Cob<'t> {
	type Output = Override<<T as Cob<'t>>::Output>;

	fn as_borrow(&'t self) -> Self::Output {
		let Override { target, manifest } = self;
		Override { target: target.as_borrow(),
		           manifest: manifest.as_borrow() }
	}
}


impl IntoOwned<Manifest<<str as ToOwned>::Owned>> for Manifest<Cow<'_, str>> {
	fn into_owned(self) -> Manifest<<str as ToOwned>::Owned> {
		Manifest { name: self.name.map(|s| s.into_owned()),
		           version: self.version.map(|s| s.into_owned()),
		           author: self.author.map(|s| s.into_owned()),
		           bundle_id: self.bundle_id.map(|s| s.into_owned()),
		           description: self.description.map(|s| s.into_owned()),
		           image_path: self.image_path.map(|s| s.into_owned()),
		           launch_sound_path: self.launch_sound_path.map(|s| s.into_owned()),
		           content_warning: self.content_warning.map(|s| s.into_owned()),
		           content_warning2: self.content_warning2.map(|s| s.into_owned()),
		           build_number: self.build_number }
	}
}

impl<S> Manifest<S> where S: ToOwned {
	pub fn clone_owned(&self) -> Manifest<<S as ToOwned>::Owned> {
		Manifest { name: self.name.as_ref().map(|s| s.to_owned()),
		           version: self.version.as_ref().map(|s| s.to_owned()),
		           author: self.author.as_ref().map(|s| s.to_owned()),
		           bundle_id: self.bundle_id.as_ref().map(|s| s.to_owned()),
		           description: self.description.as_ref().map(|s| s.to_owned()),
		           image_path: self.image_path.as_ref().map(|s| s.to_owned()),
		           launch_sound_path: self.launch_sound_path.as_ref().map(|s| s.to_owned()),
		           content_warning: self.content_warning.as_ref().map(|s| s.to_owned()),
		           content_warning2: self.content_warning2.as_ref().map(|s| s.to_owned()),
		           build_number: self.build_number }
	}
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "S: Deserialize<'de>")))]
pub struct Override<S> {
	/// Associated cargo-target name
	#[cfg_attr(feature = "serde", serde(rename = "id", alias = "target"))]
	pub(super) target: S,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub(super) manifest: Ext<Manifest<S>>,
}

impl<S: AsRef<str>> Override<S> {
	pub fn into_parts(self) -> (S, Ext<Manifest<S>>) {
		let Override { target, manifest } = self;
		(target, manifest)
	}

	pub fn as_parts(&self) -> (&S, &Ext<Manifest<S>>) {
		let Override { target, manifest } = self;
		(target, manifest)
	}
}

impl<S: AsRef<str>> TargetId for Override<S> {
	fn target(&self) -> &str { self.target.as_ref() }
}


impl<'t> IntoOwned<Override<String>> for Override<Cow<'t, str>> {
	fn into_owned(self) -> Override<String> {
		Override { target: self.target.into_owned(),
		           manifest: self.manifest.into_owned() }
	}
}

impl<S> Override<S> where S: ToOwned {
	pub fn clone_owned(&self) -> Override<<S as ToOwned>::Owned> {
		Override { target: self.target.to_owned(),
		           manifest: self.manifest.clone_owned() }
	}
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, deny_unknown_fields))]
#[cfg_attr(feature = "serde",
           serde(bound(deserialize = "S: Deserialize<'de> + Default + Eq + Hash")))]
pub enum AssetsRules<S: Eq + Hash = String> {
	/// List of paths to include.
	List(Vec<S>),
	/// Rules & queries used to resolve paths to include.
	Map(HashMap<S, RuleValue>),
}

impl<S: Eq + Hash> Default for AssetsRules<S> {
	fn default() -> Self { Self::List(Vec::with_capacity(0)) }
}

impl<S: Eq + Hash> AssetsRules<S> {
	pub fn is_empty(&self) -> bool {
		match self {
			Self::List(list) => list.is_empty(),
			Self::Map(map) => map.is_empty(),
		}
	}
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RuleValue {
	Boolean(bool),
	String(String),
}

impl Default for RuleValue {
	fn default() -> Self { Self::Boolean(true) }
}


pub type ExtraFields<V> = HashMap<String, V>;


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExtraValue {
	Boolean(bool),
	String(String),
	Int(i64),
}

impl ExtraValue {
	pub fn is_empty(&self) -> bool {
		match self {
			Self::String(s) => s.trim().is_empty(),
			_ => false,
		}
	}
}

impl std::fmt::Display for ExtraValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Boolean(v) => v.fmt(f),
			Self::String(v) => v.trim().fmt(f),
			Self::Int(v) => v.fmt(f),
		}
	}
}

impl From<bool> for ExtraValue {
	fn from(value: bool) -> Self { Self::Boolean(value) }
}
impl From<i64> for ExtraValue {
	fn from(value: i64) -> Self { Self::Int(value) }
}
impl From<isize> for ExtraValue {
	fn from(value: isize) -> Self { Self::Int(value as _) }
}
impl From<u64> for ExtraValue {
	fn from(value: u64) -> Self { Self::Int(value as _) }
}
impl From<usize> for ExtraValue {
	fn from(value: usize) -> Self { Self::Int(value as _) }
}
impl From<String> for ExtraValue {
	fn from(value: String) -> Self { Self::String(value) }
}
impl From<&str> for ExtraValue {
	fn from(value: &str) -> Self { Self::String(value.to_string()) }
}
impl<'t> From<Cow<'t, str>> for ExtraValue {
	fn from(value: Cow<'t, str>) -> Self { Self::String(value.into_owned()) }
}

impl AsRef<ExtraValue> for ExtraValue {
	fn as_ref(&self) -> &ExtraValue { self }
}
impl AsMut<ExtraValue> for ExtraValue {
	fn as_mut(&mut self) -> &mut ExtraValue { self }
}


impl<S> ManifestSourceOpt for Manifest<S> where S: Deref<Target = str> {
	const MAY_BE_INCOMPLETE: bool = true;

	fn name(&self) -> Option<&str> { self.name.as_deref() }
	fn version(&self) -> Option<&str> { self.version.as_deref() }
	fn author(&self) -> Option<&str> { self.author.as_deref() }
	fn bundle_id(&self) -> Option<&str> { self.bundle_id.as_deref() }
	fn description(&self) -> Option<&str> { self.description.as_deref() }
	fn image_path(&self) -> Option<&str> { self.image_path.as_deref() }
	fn launch_sound_path(&self) -> Option<&str> { self.launch_sound_path.as_deref() }
	fn content_warning(&self) -> Option<&str> { self.content_warning.as_deref() }
	fn content_warning2(&self) -> Option<&str> { self.content_warning2.as_deref() }
	fn build_number(&self) -> Option<usize> { self.build_number }
}

impl<T: ManifestSourceOpt> ManifestSourceOpt for Ext<T> {
	const MAY_BE_INCOMPLETE: bool = Manifest::<String>::MAY_BE_INCOMPLETE;

	fn name(&self) -> Option<&str> { self.inner().name() }
	fn version(&self) -> Option<&str> { self.inner().version() }
	fn author(&self) -> Option<&str> { self.inner().author() }
	fn bundle_id(&self) -> Option<&str> { self.inner().bundle_id() }
	fn description(&self) -> Option<&str> { self.inner().description() }
	fn image_path(&self) -> Option<&str> { self.inner().image_path() }
	fn launch_sound_path(&self) -> Option<&str> { self.inner().launch_sound_path() }
	fn content_warning(&self) -> Option<&str> { self.inner().content_warning() }
	fn content_warning2(&self) -> Option<&str> { self.inner().content_warning2() }
	fn build_number(&self) -> Option<usize> { self.inner().build_number() }
}
impl<T: ManifestSourceOpt> ManifestSourceOpt for &Ext<T> {
	const MAY_BE_INCOMPLETE: bool = T::MAY_BE_INCOMPLETE;

	fn name(&self) -> Option<&str> { (*self).name() }
	fn version(&self) -> Option<&str> { (*self).version() }
	fn author(&self) -> Option<&str> { (*self).author() }
	fn bundle_id(&self) -> Option<&str> { (*self).bundle_id() }
	fn description(&self) -> Option<&str> { (*self).description() }
	fn image_path(&self) -> Option<&str> { (*self).image_path() }
	fn launch_sound_path(&self) -> Option<&str> { (*self).launch_sound_path() }
	fn content_warning(&self) -> Option<&str> { (*self).content_warning() }
	fn content_warning2(&self) -> Option<&str> { (*self).content_warning2() }
	fn build_number(&self) -> Option<usize> { (*self).build_number() }
}


impl<T: ManifestSourceOpt> ManifestSourceOptExt for Ext<T> {
	const MAY_HAVE_EXTRA: bool = true;

	fn has_extra(&self) -> bool { !self.extra.is_empty() }
	fn iter_extra(&self) -> Option<impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<ExtraValue>)>> {
		if self.extra.is_empty() {
			None
		} else {
			Some(self.extra.iter())
		}
	}
}

impl<S> ManifestSourceOptExt for Manifest<S> where S: Deref<Target = str> {
	const MAY_HAVE_EXTRA: bool = false;

	fn has_extra(&self) -> bool { false }
	fn iter_extra(&self) -> Option<impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<ExtraValue>)>> {
		None::<std::collections::hash_map::Iter<'_, &str, &ExtraValue>>
	}
}

impl<'s, T: ManifestSourceOpt, S: From<&'s str>> From<&'s T> for Manifest<S> {
	fn from(source: &'s T) -> Self {
		Self { name: source.name().map(Into::into),
		       version: source.version().map(Into::into),
		       author: source.author().map(Into::into),
		       bundle_id: source.bundle_id().map(Into::into),
		       description: source.description().map(Into::into),
		       image_path: source.image_path().map(Into::into),
		       launch_sound_path: source.launch_sound_path().map(Into::into),
		       content_warning: source.content_warning().map(Into::into),
		       content_warning2: source.content_warning2().map(Into::into),
		       build_number: source.build_number() }
	}
}


impl<T: ManifestSourceOptExt> From<&T> for Ext<Manifest<String>> {
	fn from(source: &T) -> Self {
		let main = Manifest::from(source);
		Ext { main,
		      extra: source.iter_extra()
		                   .map(|i| {
			                   i.into_iter()
			                    .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()))
			                    .collect()
		                   })
		                   .unwrap_or_default() }
	}
}

impl<'t, T: ManifestSourceOptExt> From<&'t T> for Ext<Manifest<Cow<'t, str>>> {
	fn from(source: &'t T) -> Self {
		Ext { main: Manifest::from(source),
		      extra: source.iter_extra()
		                   .map(|i| {
			                   i.into_iter()
			                    .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()))
			                    .collect()
		                   })
		                   .unwrap_or_default() }
	}
}

impl<'t, T: ManifestSourceOptExt + 't> IntoOwned<Ext<Manifest<String>>> for T {
	fn into_owned(self) -> Ext<Manifest<String>> {
		Ext { main: Manifest::from(&self).into_owned(),
		      extra: self.iter_extra()
		                 .map(|i| {
			                 i.into_iter()
			                  .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()))
			                  .collect()
		                 })
		                 .unwrap_or_default() }
	}
}


impl<S> ManifestSourceOpt for Override<S> where Manifest<S>: ManifestSourceOpt {
	const MAY_BE_INCOMPLETE: bool = Manifest::<S>::MAY_BE_INCOMPLETE;

	fn name(&self) -> Option<&str> { self.manifest.name() }
	fn version(&self) -> Option<&str> { self.manifest.version() }
	fn author(&self) -> Option<&str> { self.manifest.author() }
	fn bundle_id(&self) -> Option<&str> { self.manifest.bundle_id() }
	fn description(&self) -> Option<&str> { self.manifest.description() }
	fn image_path(&self) -> Option<&str> { self.manifest.image_path() }
	fn launch_sound_path(&self) -> Option<&str> { self.manifest.launch_sound_path() }
	fn content_warning(&self) -> Option<&str> { self.manifest.content_warning() }
	fn content_warning2(&self) -> Option<&str> { self.manifest.content_warning2() }
	fn build_number(&self) -> Option<usize> { self.manifest.build_number() }
}

impl<S> ManifestSourceOptExt for Override<S> where Manifest<S>: ManifestSourceOpt {
	const MAY_HAVE_EXTRA: bool = Ext::<Manifest<S>>::MAY_HAVE_EXTRA;

	fn iter_extra(&self) -> Option<impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<ExtraValue>)>> {
		self.manifest.iter_extra()
	}
}


#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Options {
	/// Use [`PackageSource::default_options`] as defaults for this.
	#[cfg_attr(feature = "serde", serde(default))]
	pub workspace: bool,
	#[cfg_attr(feature = "serde", serde(default))]
	pub assets: AssetsOptions,
	// Output layout ctrl, temporary removed.
}

impl Options {
	pub fn with_workspace(&self, def: Option<&ws::OptionsDefault>) -> Cow<'_, Options> {
		let merge_assets = |assets: &AssetsOptions| {
			if def.is_some() {
				log::debug!("merge options.assets with ws.defaults")
			}

			let overwrite = assets.overwrite
			                      .or_else(|| def.and_then(|d| d.assets.overwrite))
			                      .unwrap_or(AssetsOptions::default_overwrite());
			let follow_symlinks = assets.follow_symlinks
			                            .or_else(|| def.and_then(|d| d.assets.follow_symlinks))
			                            .unwrap_or(AssetsOptions::default_follow_symlinks());
			let method = assets.method
			                   .or_else(|| def.and_then(|d| d.assets.method))
			                   .unwrap_or_default();
			let dependencies = assets.dependencies
			                         .or_else(|| def.and_then(|d| d.assets.dependencies))
			                         .unwrap_or(AssetsOptions::default_dependencies());

			AssetsOptions { overwrite: Some(overwrite),
			                follow_symlinks: Some(follow_symlinks),
			                method: Some(method),
			                dependencies: Some(dependencies) }
		};


		if self.workspace {
			let res = Self { workspace: self.workspace,
			                 assets: merge_assets(&self.assets) };
			Cow::Owned(res)
		} else {
			Cow::Borrowed(self)
		}
	}
}


#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct AssetsOptions {
	#[cfg_attr(feature = "serde", serde(alias = "override"))]
	overwrite: Option<bool>,

	#[cfg_attr(feature = "serde", serde(alias = "follow-symlinks"))]
	follow_symlinks: Option<bool>,

	#[cfg_attr(feature = "serde", serde(alias = "build-method"))]
	method: Option<AssetsBuildMethod>,

	/// Allow building assets for dependencies
	dependencies: Option<bool>,
}

impl AssetsOptions {
	pub fn overwrite(&self) -> bool { self.overwrite.unwrap_or(Self::default_overwrite()) }
	pub fn dependencies(&self) -> bool { self.dependencies.unwrap_or(Self::default_dependencies()) }
	pub fn follow_symlinks(&self) -> bool { self.follow_symlinks.unwrap_or(Self::default_follow_symlinks()) }
	pub fn method(&self) -> AssetsBuildMethod { self.method.unwrap_or_default() }

	const fn default_overwrite() -> bool { true }
	const fn default_follow_symlinks() -> bool { true }
	const fn default_dependencies() -> bool { false }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AssetsBuildMethod {
	Copy,
	Link,
}

impl Default for AssetsBuildMethod {
	fn default() -> Self { Self::Link }
}


/// Compatibility options.
/// e.g. Crank manifest path.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Support {
	// #[serde(alias = "crank-manifest")]
	// pub crank_manifest: Option<PathBuf>
}


/// Because serde's error for untagged enum with various inner types is not pretty helpful,
/// like "data did not match any variant of untagged enum AssetsRules",
/// we need some custom implementations with more detailed error messages.
#[cfg(feature = "serde")]
mod one_of {
	use std::marker::PhantomData;

	use std::fmt;
	use serde::de;
	use serde::de::MapAccess;
	use serde::de::SeqAccess;
	use serde::de::Visitor;

	use super::*;


	pub fn usize_or_from_str<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
		where D: Deserializer<'de> {
		struct OneOf;

		impl<'de> Visitor<'de> for OneOf {
			type Value = Option<usize>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("unsigned integer or string with it")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> { Ok(Some(v as _)) }
			fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> { Ok(Some(v as _)) }
			fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> { Ok(Some(v as _)) }

			fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
				Ok(Some(v.try_into().map_err(de::Error::custom)?))
			}

			fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
				Ok(Some(v.try_into().map_err(de::Error::custom)?))
			}

			fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
				if v.is_negative() {
					Err(de::Error::invalid_type(de::Unexpected::Signed(v), &self))
				} else {
					Ok(Some(v.try_into().map_err(de::Error::custom)?))
				}
			}

			fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
				let v = s.parse().map_err(serde::de::Error::custom)?;
				Ok(Some(v))
			}
		}

		deserializer.deserialize_any(OneOf)
	}


	pub fn assets_rules<'de, S, D>(deserializer: D) -> Result<super::AssetsRules<S>, D::Error>
		where D: Deserializer<'de>,
		      S: Deserialize<'de> + Eq + Hash {
		struct OneOf<S>(PhantomData<S>);

		impl<'de, S> Visitor<'de> for OneOf<S> where S: Deserialize<'de> + Eq + Hash {
			type Value = super::AssetsRules<S>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("list of includes or map of rules")
			}

			fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
				let deserializer = de::value::SeqAccessDeserializer::new(seq);
				let res: Vec<S> = Deserialize::deserialize(deserializer)?;
				Ok(super::AssetsRules::List(res))
			}

			fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
				let deserializer = de::value::MapAccessDeserializer::new(map);
				let res: HashMap<S, super::RuleValue> = Deserialize::deserialize(deserializer)?;
				Ok(super::AssetsRules::Map(res))
			}
		}

		deserializer.deserialize_any(OneOf::<S>(PhantomData))
	}


	pub fn targets_overrides<'de, S, D>(deserializer: D) -> Result<Vec<super::Override<S>>, D::Error>
		where D: Deserializer<'de>,
		      S: Deserialize<'de> + Eq + Hash {
		struct OneOf<S>(PhantomData<S>);

		impl<'de, S> Visitor<'de> for OneOf<S> where S: Deserialize<'de> + Eq + Hash {
			type Value = Vec<super::Override<S>>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("list of includes or map of rules")
			}

			fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
				let deserializer = de::value::SeqAccessDeserializer::new(seq);
				Deserialize::deserialize(deserializer)
			}

			fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
				use super::{Ext, Manifest};

				let deserializer = de::value::MapAccessDeserializer::new(map);
				let res: HashMap<S, Ext<Manifest<S>>> = Deserialize::deserialize(deserializer)?;
				Ok(res.into_iter()
				      .map(|(k, v)| {
					      Override::<S> { target: k,
					                      manifest: v }
				      })
				      .collect())
			}
		}

		deserializer.deserialize_any(OneOf::<S>(PhantomData))
	}
}


#[cfg(test)]
#[cfg(feature = "toml")]
mod tests {
	use super::*;
	use crate::manifest::format::ManifestFmt;

	use std::assert_matches::assert_matches;


	type ManifestWithAny = Ext<Manifest<String>>;
	type ManifestStrict = Manifest<String>;
	type ManifestStrictRef<'t> = Manifest<Cow<'t, str>>;


	#[test]
	fn minimal_strict() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		          "#;
		let m: ManifestStrict = toml::from_str(src).unwrap();
		assert!(m.bundle_id.is_some());
		let m: ManifestStrictRef = toml::from_str(src).unwrap();
		assert!(m.bundle_id.is_some());
	}

	#[test]
	fn minimal_strict_err() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             foo = "bar"
		          "#;
		assert!(toml::from_str::<ManifestStrict>(src).is_err());

		let src = r#"foo = "bar""#;
		assert!(toml::from_str::<ManifestStrict>(src).is_err());
		assert!(toml::from_str::<ManifestStrictRef>(src).is_err());
	}

	#[test]
	fn minimal_extra() {
		let src = r#"bundle-id = "test.workspace.main.crate""#;
		assert!(toml::from_str::<ManifestWithAny>(src).is_ok());


		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             foo = "bar"
		          "#;

		let m: ManifestWithAny = toml::from_str(src).unwrap();

		assert!(m.inner().bundle_id.is_some());
		assert!(m.inner().description.is_some());
		assert!(m.extra().get("foo").is_some());
	}

	#[test]
	fn meta_minimal() {
		assert!(toml::from_str::<Metadata>("").is_ok());

		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();
		assert!(m.inner.manifest.inner().bundle_id.is_some());
		assert!(m.inner.manifest.inner().description.is_some());
		assert!(m.inner.manifest.extra.is_empty());
	}


	#[test]
	fn meta_extra() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             boo = 42
		             [assets]
		             foo = "bar"
		          "#;
		let expected_id = Some("test.workspace.main.crate");

		let m: super::MetadataInner = toml::from_str(src).unwrap();
		assert_eq!(expected_id, m.manifest.inner().bundle_id.as_deref());
		assert!(m.manifest.inner().description.is_some());
		assert!(m.manifest.extra().get("boo").is_some());

		let m: Metadata = toml::from_str(src).unwrap();
		assert_eq!(expected_id, m.inner.manifest.inner().bundle_id.as_deref());
		assert!(m.inner.manifest.inner().description.is_some());
		assert!(m.inner.manifest.extra().get("boo").is_some());

		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             foo = "bar"
		             assets.target = "source"
		          "#;
		let m: Metadata = toml::from_str(src).unwrap();
		assert_eq!(expected_id, m.inner.manifest.inner().bundle_id.as_deref());
		assert!(m.inner.manifest.inner().description.is_some());
		assert!(m.inner.manifest.extra().get("foo").is_some());
		assert!(!m.inner.assets.is_empty());
	}


	#[test]
	fn meta_strict_bins() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             [[bin]]
		             target = "cargo-target-name"
		             name = "Other Name"
		             [[bin]]
		             target = "cargo-another-target"
		             name = "Another Name"
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();
		assert!(m.inner.manifest.inner().bundle_id.is_some());
		assert!(m.inner.manifest.inner().description.is_some());
		assert_eq!(2, m.inner.bins.len());
	}

	#[test]
	fn meta_extra_bins() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             foo = "bar"

		             [[bin]]
		             target = "cargo-target-name"
		             name = "Other Name"
		             boo = "bar"
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();
		assert!(m.inner.manifest.inner().bundle_id.is_some());
		assert!(m.inner.manifest.extra().get("foo").is_some());
		assert_eq!(1, m.inner.bins.len());
		assert!(
		        m.inner
		         .bins
		         .first()
		         .unwrap()
		         .manifest
		         .extra()
		         .get("boo")
		         .is_some()
		);
	}

	#[test]
	fn meta_strict_examples() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             [[example]]
		             target = "cargo-target-name"
		             name = "Other Name"
		             [[example]]
		             target = "cargo-another-target"
		             name = "Another Name"
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();
		assert!(m.inner.manifest.inner().bundle_id.is_some());
		assert!(m.inner.manifest.inner().description.is_some());
		assert_eq!(2, m.inner.examples.len());
	}

	#[test]
	fn meta_strict_examples_map() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             [example.cargo-target-name]
		             name = "Other Name"
		             [example.cargo-another-target]
		             name = "Another Name"
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();
		assert!(m.inner.manifest.inner().bundle_id.is_some());
		assert!(m.inner.manifest.inner().description.is_some());
		assert_eq!(2, m.inner.examples.len());
	}

	#[test]
	fn meta_strict_examples_mix_err() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             description = "test"
		             [example.cargo-target-name]
		             name = "Other Name"
		             [[example]]
		             target = "cargo-another-target"
		             name = "Another Name"
		          "#;

		assert!(toml::from_str::<Metadata>(src).is_err());
	}

	#[test]
	fn meta_extra_examples_mix_err() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             foo = "bar"
		             [example.cargo-target-name]
		             name = "Other Name"
		             [[example]]
		             target = "cargo-another-target"
		             name = "Another Name"
		          "#;

		assert!(toml::from_str::<Metadata>(src).is_err());
	}


	#[test]
	fn assets_num_err() {
		let src = r#"
		             [playdate]
		             bundle-id = "test.workspace.main.crate"
						 [playdate.assets]
						 foo = "bar" # ok
						 num = 42 # err
		          "#;
		assert!(toml::from_str::<CrateMetadata>(src).is_err());
	}


	#[test]
	fn options_empty() {
		let m = toml::from_str::<Options>("").unwrap();
		assert_eq!(Options::default(), m);
	}

	#[test]
	fn options_assets_deps() {
		// default is false
		assert!(!AssetsOptions::default_dependencies());
		let src = r#" [assets] "#;
		let m = toml::from_str::<Options>(src).unwrap();
		assert_matches!(
		                m.assets,
		                AssetsOptions { dependencies: None,
		                                .. }
		);

		// overrides default
		let src = r#"
		             [assets]
		             dependencies = true
		          "#;
		let m = toml::from_str::<Options>(src).unwrap();
		assert_matches!(
		                m.assets,
		                AssetsOptions { dependencies: Some(true),
		                                .. }
		);
	}

	#[test]
	fn assets_rules_empty() {
		let m = toml::from_str::<AssetsRules>("").unwrap();
		assert!(m.is_empty());
		match m {
			AssetsRules::List(rules) => assert!(rules.is_empty()),
			AssetsRules::Map(rules) => assert!(rules.is_empty()),
		}
	}

	#[test]
	fn assets_rules_list_wrapped() {
		#[derive(Debug, Clone, PartialEq, Deserialize)]
		pub(super) struct Temp {
			assets: AssetsRules,
		}

		let src = r#"
		             assets = ["one", "two"]
		          "#;
		let m = toml::from_str::<Temp>(src).unwrap();
		assert!(!m.assets.is_empty());
		assert_matches!(m.assets, AssetsRules::List(rules) if rules.len() == 2);
	}

	#[test]
	fn assets_rules_map() {
		let src = r#"
		             included = true
		             excluded = false
		             "into/" = "files.*"
		          "#;
		let m = toml::from_str::<AssetsRules>(src).unwrap();
		assert_matches!(m, AssetsRules::Map(rules) if rules.len() == 3);
	}


	#[test]
	fn assets_rules_map_wrapped() {
		#[derive(Debug, Clone, PartialEq, Deserialize)]
		pub(super) struct Temp {
			assets: AssetsRules,
		}
		let src = r#"
		             [assets]
		             included = true
		             excluded = false
		             "into/" = "files.*"
		          "#;
		let m = toml::from_str::<Temp>(src).unwrap();
		assert_matches!(m.assets, AssetsRules::Map(rules) if rules.len() == 3);
	}


	#[test]
	fn options_assets_err() {
		let src = r#"
		             [playdate]
		             bundle-id = "test.workspace.main.crate"
		             [playdate.options.assets]
		             foo = "bar" # err
		          "#;
		let result = toml::from_str::<CrateMetadata>(src);
		assert!(result.is_err(), "must be err, but {result:?}");
		assert!(result.as_ref()
		              .unwrap_err()
		              .to_string()
		              .contains("unknown field `foo`"));
	}

	#[test]
	fn assets_options_err() {
		let src = r#"
		             [playdate]
		             bundle-id = "test.workspace.main.crate"
		             [playdate.assets]
		             foo = "bar"
		             options = { dependencies = true }
		          "#;
		let result = toml::from_str::<CrateMetadata>(src);
		assert!(result.is_err(), "must be err, but {result:?}");

		let src = r#"
		             [playdate]
		             bundle-id = "test.workspace.main.crate"
		             [playdate.assets.options]
		             dependencies = true
		          "#;
		let result = toml::from_str::<CrateMetadata>(src);
		assert!(result.is_err(), "must be err, but {result:?}");
	}

	#[test]
	fn meta_assets_options() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [options.assets]
		             [assets]
		          "#;
		assert!(toml::from_str::<Metadata>(src).is_ok());

		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [options.assets]
		             dependencies = true
		          "#;
		let m = toml::from_str::<MetadataInner>(src).unwrap();
		assert!(m.assets.is_empty());
		assert_matches!(
		                m.options.assets,
		                AssetsOptions { dependencies: Some(true),
		                                .. }
		);
	}

	#[test]
	fn meta_assets_options_legacy() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [assets]
		             options = {}
		          "#;
		assert!(toml::from_str::<Metadata>(src).is_err());

		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [assets]
		             options = { dependencies = true }
		          "#;
		assert!(toml::from_str::<Metadata>(src).is_err());

		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [assets]
						 foo = "bar"
						 boo = true
		             options = { }
		          "#;
		assert!(toml::from_str::<Metadata>(src).is_err());


		let src = r#"
		             [playdate]
		             bundle-id = "test.workspace.main.crate"
		             [playdate.assets]
		             [playdate.assets.options] # err
		          "#;
		assert!(toml::from_str::<CrateMetadata>(src).is_err());
	}

	#[test]
	fn meta_options_assets() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [options]
		             assets = {}
		          "#;

		assert!(toml::from_str::<Metadata>(src).is_ok());
	}

	#[test]
	fn meta_assets_options_mix() {
		let src = r#"
		             bundle-id = "test.workspace.main.crate"
		             [options]
		             assets = {}
		             [assets]
		             options = {}
		          "#;

		assert!(toml::from_str::<Metadata>(src).is_err());
	}


	#[test]
	fn meta_assets_maps() {
		let src = r#"
		             [assets]
		             included = true
		             excluded = false
		             other = "from/path"
		             [dev-assets]
		             a = true
		             b = false
		             c = "/c/path"
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();

		assert_matches!(m.assets(), AssetsRules::Map(_));
		match m.assets() {
			AssetsRules::Map(rules) => {
				assert_eq!(3, rules.len());
				assert_eq!(Some(&RuleValue::Boolean(true)), rules.get("included"));
				assert_eq!(Some(&RuleValue::Boolean(false)), rules.get("excluded"));
				assert_eq!(Some(&RuleValue::String("from/path".into())), rules.get("other"));
			},
			_ => unreachable!(),
		}

		assert_matches!(m.dev_assets(), AssetsRules::Map(_));
		match m.dev_assets() {
			AssetsRules::Map(rules) => {
				assert_eq!(3, rules.len());
				assert_eq!(Some(&RuleValue::Boolean(true)), rules.get("a"));
				assert_eq!(Some(&RuleValue::Boolean(false)), rules.get("b"));
				assert_eq!(Some(&RuleValue::String("/c/path".into())), rules.get("c"));
			},
			_ => unreachable!(),
		}
	}

	#[test]
	fn meta_assets_lists() {
		let src = r#"
		             assets = ["a", "b", "c"]
		             dev-assets = ["d", "e", "f"]
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();

		assert_matches!(m.assets(), AssetsRules::List(_));
		assert_matches!(m.dev_assets(), AssetsRules::List(_));
		match m.assets() {
			AssetsRules::List(rules) => assert_eq!(&["a", "b", "c"], &rules[..]),
			_ => unreachable!(),
		}
		match m.dev_assets() {
			AssetsRules::List(rules) => assert_eq!(&["d", "e", "f"], &rules[..]),
			_ => unreachable!(),
		}
	}

	#[test]
	fn meta_assets_mix() {
		let src = r#"
		             assets = ["d", "e", "f"]
		             [dev-assets]
		             a = true
		             b = true
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();

		assert_matches!(m.assets(), AssetsRules::List(_));
		match m.assets() {
			AssetsRules::List(rules) => {
				assert_eq!(3, rules.len());
				assert_eq!(&["d", "e", "f"], &rules[..]);
			},
			_ => unreachable!(),
		}

		assert_matches!(m.dev_assets(), AssetsRules::Map(_));
		match m.dev_assets() {
			AssetsRules::Map(rules) => {
				assert_eq!(2, rules.len());
				assert_eq!(Some(&RuleValue::Boolean(true)), rules.get("a"));
				assert_eq!(Some(&RuleValue::Boolean(true)), rules.get("b"));
			},
			_ => unreachable!(),
		}
	}


	#[test]
	fn meta_full() {
		let src = r#"
		             foo = "bar" # custom field
		             name = "Crate Name"
		             version = "0.1"
		             bundle-id = "test.workspace.main.crate"
		             description = "Crate description"
		             author = "Crate Author"
		             image-path = "image/path"
		             launch-sound-path = "launch-sound/path"
		             content-warning = "Attention!"
		             content-warning2 = "Alarm!"
		             build-number = 42
		             options.assets.dependencies = true
		             [assets]
		             included = true
		             excluded = false
		             other = "from/path"
		             [dev-assets]
		             "dev-included" = true
		             [[bin]]
		             target = "cargo-target-bin-name"
		             name = "Bin Name"
		             bundle-id = "test.workspace.main.bin"
		             description = "This is a bin"
		             [[example]]
		             target = "cargo-target-example-name"
		             name = "Example Name"
		             bundle-id = "test.workspace.main.example"
		             description = "This is an example"
		             example-extra = 101
		          "#;

		let m = toml::from_str::<Metadata>(src).unwrap();
		assert_eq!(Some("Crate Name"), m.manifest().name());
		assert_eq!(Some("0.1"), m.manifest().version());
		assert_eq!(Some("test.workspace.main.crate"), m.manifest().bundle_id());
		assert_eq!(Some("Crate description"), m.manifest().description());
		assert_eq!(Some("Crate Author"), m.manifest().author());
		assert_eq!(Some("image/path"), m.manifest().image_path());
		assert_eq!(Some("launch-sound/path"), m.manifest().launch_sound_path());
		assert_eq!(Some("Attention!"), m.manifest().content_warning());
		assert_eq!(Some("Alarm!"), m.manifest().content_warning2());

		{
			let s = m.manifest().to_manifest_string().unwrap();
			println!("meta manifest:\n{}", s.trim())
		}


		let opts = m.assets_options();
		assert!(opts.dependencies());
		assert!(!AssetsOptions::default_dependencies());

		assert_matches!(m.assets(), AssetsRules::Map(_));
		match m.assets() {
			AssetsRules::Map(rules) => {
				assert_eq!(3, rules.len());
				assert_eq!(Some(&RuleValue::Boolean(true)), rules.get("included"));
				assert_eq!(Some(&RuleValue::Boolean(false)), rules.get("excluded"));
				assert_eq!(Some(&RuleValue::String("from/path".into())), rules.get("other"));
			},
			_ => unreachable!(),
		}
		assert_matches!(m.dev_assets(), AssetsRules::Map(rules) if rules.get("dev-included").is_some());

		assert_eq!(1, m.bins().len());
		assert_eq!(1, m.examples().len());

		let bin_trg = m.bin_targets().into_iter().next().unwrap();
		assert_eq!("cargo-target-bin-name", bin_trg);

		let example_trg = m.example_targets().into_iter().next().unwrap();
		assert_eq!("cargo-target-example-name", example_trg);

		let (bin_trg_by_iter, bin) = m.bins_iter().and_then(|mut i| i.next()).unwrap().as_parts();
		assert_eq!(bin_trg, bin_trg_by_iter);

		let (example_trg_by_iter, example) = m.examples_iter().and_then(|mut i| i.next()).unwrap().as_parts();
		assert_eq!(example_trg, example_trg_by_iter);


		assert_eq!(Some("Bin Name"), bin.name());
		assert_eq!(Some("test.workspace.main.bin"), bin.bundle_id());
		assert_eq!(Some("This is a bin"), bin.description());
		assert!(bin.version().is_none());
		assert!(bin.author().is_none());
		assert!(bin.image_path().is_none());
		assert!(bin.launch_sound_path().is_none());
		assert!(bin.content_warning().is_none());
		assert!(bin.content_warning2().is_none());
		assert!(!bin.has_extra());

		{
			let s = bin.to_manifest_string().unwrap();
			println!("bin over:\n{}", s.trim())
		}


		assert_eq!(Some("Example Name"), example.name());
		assert_eq!(Some("test.workspace.main.example"), example.bundle_id());
		assert_eq!(Some("This is an example"), example.description());
		assert!(example.version().is_none());
		assert!(example.author().is_none());
		assert!(example.image_path().is_none());
		assert!(example.launch_sound_path().is_none());
		assert!(example.content_warning().is_none());
		assert!(example.content_warning2().is_none());
		assert!(example.has_extra());
		let example_extra: HashMap<_, _> = example.iter_extra()
		                                          .unwrap()
		                                          .into_iter()
		                                          .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().to_owned()))
		                                          .collect();
		assert_eq!(1, example_extra.len());
		assert_eq!(Some(&ExtraValue::Int(101)), example_extra.get("example-extra"));


		{
			let s = example.to_manifest_string().unwrap();
			println!("example over:\n{}", s.trim())
		}


		// test merged

		let bin = m.manifest_for_target(bin_trg, false).unwrap();
		assert_eq!(Some("Bin Name"), bin.name());
		assert_eq!(Some("0.1"), bin.version());
		assert_eq!(Some("test.workspace.main.bin"), bin.bundle_id());
		assert_eq!(Some("This is a bin"), bin.description());
		assert_eq!(Some("Crate Author"), bin.author());
		assert_eq!(Some("image/path"), bin.image_path());
		assert_eq!(Some("launch-sound/path"), bin.launch_sound_path());
		assert_eq!(Some("Attention!"), bin.content_warning());
		assert_eq!(Some("Alarm!"), bin.content_warning2());
		{
			let s = bin.to_manifest_string().unwrap();
			println!("bin manifest:\n{}", s.trim())
		}

		let example = m.manifest_for_target(example_trg, true).unwrap();
		assert_eq!(Some("Example Name"), example.name());
		assert_eq!(Some("0.1"), example.version());
		assert_eq!(Some("test.workspace.main.example"), example.bundle_id());
		assert_eq!(Some("This is an example"), example.description());
		assert_eq!(Some("Crate Author"), example.author());
		assert_eq!(Some("image/path"), example.image_path());
		assert_eq!(Some("launch-sound/path"), example.launch_sound_path());
		assert_eq!(Some("Attention!"), example.content_warning());
		assert_eq!(Some("Alarm!"), example.content_warning2());
		{
			let s = example.to_manifest_string().unwrap();
			println!("example manifest:\n{}", s.trim())
		}


		// test merged any kind of target, just named

		let example = m.manifest_for_target_any(example_trg).unwrap();
		assert_eq!(Some("Example Name"), example.name());
		assert_eq!(Some("0.1"), example.version());
		assert_eq!(Some("test.workspace.main.example"), example.bundle_id());
		assert_eq!(Some("This is an example"), example.description());
		assert_eq!(Some("Crate Author"), example.author());
		assert_eq!(Some("image/path"), example.image_path());
		assert_eq!(Some("launch-sound/path"), example.launch_sound_path());
		assert_eq!(Some("Attention!"), example.content_warning());
		assert_eq!(Some("Alarm!"), example.content_warning2());
		{
			let s = example.to_manifest_string().unwrap();
			println!("example manifest:\n{}", s.trim())
		}

		let missing = m.manifest_for_target_any("missing, wrong name").unwrap();
		assert_eq!(Some("Crate Name"), missing.name());
		assert_eq!(Some("0.1"), missing.version());
		assert_eq!(Some("test.workspace.main.crate"), missing.bundle_id());
		assert_eq!(Some("Crate description"), missing.description());
		assert_eq!(Some("Crate Author"), missing.author());
		assert_eq!(Some("image/path"), missing.image_path());
		assert_eq!(Some("launch-sound/path"), missing.launch_sound_path());
		assert_eq!(Some("Attention!"), missing.content_warning());
		assert_eq!(Some("Alarm!"), missing.content_warning2());
		{
			let s = missing.to_manifest_string().unwrap();
			println!("missing (base meta) manifest:\n{}", s.trim())
		}
		assert_eq!(m.manifest().into_owned(), missing.into_owned());
	}
}
