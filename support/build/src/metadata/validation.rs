use std::fmt::Display;

use super::source::CrateInfoSource;
use super::source::ManifestSourceOptExt;


#[derive(Debug, Clone)]
pub enum Problem {
	UnknownTarget { name: String },
	MissingField { field: String },
	Warning(Warning),
}

#[derive(Debug, Clone)]
pub enum Warning {
	StrangeValue {
		field: String,
		value: String,
		reason: Option<&'static str>,
	},
	UnknownField {
		field: String,
		reason: Option<&'static str>,
	},
	MissingField {
		field: String,
		reason: Option<&'static str>,
	},
}

impl Display for Warning {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::StrangeValue { field, value, reason } => {
				write!(f, "Strange value {value:?} for field '{field}'")?;
				if let Some(reason) = reason {
					write!(f, ", {reason}")
				} else {
					Ok(())
				}
			},
			Self::UnknownField { field, reason } => {
				write!(f, "Unknown field '{field}'")?;
				if let Some(reason) = reason {
					write!(f, ", {reason}")
				} else {
					Ok(())
				}
			},
			Self::MissingField { field, reason } => {
				write!(f, "Missing field '{field}'")?;
				if let Some(reason) = reason {
					write!(f, ", {reason}")
				} else {
					Ok(())
				}
			},
		}
	}
}


impl Problem {
	pub fn is_err(&self) -> bool {
		match self {
			Problem::Warning(_) => false,
			_ => true,
		}
	}

	pub fn is_warn(&self) -> bool { !self.is_err() }
}

impl Display for Problem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnknownTarget { name } => write!(f, "Unknown cargo-target: {name}"),
			Self::MissingField { field } => write!(f, "Missing field: {field}"),
			Self::Warning(warning) => warning.fmt(f),
		}
	}
}


/// Check the implementor validity.
pub trait Validate {
	/// Check critical requirements, returns it as errors.
	/// Also returns warnings fo not so critical problems.
	/// Use it before render the final result.
	fn validate(&self) -> impl IntoIterator<Item = Problem>;
}

impl<T: ManifestSourceOptExt> Validate for T {
	fn validate(&self) -> impl IntoIterator<Item = Problem> {
		let is_not_empty = |s: &&str| !s.trim().is_empty();

		fn check_some<T>(name: &'static str, v: Option<T>) -> Option<Problem> {
			v.is_none().then(|| Problem::MissingField { field: name.into() })
		}

		fn warn_none<T>(name: &'static str, v: Option<T>, warn_msg: Option<&'static str>) -> Option<Problem> {
			v.is_none().then(|| {
				           Problem::Warning(Warning::MissingField { field: name.into(),
				                                                    reason: warn_msg })
			           })
		}


		let missed = [
		              (
			"build-number",
			self.build_number().is_some(),
			Some("required for sideloaded games."),
		),
		              ("description", self.description().is_some(), None),
		].into_iter()
		             .filter_map(|(k, v, msg)| warn_none(k, v.then_some(()), msg));


		let unknown = self.iter_extra().into_iter().flatten().map(|(k, _)| {
			                                                     Problem::Warning(Warning::UnknownField { field: k.as_ref()
				                                                                                        .to_owned(),
				                                                                                reason: None })
		                                                     });


		// required fields
		let errors = [
		              ("name", self.name().filter(is_not_empty)),
		              ("version", self.version().filter(is_not_empty)),
		              ("bundle-id", self.bundle_id().filter(is_not_empty)),
		].into_iter()
		             .filter_map(|(k, v)| check_some(k, v));


		errors.chain(missed)
		      .chain(self.version().into_iter().filter_map(validate_version))
		      .chain(unknown)
	}
}


fn validate_version(value: &str) -> Option<Problem> {
	let re = regex::Regex::new(r"^\d+(?:\.\d+){0,2}$").unwrap();
	if !re.is_match(value.trim()) {
		if semver::Version::parse(value).is_err() {
			Some(Problem::Warning(Warning::StrangeValue { field: "version".into(),
			                                              value: value.into(),
			                                              reason: Some("can be confusing.") }))
		} else {
			None
		}
	} else {
		None
	}
}


/// Lint the crate-level source.
pub trait ValidateCrate: CrateInfoSource {
	fn validate(&self) -> impl IntoIterator<Item = Problem> {
		// - main manifest missing fields
		// - main manifest fields in bad format
		// - for each final target manifest:
		//   -> same as for the main manifest


		if let Some(_meta) = self.metadata() {
			// Check that all targets are exists
			// - search the target in the crate for each in meta.all_targets()
		} else {
			// - warn: no metadata found
		}

		// just temporary this, because not implemented yet:
		self.manifest_for_crate()
		    .validate()
		    .into_iter()
		    .collect::<Vec<_>>()
	}


	fn validate_for(&self, _target: &str) -> impl IntoIterator<Item = Problem> { [] }
}

impl<T> ValidateCrate for T where T: CrateInfoSource {}


#[cfg(test)]
mod tests {
	use super::Validate;
	use super::super::format::Manifest;


	#[test]
	fn validate_version() {
		let cases = [
		             ("0", true),
		             ("0.0", true),
		             ("0.0.0", true),
		             ("0.0.0-pre", true),
		             ("", false),
		             ("0.0.a", false),
		             ("beta", false),
		];

		for (i, (ver, ok)) in cases.iter().enumerate() {
			let result = super::validate_version(ver);
			assert_eq!(*ok, result.is_none(), "{i}: {result:?}");
		}
	}


	#[test]
	fn manifest_empty() {
		let m = Manifest::<&str>::default();
		let errors = m.validate().into_iter().collect::<Vec<_>>();
		assert_eq!(5, errors.len(), "{:#?}", errors);
		assert_eq!(3, errors.iter().filter(|e| e.is_err()).count());
	}

	#[test]
	fn manifest_valid() {
		let m = Manifest::<&str> { name: "name".into(),
		                           version: "0.0".into(),
		                           author: "author".into(),
		                           bundle_id: "bundle.id".into(),
		                           description: "description".into(),
		                           image_path: "image_path".into(),
		                           launch_sound_path: "launch_sound_path".into(),
		                           content_warning: "content_warning".into(),
		                           content_warning2: "content_warning2".into(),
		                           build_number: 42.into() };
		let errors = m.validate().into_iter().collect::<Vec<_>>();
		assert!(errors.is_empty(), "{:#?}", errors);
	}
}
