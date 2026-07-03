# Playdate Simulator Utils

Cross-platform utils to do things with Playdate Simulator.


Usage:

[Common prerequisites described in the wiki](https://github.com/boozook/playdate/wiki#prerequisites).

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


## State

Early development state.

There is just one method to run pdx with sim now.



- - -

This software is not sponsored or supported by Panic.
