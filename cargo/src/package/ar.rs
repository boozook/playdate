#![allow(dead_code)]

use std::ffi::OsStr;
use std::fs::DirEntry;
use std::io::prelude::*;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

use zip::result::ZipResult;
use zip::{ZipWriter, CompressionMethod};
use zip::write::{FileOptionExtension, FileOptions};
use cargo::CargoResult;

use crate::utils::path::AsRelativeTo;


fn ar_path(source: &Path) -> PathBuf { source.with_extension("pdx.zip") }


pub fn build(source: Cow<'_, Path>) -> CargoResult<Cow<'_, Path>> {
	assert!(source.exists(), "source does not exist");
	assert!(source.is_dir(), "source is not a directory");

	let dst = ar_path(&source);
	log::debug!("ar path: {}", dst.display());
	let file = std::fs::File::create(&dst)?;

	let mut ar = ZipWriter::new(file);
	let options = FileOptions::<()>::default().compression_method(CompressionMethod::Deflated)
	                                          .compression_level(Some(9));
	ar.set_comment(INFO_FULL);

	fn zip_entry<T: Write + Seek, Opts: FileOptionExtension + Copy>(zip: &mut ZipWriter<T>,
	                                                                entry: DirEntry,
	                                                                root: &Path,
	                                                                options: FileOptions<Opts>)
	                                                                -> ZipResult<()> {
		let path = entry.path();
		let relative = path.as_relative_to(&root.parent().expect("parent directory"));

		if path.is_dir() {
			for entry in std::fs::read_dir(&path)? {
				log::debug!("  +dir: {}", relative.display());
				zip_entry(zip, entry?, root, options)?;
			}
		} else {
			// sanitize path:
			// TODO: test on windows
			let inner_path = relative.components()
			                         .map(|c| c.as_os_str())
			                         .collect::<Vec<_>>()
			                         .join(OsStr::new("/"));
			log::debug!("  +file: {}", inner_path.to_string_lossy());
			zip.start_file(inner_path.to_string_lossy(), options)?;
			log::debug!("  +file: {inner_path:?}");
			let mut file = std::fs::File::open(path)?;
			let mut buffer = Vec::new();
			file.read_to_end(&mut buffer)?;
			zip.write_all(&buffer)?;
		}

		Ok(())
	}

	for entry in std::fs::read_dir(&source)? {
		zip_entry(&mut ar, entry?, &source, options)?;
	}

	ar.finish()?.flush()?;
	Ok(dst.into())
}


pub fn add_info_meta<P: AsRef<Path>>(source: P) -> CargoResult<()> {
	let source = source.as_ref();
	assert!(source.exists(), "source does not exist");
	assert!(source.is_dir(), "source is not a directory");
	std::fs::write(source.join(INFO), INFO_FULL.as_bytes())?;
	Ok(())
}


const INFO: &str = "INFO";
const INFO_SHORT: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));
const INFO_FULL: &str = concat!("Built with Rust and ",
                                env!("CARGO_PKG_NAME"),
                                " v",
                                env!("CARGO_PKG_VERSION"),
                                "\n",
                                env!("CARGO_PKG_HOMEPAGE"),
                                "\n");
