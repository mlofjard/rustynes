extern crate rustc_serialize;
extern crate docopt;

//use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use docopt::Docopt;

mod rom;

#[derive(RustcDecodable, Debug)]
enum Mode {
    Run,
    RomInfo
}

static USAGE: &'static str = "
Usage: rustynes [options] <romfile>
       rustynes (-h | --help)
       rustynes --version

Options:
    -h, --help         Show this message.
    --version          Show the version of RustyNES.
    --mode MODE        Choose mode of execution.
                       Valid values: run (default), rominfo.
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_romfile:    String,
    flag_help:      bool,
    flag_version:   bool,
    flag_mode:      Option<Mode>,
}

fn main() { 
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("rustynes {}", VERSION);
        return ();
    }

    let path = {
        let pathstr = match std::str::from_utf8(args.arg_romfile.as_bytes()) {
            Err(why) => panic!("ROM filename parse error: {}", Error::description(&why)),
            Ok(result) => result 
        };
        Path::new(pathstr)
    };

    let pathd = path.display();

    let mut rom_handle = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", pathd, Error::description(&why)),
        Ok(file) => file
    };

    let rom = rom::Rom::create_from_file(&mut rom_handle);

    match args.flag_mode {
        Some(Mode::RomInfo) => {
            println!("ROM format: {}", rom.get_rom_format());
            println!("Size of PRG-ROM: {} Kb", rom.header.prg_rom_size as i32 * 16384);
            println!("Size of CHR-ROM: {} Kb", rom.header.chr_rom_size as i32 * 8192);
            println!("Size of PRG-RAM: {} Kb", rom.get_prg_ram_size());
            println!("Mapper no: {}", rom.get_mapper_number());

            println!("PrgROM [0..10]:");
            for byte in &rom.prg_rom[..10] {
                println!("{}", byte);
            }
        },
        Some(Mode::Run) | None => ()
    }
}