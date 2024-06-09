use std::hash::Hash;
use std::borrow::Cow;
use std::str;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use regex::Regex;
use wax::{Glob, LinkBehavior, WalkError, WalkEntry};

use crate::config::Env;
use super::log_err;
use super::Error;


pub fn resolve_includes<S: AsRef<str>, Excl: AsRef<str>>(expr: S,
                                                         crate_root: &Path,
                                                         exclude: &[Excl],
                                                         links: LinkBehavior)
                                                         -> Result<Vec<Match>, Error> {
	let expr = sanitize_path_pattern(expr.as_ref());

	// let crate_root = crate_root.to_string_lossy();
	// #[cfg(windows)]
	// let crate_root = unixish_path_pattern(crate_root.as_ref());
	// let crate_root = Path::new(crate_root.as_ref());

	let glob = Glob::new(expr.as_ref()).map_err(|err| {
		           // According wax's issue https://github.com/olson-sean-k/wax/issues/34
		           // we doesn't support Windows absolute paths and hope to partially relative paths.
		           if cfg!(windows) {
			           let expr = PathBuf::from(expr.as_ref());
			           if expr.is_absolute() || expr.has_root() {
				           let issue = "Wax issue https://github.com/olson-sean-k/wax/issues/34";
				           Error::Error(format!("{err}, Windows absolute paths are not supported, {issue}"))
			           } else {
				           Error::from(err)
			           }
		           } else {
			           Error::from(err)
		           }
	           })?;
	let exclude = exclude.iter().map(AsRef::as_ref).chain(["**/.*/**"]);
	let walker = glob.walk_with_behavior(crate_root, links)
	                 .not(exclude)?
	                 .map(|res| res.map(Match::from));

	let files = walker.map(|res| {
		                  let mut inc = res.map_err(log_err)?;
		                  let target = inc.target();
		                  // modify target path:
		                  let new = if target.is_absolute() && target.starts_with(crate_root) {
			                  // make it relative to crate_root:
			                  if !cfg!(windows) {
				                  let len = crate_root.components().count();
				                  Some(target.components().skip(len).collect())
			                  } else {
				                  let target = target.display().to_string();
				                  target.strip_prefix(&crate_root.display().to_string())
				                        .map(|s| {
					                        let mut s = Cow::from(s);
					                        while let Some(stripped) = s.strip_prefix([MAIN_SEPARATOR, '/', '\\']) {
						                        s = stripped.to_owned().into()
					                        }
					                        s.into_owned()
				                        })
				                        .map(PathBuf::from)
			                  }
		                  } else if target.is_absolute() {
			                  Some(PathBuf::from(target.file_name().expect("target filename")))
		                  } else {
			                  // as-is
			                  None
		                  };
		                  if let Some(new) = new {
			                  inc.set_target(new)
		                  }
		                  Ok::<_, WalkError>(inc)
	                  });

	let mut resolved = Vec::new();
	for file in files {
		resolved.push(file?);
	}

	Ok(resolved)
}


// TODO: Tests for `sanitize_path_pattern`
/// Adapt path to wax walker, so kind of "patternize" or "unixish".
///
/// On Windows makes given absolute path to look like POSIX or UNC:
/// `C:/foo/bar/**` or `//./C:/foo/bar/**`.
///
/// In details:
/// - replace all `\` with `/`
/// - if pattern starts with `<driveletter>:`, escape it as `<driveletter>\`:
///
/// On unix does nothing.
pub fn sanitize_path_pattern(path: &str) -> Cow<'_, str> {
	// TODO: Before patternize use normalize/canonicalize, crates: dunce, normpath, or path-slash
	if cfg!(windows) {
		path.replace('\\', "/")
		    .replace(':', "\\:")
		    .replace("//", "/")
		    .into()
	} else {
		path.into()
	}
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
	pub fn str<S: AsRef<str>>(&self, s: S, env: &Env) -> String {
		let re = &self.0;

		// Possible recursion for case "${VAR}" where $VAR="${VAR}"
		let mut anti_recursion_counter: u8 = 42;

		let mut replaced = String::from(s.as_ref());
		while re.is_match(replaced.as_str()) && anti_recursion_counter > 0 {
			anti_recursion_counter -= 1;

			if let Some(captures) = re.captures(replaced.as_str()) {
				let full = &captures[0];
				let name = &captures[2];

				let var = env.vars
				             .get(name)
				             .map(Cow::from)
				             .or_else(|| std::env::var(name).map_err(log_err).ok().map(Cow::from))
				             .unwrap_or_else(|| name.into());

				replaced = replaced.replace(full, &var);
			} else {
				break;
			}
		}
		replaced
	}

	pub fn str_only<'c, S: AsRef<str>>(&self, s: S) -> Cow<'c, str> {
		let re = &self.0;

		let mut replaced = String::from(s.as_ref());
		while re.is_match(replaced.as_str()) {
			if let Some(captures) = re.captures(replaced.as_str()) {
				let full = &captures[0];
				let name = &captures[2];

				let var = std::env::var(name).map_err(log_err)
				                             .map(Cow::from)
				                             .unwrap_or_else(|_| name.into());
				replaced = replaced.replace(full, &var);
			}
		}
		replaced.into()
	}

	pub fn expr<'e, Ex: AsMut<Expr<'e>>>(&self, mut expr: Ex, env: &Env) -> Ex {
		let editable = expr.as_mut();
		let replaced = self.str(editable.actual(), env);
		if replaced != editable.actual() {
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

#[cfg(feature = "serde")]
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
			Match::Match(source) => Cow::Borrowed(source.path()),
			Match::Pair { source, .. } => Cow::Borrowed(source.as_path()),
		}
	}
	pub fn target(&self) -> Cow<Path> {
		match self {
			Match::Match(source) => Cow::Borrowed(Path::new(source.matched().complete())),
			Match::Pair { target, .. } => Cow::Borrowed(target.as_path()),
		}
	}

	pub fn into_parts(self) -> (PathBuf, PathBuf) {
		match self {
			Match::Match(source) => {
				let target = source.matched().complete().into();
				let source = source.into_path();
				(source, target)
			},
			Match::Pair { source, target } => (source, target),
		}
	}

	// TODO: tests for `Match::set_target`
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

impl From<&Expr<'_>> for PathBuf {
	fn from(expr: &Expr<'_>) -> Self { expr.actual().into() }
}

impl From<Expr<'_>> for PathBuf {
	fn from(expr: Expr<'_>) -> Self {
		let actual: PathBuf = match expr {
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
	// TODO: tests for `Expr::set`
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


#[cfg(feature = "serde")]
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


#[cfg(test)]
mod tests {
	use super::*;


	const LINKS: LinkBehavior = LinkBehavior::ReadTarget;

	fn crate_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }


	#[test]
	fn resolve_includes_one_exact() {
		for file in ["Cargo.toml", "src/lib.rs"] {
			let resolved = resolve_includes::<_, &str>(file, &crate_root(), &[], LINKS).unwrap();
			assert_eq!(1, resolved.len());

			let matched = resolved.first().unwrap();

			assert_eq!(Path::new(file), matched.target());
			assert_eq!(
			           Path::new(file).canonicalize().unwrap(),
			           matched.source().canonicalize().unwrap()
			);
		}
	}

	#[test]
	fn resolve_includes_one_glob() {
		for (file, expected) in [("Cargo.tom*", "Cargo.toml"), ("**/lib.rs", "src/lib.rs")] {
			let resolved = resolve_includes::<_, &str>(file, &crate_root(), &[], LINKS).unwrap();
			assert_eq!(1, resolved.len());

			let matched = resolved.first().unwrap();

			assert_eq!(Path::new(expected), matched.target());
			assert_eq!(
			           Path::new(expected).canonicalize().unwrap(),
			           matched.source().canonicalize().unwrap()
			);
		}
	}

	#[test]
	fn resolve_includes_many_glob() {
		for (file, expected) in [
		                         ("Cargo.*", &["Cargo.toml"][..]),
		                         ("**/*.rs", &["src/lib.rs", "src/assets/mod.rs"][..]),
		] {
			let resolved = resolve_includes::<_, &str>(file, &crate_root(), &[], LINKS).unwrap();
			assert!(!resolved.is_empty());


			let mut expected_passed = 0;

			for expected in expected {
				expected_passed += 1;
				let expected = PathBuf::from(expected);
				let matched = resolved.iter()
				                      .find(|matched| matched.target() == expected)
				                      .unwrap();

				assert_eq!(expected.as_path(), matched.target());
				assert_eq!(
				           expected.canonicalize().unwrap(),
				           matched.source().canonicalize().unwrap()
				);
			}

			assert_eq!(expected.len(), expected_passed);
		}
	}

	#[test]
	fn resolve_includes_many_glob_exclude() {
		let exclude = ["**/lib.*"];
		for (file, expected) in [("Cargo.*", &["Cargo.toml"]), ("**/*.rs", &["src/assets/mod.rs"])] {
			let resolved = resolve_includes::<_, &str>(file, &crate_root(), &exclude, LINKS).unwrap();
			assert!(!resolved.is_empty());


			let mut expected_passed = 0;

			for expected in expected {
				let matched = resolved.iter()
				                      .find(|matched| matched.target() == Path::new(expected))
				                      .unwrap();

				assert_eq!(Path::new(expected), matched.target());
				assert_eq!(
				           Path::new(expected).canonicalize().unwrap(),
				           matched.source().canonicalize().unwrap()
				);
				expected_passed += 1;
			}

			assert_eq!(expected.len(), expected_passed);
		}
	}

	#[test]
	#[cfg_attr(windows, should_panic)]
	fn resolve_includes_glob_abs_to_local() {
		let (file, expected) = (env!("CARGO_MANIFEST_DIR").to_owned() + "/Cargo.*", &["Cargo.toml"]);

		let resolved = resolve_includes::<_, &str>(file, &crate_root(), &[], LINKS).unwrap();
		assert_eq!(expected.len(), resolved.len());

		let mut expected_passed = 0;

		for expected in expected {
			expected_passed += 1;
			let matched = resolved.iter()
			                      .find(|matched| matched.target() == Path::new(expected))
			                      .unwrap();

			assert_eq!(Path::new(expected), matched.target());
			assert_eq!(
			           Path::new(expected).canonicalize().unwrap(),
			           matched.source().canonicalize().unwrap()
			);
		}

		assert_eq!(expected.len(), expected_passed);
	}


	#[test]
	fn resolver_expr() {
		let resolver = EnvResolver::new();

		let env = {
			let mut env = Env::try_default().unwrap();
			env.vars.insert("FOO".into(), "foo".into());
			env.vars.insert("BAR".into(), "bar".into());
			env
		};

		let exprs = [
		             ("${FOO}/file.txt", "foo/file.txt"),
		             ("${BAR}/file.txt", "bar/file.txt"),
		];

		for (src, expected) in exprs {
			let mut expr = Expr::from(src);
			resolver.expr(&mut expr, &env);

			assert_eq!(expected, expr.actual());
			assert_eq!(expected, expr.as_str());
			assert_eq!(src, expr.original());
		}
	}

	#[test]
	fn resolver_missed() {
		let resolver = EnvResolver::new();
		let env = Env::try_default().unwrap();
		let mut expr = Expr::from("${MISSED}/file.txt");
		resolver.expr(&mut expr, &env);

		assert_eq!("MISSED/file.txt", expr.actual());
		assert_eq!("MISSED/file.txt", expr.as_str());
		assert_eq!("${MISSED}/file.txt", expr.original());
	}

	#[test]
	fn resolver_recursion() {
		let resolver = EnvResolver::new();
		let mut env = Env::try_default().unwrap();
		env.vars.insert("VAR".into(), "${VAR}".into());
		let expr = Expr::from("${VAR}/file.txt");
		resolver.expr(expr, &env);
	}
}
