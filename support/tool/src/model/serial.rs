use std::borrow::Cow;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;


/// Represents a device serial number.
/// e.g.: PDU1-Y000235
#[derive(Clone)]
pub struct SerialNumber(String);

const UNKNOWN: &str = "UNKNOWN";

impl SerialNumber {
	pub(crate) fn unknown() -> Self { Self(UNKNOWN.to_string()) }
}


impl SerialNumber {
	pub fn contained_in<S: AsRef<str>>(s: S) -> Option<Self> {
		pub const REGEX_NAME: &str = r"^.*(PDU\d+[_-][a-zA-Z0-9]+).*$";
		let re = Regex::new(REGEX_NAME).expect("invalid regex");
		let captures = re.captures(s.as_ref())?;
		let serial = Self::unify(captures.get(1)?.as_str());
		let serial = if serial.contains("_") {
			serial.replace("_", "-")
		} else {
			serial.to_string()
		};

		Some(Self(serial.to_owned()))
	}


	fn unify<'s, S: Into<Cow<'s, str>>>(s: S) -> Cow<'s, str> {
		let s = s.into();
		if s.contains("_") {
			s.replace("_", "-").into()
		} else {
			s
		}
	}
}

impl FromStr for SerialNumber {
	type Err = DeviceSerialFormatError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::contained_in(s).ok_or(DeviceSerialFormatError(s.to_string()))
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
	fn eq(&self, other: &Self) -> bool {
		(self.0 != UNKNOWN && other.0 != UNKNOWN) && (self.0.contains(&other.0) || other.0.contains(&self.0))
	}
}

impl std::fmt::Debug for SerialNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Serial").field(&self.0).finish()
	}
}

impl std::fmt::Display for SerialNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}


#[derive(Debug)]
pub struct DeviceSerialFormatError(String);
impl std::error::Error for DeviceSerialFormatError {}

impl std::fmt::Display for DeviceSerialFormatError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
		       f,
		       "Invalid serial number format: {}, should be PDUN-XNNNNNN",
		       self.0
		)
	}
}

impl From<String> for DeviceSerialFormatError {
	fn from(value: String) -> Self { Self(value) }
}

impl From<&str> for DeviceSerialFormatError {
	fn from(value: &str) -> Self { Self(value.to_owned()) }
}
