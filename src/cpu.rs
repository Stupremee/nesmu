use crate::bus::Bus;
use crate::opcode::{self, AddressMode, Opcode};

#[derive(Debug)]
#[repr(u8)]
pub enum StatusFlag {
    Carry = 1 << 0,
    Zero = 1 << 1,
    NoInterrupts = 1 << 2,
    Decimal = 1 << 3,
    Break = 1 << 4,
    Unused = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

#[derive(Debug)]
pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
}

impl Registers {
    pub fn set_flag(&mut self, flag: StatusFlag, mode: bool) {
        if mode {
            self.p |= flag as u8;
        } else {
            self.p &= !(flag as u8);
        }
    }

    pub fn get_flag(&mut self, flag: StatusFlag) -> bool {
        (self.p & flag as u8) != 0
    }
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

#[derive(Debug)]
pub enum Operand {
    Accumulator,
    Address(u16),
    Implied,
}

impl Operand {
    pub fn read(&self, cpu: &Cpu) -> Option<u8> {
        match self {
            Operand::Accumulator => Some(cpu.reg.a),
            Operand::Address(addr) => Some(cpu.read(*addr)),
            Operand::Implied => None,
        }
    }

    pub fn write(&self, cpu: &mut Cpu, val: u8) {
        match self {
            Operand::Accumulator => cpu.reg.a = val,
            Operand::Address(addr) => cpu.write(*addr, val),
            Operand::Implied => {},
        };
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

    fn fetch_operand(&mut self, op: &Opcode) -> Operand {
        match op.addr {
            AddressMode::Accumulator => Operand::Accumulator,
            AddressMode::Absolute => Operand::Address(self.fetch_word()),
            AddressMode::AbsoluteXIndexed => Operand::Address(self.fetch_absolute(self.reg.x)),
            AddressMode::AbsoluteYIndexed => Operand::Address(self.fetch_absolute(self.reg.y)),
            AddressMode::Immediate => Operand::Address(self.fetch_immediate()),
            AddressMode::Implied => Operand::Implied,
            AddressMode::Indirect => Operand::Address(self.fetch_indirect()),
            AddressMode::IndirectXIndexed => Operand::Address(self.fetch_indirect_x()),
            AddressMode::IndirectYIndexed => Operand::Address(self.fetch_indirect_y()),
            AddressMode::Relative => Operand::Address(self.fetch_relative()),
            AddressMode::Zeropage => Operand::Address(self.fetch_zeropage()),
            AddressMode::ZeropageXIndexed => Operand::Address(self.fetch_zeropage_x()),
            AddressMode::ZeropageYIndexed => Operand::Address(self.fetch_zeropage_y()),
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
        let base = self.fetch() as i8;
        (self.reg.pc as i32 + base as i32) as u16
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

    fn adc(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap() as u16;
        let val = fetched + self.reg.a as u16 + self.reg.get_flag(StatusFlag::Carry) as u16;

        self.reg.set_flag(StatusFlag::Carry, val > 255);
        self.reg.set_flag(StatusFlag::Zero, (val & 0xFF) == 0);
        self.reg.set_flag(StatusFlag::Overflow, (!(self.reg.a as u16 ^ fetched) & (self.reg.a as u16 ^ val) & 0x80) != 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, (val & 0xFF) as u8);
    }

    fn and(&mut self, op: Operand) {
        let val = op.read(self).unwrap();
        let val = self.reg.a & val;

        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);

        op.write(self, val);
    }

    fn asl(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap();
        let val = fetched << 1;

        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Carry, fetched & 0x80 != 0);

        op.write(self, val);
    }
}
