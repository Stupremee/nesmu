use crate::mem::{Memory, Ram};

#[derive(Default)]
pub struct Bus {
    ram: Ram,
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram.read(addr),
            0x2000..=0x3FFF => todo!("read from ppu"),
            0x4000..=0x4017 => todo!("read from apu or io registers"),
            0x4018..=0x401F => panic!("this memory region is disabled"),
            0x4020..=0xFFFF => todo!("read from cartridge"),
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram.write(addr, val),
            0x2000..=0x3FFF => todo!("write from ppu"),
            0x4000..=0x4017 => todo!("write from apu or io registers"),
            0x4018..=0x401F => panic!("this memory region is disabled"),
            0x4020..=0xFFFF => todo!("write from cartridge"),
        };
    }
}
