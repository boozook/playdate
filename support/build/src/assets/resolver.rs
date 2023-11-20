use std::hash::Hash;
use std::borrow::Cow;
use std::str;
use std::path::{Path, PathBuf};

use regex::Regex;
use wax::{Glob, LinkBehavior, WalkError, WalkEntry, BuildError};

use crate::cargo;
use crate::config::Env;
use super::log_err;
use super::Error;


pub fn resolve_includes<S: AsRef<str>, Excl: AsRef<str>>(expr: S,
                                                         crate_root: &Path,
                                                         exclude: &[Excl],
                                                         links: LinkBehavior)
                                                         -> Result<Vec<Match>, Error> {
	let glob = Glob::new(expr.as_ref())?;
	let exclude = exclude.into_iter().map(AsRef::as_ref).chain(["**/.*/**"]);
	let walker = glob.walk_with_behavior(&crate_root, links)
	                 .not(exclude)?
	                 .map(|res| res.map(|entry| Match::from(entry)));

	let files = walker.map(|res| {
		                  let mut inc = res.map_err(log_err)?;
		                  let target = inc.target();
		                  // modify target path:
		                  if target.is_absolute() && target.starts_with(crate_root) {
			                  // make it relative to crate_root:
			                  let len = crate_root.components().count();
			                  Some(target.components().skip(len).collect())
		                  // TODO: need test it:
		                  // let a = PathBuf::from(&target.display().to_string()[(crate_root.display().to_string().len() +
		                  //                                                      MAIN_SEPARATOR_STR.to_string().len())..]);
		                  // // relative part let b = target.components().skip(len).collect::<PathBuf>();
		                  // assert_eq!(a, b);
		                  } else if target.is_absolute() {
			                  Some(PathBuf::from(target.file_name().expect("target filename")))
		                  } else {
			                  // as-is
			                  None
		                  }.and_then(|new| Some(inc.set_target(new)));
		                  Ok::<_, WalkError>(inc)
	                  });

	let mut resolved = Vec::new();
	for file in files {
		resolved.push(file?);
	}

	Ok(resolved)
}


// TODO: use config.env .unwrap_or
pub struct EnvResolver(Regex);
impl EnvResolver {
	pub fn new() -> Self { Self(Regex::new(r"(\$\{([^}]+)\})").unwrap()) }
}
impl Default for EnvResolver {
	fn default() -> Self { Self::new() }
}

impl EnvResolver {
	pub fn str<'c, S: AsRef<str>>(&self, s: S, env: &'c Env) -> Cow<'c, str> {
		let re = &self.0;

		// XXX: possible recursion for case "${VAR}" where $VAR="${VAR}"
		let mut replaced = String::from(s.as_ref());
		while re.is_match(replaced.as_str()) {
			if let Some(captures) = re.captures(replaced.as_str()) {
				let full = &captures[0];
				let name = &captures[2];
				cargo!(env "{name}");

				let var = env.vars
				             .get(name)
				             .map(Cow::from)
				             .or_else(|| std::env::var(name).map_err(log_err).ok().map(Cow::from))
				             .expect(&format!("Env var \"{name}\" not found"));

				replaced = replaced.replace(full, &var);
			}
		}
		replaced.into()
	}

	pub fn str_only<'c, S: AsRef<str>>(&self, s: S) -> Cow<'c, str> {
		let re = &self.0;

		let mut replaced = String::from(s.as_ref());
		while re.is_match(replaced.as_str()) {
			if let Some(captures) = re.captures(replaced.as_str()) {
				let full = &captures[0];
				let name = &captures[2];
				cargo!(env "{name}");

				let var = std::env::var(name).map_err(log_err)
				                             .map(Cow::from)
				                             .expect(&format!("Env var \"{name}\" not found"));
				replaced = replaced.replace(full, &var);
			}
		}
		replaced.into()
	}

	pub fn expr<'a, 'e, 'c: 'e, Ex: AsMut<Expr<'e>>>(&self, mut expr: Ex, env: &'c Env) -> Ex {
		let editable = expr.as_mut();
		let replaced = self.str(editable.actual(), env);
		if &replaced != editable.actual() {
			editable.set(replaced);
		}
		expr
	}
}


#[derive(Debug)]
pub enum Match {
	Match(wax::WalkEntry<'static>),
	Pair {
		/// The path to the file to include.
		source: PathBuf,

		/// Matched part of path.
		/// Related path that should be preserved in the output.
		target: PathBuf,
	},
}

impl serde::Serialize for Match {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: serde::Serializer {
		use serde::ser::SerializeStructVariant;
		use self::Match::*;

		let tag = match *self {
			Pair { .. } => "Pair",
			Match(..) => "Match",
		};
		let mut s = serializer.serialize_struct_variant("Match", 0, tag, 1)?;
		s.serialize_field("source", &self.source())?;
		s.serialize_field("target", &self.target())?;
		s.end()
	}
}


impl Eq for Match {}

impl Match {
	pub fn source(&self) -> Cow<Path> {
		match self {
			Match::Match(source) => {
				let c: Cow<Path> = source.path().into();
				c
			},
			Match::Pair { source, .. } => source.into(),
		}
	}
	pub fn target(&self) -> Cow<Path> {
		match self {
			Match::Match(source) => {
				let c: Cow<Path> = Path::new(source.matched().complete()).into();
				c
			},
			Match::Pair { target, .. } => target.into(),
		}
	}

	// TODO: test it
	fn set_target<P: Into<PathBuf>>(&mut self, path: P) {
		trace!("old target: {}", self.target().display());
		match self {
			Match::Match(entry) => {
				let mut new = Self::Pair { source: entry.path().into(),
				                           target: path.into() };
				std::mem::swap(self, &mut new);
			},
			Match::Pair { target, .. } => {
				let _ = std::mem::replace(target, path.into());
			},
		}
	}
}

impl From<WalkEntry<'_>> for Match {
	fn from(entry: WalkEntry) -> Self { Match::Match(entry.into_owned()) }
}

impl Match {
	pub fn new<S: Into<PathBuf>, T: Into<PathBuf>>(source: S, target: T) -> Self {
		Match::Pair { source: source.into(),
		              target: target.into() }
	}
}

impl PartialEq for Match {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Match(l), Self::Match(r)) => {
				l.path() == r.path() && l.matched().complete() == r.matched().complete()
			},
			(
			 Self::Pair { source: ls,
			              target: lt, },
			 Self::Pair { source: rs,
			              target: rt, },
			) => ls == rs && lt == rt,
			_ => false,
		}
	}
}

impl Hash for Match {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		core::mem::discriminant(self).hash(state);
		self.source().hash(state);
	}
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expr<'s> {
	// TODO: source: Span<..>
	Original(String),
	Modified { original: String, actual: Cow<'s, str> },
}


impl<T: ToString> From<T> for Expr<'_> {
	fn from(source: T) -> Self { Self::Original(source.to_string()) }
}

impl<'e> AsMut<Expr<'e>> for Expr<'e> {
	fn as_mut(&mut self) -> &mut Expr<'e> { self }
}

impl Expr<'_> {
	pub fn as_str(&self) -> &str { self.actual() }
}

impl AsRef<str> for Expr<'_> {
	fn as_ref(&self) -> &str { self.actual() }
}

impl Into<PathBuf> for &Expr<'_> {
	fn into(self) -> PathBuf { self.actual().into() }
}

impl Into<PathBuf> for Expr<'_> {
	fn into(self) -> PathBuf {
		let actual: PathBuf = match self {
			Expr::Original(original) => original.into(),
			Expr::Modified { actual, .. } => actual.into_owned().into(),
		};
		actual
	}
}

impl Expr<'_> {
	pub fn original(&self) -> &str {
		match self {
			Expr::Original(ref s) => s,
			Expr::Modified { ref original, .. } => original,
		}
	}
	pub fn actual(&self) -> &str {
		match self {
			Expr::Original(ref s) => s,
			Expr::Modified { ref actual, .. } => actual,
		}
	}
}

impl<'e> Expr<'e> {
	fn set<'s, S: Into<Cow<'s, str>>>(&mut self, actual: S)
		where 's: 'e {
		let original = match self {
			Expr::Original(original) => original,
			Expr::Modified { original, .. } => original,
		};

		let new = Self::Modified { original: std::mem::replace(original, String::with_capacity(0)),
		                           actual: actual.into() };

		let _ = std::mem::replace(self, new);
	}
}


impl<'e> serde::Serialize for Expr<'e> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: serde::Serializer {
		use serde::ser::SerializeStructVariant;
		use self::Expr::*;

		match &self {
			Original(original) => {
				let mut s = serializer.serialize_struct_variant("Expr", 0, "Original", 1)?;
				s.serialize_field("original", original)?;
				s.end()
			},
			Modified { original, actual } => {
				let mut s = serializer.serialize_struct_variant("Expr", 1, "Modified", 1)?;
				s.serialize_field("original", original)?;
				s.serialize_field("actual", actual)?;
				s.end()
			},
		}
	}
}
