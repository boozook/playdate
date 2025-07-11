# Playdate Bindings Generator

Util to generate bindings to Playdate with additional code-gen features like in-code documentation directly by official c-reference.


## Requirements

1. [Common prerequisites described in the wiki](https://github.com/boozook/playdate/wiki#prerequisites).
1. Requirements inherited by [bindgen][bindgen-crate], follow [official documentation][bindgen-requirements].

[bindgen-requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html#clang



## Configuration

Inherited configuration by bindgen: follow [bindgen official documentation][bindgen-env-var].

Env var `PLAYDATE_SDK_PATH` as described in [playdate official documentation][sdk-env-var].

Optional env var `ARM_GCC_PATH` to help to find `arm-none-eabi-gcc` (or `gcc-arm-none-eabi`) with entire toolchain. Useful only for troubleshooting.



[bindgen-crate]: https://crates.io/crates/bindgen
[bindgen-env-var]: https://github.com/rust-lang/rust-bindgen/tree/main#environment-variables
[sdk-env-var]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_set_playdate_sdk_path_environment_variable



# Usage

Cargo.toml:
```toml
[build-dependencies.bindgen]
package = "playdate-bindgen"
version = "*"
```

Add this to `build-dependencies` and add to your build-script something like this:

```rust
let cfg = bindgen::cfg::Config::default();
let generator = bindgen::Generator::new(cfg).expect("Couldn't create bindings generator.");
let out_path = bindgen::env_var("OUT_DIR").map(PathBuf::from)
	                                       .map(|p| p.join(&generator.filename.to_string()))
	                                       .unwrap();
let bindings = generator.generate().expect("Couldn't generate bindings.");
bindings.write_to_file(&out_path).expect("Couldn't write bindings.");
```

For complex examples see build-script in the [playdate-sys crate][playdate-sys-crate].



[playdate-sys-crate]: https://crates.io/crates/playdate-sys





- - -

This software is not sponsored or supported by Panic.
