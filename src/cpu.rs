use crate::bus::Bus;
use crate::opcode::{self, AddressMode, Instruction, Opcode};

const STACK_ADDRESS: u16 = 0x0100;

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
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
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
            sp: 0xFD,
            p: 0x34,
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Accumulator,
    XRegister,
    YRegister,
    Address(u16),
    Relative(i8),
    Implied,
}

impl Operand {
    pub fn read(&self, cpu: &Cpu) -> Option<u8> {
        match self {
            Operand::Accumulator => Some(cpu.reg.a),
            Operand::Address(addr) => Some(cpu.read(*addr)),
            Operand::XRegister => Some(cpu.reg.x),
            Operand::YRegister => Some(cpu.reg.y),
            _ => None,
        }
    }

    pub fn write(&self, cpu: &mut Cpu, val: u8) {
        match self {
            Operand::Accumulator => cpu.reg.a = val,
            Operand::XRegister => cpu.reg.x = val,
            Operand::YRegister => cpu.reg.y = val,
            Operand::Address(addr) => cpu.write(*addr, val),
            _ => {}
        };
    }

    pub fn absolute_addr(&self, cpu: &Cpu) -> Option<u16> {
        match self {
            Operand::Address(addr) => Some(*addr),
            Operand::Relative(addr) => Some((cpu.reg.pc as i32 + *addr as i32) as u16),
            _ => None,
        }
    }
}

#[derive(Default, Debug)]
pub struct Cpu {
    pub bus: Bus,
    pub reg: Registers,
    pub cycles: u8,
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
            return;
        }

        let opcode = self.fetch();
        let (opcode, raw_opcode) = (&opcode::OPCODES[opcode as usize], opcode);

        self.cycles = opcode.cycles;

        println!("Processing opcode: {:?}. reg = {:?}", opcode, self.reg);
        let operand = self.fetch_operand(opcode);
        self.execute_op(opcode, operand, raw_opcode);
    }

    fn execute_op(&mut self, code: &Opcode, op: Operand, raw: u8) {
        match code.inst {
            Instruction::ADC => self.adc(op),
            Instruction::AND => self.and(op),
            Instruction::ASL => self.asl(op),
            Instruction::BCC => self.bcc(op),
            Instruction::BCS => self.bcs(op),
            Instruction::BEQ => self.beq(op),
            Instruction::BIT => self.bit(op),
            Instruction::BMI => self.bmi(op),
            Instruction::BNE => self.bne(op),
            Instruction::BPL => self.bpl(op),
            Instruction::BRK => self.brk(op),
            Instruction::BVC => self.bvc(op),
            Instruction::BVS => self.bvs(op),
            Instruction::CLC => self.clc(),
            Instruction::CLD => self.cld(),
            Instruction::CLI => self.cli(),
            Instruction::CLV => self.clv(),
            Instruction::CMP => self.cmp(op),
            Instruction::CPX => self.cpx(op),
            Instruction::CPY => self.cpy(op),
            Instruction::DEC => self.dec(op),
            Instruction::DEX => self.dex(),
            Instruction::DEY => self.dey(),
            Instruction::EOR => self.eor(op),
            Instruction::INC => self.inc(op),
            Instruction::INX => self.inx(),
            Instruction::INY => self.iny(),
            Instruction::JMP => self.jmp(op),
            Instruction::JSR => self.jsr(op),
            Instruction::LDA => self.lda(op),
            Instruction::LDX => self.ldx(op),
            Instruction::LDY => self.ldy(op),
            Instruction::LSR => self.lsr(op),
            Instruction::NOP => self.nop(),
            Instruction::ORA => self.ora(op),
            Instruction::PHA => self.pha(),
            Instruction::PHP => self.php(),
            Instruction::PLA => self.pla(),
            Instruction::PLP => self.plp(),
            Instruction::ROL => self.rol(op),
            Instruction::ROR => self.ror(op),
            Instruction::RTI => self.rti(),
            Instruction::RTS => self.rts(),
            Instruction::SBC => self.sbc(op),
            Instruction::SEC => self.sec(),
            Instruction::SED => self.sed(),
            Instruction::SEI => self.sei(),
            Instruction::STA => self.sta(op),
            Instruction::STX => self.stx(op),
            Instruction::STY => self.sty(op),
            Instruction::TAX => self.tax(),
            Instruction::TAY => self.tay(),
            Instruction::TSX => self.tsx(),
            Instruction::TXA => self.txa(),
            Instruction::TXS => self.txs(),
            Instruction::TYA => self.tya(),
            Instruction::XXX => panic!(
                "Invalid opcode received. Opcode = {:?} Raw = {:x}",
                code, raw
            ),
        };
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
            AddressMode::Relative => Operand::Relative(self.fetch() as i8),
            AddressMode::Zeropage => Operand::Address(self.fetch_zeropage()),
            AddressMode::ZeropageXIndexed => Operand::Address(self.fetch_zeropage_x()),
            AddressMode::ZeropageYIndexed => Operand::Address(self.fetch_zeropage_y()),
        }
    }

    pub fn reset(&mut self) {
        self.reg.a = 0;
        self.reg.x = 0;
        self.reg.y = 0;
        self.reg.sp = 0xFD;
        self.reg.p = 0x0;

        self.reg.set_flag(StatusFlag::Unused, true);

        let addr = 0xFFFC;
        let lower = self.read(addr) as u16;
        let upper = self.read(addr + 1) as u16;

        self.reg.pc = (upper << 8) | lower;

        self.cycles = 8;
    }

    pub fn complete(&self) -> bool {
        self.cycles == 0
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
        self.reg.pc - 1
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

    fn push(&mut self, val: u8) {
        self.write(STACK_ADDRESS + self.reg.sp as u16, val);
        self.reg.sp -= 1;
    }

    fn pop(&mut self) -> u8 {
        self.reg.sp += 1;
        self.read(STACK_ADDRESS + self.reg.sp as u16)
    }

    fn adc(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap() as u16;
        let val = fetched + self.reg.a as u16 + self.reg.get_flag(StatusFlag::Carry) as u16;

        self.reg.set_flag(StatusFlag::Carry, val > 255);
        self.reg.set_flag(StatusFlag::Zero, (val & 0xFF) == 0);
        self.reg.set_flag(
            StatusFlag::Overflow,
            (!(self.reg.a as u16 ^ fetched) & (self.reg.a as u16 ^ val) & 0x80) != 0,
        );
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val as u8);
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

    fn branch(&mut self, op: Operand, condition: bool) {
        if condition {
            self.cycles += 1;
            let addr = op.absolute_addr(self).unwrap();

            if (addr & 0xFF00) != (self.reg.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.reg.pc = addr;
        }
    }

    fn bcc(&mut self, op: Operand) {
        let condition = !self.reg.get_flag(StatusFlag::Carry);
        self.branch(op, condition);
    }

    fn bcs(&mut self, op: Operand) {
        let condition = self.reg.get_flag(StatusFlag::Carry);
        self.branch(op, condition);
    }

    fn beq(&mut self, op: Operand) {
        let condition = self.reg.get_flag(StatusFlag::Zero);
        self.branch(op, condition);
    }

    fn bit(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap();
        let val = fetched & self.reg.a;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Overflow, fetched & 0x40 != 0);
        self.reg.set_flag(StatusFlag::Negative, fetched & 0x80 != 0);
    }

    fn bmi(&mut self, op: Operand) {
        let condition = self.reg.get_flag(StatusFlag::Negative);
        self.branch(op, condition);
    }

    fn bne(&mut self, op: Operand) {
        let condition = !self.reg.get_flag(StatusFlag::Zero);
        self.branch(op, condition);
    }

    fn bpl(&mut self, op: Operand) {
        let condition = !self.reg.get_flag(StatusFlag::Negative);
        self.branch(op, condition);
    }

    fn brk(&mut self, op: Operand) {
        self.reg.set_flag(StatusFlag::NoInterrupts, true);

        self.push((self.reg.pc >> 8) as u8);
        self.push(self.reg.pc as u8);

        self.reg.set_flag(StatusFlag::Break, true);
        self.push(self.reg.p);
        self.reg.set_flag(StatusFlag::Break, false);

        let lower = self.read(0xFFFE) as u16;
        let upper = self.read(0xFFFF) as u16;
        self.reg.pc = (upper << 8) | lower
    }

    fn bvc(&mut self, op: Operand) {
        let condition = !self.reg.get_flag(StatusFlag::Overflow);
        self.branch(op, condition);
    }

    fn bvs(&mut self, op: Operand) {
        let condition = self.reg.get_flag(StatusFlag::Overflow);
        self.branch(op, condition);
    }

    fn clc(&mut self) {
        self.reg.set_flag(StatusFlag::Carry, false);
    }

    fn cld(&mut self) {
        self.reg.set_flag(StatusFlag::Decimal, false);
    }

    fn cli(&mut self) {
        self.reg.set_flag(StatusFlag::NoInterrupts, false);
    }

    fn clv(&mut self) {
        self.reg.set_flag(StatusFlag::Overflow, false);
    }

    fn compare(&mut self, op: Operand, reg: Operand) {
        let val = op.read(self).unwrap();
        let reg = op.read(self).unwrap();

        let diff = reg as i16 - val as i16;
        if diff == 0 {
            self.reg.set_flag(StatusFlag::Carry, true);
            self.reg.set_flag(StatusFlag::Zero, true);
        } else if diff > 0 {
            self.reg.set_flag(StatusFlag::Carry, true);
        } else if val & 0x80 != 0 {
            self.reg.set_flag(StatusFlag::Negative, true);
        }
    }

    #[inline]
    fn cmp(&mut self, op: Operand) {
        self.compare(op, Operand::Accumulator);
    }

    #[inline]
    fn cpx(&mut self, op: Operand) {
        self.compare(op, Operand::XRegister);
    }

    #[inline]
    fn cpy(&mut self, op: Operand) {
        self.compare(op, Operand::YRegister);
    }

    fn dec(&mut self, op: Operand) {
        let val = op.read(self).unwrap() as i16;
        let val = val - 1;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val as u8);
    }

    #[inline]
    fn dex(&mut self) {
        self.dec(Operand::XRegister);
    }

    #[inline]
    fn dey(&mut self) {
        self.dec(Operand::YRegister);
    }

    fn eor(&mut self, op: Operand) {
        let val = op.read(self).unwrap();
        let val = self.reg.a ^ val;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        self.reg.a = val;
    }

    fn inc(&mut self, op: Operand) {
        let val = op.read(self).unwrap() as i16;
        let val = val + 1;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val as u8);
    }

    #[inline]
    fn inx(&mut self) {
        self.dec(Operand::XRegister);
    }

    #[inline]
    fn iny(&mut self) {
        self.dec(Operand::YRegister);
    }

    fn jmp(&mut self, op: Operand) {
        self.reg.pc = op.absolute_addr(self).unwrap();
    }

    fn jsr(&mut self, op: Operand) {
        self.push((self.reg.pc >> 8) as u8);
        self.push(self.reg.pc as u8);

        self.reg.pc = op.absolute_addr(self).unwrap();
    }

    fn ld_reg(&mut self, op: Operand, reg: Operand) {
        let val = op.read(self).unwrap();

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        reg.write(self, val);
    }

    #[inline]
    fn lda(&mut self, op: Operand) {
        self.ld_reg(op, Operand::Accumulator);
    }

    #[inline]
    fn ldx(&mut self, op: Operand) {
        self.ld_reg(op, Operand::XRegister);
    }

    #[inline]
    fn ldy(&mut self, op: Operand) {
        self.ld_reg(op, Operand::YRegister);
    }

    fn lsr(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap();
        let val = fetched >> 1;

        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Carry, fetched & 0x1 != 0);

        op.write(self, val);
    }

    fn nop(&mut self) {}

    fn ora(&mut self, op: Operand) {
        let val = op.read(self).unwrap();
        let val = self.reg.a | val;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        self.reg.a = val;
    }

    fn pha(&mut self) {
        self.push(self.reg.a);
    }

    fn php(&mut self) {
        self.push(self.reg.p);
    }

    fn pla(&mut self) {
        let val = self.pop();
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.a = val;
    }

    fn plp(&mut self) {
        let val = self.pop();
        self.reg.p = val;
    }

    fn rol(&mut self, op: Operand) {
        let val = op.read(self).unwrap() as u16;
        let val = (val << 1) | self.reg.get_flag(StatusFlag::Carry) as u16;

        self.reg.set_flag(StatusFlag::Carry, val & 0xFF00 != 0);
        self.reg.set_flag(StatusFlag::Zero, val & 0xFF == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val as u8);
    }

    fn ror(&mut self, op: Operand) {
        let val = op.read(self).unwrap() as u16;
        let val = (self.reg.get_flag(StatusFlag::Carry) as u16) << 1 | val as u16 >> 1;

        self.reg.set_flag(StatusFlag::Carry, val & 0x01 != 0);
        self.reg.set_flag(StatusFlag::Zero, val & 0xFF == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val as u8);
    }

    fn rti(&mut self) {
        self.reg.p = self.pop();

        let lower = self.pop() as u16;
        let upper = self.pop() as u16;
        self.reg.pc = (upper << 8) | lower;
    }

    fn rts(&mut self) {
        let lower = self.pop() as u16;
        let upper = self.pop() as u16;
        self.reg.pc = (upper << 8) | lower;
    }

    fn sbc(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap() as u16;
        let fetched = fetched ^ 0xFF;
        let val = self.reg.a as u16 + fetched + self.reg.get_flag(StatusFlag::Zero) as u16;

        self.reg.set_flag(StatusFlag::Carry, val & 0xFF00 != 0);
        self.reg.set_flag(StatusFlag::Zero, val & 0xFF == 0);
        self.reg.set_flag(
            StatusFlag::Overflow,
            (val ^ self.reg.a as u16) & (val ^ fetched) & 0x80 != 0,
        );
        self.reg.set_flag(StatusFlag::Negative, (val & 0x80) != 0);

        self.reg.a = val as u8;
    }

    fn sec(&mut self) {
        self.reg.set_flag(StatusFlag::Carry, true);
    }

    fn sed(&mut self) {
        self.reg.set_flag(StatusFlag::Decimal, true);
    }

    fn sei(&mut self) {
        self.reg.set_flag(StatusFlag::NoInterrupts, true);
    }

    fn sta(&mut self, op: Operand) {
        let val = self.reg.a;
        op.write(self, val);
    }

    fn stx(&mut self, op: Operand) {
        let val = self.reg.x;
        op.write(self, val);
    }

    fn sty(&mut self, op: Operand) {
        let val = self.reg.y;
        op.write(self, val);
    }

    fn tax(&mut self) {
        self.reg.x = self.reg.a;
        self.reg.set_flag(StatusFlag::Zero, self.reg.x == 0);
        self.reg.set_flag(StatusFlag::Zero, self.reg.x & 0x80 != 1);
    }

    fn tay(&mut self) {
        self.reg.y = self.reg.a;
        self.reg.set_flag(StatusFlag::Zero, self.reg.y == 0);
        self.reg.set_flag(StatusFlag::Zero, self.reg.y & 0x80 != 1);
    }

    fn tsx(&mut self) {
        self.reg.x = self.reg.sp;
        self.reg.set_flag(StatusFlag::Zero, self.reg.x == 0);
        self.reg.set_flag(StatusFlag::Zero, self.reg.x & 0x80 != 1);
    }

    fn txa(&mut self) {
        self.reg.a = self.reg.x;
        self.reg.set_flag(StatusFlag::Zero, self.reg.a == 0);
        self.reg.set_flag(StatusFlag::Zero, self.reg.a & 0x80 != 1);
    }

    fn txs(&mut self) {
        self.reg.sp = self.reg.x;
    }

    fn tya(&mut self) {
        self.reg.a = self.reg.y;
        self.reg.set_flag(StatusFlag::Zero, self.reg.a == 0);
        self.reg.set_flag(StatusFlag::Zero, self.reg.a & 0x80 != 1);
    }
}
