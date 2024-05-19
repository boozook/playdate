use std::fmt::Debug;
use std::fmt::Display;

use crate::metadata::format::AssetsOptions;

/// Value that can be __one of__ `bool` or `String`.
#[cfg(feature = "serde")]
pub trait Value: for<'de> serde::de::Deserialize<'de> + Clone + Debug + Display
	where Self: TryInto<AssetsOptions> {
	fn as_bool(&self) -> Option<bool>;
	fn as_str(&self) -> Option<&str>;
}

/// Value that can be __one of__ `bool` or `String`,
/// without `serde::Deserialize` requirement.
#[cfg(not(feature = "serde"))]
pub trait Value: for<'de> Clone + Debug + Display
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


#[cfg(test)]
pub mod default {
	use super::AssetsOptions;

	#[derive(Debug, Clone, PartialEq)]
	pub enum Value {
		Boolean(bool),
		String(String),
	}

	/// Fake `Value` for tests.
	impl super::Value for Value {
		fn as_bool(&self) -> Option<bool> {
			match self {
				Value::Boolean(v) => Some(*v),
				Value::String(_) => None,
			}
		}

		fn as_str(&self) -> Option<&str> {
			match self {
				Value::Boolean(_) => None,
				Value::String(s) => Some(s),
			}
		}
	}

	impl std::fmt::Display for Value {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Value::Boolean(v) => v.fmt(f),
				Value::String(v) => v.fmt(f),
			}
		}
	}

	#[cfg(feature = "serde")]
	impl<'t> serde::Deserialize<'t> for Value {
		fn deserialize<D>(_: D) -> Result<Self, D::Error>
			where D: serde::Deserializer<'t> {
			unreachable!()
		}
	}

	impl TryInto<AssetsOptions> for Value {
		type Error = &'static str;
		fn try_into(self) -> Result<AssetsOptions, Self::Error> { unreachable!() }
	}


	impl From<bool> for Value {
		fn from(value: bool) -> Self { Self::Boolean(value) }
	}

	impl From<&str> for Value {
		fn from(value: &str) -> Self { value.to_string().into() }
	}

	impl From<String> for Value {
		fn from(value: String) -> Self { Self::String(value) }
	}
}
