# Playdate Simulator Utils

Cross-platform utils to do things with Playdate Simulator.


Usage:

```rust
let pdx = PathBuf::from("path/to/my-game.pdx");
let sdk = PathBuf::from("path/to/playdate-sdk");

// Create a future with command execution:
simulator::run::run(&pdx, Some(&sdk)).await;

// Or create a command and do whatever:
let mut cmd = simulator::run::command(&pdx, Some(&sdk)).unwrap();
let stdout = cmd.output().unwrap().stdout;
println!("Sim output: {}", std::str::from_utf8(&stdout).unwrap());
```


## Prerequisites

1. Rust __nightly__ toolchain
3. [Playdate SDK][sdk] with Simulator
   - Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root. _This is optional, but good move to help the tool to find SDK, and also useful if you have more then one version of SDK._


[playdate-website]: https://play.date
[sdk]: https://play.date/dev/#cardSDK



## State

Early development state.

There is just one method to run pdx with sim now.



- - -

This software is not sponsored or supported by Panic.
