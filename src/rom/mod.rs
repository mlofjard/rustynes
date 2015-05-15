use std::fs::File;
use std::io::Read;
use std::vec::Vec;

// 16 Kb iNES or NES 2.0 header
pub struct Header {
	pub file_ident: [u8; 4],	// string reading "NES"<EOF>
    pub prg_rom_size: u8,   	// 16 Kb units
    pub chr_rom_size: u8,   	// 8 Kb units (0 indicates CHR-RAM)
    pub byte_6: u8,
    pub byte_7: u8,
    pub byte_8: u8,
    pub byte_9: u8,
    pub byte_10: u8,
    pub byte_11: u8,
    pub byte_12: u8,
    pub byte_13: u8,
    pub padding: [u8; 2], 		// always 0x00 in iNES and NES 2.0
}

pub struct Rom {
    pub header: Header,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Rom {
    pub fn create_from_file(file: &mut File) -> Rom {
        let mut buffer = [ 0; 16 ];
        file.read(&mut buffer).unwrap();

        let header = Header {
            file_ident: [
                buffer[0],
                buffer[1],
                buffer[2],
                buffer[3],
            ],
            prg_rom_size: buffer[4],
            chr_rom_size: buffer[5],
            byte_6: buffer[6],
            byte_7: buffer[7],
            byte_8: buffer[8],
            byte_9: buffer[9],
            byte_10: buffer[10],
            byte_11: buffer[11],
            byte_12: buffer[12],
            byte_13: buffer[13],
            padding: [ 0; 2 ]
        };

        assert!(header.file_ident == [
            0x4e, // N
            0x45, // E
            0x53, // S
            0x1a, // <EOF>
        ], "Broken header! Not a NES file in iNES or NES 2.0 format.");

        let mut prg_rom = vec![0u8; header.prg_rom_size as usize * 16384];
        file.read(&mut prg_rom).unwrap();
        
        let mut chr_rom = vec![0u8; header.chr_rom_size as usize * 8192];
        file.read(&mut chr_rom).unwrap();

        Rom {
            header: header,
            prg_rom: prg_rom,
            chr_rom: chr_rom,
        }
    }
}