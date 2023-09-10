# Playdate Tool

CLI-tool and lib for interaction with Playdate device and sim.


### Status

This is earlier version, that means "alpha" or "MVP".
API can be changed in future versions.
Global __refactoring is planned__ with main reason of properly work with usb on all platforms.

Currently tested and works good on following platforms:
- Unix (x86-64 and aarch64)
  - macos 👍
  - linux 👍
- Windows (x86-64 and aarch64)
  -  ⚠️ known issues with hardware lookup, work in progress.



## Prerequisites

To build playdate-tool you're need:
1. Rust __nightly__ toolchain
2. Probably `libusb` and `pkg-config` or `vcpkg`, follow [instructions for rusb crate][rusb].

To use playdate-tool you're need:
1. [Playdate SDK][sdk]
   - Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root
2. This tool.


[sdk]: https://play.date/dev/#cardSDK
[doc-prerequisites]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites
[rusb]: https://crates.io/crates/rusb



## Installation

```bash
cargo install playdate-tool --features=cli
```





- - -

This software is not sponsored or supported by Panic.
