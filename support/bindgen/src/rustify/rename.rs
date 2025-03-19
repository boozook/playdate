use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::RwLock;

use bindgen::callbacks::DiscoveredItem;
use bindgen::callbacks::DiscoveredItemId;
use convert_case::{Case, Casing};


pub type SharedRenamed = Arc<RwLock<BTreeMap<Kind, String>>>;


pub fn reduce(_changes: &mut SharedRenamed) {}


/// Renames symbols in the bindings.
#[derive(Debug, Default)]
pub struct RenameMap {
	pub renamed: SharedRenamed,
}

impl RenameMap {
	pub fn new() -> Self { Default::default() }

	fn renamed(&self, was: Kind, now: String) {
		self.renamed
		    .write()
		    .expect("renamed set is locked")
		    .insert(was, now);
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
			_ => {},
		}
	}

	fn item_name(&self, name: &str) -> Option<String> {
		if name.starts_with("_bindgen_") ||
		   name.starts_with("__bindgen_") ||
		   name.starts_with("__builtin_") ||
		   name.starts_with("ptr_")
		{
			return None;
		}

		let mut exact = BTreeMap::new();
		exact.insert("PDNetErr", "NetworkError");
		exact.insert("PlaydateAPI", "Playdate");
		exact.insert("playdate_videostream", "PlaydateVideoStream");
		exact.insert("l_valtype", "LuaValueType");
		exact.insert("PDRect", "Rect");
		exact.insert("LCDRect", "Aabb");

		let mut ignore = BTreeSet::new();
		ignore.extend([
			"void",
			"root",
			//
			"SEEK_SET",
			"SEEK_CUR",
			"SEEK_END",
			"LCD_COLUMNS",
			"LCD_ROWS",
			"LCD_ROWSIZE",
			"AUDIO_FRAMES_PER_CYCLE",
			"NOTE_C4",
		]);

		if ignore.contains(name) {
			return None;
		}

		if let Some(s) = exact.get(name) {
			self.renamed(Kind::Item(name.to_owned()), s.to_string());
			return Some(s.to_string());
		}

		let res = if name.starts_with("PD") {
			&name[2..]
		} else if name.starts_with("LCD") {
			&name[3..]
		} else {
			name
		};

		let res = res.to_case(Case::UpperCamel);

		if res != name {
			self.renamed(Kind::Item(name.to_owned()), res.clone());
			Some(res)
		} else {
			None
		}
	}

	fn enum_variant_name(&self,
	                     ename: Option<&str>,
	                     vname: &str,
	                     _: bindgen::callbacks::EnumVariantValue)
	                     -> Option<String> {
		let mut exact_var = BTreeMap::new();
		exact_var.insert("kASCIIEncoding", "ASCII");
		exact_var.insert("kUTF8Encoding", "UTF8");
		exact_var.insert("k16BitLEEncoding", "UTF16");

		exact_var.insert("kSound8bitMono", "Mono8bit");
		exact_var.insert("kSound8bitStereo", "Stereo8bit");
		exact_var.insert("kSound16bitMono", "Mono16bit");
		exact_var.insert("kSound16bitStereo", "Stereo16bit");
		exact_var.insert("kSoundADPCMMono", "MonoADPCM");
		exact_var.insert("kSoundADPCMStereo", "StereoADPCM");
		exact_var.insert("kColorXOR", "XOR");
		exact_var.insert("kDrawModeXOR", "XOR");
		exact_var.insert("kDrawModeNXOR", "NXOR");

		let mut prefix = BTreeMap::new();
		prefix.insert("PDTextWrappingMode", "Wrap");
		prefix.insert("PDTextAlignment", "AlignText");
		prefix.insert("MicSource", "MicInput");
		prefix.insert("PDLanguage", "PdLanguage");
		prefix.insert("LCDFontLanguage", "LcdFontLanguage");
		prefix.insert("LFOType", "LfoType");
		prefix.insert("PDButtons", "Button");
		prefix.insert("json_value_type", "Json");

		let mut ignore = BTreeSet::new();
		ignore.extend(["idtype_t"]);

		let ename = ename.expect("enum name for {vname} must not be empty");


		if ignore.contains(&ename) || ignore.contains(vname) {
			return None;
		}


		// workaround bindgen's bug: enum name is prefixed with "enum " sometimes
		let ename = ename.strip_prefix("enum ").unwrap_or(ename);


		if let Some(s) = exact_var.get(vname) {
			self.renamed(
			             Kind::EnumVariant(ename.to_owned(), vname.to_owned()),
			             s.to_string(),
			);
			return Some(s.to_string());
		}


		let res = if vname.starts_with('k') {
			Cow::Owned((&vname[1..]).to_case(Case::UpperCamel))
		} else {
			vname.into()
		};


		let res = if let Some(prefix) = prefix.get(&ename) {
			res.strip_prefix(prefix).unwrap_or(&res)
		} else {
			&res
		};

		let mut res = res.to_case(Case::UpperCamel);

		// auto-trim-prefix:
		{
			let eparts = Case::Pascal.split(&ename);
			let vparts = Case::Pascal.split(&res);
			let mut rparts = vparts.as_slice();

			for word in eparts.iter() {
				if let Some(parts) = rparts.strip_prefix(&[*word]) {
					rparts = parts;
				}
			}

			if rparts != vparts {
				res = rparts.join("");
			}
		}


		if res != vname {
			self.renamed(Kind::EnumVariant(ename.to_owned(), vname.to_owned()), res.clone());
		}

		if res != vname { Some(res) } else { None }
	}
}
