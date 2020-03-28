use nesmu::rom::Rom;
use nesmu::cpu::Cpu;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("rom.nes")?;
    let rom = Rom::load(&mut file)?;
    let mut cpu = Cpu::new(rom);
    while !cpu.finished() {
        cpu.run();
    }
    Ok(())
}
