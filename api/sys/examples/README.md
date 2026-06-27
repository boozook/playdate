# Examples

⚠️ All of the examples here are very low-level.


# How to run

```bash
cargo playdate run -p=playdate-sys --example=hello-world --features=lang-items

cargo playdate run -p=playdate-sys --example=handler --features=lang-items,entry-point

# same hello-world, but with bin target build without arm-gcc
cargo playdate run -p=playdate-sys --device --no-gcc --example=hello-world-bin --features=lang-items
cargo playdate run -p=playdate-sys --device --no-gcc --example=handler-bin --features=lang-items,entry-point
```

More information how to use [cargo-playdate][] in the `help` cmd: `cargo playdate --help`.


[cargo-playdate]: https://crates.io/crates/cargo-playdate
