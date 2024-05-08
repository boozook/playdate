# Build System for Playdate applications

Cargo-playdate is a cross-platform plugin for cargo that can build programs for [Playdate handheld gaming system](https://play.date) written in Rust. It also works as standalone tool.

It can build programs written in Rust, manage assets, build package for Playdate and run on sim or device.
Usually it builds static or dynamic libraries for sim and hardware,
but also it can build executable binaries for hardware and this method produces highly optimized output with dramatically minimized size (thanks to DCE & LTO)\*.

\* For executable binaries use `--no-gcc` argument._

### Platform specific pre-req install instructions

MacOS:

1. Install the [PlaydateSDK](https://play.date/dev/) (by default installs into `~/Developer/PlaydateSDK`)
1. Set PLAYDATE_SDK_PATH env var: `export PLAYDATE_SDK_PATH="$HOME/Developer/PlaydateSDK"`
1. Install cmake: `brew install cmake`
1. Install rust nightly: `rustup toolchain install nightly`
1. Arm toolchain is included with PlaydateSDK.

Ubuntu Linux:
1. Install the [PlaydateSDK](https://play.date/dev/) - remember where you extracted it.
1. Set PLAYDATE_SDK_PATH env var: `export PLAYDATE_SDK_PATH="/path/to/PlaydateSDK-2.x.x/"`
1. Install cmake: `sudo apt-get install cmake`
1. Install rust nightly: `rustup toolchain install nightly`
1. Install arm toolchain: `sudo apt-get install gcc-arm-none-eabi`
1. Install libudev: `sudo apt-get install libudev-dev`

Windows:
1. Install the [PlaydateSDK](https://play.date/dev/) (by default installs into `~/Documents/PlaydateSDK`)
1. Set PLAYDATE_SDK_PATH
    1. windows+r, run: `sysdm.cpl`
    2. Advanced Tab -> Environment Variables -> New
    3. Variable name: `PLAYDATE_SDK_PATH`
    4. Variable value: `C:\Users\username\Documents\PlaydateSDK`
1. Install CMake: [cmake downloads](https://cmake.org/download/)
1. Install rust nightly: `rustup toolchain install nightly`
1. Install arm toolchain: [arm gnu toolchain downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads). Filename will be like 'arm-gnu-toolchain-13.2.rel1-mingw-w64-i686-arm-none-eabi.exe'.
1. Add arm toolchain and cmake to PATH environment:
    1. windows+r, run: `sysdm.cpl`
    2. Advanced Tab -> Environment Variables.
    3. Find `Path` and click `Edit`
    4. Click `New` and add `C:\Program Files\CMake\bin`
    4. Click `New` and add `C:\Program Files (x86)\Arm GNU Toolchain arm-none-eabi\13.2 Rel1\bin`


 <!-- TODO: Make gcc optional -->

See also: [Inside Playdate with C: Prerequisites](https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites)

## Installation

```bash
cargo +nightly install cargo-playdate
# or
cargo +nightly install --git="https://github.com/boozook/playdate.git" --bin=cargo-playdate
```


## Hello World

Generate new project using `new` or `init` command.

```bash
mkdir -p ~/code/pd-hello/
cd ~/code/pd-hello/
cargo +nightly playdate init --lib --full-metadata --deps="playdate"
cargo +nightly playdate run
```

> Note, there are more options for this command, e.g. `--deps="sys:git, controls:git"`.
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
