# Build System for Playdate applications

Cargo-playdate is a plugin for cargo that can build programs for [Playdate handheld gaming system](https://play.date) written in Rust.

It can build programs written in Rust, manage assets, build package for Playdate and run in sim or install and run on device.
Usually it builds static or dynamic libraries for sim and hardware,
but also it can build executable binaries for hardware and this method produces highly optimized output with dramatically minimized size (thanks to DCE & LTO).
_\* But for binaries you're need also patched pdc from [dev-forum][]._


[dev-forum]: https://devforum.play.date/t/sdk-2-0-b2-pdc-produces-pdx-with-broken-binary/11345/28


### Status

Currently tested and works good on following platforms:
- Unix (x86-64 and aarch64)
  - macos 👍
  - linux 👍
- Windows (x86-64 and aarch64)
  - build 👍
  - package 👍
  - install & run ⚠️ - issues, work in progress, see [troubleshooting](#troubleshooting).


## Prerequisites

To build cargo-playdate you're need:
1. Rust __nightly__ toolchain
2. Probably `libusb` and `pkg-config` or `vcpkg`, follow [instructions for rusb crate][rusb].

To build programs using cargo-playdate you need:
1. Rust __nightly__ toolchain
2. [Playdate SDK][sdk]
   - Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root
3. Follow the [official documentation][doc-prerequisites]
   - Ensure that `arm-none-eabi-gcc` or `gcc-arm-none-eabi` in your `PATH`
4. This tool.

[sdk]: https://play.date/dev/#cardSDK
[doc-prerequisites]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites
[rusb]: https://crates.io/crates/rusb


## Installation

```bash
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


3. Assets especially for `example` cargo-targets inherits from package assets. Currently there's no way to set assets for single cargo-target, but only for entire package. WiP, there will be "dev-assets" extra table inherited by main.



## Troubleshooting

* Is some cases (see [status](#status)) hardware cannot be detected. Try to build cargo-playdate with or without feature `usb`.

* Welcome to [discussions](https://github.com/boozook/playdate/discussions) and [issues](https://github.com/boozook/playdate/issues).

- - -

This software is not sponsored or supported by Panic.
