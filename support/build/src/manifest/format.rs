use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};


#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "Value: Deserialize<'de>")))]
pub struct Manifest<Value> {
	pub name: String,
	pub author: String,
	pub description: String,
	#[cfg_attr(feature = "serde", serde(rename = "bundleID"))]
	pub bundle_id: String,
	pub version: String,
	pub build_number: Option<usize>,
	pub image_path: Option<String>,
	pub launch_sound_path: Option<String>,
	pub content_warning: Option<String>,
	pub content_warning2: Option<String>,

	/// Manifest extra fields, e.g: `pdxversion=20000`
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub extra: HashMap<String, Value>,
}


impl<T: crate::value::Value> Manifest<T> {
	pub fn to_manifest_string(&self) -> String {
		let mut result = String::new();

		fn to_row<K: AsRef<str>, V: AsRef<str>>(key: K, value: V) -> String {
			if !value.as_ref().trim().is_empty() {
				format!("{}={}\n", key.as_ref(), value.as_ref())
			} else {
				String::with_capacity(0)
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
		for (key, value) in &self.extra {
			if let Some(value) = value.as_str() {
				result.push_str(&to_row(key, value));
			} else if let Some(value) = value.as_bool() {
				result.push_str(&to_row(key, format!("{value}")));
			} else {
				warn!("Manifest extra field `{key}={value}` has unsupported type");
			}
		}
		result
	}
}
