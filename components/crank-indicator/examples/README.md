# Examples


There is one example that demonstrates `CrankIndicator` in various modes and environments.

Use controls to change:
- `<-` & `->` arrows to change global render offset
- `^` & `⌄` arrows to change global render scale factor
- use system menu to change framerate


![example](https://github.com/boozook/playdate/assets/888526/70fe1d74-4cea-4fd4-ab2b-8e56d178c3b7)


# How to run

```bash
cargo playdate run -p=playdate-ui-crank-indicator --example=example --features=sys/lang-items,sys/entry-point,sys/try-trait-v2,system/try-trait-v2
```

More information how to use [cargo-playdate][] in help: `cargo playdate --help`.



[cargo-playdate]: https://crates.io/crates/cargo-playdate
