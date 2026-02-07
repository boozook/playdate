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
The above ☝🏻 snippet assumes that you have prepared a trace-dump...
</summary>

⚠️
If you don't know what it is, you probably don't need it. Be careful and perform all actions at your own risk.

### How to retrieve cpu-trace

1. connect to device
2. send command `"trace"`, dump it to `./trace-dump.txt`
3. send `"stoptrace"`

How to do it using [`pdtool`][pdtool] - see [Usage with `pdtool` / Trace](#trace) below.

</details>



All tools have `--help` parameter.


### Usage with `pdtool`

The [`pdtool`][pdtool] is a util for ease interaction with a device via USB.


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

```bash
cargo install playdate-symbolize
# or
cargo install playdate-symbolize --git=https://github.com/boozook/playdate.git
```

### Prerequisites

[Common prerequisites described in the wiki](https://github.com/boozook/playdate/wiki#prerequisites).

To symbolize pointers (or offsets) outside of your program you need [Playdate SDK][sdk].

Also you need your program - `elf` file saved __before packing into pdx__, definitely.



[pdtool]: https://crates.io/crates/playdate-tool
[sdk]: https://play.date/dev/#cardSDK


- - -

This software is not sponsored or supported by Panic.
