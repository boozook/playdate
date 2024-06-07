use std::hash::Hash;
use std::borrow::Cow;
use std::path::Path;

use super::format::{AssetsOptions, AssetsRules, Ext, ExtraFields, ExtraValue, Manifest, Options, Support};
use super::format::ws::OptionsDefault;


pub trait PackageSource {
	type Authors: ?Sized + std::slice::Join<&'static str, Output = String>;
	type Metadata: MetadataSource;


	/// Crate name.
	fn name(&self) -> Cow<str>;
	/// Crate authors.
	fn authors(&self) -> &Self::Authors;
	// fn authors(&self) -> &[&str];
	/// Crate version (semver).
	fn version(&self) -> Cow<str>;
	/// Crate description.
	fn description(&self) -> Option<Cow<str>>;
	/// Crate metadata - `playdate` table.
	fn metadata(&self) -> Option<&Self::Metadata>;

	/// [`Options`] used as default.
	///
	/// Usually it may be from `workspace.metadata.playdate.options` or some external config,
	/// depends on implementation.
	///
	/// If this __and__ `metadata.options` is `None` - [`Options::default()`] is used.
	fn default_options(&self) -> Option<&OptionsDefault> { None }

	/// Names of `bin` cargo-targets.
	fn bins(&self) -> &[&str];
	/// Names of `example` cargo-targets.
	fn examples(&self) -> &[&str];

	/// Crate manifest path (Cargo.toml).
	fn manifest_path(&self) -> Cow<Path>;


	fn manifest_for_crate(&self) -> impl ManifestSourceOptExt {
		use super::format::Manifest;
		use std::slice::Join;

		let author = {
			let author = Join::join(self.authors(), ", ");
			if author.trim().is_empty() {
				None
			} else {
				Some(author.into())
			}
		};
		let version = Some(self.version());
		let package = Manifest { name: Some(self.name()),
		                         description: self.description(),
		                         author,
		                         version,
		                         bundle_id: None,
		                         image_path: None,
		                         launch_sound_path: None,
		                         content_warning: None,
		                         content_warning2: None,
		                         build_number: None };

		if let Some(meta) = self.metadata() {
			let manifest = meta.manifest();
			let base = Ext { main: package,
			                 extra: Default::default() };
			// TODO: Reduce coping, return associated type instead with all strings in the Cow<'self>.
			// Also get merged manifest with refs, using `override_with_extra_ref`
			let result = base.override_with_extra(manifest);
			Ext { main: Manifest::from(&result),
			      extra: result.iter_extra()
			                   .map(|m| {
				                   m.into_iter()
				                    .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()))
				                    .collect()
			                   })
			                   .unwrap_or_default() }
		} else {
			Ext { main: package.into_owned(),
			      extra: Default::default() }
		}
	}


	/// Returns `None` if manifest for `target` not found, no fallback.
	fn manifest_for(&self, target: &str, dev: bool) -> Option<Ext<Manifest<String>>> {
		let base = self.manifest_for_crate();

		if let Some(root) = self.metadata() {
			if dev {
				if let Some(man) = root.example(target) {
					Some(base.override_with_extra(man).into_owned())
				} else {
					log::debug!("target not found: {}", target);
					None
				}
			} else if let Some(man) = root.bin(target) {
				Some(base.override_with_extra(man).into_owned())
			} else {
				log::debug!("target not found: {}", target);
				None
			}
		} else {
			Some(base.into_owned())
		}
	}

	/// Returns manifest for specified `target`. If not found, returns manifest for crate.
	fn manifest_for_opt(&self, target: Option<&str>, dev: bool) -> Ext<Manifest<String>> {
		target.and_then(|target| self.manifest_for(target, dev))
		      .unwrap_or_else(|| self.manifest_for_crate().into_owned())
	}
}


pub trait MetadataSource {
	type S: Eq + Hash;
	type Manifest: ManifestSourceOptExt;
	type TargetManifest: ManifestSourceOptExt + TargetId;

	/// Main manifest, default and base for all cargo-targets.
	fn manifest(&self) -> &Self::Manifest;

	/// All manifests for "bin" cargo-targets.
	/// Overrides main manifest field-by-field.
	fn bins(&self) -> &[Self::TargetManifest];
	/// All manifests for "example" cargo-targets.
	/// Overrides main manifest field-by-field.
	fn examples(&self) -> &[Self::TargetManifest];

	/// Manifest for specified "bin" cargo-target.
	/// Overrides main manifest field-by-field.
	fn bin<'t>(&'t self, target: &'_ str) -> Option<&'t Self::TargetManifest> {
		self.bins().iter().find(|b| b.target() == target)
	}
	/// Manifest for specified "example" cargo-target.
	/// Overrides main manifest field-by-field.
	fn example<'t>(&'t self, target: &'_ str) -> Option<&'t Self::TargetManifest> {
		self.examples().iter().find(|b| b.target() == target)
	}

	fn bin_targets(&self) -> impl IntoIterator<Item = &str>;
	fn example_targets(&self) -> impl IntoIterator<Item = &str>;
	fn all_targets(&self) -> impl IntoIterator<Item = &str> {
		self.bin_targets().into_iter().chain(self.example_targets())
	}

	fn bins_iter(&self) -> Option<impl Iterator<Item = &Self::TargetManifest>> {
		(!self.bins().is_empty()).then_some(self.bins().iter())
	}
	fn examples_iter(&self) -> Option<impl Iterator<Item = &Self::TargetManifest>> {
		(!self.examples().is_empty()).then_some(self.examples().iter())
	}

	fn all_targets_iter(&self) -> impl Iterator<Item = &Self::TargetManifest> {
		self.bins_iter()
		    .into_iter()
		    .flatten()
		    .chain(self.examples_iter().into_iter().flatten())
	}

	fn assets(&self) -> &AssetsRules<Self::S>;
	fn dev_assets(&self) -> &AssetsRules<Self::S>;

	fn options(&self) -> &Options;
	fn assets_options(&self) -> Cow<'_, AssetsOptions>;

	fn support(&self) -> &Support;

	/// Make a manifest for a specific target, merged with base manifest for package.
	/// Returns `None` if the target is not found.
	fn manifest_for_target(&self, target: &str, dev: bool) -> Option<impl ManifestSourceOptExt> {
		// manifest() returns T without lifetime, so can't be associated with `&self`,
		// that should be fixed
		let base = self.manifest();

		if dev {
			if let Some(target) = self.example(target) {
				let trg = base.override_with_extra_ref(target);
				Some(trg.into_owned())
			} else {
				None
			}
		} else if let Some(target) = self.bin(target) {
			let trg = base.override_with_extra_ref(target);
			Some(trg.into_owned())
		} else {
			None
		}
	}

	fn manifest_for_target_any(&self, target: &str) -> Option<impl ManifestSourceOptExt> {
		self.manifest_for_target(target, false)
		    .or_else(|| self.manifest_for_target(target, true))
		    .map(|m| m.into_manifest())
		    .or_else(|| Some(Ext::<Manifest<String>>::from(self.manifest())))
	}
}


impl<T: MetadataSource> MetadataSource for &T {
	type S = <T as MetadataSource>::S;
	type Manifest = <T as MetadataSource>::Manifest;
	type TargetManifest = <T as MetadataSource>::TargetManifest;


	fn manifest(&self) -> &Self::Manifest { (*self).manifest() }

	fn bins(&self) -> &[Self::TargetManifest] { <T as MetadataSource>::bins(*self) }
	fn examples(&self) -> &[Self::TargetManifest] { <T as MetadataSource>::examples(*self) }

	fn bin_targets(&self) -> impl IntoIterator<Item = &str> { (*self).bin_targets() }
	fn example_targets(&self) -> impl IntoIterator<Item = &str> { (*self).example_targets() }

	fn assets(&self) -> &AssetsRules<Self::S> { (*self).assets() }
	fn dev_assets(&self) -> &AssetsRules<Self::S> { (*self).dev_assets() }
	fn options(&self) -> &Options { (*self).options() }
	fn assets_options(&self) -> Cow<'_, AssetsOptions> { (*self).assets_options() }
	fn support(&self) -> &Support { (*self).support() }
}


pub trait ManifestSource {
	fn name(&self) -> &str;
	fn version(&self) -> &str;
	fn author(&self) -> &str;
	fn bundle_id(&self) -> &str;
	fn description(&self) -> &str;
	fn image_path(&self) -> &str;
	fn launch_sound_path(&self) -> &str;
	fn content_warning(&self) -> &str;
	fn content_warning2(&self) -> &str;
	fn build_number(&self) -> Option<usize>;
}


pub trait ManifestSourceOpt {
	/// Possibly incomplete, that means that some of values could be `None`.
	const MAY_BE_INCOMPLETE: bool;

	fn name(&self) -> Option<&str>;
	fn version(&self) -> Option<&str>;
	fn author(&self) -> Option<&str>;
	fn bundle_id(&self) -> Option<&str>;
	fn description(&self) -> Option<&str>;
	fn image_path(&self) -> Option<&str>;
	fn launch_sound_path(&self) -> Option<&str>;
	fn content_warning(&self) -> Option<&str>;
	fn content_warning2(&self) -> Option<&str>;
	fn build_number(&self) -> Option<usize>;

	fn override_with<'a, Over>(&'a self, over: &'a Over) -> impl ManifestSourceOpt + 'a
		where Over: ManifestSourceOpt {
		use super::format::Manifest;

		Manifest::<Cow<str>> { name: over.name().or(self.name()).map(Into::into),
		                       version: over.version().or(self.version()).map(Into::into),
		                       author: over.author().or(self.author()).map(Into::into),
		                       bundle_id: over.bundle_id().or(self.bundle_id()).map(Into::into),
		                       description: over.description().or(self.description()).map(Into::into),
		                       image_path: over.image_path().or(self.image_path()).map(Into::into),
		                       launch_sound_path: over.launch_sound_path()
		                                              .or(self.launch_sound_path())
		                                              .map(Into::into),
		                       content_warning: over.content_warning().or(self.content_warning()).map(Into::into),
		                       content_warning2: over.content_warning2()
		                                             .or(self.content_warning2())
		                                             .map(Into::into),
		                       build_number: over.build_number().or(self.build_number()) }
	}
}


pub trait ManifestSourceOptExt: ManifestSourceOpt {
	const MAY_HAVE_EXTRA: bool;

	fn has_extra(&self) -> bool { Self::MAY_HAVE_EXTRA && self.iter_extra().is_some() }
	fn iter_extra(&self) -> Option<impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<ExtraValue>)>>;

	fn override_with_extra_ref<'t, Over>(&'t self, over: &'t Over) -> impl ManifestSourceOptExt + 't
		where Over: ManifestSourceOptExt {
		let manifest = self.override_with(over);
		let extra = if over.has_extra() || self.has_extra() {
			match (self.iter_extra(), over.iter_extra()) {
				(None, None) => None,
				(None, Some(extra)) => {
					let result = extra.into_iter()
					                  .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()));
					Some(result.collect())
				},
				(Some(extra), None) => {
					let result = extra.into_iter()
					                  .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()));
					Some(result.collect())
				},
				(Some(base), Some(extra)) => {
					let mut result: ExtraFields<ExtraValue> = base.into_iter()
					                                              .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone()))
					                                              .collect();
					result.extend(
					              extra.into_iter()
					                   .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().clone())),
					);
					Some(result)
				},
			}.unwrap_or_else(|| ExtraFields::with_capacity(0))
		} else {
			ExtraFields::with_capacity(0)
		};

		Ext { main: manifest,
		      extra }
	}


	fn override_with_extra<Over: ManifestSourceOptExt>(&self,
	                                                   overrider: &Over)
	                                                   -> impl ManifestSourceOptExt + Cob<'static> {
		self.override_with_extra_ref(overrider).into_manifest()
	}
}


impl<T: ManifestSource> ManifestSourceOpt for T {
	const MAY_BE_INCOMPLETE: bool = false;
	fn name(&self) -> Option<&str> { Some(ManifestSource::name(self)) }
	fn version(&self) -> Option<&str> { Some(ManifestSource::version(self)) }
	fn author(&self) -> Option<&str> { Some(ManifestSource::author(self)) }
	fn bundle_id(&self) -> Option<&str> { Some(ManifestSource::bundle_id(self)) }
	fn description(&self) -> Option<&str> { Some(ManifestSource::description(self)) }
	fn image_path(&self) -> Option<&str> { Some(ManifestSource::image_path(self)) }
	fn launch_sound_path(&self) -> Option<&str> { Some(ManifestSource::launch_sound_path(self)) }
	fn content_warning(&self) -> Option<&str> { Some(ManifestSource::content_warning(self)) }
	fn content_warning2(&self) -> Option<&str> { Some(ManifestSource::content_warning2(self)) }
	fn build_number(&self) -> Option<usize> { ManifestSource::build_number(self) }
}

impl<T: Clone + ManifestSourceOpt> ManifestSourceOpt for Cow<'_, T> {
	const MAY_BE_INCOMPLETE: bool = T::MAY_BE_INCOMPLETE;

	fn name(&self) -> Option<&str> { self.as_ref().name() }
	fn version(&self) -> Option<&str> { self.as_ref().version() }
	fn author(&self) -> Option<&str> { self.as_ref().author() }
	fn bundle_id(&self) -> Option<&str> { self.as_ref().bundle_id() }
	fn description(&self) -> Option<&str> { self.as_ref().description() }
	fn image_path(&self) -> Option<&str> { self.as_ref().image_path() }
	fn launch_sound_path(&self) -> Option<&str> { self.as_ref().launch_sound_path() }
	fn content_warning(&self) -> Option<&str> { self.as_ref().content_warning() }
	fn content_warning2(&self) -> Option<&str> { self.as_ref().content_warning2() }
	fn build_number(&self) -> Option<usize> { self.as_ref().build_number() }
}

impl<'t, T: ManifestSourceOptExt> ManifestSourceOptExt for &'t T where &'t T: ManifestSourceOpt {
	const MAY_HAVE_EXTRA: bool = true;

	fn iter_extra(&self) -> Option<impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<ExtraValue>)>> {
		(*self).iter_extra()
	}
}


pub trait TargetId {
	fn target(&self) -> &str;
}


pub trait IntoManifest: Sized + ManifestSourceOptExt {
	fn into_manifest(self) -> Ext<Manifest<String>> { self.into_owned() }
}
impl<T: ManifestSourceOptExt> IntoManifest for T {}


pub(super) trait IntoOwned<T> {
	fn into_owned(self) -> T;
}


/// Cob as CopyBorrow - partially copy, partially borrow.
/// Used to produce instance of type with internally borrowed things from `self`.
pub trait Cob<'t>
	where Self::Output: 't {
	type Output;
	fn as_borrow(&'t self) -> Self::Output;
}

impl<'t> Cob<'t> for str where Self: 't {
	type Output = Cow<'t, str>;
	fn as_borrow(&'t self) -> Self::Output { self.into() }
}

impl<'t, S: AsRef<str>> Cob<'t> for S {
	type Output = Cow<'t, str>;
	fn as_borrow(&'t self) -> Self::Output { self.as_ref().into() }
}


#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use crate::metadata::format::Manifest;
	use crate::metadata::format::Override;
	use crate::metadata::format::Metadata;
	use crate::metadata::format::MetadataInner;
	use super::*;


	// Default impl needed for tests only!
	impl<S: Default> Default for Manifest<S> {
		fn default() -> Self {
			Self { name: Default::default(),
			       version: Default::default(),
			       author: Default::default(),
			       bundle_id: Default::default(),
			       description: Default::default(),
			       image_path: Default::default(),
			       launch_sound_path: Default::default(),
			       content_warning: Default::default(),
			       content_warning2: Default::default(),
			       build_number: Default::default() }
		}
	}


	#[test]
	fn manifest_override() {
		let base = Manifest { name: Some("Name"),
		                      bundle_id: Some("dev.foo.bar"),
		                      ..Default::default() };

		let over = Manifest { name: Some("Over"),
		                      bundle_id: None,
		                      description: Some("description"),
		                      ..Default::default() };


		{
			let res = base.override_with(&over);
			assert_eq!(Some("Over"), res.name());
			assert_eq!(Some("dev.foo.bar"), res.bundle_id());
			assert_eq!(Some("description"), res.description());
		}

		{
			let res = base.override_with(&over);
			assert_eq!(Some("Over"), res.name());
			assert_eq!(Some("dev.foo.bar"), res.bundle_id());
			assert_eq!(Some("description"), res.description());
		}

		{
			let res = base.override_with_extra(&over);
			assert_eq!(Some("Over"), res.name());
			assert_eq!(Some("dev.foo.bar"), res.bundle_id());
			assert_eq!(Some("description"), res.description());
			assert!(res.iter_extra().is_none());
		}
	}


	#[test]
	fn manifest_override_ext() {
		let base = Manifest { name: Some("Name"),
		                      bundle_id: Some("dev.foo.bar"),
		                      ..Default::default() };

		let mut extra = ExtraFields::with_capacity(1);
		extra.insert("foo".into(), "bar".into());


		let base = Ext { main: base, extra };

		let over = Manifest { name: Some("Over"),
		                      bundle_id: None,
		                      description: Some("description"),
		                      ..Default::default() };


		{
			let res = base.override_with(&over);
			assert_eq!(Some("Over"), res.name());
			assert_eq!(Some("dev.foo.bar"), res.bundle_id());
			assert_eq!(Some("description"), res.description());
		}

		{
			let res = base.override_with(&over);
			assert_eq!(Some("Over"), res.name());
			assert_eq!(Some("dev.foo.bar"), res.bundle_id());
			assert_eq!(Some("description"), res.description());
		}

		{
			let res = base.override_with_extra(&over);
			assert_eq!(Some("Over"), res.name());
			assert_eq!(Some("dev.foo.bar"), res.bundle_id());
			assert_eq!(Some("description"), res.description());

			assert!(res.iter_extra().is_some());
			let (k, v) = res.iter_extra().unwrap().into_iter().next().unwrap();
			assert_eq!("foo", k.as_ref());
			assert_eq!(&ExtraValue::String("bar".into()), v.as_ref());
		}
	}


	struct CrateInfoNoMeta;
	impl PackageSource for CrateInfoNoMeta {
		type Authors = [&'static str];
		type Metadata = Metadata<String>;

		fn name(&self) -> Cow<str> { "Name".into() }
		fn authors(&self) -> &Self::Authors { &["John"] }
		fn version(&self) -> Cow<str> { "0.0.0".into() }
		fn description(&self) -> Option<Cow<str>> { None }
		fn bins(&self) -> &[&str] { &[SOME_TARGET] }
		fn examples(&self) -> &[&str] { &[] }
		fn metadata(&self) -> Option<&Self::Metadata> { None }

		fn manifest_path(&self) -> Cow<Path> { Cow::Borrowed(Path::new("Cargo.toml")) }
	}

	#[test]
	fn manifest_for_base() {
		let base = CrateInfoNoMeta.manifest_for_crate();
		let spec = CrateInfoNoMeta.manifest_for_opt("target".into(), false);
		let opt = CrateInfoNoMeta.manifest_for("target", false);
		assert_eq!(opt, Some(spec.to_owned()));
		assert_eq!(spec, base.into_owned());
	}


	struct CrateInfo(Metadata);
	impl CrateInfo {
		fn new() -> Self {
			let base = Manifest { name: Some("Meta Name"),
			                      bundle_id: Some("crate.id"),
			                      ..Default::default() };

			let mut extra = ExtraFields::with_capacity(1);
			extra.insert("foo".into(), "bar".into());
			assert!(!extra.is_empty());

			let manifest = Ext { main: base, extra }.into_owned();
			assert!(manifest.has_extra());

			let bins = {
				let base = Manifest { name: Some("Bin Name"),
				                      author: Some("Alex"),
				                      bundle_id: Some("bin.id"),
				                      description: Some("description"),
				                      ..Default::default() };

				let mut extra = ExtraFields::with_capacity(1);
				extra.insert("boo".into(), 42_usize.into());


				let manifest = Ext { main: base, extra }.into_owned();
				vec![Override { target: SOME_TARGET.to_owned(),
				                manifest }]
			};

			let meta = Metadata { inner: MetadataInner { manifest,
			                                             bins,
			                                             examples: vec![],
			                                             assets: Default::default(),
			                                             dev_assets: Default::default(),
			                                             options: Default::default(),
			                                             support: Default::default() } };

			Self(meta)
		}
	}

	impl PackageSource for CrateInfo {
		type Authors = [&'static str];
		type Metadata = Metadata<String>;

		fn name(&self) -> Cow<str> { "Crate Name".into() }
		fn authors(&self) -> &[&'static str] { &["John"] }
		fn version(&self) -> Cow<str> { "0.0.0".into() }
		fn description(&self) -> Option<Cow<str>> { None }

		fn bins(&self) -> &[&str] { &[SOME_TARGET] }
		fn examples(&self) -> &[&str] { &[] }

		fn manifest_path(&self) -> Cow<Path> { Cow::Borrowed(Path::new("Cargo.toml")) }

		fn metadata(&self) -> Option<&Self::Metadata> { Some(&self.0) }
	}

	const SOME_TARGET: &str = "some-target";

	#[test]
	fn manifest_for_crate() {
		let base_src = CrateInfo::new();
		let base = base_src.manifest_for_crate();
		assert_eq!(Some("Meta Name"), base.name());
		assert_eq!(Some("John"), base.author());
		assert_eq!(Some("0.0.0"), base.version());
		assert_eq!(Some("crate.id"), base.bundle_id());
		assert!(base.description().is_none());
		assert!(base.has_extra());
		let extra = base.iter_extra()
		                .unwrap()
		                .into_iter()
		                .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().to_owned()))
		                .collect::<HashMap<_, _>>();
		assert_eq!(1, extra.len());
		assert_eq!(Some(&"bar".into()), extra.get("foo"));
	}

	#[test]
	fn manifest_for_target_wrong_no_meta() {
		let spec = CrateInfoNoMeta.manifest_for_opt(Some("WRONG"), false);

		assert_eq!(Some("Name"), spec.name());
		assert_eq!(Some("John"), spec.author());
		assert_eq!(Some("0.0.0"), spec.version());
		assert!(spec.bundle_id().is_none());
	}

	#[test]
	fn manifest_for_target_wrong() {
		let base_src = CrateInfo::new();
		let base = base_src.manifest_for_crate();
		let spec = base_src.manifest_for_opt(Some("WRONG"), false);
		assert_eq!(Some("Meta Name"), spec.name());
		assert_eq!(Some("John"), spec.author());
		assert_eq!(Some("0.0.0"), spec.version());
		assert_eq!(Some("crate.id"), spec.bundle_id());
		assert_eq!(spec, base.into_owned());
	}

	#[test]
	fn manifest_for_target_bin() {
		let base_src = CrateInfo::new();
		let spec = base_src.manifest_for_opt(SOME_TARGET.into(), false);
		assert_eq!(Some("Bin Name"), spec.name());
		assert_eq!(Some("Alex"), spec.author());
		assert_eq!(Some("0.0.0"), spec.version());
		assert_eq!(Some("bin.id"), spec.bundle_id());
		assert_eq!(Some("description"), spec.description());
		let extra = spec.iter_extra()
		                .unwrap()
		                .into_iter()
		                .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().to_owned()))
		                .collect::<HashMap<_, _>>();
		assert_eq!(2, extra.len());
		assert_eq!(Some(&"bar".into()), extra.get("foo"));
		assert_eq!(Some(&42_usize.into()), extra.get("boo"));
	}
}
