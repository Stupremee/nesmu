use nesmu::{
    bus::Bus,
    cpu::{Cpu, Registers},
};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ROM_PATH: &str = "./tests/roms/nestest.nes";
const LOG_PATH: &str = "./tests/roms/nestest.log";

fn load_nestest() -> Result<Vec<u8>, io::Error> {
    let file = File::open(ROM_PATH)?;
    let mut read = BufReader::new(file);
    let mut buf = Vec::new();
    read.read_to_end(&mut buf)?;

    let prg_size = buf[5] as usize * 16384;
    let prg = &buf[16..=prg_size];
    Ok(prg.to_vec())
}

fn load_nestest_log() -> Result<Vec<String>, io::Error> {
    let file = File::open(LOG_PATH)?;
    let read = BufReader::new(file);
    Ok(read
        .lines()
        .map(|x| x.expect("failed to read nestest file"))
        .collect())
}

fn parse_log_line(line: String) -> Result<(u32, Registers), Box<dyn std::error::Error>> {
    let pc = u16::from_str_radix(&line[0..4], 16)?;
    let a = u8::from_str_radix(&line[50..=51], 16)?;
    let x = u8::from_str_radix(&line[55..=56], 16)?;
    let y = u8::from_str_radix(&line[60..=61], 16)?;
    let p = u8::from_str_radix(&line[65..=66], 16)?;
    let sp = u8::from_str_radix(&line[71..=72], 16)?;
    let cycle = u32::from_str_radix(&line[90..line.len()], 16)?;
    Ok((cycle, Registers { pc, a, x, y, p, sp }))
}

#[test]
fn nestest() {
    let rom = load_nestest().expect("Failed to read nestest file");
    let log = load_nestest_log().expect("Failed to read nestest log file");
    let mut offset = 0xC000;
    let mut ram = [0u8; 0x10000];

    for b in rom.iter() {
        ram[offset] = *b;
        offset += 1;
    }

    let bus = Bus::from_ram(ram);
    let mut cpu = Cpu::new(bus, Default::default());
    cpu.reset();
    cpu.reg.pc = 0xC000;
    cpu.reg.p = 36;

    let mut log = log.iter();
    while let Some(line) = log.next() {
        println!("Processing line {}", line);
        let cpu_reg = cpu.reg.clone();
        let cpu_cycles = cpu.cycle_count.clone();
        cpu.execute_instruction();
        let (cycles, reg) = parse_log_line(line.to_string()).expect("failed to parse log line");

        assert_eq!(reg, cpu_reg);
        assert_eq!(cycles, cpu_cycles);
    }
}
