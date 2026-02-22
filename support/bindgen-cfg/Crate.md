# Configuration for Playdate Bindings Generator

Lightweight util for build-scripts to configure and execute [playdate-bindgen][bindgen-crate] used to generate bindings to Playdate with extras.


[bindgen-crate]: https://crates.io/crates/playdate-bindgen


# Usage

Cargo.toml:
```toml
[build-dependencies.bindgen]
package = "playdate-bindgen-cfg"
version = "*"
```

Add this to `build-dependencies` and add to your build-script something like this:

```rust
let mut cfg = bindgen::Cfg::default();
cfg.output = Some("some/output/path.rs");

let pdbindgen_found = bindgen::Runner::find_tool(&cfg);    // find existing pdbindgen (path, version)
let sdk_version = bindgen::Runner::find_sdk_version(&cfg); // execute pdbindgen to find SDK properly
let result = bindgen::Runner::gen_cmd(&cfg);               // execute pdbindgen to generate bindings
```

For complex examples see build-script in the [playdate-sys crate][playdate-sys-crate].


[playdate-sys-crate]: https://crates.io/crates/playdate-sys


- - -

This software is not sponsored or supported by Panic.
