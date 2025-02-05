//! Bundled prebuilt bindings.
use core::str;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use bindgen_cfg::DerivesMask;
use bindgen_cfg::Filename;
use bindgen_cfg::Target;

use crate::cargo::warn;


/// Path of the root of bundled bindings.
pub fn root() -> PathBuf {
	env::var("CARGO_MANIFEST_DIR").map(PathBuf::from)
	                              .map(|p| p.join("gen"))
	                              .expect("CARGO_MANIFEST_DIR")
}

/// Path of the file with `filename` in the root of bundled bindings.
pub fn path(filename: &Filename) -> PathBuf { root().join(&filename.to_string()) }


/// Returns highest version value of existing bindings for that version SDK.
/// `all` means all versions including betas.
pub fn highest_version(all: bool) -> String {
	let path = root().join("LATEST");

	match fs::read_to_string(path) {
		Ok(ver) => ver.trim().to_owned(),
		Err(err) => {
			let target = Target::from_env_target().inspect_err(|err| warn(err)).ok();
			let result = search_highest_version(all, target.as_ref()).inspect_err(|err| warn(err))
			                                                         .ok()
			                                                         .flatten()
			                                                         .map(|(v, _)| v);

			if let Some(highest) = result {
				highest.to_string_lossy().into_owned()
			} else {
				panic!("Bundled bindings not found: {err}");
			}
		},
	}
}


/// Simple per-comp cmp.
// XXX: this is sort of stupid absolutelly not optimal implementation.
fn cmp_versions(a: impl AsRef<[u8]>, b: impl AsRef<[u8]>) -> Ordering {
	let a = a.as_ref().split(|v| *v == b'.');
	let b = b.as_ref().split(|v| *v == b'.');

	let parse = |v: &[u8]| {
		let s = unsafe { str::from_raw_parts(v.as_ptr(), v.len()) };
		s.parse::<usize>()
	};

	for (a, b) in a.map(parse).zip(b.map(parse)) {
		if a.is_err() {
			return Ordering::Less;
		};
		if b.is_err() {
			return Ordering::Greater;
		};

		let a = unsafe { a.unwrap_unchecked() };
		let b = unsafe { b.unwrap_unchecked() };

		if a != b {
			return a.cmp(&b);
		}
	}
	Ordering::Equal
}


/// Returns `(version, random filename)`.
/// There "random filename" is because the file was chosen at kinda random and depends on ordering of dir-read.
/// - `all` means all versions including betas.
pub fn search_highest_version(all: bool, target: Option<&Target>) -> Result<Option<(OsString, OsString)>, Error> {
	let target = target.map(ToString::to_string);
	let get_version = Filename::get_sdk_version_from_filename_with_target;
	let not_beta: fn(&(OsString, OsString)) -> bool = if !all {
		|(ver, _)| !ver.as_encoded_bytes().contains(&b'-')
	} else {
		|_| true
	};

	let highest = |a: (OsString, _), b: (OsString, _)| -> (OsString, _) {
		match cmp_versions(a.0.as_encoded_bytes(), b.0.as_encoded_bytes()) {
			Ordering::Greater | Ordering::Equal => a,
			Ordering::Less => b,
		}
	};

	let existing = |(_, name): &(_, OsString)| root().join(name.as_os_str()).exists();

	let highest = fs::read_dir(root())?.filter_map(|res| res.ok())
	                                   .filter_map(|entry| {
		                                   let name = entry.file_name();
		                                   get_version(&name, target.as_deref()).map(Cow::into_owned)
		                                                                        .map(|ver| (ver, name))
	                                   })
	                                   .filter(not_beta)
	                                   .filter(existing)
	                                   .reduce(highest);
	Ok(highest)
}

#[allow(dead_code)]
/// Same as [`highest_version`] but with all filenames.
pub fn highest_version_with_filenames(all: bool,
                                      target: Option<&Target>)
                                      -> Result<Option<(OsString, Vec<OsString>)>, Error> {
	let target = target.map(ToString::to_string);
	let get_version = Filename::get_sdk_version_from_filename_with_target;
	let not_beta: fn(&(OsString, OsString)) -> bool = if !all {
		|(ver, _)| !ver.as_encoded_bytes().contains(&b'-')
	} else {
		|_| true
	};

	let highest = |a: (OsString, _), b: (OsString, _)| -> (OsString, _) {
		if a.0 == b.0 {
			a
		} else if a.0 > b.0 {
			a
		} else {
			b
		}
	};

	let existing = |(_, name): &(_, OsString)| root().join(name.as_os_str()).exists();

	let mut map = BTreeMap::<OsString, Vec<OsString>>::new();

	fs::read_dir(root())?.filter_map(|res| res.ok())
	                     .filter_map(|entry| {
		                     let name = entry.file_name();
		                     get_version(&name, target.as_deref()).map(Cow::into_owned)
		                                                          .map(|ver| (ver, name))
	                     })
	                     .filter(not_beta)
	                     .filter(existing)
	                     .for_each(|(ver, name)| {
		                     if let Some(vec) = map.get_mut(&ver) {
			                     vec.push(name);
		                     } else {
			                     map.insert(ver, vec![name]);
		                     }
	                     });

	let highest = map.into_iter().reduce(highest);
	Ok(highest)
}


type Error = Box<dyn std::error::Error>;

pub fn all_with_prefix(prefix: &str) -> Result<impl Iterator<Item = OsString> + use<'_>, Error> {
	let iter = fs::read_dir(root())?.filter_map(|res| res.ok())
	                                .filter_map(|entry| {
		                                let name = entry.file_name();
		                                name.as_encoded_bytes()
		                                    .starts_with(prefix.as_bytes())
		                                    .then_some(name)
	                                });
	Ok(iter)
}


pub fn nearest_applicable_parts(filename: &Filename) -> Result<Option<(OsString, DerivesMask)>, Error> {
	let prefix = filename.trim_suffix();
	let mask = &filename.mask;


	let applicable = all_with_prefix(&prefix)?;

	let start = prefix.as_bytes().len();

	let res = applicable.filter_map(|filename| {
		                    let suffix = {
			                    let bytes = filename.as_encoded_bytes();
			                    let end = bytes.len() - Filename::DOT_EXT.len();
			                    &bytes[start..end]
		                    };
		                    let existing = DerivesMask::from_ascii(suffix).inspect_err(|err| {
			                                                                  warn(format!("{filename:?} {err}"));
		                                                                  })
		                                                                  .ok()?;
		                    (&existing >= mask).then(|| (filename, existing.distance(&mask), existing))
	                    })
	                    .reduce(|left, right| if left.1 > right.1 { left } else { right })
	                    .map(|(name, _, mask)| (name, mask));

	Ok(res)
}


pub fn nearest_applicable(filename: &Filename) -> Result<Option<Filename>, Error> {
	let res = nearest_applicable_parts(filename);
	res.map(|opt| {
		   opt.map(|(name, mask)| {
			      let res = Filename { sdk: filename.sdk.to_owned(),
			                           mask,
			                           target: filename.target.to_owned() };
			      debug_assert_eq!(name.into_string().unwrap(), res.to_string());
			      res
		      })
	   })
}
