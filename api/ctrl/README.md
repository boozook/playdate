# Controls API for PlayDate

High-level controls API built on-top of [playdate-sys][].

Covered components: buttons, accelerometer and crank.

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


## Prerequisites

1. Rust __nightly__ toolchain (rustup is optional)
1. [Playdate SDK][sdk]
1. Follow the [official documentation][doc-prerequisites]
1. Follow the instructions for [playdate-sys][]

[sdk]: https://play.date/dev/#cardSDK
[doc-prerequisites]: https://sdk.play.date/Inside%20Playdate%20with%20C.html#_prerequisites


## Usage

Buttons:
```rust
extern crate playdate_controls as controls;

// Get buttons state
let buttons = controls::peripherals::Buttons::get();

if buttons.current.a() { println("button A currently is DOWN") }
if buttons.pushed.b() { println("button B was pushed") }
if buttons.released.b() { println("button B was released") }
```

Accelerometer:
```rust
extern crate playdate_controls as controls;

// Turn on the accelerometer
controls::peripherals::Accelerometer::enable();

// Get accelerometer data
let (x, y, z) = controls::peripherals::Accelerometer::get();
println!("[{x:.2},{y:.2},{z:.2}]");
```

See more in [examples][playdate-controls-examples].


[playdate-sys]: https://crates.io/crates/playdate-sys
[playdate-controls-examples]: https://github.com/boozook/playdate#//TODO:PATH-TO-EXAMPLES




- - -

This software is not sponsored or supported by Panic.
