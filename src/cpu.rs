pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
    memory: [u8; crate::MEMORY_SIZE],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            p: 0,
            memory: [0; crate::MEMORY_SIZE],
        }
    }

    pub fn mem_store(&mut self, address: usize, val: u8) {
        self.memory[address] = val;
    }

    pub fn mem_get(&mut self, address: usize) -> u8 {
        self.memory[address]
    }
}
