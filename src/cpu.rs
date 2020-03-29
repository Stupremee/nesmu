use crate::{opcode::get_opcode, rom::Rom};

pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
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
    finished: bool,
}

impl Cpu {
    pub fn new(rom: Rom) -> Self {
        Self {
            reg: Registers::new(),
            rom,
            memory: [0; crate::MEMORY_SIZE],
            finished: false,
        }
    }

    pub fn run(&mut self) {
        let instruction = self.instruction();
        if instruction.is_none() {
            self.finished = true;
            return;
        }
        let code = get_opcode(instruction.unwrap());
        if code.is_none() {
            println!("Invalid instruction: 0x{:x}", instruction.unwrap());
            return;
        }
        println!("Opcode: {:?}", code);
    }

    pub fn instruction(&mut self) -> Option<u8> {
        let code = self.read(self.reg.pc as usize);
        self.reg.pc += 1;
        code
    }

    pub fn read(&self, addr: usize) -> Option<u8> {
        match addr {
            0x8000..=0xFFFF => self.rom.prg_read(addr - 0x8000),
            _ => self.memory.get(addr).map(|b| *b),
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}
