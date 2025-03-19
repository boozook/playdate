# Scoreboards API for PlayDate

High-level scoreboards API built on-top of [playdate-sys][].

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


## Usage

```rust
use playdate_scoreboards::*;
use playdate_sys::println;

let scoreboards = Scoreboards::Cached();

scoreboards.get_scoreboards(|boards| {
	           println!("{boards:?}");
           });
```


[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

This software is not sponsored or supported by Panic.
