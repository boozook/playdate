# Build System for Playdate applications

Cargo-playdate is a cross-platform plugin for cargo that can build programs for [Playdate handheld gaming system](https://play.date) written in Rust. It also works as standalone tool.

It can build programs written in Rust, manage assets, build package for Playdate and run on sim or device.
Usually it builds static or dynamic libraries for sim and hardware,
but also it can build executable binaries for hardware and this method produces highly optimized output with dramatically minimized size (thanks to DCE & LTO)\*.

\* For executable binaries use `--no-gcc` argument._



## Prerequisites

To build `cargo-playdate` you're need:
1. Rust __nightly__ toolchain

To build programs using `cargo-playdate` you need:
1. Rust __nightly__ toolchain
1. [Playdate SDK][sdk]
   - Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root
1. Follow the [official documentation][doc-prerequisites]
   - Ensure that `arm-none-eabi-gcc` or `gcc-arm-none-eabi` in your `PATH`

 <!-- TODO: Make gcc optional -->

To run on sim or dev with `cargo-playdate`:
1. Linux only:
  - `libudev`, follow [instructions for udev crate][udev-crate-deps].
1. Windows only:
  - `powershell` (used as fallback)

[sdk]: https://play.date/dev/#cardSDK
[doc-prerequisites]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites
[rusb]: https://crates.io/crates/rusb


## Installation

```bash
cargo install cargo-playdate
# or
cargo install --git="https://github.com/boozook/playdate.git" --bin=cargo-playdate
```


## Hello World

Generate new project using `new` or `init` command.

```bash
mkdir hello-world && cd $_
cargo playdate init --lib --full-metadata --deps="playdate"
cargo playdate run
```

> Note, there is more options for this commands, e.g. `--deps="sys:git, controls:git"`.
> Run `cargo playdate new --help` for more about it.

New package will be created.

Take a look at the package manifest file (Cargo.toml).

There is extra metadata for your playdate package.

For more information about metadata read [documentation][metadata-readme].

[metadata-readme]: https://github.com/boozook/playdate/blob/main/support/build/README.md#assets


<!--
## Crank(start) compatibility

TODO: Impl and describe compatibility with crank(start).
-->


## Configuration

There is no configuration other then inherited by cargo and some special environment variables.

- `CARGO_PLAYDATE_LOG` working same way as `CARGO_LOG` or default `RUST_LOG`. Also `CARGO_PLAYDATE_LOG_STYLE`
- `PLAYDATE_SDK_PATH` path to the SDK root
- `ARM_GCC_PATH` path to the `arm-none-eabi-gcc` executable.

Execute `cargo playdate -h` for more details, or with `--help` for further more.



### Limitations

1. Global crate-level attributes like `crate_type` and `crate_name` doesn't supported, e.g:
```rust
#![crate_name = "Game"]
#![crate_type = "lib"]
```

2. Cargo-targets such as `bin` and `example` should be in the cargo manifest. Autodetect isn't yet tested and may not work. Example:
```toml
[[example]]
name = "demo"
crate-type = ["dylib", "staticlib"]
path = "examples/demo.rs"
```


3. Assets especially for `example` cargo-targets inherits from package assets. Currently there's no way to set assets for single cargo-target, but only for entire package __or for dev-targets__ - [there is `dev-assets` extra table][dev-assets-doc] inherited by main.


[dev-assets-doc]: https://github.com/boozook/playdate/tree/main/support/build#dev-assets


## Troubleshooting

* On windows in some cases hardware cannot be ejected because of no permissions. Try to give rights and/or build `cargo-playdate` with feature `eject`.

* Welcome to [discussions](https://github.com/boozook/playdate/discussions) and [issues](https://github.com/boozook/playdate/issues).

- - -

This software is not sponsored or supported by Panic.
