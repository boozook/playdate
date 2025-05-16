# Playdate Tool

Cross-platform CLI-tool for interaction with [Playdate][playdate-website] device and simulator.


Can do for you:
- operate with multiple devices simultaneously
  - find connected devices, filter by mode, state, serial-number
  - send commands
  - read from devices
  - mount as drive (mass storage usb)
  - unmount
  - install pdx (playdate package)
  - run pdx (optionally with install pdx before run)
- operate with simulator
  - run pdx
  - read output from sim.


Tested on following platforms:
- MacOs
- Linux
- Windows


## Prerequisites

1. [Common prerequisites described in the wiki](https://github.com/boozook/playdate/wiki#prerequisites).
1. Linux only:
  - `libudev`, follow [instructions for udev crate][udev-crate-deps].
1. Windows only:
  - `powershell` (used as fallback)


[playdate-website]: https://play.date
[udev-crate-deps]: https://crates.io/crates/udev#Dependencies


## Installation

```bash
cargo install playdate-tool
```

Or with optional feature `eject`, windows only:
```bash
cargo install playdate-tool --feature=eject
```


## Usage

```bash
pdtool --help
```


- - -

This software is not sponsored or supported by Panic.
