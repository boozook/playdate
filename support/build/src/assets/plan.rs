use std::hash::Hash;
use std::borrow::Cow;
use std::str::FromStr;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use wax::{Glob, Pattern};

use crate::config::Env;
use crate::metadata::format::{AssetsOptions, AssetsRules, RuleValue};

use super::resolver::*;


/// Create build plan for assets.
pub fn build_plan<'l, 'r, S>(env: &Env,
                             assets: &AssetsRules<S>,
                             options: &AssetsOptions,
                             crate_root: Option<Cow<'_, Path>>)
                             -> Result<BuildPlan<'l, 'r>, super::Error>
	where S: Eq + Hash + ToString
{
	// copy_unresolved    => get all files with glob
	// include_unresolved => same
	// exclude_unresolved =>
	// 							- filter include_unresolved (actually already resolved)
	// 							- filter list of files for items in copy_unresolved
	// 								=> mark as have exclusions, so linking "file-file" instead of "dir-dir"

	let mut map_unresolved = Vec::new();
	let mut include_unresolved = Vec::new();
	let mut exclude_exprs = Vec::new();

	const PATH_SEPARATOR: [char; 2] = [MAIN_SEPARATOR, '/'];

	let enver = EnvResolver::new();
	let crate_root = crate_root.unwrap_or_else(|| env.cargo_manifest_dir().into());
	let link_behavior = options.link_behavior();

	let to_relative = |s: &S| -> String {
		let s = s.to_string();
		let p = Path::new(&s);
		if p.is_absolute() || p.has_root() {
			let trailing_sep = p.components().count() > 1 && s.ends_with(PATH_SEPARATOR);
			let mut s = p.components().skip(1).collect::<PathBuf>().display().to_string();
			// preserve trailing separator
			if trailing_sep && !s.ends_with(PATH_SEPARATOR) {
				s.push(MAIN_SEPARATOR);
			}
			sanitize_path_pattern(&s).into_owned()
		} else {
			s.to_owned()
		}
	};

	match assets {
		AssetsRules::List(vec) => {
			include_unresolved.extend(
			                          vec.iter()
			                             .map(to_relative)
			                             .map(Expr::from)
			                             .map(|e| enver.expr(e, env)),
			)
		},
		AssetsRules::Map(map) => {
			for (k, v) in map {
				let k = to_relative(k);
				match v {
					RuleValue::Boolean(v) => {
						match v {
							true => include_unresolved.push(enver.expr(Expr::from(k), env)),
							false => exclude_exprs.push(enver.expr(Expr::from(k), env)),
						}
					},
					RuleValue::String(from) => {
						map_unresolved.push((enver.expr(Expr::from(k), env), enver.expr(Expr::from(from), env)))
					},
				}
			}
		},
	}


	// prepare globs:
	// TODO: possible opt - split exclude_exprs into absolute and relative
	let exclude_globs: Vec<_> =
		exclude_exprs.iter()
		             .filter_map(|expr| {
			             Glob::from_str(expr.as_str()).map_err(|err| error!("invalid filter expression: {err}"))
			                                          .ok()
		             })
		             .collect();


	// resolve map-pairs:
	let mut mappings = Vec::new();
	for (k, v) in map_unresolved.into_iter() {
		let key = PathBuf::from(k.as_str());
		let value = Cow::Borrowed(v.as_str());
		let into_dir = k.as_str().ends_with(PATH_SEPARATOR);
		let source_exists = abs_if_existing(Path::new(value.as_ref()), &crate_root)?.is_some();

		let mapping = match (source_exists, into_dir) {
			(true, true) => Mapping::Into(Match::new(value.as_ref(), key), (k, v)),
			(true, false) => Mapping::AsIs(Match::new(value.as_ref(), key), (k, v)),
			(false, _) => {
				let mut resolved = resolve_includes(value, &crate_root, &exclude_exprs, link_behavior)?;

				debug!("Possible ManyInto, resolved: {}", resolved.len());

				// filter resolved includes:
				let _excluded: Vec<_> = resolved.extract_if(|inc| {
					                                let path = key.join(inc.target());
					                                glob_matches_any(&path, &exclude_globs)
				                                })
				                                .collect();


				Mapping::ManyInto { sources: resolved,
				                    target: (&k).into(),
				                    exprs: (k, v),
				                    #[cfg(feature = "assets-report")]
				                    excluded: _excluded }
			},
		};

		mappings.push(mapping);
	}


	// re-mapping if needed:
	for mapping in mappings.iter_mut() {
		let possible = match &mapping {
			Mapping::AsIs(inc, ..) => inc.source().is_dir() && possibly_matching_any(&inc.target(), &exclude_exprs),
			Mapping::Into(inc, ..) => inc.source().is_dir() && possibly_matching_any(&inc.target(), &exclude_exprs),
			Mapping::ManyInto { .. } => false,
		};

		if possible {
			let (source_root, target, exprs) = match &mapping {
				// 0. we're have path of existing dir `source`.
				// 1. get all files from root `source` => `path` of files related to `source`
				// 2. `target` path of file will depends on this `mapping`:
				Mapping::AsIs(inc, expr) => {
					// 2. `target` path of file:
					// replace `source` with `target` in the abs path
					(inc.source(), inc.target(), expr)
				},
				Mapping::Into(inc, expr) => {
					// 2. `target` path of file:
					// `target`/{source.name}/{rel path of file}
					let source = inc.source();
					let target = inc.target();
					let target_base = target.join(source.file_name().expect("source filename"));
					(source, Cow::from(target_base), expr)
				},
				Mapping::ManyInto { .. } => unreachable!(),
			};

			// find all/any files in the source:
			let mut resolved = resolve_includes("**/*", &source_root, &exclude_exprs, link_behavior)?;

			// filter resolved includes:
			let is_not_empty = |inc: &Match| !inc.target().as_os_str().is_empty();
			let excluded: Vec<_> = resolved.extract_if(|inc| {
				                               let target = target.join(inc.target());
				                               !is_not_empty(inc) ||
				                               glob_matches_any(&inc.source(), &exclude_globs) ||
				                               glob_matches_any(&target, &exclude_globs)
			                               })
			                               .collect();

			// skip if no exclusions:
			if excluded.is_empty() {
				continue;
			}

			*mapping = Mapping::ManyInto { sources: resolved,
			                               target: target.into(),
			                               exprs: exprs.to_owned(),
			                               #[cfg(feature = "assets-report")]
			                               excluded };
		}
	}


	for k in include_unresolved {
		let resolved = resolve_includes(&k, &crate_root, &exclude_exprs, link_behavior)?;
		mappings.extend(resolved.into_iter()
		                        .map(|inc| Mapping::AsIs(inc, (k.clone(), "true".into()))));
	}


	// TODO: sort before dedup?
	mappings.dedup_by(|a, b| a.eq_ignore_expr(b));

	// TODO: find source duplicates and warn!

	Ok(BuildPlan { plan: mappings,
	               crate_root: crate_root.to_path_buf() })
}


/// Make path relative to `crate_root` if it isn't absolute, checking existence.
/// Returns `None` if path doesn't exist.
///
/// Input `path` must be absolute or relative to the `root`.
pub fn abs_if_existing<'t, P1, P2>(path: P1, root: P2) -> std::io::Result<Option<Cow<'t, Path>>>
	where P1: 't + AsRef<Path> + Into<Cow<'t, Path>>,
	      P2: AsRef<Path> {
	let p = if path.as_ref().is_absolute() && path.as_ref().try_exists()? {
		Some(path.into())
	} else {
		let abs = root.as_ref().join(path);
		if abs.try_exists()? {
			Some(Cow::Owned(abs))
		} else {
			None
		}
	};
	Ok(p)
}

/// Same as [`abs_or_rel_crate_existing`], but returns given `path` as fallback.
#[inline]
pub fn abs_if_existing_any<'t, P1, P2>(path: P1, root: P2) -> Cow<'t, Path>
	where P1: 't + AsRef<Path> + Into<Cow<'t, Path>> + Clone,
	      P2: AsRef<Path> {
	abs_if_existing(path.clone(), root).ok()
	                                   .flatten()
	                                   .unwrap_or(path.into())
}


fn glob_matches_any<'a, I: IntoIterator<Item = &'a Glob<'a>>>(path: &Path, exprs: I) -> bool {
	exprs.into_iter().any(|glob| glob.is_match(path))
}


/// Compare (apply) each `expr` with `path` using exact same or more number of [`components`] as in `path`.
/// Returns `true` if any of `exprs` matches the `path`.
///
/// Uses [`possibly_matching`].
///
/// [`components`]: PathBuf::components
fn possibly_matching_any<P: Into<PathBuf>, I: IntoIterator<Item = P>>(path: &Path, exprs: I) -> bool {
	exprs.into_iter().any(|expr| possibly_matching(path, expr))
}


// TODO: tests for `possibly_matching`
/// Check that filter (possibly) pattern `expr` matches the `path`.
fn possibly_matching<P: Into<PathBuf>>(path: &Path, expr: P) -> bool {
	// TODO: remove {crate_root} part if it is from filter (or both?).

	let len = path.components().count();
	let filter: PathBuf = expr.into()
	                          .components()
	                          .enumerate() // TODO: just `skip`
	                          .filter(|(i, _)| *i < len)
	                          .map(|(_, p)| p)
	                          .collect();

	let glob = Glob::new(filter.as_os_str().to_str().unwrap()).unwrap();
	glob.is_match(path)
}


/// Assets Build Plan for a crate.
#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BuildPlan<'left, 'right> {
	/// Instructions - what file where to put
	plan: Vec<Mapping<'left, 'right>>,
	/// Root directory of associated crate
	crate_root: PathBuf,
}

impl<'left, 'right> BuildPlan<'left, 'right> {
	pub fn into_inner(self) -> Vec<Mapping<'left, 'right>> { self.plan }
	pub fn as_inner(&self) -> &[Mapping<'left, 'right>] { &self.plan[..] }
	pub fn into_parts(self) -> (Vec<Mapping<'left, 'right>>, PathBuf) { (self.plan, self.crate_root) }

	pub fn crate_root(&self) -> &Path { &self.crate_root }
	pub fn set_crate_root<T: Into<PathBuf>>(&mut self, path: T) -> PathBuf {
		let old = std::mem::replace(&mut self.crate_root, path.into());
		old
	}
}

impl<'left, 'right> AsRef<[Mapping<'left, 'right>]> for BuildPlan<'left, 'right> {
	fn as_ref(&self) -> &[Mapping<'left, 'right>] { &self.plan[..] }
}


impl std::fmt::Display for BuildPlan<'_, '_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let align = |f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
			if !matches!(f.align(), Some(std::fmt::Alignment::Right)) {
				return Ok(());
			}

			let c = f.fill();
			if let Some(width) = f.width() {
				for _ in 0..width {
					write!(f, "{c}")?;
				}
				Ok(())
			} else {
				write!(f, "{c}")
			}
		};


		let print =
			|f: &mut std::fmt::Formatter<'_>, inc: &Match, (left, right): &(Expr, Expr)| -> std::fmt::Result {
				let target = inc.target();
				let source = inc.source();
				let left = left.original();
				let right = right.original();
				align(f)?;
				write!(f, "{target:#?} <- {source:#?}  ({left} = {right})")
			};

		for item in self.as_inner() {
			match item {
				Mapping::AsIs(inc, exprs) => print(f, inc, exprs)?,
				Mapping::Into(inc, exprs) => print(f, inc, exprs)?,
				Mapping::ManyInto { sources,
				                    target,
				                    exprs,
				                    .. } => {
					for inc in sources {
						print(f, &Match::new(inc.source(), target.join(inc.target())), exprs)?;
						writeln!(f)?;
					}
				},
			}
		}

		Ok(())
	}
}

impl BuildPlan<'_, '_> {
	pub fn targets(&self) -> impl Iterator<Item = Cow<'_, Path>> {
		self.as_inner().iter().flat_map(|mapping| {
			                      match mapping {
				                      Mapping::AsIs(inc, ..) => vec![inc.target()].into_iter(),
			                         Mapping::Into(inc, ..) => vec![inc.target()].into_iter(),
			                         Mapping::ManyInto { sources, target, .. } => {
				                         sources.iter()
				                                .map(|inc| Cow::from(target.join(inc.target())))
				                                .collect::<Vec<_>>()
				                                .into_iter()
			                         },
			                      }
		                      })
	}

	pub fn iter_flatten(
		&self)
		-> impl Iterator<Item = (MappingKind, PathBuf, (PathBuf, Option<std::time::SystemTime>))> + '_ {
		let pair = |inc: &Match| {
			(inc.target().to_path_buf(), abs_if_existing_any(inc.source(), &self.crate_root).to_path_buf())
		};

		self.as_inner()
		    .iter()
		    .flat_map(move |mapping| {
			    let mut rows = Vec::new();
			    let kind = mapping.kind();
			    match mapping {
				    Mapping::AsIs(inc, _) | Mapping::Into(inc, _) => rows.push(pair(inc)),
			       Mapping::ManyInto { sources, target, .. } => {
				       rows.extend(sources.iter()
				                          .map(|inc| pair(&Match::new(inc.source(), target.join(inc.target())))));
			       },
			    };
			    rows.into_iter().map(move |(l, r)| (kind, l, r))
		    })
		    .map(|(k, t, p)| {
			    let time = p.metadata().ok().and_then(|m| m.modified().ok());
			    (k, t, (p, time))
		    })
	}
}


#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Mapping<'left, 'right>
	where Self: 'left + 'right {
	// if right part exact path to ONE existing fs item
	/// Copy source to target as-is.
	AsIs(Match, (Expr<'left>, Expr<'right>)), // left part without trailing /

	/// Copy source into target as-is.
	Into(Match, (Expr<'left>, Expr<'right>)),
	// if right part not exact (exist one)
	ManyInto {
		sources: Vec<Match>,

		/// Target __directory__. Related path that should be preserved in the output.
		target: PathBuf,

		#[cfg(feature = "assets-report")]
		excluded: Vec<Match>, // TODO: add reason for exclusions

		exprs: (Expr<'left>, Expr<'right>),
	},
}

impl Mapping<'_, '_> {
	// TODO: tests for `Mapping::eq_ignore_expr`
	pub fn eq_ignore_expr(&self, other: &Self) -> bool {
		match (self, other) {
			(Mapping::AsIs(a, _), Mapping::AsIs(b, _)) | (Mapping::Into(a, _), Mapping::Into(b, _)) => a.eq(b),
			(Mapping::AsIs(a, _), Mapping::Into(b, _)) | (Mapping::Into(b, _), Mapping::AsIs(a, _)) => a.eq(b),

			(Mapping::AsIs(..), Mapping::ManyInto { .. }) => false,
			(Mapping::Into(..), Mapping::ManyInto { .. }) => false,
			(Mapping::ManyInto { .. }, Mapping::AsIs(..)) => false,
			(Mapping::ManyInto { .. }, Mapping::Into(..)) => false,

			(
			 Mapping::ManyInto { sources: sa,
			                     target: ta,
			                     .. },
			 Mapping::ManyInto { sources: sb,
			                     target: tb,
			                     .. },
			) => sa.eq(sb) && ta.eq(tb),
		}
	}

	pub fn exprs(&self) -> (&Expr<'_>, &Expr<'_>) {
		match self {
			Mapping::AsIs(_, (left, right)) | Mapping::Into(_, (left, right)) => (left, right),
			Mapping::ManyInto { exprs: (left, right), .. } => (left, right),
		}
	}

	pub fn sources(&self) -> Vec<&Match> {
		match self {
			Mapping::AsIs(source, ..) | Mapping::Into(source, ..) => vec![source],
			Mapping::ManyInto { sources, .. } => sources.iter().collect(),
		}
	}

	pub fn kind(&self) -> MappingKind {
		match self {
			Mapping::AsIs(..) => MappingKind::AsIs,
			Mapping::Into(..) => MappingKind::Into,
			Mapping::ManyInto { .. } => MappingKind::ManyInto,
		}
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MappingKind {
	/// Copy source __to__ target.
	AsIs,
	/// Copy source __into__ target as-is, preserving related path.
	Into,
	/// Copy sources __into__ target as-is, preserving matched path.
	ManyInto,
}

impl std::fmt::Display for MappingKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AsIs => "as-is".fmt(f),
			Self::Into => "into".fmt(f),
			Self::ManyInto => "many-into".fmt(f),
		}
	}
}


#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use std::collections::HashSet;
	use std::path::{PathBuf, Path};

	use crate::config::Env;
	use crate::assets::resolver::Expr;
	use crate::metadata::format::RuleValue;
	use crate::metadata::format::AssetsRules;
	use super::*;


	fn crate_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }


	mod abs_if_existing {
		use std::borrow::Cow;

		use super::*;
		use super::abs_if_existing;


		#[test]
		fn local() {
			let roots = [
			             Cow::from(Path::new(env!("CARGO_MANIFEST_DIR"))),
			             crate_root().into(),
			];
			let paths = ["Cargo.toml", "src/lib.rs"];

			for root in roots {
				for test in paths {
					let path = Path::new(test);

					let crated = abs_if_existing(path, &root).unwrap();
					assert!(crated.is_some(), "{crated:?} should be exist (src: {path:?})");

					let crated = crated.unwrap();
					let expected = root.join(path);
					assert_eq!(expected, crated);
				}
			}
		}

		#[test]
		fn external_rel() {
			let roots = [
			             Cow::from(Path::new(env!("CARGO_MANIFEST_DIR"))),
			             crate_root().into(),
			];
			let paths = ["../utils/Cargo.toml", "./../utils/src/lib.rs"];

			for root in roots {
				for test in paths {
					let path = Path::new(test);

					let crated = abs_if_existing(path, &root).unwrap();
					assert!(crated.is_some(), "{crated:?} should be exist (src: {path:?})");

					let crated = crated.unwrap();
					let expected = root.join(path);
					assert_eq!(expected, crated);
				}
			}
		}

		#[test]
		fn external_abs() {
			let roots = [
			             Cow::from(Path::new(env!("CARGO_MANIFEST_DIR"))),
			             crate_root().into(),
			];
			let paths = ["utils", "utils/Cargo.toml"];

			for root in roots {
				for test in paths {
					let path = root.parent().unwrap().join(test);

					let crated = abs_if_existing(&path, &root).unwrap();
					assert!(crated.is_some(), "{crated:?} should be exist (src: {path:?})");

					let crated = crated.unwrap();
					let expected = path.as_path();
					assert_eq!(expected, crated);
				}
			}
		}
	}


	mod plan {
		use super::*;
		use std::env::temp_dir;


		fn prepared_tmp(test_name: &str) -> (PathBuf, PathBuf, [&'static str; 4], Env) {
			let temp = temp_dir().join(env!("CARGO_PKG_NAME"))
			                     .join(env!("CARGO_PKG_VERSION"))
			                     .join(test_name);

			let sub = temp.join("dir");

			if !temp.exists() {
				println!("creating temp dir: {temp:?}")
			} else {
				println!("temp dir: {temp:?}")
			}
			std::fs::create_dir_all(&temp).unwrap();
			std::fs::create_dir_all(&sub).unwrap();

			// add temp files
			let files = ["foo.txt", "bar.txt", "dir/baz.txt", "dir/boo.txt"];
			for name in files {
				std::fs::write(temp.join(name), []).unwrap();
			}

			let env = {
				let mut env = Env::try_default().unwrap();
				env.vars.insert("TMP".into(), temp.to_string_lossy().into_owned());
				env.vars.insert("SUB".into(), sub.to_string_lossy().into_owned());
				env
			};

			(temp, sub, files, env)
		}


		mod list {
			use super::*;


			mod as_is {
				use super::*;


				#[test]
				fn local_exact() {
					let env = Env::try_default().unwrap();
					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(Cow::Borrowed(root.as_path()));

					let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();

					let exprs = tests.iter().map(|s| s.to_string()).collect();
					let assets = AssetsRules::List(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						assert!(matches!(
							pair,
							Mapping::AsIs(_, (Expr::Original(left), Expr::Original(right)))
							if right == "true" && tests.contains(left.as_str())
						));
					}
				}


				#[test]
				fn resolve_local_abs() {
					let env = {
						let mut env = Env::try_default().unwrap();
						env.vars.insert(
						                "SRC_ABS".into(),
						                concat!(env!("CARGO_MANIFEST_DIR"), "/src").into(),
						);
						env
					};

					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					let tests: HashMap<_, _> = {
						let man_abs = PathBuf::from("Cargo.toml").canonicalize()
						                                         .unwrap()
						                                         .to_string_lossy()
						                                         .to_string();
						let lib_abs = PathBuf::from("src/lib.rs").canonicalize()
						                                         .unwrap()
						                                         .to_string_lossy()
						                                         .to_string();
						vec![
						     ("${CARGO_MANIFEST_DIR}/Cargo.toml", man_abs),
						     ("${SRC_ABS}/lib.rs", lib_abs),
						].into_iter()
						.collect()
					};

					let exprs = tests.keys().map(|s| s.to_string()).collect();
					let assets = AssetsRules::List(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						assert!(matches!(
							pair,
							Mapping::AsIs(matched, (Expr::Modified{original, actual}, Expr::Original(right)))
							if right == "true"
							&& tests[original.as_str()] == actual.as_ref()
							&& matched.source() == Path::new(&tests[original.as_str()]).canonicalize().unwrap()
						));
					}
				}


				#[test]
				fn resolve_local() {
					let env = {
						let mut env = Env::try_default().unwrap();
						env.vars.insert("SRC".into(), "src".into());
						env
					};

					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					let tests: HashMap<_, _> = { vec![("${SRC}/lib.rs", "src/lib.rs"),].into_iter().collect() };

					let exprs = tests.keys().map(|s| s.to_string()).collect();
					let assets = AssetsRules::List(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						if let Mapping::AsIs(matched, (Expr::Modified { original, actual }, Expr::Original(right))) =
							pair
						{
							assert_eq!("true", right);
							assert_eq!(tests[original.as_str()], actual.as_ref());
							assert_eq!(
							           matched.source().canonicalize().unwrap(),
							           Path::new(&tests[original.as_str()]).canonicalize().unwrap()
							);
							assert_eq!(matched.target(), Path::new(&tests[original.as_str()]));
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}


				#[test]
				#[cfg_attr(windows, should_panic)]
				fn resolve_exact_external_abs() {
					let (temp, sub, _files, env) = prepared_tmp("as_is-resolve_external");

					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());


					// tests:

					let tests: HashMap<_, _> = {
						vec![
						     ("${TMP}/foo.txt", (temp.join("foo.txt"), "foo.txt")),
						     ("${TMP}/bar.txt", (temp.join("bar.txt"), "bar.txt")),
						     ("${SUB}/baz.txt", (sub.join("baz.txt"), "baz.txt")),
						     ("${TMP}/dir/boo.txt", (sub.join("boo.txt"), "boo.txt")),
						].into_iter()
						.collect()
					};

					let exprs = tests.keys().map(|s| s.to_string()).collect();
					let assets = AssetsRules::List(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					// check targets len
					{
						let targets = plan.targets().collect::<Vec<_>>();
						let expected = tests.values().map(|(_, name)| name).collect::<Vec<_>>();
						assert_eq!(expected.len(), targets.len());
					}

					// full check
					for pair in plan.as_inner() {
						if let Mapping::AsIs(matched, (Expr::Modified { original, actual }, Expr::Original(right))) =
							pair
						{
							assert_eq!("true", right);
							assert_eq!(tests[original.as_str()].0.to_string_lossy(), actual.as_ref());
							assert_eq!(matched.source(), tests[original.as_str()].0);
							assert_eq!(matched.target().to_string_lossy(), tests[original.as_str()].1);
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}


				#[test]
				#[cfg_attr(windows, should_panic)]
				fn resolve_glob_external_many() {
					let (_, _, files, env) = prepared_tmp("as_is-resolve_external_many");

					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					let exprs = ["${TMP}/*.txt", "${SUB}/*.txt"];

					let assets = AssetsRules::List(exprs.iter().map(|s| s.to_string()).collect());

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					// check targets len
					{
						let targets = plan.targets().collect::<Vec<_>>();
						assert_eq!(files.len(), targets.len());
					}

					// full check
					for pair in plan.as_inner() {
						if let Mapping::AsIs(matched, (Expr::Modified { original, actual }, Expr::Original(right))) =
							pair
						{
							assert!(exprs.contains(&original.as_str()));
							assert!(Path::new(actual.as_ref()).is_absolute());
							assert_eq!("true", right);

							if let Match::Pair { source, target } = matched {
								// target is just filename:
								assert_eq!(1, target.components().count());
								assert_eq!(target.file_name(), source.file_name());
							} else {
								panic!("pair.matched is not matching: {matched:#?}");
							}
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}
			}
		}


		mod map {
			use super::*;


			mod as_is {
				use super::*;


				#[test]
				fn local_exact() {
					let env = Env::try_default().unwrap();
					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();

					let exprs = tests.iter()
					                 .map(|s| (s.to_string(), RuleValue::Boolean(true)))
					                 .collect();

					let assets = AssetsRules::Map(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						if let Mapping::AsIs(matched, (Expr::Original(left), Expr::Original(right))) = pair {
							assert_eq!("true", right);
							assert!(tests.contains(left.as_str()));
							assert_eq!(
							           left.as_str(),
							           sanitize_path_pattern(matched.target().to_string_lossy().as_ref())
							);
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}


				#[test]
				fn local_exact_target() {
					let env = Env::try_default().unwrap();
					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					// left hand of rule:
					let targets = ["trg", "/trg", "//trg"];
					// right hand of rule:
					let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();
					// latest because there is no to files into one target, so "into" will be used

					for trg in targets {
						let stripped_trg = &trg.replace('/', "").trim().to_owned();

						let exprs = tests.iter()
						                 .map(|s| (trg.to_string(), RuleValue::String(s.to_string())))
						                 .collect();

						let assets = AssetsRules::Map(exprs);

						let plan = build_plan(&env, &assets, &opts, root.clone()).unwrap();

						for pair in plan.as_inner() {
							if let Mapping::AsIs(
							                     Match::Pair { source, target },
							                     (Expr::Original(left), Expr::Original(right)),
							) = pair
							{
								assert_eq!(left, stripped_trg);
								assert!(tests.contains(right.as_str()));
								assert_eq!(source, Path::new(right));
								assert_eq!(target, Path::new(stripped_trg));
							} else {
								panic!("pair is not matching: {pair:#?}");
							}
						}
					}
				}
			}


			mod one_into {
				use super::*;


				#[test]
				fn local_exact_target() {
					let env = Env::try_default().unwrap();
					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					// left hand of rule:
					let targets = ["trg/", "trg//", "/trg/", "//trg/"];
					let targets_rel = ["trg/", "trg//"]; // non-abs targets
					// right hand of rule:
					let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();

					for trg in targets {
						let exprs = tests.iter()
						                 .map(|s| (trg.to_string(), RuleValue::String(s.to_string())))
						                 .collect();

						let assets = AssetsRules::Map(exprs);

						let plan = build_plan(&env, &assets, &opts, root.clone()).unwrap();

						for pair in plan.as_inner() {
							if let Mapping::Into(
							                     Match::Pair { source, target },
							                     (Expr::Original(left), Expr::Original(right)),
							) = pair
							{
								assert_eq!(left, target.to_string_lossy().as_ref());
								assert!(targets_rel.contains(&left.as_str()));
								assert!(tests.contains(right.as_str()));
								assert_eq!(source, Path::new(right));
							} else {
								panic!("pair is not matching: {pair:#?}");
							}
						}
					}
				}
			}


			mod many_into {
				use super::*;

				#[test]
				#[cfg_attr(windows, should_panic)]
				fn glob_local_target() {
					let env = Env::try_default().unwrap();
					let opts = AssetsOptions::default();

					let root = crate_root();
					let root = Some(root.as_path().into());

					// left hand of rule:
					let targets = ["/trg/", "//trg/", "/trg", "trg"];
					let targets_rel = ["trg/", "trg"]; // non-abs targets
					// right hand of rule:
					let tests: HashSet<_> = vec!["Cargo.tom*", "src/lib.*"].into_iter().collect();
					// latest because there is no to files into one target, so "into" will be used

					for trg in targets {
						let exprs = tests.iter()
						                 .map(|s| (trg.to_string(), RuleValue::String(s.to_string())))
						                 .collect();

						let assets = AssetsRules::Map(exprs);

						let plan = build_plan(&env, &assets, &opts, root.clone()).unwrap();

						for pair in plan.as_inner() {
							if let Mapping::ManyInto { sources,
							                           target,
							                           #[cfg(feature = "assets-report")]
							                           excluded,
							                           exprs: (Expr::Original(left), Expr::Original(right)), } = pair
							{
								assert!(targets_rel.contains(&target.to_string_lossy().as_ref()));
								assert_eq!(&target.to_string_lossy(), left);

								assert_eq!(1, sources.len());
								assert!(tests.contains(right.as_str()));

								#[cfg(feature = "assets-report")]
								assert_eq!(0, excluded.len());
							} else {
								panic!("pair is not matching: {pair:#?}");
							}
						}
					}
				}
			}
		}
	}
}
