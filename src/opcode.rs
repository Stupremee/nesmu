use lazy_static::lazy_static;
use std::collections::HashMap;

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

lazy_static! {
    static ref OPCODES: HashMap<u8, Opcode> = {
        let mut m = HashMap::new();
        m.insert(0xA9, Opcode::new(Instruction::LDA, AddressMode::Immediate, CYCLES[0xA9]));
        m.insert(0xA5, Opcode::new(Instruction::LDA, AddressMode::Zeropage, CYCLES[0xA5]));
        m.insert(0xB5, Opcode::new(Instruction::LDA, AddressMode::ZeropageXIndexed, CYCLES[0xB5]));
        m.insert(0xAD, Opcode::new(Instruction::LDA, AddressMode::Absolute, CYCLES[0xAD]));
        m.insert(0xBD, Opcode::new(Instruction::LDA, AddressMode::AbsoluteXIndexed, CYCLES[0xBD]));
        m.insert(0xB9, Opcode::new(Instruction::LDA, AddressMode::AbsoluteYIndexed, CYCLES[0xB9]));
        m.insert(0xA1, Opcode::new(Instruction::LDA, AddressMode::IndirectXIndexed, CYCLES[0xA1]));
        m.insert(0xB1, Opcode::new(Instruction::LDA, AddressMode::IndirectYIndexed, CYCLES[0xB1]));
        m.insert(0xA2, Opcode::new(Instruction::LDX, AddressMode::Immediate, CYCLES[0xA2]));
        m.insert(0xA6, Opcode::new(Instruction::LDX, AddressMode::Zeropage, CYCLES[0xA6]));
        m.insert(0xAE, Opcode::new(Instruction::LDX, AddressMode::Absolute, CYCLES[0xAE]));
        m.insert(0xB6, Opcode::new(Instruction::LDX, AddressMode::ZeropageYIndexed, CYCLES[0xB6]));
        m.insert(0xBE, Opcode::new(Instruction::LDX, AddressMode::AbsoluteYIndexed, CYCLES[0xBE]));
        m.insert(0xA0, Opcode::new(Instruction::LDY, AddressMode::Immediate, CYCLES[0xA0]));
        m.insert(0xA4, Opcode::new(Instruction::LDY, AddressMode::Zeropage, CYCLES[0xA4]));
        m.insert(0xAC, Opcode::new(Instruction::LDY, AddressMode::Absolute, CYCLES[0xAC]));
        m.insert(0xB4, Opcode::new(Instruction::LDY, AddressMode::ZeropageXIndexed, CYCLES[0xB4]));
        m.insert(0xBC, Opcode::new(Instruction::LDY, AddressMode::AbsoluteXIndexed, CYCLES[0xBC]));
        m.insert(0x85, Opcode::new(Instruction::STA, AddressMode::Zeropage, CYCLES[0x85]));
        m.insert(0x8D, Opcode::new(Instruction::STA, AddressMode::Absolute, CYCLES[0x8D]));
        m.insert(0x95, Opcode::new(Instruction::STA, AddressMode::ZeropageXIndexed, CYCLES[0x95]));
        m.insert(0x9D, Opcode::new(Instruction::STA, AddressMode::AbsoluteXIndexed, CYCLES[0x9D]));
        m.insert(0x99, Opcode::new(Instruction::STA, AddressMode::AbsoluteYIndexed, CYCLES[0x99]));
        m.insert(0x81, Opcode::new(Instruction::STA, AddressMode::IndirectXIndexed, CYCLES[0x81]));
        m.insert(0x91, Opcode::new(Instruction::STA, AddressMode::IndirectYIndexed, CYCLES[0x91]));
        m.insert(0x86, Opcode::new(Instruction::STX, AddressMode::Zeropage, CYCLES[0x86]));
        m.insert(0x8E, Opcode::new(Instruction::STX, AddressMode::Absolute, CYCLES[0x8E]));
        m.insert(0x96, Opcode::new(Instruction::STX, AddressMode::ZeropageYIndexed, CYCLES[0x96]));
        m.insert(0x84, Opcode::new(Instruction::STY, AddressMode::Zeropage, CYCLES[0x84]));
        m.insert(0x8C, Opcode::new(Instruction::STY, AddressMode::Absolute, CYCLES[0x8C]));
        m.insert(0x94, Opcode::new(Instruction::STY, AddressMode::ZeropageXIndexed, CYCLES[0x94]));
        m.insert(0x8A, Opcode::new(Instruction::TXA, AddressMode::Implied, CYCLES[0x8A]));
        m.insert(0x98, Opcode::new(Instruction::TYA, AddressMode::Implied, CYCLES[0x98]));
        m.insert(0x9A, Opcode::new(Instruction::TXS, AddressMode::Implied, CYCLES[0x9A]));
        m.insert(0xA8, Opcode::new(Instruction::TAY, AddressMode::Implied, CYCLES[0xA8]));
        m.insert(0xAA, Opcode::new(Instruction::TAX, AddressMode::Implied, CYCLES[0xAA]));
        m.insert(0xBA, Opcode::new(Instruction::TSX, AddressMode::Implied, CYCLES[0xBA]));
        m.insert(0x08, Opcode::new(Instruction::PHP, AddressMode::Implied, CYCLES[0x08]));
        m.insert(0x28, Opcode::new(Instruction::PLP, AddressMode::Implied, CYCLES[0x28]));
        m.insert(0x48, Opcode::new(Instruction::PHA, AddressMode::Implied, CYCLES[0x48]));
        m.insert(0x68, Opcode::new(Instruction::PLA, AddressMode::Implied, CYCLES[0x68]));
        m.insert(0x69, Opcode::new(Instruction::ADC, AddressMode::Immediate, CYCLES[0x69]));
        m.insert(0x65, Opcode::new(Instruction::ADC, AddressMode::Zeropage, CYCLES[0x65]));
        m.insert(0x6D, Opcode::new(Instruction::ADC, AddressMode::Absolute, CYCLES[0x6D]));
        m.insert(0x75, Opcode::new(Instruction::ADC, AddressMode::ZeropageXIndexed, CYCLES[0x75]));
        m.insert(0x7D, Opcode::new(Instruction::ADC, AddressMode::AbsoluteXIndexed, CYCLES[0x7D]));
        m.insert(0x79, Opcode::new(Instruction::ADC, AddressMode::AbsoluteYIndexed, CYCLES[0x79]));
        m.insert(0x61, Opcode::new(Instruction::ADC, AddressMode::IndirectXIndexed, CYCLES[0x61]));
        m.insert(0x71, Opcode::new(Instruction::ADC, AddressMode::IndirectYIndexed, CYCLES[0x71]));
        m.insert(0xE9, Opcode::new(Instruction::SBC, AddressMode::Immediate, CYCLES[0xE9]));
        m.insert(0xE5, Opcode::new(Instruction::SBC, AddressMode::Zeropage, CYCLES[0xE5]));
        m.insert(0xED, Opcode::new(Instruction::SBC, AddressMode::Absolute, CYCLES[0xED]));
        m.insert(0xF5, Opcode::new(Instruction::SBC, AddressMode::ZeropageXIndexed, CYCLES[0xF5]));
        m.insert(0xFD, Opcode::new(Instruction::SBC, AddressMode::AbsoluteXIndexed, CYCLES[0xFD]));
        m.insert(0xF9, Opcode::new(Instruction::SBC, AddressMode::AbsoluteYIndexed, CYCLES[0xF9]));
        m.insert(0xE1, Opcode::new(Instruction::SBC, AddressMode::IndirectXIndexed, CYCLES[0xE1]));
        m.insert(0xF1, Opcode::new(Instruction::SBC, AddressMode::IndirectYIndexed, CYCLES[0xF1]));
        m.insert(0xE0, Opcode::new(Instruction::CPX, AddressMode::Immediate, CYCLES[0xE0]));
        m.insert(0xE4, Opcode::new(Instruction::CPX, AddressMode::Zeropage, CYCLES[0xE4]));
        m.insert(0xEC, Opcode::new(Instruction::CPX, AddressMode::Absolute, CYCLES[0xEC]));
        m.insert(0xC0, Opcode::new(Instruction::CPY, AddressMode::Immediate, CYCLES[0xC0]));
        m.insert(0xC4, Opcode::new(Instruction::CPY, AddressMode::Zeropage, CYCLES[0xC4]));
        m.insert(0xCC, Opcode::new(Instruction::CPY, AddressMode::Absolute, CYCLES[0xCC]));
        m.insert(0xC9, Opcode::new(Instruction::CMP, AddressMode::Immediate, CYCLES[0xC9]));
        m.insert(0xC5, Opcode::new(Instruction::CMP, AddressMode::Zeropage, CYCLES[0xC5]));
        m.insert(0xCD, Opcode::new(Instruction::CMP, AddressMode::Absolute, CYCLES[0xCD]));
        m.insert(0xD5, Opcode::new(Instruction::CMP, AddressMode::ZeropageXIndexed, CYCLES[0xD5]));
        m.insert(0xDD, Opcode::new(Instruction::CMP, AddressMode::AbsoluteXIndexed, CYCLES[0xDD]));
        m.insert(0xD9, Opcode::new(Instruction::CMP, AddressMode::AbsoluteYIndexed, CYCLES[0xD9]));
        m.insert(0xC1, Opcode::new(Instruction::CMP, AddressMode::IndirectXIndexed, CYCLES[0xC1]));
        m.insert(0xD1, Opcode::new(Instruction::CMP, AddressMode::IndirectYIndexed, CYCLES[0xD1]));
        m.insert(0x29, Opcode::new(Instruction::AND, AddressMode::Immediate, CYCLES[0x29]));
        m.insert(0x25, Opcode::new(Instruction::AND, AddressMode::Zeropage, CYCLES[0x25]));
        m.insert(0x2D, Opcode::new(Instruction::AND, AddressMode::Absolute, CYCLES[0x2D]));
        m.insert(0x35, Opcode::new(Instruction::AND, AddressMode::ZeropageXIndexed, CYCLES[0x35]));
        m.insert(0x3D, Opcode::new(Instruction::AND, AddressMode::AbsoluteXIndexed, CYCLES[0x3D]));
        m.insert(0x39, Opcode::new(Instruction::AND, AddressMode::AbsoluteYIndexed, CYCLES[0x39]));
        m.insert(0x21, Opcode::new(Instruction::AND, AddressMode::IndirectXIndexed, CYCLES[0x21]));
        m.insert(0x31, Opcode::new(Instruction::AND, AddressMode::IndirectYIndexed, CYCLES[0x31]));
        m.insert(0x49, Opcode::new(Instruction::EOR, AddressMode::Immediate, CYCLES[0x49]));
        m.insert(0x45, Opcode::new(Instruction::EOR, AddressMode::Zeropage, CYCLES[0x45]));
        m.insert(0x4D, Opcode::new(Instruction::EOR, AddressMode::Absolute, CYCLES[0x4D]));
        m.insert(0x55, Opcode::new(Instruction::EOR, AddressMode::ZeropageXIndexed, CYCLES[0x55]));
        m.insert(0x5D, Opcode::new(Instruction::EOR, AddressMode::AbsoluteXIndexed, CYCLES[0x5D]));
        m.insert(0x59, Opcode::new(Instruction::EOR, AddressMode::AbsoluteYIndexed, CYCLES[0x59]));
        m.insert(0x41, Opcode::new(Instruction::EOR, AddressMode::IndirectXIndexed, CYCLES[0x41]));
        m.insert(0x51, Opcode::new(Instruction::EOR, AddressMode::IndirectYIndexed, CYCLES[0x51]));
        m.insert(0x09, Opcode::new(Instruction::ORA, AddressMode::Immediate, CYCLES[0x09]));
        m.insert(0x05, Opcode::new(Instruction::ORA, AddressMode::Zeropage, CYCLES[0x05]));
        m.insert(0x0D, Opcode::new(Instruction::ORA, AddressMode::Absolute, CYCLES[0x0D]));
        m.insert(0x15, Opcode::new(Instruction::ORA, AddressMode::ZeropageXIndexed, CYCLES[0x15]));
        m.insert(0x1D, Opcode::new(Instruction::ORA, AddressMode::AbsoluteXIndexed, CYCLES[0x1D]));
        m.insert(0x19, Opcode::new(Instruction::ORA, AddressMode::AbsoluteYIndexed, CYCLES[0x19]));
        m.insert(0x01, Opcode::new(Instruction::ORA, AddressMode::IndirectXIndexed, CYCLES[0x01]));
        m.insert(0x11, Opcode::new(Instruction::ORA, AddressMode::IndirectYIndexed, CYCLES[0x11]));
        m.insert(0x24, Opcode::new(Instruction::BIT, AddressMode::Zeropage, CYCLES[0x24]));
        m.insert(0x2C, Opcode::new(Instruction::BIT, AddressMode::Absolute, CYCLES[0x2C]));
        m.insert(0x0A, Opcode::new(Instruction::ASL, AddressMode::Accumulator, CYCLES[0x0A]));
        m.insert(0x06, Opcode::new(Instruction::ASL, AddressMode::Zeropage, CYCLES[0x06]));
        m.insert(0x0E, Opcode::new(Instruction::ASL, AddressMode::Absolute, CYCLES[0x0E]));
        m.insert(0x16, Opcode::new(Instruction::ASL, AddressMode::ZeropageXIndexed, CYCLES[0x16]));
        m.insert(0x1E, Opcode::new(Instruction::ASL, AddressMode::AbsoluteXIndexed, CYCLES[0x1E]));
        m.insert(0x4A, Opcode::new(Instruction::LSR, AddressMode::Accumulator, CYCLES[0x4A]));
        m.insert(0x46, Opcode::new(Instruction::LSR, AddressMode::Zeropage, CYCLES[0x46]));
        m.insert(0x4E, Opcode::new(Instruction::LSR, AddressMode::Absolute, CYCLES[0x4E]));
        m.insert(0x56, Opcode::new(Instruction::LSR, AddressMode::ZeropageXIndexed, CYCLES[0x56]));
        m.insert(0x5E, Opcode::new(Instruction::LSR, AddressMode::AbsoluteXIndexed, CYCLES[0x5E]));
        m.insert(0x2A, Opcode::new(Instruction::ROL, AddressMode::Accumulator, CYCLES[0x2A]));
        m.insert(0x26, Opcode::new(Instruction::ROL, AddressMode::Zeropage, CYCLES[0x26]));
        m.insert(0x2E, Opcode::new(Instruction::ROL, AddressMode::Absolute, CYCLES[0x2E]));
        m.insert(0x36, Opcode::new(Instruction::ROL, AddressMode::ZeropageXIndexed, CYCLES[0x36]));
        m.insert(0x3E, Opcode::new(Instruction::ROL, AddressMode::AbsoluteXIndexed, CYCLES[0x3E]));
        m.insert(0x6A, Opcode::new(Instruction::ROR, AddressMode::Accumulator, CYCLES[0x6A]));
        m.insert(0x66, Opcode::new(Instruction::ROR, AddressMode::Zeropage, CYCLES[0x66]));
        m.insert(0x6E, Opcode::new(Instruction::ROR, AddressMode::Absolute, CYCLES[0x6E]));
        m.insert(0x76, Opcode::new(Instruction::ROR, AddressMode::ZeropageXIndexed, CYCLES[0x76]));
        m.insert(0x7E, Opcode::new(Instruction::ROR, AddressMode::AbsoluteXIndexed, CYCLES[0x7E]));
        m.insert(0xE8, Opcode::new(Instruction::INX, AddressMode::Implied, CYCLES[0xE8]));
        m.insert(0xC8, Opcode::new(Instruction::INY, AddressMode::Implied, CYCLES[0xC8]));
        m.insert(0xE6, Opcode::new(Instruction::INC, AddressMode::Zeropage, CYCLES[0xE6]));
        m.insert(0xEE, Opcode::new(Instruction::INC, AddressMode::Absolute, CYCLES[0xEE]));
        m.insert(0xF6, Opcode::new(Instruction::INC, AddressMode::ZeropageXIndexed, CYCLES[0xF6]));
        m.insert(0xFE, Opcode::new(Instruction::INC, AddressMode::AbsoluteXIndexed, CYCLES[0xFE]));
        m.insert(0xCA, Opcode::new(Instruction::DEX, AddressMode::Implied, CYCLES[0xCA]));
        m.insert(0x88, Opcode::new(Instruction::DEY, AddressMode::Implied, CYCLES[0x88]));
        m.insert(0xC6, Opcode::new(Instruction::DEC, AddressMode::Zeropage, CYCLES[0xC6]));
        m.insert(0xCE, Opcode::new(Instruction::DEC, AddressMode::Absolute, CYCLES[0xCE]));
        m.insert(0xD6, Opcode::new(Instruction::DEC, AddressMode::ZeropageXIndexed, CYCLES[0xD6]));
        m.insert(0xDE, Opcode::new(Instruction::DEC, AddressMode::AbsoluteXIndexed, CYCLES[0xDE]));
        m.insert(0x18, Opcode::new(Instruction::CLC, AddressMode::Implied, CYCLES[0x18]));
        m.insert(0x58, Opcode::new(Instruction::CLI, AddressMode::Implied, CYCLES[0x58]));
        m.insert(0xB8, Opcode::new(Instruction::CLV, AddressMode::Implied, CYCLES[0xB8]));
        m.insert(0x38, Opcode::new(Instruction::SEC, AddressMode::Implied, CYCLES[0x38]));
        m.insert(0x78, Opcode::new(Instruction::SEI, AddressMode::Implied, CYCLES[0x78]));
        m.insert(0xEA, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xEA]));
        m.insert(0x00, Opcode::new(Instruction::BRK, AddressMode::Implied, CYCLES[0x00]));
        m.insert(0x20, Opcode::new(Instruction::JSR, AddressMode::Absolute, CYCLES[0x20]));
        m.insert(0x4C, Opcode::new(Instruction::JMP, AddressMode::Absolute, CYCLES[0x4C]));
        m.insert(0x6C, Opcode::new(Instruction::JMP, AddressMode::Indirect, CYCLES[0x6C]));
        m.insert(0x40, Opcode::new(Instruction::RTI, AddressMode::Implied, CYCLES[0x40]));
        m.insert(0x60, Opcode::new(Instruction::RTS, AddressMode::Implied, CYCLES[0x60]));
        m.insert(0x10, Opcode::new(Instruction::BPL, AddressMode::Relative, CYCLES[0x10]));
        m.insert(0x30, Opcode::new(Instruction::BMI, AddressMode::Relative, CYCLES[0x30]));
        m.insert(0x50, Opcode::new(Instruction::BVC, AddressMode::Relative, CYCLES[0x50]));
        m.insert(0x70, Opcode::new(Instruction::BVS, AddressMode::Relative, CYCLES[0x70]));
        m.insert(0x90, Opcode::new(Instruction::BCC, AddressMode::Relative, CYCLES[0x90]));
        m.insert(0xB0, Opcode::new(Instruction::BCS, AddressMode::Relative, CYCLES[0xB0]));
        m.insert(0xD0, Opcode::new(Instruction::BNE, AddressMode::Relative, CYCLES[0xD0]));
        m.insert(0xF0, Opcode::new(Instruction::BEQ, AddressMode::Relative, CYCLES[0xF0]));
        m.insert(0xF8, Opcode::new(Instruction::SED, AddressMode::Implied, CYCLES[0xF8]));
        m.insert(0xD8, Opcode::new(Instruction::CLD, AddressMode::Implied, CYCLES[0xD8]));
        m.insert(0x1A, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x1A]));
        m.insert(0x3A, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x3A]));
        m.insert(0x5A, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x5A]));
        m.insert(0x7A, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x7A]));
        m.insert(0xDA, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xDA]));
        m.insert(0xFA, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xFA]));
        m.insert(0x02, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x02]));
        m.insert(0x12, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x12]));
        m.insert(0x22, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x22]));
        m.insert(0x32, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x32]));
        m.insert(0x42, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x42]));
        m.insert(0x52, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x52]));
        m.insert(0x62, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x62]));
        m.insert(0x72, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x72]));
        m.insert(0x92, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x92]));
        m.insert(0xB2, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xB2]));
        m.insert(0xD2, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xD2]));
        m.insert(0xF2, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xF2]));
        m.insert(0x80, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x80]));
        m.insert(0x82, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x82]));
        m.insert(0x89, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x89]));
        m.insert(0xC2, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xC2]));
        m.insert(0xE2, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xE2]));
        m.insert(0x04, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x04]));
        m.insert(0x44, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x44]));
        m.insert(0x64, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x64]));
        m.insert(0x14, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x14]));
        m.insert(0x34, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x34]));
        m.insert(0x54, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x54]));
        m.insert(0x74, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x74]));
        m.insert(0xD4, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xD4]));
        m.insert(0xF4, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xF4]));
        m.insert(0x0C, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x0C]));
        m.insert(0x1C, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x1C]));
        m.insert(0x3C, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x3C]));
        m.insert(0x5C, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x5C]));
        m.insert(0x7C, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0x7C]));
        m.insert(0xDC, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xDC]));
        m.insert(0xFC, Opcode::new(Instruction::NOP, AddressMode::Implied, CYCLES[0xFC]));
        m.insert(0xEB, Opcode::new(Instruction::SBC, AddressMode::Immediate, CYCLES[0xEB]));
        m
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opcode {
    pub inst: Instruction,
    pub addr: AddressMode,
    pub cycles: u8,
}

impl Opcode {
    pub const fn new(inst: Instruction, addr: AddressMode, cycles: u8) -> Self {
        Self { inst, addr, cycles }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn get_opcode(byte: u8) -> Option<Opcode> {
    OPCODES.get(&byte).map(|op| op.clone())
}
