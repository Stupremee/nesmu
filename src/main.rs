use nesmu::rom::Rom;
use nesmu::cpu::Cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::from_file("rom.nes".to_string())?;
    let mut cpu = Cpu::new(rom);
    while !cpu.finished() {
        cpu.run();
    }
    Ok(())
}
