# System API for PlayDate

High-level system API built on-top of [playdate-sys][].

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


## Usage

```rust
use playdate_system::*;
use playdate_sys::println;

let system = System::new();

match system.language() {
	PDLanguage::English => println!("Hello"),
	PDLanguage::Japanese => println!("こんにちは"),
	PDLanguage::Unknown => println!("Привет"),
}
system.draw_fps(20, 20);
```


[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

This software is not sponsored or supported by Panic.
