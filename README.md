RustyNES
===
A NES emulator written in Rust (for fun)
---

### Requirements

* Rust (duh)
* Cargo

### Compilation

```
$ cargo build
```

or

```
$ cargo build --release
```

will produce an executable `rustynes` in `target/debug` or `target/release` respectively.

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