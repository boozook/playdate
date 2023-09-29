# Examples

Here is two examples:
- `sp` is for `SamplePlayer`, additionally uses other crates with parts of Playdate API to minimize the amount of code
- `fp` is for `FilePlayer`, very low-level, except for the parts that directly demonstrate the functionality of this package


# How to run

```bash
cargo playdate run -p=playdate-sound --example=fp --features=sys/lang-items

cargo playdate run -p=playdate-sound --example=sp --features=sys/lang-items
```


More information how to use [cargo-playdate][] in help: `cargo playdate --help`.



[cargo-playdate]: https://crates.io/crates/cargo-playdate
