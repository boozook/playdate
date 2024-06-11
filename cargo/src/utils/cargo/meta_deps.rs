use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use cargo::core::compiler::CompileMode;
use cargo::core::{PackageId, PackageIdSpecQuery};
use cargo::util::interning::InternedString;
use cargo::CargoResult;
use playdate::manifest::PackageSource;
use playdate::metadata::format::ws::WorkspaceMetadata;
use playdate::metadata::source::MetadataSource;
use playdate::metadata::format::Metadata as MainMetadata;
use serde::Deserialize;
use serde::de::IntoDeserializer;

use crate::config::Config;
use crate::logger::LogErr;

use super::build_plan::format::TargetKind;
use super::build_plan::TargetKindWild;
use super::metadata::format::{Package, CrateMetadata};
use super::metadata::CargoMetadataPd;
use super::unit_graph::format::{Unit, UnitTarget};
use super::unit_graph::format::UnitGraph;


pub fn meta_deps<'cfg>(cfg: &'cfg Config<'cfg>) -> CargoResult<MetaDeps<'cfg>> {
	let units = cfg.unit_graph()?;
	let meta = cfg.metadata()?;
	Ok(MetaDeps::new(units, meta))
}


pub struct MetaDeps<'cfg> {
	units: &'cfg UnitGraph,
	meta: &'cfg CargoMetadataPd,
	roots: Vec<RootNode<'cfg>>,
}

#[derive(Debug, Clone)]
pub struct RootNode<'cfg> {
	node: Node<'cfg>,
	deps: Vec<Node<'cfg>>,

	ws: Option<&'cfg WorkspaceMetadata>,
}

impl<'t> RootNode<'t> {
	pub fn package_id(&self) -> &'t PackageId { self.node.package_id() }

	pub fn node(&self) -> &Node<'t> { &self.node }

	/// Dependencies _with assets_ of the root node,
	/// in topological order,
	/// including the root node if it has assets.
	pub fn deps(&self) -> &[Node<'t>] { &self.deps }
}


#[derive(Debug, Clone, Copy)]
pub struct Node<'cfg> {
	meta: Option<&'cfg Package<CrateMetadata<InternedString>>>,
	unit: &'cfg Unit,
}

impl<'t> Node<'t> {
	pub fn package_id(&self) -> &'t PackageId { &self.unit.package_id }

	pub fn unit(&self) -> &'t Unit { self.unit }
	pub fn meta(&self) -> Option<&'t Package<CrateMetadata<InternedString>>> { self.meta }
	pub fn target(&self) -> &'t UnitTarget { &self.unit.target }

	pub fn manifest_path(&self) -> Option<&'t Path> { self.meta.as_ref().map(|m| m.manifest_path.as_path()) }
}


impl<'t> MetaDeps<'t> {
	pub fn new(units: &'t UnitGraph, meta: &'t CargoMetadataPd) -> Self {
		let mode_is_build = |u: &&Unit| matches!(u.mode, CompileMode::Build);
		let is_norm_tk = |u: &&Unit| {
			matches!(
			         u.target.kind,
			         TargetKind::Lib(_) | TargetKind::Bin | TargetKind::Example
			)
		};
		let is_sub_tk = |u: &&Unit| matches!(u.target.kind, TargetKind::Lib(_));

		let ws = meta.workspace_metadata.as_ref();

		let mut roots = units.roots
		                     .iter()
		                     .map(|i| &units.units[*i])
		                     .filter(mode_is_build)
		                     .filter(is_norm_tk)
		                     .map(|u| {
			                     let m = meta.packages.iter().find(|p| p.id.matches(u.package_id));
			                     Node::<'t> { meta: m, unit: u }
		                     })
		                     .map(|node| {
			                     RootNode::<'t> { ws,
			                                      node,
			                                      deps: Vec::with_capacity(0) }
		                     })
		                     .collect::<Vec<_>>();


		let deps_of = |u: &'t Unit| {
			log::trace!("deps of {}::{}:", u.package_id.name(), u.target.name);
			u.dependencies
			 .iter()
			 .map(|d| &units.units[d.index])
			 .filter(mode_is_build)
			 .filter(is_sub_tk)
			 .map(|u| {
				 let m = meta.packages.iter().find(|p| p.id.matches(u.package_id));
				 Node::<'t> { meta: m, unit: u }
			 })
			 .inspect(|n| {
				 log::trace!("  {} (meta: {})", n.package_id().name(), n.meta.is_some(),);
			 })
		};


		// flat meta-deps-tree:
		for root in roots.iter_mut() {
			let deps_allowed = root.deps_allowed();
			let root_is_dev = matches!(root.node.unit.target.kind, TargetKind::Example);
			log::trace!("root (dev: {root_is_dev}) {}", root.package_id().name());
			log::trace!("deps allowed: {deps_allowed} for {}", root.package_id().name());

			let mut deps = Vec::with_capacity(units.units.len());

			if deps_allowed {
				// level 0:
				deps.extend(deps_of(root.node.unit));

				let mut from = 0;
				let mut end = deps.len();

				// level 1..-:
				let mut level = 1;
				while from != end {
					log::trace!("depth level: {level}");

					let next: Vec<_> = deps[from..].iter().map(|n| n.unit).flat_map(deps_of).collect();
					deps.extend(next.into_iter());

					from = end;
					end = deps.len();
					level += 1;
				}

				log::debug!(
				            "Total deps for {}::{} ({:?}) : {}",
				            root.package_id().name(),
				            root.node.unit.target.name,
				            root.node.unit.target.kind,
				            deps.len()
				);


				// Pre-filter. Remove following:
				// - units without pd-meta
				// - units without assets in the pd-meta
				// - remove units with id eq root's
				let removed = deps.extract_if(|n| {
					                  n.package_id() == root.package_id() ||
					                  n.meta
					                   .and_then(|m| m.metadata.as_ref())
					                   .and_then(|m| m.inner.as_ref())
					                   .filter(|m| !m.assets().is_empty())
					                   .is_none()
				                  })
				                  .count();
				log::debug!("removed {removed} without metadata or assets");


				// dedup: remove first ones, leave a last one:
				let mut dups = deps.iter().fold(HashMap::new(), |mut acc, n| {
					                          acc.entry(n.package_id()).and_modify(|v| *v += 1).or_insert(0);
					                          acc
				                          });
				let removed = deps.extract_if(move |n| {
					                  let v = dups[n.package_id()];
					                  if v > 0 {
						                  dups.insert(n.package_id(), v - 1);
						                  true
					                  } else {
						                  false
					                  }
				                  })
				                  .count();
				log::debug!("removed {removed} duplicates");

				log::debug!(
				            "Total reduced deps for {}::{} ({:?}) : {}",
				            root.package_id().name(),
				            root.node.unit.target.name,
				            root.node.unit.target.kind,
				            deps.len()
				);
			} else {
				log::debug!(
				            "No deps for {}::{} 🤷🏻‍♂️",
				            root.package_id().name(),
				            root.node.unit.target.name
				);
			}


			// add the root:
			if root.node
			       .meta
			       .and_then(|m| m.metadata.as_ref())
			       .and_then(|m| m.inner.as_ref())
			       .filter(|m| !m.assets().is_empty() || (root_is_dev && !m.dev_assets().is_empty()))
			       .is_some()
			{
				log::trace!(
				            "add root too because it has assets for {}",
				            root.node.unit.target.name
				);

				deps.insert(0, root.node);
			}


			deps.iter().enumerate().for_each(|(i, n)| {
				                       log::trace!(
				                                   "{i}: {}::{} ({:?}), meta: {}",
				                                   root.package_id().name(),
				                                   root.node.unit.target.name,
				                                   root.node.unit.target.kind,
				                                   n.meta.is_some()
				);
			                       });

			log::debug!(
			            "Total finally deps for {}::{} ({:?}) : {}",
			            root.package_id().name(),
			            root.node.unit.target.name,
			            root.node.unit.target.kind,
			            deps.len()
			);

			root.deps = deps;
		}

		Self { units, meta, roots }
	}


	pub fn roots(&self) -> &[RootNode<'t>] { self.roots.as_slice() }


	/// Groups of root-units by compile-target.
	///
	/// Possible groups:
	/// 1. Contains just one root-unit;
	/// 2. Contains two units with __same cargo-target__ and different rustc-targets __including__ playdate.
	pub fn root_groups(&self) { unimplemented!() }


	/// Returns first root for each group by [`roots_by_compile_target`][Self::roots_by_compile_target].
	pub fn roots_compile_target_agnostic(&self) -> impl Iterator<Item = &RootNode<'t>> {
		self.roots_by_compile_target().into_iter().flat_map(|(key, _)| {
			                                          self.roots().iter().find(|n| {
				                                                             n.node().unit().package_id ==
				                                                             *key.package_id &&
				                                                             &n.node().unit().target == key.target
			                                                             })
		                                          })
	}

	/// Groups of root-units by target.
	///
	/// Grouping: (package_id + cargo-target) => [rustc-target].
	pub fn roots_by_compile_target(&self) -> BTreeMap<TargetKey, BTreeSet<cargo::core::compiler::CompileKind>> {
		self.roots.iter().fold(BTreeMap::new(), |mut acc, root| {
			                 let key = TargetKey::from(root);
			                 acc.entry(key)
			                    .or_default()
			                    .insert(root.node().unit().platform);
			                 acc
		                 })
	}


	pub fn root_for(&self, id: &PackageId, tk: &TargetKindWild, tname: &str) -> CargoResult<&RootNode<'t>> {
		self.roots
		    .iter()
		    .find(|n| n.package_id() == id && *tk == n.node.unit.target.kind && n.node.unit.target.name == tname)
		    .ok_or_else(|| anyhow::anyhow!("Root not found for {id}::{tname}"))
	}

	pub fn units(&self) -> &'t UnitGraph { self.units }
	pub fn meta(&self) -> &'t CargoMetadataPd { self.meta }


	pub fn deps_allowed_for(&self, root: PackageId) -> bool {
		self.roots
		    .iter()
		    .find(|u| u.package_id() == &root)
		    .and_then(|u| {
			    u.node
			     .meta
			     .as_ref()
			     .and_then(|m| m.metadata.as_ref())
			     .and_then(|m| m.inner.as_ref())
			     .map(|m| m.assets_options())
		    })
		    .unwrap_or_default()
		    .dependencies()
	}
}


#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct TargetKey<'t> {
	package_id: &'t PackageId,
	target: &'t UnitTarget,
}

impl<'t> From<&'t RootNode<'t>> for TargetKey<'t> {
	fn from(node: &'t RootNode<'t>) -> Self { TargetKey::from(node.node()) }
}

impl<'t> From<&'t Node<'t>> for TargetKey<'t> {
	fn from(node: &'t Node<'t>) -> Self { TargetKey::from(node.unit()) }
}

impl<'t> From<&'t Unit> for TargetKey<'t> {
	fn from(unit: &'t Unit) -> Self {
		TargetKey { package_id: &unit.package_id,
		            target: &unit.target }
	}
}


pub trait DependenciesAllowed {
	fn deps_allowed(&self) -> bool;
}


impl DependenciesAllowed for RootNode<'_> {
	fn deps_allowed(&self) -> bool { self.node.deps_allowed() || self.as_source().assets_options().dependencies() }
}

impl DependenciesAllowed for Node<'_> {
	fn deps_allowed(&self) -> bool {
		self.meta
		    .as_ref()
		    .and_then(|m| m.metadata.as_ref())
		    .and_then(|m| m.inner.as_ref())
		    .map(|m| m.assets_options())
		    .unwrap_or_default()
		    .dependencies()
	}
}


impl DependenciesAllowed for cargo::core::Package {
	fn deps_allowed(&self) -> bool {
		self.manifest()
		    .custom_metadata()
		    .and_then(|v| {
			    CrateMetadata::<InternedString>::deserialize(v.to_owned().into_deserializer()).log_err()
			                                                                                  .ok()
		    })
		    .and_then(|m| m.inner)
		    .map(|m| m.assets_options().dependencies())
		    .unwrap_or_default()
	}
}


impl<'t> Node<'t> {
	pub fn into_source(self) -> impl PackageSource<Metadata = MainMetadata<InternedString>> + 't {
		CrateNode::from(self)
	}
	pub fn as_source(&self) -> impl PackageSource<Metadata = MainMetadata<InternedString>> + 't {
		self.to_owned().into_source()
	}
}

impl<'t> RootNode<'t> {
	pub fn into_source(self) -> impl PackageSource<Metadata = MainMetadata<InternedString>> + 't {
		CrateNode::from(&self)
	}
	pub fn as_source(&self) -> impl PackageSource<Metadata = MainMetadata<InternedString>> + 't {
		CrateNode::from(self)
	}
}


struct CrateNode<'t> {
	node: Node<'t>,
	bins: Vec<&'t str>,
	examples: Vec<&'t str>,

	ws: Option<&'t WorkspaceMetadata>,
}

impl<'t> From<Node<'t>> for CrateNode<'t> {
	fn from(node: Node<'t>) -> Self {
		Self { node,
		       bins: node.meta
		                 .as_ref()
		                 .into_iter()
		                 .flat_map(|m| m.targets.iter())
		                 .filter(|t| t.kind == TargetKind::Bin)
		                 .map(|t| t.name.as_str())
		                 .collect(),
		       examples: node.meta
		                     .as_ref()
		                     .into_iter()
		                     .flat_map(|m| m.targets.iter())
		                     .filter(|t| t.kind == TargetKind::Example)
		                     .map(|t| t.name.as_str())
		                     .collect(),
		       ws: None }
	}
}

impl<'t> From<&RootNode<'t>> for CrateNode<'t> {
	fn from(root: &RootNode<'t>) -> Self {
		let node = root.node;
		Self { node,
		       bins: node.meta
		                 .as_ref()
		                 .into_iter()
		                 .flat_map(|m| m.targets.iter())
		                 .filter(|t| t.kind == TargetKind::Bin)
		                 .map(|t| t.name.as_str())
		                 .collect(),
		       examples: node.meta
		                     .as_ref()
		                     .into_iter()
		                     .flat_map(|m| m.targets.iter())
		                     .filter(|t| t.kind == TargetKind::Example)
		                     .map(|t| t.name.as_str())
		                     .collect(),
		       ws: root.ws }
	}
}

impl PackageSource for CrateNode<'_> {
	type Authors = [String];
	type Metadata = MainMetadata<InternedString>;

	fn name(&self) -> std::borrow::Cow<str> { self.node.package_id().name().as_str().into() }

	fn authors(&self) -> &Self::Authors {
		self.node
		    .meta
		    .as_ref()
		    .map(|m| m.authors.as_slice())
		    .unwrap_or_default()
	}

	fn version(&self) -> std::borrow::Cow<str> {
		self.node
		    .meta
		    .as_ref()
		    .map(|m| m.version.as_str().into())
		    .unwrap_or_default()
	}

	fn description(&self) -> Option<std::borrow::Cow<str>> {
		self.node
		    .meta
		    .as_ref()
		    .and_then(|m| m.description.as_deref())
		    .map(Into::into)
	}

	fn metadata(&self) -> Option<&Self::Metadata> {
		self.node
		    .meta
		    .as_ref()
		    .and_then(|m| m.metadata.as_ref())
		    .and_then(|m| m.inner.as_ref())
	}

	fn bins(&self) -> &[&str] { &self.bins }

	fn examples(&self) -> &[&str] { &self.examples }

	fn manifest_path(&self) -> std::borrow::Cow<std::path::Path> {
		self.node
		    .meta
		    .as_ref()
		    .map(|m| m.manifest_path.as_path().into())
		    .unwrap_or_default()
	}


	// from ws metadata:
	fn default_options(&self) -> Option<&playdate::metadata::format::ws::OptionsDefault> {
		self.ws
		    .and_then(|m| m.inner.as_ref())
		    .as_ref()
		    .and_then(|m| m.options.as_ref())
	}
}
