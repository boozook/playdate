# Examples


# How to run

```bash
cargo playdate run -p=playdate-system --example=handler-static --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=handler-boxed --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=handler-pinned --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=serial-message --features=sys/lang-items,sys/entry-point
# then type in simulator's console: "!msg foobar" four times

cargo playdate run -p=playdate-system --example=update-state-in-serial-message-callback --features=sys/lang-items,sys/entry-point
```

More information how to use [cargo-playdate][] in help: `cargo playdate --help`.



[cargo-playdate]: https://crates.io/crates/cargo-playdate
