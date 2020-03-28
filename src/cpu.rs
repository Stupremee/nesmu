use crate::{opcode::get_opcode, rom::Rom};

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
    rom: Rom,
    memory: [u8; crate::MEMORY_SIZE],
    finished: bool,
}

impl Cpu {
    pub fn new(rom: Rom) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000,
            sp: 0xFD,
            p: 0x34,
            rom,
            memory: [0; crate::MEMORY_SIZE],
            finished: false,
        }
    }

    pub fn run(&mut self) {
        let code = self.instruction();
        if code.is_none() {
            self.finished = true;
            return;
        }
        let code = get_opcode(code.unwrap());
        if code.is_none() {
            self.finished = true;
            return;
        }
        println!("Opcode: {:?}", code);
    }

    pub fn instruction(&mut self) -> Option<u8> {
        let code = self.read(self.pc as usize);
        self.pc += 1;
        code
    }

    pub fn read(&self, addr: usize) -> Option<u8> {
        (match addr {
            0x8000..=0xFFFF => self.rom.read(addr - 0x8000),
            _ => self.memory.get(addr),
        })
        .map(|b| *b)
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}
