# Playdate Crank-Indicator Alert

Requires SDK 2.1.

Optimized port of [official lua version][crank-indicator-lua], implemented as sprite.

> Small system-styled indicator, alerting the player that this game will use the crank.

<img src="https://github.com/boozook/playdate/assets/888526/1a0cce5d-7e0b-471d-85ad-19fa1dcd9fc3" align="right">

See [examples][crank-indicator-examples] to learn how to use.
```rust
use playdate_ui_crank_indicator::CrankIndicator;
use playdate_display::DisplayScale;
use playdate_sprite::add_sprite;

let crank = CrankIndicator::new(DisplayScale::Normal)?;
add_sprite(&crank);
```


[crank-indicator-gh]: https://github.com/boozook/playdate/tree/main/components/crank-indicator
[crank-indicator-examples]: https://github.com/boozook/playdate/tree/main/components/crank-indicator/examples
[crank-indicator-lua]: https://sdk.play.date/Inside%20Playdate.html#C-ui.crankIndicator



- - -

This software is not sponsored or supported by Panic.
