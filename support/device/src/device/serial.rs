use std::borrow::Cow;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;


/// Represents a device serial number.
/// e.g.: PDU1-Y000235
#[derive(Clone)]
pub struct SerialNumber(String);


impl SerialNumber {
	pub fn contained_in<S: AsRef<str>>(s: S) -> Option<Self> {
		pub const REGEX_NAME: &str = r"^.*(PDU\d+[_-][a-zA-Z0-9]+).*$";
		let re = Regex::new(REGEX_NAME).expect("invalid regex");
		let captures = re.captures(s.as_ref())?;
		let serial = Self::unify(captures.get(1)?.as_str());
		let serial = if serial.contains('_') {
			serial.replace('_', "-")
		} else {
			serial.to_string()
		};

		Some(Self(serial.to_owned()))
	}


	fn unify<'s, S: Into<Cow<'s, str>>>(s: S) -> Cow<'s, str> {
		let s = s.into();
		if s.contains('_') {
			s.replace('_', "-").into()
		} else {
			s
		}
	}


	pub fn as_str(&self) -> &str { &self.0 }
}

impl FromStr for SerialNumber {
	type Err = error::SerialNumberFormatError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::contained_in(s).ok_or_else(|| error::SerialNumberFormatError::from(s))
	}
}


impl TryFrom<String> for SerialNumber {
	type Error = <Self as FromStr>::Err;
	fn try_from(value: String) -> Result<Self, Self::Error> { Self::from_str(value.as_str()) }
}

impl TryFrom<&str> for SerialNumber {
	type Error = <Self as FromStr>::Err;
	fn try_from(value: &str) -> Result<Self, Self::Error> { Self::from_str(value) }
}

impl TryFrom<&Path> for SerialNumber {
	type Error = <Self as FromStr>::Err;
	fn try_from(value: &Path) -> Result<Self, Self::Error> { Self::from_str(value.to_string_lossy().as_ref()) }
}


impl PartialEq for SerialNumber {
	fn eq(&self, other: &Self) -> bool { self.0.contains(&other.0) || other.0.contains(&self.0) }
}

impl<T: AsRef<str>> PartialEq<T> for SerialNumber {
	fn eq(&self, other: &T) -> bool {
		let other = other.as_ref().to_uppercase();
		other.len() >= 3 && (self.0.contains(&other) || other.contains(&self.0))
	}
}

// Commutative pares fore above
impl PartialEq<SerialNumber> for &str {
	fn eq(&self, sn: &SerialNumber) -> bool { sn.eq(self) }
}
impl PartialEq<SerialNumber> for String {
	fn eq(&self, sn: &SerialNumber) -> bool { sn.eq(self) }
}

impl std::fmt::Debug for SerialNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Serial").field(&self.0).finish()
	}
}

impl std::fmt::Display for SerialNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}


pub mod error {

	use std::backtrace::Backtrace;
	use thiserror::Error;
	use miette::Diagnostic;


	#[derive(Error, Debug, Diagnostic)]
	#[error("invalid serial number `{value}`, expected format `PDUN-XNNNNNN`.")]
	pub struct SerialNumberFormatError {
		pub value: String,
		#[backtrace]
		backtrace: Backtrace,
	}

	impl SerialNumberFormatError {
		fn new(value: String) -> Self {
			Self { value,
			       backtrace: Backtrace::capture() }
		}
	}

	impl From<String> for SerialNumberFormatError {
		fn from(value: String) -> Self { Self::new(value) }
	}

	impl From<&str> for SerialNumberFormatError {
		fn from(value: &str) -> Self { Self::new(value.to_owned()) }
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	const SN: &str = "PDU0-X000042";
	const SN_UNDERSCORE: &str = "PDU0_X000042";
	const SN_FORMS: &[&str] = &[SN, SN_UNDERSCORE];

	const PATHS: &[&str] = &["/dev/cu.usbmodem", "other/path/", "/", ""];

	#[test]
	fn from_str() {
		let sn = SerialNumber::from_str(SN).unwrap();
		let sn_ = SerialNumber::from_str(SN_UNDERSCORE).unwrap();
		assert_eq!(sn, sn_);
		assert_eq!(sn.0, sn_.0);
		assert_eq!(sn.as_str(), sn_.as_str());
	}

	#[test]
	fn from_port_path() {
		const SUFFIX: &[Option<&str>] = &[None, Some("0"), Some("1"), Some("2"), Some("42")];

		for sn in SN_FORMS {
			for suffix in SUFFIX {
				let suffix = suffix.unwrap_or_default();
				for path in PATHS {
					let path = format!("{path}{sn}{suffix}");
					println!("parsing {path}");
					let parsed = SerialNumber::from_str(&path).unwrap();
					assert!(parsed == SN);
					assert!(SN == parsed);
				}
			}
		}
	}

	#[test]
	fn from_port_path_nq() {
		const SUFFIX: &[Option<&str>] = &[None, Some("0"), Some("1"), Some("2"), Some("42")];
		let sn_forms: &[String] = &[SN.replace("42", "11"), SN_UNDERSCORE.replace("42", "11")];

		for sn in sn_forms {
			for suffix in SUFFIX {
				let suffix = suffix.unwrap_or_default();
				for path in PATHS {
					let path = format!("{path}{sn}{suffix}");
					println!("parsing {path}");
					let parsed = SerialNumber::from_str(&path).unwrap();
					assert_eq!(false, parsed == SN);
					assert_eq!(false, SN == parsed);
				}
			}
		}
	}

	#[test]
	fn invalid() {
		assert!(SerialNumber::from_str("").is_err());
		assert!(SerialNumber::from_str("PDU").is_err());
		assert!(SerialNumber::from_str("001").is_err());
		assert!(SerialNumber::from_str("001-00000").is_err());
		assert!(SerialNumber::from_str("PDU0--AAAAAAA").is_err());
	}
}
