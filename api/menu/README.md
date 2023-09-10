# System Menu API for PlayDate

High-level system menu API built on-top of [playdate-sys][].


## Usage

```rust
extern crate playdate_menu;
use playdate_menu::*;

fn callback(userdata: &mut u32) { *userdata += 1 }

let simple = SimpleMenuItem::new("Simple", Some(callback), 0);
let check = CheckMenuItem::new("Check", false, None, ());
let opts = OptionsMenuItem::new("Opts", ["No", "Yes"], None, ());
```


[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

This software is not sponsored or supported by Panic.
