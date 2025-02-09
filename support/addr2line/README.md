# Symbolizer for Playdate

These tools helps symbolize addresses from your game's ELF file (`pdex.elf`) and Playdate's CPU-trace logs or crash logs.

Provided command-line utilities to make this process easier:
- `pd-addr2line` - takes address, returns symbol with span
- `pd-symbolize-crashlog` - takes crashlog.txt, returns symbolized crashlog
- `pd-symbolize-trace` - takes trace-dump and symbolizing it

All of them have almost same interface.


## Usage

The `pd-addr2line` can operate with stdin or file.
```bash
pd-addr2line --exe pdex.elf -Cfri 0xc0a 0x8053C75 10 0x6000027a 0x080bf518
echo "0x8053c75\n0x80bf518" | pd-addr2line --exe pdex.elf -Cfri
```


The `pd-symbolize-crashlog` can process crashlog file, currently stdin not supported.
```bash
pd-symbolize-crashlog --exe pdex.elf -Cfr /Volumes/PLAYDATE/crashlog.txt
```


The `pd-symbolize-trace` can operate with stdin or file.
Parameter `--exe` is optional and usually not needed for traces, but 🤷🏻‍♂️.
```bash
export RUST_LOG="info" # prevent unnecessary logs from appearing in the output
pd-symbolize-trace -Cfri trace-dump.txt # parse file, without elf
pd-symbolize-trace --exe pdex.elf -Cfri trace-dump.txt # with elf
cat trace-dump.txt | pd-symbolize-trace --exe pdex.elf -Cfri # pipe
```

<details><summary>
The above ☝🏻 snippet assumes that you have prepared a trace-dump.
</summary>

⚠️
If you don't know what it is, you probably don't need it. Be careful and perform all actions at your own risk.

The above ☝🏻 snippet assumes that you have prepared a trace-dump.
You can do this by following the following steps.
I recomend to use `pdtool` for it.
1. connect to device
2. send command "trace", dump it to `./trace-dump.txt`
3. send "stoptrace"

</details>



All tools have `--help` parameter.


### Usage with `pdtool`

The [`pdtool`][] is a util for ease interaction with a device via USB.


#### Trace

```bash
pdtool send ! "trace 10" && pdtool read | pd-symbolize-trace --exe pdex.elf -Cfri | ./symbolized-trace.log
# ...
pdtool send ! stoptrace
```

#### Crashlog

```bash
pdtool mount --wait && pd-symbolize-crashlog --exe pdex.elf -Cfri /Volumes/PLAYDATE/crashlog.txt;
pdtool unmount
```


## Install

You can grab the latest [release][] or you can build your own.


### Build

To build tools you need Rust __nightly__ toolchain. Recomended version is [there][rust-toolchain].

#### From crates.io:

```bash
cargo install playdate-symbolize
```

#### From the repo:

```bash
cargo install playdate-symbolize --git=https://github.com/boozook/playdate.git
```


### Prerequisites

To symbolize pointers (or offsets) outside of your program you need [Playdate SDK][sdk].
Ensure that env var `PLAYDATE_SDK_PATH` points to the SDK root. _This is optional, but good move to help the tool to find SDK, and also useful if you have more then one version of SDK._

Also you need your program - `elf` saved before packing into pdx.



[pdtool]: https://crates.io/crates/playdate-tool
[release]: https://github.com/boozook/playdate/releases
[sdk]: https://play.date/dev/#cardSDK
[rust-toolchain]: https://github.com/boozook/playdate/blob/main/rust-toolchain.toml



- - -

This software is not sponsored or supported by Panic.
