use std::io::{self, prelude::*};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CartridgeLoadError {
    #[error("failed to read input")]
    IoError(#[from] io::Error),
    // TODO: Better and nicer errors
    #[error("rom has invalid format")]
    FormatError,
}

#[derive(Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Cartridge {
    pub fn load(r: &mut dyn Read) -> Result<Cartridge, CartridgeLoadError> {
        // TODO: Replace this with nom and much better error handling
        let mut header = [0u8; 16];

        if header[0..4] != *b"NES\x1a" {
            return Err(CartridgeLoadError::FormatError);
        }

        let header = CartridgeHeader {
            prg_rom_chunks: header[4],
            chr_rom_chunks: header[5],
            flags_6: header[6],
            flags_7: header[7],
            prg_ram_size: header[8],
            flags_9: header[9],
            flags_10: header[10],
        };

        let prg_bytes = header.prg_rom_chunks * 16384;
        let mut prg_rom = vec![0u8; prg_bytes as usize];
        r.read(&mut prg_rom)?;

        let chr_bytes = header.chr_rom_chunks * 8192;
        let mut chr_rom = vec![0u8; chr_bytes as usize];
        r.read(&mut chr_rom)?;

        Ok(Cartridge {
            header,
            prg_rom,
            chr_rom,
        })
    }
}

#[derive(Debug)]
pub struct CartridgeHeader {
    pub prg_rom_chunks: u8,
    pub chr_rom_chunks: u8,
    pub flags_6: u8,
    pub flags_7: u8,
    pub prg_ram_size: u8,
    pub flags_9: u8,
    pub flags_10: u8,
}
