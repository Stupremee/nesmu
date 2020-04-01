macro_rules! opcode {
    ($inst:ident, $addr:ident, $num:expr) => {
        Opcode::new(Instruction::$inst, AddressMode::$addr, CYCLES[$num])
    };
}

const CYCLES: [u8; 256] = [
    7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5,
    2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
    2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];

const OPCODES: [Opcode; 256] = [
    // ==========================
    opcode!(BRK, Implied, 0),
    opcode!(ORA, IndirectXIndexed, 0x01),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(ORA, Zeropage, 0x05),
    opcode!(ASL, Zeropage, 0x06),
    Opcode::invalid(),
    opcode!(PHP, Implied, 0x08),
    opcode!(ORA, Immediate, 0x09),
    opcode!(ASL, Accumulator, 0x0A),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(ORA, Absolute, 0x0D),
    opcode!(ASL, Absolute, 0x0E),
    Opcode::invalid(),
    // ==========================
    opcode!(BPL, Relative, 0x10),
    opcode!(ORA, IndirectYIndexed, 0x11),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(ORA, ZeropageXIndexed, 0x15),
    opcode!(ASL, ZeropageXIndexed, 0x16),
    Opcode::invalid(),
    opcode!(CLC, Implied, 0x18),
    opcode!(ASL, AbsoluteYIndexed, 0x19),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(ORA, AbsoluteXIndexed, 0x1D),
    opcode!(ASL, AbsoluteXIndexed, 0x1E),
    Opcode::invalid(),
    // ==========================
    opcode!(JSR, Absolute, 0x20),
    opcode!(AND, IndirextXIndexed, 0x20),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(BIT, Zeropage, 0x24),
    opcode!(AND, Zeropage, 0x25),
    opcode!(ROL, Zeropage, 0x26),
    Opcode::invalid(),
    opcode!(PLP, Implied, 0x28),
    opcode!(AND, Immediate, 0x29),
    opcode!(ROL, Absolute, 0x2A),
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
    Opcode::invalid(),
    opcode!(AND, ZeropageXIndexed, 0x35),
    opcode!(ROL, ZeropageXIndexed, 0x36),
    Opcode::invalid(),
    opcode!(SEC, Implied, 0x38),
    opcode!(AND, AbsoluteYIndexed, 0x39),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(AND, AbsoluteXIndexed, 0x3D),
    opcode!(ROL, AbsoluteXIndexed, 0x3E),
    Opcode::invalid(),
    // ==========================
    opcode!(RTI, Implied, 0x40),
    opcode!(EOR, IndirectXIndexed, 0x41),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
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
    Opcode::invalid(),
    opcode!(EOR, ZeropageXIndexed, 0x55),
    opcode!(LSR, ZeropageXIndexed, 0x56),
    Opcode::invalid(),
    opcode!(CLI, Implied, 0x58),
    opcode!(EOR, AbsoluteYIndexed, 0x59),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(EOR, AbsoluteXIndexed, 0x5D),
    opcode!(LSR, AbsoluteXIndexed, 0x5E),
    Opcode::invalid(),
    // ==========================
    opcode!(RTS, Implied, 0x60),
    opcode!(ADC, IndirectXIndexed, 0x61),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
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
    opcode!(ADC, IndirectXIndexed, 0x71),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(ADC, ZeropageXIndexed, 0x75),
    opcode!(ROR, ZeropageXIndexed, 0x76),
    Opcode::invalid(),
    opcode!(SEI, Implied, 0x78),
    opcode!(ADC, AbsoluteYIndexed, 0x79),
    Opcode::invalid(),
    Opcode::invalid(),
    Opcode::invalid(),
    opcode!(ADC, AbsoluteXIndexed, 0x7D),
    opcode!(ROR, AbsoluteXIndexed, 0x7E),
    Opcode::invalid(),
    // ==========================
    Opcode::invalid(),
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
    opcode!(STA, AbsoluteXIndexed, 0x99),
    Opcode::invalid(),
    Opcode::invalid(),
    // ==========================
    opcode!(LDY, Immediate, 0xA0),
    opcode!(LDA, IndirectXIndexed, 0xA1),
    opcode!(LDX, Immediate, 0xA2),
    Opcode::invalid(),
    opcode!(LDY, Zeropage, 0xA4),
    opcode!(LDA, Zeropage, 0xA5),
    opcode!(LDX, Zeropage, 0xA6),
    Opcode::invalid(),
    opcode!(TAY, Implied, 0xA8),
    opcode!(LDA, Immediate, 0xA9),
    opcode!(TAX, Implied, 0xAA),
    Opcode::invalid(),
    opcode!(LDY, Absolute, 0xAC),
    opcode!(LDA, Absolute, 0xAD),
    opcode!(LDX, Absolute, 0xAE),
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