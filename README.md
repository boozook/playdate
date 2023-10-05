# 🦀 Playdate API and build tools in Rust

This project allows you to create games for the [Playdate handheld gaming system][playdate-website] in Rust lang.

[playdate-website]: https://play.date/


* [Modular build system][support-dir]
  - build-support libraries
  - pre-configured bindgen with extra codegen
  - [cargo-playdate][cargo-dir] - one-button solution to build, package and run programs
* [Modular low- & high- level API][api-dir]
  - with [examples][ctrl-examples-dir]
* __All the parts of API are accumulated in [One Crate][playdate-crate]__ ([git][playdate-crate-git])
* UI components
  - [crank-indicator][crank-indicator-gh] (port from [lua version][crank-indicator-lua]), requires SDK 2.1

Welcome to [discussions][] and [issues][] for any questions and suggestions.
Take a look at [videos](#demo) or [do something great](#usage).


[crank-indicator-gh]: https://github.com/boozook/playdate/tree/main/components/crank-indicator
[crank-indicator-lua]: https://sdk.play.date/Inside%20Playdate.html#C-ui.crankIndicator


## Prerequisites

Follow the instructions for:
1. [SDK](https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites)
1. [cargo-playdate](https://github.com/boozook/playdate/blob/main/cargo/README.md#prerequisites)
1. [playdate-sys](https://github.com/boozook/playdate/tree/main/api/sys#prerequisites)


## Compatibility

* __Minimal__ supported versions of Playdate SDK is `2.0.0` but with earlier versions all should work probably.

* __Latest__ version of SDK is `2.0.3`, all tested and works correctly.



## Usage

Run example:
```bash
cargo run -p=cargo-playdate -- run -p=playdate-controls --example=buttons
```

Install `cargo-playdate` and build & run another example:
```bash
cargo install cargo-playdate
cargo playdate run -p=playdate-sound --example=sp-simple
cargo playdate run -p=playdate-sound --example=sp-simple --device
```

### Demo

Early development demonstrations of the building process: [bin][video-bin] and [sim][video-simulator] screencasts originally posted on [dev-forum][video-post].

[video-post]: https://devforum.play.date/t/sdk-2-0-b2-pdc-produces-pdx-with-broken-binary/11345/37
[video-bin]: https://www.youtube.com/watch?v=hfFspYbnF5k
[video-simulator]: https://www.youtube.com/watch?v=w-pZrn8qex4


## Modularity

Thanks to the modular structure of the system, you can use all or only the parts of the system you need.

### Create a Game

1. Add [playdate crate][playdate-crate] as dependencies to your project
  - Or/and [API-components][api-dir]
2. Install [cargo-playdate][] to build your project

### Create an API-extension

1. Add [playdate-sys][] to dependencies
1. Write neat code
1. Build & test using cargo, [cargo-playdate][] or anything else.

Here is [example][color-dir] of simple API-extension.

Please follow [the instructions of playdate-sys](https://github.com/boozook/playdate/tree/main/api/sys#extension-development).

### Create your bindings

1. Use [playdate-bindgen][] in your build-script

### Create your build-system

1. Use [build-support crates][support-dir]

There is all needed to find SDK and arm-gnu toolchain on user's system, build flags, configurations, formats including pdxinfo, etc.



[playdate-crate]: https://crates.io/crates/playdate
[playdate-sys]: https://crates.io/crates/playdate-sys
[cargo-playdate]: https://crates.io/crates/cargo-playdate
[playdate-bindgen]: https://crates.io/crates/playdate-bindgen

[playdate-crate-git]: https://github.com/boozook/playdate/blob/main/api/playdate
[support-dir]: https://github.com/boozook/playdate/tree/main/support
[cargo-dir]: https://github.com/boozook/playdate/tree/main/cargo
[api-dir]: https://github.com/boozook/playdate/tree/main/api
[ctrl-examples-dir]: https://github.com/boozook/playdate/tree/main/api/ctrl/examples
[color-dir]: https://github.com/boozook/playdate/tree/main/api/color

[issues]: https://github.com/boozook/playdate/issues
[discussions]: https://github.com/boozook/playdate/discussions


- - -

This software is not sponsored or supported by Panic.
