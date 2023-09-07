use serde::de::Deserialize;
use std::fmt::Debug;
use std::fmt::Display;

use crate::metadata::format::AssetsOptions;

pub trait Value: for<'de> Deserialize<'de> + Clone + Debug + Display
	where Self: TryInto<AssetsOptions> {
	fn as_bool(&self) -> Option<bool>;
	fn as_str(&self) -> Option<&str>;
}


#[cfg(feature = "serde_json")]
impl Value for serde_json::Value {
	fn as_bool(&self) -> Option<bool> { serde_json::Value::as_bool(self) }
	fn as_str(&self) -> Option<&str> { serde_json::Value::as_str(self) }
}

#[cfg(feature = "toml")]
impl Value for toml::Value {
	fn as_bool(&self) -> Option<bool> { toml::Value::as_bool(self) }
	fn as_str(&self) -> Option<&str> { toml::Value::as_str(self) }
}
