const CPU_RAM_SIZE: usize = 0x800;

/// The Memory trait represents a thing that has a memory to write and read data.
pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);

    fn read_word(&self, addr: u16) -> u16 {
        let lower = self.read(addr) as u16;
        let upper = self.read(addr + 1) as u16;
        upper << 8 | lower
    }

    fn write_word(&mut self, addr: u16, val: u16) {
        self.write(addr, val as u8);
        self.write(addr, (val >> 8) as u8);
    }
}

/// Represents the internal CPU ram.
pub struct Ram {
    ram: [u8; CPU_RAM_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            ram: [0u8; CPU_RAM_SIZE],
        }
    }
}

impl Memory for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.ram[(addr & 0x7FF) as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.ram[(addr & 0x7FF) as usize] = val;
    }
}
