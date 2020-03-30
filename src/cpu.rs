use crate::rom::Rom;
use crate::opcode::{get_opcode, Opcode, AddressMode};

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000,
            sp: 0xFD,
            p: 0x34,
        }
    }
}

pub struct Cpu {
    reg: Registers,
    rom: Rom,
    memory: [u8; crate::MEMORY_SIZE],
}

impl Cpu {
    pub fn new(rom: Rom) -> Self {
        Self {
            reg: Registers::new(),
            rom,
            memory: [0; crate::MEMORY_SIZE],
        }
    }

    pub fn init(&mut self) {
        self.reg.pc = self.readw(0xFFFC);
        println!("pc = {:x}", self.reg.pc);
    }

    pub fn run(&mut self) {
        let code = get_opcode(self.fetch());
        if code.is_none() {
            println!("Invalid opcode read.");
            return;
        }
        let code = code.unwrap();
        let operand = self.fetch_operand(&code);

        println!("Opcode: {:?}", code);
        println!("Operand: {:?}", operand);
    }

    pub fn readb(&self, addr: usize) -> u8 {
        match addr {
            0x8000..=0xBFFF => self.rom.prg_readb(addr - 0x8000),
            0xC000..=0xFFFF if self.rom.prg_rom.len() <= 0x4000 => {
                self.rom.prg_readb(addr - 0xC000)
            },
            0xC000..=0xFFFF => self.rom.prg_readb(addr - 0x8000),
            _ => self.memory[addr],
        }
    }

    pub fn readw(&self, addr: usize) -> u16 {
        let lower = self.readb(addr) as u16;
        let upper = self.readb(addr + 1) as u16;
        upper << 8 | lower
    }

    fn fetch(&mut self) -> u8 {
        let code = self.readb(self.reg.pc as usize);
        self.reg.pc += 1;
        code
    }

    fn fetch_operand(&mut self, op: &Opcode) -> u16 {
        match op.addr {
            AddressMode::Accumulator => unimplemented!(),
            AddressMode::Absolute => unimplemented!(),
            AddressMode::AbsoluteXIndexed => unimplemented!(),
            AddressMode::AbsoluteYIndexed => unimplemented!(),
            AddressMode::Immediate => {
                let b = self.fetch();
                self.readw(b as usize)
            },
            AddressMode::Implied => 0x00,
            AddressMode::Indirect => unimplemented!(),
            AddressMode::IndirectXIndexed => unimplemented!(),
            AddressMode::IndirectYIndexed => unimplemented!(),
            AddressMode::Relative => unimplemented!(),
            AddressMode::Zeropage => unimplemented!(),
            AddressMode::ZeropageXIndexed => unimplemented!(),
            AddressMode::ZeropageYIndexed => unimplemented!(),
        }
    }
}
