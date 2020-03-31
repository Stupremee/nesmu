const RAM_SIZE: usize = 0x4000;

pub struct Bus {
    ram: [u8; RAM_SIZE],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0u8; RAM_SIZE],
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lower = self.read(addr) as u16;
        let upper = self.read(addr + 1) as u16;
        upper << 8 | lower
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0xFFFF => self.ram[addr as usize],
            _ => 0x0,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0xFFFF => self.ram[addr as usize] = val,
        };
    }
}
