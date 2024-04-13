use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};


#[inline]
pub fn parent_of(path: &Path) -> Result<&Path> {
	path.parent()
	    .ok_or_else(|| IoError::new(IoErrorKind::NotFound, format!("Parent for {}", path.display())))
}

pub fn ensure_dir_exists<P: AsRef<Path>>(path: P, boundary: impl AsRef<Path>) -> std::io::Result<()> {
	check_top_boundary_ok(&path.as_ref().join(PathBuf::from("...")), boundary.as_ref())?;
	if !path.as_ref().try_exists()? {
		std::fs::create_dir_all(&path)?;
	}
	Ok(())
}


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
		Err(IoError::new(IoErrorKind::AlreadyExists, link.as_ref().display().to_string()))
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
		Err(IoError::new(IoErrorKind::AlreadyExists, link.as_ref().display().to_string()))
	}
}


/// Follows symlinks.
/// Both given paths will be canonicalized.
/// - `boundary` should existing and be directory
/// - canonicalized `path` should start with canonicalized `boundary`
fn check_top_boundary<B: AsRef<Path>>(path: &Path, boundary: B) -> Result<bool> {
	log::debug!(
	            "Checking out of top boundary {} for {}",
	            boundary.as_ref().display(),
	            path.display()
	);
	let boundary = boundary.as_ref()
	                       .canonicalize()
	                       .unwrap_or_else(|_| boundary.as_ref().to_owned());

	// Without last component, skip to parent:
	// We don't care where to pointing last component, but parents is important.
	let path = path.parent()
	               .ok_or_else(|| IoError::new(IoErrorKind::NotFound, format!("Parent of {}", path.display())))?;
	let (existing, tail) = get_existing_ancestor(path)?;
	Ok(existing.starts_with(&boundary) || existing.join(tail).starts_with(&boundary))
}

fn check_top_boundary_ok<B: AsRef<Path>>(path: &Path, boundary: B) -> Result<()> {
	if !check_top_boundary(path, boundary.as_ref())? {
		let message = format!(
		                      "Path '{}' is outside of top-bound directory: '{}'",
		                      path.display(),
		                      boundary.as_ref().display()
		);
		return Err(IoError::new(IoErrorKind::ReadOnlyFilesystem, message));
	}
	Ok(())
}


/// Returns canonicalized existing part of path,
/// and non-existing tail of path.
fn get_existing_ancestor(path: &Path) -> Result<(PathBuf, &Path)> {
	use std::io::{Error, ErrorKind};

	let parent = path.ancestors()
	                 .find(|p| p.try_exists().ok().filter(|v| *v).is_some())
	                 .ok_or_else(|| {
		                 Error::new(
		                            ErrorKind::NotFound,
		                            format!("Existing parent of {}", path.display()),
		)
	                 })?;

	let tail = path.strip_prefix(parent).unwrap(); // TODO: unwrap
	let mut parent = parent.canonicalize()?;

	for (i, comp) in tail.components().enumerate() {
		let next = parent.join(comp);
		if next.try_exists()? {
			// Note: this can fail if canonicalizing symlink pointing to nowhere.
			parent = next.canonicalize()?;
		} else {
			if i > 0 {
				let existing = tail.components().take(i).collect::<PathBuf>();
				let tail = tail.strip_prefix(existing).unwrap(); // TODO: unwrap
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
