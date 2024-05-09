use std::collections::HashMap;
use std::path::Path;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use wax::{LinkBehavior, WalkError};
use fs_extra::error::Error as FsExtraError;

use crate::io::soft_link_checked;
use crate::metadata::format::AssetsBuildMethod;
use crate::metadata::format::AssetsOptions;


pub mod plan;
pub mod resolver;
mod tests;

use self::plan::*;


pub fn apply_build_plan<'l, 'r, P: AsRef<Path>>(plan: BuildPlan<'l, 'r>,
                                                target_root: P,
                                                assets_options: &AssetsOptions)
                                                -> Result<BuildReport<'l, 'r>, FsExtraError> {
	use crate::io::parent_of;
	use crate::io::ensure_dir_exists;

	let target_root = target_root.as_ref();
	let build_method = assets_options.method;
	let overwrite = assets_options.overwrite;
	info!("collecting assets:");
	debug!("assets build method: {build_method:?}, overwrite: {overwrite}");


	let def_options = fs_extra::dir::CopyOptions { overwrite,
	                                               skip_exist: true,
	                                               copy_inside: false,
	                                               content_only: false,
	                                               ..Default::default() };

	// ensures that there's no symlink or any `..` component in path, just resolve and compare with root:
	let ensure_out_of_root = |path: &Path| -> std::io::Result<()> {
		if path.is_symlink() {
			let real = path.read_link()
			               .map_or_else(|err| format!("{err}"), |p| p.display().to_string());
			return Err(IoError::new(
				IoErrorKind::AlreadyExists,
				format!("Scary to overwrite symlink ({real})"),
			));
		}

		let real = path.canonicalize()?;
		if !real.starts_with(target_root) {
			return Err(IoError::new(
				IoErrorKind::AlreadyExists,
				format!("Target points out from target directory: {}", real.display()),
			));
		}

		Ok(())
	};


	let copy_method = |source: &Path, target: &Path, to_inside| -> Result<OpRes, FsExtraError> {
		let into = target_root.join(target);
		let copied = if source.is_dir() {
			ensure_dir_exists(&into, target_root)?;
			ensure_out_of_root(&into)?;
			let options = fs_extra::dir::CopyOptions { copy_inside: to_inside,
			                                           ..def_options };
			fs_extra::dir::copy(source, into, &options).map(OpRes::Write)?
		} else if to_inside {
			ensure_dir_exists(&into, target_root)?;
			ensure_out_of_root(&into)?;
			let filename = source.file_name().ok_or_else(|| {
				                                  IoError::new(
				                                               IoErrorKind::InvalidFilename,
				                                               format!("Filename not found for {}", into.display()),
				)
			                                  })?;
			let into = into.join(filename);
			ensure_out_of_root(&into)?;
			std::fs::copy(source, into).map(OpRes::Write)?
		} else {
			let into_parent = parent_of(&into)?;
			ensure_dir_exists(into_parent, target_root)?;
			ensure_out_of_root(into_parent)?;

			if !into.try_exists()? || overwrite {
				std::fs::copy(source, into).map(OpRes::Write)?
			} else {
				OpRes::Skip
			}
		};
		info!("  {copied:?} copy: {} <- {}", target.display(), source.display());
		Ok(copied)
	};

	let link_method = |source: &Path, target: &Path, to_inside| -> Result<OpRes, FsExtraError> {
		let into = target_root.join(target);
		let linked = if to_inside {
			             ensure_dir_exists(&into, target_root)?;
			             let filename =
				             source.file_name().ok_or_else(|| {
					                                let msg = format!("Filename not found for {}", into.display());
					                                IoError::new(IoErrorKind::InvalidFilename, msg)
				                                })?;
			             let into = into.join(filename);
			             soft_link_checked(source, into, overwrite, target_root)
		             } else {
			             let into_parent = parent_of(&into)?;
			             ensure_dir_exists(into_parent, target_root)?;
			             soft_link_checked(source, &into, overwrite, target_root)
		             }.map(|was| if was { OpRes::Link } else { OpRes::Skip })?;
		info!("  {linked:?} link: {} <- {}", target.display(), source.display());
		Ok(linked)
	};

	let method: &dyn Fn(&Path, &Path, bool) -> Result<OpRes, FsExtraError> = match build_method {
		AssetsBuildMethod::Copy => &copy_method,
		AssetsBuildMethod::Link => &link_method,
	};

	let (mut plan, crate_root) = plan.into_parts();
	let mut results = HashMap::with_capacity(plan.len());
	for entry in plan.drain(..) {
		let current: Vec<_> = match &entry {
			Mapping::AsIs(inc, ..) => {
				let source = abs_if_existing_any(inc.source(), crate_root);
				vec![method(&source, &inc.target(), false)]
			},
			Mapping::Into(inc, ..) => {
				let source = abs_if_existing_any(inc.source(), crate_root);
				vec![method(&source, &inc.target(), true)]
			},
			Mapping::ManyInto { sources, target, .. } => {
				sources.iter()
				       .map(|inc| (abs_if_existing_any(inc.source(), crate_root), target.join(inc.target())))
				       .map(|(ref source, ref target)| method(source, target, false))
				       .collect()
			},
		};

		results.insert(entry, current);
	}

	Ok(BuildReport { results })
}


#[derive(Debug)]
pub struct BuildReport<'left, 'right>
	where Self: 'left + 'right {
	pub results: HashMap<Mapping<'left, 'right>, Vec<Result<OpRes, FsExtraError>>>,
}

impl BuildReport<'_, '_> {
	pub fn has_errors(&self) -> bool {
		self.results
		    .iter()
		    .flat_map(|(_, results)| results.iter())
		    .any(|result| result.is_err())
	}
}


#[derive(Debug)]
pub enum OpRes {
	Write(u64),
	Link,
	Skip,
}


impl AssetsOptions {
	fn link_behavior(&self) -> LinkBehavior {
		if self.follow_symlinks {
			LinkBehavior::ReadTarget
		} else {
			LinkBehavior::ReadFile
		}
	}
}


fn log_err<Err: std::fmt::Display>(err: Err) -> Err {
	error!("[package.metadata.playdate.assets]: {err}");
	err
}


#[derive(Debug)]
pub enum Error {
	Io(std::io::Error),
	Wax(wax::BuildError),
	Walk(WalkError),
	Error(String),
}


impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self { Self::Io(err) }
}
impl From<wax::BuildError> for Error {
	fn from(err: wax::BuildError) -> Self { Self::Wax(err) }
}
impl From<WalkError> for Error {
	fn from(err: WalkError) -> Self { Self::Walk(err) }
}
impl From<String> for Error {
	fn from(value: String) -> Self { Self::Error(value) }
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Io(err) => err.fmt(f),
			Error::Wax(err) => err.fmt(f),
			Error::Walk(err) => err.fmt(f),
			Error::Error(err) => err.fmt(f),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Io(err) => Some(err),
			Error::Wax(err) => Some(err),
			Error::Walk(err) => Some(err),
			Error::Error(_) => None,
		}
	}
}
