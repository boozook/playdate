use std::hash::Hash;
use std::borrow::Cow;
use std::str::FromStr;
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

use wax::{Glob, Pattern};

use crate::config::Env;
use crate::metadata::format::AssetsOptions;
use crate::metadata::format::PlayDateMetadataAssets;
use crate::value::Value;

use super::resolver::*;


/// Create build plan for assets.
pub fn build_plan<'l, 'r, 'c: 'l, V>(env: &'c Env,
                                     assets: &PlayDateMetadataAssets<V>,
                                     options: &AssetsOptions,
                                     crate_root: Option<&Path>)
                                     -> Result<BuildPlan<'l, 'r>, super::Error>
	where V: Value
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

	let enver = EnvResolver::new();
	let crate_root = crate_root.unwrap_or_else(|| env.cargo_manifest_dir());
	let link_behavior = options.link_behavior();

	let to_relative = |s: &String| -> String {
		let p = Path::new(&s);
		if p.is_absolute() || p.has_root() {
			p.components().skip(1).collect::<PathBuf>().display().to_string()
		} else {
			s.to_owned()
		}
	};

	match assets {
		PlayDateMetadataAssets::List(vec) => {
			include_unresolved.extend(
			                          vec.iter()
			                             .map(to_relative)
			                             .map(Expr::from)
			                             .map(|e| enver.expr(e, env)),
			)
		},
		PlayDateMetadataAssets::Map(map) => {
			for (k, v) in map {
				let k = to_relative(k);
				if let Some(v) = v.as_bool() {
					match v {
						true => include_unresolved.push(enver.expr(Expr::from(k), env)),
						false => exclude_exprs.push(enver.expr(Expr::from(k), env)),
					}
				} else if let Some(from) = v.as_str() {
					map_unresolved.push((enver.expr(Expr::from(k), env), enver.expr(Expr::from(from), env)))
				} else {
					return Err(format!("not supported type of value: {v} for key: {k}").into());
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
		let value = v.as_str();
		let into_dir = k.as_str().ends_with(MAIN_SEPARATOR_STR);
		let source_exists = {
			let p = Path::new(value);
			p.is_absolute()
			 .then(|| p.try_exists())
			 .unwrap_or_else(|| crate_root.join(p).try_exists())?
		};

		let mapping = match (source_exists, into_dir) {
			(true, true) => Mapping::Into(Match::new(value, key), (k, v)),
			(true, false) => Mapping::AsIs(Match::new(value, key), (k, v)),
			(false, _) => {
				let mut resolved = resolve_includes(value, crate_root, &exclude_exprs, link_behavior)?;

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
		let resolved = resolve_includes(&k, crate_root, &exclude_exprs, link_behavior)?;
		mappings.extend(resolved.into_iter()
		                        .map(|inc| Mapping::AsIs(inc, (k.clone(), "true".into()))));
	}


	// TODO: sort before dedup?
	mappings.dedup_by(|a, b| a.eq_ignore_expr(b));

	// TODO: find source duplicates and warn!

	Ok(BuildPlan(mappings))
}


/// Make path relative to `crate_root` if it isn't absolute, checking existence.
/// Returns `None` if path doesn't exist.
pub fn existing_abs_rel_to_crate_root<'t, P1, P2>(p: P1, root: P2) -> std::io::Result<Option<Cow<'t, Path>>>
	where P1: 't + AsRef<Path> + Into<Cow<'t, Path>>,
	      P2: AsRef<Path> {
	let p = if p.as_ref().is_absolute() && p.as_ref().try_exists()? {
		Some(p.into())
	} else {
		let abs = root.as_ref().join(p);
		if abs.try_exists()? {
			Some(Cow::Owned(abs))
		} else {
			None
		}
	};
	Ok(p)
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


#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct BuildPlan<'left, 'right>(Vec<Mapping<'left, 'right>>);

impl<'left, 'right> BuildPlan<'left, 'right> {
	pub fn into_inner(self) -> Vec<Mapping<'left, 'right>> { self.0 }
	pub fn as_inner(&self) -> &[Mapping<'left, 'right>] { &self.0[..] }
}

impl<'left, 'right> AsRef<[Mapping<'left, 'right>]> for BuildPlan<'left, 'right> {
	fn as_ref(&self) -> &[Mapping<'left, 'right>] { &self.0[..] }
}

impl BuildPlan<'_, '_> {
	pub fn print(&self) {
		info!("assets build plan:");

		let print = |inc: &Match, (left, right): &(Expr, Expr)| {
			info!(
			      "  {} <- {}  ({left} = {right})",
			      inc.target().display(),
			      inc.source().display(),
			      left = left.original(),
			      right = right.original()
			)
		};

		self.as_inner().iter().for_each(|mapping| {
			                      match mapping {
				                      Mapping::AsIs(inc, exprs) => print(inc, exprs),
			                         Mapping::Into(inc, exprs) => print(inc, exprs),
			                         Mapping::ManyInto { sources,
			                                             target,
			                                             exprs,
			                                             .. } => {
				                         sources.iter().for_each(|inc| {
					                                       print(
					                                             &Match::new(inc.source(), target.join(inc.target())),
					                                             exprs,
					);
				                                       })
			                         },
			                      };
		                      });
	}

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
			(inc.target().to_path_buf(), abs_or_rel_crate_any(inc.source(), self.crate_root).to_path_buf())
		};

		self.as_inner()
		    .iter()
		    .flat_map(move |mapping| {
			    let mut rows = Vec::new();
			    match mapping {
				    Mapping::AsIs(inc, _) | Mapping::Into(inc, _) => rows.push(pair(inc)),
			       Mapping::ManyInto { sources, target, .. } => {
				       rows.extend(sources.iter()
				                          .map(|inc| pair(&Match::new(inc.source(), target.join(inc.target())))));
			       },
			    };
			    rows.into_iter()
		    })
		    .map(|(t, p)| {
			    let time = p.metadata().ok().and_then(|m| m.modified().ok());
			    (t, (p, time))
		    })
	}
}


#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize)]
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
}
