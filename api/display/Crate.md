# Display API for PlayDate

High-level display API built on-top of [playdate-sys][].


## Usage

```rust
use playdate_display::Display;

let display = Display::default();

let width = display.width();
let height = display.height();
display.set_refresh_rate(30.0);
```


[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


This software is not sponsored or supported by Panic.
