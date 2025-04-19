# Controls API for PlayDate

High-level controls API built on-top of [playdate-sys][].

Covered components: buttons, accelerometer and crank.


## Usage

[Common prerequisites described in the wiki](https://github.com/boozook/playdate/wiki#prerequisites).

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

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.

This software is not sponsored or supported by Panic.
