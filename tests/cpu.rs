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
    Ok(buf)
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
    let pc = line[0..4].parse::<u16>()?;
    let a = line[50..=51].parse::<u8>()?;
    let x = line[55..=56].parse::<u8>()?;
    let y = line[60..=61].parse::<u8>()?;
    let p = line[65..=66].parse::<u8>()?;
    let sp = line[71..=72].parse::<u8>()?;
    let cycle = line[90..line.len()].parse::<u32>()?;
    Ok((cycle, Registers { pc, a, x, y, p, sp }))
}

#[test]
fn nestest() {
    let rom = load_nestest().expect("Failed to read nestest file");
    let log = load_nestest_log().expect("Failed to read nestest log file");
    let mut offset = 0x8000;
    let mut ram = [0u8; 0x10000];

    for b in rom.iter() {
        ram[offset] = *b;
        offset += 1;
    }

    let bus = Bus::from_ram(ram);
    let mut cpu = Cpu::new(bus, Default::default());
    cpu.reg.pc = 0xC000;

    while let Some(line) = log.iter().next() {
        let (cycles, reg) = parse_log_line(line.to_string()).expect("failed to parse log line");

        assert_eq!(reg, cpu.reg);
        assert_eq!(cycles, cpu.cycle_count);

        cpu.clock();
    }
}
