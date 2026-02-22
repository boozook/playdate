use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::RwLock;

use bindgen::callbacks::DiscoveredItem;
use bindgen::callbacks::DiscoveredItemId;
use bindgen::callbacks::ItemInfo;
use bindgen::callbacks::ItemKind;
use convert_case::{Case, Casing};


pub type SharedIdents = Arc<RwLock<BTreeMap<Kind, String>>>;
pub type SharedCfg = Arc<RwLock<RenameMapCfg>>;


pub fn reduce(changes: SharedIdents) {
	let mut items = BTreeSet::new();
	{
		let changes = changes.read().expect("renamed set is locked");
		for k in changes.keys().filter(|k| matches!(k, Kind::Struct(_))) {
			let name = match k {
				Kind::Struct(name) | Kind::Union(name) => name.as_str(),
				Kind::Item(_) | Kind::EnumVariant(..) => unreachable!("already filtered-out"),
			};
			let ik = Kind::Item(name.to_string());
			if changes.contains_key(&ik) {
				items.insert(ik);
			}
		}
	}

	let mut changes = changes.write().expect("renamed set is locked");
	for k in items {
		changes.remove(&k);
	}
}


/// Renames symbols in the bindings.
#[derive(Debug, Default)]
pub struct RenameMap {
	pub renamed: SharedIdents,
	pub cfg: SharedCfg,
}

impl RenameMap {
	pub fn new(cfg: SharedCfg) -> Self {
		Self { cfg,
		       ..Default::default() }
	}

	fn renamed(&self, orig: Kind, new: String) {
		let eq = match &orig {
			Kind::Item(ty) | Kind::Struct(ty) | Kind::Union(ty) => new.eq(ty),
			Kind::EnumVariant(ty, v) => new.eq(v) || new.eq(ty),
		};
		if eq {
			return;
		}

		self.renamed
		    .write()
		    .expect("renamed set is locked")
		    .insert(orig, new);
	}
}


/// Contains original names.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
	/// Item with original name
	Item(String),
	/// Struct with original name
	Struct(String),
	/// Union with original name
	Union(String),
	/// `(enum name, variant name)`
	EnumVariant(String, String),
}

impl Kind {
	pub fn item_name(&self) -> &str {
		match self {
			Self::Item(name) | Self::Struct(name) | Self::Union(name) | Self::EnumVariant(name, _) => name.as_str(),
		}
	}
}

impl Display for Kind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Kind::Item(name) => {
				write!(f, "item `{name}`")
			},
			Kind::Struct(name) => {
				write!(f, "struct `{name}`")
			},
			Kind::Union(name) => {
				write!(f, "union `{name}`")
			},
			Kind::EnumVariant(name, variant) => {
				write!(f, "enum `{name}::{variant}`")
			},
		}
	}
}


impl bindgen::callbacks::ParseCallbacks for RenameMap {
	fn new_item_found(&self, _id: DiscoveredItemId, item: DiscoveredItem) {
		match item {
			DiscoveredItem::Struct { original_name: Some(original_name),
			                         final_name, } => {
				self.renamed(Kind::Struct(original_name), final_name);
			},
			DiscoveredItem::Union { original_name: Some(original_name),
			                        final_name, } => {
				self.renamed(Kind::Union(original_name), final_name);
			},
			// DiscoveredItem::Alias { alias_name, .. } => println!("gen alias: {alias_name}"),
			// DiscoveredItem::Enum { final_name } => println!("gen enum: {final_name}"),
			// DiscoveredItem::Function { final_name } => println!("gen function: {final_name}"),
			// DiscoveredItem::Method { final_name, parent } => println!("gen method: {final_name} of {parent:?}"),
			_ => {},
		}
	}

	fn item_name(&self, info: ItemInfo) -> Option<String> {
		let name = info.name;
		let exclude = |s: &str| matches!(info.kind, ItemKind::Var) || s.starts_with('_') || s == "va_list";

		let cfg = self.cfg.read().expect("rename-map is locked");
		if let Some(new) = cfg.items.get(name) {
			Some(new.to_owned())
		} else if !exclude(name) {
			let strip_prefix = cfg.strip_prefix.as_deref().filter(|p| !p.is_empty());

			match (strip_prefix, cfg.fix_case) {
				(Some(prefix), true) => {
					let new = prefix.iter()
					                .find_map(|prefix| name.strip_prefix(prefix))
					                .unwrap_or(name)
					                .to_case(Case::UpperCamel);
					(new != name).then_some(new)
				},
				(Some(prefix), false) => {
					prefix.iter()
					      .find_map(|prefix| name.strip_prefix(prefix))
					      .map(ToOwned::to_owned)
				},
				(None, true) => {
					let new = name.to_case(Case::UpperCamel);
					(new != name).then_some(new)
				},
				(None, false) => None,
			}
		} else {
			None
		}.inspect(|new| self.renamed(Kind::Item(name.to_owned()), new.to_owned()))
	}

	fn enum_variant_name(&self,
	                     ename: Option<&str>,
	                     vname: &str,
	                     _: bindgen::callbacks::EnumVariantValue)
	                     -> Option<String> {
		let ename = ename?;
		// workaround bindgen's bug: enum name is prefixed with "enum " sometimes
		let ty = ename.strip_prefix("enum ").unwrap_or(ename);

		let cfg = self.cfg.read().expect("rename-map is locked");
		let id = format!("{ty}::{vname}");
		let exclude = |s: &str| s.starts_with('_');

		if let Some(new) = cfg.enums.get(&id) {
			Some(new.to_owned())
		} else if !exclude(&id) && !exclude(vname) {
			cfg.fix_case
			   .then(|| vname.to_case(Case::UpperCamel))
			   .filter(|new| new != vname)
		} else {
			None
		}
	}
}


#[derive(Debug, Default, serde::Deserialize)]
pub struct RenameMapCfg {
	#[serde(alias = "to-upper-camel-case", rename = "to-upper-camel-case")]
	fix_case: bool,
	#[serde(alias = "strip-prefix", rename = "strip-prefix")]
	strip_prefix: Option<Vec<String>>,
	#[serde(alias = "item", rename = "item")]
	items: HashMap<String, String>,
	#[serde(alias = "enum", rename = "enum")]
	enums: HashMap<String, String>,
}
