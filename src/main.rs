//use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

mod rom;

fn main() { 
    let path = Path::new("nestest.nes");
    let pathd = path.display();

    let mut rom_handle = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", pathd, Error::description(&why)),
        Ok(file) => file,
    };

    let rom = rom::Rom::create_from_file(&mut rom_handle);

    println!("Size of PrgROM: {}", rom.header.prg_rom_size as i32 * 16384);
    println!("Size of ChrROM: {}", rom.header.chr_rom_size as i32 * 8192);

    println!("PrgROM [0..10]:");
    for byte in &rom.prg_rom[..10] {
        println!("{}", byte);
    }
}