# Sound API for PlayDate

High-level sound API built on-top of [playdate-sys][].

Covered parts of the sound API:
- File Player
- Sample Player
  - Sample
- Sound Source
- Headphones and microphone (incomplete)

Not covered things:
- channel
- synth
- sequence
- effect
- lfo
- envelope
- callbacks

⚠️ Prior to the version `0.3` API is unstable and can be changed without deprecation period.


## Prerequisites

1. Rust __nightly__ toolchain (rustup is optional)
1. [Playdate SDK][sdk]
1. Follow the [official documentation][doc-prerequisites]
1. Follow the instructions for [playdate-sys][]

[sdk]: https://play.date/dev/#cardSDK
[doc-prerequisites]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites


## Usage

```rust
use playdate_sound::sample::*;
use playdate_sound::player::sp::*;
use playdate_sound::player::Repeat;

let player = Player::<api::Cache>::new()?;
let sample = Sample::new_from_file("game_main_theme.pda")?;

player.set_sample(&sample);
player.play(Repeat::LoopsEndlessly, 1.0);
```

See more in [examples][playdate-sound-examples].


[playdate-sys]: https://crates.io/crates/playdate-sys
[playdate-sound-examples]: https://github.com/boozook/playdate/tree/main/api/sound/examples



- - -

This software is not sponsored or supported by Panic.
