use crate::bus::Bus;

pub struct Cpu {
    bus: Bus,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self { bus }
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
