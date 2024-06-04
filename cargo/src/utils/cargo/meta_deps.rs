use std::collections::HashMap;

use cargo::core::compiler::CompileMode;
use cargo::core::PackageId;
use cargo::util::interning::InternedString;
use cargo::CargoResult;
use playdate::manifest::CrateInfoSource;
use playdate::metadata::source::MetadataSource;
use serde::Deserialize;
use serde::de::IntoDeserializer;

use crate::config::Config;
use crate::logger::LogErr;

use super::build_plan::format::TargetKind;
use super::build_plan::TargetKindWild;
use super::metadata::format::{Package, Metadata};
use super::metadata::CargoMetadataPd;
use super::unit_graph::format::Unit;
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
}


#[derive(Debug, Clone, Copy)]
pub struct Node<'cfg> {
	meta: Option<&'cfg Package<Metadata<InternedString>>>,
	unit: &'cfg Unit,
}

impl<'t> RootNode<'t> {
	pub fn package_id(&self) -> &'t PackageId { &self.node.package_id() }

	pub fn node(&self) -> &Node<'t> { &self.node }

	/// Dependencies _with assets_ of the root node,
	/// in topological order,
	/// including the root node if it has assets.
	pub fn deps(&self) -> &[Node<'t>] { &self.deps }
}

impl<'t> Node<'t> {
	pub fn package_id(&self) -> &'t PackageId { &self.unit.package_id }
}


impl<'t> MetaDeps<'t> {
	pub fn new(units: &'t UnitGraph, meta: &'t CargoMetadataPd) -> Self {
		let mode_is_build = |u: &&Unit| matches!(u.mode, CompileMode::Build);
		let is_prime_tk = |u: &&Unit| {
			matches!(
			         u.target.kind,
			         TargetKind::Lib(_) | TargetKind::Bin | TargetKind::Example
			)
		};
		let is_sub_tk = |u: &&Unit| matches!(u.target.kind, TargetKind::Lib(_));


		let mut roots = units.roots
		                     .iter()
		                     .map(|i| &units.units[*i])
		                     .filter(mode_is_build)
		                     .filter(is_prime_tk)
		                     .map(|u| {
			                     let m = meta.packages.iter().find(|p| p.id == u.package_id);
			                     Node::<'t> { meta: m, unit: u }
		                     })
		                     .map(|node| {
			                     RootNode::<'t> { node,
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
				 let m = meta.packages.iter().find(|p| p.id == u.package_id);
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

				deps.insert(0, root.node.clone());
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

	pub fn root_for(&self, id: &PackageId, tk: &TargetKindWild, tname: &str) -> CargoResult<&RootNode<'t>> {
		self.roots
		    .iter()
		    .find(|n| n.package_id() == id && *tk == n.node.unit.target.kind && n.node.unit.target.name == tname)
		    .ok_or_else(|| anyhow::anyhow!("Root not found for {id}::{tname}"))
	}

	pub fn units(&self) -> &'t UnitGraph { &*self.units }
	pub fn meta(&self) -> &'t CargoMetadataPd { &*self.meta }


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
		    .dependencies
	}
}


pub trait DependenciesAllowed {
	fn deps_allowed(&self) -> bool;
}


impl DependenciesAllowed for RootNode<'_> {
	fn deps_allowed(&self) -> bool { self.node.deps_allowed() }
}

impl DependenciesAllowed for Node<'_> {
	fn deps_allowed(&self) -> bool {
		self.meta
		    .as_ref()
		    .and_then(|m| m.metadata.as_ref())
		    .and_then(|m| m.inner.as_ref())
		    .map(|m| m.assets_options())
		    .unwrap_or_default()
		    .dependencies
	}
}


impl DependenciesAllowed for cargo::core::Package {
	fn deps_allowed(&self) -> bool {
		self.manifest()
		    .custom_metadata()
		    .and_then(|v| {
			    Metadata::<InternedString>::deserialize(v.to_owned().into_deserializer()).log_err()
			                                                                             .ok()
		    })
		    .and_then(|m| m.inner)
		    .map(|m| m.assets_options().dependencies)
		    .unwrap_or_default()
	}
}


impl<'t> Node<'t> {
	pub fn into_source(self) -> impl CrateInfoSource + 't { CrateNode::from(self) }
	pub fn as_source(&self) -> impl CrateInfoSource + 't { self.to_owned().into_source() }
}

impl<'t> RootNode<'t> {
	pub fn into_source(self) -> impl CrateInfoSource + 't { CrateNode::from(self.node) }
	pub fn as_source(&self) -> impl CrateInfoSource + 't { self.to_owned().into_source() }
}


struct CrateNode<'t> {
	node: Node<'t>,
	bins: Vec<&'t str>,
	examples: Vec<&'t str>,
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
		                     .collect() }
	}
}

impl CrateInfoSource for CrateNode<'_> {
	type Authors = [String];

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

	fn metadata(&self) -> Option<impl MetadataSource> {
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
}
