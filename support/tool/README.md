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

To build playdate-tool you're need:
1. Rust __nightly__ toolchain
2. Linux only:
  - `libudev`, follow [instructions for udev crate][udev-crate-deps].
3. [Playdate SDK][sdk] with simulator
   - Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root. _This is optional, but good move to help the tool to find SDK, and also useful if you have more then one version of SDK._


[playdate-website]: https://play.date
[udev-crate-deps]: https://crates.io/crates/udev#Dependencies
[sdk]: https://play.date/dev/#cardSDK


## Installation

```bash
cargo install playdate-tool
```


## Usage

```bash
pdtool --help
```

<details><summary>Help output example</summary>


```text
Usage: pdtool [OPTIONS] <COMMAND>

Commands:
  list     Print list of connected active Playdate devices
  mount    Mount a Playdate device if specified, otherwise mount all Playdates as possible
  unmount  Unmount a Playdate device if specified, otherwise unmount all mounted Playdates
  install  Install given package to device if specified, otherwise use all devices as possible
  run      Install and run given package on the specified device or simulator
  read     Connect to device and proxy output to stdout
  send     Send command to specified device
  help     Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>  Standard output format [default: human] [possible values: human, json]
  -h, --help             Print help
  -V, --version          Print version
```

</details>


- - -

This software is not sponsored or supported by Panic.
