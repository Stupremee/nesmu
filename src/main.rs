use nesmu::rom::Rom;
use nesmu::cpu::Cpu;
use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("rom.nes")?;
    let mut r = BufReader::new(file);
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;
    let rom = Rom::load(&mut buf);
    let mut cpu = Cpu::new(rom);
    cpu.init();
    loop {
        cpu.run();
    }
    Ok(())
}
