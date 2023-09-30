# Examples

These examples additionally uses other crates with parts of Playdate API to minimize the amount of code.

- `sp` is for `SamplePlayer`, loads sample from file and play
- `fp` is for `FilePlayer`, same as above, but for `FilePlayer`


# How to run

```bash
cargo playdate run -p=playdate-sound --example=fp --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-sound --example=sp --features=sys/lang-items,sys/entry-point
```


More information how to use [cargo-playdate][] in help: `cargo playdate --help`.



[cargo-playdate]: https://crates.io/crates/cargo-playdate
