# File-system API for PlayDate

High-level interface to file-system on-top of `playdate-sys` crate.

Api mimics [std::fs](https://doc.rust-lang.org/std/fs/index.html).

Inside are two versions of the same API: scoped and non-scoped.
* non-scoped:
	- is simpler, but in case of an error it allocates memory and copies error from the c-side and returns the result
	- has module-level functions
* scoped:
	- requires mutability for each call, but as benefit, every operation returns result with borrowed error that cannot escape from the scope.

```rust
extern crate playdate_fs as fs;
use fs::prelude::*;

const FILE: &Path = c"pdxinfo";


fn simple() -> Result<FileStat, FsError> {
	let fs = fs::Fs::default();
	let meta = fs.metadata(FILE)?;
	Ok(meta)
}

fn scoped() -> Result<FileStat, FsError> {
	let fs = fs::Fs::default();
	let meta = fs.scoped(|fs| {
							// do some op(s), print borrowed error, make error owned via `into_owned`:
							fs.metadata(FILE)
							.inspect_err(|err| println!("{err:?}"))
							.map_err(|err| err.into_owned())
						})?;
	Ok(meta)
}

fn scoped_direct() -> Result<FileStat, FsError> {
	// create mutable scoped endpoint:
	let mut fs = fs::scoped::Fs::default();
	let meta = fs.metadata(FILE).inspect_err(|err| println!("{err:?}"))?;
	// borrowed error implicitly converting into owned `FsError` via `try`.
	Ok(meta)
}
```


- - -

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


This software is not sponsored or supported by Panic.
