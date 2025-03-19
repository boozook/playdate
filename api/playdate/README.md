# PlayDate API

Mostly high-level rusty API for the [Playdate handheld gaming system][playdate-website].

Usage with [cargo-playdate][cargo-playdate] is strongly recommended.


[playdate-website]: https://play.date/

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


## What's inside

### Covered parts of API:

- [controls](https://crates.io/crates/playdate-controls)
- [display](https://crates.io/crates/playdate-display)
- [file system](https://crates.io/crates/playdate-fs)
- [graphics](https://crates.io/crates/playdate-graphics) (with [color](https://crates.io/crates/playdate-color))
- [lua](https://crates.io/crates/playdate-lua)
- [scoreboards](https://crates.io/crates/playdate-scoreboards)
- [sound](https://crates.io/crates/playdate-sound)
- [sprite](https://crates.io/crates/playdate-sprite)
- [system](https://crates.io/crates/playdate-system) (with [menu](https://crates.io/crates/playdate-menu))
- [sys](https://crates.io/crates/playdate-sys) - base, low-level cffi bindings

Plus some extensions to make it all more rust-ish.


### Not yet covered parts:

- json


## How to start

Look at the [examples][gh-playdate-examples].


## Examples

[Here is available examples][gh-playdate-examples].
You car run it with following command:

```bash
# Simulator:
cargo playdate run -p=playdate --example=hello-world --features=entry-point
# Device:
cargo playdate run -p=playdate --example=video --features=entry-point --device
```

More information how to use [cargo-playdate][] in help: `cargo playdate --help`.


[gh-playdate-examples]: https://github.com/boozook/playdate/tree/main/api/gfx/examples


### Prerequisites

Follow the instructions for:
1. [SDK](https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites)
1. [cargo-playdate](https://github.com/boozook/playdate/blob/main/cargo/README.md#prerequisites)
1. [playdate-sys](https://github.com/boozook/playdate/tree/main/api/sys#prerequisites) (prerequisites only)


### Hello World

There is multiple ways to learn and start.

Following two is just a quick introduction.
In details it all will be explained soon in the wiki.

#### Short Way

1. Install [cargo-playdate][].
1. `cd where/your/project/will/be`
1. `cargo playdate init --lib --full-metadata --deps="playdate"` ([more about it][cargo-playdate-hw])
1. `cargo playdate run`

Done. Now take a look at long way below for details what's happen.

#### Alternative Way

Just run `cargo new <your options>` and add do following:
1. Add [playdate crate][playdate-crate] as dependency to your project
1. `#![no_std]` for library, or `#![no_std] #![no_main]` for executable binary
1. Look at existing example for api-parts [like that][sprite-examples]
1. Add minimally required metadata to build package:
	- `package.metadata.playdate.bundle-id = "com.yourcompany.game"`
	- Read about [playdate metadata format][] if needed
1. Install [cargo-playdate][] to build your project
1. Run `cargo playdate run`
1. Help this project somehow.



[sprite-examples]: https://github.com/boozook/playdate/tree/main/api/sprite/examples
[cargo-playdate]: https://crates.io/crates/cargo-playdate
[cargo-playdate-hw]: https://github.com/boozook/playdate/tree/main/cargo#hello-world
[playdate metadata format]: https://github.com/boozook/playdate/tree/main/support/build#metadata

- - -

Made with ❤️‍🔥 by [me](https://a.koz.world).

This software is not sponsored or supported by Panic.
