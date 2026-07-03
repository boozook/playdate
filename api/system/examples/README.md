# Examples


## How to run

```bash
cargo playdate run -p=playdate-system --example=update-callback --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=server-time --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=serial-message --features=sys/lang-items,sys/entry-point
# then type in simulator's console: "!msg foobar" five times

cargo playdate run -p=playdate-system --example=buttons --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=accelerometer --features=sys/lang-items,sys/entry-point

cargo playdate run -p=playdate-system --example=launch-args --features=sys/lang-items,sys/entry-point
```

More information how to use [cargo-playdate][]: `cargo playdate --help`.



[cargo-playdate]: https://crates.io/crates/cargo-playdate
