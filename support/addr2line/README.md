# Symbolizer for Playdate

Three tools:
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

The `pd-symbolize-trace` can operate with stdin or file.
Parameter `--exe` is optional and usually not needed for traces, but 🤷🏻‍♂️.
```bash
export RUST_LOG="info" # prevent unnecessary logs from appearing in the output
pd-symbolize-trace -Cfri trace-dump.txt # parse file, without elf
pd-symbolize-trace --exe pdex.elf -Cfri trace-dump.txt # with elf
cat trace-dump.txt | pd-symbolize-trace --exe pdex.elf -Cfri # pipe
```

The `pd-symbolize-crashlog` can process crashlog file, currently stdin not supported.
```bash
pd-symbolize-crashlog --exe pdex.elf -Cfr /Volumes/PLAYDATE/crashlog.txt
```

All tools have `--help` parameter.
