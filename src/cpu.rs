use crate::bus::Bus;
use crate::mem::Memory;
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
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

#[derive(Default)]
pub struct Cpu {
    pub bus: Bus,
    pub reg: Registers,
    pub cycles: u8,
    pub cycle_count: u32,
    pub additional_cycle: bool,
}

impl Cpu {
    pub fn new(bus: Bus, reg: Registers) -> Self {
        Self {
            bus,
            reg,
            cycles: 0,
            cycle_count: 0,
            additional_cycle: false,
        }
    }

    pub fn execute_instruction(&mut self) {
        loop {
            self.clock();
            if self.cycles == 0 {
                break;
            }
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

        self.additional_cycle = false;
        let operand = self.fetch_operand(opcode);
        self.execute_op(opcode, operand.clone(), raw_opcode);

        if self.additional_cycle {
            self.cycles += 1;
        }

        self.cycle_count += self.cycles as u32;
        println!(
            "Processing opcode: {:?}. reg = {:?} operand = {:?} cycles = {:?}",
            opcode, self.reg, operand, self.cycle_count
        );
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
            Instruction::NOP => self.nop(raw),
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
            Instruction::XXX => self.nop(raw),
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

        let addr = 0xFFFC;
        self.reg.pc = self.read_word(0xFFFC);

        self.cycles = 7;
        self.cycle_count = 7;
    }

    pub fn irq(&mut self) {
        if self.reg.get_flag(StatusFlag::NoInterrupts) {
            return;
        }
        self.push_word(self.reg.pc);

        self.reg.set_flag(StatusFlag::Break, false);
        self.reg.set_flag(StatusFlag::NoInterrupts, false);
        self.push(self.reg.p);

        self.reg.pc = self.read_word(0xFFFE);
        self.cycles = 7;
    }

    pub fn nmi(&mut self) {
        self.push_word(self.reg.pc);

        self.reg.set_flag(StatusFlag::Break, false);
        self.reg.set_flag(StatusFlag::NoInterrupts, false);
        self.push(self.reg.p);

        self.reg.pc = self.read_word(0xFFFA);
        self.cycles = 7;
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
        let lower = self.fetch() as u16;
        let upper = self.fetch() as u16;
        let addr = (upper << 8) | lower;
        let addr = addr.wrapping_add(offset as u16);

        if (addr & 0xFF00) != (upper << 8) {
            self.additional_cycle = true;
        }
        addr
    }

    fn fetch_indirect(&mut self) -> u16 {
        let lower = self.fetch() as u16;
        let upper = self.fetch() as u16;

        let ptr = (upper << 8) | lower;

        let (upper, lower) = if lower == 0xFF {
            (self.read(ptr & 0xFF00) as u16, self.read(ptr) as u16)
        } else {
            (self.read(ptr + 1) as u16, self.read(ptr) as u16)
        };

        let addr = (upper << 8) | lower;
        addr
    }

    fn fetch_indirect_x(&mut self) -> u16 {
        let ptr = self.fetch() as u16;

        let x = self.reg.x as u16;
        let lower = self.read(ptr + x & 0x00FF) as u16;
        let upper = self.read((ptr + x + 1) & 0x00FF) as u16;

        (upper << 8) | lower
    }

    fn fetch_indirect_y(&mut self) -> u16 {
        let ptr = self.fetch() as u16;

        let lower = self.read(ptr & 0xFF) as u16;
        let upper = self.read((ptr + 1) & 0xFF) as u16;

        let addr = (upper << 8) | lower;
        let addr = addr.wrapping_add(self.reg.y as u16);

        if (addr & 0xFF00) != (upper << 8) {
            self.additional_cycle = true;
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

    fn push_word(&mut self, val: u16) {
        self.push((val >> 8) as u8);
        self.push(val as u8);
    }

    fn pop(&mut self) -> u8 {
        self.reg.sp += 1;
        self.read(STACK_ADDRESS + self.reg.sp as u16)
    }

    fn pop_word(&mut self) -> u16 {
        let lower = self.pop() as u16;
        let upper = self.pop() as u16;
        (upper << 8) | lower
    }

    fn adc(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap() as u16;
        let a = self.reg.a as u16;
        let val = a + fetched + self.reg.get_flag(StatusFlag::Carry) as u16;

        let v = !(a ^ fetched) & (a ^ val) & 0x80;
        self.reg.set_flag(StatusFlag::Overflow, v != 0);
        self.reg.set_flag(StatusFlag::Carry, (val & 0x100) != 0);
        self.reg.set_flag(StatusFlag::Zero, val & 0xFF == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.a = val as u8;

        self.additional_cycle &= true;
    }

    fn and(&mut self, op: Operand) {
        let val = op.read(self).unwrap();
        let val = self.reg.a & val;

        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);

        self.reg.a = val;

        self.additional_cycle &= true;
    }

    fn asl(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap();
        let val = fetched << 1;

        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Carry, fetched & 0x80 != 0);

        op.write(self, val);
        self.additional_cycle &= false;
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
        self.additional_cycle &= false;
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

        self.additional_cycle &= false;
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

        self.push_word(self.reg.pc);

        self.reg.set_flag(StatusFlag::Break, true);
        self.push(self.reg.p);
        self.reg.set_flag(StatusFlag::Break, false);

        self.reg.pc = self.read_word(0xFFFE);

        self.additional_cycle &= false;
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

        self.additional_cycle &= false;
    }

    fn cld(&mut self) {
        self.reg.set_flag(StatusFlag::Decimal, false);

        self.additional_cycle &= false;
    }

    fn cli(&mut self) {
        self.reg.set_flag(StatusFlag::NoInterrupts, false);

        self.additional_cycle &= false;
    }

    fn clv(&mut self) {
        self.reg.set_flag(StatusFlag::Overflow, false);

        self.additional_cycle &= false;
    }

    fn compare(&mut self, op: Operand, reg: Operand) {
        let val = op.read(self).unwrap();
        let reg = reg.read(self).unwrap();

        let diff = (reg as u16).wrapping_sub(val as u16);
        self.reg.set_flag(StatusFlag::Carry, reg >= val);
        self.reg.set_flag(StatusFlag::Zero, diff & 0xFF == 0);
        self.reg.set_flag(StatusFlag::Negative, diff & 0x80 != 0);
    }

    #[inline]
    fn cmp(&mut self, op: Operand) {
        self.compare(op, Operand::Accumulator);
        self.additional_cycle &= true;
    }

    #[inline]
    fn cpx(&mut self, op: Operand) {
        self.compare(op, Operand::XRegister);

        self.additional_cycle &= false;
    }

    #[inline]
    fn cpy(&mut self, op: Operand) {
        self.compare(op, Operand::YRegister);

        self.additional_cycle &= false;
    }

    fn dec(&mut self, op: Operand) {
        let val = op.read(self).unwrap() as i16;
        let val = val - 1;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val as u8);

        self.additional_cycle &= false;
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
        self.additional_cycle &= true;
    }

    fn inc(&mut self, op: Operand) {
        let val = op.read(self).unwrap();
        let val = val.wrapping_add(1);

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val);

        self.additional_cycle &= false;
    }

    #[inline]
    fn inx(&mut self) {
        self.inc(Operand::XRegister);
    }

    #[inline]
    fn iny(&mut self) {
        self.inc(Operand::YRegister);
    }

    fn jmp(&mut self, op: Operand) {
        self.reg.pc = op.absolute_addr(self).unwrap();

        self.additional_cycle &= false;
    }

    fn jsr(&mut self, op: Operand) {
        self.push_word(self.reg.pc - 1);
        self.reg.pc = op.absolute_addr(self).unwrap();

        self.additional_cycle &= false;
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
        self.additional_cycle &= true;
    }

    #[inline]
    fn ldx(&mut self, op: Operand) {
        self.ld_reg(op, Operand::XRegister);
        self.additional_cycle &= true;
    }

    #[inline]
    fn ldy(&mut self, op: Operand) {
        self.ld_reg(op, Operand::YRegister);
        self.additional_cycle &= true;
    }

    fn lsr(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap();
        let val = fetched >> 1;

        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Carry, fetched & 0x1 != 0);

        op.write(self, val);

        self.additional_cycle &= false;
    }

    fn nop(&mut self, op: u8) {
        self.additional_cycle &= match op {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => true,
            _ => false,
        }
    }

    fn ora(&mut self, op: Operand) {
        let val = op.read(self).unwrap();
        let val = self.reg.a | val;

        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        self.reg.a = val;
        self.additional_cycle &= true;
    }

    fn pha(&mut self) {
        self.push(self.reg.a);

        self.additional_cycle &= false;
    }

    fn php(&mut self) {
        self.reg.set_flag(StatusFlag::Unused, true);
        self.reg.set_flag(StatusFlag::Break, true);
        self.push(self.reg.p);
        self.reg.set_flag(StatusFlag::Break, false);

        self.additional_cycle &= false;
    }

    fn pla(&mut self) {
        let val = self.pop();
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.a = val;

        self.additional_cycle &= false;
    }

    fn plp(&mut self) {
        let val = self.pop();
        self.reg.p = val;
        self.reg.set_flag(StatusFlag::Unused, true);
        self.reg.set_flag(StatusFlag::Break, false);

        self.additional_cycle &= false;
    }

    fn rol(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap();
        let carry = self.reg.get_flag(StatusFlag::Carry) as u8;
        let val = (fetched << 1) | carry;

        self.reg.set_flag(StatusFlag::Carry, fetched & 0x80 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);

        op.write(self, val);

        self.additional_cycle &= false;
    }

    fn ror(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap() as u16;
        let carry = self.reg.get_flag(StatusFlag::Carry) as u8;
        let val = (self.reg.a >> 1) | (carry << 7);

        self.reg.set_flag(StatusFlag::Carry, fetched & 0x01 != 0);
        self.reg.set_flag(StatusFlag::Zero, val == 0);
        self.reg.set_flag(StatusFlag::Negative, carry != 0);

        op.write(self, val);

        self.additional_cycle &= false;
    }

    fn rti(&mut self) {
        self.reg.p = self.pop();
        self.reg.set_flag(StatusFlag::Unused, true);
        self.reg.pc = self.pop_word();

        self.additional_cycle &= false;
    }

    fn rts(&mut self) {
        self.reg.pc = self.pop_word() + 1;

        self.additional_cycle &= false;
    }

    fn sbc(&mut self, op: Operand) {
        let fetched = op.read(self).unwrap() as u16;
        let a = self.reg.a as u16;
        let val = a + (!fetched & 0xFF) + self.reg.get_flag(StatusFlag::Carry) as u16;

        let v = (a ^ fetched) & (a ^ val) & 0x80;
        self.reg.set_flag(StatusFlag::Overflow, v != 0);
        self.reg.set_flag(StatusFlag::Carry, (val & 0x100) != 0);
        self.reg.set_flag(StatusFlag::Zero, val & 0xFF == 0);
        self.reg.set_flag(StatusFlag::Negative, val & 0x80 != 0);
        self.reg.a = val as u8;

        self.additional_cycle &= true;
    }

    fn sec(&mut self) {
        self.reg.set_flag(StatusFlag::Carry, true);

        self.additional_cycle &= false;
    }

    fn sed(&mut self) {
        self.reg.set_flag(StatusFlag::Decimal, true);

        self.additional_cycle &= false;
    }

    fn sei(&mut self) {
        self.reg.set_flag(StatusFlag::NoInterrupts, true);

        self.additional_cycle &= false;
    }

    fn sta(&mut self, op: Operand) {
        let val = self.reg.a;
        op.write(self, val);

        self.additional_cycle &= false;
    }

    fn stx(&mut self, op: Operand) {
        let val = self.reg.x;
        op.write(self, val);

        self.additional_cycle &= false;
    }

    fn sty(&mut self, op: Operand) {
        let val = self.reg.y;
        op.write(self, val);

        self.additional_cycle &= false;
    }

    fn tax(&mut self) {
        self.reg.x = self.reg.a;
        self.reg.set_flag(StatusFlag::Zero, self.reg.x == 0);
        self.reg
            .set_flag(StatusFlag::Negative, self.reg.x & 0x80 != 0);

        self.additional_cycle &= false;
    }

    fn tay(&mut self) {
        self.reg.y = self.reg.a;
        self.reg.set_flag(StatusFlag::Zero, self.reg.y == 0);
        self.reg
            .set_flag(StatusFlag::Negative, self.reg.y & 0x80 != 0);

        self.additional_cycle &= false;
    }

    fn tsx(&mut self) {
        self.reg.x = self.reg.sp;
        self.reg.set_flag(StatusFlag::Zero, self.reg.x == 0);
        self.reg
            .set_flag(StatusFlag::Negative, self.reg.x & 0x80 != 0);

        self.additional_cycle &= false;
    }

    fn txa(&mut self) {
        self.reg.a = self.reg.x;
        self.reg.set_flag(StatusFlag::Zero, self.reg.a == 0);
        self.reg
            .set_flag(StatusFlag::Negative, self.reg.a & 0x80 != 0);

        self.additional_cycle &= false;
    }

    fn txs(&mut self) {
        self.reg.sp = self.reg.x;

        self.additional_cycle &= false;
    }

    fn tya(&mut self) {
        self.reg.a = self.reg.y;
        self.reg.set_flag(StatusFlag::Zero, self.reg.a == 0);
        self.reg
            .set_flag(StatusFlag::Negative, self.reg.a & 0x80 != 0);

        self.additional_cycle &= false;
    }
}
