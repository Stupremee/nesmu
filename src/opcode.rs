macro_rules! opcode {
    ($inst:ident, $addr:ident, $num:expr) => {
        Opcode::new(Instruction::$inst, AddressMode::$addr, CYCLES[$num])
    };
}

#[rustfmt::skip]
const CYCLES: [u8; 256] = [
    /* 0x00 */ 7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6,
    /* 0x10 */ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /* 0x20 */ 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6,
    /* 0x30 */ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /* 0x40 */ 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6,
    /* 0x50 */ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /* 0x60 */ 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
    /* 0x70 */ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /* 0x80 */ 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
    /* 0x90 */ 2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5,
    /* 0xA0 */ 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
    /* 0xB0 */ 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
    /* 0xC0 */ 2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
    /* 0xD0 */ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    /* 0xE0 */ 2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
    /* 0xF0 */ 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];

pub const OPCODES: [Opcode; 256] = [
    // ==========================
    opcode!(BRK, Implied, 0),
    opcode!(ORA, IndirectXIndexed, 0x01),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, Zeropage, 0x04),
    opcode!(ORA, Zeropage, 0x05),
    opcode!(ASL, Zeropage, 0x06),
    Opcode::invalid(),
    opcode!(PHP, Implied, 0x08),
    opcode!(ORA, Immediate, 0x09),
    opcode!(ASL, Accumulator, 0x0A),
    Opcode::invalid(),
    opcode!(NOP, Absolute, 0x0C),
    opcode!(ORA, Absolute, 0x0D),
    opcode!(ASL, Absolute, 0x0E),
    Opcode::invalid(),
    // ==========================
    opcode!(BPL, Relative, 0x10),
    opcode!(ORA, IndirectYIndexed, 0x11),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, ZeropageXIndexed, 0x14),
    opcode!(ORA, ZeropageXIndexed, 0x15),
    opcode!(ASL, ZeropageXIndexed, 0x16),
    Opcode::invalid(),
    opcode!(CLC, Implied, 0x18),
    opcode!(ORA, AbsoluteYIndexed, 0x19),
    opcode!(NOP, Implied, 0x1A),
    Opcode::invalid(),
    opcode!(NOP, AbsoluteXIndexed, 0x1C),
    opcode!(ORA, AbsoluteXIndexed, 0x1D),
    opcode!(ASL, AbsoluteXIndexed, 0x1E),
    Opcode::invalid(),
    // ==========================
    opcode!(JSR, Absolute, 0x20),
    opcode!(AND, IndirectXIndexed, 0x20),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(BIT, Zeropage, 0x24),
    opcode!(AND, Zeropage, 0x25),
    opcode!(ROL, Zeropage, 0x26),
    Opcode::invalid(),
    opcode!(PLP, Implied, 0x28),
    opcode!(AND, Immediate, 0x29),
    opcode!(ROL, Accumulator, 0x2A),
    Opcode::invalid(),
    opcode!(BIT, Absolute, 0x2C),
    opcode!(AND, Absolute, 0x2D),
    opcode!(ROL, Absolute, 0x2E),
    Opcode::invalid(),
    // ==========================
    opcode!(BMI, Relative, 0x30),
    opcode!(AND, IndirectYIndexed, 0x31),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, ZeropageXIndexed, 0x34),
    opcode!(AND, ZeropageXIndexed, 0x35),
    opcode!(ROL, ZeropageXIndexed, 0x36),
    Opcode::invalid(),
    opcode!(SEC, Implied, 0x38),
    opcode!(AND, AbsoluteYIndexed, 0x39),
    opcode!(NOP, Implied, 0x3A),
    Opcode::invalid(),
    opcode!(NOP, AbsoluteXIndexed, 0x3C),
    opcode!(AND, AbsoluteXIndexed, 0x3D),
    opcode!(ROL, AbsoluteXIndexed, 0x3E),
    Opcode::invalid(),
    // ==========================
    opcode!(RTI, Implied, 0x40),
    opcode!(EOR, IndirectXIndexed, 0x41),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, Zeropage, 0x04),
    opcode!(EOR, Zeropage, 0x45),
    opcode!(LSR, Zeropage, 0x46),
    Opcode::invalid(),
    opcode!(PHA, Implied, 0x48),
    opcode!(EOR, Immediate, 0x49),
    opcode!(LSR, Accumulator, 0x4A),
    Opcode::invalid(),
    opcode!(JMP, Absolute, 0x4C),
    opcode!(EOR, Absolute, 0x4D),
    opcode!(LSR, Absolute, 0x4E),
    Opcode::invalid(),
    // ==========================
    opcode!(BVC, Relative, 0x50),
    opcode!(EOR, IndirectYIndexed, 0x51),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, ZeropageXIndexed, 0x34),
    opcode!(EOR, ZeropageXIndexed, 0x55),
    opcode!(LSR, ZeropageXIndexed, 0x56),
    Opcode::invalid(),
    opcode!(CLI, Implied, 0x58),
    opcode!(EOR, AbsoluteYIndexed, 0x59),
    opcode!(NOP, Implied, 0x5A),
    Opcode::invalid(),
    opcode!(NOP, AbsoluteXIndexed, 0x5C),
    opcode!(EOR, AbsoluteXIndexed, 0x5D),
    opcode!(LSR, AbsoluteXIndexed, 0x5E),
    Opcode::invalid(),
    // ==========================
    opcode!(RTS, Implied, 0x60),
    opcode!(ADC, IndirectXIndexed, 0x61),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, Zeropage, 0x64),
    opcode!(ADC, Zeropage, 0x65),
    opcode!(ROR, Zeropage, 0x66),
    Opcode::invalid(),
    opcode!(PLA, Implied, 0x68),
    opcode!(ADC, Immediate, 0x69),
    opcode!(ROR, Accumulator, 0x6A),
    Opcode::invalid(),
    opcode!(JMP, Indirect, 0x6C),
    opcode!(ADC, Absolute, 0x6D),
    opcode!(ROR, Absolute, 0x6E),
    Opcode::invalid(),
    // ==========================
    opcode!(BVS, Relative, 0x70),
    opcode!(ADC, IndirectYIndexed, 0x71),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, ZeropageXIndexed, 0x74),
    opcode!(ADC, ZeropageXIndexed, 0x75),
    opcode!(ROR, ZeropageXIndexed, 0x76),
    Opcode::invalid(),
    opcode!(SEI, Implied, 0x78),
    opcode!(ADC, AbsoluteYIndexed, 0x79),
    opcode!(NOP, Implied, 0x7A),
    Opcode::invalid(),
    opcode!(NOP, AbsoluteXIndexed, 0x7C),
    opcode!(ADC, AbsoluteXIndexed, 0x7D),
    opcode!(ROR, AbsoluteXIndexed, 0x7E),
    Opcode::invalid(),
    // ==========================
    opcode!(NOP, Immediate, 0x80),
    opcode!(STA, IndirectXIndexed, 0x81),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(STY, Zeropage, 0x84),
    opcode!(STA, Zeropage, 0x85),
    opcode!(STX, Zeropage, 0x86),
    Opcode::invalid(),
    opcode!(DEY, Implied, 0x88),
    Opcode::invalid(),
    opcode!(TXA, Implied, 0x8A),
    Opcode::invalid(),
    opcode!(STY, Absolute, 0x8C),
    opcode!(STA, Absolute, 0x8D),
    opcode!(STX, Absolute, 0x8E),
    Opcode::invalid(),
    // ==========================
    opcode!(BCC, Relative, 0x90),
    opcode!(STA, IndirectYIndexed, 0x91),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(STY, ZeropageXIndexed, 0x94),
    opcode!(STA, ZeropageXIndexed, 0x95),
    opcode!(STX, ZeropageYIndexed, 0x96),
    Opcode::invalid(),
    opcode!(TYA, Implied, 0x98),
    opcode!(STA, AbsoluteYIndexed, 0x99),
    opcode!(TXS, Implied, 0x9A),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(STA, AbsoluteXIndexed, 0x9D),
    Opcode::invalid(),
    Opcode::invalid(),
    // ==========================
    opcode!(LDY, Immediate, 0xA0),
    opcode!(LDA, IndirectXIndexed, 0xA1),
    opcode!(LDX, Immediate, 0xA2),
    opcode!(LAX, IndirectXIndexed, 0xA3),
    opcode!(LDY, Zeropage, 0xA4),
    opcode!(LDA, Zeropage, 0xA5),
    opcode!(LDX, Zeropage, 0xA6),
    opcode!(LAX, Zeropage, 0xA7),
    opcode!(TAY, Implied, 0xA8),
    opcode!(LDA, Immediate, 0xA9),
    opcode!(TAX, Implied, 0xAA),
    Opcode::invalid(),
    opcode!(LDY, Absolute, 0xAC),
    opcode!(LDA, Absolute, 0xAD),
    opcode!(LDX, Absolute, 0xAE),
    opcode!(LAX, Absolute, 0xAF),
    // ==========================
    opcode!(BCS, Relative, 0xB0),
    opcode!(LDA, IndirectYIndexed, 0xB1),
    Opcode::invalid(),
    opcode!(LAX, IndirectYIndexed, 0xB3),
    opcode!(LDY, ZeropageXIndexed, 0xB4),
    opcode!(LDA, ZeropageXIndexed, 0xB5),
    opcode!(LDX, ZeropageYIndexed, 0xB6),
    opcode!(LAX, ZeropageYIndexed, 0xB7),
    opcode!(CLV, Implied, 0xB8),
    opcode!(LDA, AbsoluteYIndexed, 0xB9),
    opcode!(TSX, Implied, 0xBA),
    Opcode::invalid(),
    opcode!(LDY, AbsoluteXIndexed, 0xBC),
    opcode!(LDA, AbsoluteXIndexed, 0xBD),
    opcode!(LDX, AbsoluteYIndexed, 0xBE),
    opcode!(LAX, AbsoluteYIndexed, 0xBF),
    // ==========================
    opcode!(CPY, Immediate, 0xC0),
    opcode!(CMP, IndirectXIndexed, 0xC1),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(CPY, Zeropage, 0xC4),
    opcode!(CMP, Zeropage, 0xC5),
    opcode!(DEC, Zeropage, 0xC6),
    Opcode::invalid(),
    opcode!(INY, Implied, 0xC8),
    opcode!(CMP, Immediate, 0xC9),
    opcode!(DEX, Implied, 0xCA),
    Opcode::invalid(),
    opcode!(CPY, Absolute, 0xCC),
    opcode!(CMP, Absolute, 0xCD),
    opcode!(DEC, Absolute, 0xCE),
    Opcode::invalid(),
    // ==========================
    opcode!(BNE, Relative, 0xD0),
    opcode!(CMP, IndirectYIndexed, 0xD1),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, ZeropageXIndexed, 0xD4),
    opcode!(CMP, ZeropageXIndexed, 0xD5),
    opcode!(DEC, ZeropageXIndexed, 0xD6),
    Opcode::invalid(),
    opcode!(CLD, Implied, 0xD8),
    opcode!(CMP, AbsoluteYIndexed, 0xD9),
    opcode!(NOP, Implied, 0xDA),
    Opcode::invalid(),
    opcode!(NOP, AbsoluteXIndexed, 0xDC),
    opcode!(CMP, AbsoluteXIndexed, 0xDD),
    opcode!(DEC, AbsoluteXIndexed, 0xDE),
    Opcode::invalid(),
    // ==========================
    opcode!(CPX, Immediate, 0xE0),
    opcode!(SBC, IndirectXIndexed, 0xE1),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(CPX, Zeropage, 0xE4),
    opcode!(SBC, Zeropage, 0xE5),
    opcode!(INC, Zeropage, 0xE6),
    Opcode::invalid(),
    opcode!(INX, Implied, 0xE8),
    opcode!(SBC, Immediate, 0xE9),
    opcode!(NOP, Implied, 0xEA),
    Opcode::invalid(),
    opcode!(CPX, Absolute, 0xEC),
    opcode!(SBC, Absolute, 0xED),
    opcode!(INC, Absolute, 0xEE),
    Opcode::invalid(),
    // ==========================
    opcode!(BEQ, Relative, 0xF0),
    opcode!(SBC, IndirectYIndexed, 0xF1),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(NOP, ZeropageXIndexed, 0xF4),
    opcode!(SBC, ZeropageXIndexed, 0xF5),
    opcode!(INC, ZeropageXIndexed, 0xF6),
    Opcode::invalid(),
    opcode!(SED, Implied, 0xF8),
    opcode!(SBC, AbsoluteYIndexed, 0xF9),
    opcode!(NOP, Implied, 0xFA),
    Opcode::invalid(),
    opcode!(NOP, AbsoluteXIndexed, 0xFC),
    opcode!(SBC, AbsoluteXIndexed, 0xFD),
    opcode!(INC, AbsoluteXIndexed, 0xFE),
    Opcode::invalid(),
    // ==========================
];

#[derive(Debug)]
pub struct Opcode {
    pub inst: Instruction,
    pub addr: AddressMode,
    pub cycles: u8,
}

impl Opcode {
    pub const fn new(inst: Instruction, addr: AddressMode, cycles: u8) -> Self {
        Self { inst, addr, cycles }
    }

    pub const fn invalid() -> Self {
        Self::new(Instruction::XXX, AddressMode::Implied, 0)
    }
}

#[derive(Debug)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    // Illegal opcodes
    LAX,

    XXX,
}

#[derive(Debug)]
pub enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    Immediate,
    Implied,
    Indirect,
    IndirectXIndexed,
    IndirectYIndexed,
    Relative,
    Zeropage,
    ZeropageXIndexed,
    ZeropageYIndexed,
}
