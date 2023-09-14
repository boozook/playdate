# System API for PlayDate

High-level system API built on-top of [playdate-sys][].


## Usage

```rust
use playdate_system::System;
use playdate_sys::ffi::PDLanguage;
use playdate_sys::println;

let system = System::new();

match system.language() {
	PDLanguage::kPDLanguageEnglish => println!("Hello"),
	PDLanguage::kPDLanguageJapanese => println!("こんにちは"),
	PDLanguage::kPDLanguageUnknown => println!("Привет"),
}
system.draw_fps(20, 20);
```


[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

This software is not sponsored or supported by Panic.
