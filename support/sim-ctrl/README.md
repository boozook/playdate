# Playdate Simulator Utils

Cross-platform utils to run Simulator in the Playdate SDK.


Example:

```rust
let pdx = PathBuf::from("path/to/my-game.pdx");
let sdk = PathBuf::from("path/to/playdate-sdk");

// Create a future with command execution:
simulator::run::run(&pdx, Some(&sdk)).await;

// Or create a command and do whatever:
let mut cmd = simulator::run::command(&pdx, Some(&sdk)).unwrap();
let stdout = cmd.output().unwrap().stdout;
println!("{}", std::str::from_utf8(&stdout).unwrap());
```





- - -

This software is not sponsored or supported by Panic.
