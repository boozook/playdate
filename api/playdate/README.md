# PlayDate API

Mostly high-level rusty API for the [Playdate handheld gaming system][playdate-website].

Usage with [cargo-playdate][cargo-playdate-crate] is strongly recommended.


[cargo-playdate-crate]: https://crates.io/crates/cargo-playdate
[playdate-website]: https://play.date/


## What's inside

### Covered parts of API:

- [controls](https://crates.io/crates/playdate-controls)
- [display](https://crates.io/crates/playdate-display)
- [file system](https://crates.io/crates/playdate-fs)
- [graphics](https://crates.io/crates/playdate-graphics) (with [color](https://crates.io/crates/playdate-color))
- [sound](https://crates.io/crates/playdate-sound)
- [sprite](https://crates.io/crates/playdate-sprite)
- [system](https://crates.io/crates/playdate-system) (with [menu](https://crates.io/crates/playdate-menu))
- [sys](https://crates.io/crates/playdate-sys) - base, low-level cffi bindings

Plus some extensions to make it all more rust-ish.


### Not yet covered parts:

- scoreboards
- json
- lua

Also __there is no default entry-point__ (read as event-handler) for your application.
Not yet, I'm working on it.


## How to start

Currently there is no any beautiful HL examples ready yet.
It will be in v0.2.0 or little bit earlier as well as default entry-point that mentioned above.

### Prerequisites

Follow the instructions for:
1. [SDK](https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites)
1. [cargo-playdate](https://github.com/boozook/playdate/blob/main/cargo/README.md#prerequisites)
1. [playdate-sys](https://github.com/boozook/playdate/tree/main/api/sys#prerequisites) (prerequisites only)


### Hello World

> Note, this is incomplete and not-so production-ready crate.
>
> As minimum currently you'll need to implement missed entry-point for your program by yourself.
> But you can find some examples with ugly & primitive entry-point impl,
> so you'll need to add something like [this code][ugly-entry-point] to your program.
>
> Here is [list of missed parts that not ready yet](#not-yet-covered-parts).

[ugly-entry-point]: https://github.com/boozook/playdate/blob/main/api/sys/examples/hello-world.rs#L97-L138

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


❤️‍🔥


[sprite-examples]: https://github.com/boozook/playdate/tree/main/api/sprite/examples
[cargo-playdate]: https://crates.io/crates/cargo-playdate
[cargo-playdate-hw]: https://github.com/boozook/playdate/tree/main/cargo#hello-world
[playdate metadata format]: https://github.com/boozook/playdate/tree/main/support/build#metadata

- - -

This software is not sponsored or supported by Panic.
