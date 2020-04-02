use crate::bus::Bus;
use crate::opcode::{self, AddressMode, Opcode};

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
    cycles: u8,
}

impl Cpu {
    pub fn new(bus: Bus, reg: Registers) -> Self {
        Self {
            bus,
            reg,
            cycles: 0,
        }
    }

    pub fn clock(&mut self) {
        if self.cycles > 0 {
            self.cycles -= 1;
        }

        let opcode = self.fetch();
        let opcode = &opcode::OPCODES[opcode as usize];

        self.cycles = opcode.cycles;

        let operand = self.fetch_operand(opcode);
    }

    fn fetch_operand(&mut self, op: &Opcode) -> u16 {
        match op.addr {
            AddressMode::Accumulator => self.reg.a as u16,
            AddressMode::Absolute => self.fetch_word(),
            AddressMode::AbsoluteXIndexed => self.fetch_absolute(self.reg.x),
            AddressMode::AbsoluteYIndexed => self.fetch_absolute(self.reg.y),
            AddressMode::Immediate => self.fetch_immediate(),
            AddressMode::Implied => 0x0,
            AddressMode::Indirect => self.fetch_indirect(),
            AddressMode::IndirectXIndexed => self.fetch_indirect_x(),
            AddressMode::IndirectYIndexed => self.fetch_indirect_y(),
            AddressMode::Relative => self.fetch_relative(),
            AddressMode::Zeropage => self.fetch_zeropage(),
            AddressMode::ZeropageXIndexed => self.fetch_zeropage_x(),
            AddressMode::ZeropageYIndexed => self.fetch_zeropage_y(),
        }
    }

    fn fetch(&mut self) -> u8 {
        let result = self.read(self.reg.pc);
        self.reg.pc += 1;
        result
    }
    
    fn fetch_word(&mut self) -> u16 {
        let lower = self.fetch() as u16;
        let upper = self.fetch() as u16;
        (upper << 8) | lower
    }


    fn fetch_immediate(&mut self) -> u16 {
        self.reg.pc += 1;
        self.reg.pc
    }

    fn fetch_absolute(&mut self, offset: u8) -> u16 {
        let base = self.fetch_word();
        let addr = base + offset as u16;

        if (addr & 0xFF00) != (base & 0xFF00) {
            self.cycles += 1;
        }
        addr
    }

    fn fetch_indirect(&mut self) -> u16 {
        let lower = self.fetch() as u16;
        let upper = self.fetch() as u16;

        let ptr = (upper << 8) | lower;

        let (lower, upper) = if lower == 0xFF {
            (self.read(ptr & 0xFF00) as u16, self.read(ptr) as u16)
        } else {
            (self.read(ptr & 1) as u16, self.read(ptr) as u16)
        };

        (upper << 8) | lower
    }

    fn fetch_indirect_x(&mut self) -> u16 {
        let ptr = self.fetch() as u16;

        let lower = self.read((ptr + self.reg.x as u16) & 0x00FF) as u16;
        let upper = self.read((ptr + self.reg.x as u16 + 1) & 0x00FF) as u16;

        (upper << 8) | lower
    }

    fn fetch_indirect_y(&mut self) -> u16 {
        let ptr = self.fetch() as u16;

        let lower = self.read(ptr & 0x00FF) as u16;
        let upper = self.read((ptr + 1) & 0x00FF) as u16;

        let addr = (upper << 8) | lower;
        let addr = addr + self.reg.y as u16;

        if (addr & 0xFF00) != (upper << 8) {
            self.cycles += 1;
        }

        addr
    }

    fn fetch_relative(&mut self) -> u16 {
        let base = self.fetch() as u16;

        if base < 0x80 {
            base + self.reg.pc
        } else {
            base + self.reg.pc - 256
        }
    }

    fn fetch_zeropage(&mut self) -> u16 {
        self.fetch() as u16 & 0xFF
    }

    fn fetch_zeropage_x(&mut self) -> u16 {
        (self.fetch() as u16 + self.reg.x as u16) & 0xFF
    }

    fn fetch_zeropage_y(&mut self) -> u16 {
        (self.fetch() as u16 + self.reg.y as u16) & 0xFF
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
