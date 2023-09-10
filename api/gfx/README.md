# Graphics API for PlayDate

High-level graphics API built on-top of [playdate-sys][].


## Usage

```rust
extern crate playdate_graphics;
use playdate_graphics::{bitmap, color, text};

// create and draw black rect:
let image = bitmap::Bitmap::new(100, 100, color::Color::BLACK).unwrap();
image.draw(50, 100, bitmap::BitmapFlip::kBitmapUnflipped);

// draw simple line of text:
let str = CStr::from_bytes_with_nul(b"Simple Text\0").unwrap();
text::draw_text_cstr(str, 40, 40);
```


[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

This software is not sponsored or supported by Panic.
