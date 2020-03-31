use crate::bus::Bus;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            p: 0,
        }
    }
}

#[derive(Default, Debug)]
pub struct Cpu {
    bus: Bus,
    reg: Registers,
}

impl Cpu {
    pub fn new(bus: Bus, reg: Registers) -> Self {
        Self { bus, reg }
    }

    fn read_word(&self, addr: u16) -> u16 {
        self.bus.read_word(addr)
    }

    fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.bus.write(addr, val);
    }
}
