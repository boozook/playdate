# Sprite API for PlayDate

High-level sprite API built on-top of [playdate-sys][].

⚠️ Prior to the version `1.0` API is unstable and can be changed without deprecation period.


## Usage

```rust
use playdate_sprite::*;
use playdate_graphics::*;
use playdate_graphics::bitmap::Bitmap;

let bitmap = Bitmap::new(50, 50, Color::BLACK)?;
let sprite = Sprite::new();

sprite.set_draw_mode(BitmapDrawMode::Copy);
sprite.set_image(&bitmap, BitmapFlip::Unflipped);
sprite.move_to(CENTER_X as _, CENTER_Y as _);
sprite.add();
...
```

More covered in [examples][gh-examples].

[gh-examples]: https://github.com/boozook/playdate/tree/main/api/sprite/examples
[playdate-sys]: https://crates.io/crates/playdate-sys



- - -

This software is not sponsored or supported by Panic.
