RustyNES
===
A Nintendo Entertainment System (NES) emulator written in Rust
---

### Requirements

* Rust (duh)
* Cargo

### Installation

`$ cargo build`

### Usage

```
$ rustynes --help
Usage: rustynes [options] <romfile>
       rustynes (-h | --help)
       rustynes --version

Options:
    -h, --help         Show this message.
    --version          Show the version of RustyNES.
    --mode MODE        Choose mode of execution.
                       Valid values: run (default), rominfo.
```

#### Options

* `--mode <mode>`
	* `run` (default)
	* `nestest` - Run with nestest log output