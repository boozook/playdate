# [Playdate][playdate-website] device support library

Cross-platform interface for Playdate device, async & blocking.

Contains methods for:
- find connected devices, filter by mode, state, serial-number
- send commands
- read from devices
- mount as drive (mass storage usb)
- unmount
- install pdx (playdate package)
- run pdx (optionally with install before run)
- operate with multiple devices simultaneously


### Status

This crate in active development and API can be changed in future versions, with minor version increment.

Supported platforms:
- MacOs
- Linux
- Windows


## Prerequisites

1. Rust __nightly__ toolchain
2. Linux only:
  - `libudev`, follow [instructions for udev crate][udev-crate-deps].



[playdate-website]: https://play.date
[udev-crate-deps]: https://crates.io/crates/udev#Dependencies





- - -

This software is not sponsored or supported by Panic.
