#[derive(Debug)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Rom {
    pub fn load(buf: &mut [u8]) -> Self {
        let chr_rom_start = 0x10 + buf[4] as usize * 0x4000;
        let chr_rom_end = chr_rom_start + buf[5] as usize * 0x2000;
        Self {
            prg_rom: buf[0x10..chr_rom_start].to_vec(),
            chr_rom: buf[chr_rom_start..chr_rom_end].to_vec(),
        }
    }

    pub fn prg_readb(&self, addr: usize) -> u8 {
        self.prg_rom[addr]
    }

    pub fn chr_readb(&self, addr: usize) -> u8 {
        self.chr_rom[addr]
    }

    pub fn prg_readw(&self, addr: usize) -> u16 {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.prg_rom[addr..= addr + 1]);
        u16::from_le_bytes(bytes)
    }

    pub fn chr_readw(&self, addr: usize) -> u16 {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.chr_rom[addr..= addr + 1]);
        u16::from_le_bytes(bytes)
    }
}
