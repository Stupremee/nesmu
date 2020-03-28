use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct Rom {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Rom {
    pub fn load<R: Read>(read: &mut R) -> io::Result<Self> {
        let mut header = [0u8; 16];
        read.read(&mut header)?;
        // TODO: Do something with header here
        let prg_rom_size = header[4] as usize * 0x4000;
        let chr_rom_size = header[5] as usize * 0x2000;

        let mut prg_rom = vec![0u8; prg_rom_size];
        read.read(&mut prg_rom)?;

        let mut chr_rom = vec![0u8; chr_rom_size];
        read.read(&mut chr_rom)?;

        Ok(Self { prg_rom, chr_rom })
    }

    pub fn prg_read(&self, addr: usize) -> Option<u8> {
        self.prg_rom.get(addr).map(|b| *b)
    }

    pub fn chr_read(&self, addr: usize) -> Option<u8> {
        self.chr_rom.get(addr).map(|b| *b)
    }
}
