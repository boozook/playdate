use std::io::{Result, Error, ErrorKind};
use std::path::{PathBuf, Path};


#[inline]
pub fn parent_of(path: &Path) -> Result<&Path> {
	path.parent()
	    .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("Parent for {path:?}")))
}

pub fn ensure_dir_exists<P: AsRef<Path>>(path: P, boundary: impl AsRef<Path>) -> std::io::Result<()> {
	trace!("ensure_dir_exists: path: {:?}", path.as_ref());
	if cfg!(windows) &&
	   !boundary.as_ref()
	            .try_exists()
	            .map_err(|err| error!("{err}"))
	            .ok()
	            .filter(|v| *v)
	            .unwrap_or_default()
	{
		trace!("\t boundary {:?} doesn't exist, creating it", boundary.as_ref());
		std::fs::create_dir_all(&boundary)?;
	}

	check_top_boundary_ok(&path.as_ref().join(PathBuf::from("...")), boundary.as_ref())?;
	if !path.as_ref().try_exists()? {
		trace!("path {:?} doesn't exist, creating it", path.as_ref());
		std::fs::create_dir_all(&path)?;
	}
	Ok(())
}


// TODO: Win-only: add `junction_point` as fallback method
// https://github.com/rust-lang/rust/issues/121709
/// Creates symlink.
pub fn soft_link_checked<Po: AsRef<Path>, Pl: AsRef<Path>>(origin: Po,
                                                           link: Pl,
                                                           overwrite: bool,
                                                           root: impl AsRef<Path>)
                                                           -> Result<bool> {
	check_top_boundary_ok(link.as_ref(), root.as_ref())?;

	let existing = link.as_ref().try_exists()?;
	let symlink = link.as_ref().is_symlink();

	if symlink && std::fs::read_link(&link)? == origin.as_ref() {
		return Ok(false);
	}

	if !existing && !symlink {
		soft::link(origin, link).map(|_| true)
	} else
	// FIXME: this is probably not the best way to do this.
	if (existing && overwrite) || symlink {
		soft::remove(&link).or_else(|_| {
			                   if link.as_ref().is_dir() {
				                   std::fs::remove_dir_all(&link)
			                   } else {
				                   std::fs::remove_file(&link)
			                   }
		                   })?;
		soft::link(origin, link).map(|_| true)
	} else {
		// There's anything but not symlink, so we should not touch it.
		Err(Error::new(ErrorKind::AlreadyExists, link.as_ref().display().to_string()))
	}
}


/// Creates symlink.
pub fn hard_link_forced<Po: AsRef<Path>, Pl: AsRef<Path>>(origin: Po,
                                                          link: Pl,
                                                          overwrite: bool,
                                                          root: impl AsRef<Path>)
                                                          -> Result<bool> {
	check_top_boundary_ok(link.as_ref(), root.as_ref())?;

	let existing = link.as_ref().try_exists()?;
	let symlink = link.as_ref().is_symlink();

	if symlink && std::fs::read_link(&link)? == origin.as_ref() {
		return Ok(false);
	}

	if !existing && !symlink {
		soft::link(origin, link).map(|_| true)
	} else
	// FIXME: this is probably not the best way to do this.
	if (existing && overwrite) || symlink {
		soft::remove(&link).or_else(|_| std::fs::remove_file(&link))?;
		hard::link(origin, link).map(|_| true)
	} else {
		// There's anything but not symlink, so we should not touch it.
		Err(Error::new(ErrorKind::AlreadyExists, link.as_ref().display().to_string()))
	}
}


/// Follows symlinks.
/// Both given paths will be canonicalized.
/// - `boundary` should exist and be directory
/// - canonicalized `path` should start with canonicalized `boundary`
fn check_top_boundary<B: AsRef<Path>>(path: &Path, boundary: B) -> Result<bool> {
	log::debug!(
	            "Checking out of top boundary {:?} for {path:?}",
	            boundary.as_ref(),
	);
	let boundary = boundary.as_ref()
	                       .canonicalize()
	                       .unwrap_or_else(|_| boundary.as_ref().to_owned());

	{
		let exists = boundary.try_exists()
		                     .map_err(|err| error!("{err}"))
		                     .ok()
		                     .filter(|v| *v)
		                     .unwrap_or_default();
		trace!("\t boundary: {boundary:?}, existing: {exists}");
	}

	// Without last component, skip to parent:
	// We don't care where to pointing last component, but parents is important.
	let path = path.parent()
	               .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("Parent of {path:?}")))?;
	trace!("\t parent: {path:?}");
	let (existing, tail) = get_existing_ancestor(path)?;
	trace!("\t existing: {existing:?}, tail: {tail:?}");
	Ok(existing.starts_with(&boundary) || existing.join(tail).starts_with(&boundary))
}

fn check_top_boundary_ok<B: AsRef<Path>>(path: &Path, boundary: B) -> Result<()> {
	if !check_top_boundary(path, boundary.as_ref())? {
		let message = format!(
		                      "Path {path:?} is outside of top-bound directory: {:?}",
		                      boundary.as_ref()
		);
		return Err(Error::new(ErrorKind::ReadOnlyFilesystem, message));
	}
	Ok(())
}


/// Returns canonicalized existing part of path,
/// and non-existing tail of path.
fn get_existing_ancestor(path: &Path) -> Result<(PathBuf, &Path)> {
	use std::io::{Error, ErrorKind};

	let parent = path.ancestors()
	                 .find(|p| p.try_exists().ok().filter(|v| *v).is_some())
	                 .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("Existing parent of {path:?}")))?;

	let wrap_err_io = |err| Error::new(ErrorKind::Other, err);

	let tail = path.strip_prefix(parent).map_err(wrap_err_io)?;
	let mut parent = parent.canonicalize()?;

	for (i, comp) in tail.components().enumerate() {
		let next = parent.join(comp);
		if next.try_exists()? {
			// Note: this can fail if canonicalizing symlink pointing to nowhere.
			parent = next.canonicalize()?;
		} else {
			if i > 0 {
				let existing = tail.components().take(i).collect::<PathBuf>();
				let tail = tail.strip_prefix(existing).map_err(wrap_err_io)?;
				return Ok((parent, tail));
			}
			break;
		}
	}

	Ok((parent, tail))
}


mod soft {
	use symlink::{symlink_file, symlink_dir};
	use symlink::{remove_symlink_file, remove_symlink_dir};
	use symlink::{symlink_auto, remove_symlink_auto};
	use std::io::{Result, Error, ErrorKind};
	use std::path::Path;


	pub fn link<Po: AsRef<Path>, Pl: AsRef<Path>>(origin: Po, link: Pl) -> Result<()> {
		let origin = origin.as_ref().canonicalize()?;
		if !origin.try_exists()? {
			return Err(Error::new(ErrorKind::NotFound, origin.display().to_string()));
		}

		if origin.is_file() {
			symlink_file(origin, link.as_ref())
		} else if origin.is_dir() {
			symlink_dir(origin, link.as_ref())
		} else {
			// otherwise determine if it's a file/dir reading metadata:
			symlink_auto(origin, link.as_ref())
		}
	}

	pub fn remove<P: AsRef<Path>>(path: P) -> Result<()> {
		if path.as_ref().is_file() {
			remove_symlink_file(path)
		} else if path.as_ref().is_dir() {
			remove_symlink_dir(path)
		} else {
			// otherwise determine if it's a file/dir reading metadata:
			remove_symlink_auto(path)
		}
	}
}


mod hard {
	use std::fs::hard_link;
	use std::io::Error;
	use std::io::ErrorKind;
	use std::io::Result;
	use std::path::Path;


	pub fn link<Po: AsRef<Path>, Pl: AsRef<Path>>(origin: Po, link: Pl) -> Result<()> {
		let origin = origin.as_ref().canonicalize()?;
		if !origin.try_exists()? {
			return Err(Error::new(ErrorKind::NotFound, origin.display().to_string()));
		}

		hard_link(origin, link.as_ref()).map_err(Into::into)
	}
}


#[cfg(test)]
mod tests {
	use std::borrow::Cow;
	use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR as OS_SEP};
	use super::check_top_boundary;


	const CREATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
	fn crate_root() -> PathBuf { PathBuf::from(CREATE_ROOT) }


	#[test]
	fn check_top_boundary_abs_root() {
		let paths = [
		             [CREATE_ROOT].concat(),
		             [CREATE_ROOT, OS_SEP].concat(),
		             [CREATE_ROOT, OS_SEP, OS_SEP].concat(),
		             [CREATE_ROOT, OS_SEP, "/"].concat(),
		             [CREATE_ROOT, OS_SEP, "//"].concat(),
		             [CREATE_ROOT, "/"].concat(),
		             [CREATE_ROOT, "//"].concat(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        !check_top_boundary(Path::new(&path), root).unwrap(),
				        "{path:?} is inside of {root:?}"
				);
			}
		}
	}


	#[test]
	fn check_top_boundary_abs_local() {
		let paths = [
		             [CREATE_ROOT, OS_SEP, "Cargo.toml"].concat(),
		             [CREATE_ROOT, OS_SEP, "str"].concat(),
		             [CREATE_ROOT, OS_SEP, "str", OS_SEP].concat(),
		             [CREATE_ROOT, OS_SEP, "str", "/"].concat(),
		             [CREATE_ROOT, OS_SEP, "str", "//"].concat(),
		             [CREATE_ROOT, OS_SEP, OS_SEP, "str", "//"].concat(),
		             [CREATE_ROOT, "/", "str"].concat(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        check_top_boundary(Path::new(&path), root).unwrap(),
				        "{path:?} is outside of {root:?}"
				);
			}
		}
	}

	#[test]
	fn check_top_boundary_abs_local_pb() {
		let paths = [
		             crate_root().join("Cargo.toml"),
		             crate_root().join("str"),
		             crate_root().join(["str", OS_SEP].concat()),
		             crate_root().join("str/"),
		             crate_root().join("str//"),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        check_top_boundary(&path, root).unwrap(),
				        "{path:?} is outside of {root:?}"
				);
			}
		}
	}

	#[test]
	fn check_top_boundary_abs_out() {
		let root = crate_root();
		let path = root.parent().unwrap();
		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		println!("\tChecking: {path:?}");
		for root in roots.iter() {
			assert!(
			        !check_top_boundary(path, root).unwrap(),
			        "{path:?} is inside of {root:?}"
			);
		}
	}

	#[test]
	fn check_top_boundary_abs_local_dot() {
		let paths = [
		             [CREATE_ROOT, OS_SEP, ".", OS_SEP, "Cargo.toml"].concat(),
		             [CREATE_ROOT, OS_SEP, ".", OS_SEP, "src"].concat(),
		             [CREATE_ROOT, OS_SEP, ".", OS_SEP, "unexisting"].concat(),
		             crate_root().join(".")
		                         .join("Cargo.toml")
		                         .to_string_lossy()
		                         .into_owned(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        check_top_boundary(Path::new(&path), root).unwrap(),
				        "{path:?} is outside of {root:?}"
				);
			}
		}
	}


	#[test]
	fn check_top_boundary_abs_out_dots() {
		let paths = [
		             [CREATE_ROOT, OS_SEP, "..", OS_SEP, "utils"].concat(),
		             [CREATE_ROOT, OS_SEP, "..", OS_SEP, "unexisting"].concat(),
		             crate_root().join("..")
		                         .join("utils")
		                         .to_string_lossy()
		                         .into_owned(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        !check_top_boundary(Path::new(&path), root).unwrap(),
				        "{path:?} is inside of {root:?}"
				);
			}
		}
	}

	#[test]
	fn check_top_boundary_rel_out_dots() {
		let paths = [
		             ["..", OS_SEP, "utils"].concat(),
		             ["..", OS_SEP, "unexisting"].concat(),
		             PathBuf::from("..").join("utils").to_string_lossy().into_owned(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        !check_top_boundary(Path::new(&path), root).unwrap(),
				        "{path:?} is inside of {root:?}"
				);
			}
		}
	}

	#[test]
	fn check_top_boundary_rel_local_dot() {
		let paths = [
		             [".", OS_SEP, "Cargo.toml"].concat(),
		             [".", OS_SEP, "src"].concat(),
		             [".", OS_SEP, "unexisting"].concat(),
		             PathBuf::from(".").join("Cargo.toml")
		                               .to_string_lossy()
		                               .into_owned(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(
				        check_top_boundary(Path::new(&path), root).unwrap(),
				        "{path:?} is outside of {root:?}"
				);
			}
		}
	}

	#[test]
	fn check_top_boundary_rel_local() {
		let paths = [
		             "Cargo.toml".to_owned(),
		             "src".to_owned(),
		             "unexisting".to_owned(),
		             PathBuf::from("Cargo.toml").to_string_lossy().into_owned(),
		];

		let roots = [
		             Cow::from(Path::new(CREATE_ROOT)),
		             crate_root().into(),
		             crate_root().canonicalize().unwrap().into(),
		];

		for path in paths {
			println!("\tChecking: {path:?}");
			for root in roots.iter() {
				assert!(check_top_boundary(Path::new(&path), root).is_err());
			}
		}
	}
}
