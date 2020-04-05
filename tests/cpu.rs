use nesmu::{bus::Bus, cpu::Cpu};

#[test]
fn test_some_instructions() {
    let input = &[0xA9, 0xFF, 0x00];
    let mut offset = 0x8000;
    let mut ram = [0u8; 0x10000];

    for b in input.iter() {
        ram[offset] = *b;
        offset += 1;
    }

    ram[0xFFFC] = 0x00;
    ram[0xFFFD] = 0x80;

    let bus = Bus::from_ram(ram);
    let cpu = &mut Cpu::new(bus, Default::default());

    cpu.reset();
    execute_instruction(cpu);
    execute_instruction(cpu);
    assert_eq!(cpu.reg.a, 0xFF);
}

fn execute_instruction(cpu: &mut Cpu) {
    loop {
        cpu.clock();
        if cpu.complete() {
            break;
        }
    }
}
