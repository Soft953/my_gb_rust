mod bootrom;
// mod instruction;
mod cpu;


// will parse the rom file to a list of cpu operation to execute
fn parse_rom(rom: &str) -> Vec<String> {
    let v: Vec<String> = rom.lines().map(str::to_string).collect();
    for line in &v{
        println!("{}", line);
    }

    return v;
}

fn main() {
    // basic emulator loop
    // while (¡stop_emulation)
    // {
    // executeCPU(cycles_to_execute);
    // generateInterrupts();
    // emulateGraphics();
    // emulateSound();
    // emulateOtherSoftware();
    // timeSincronization();
    // }
    // println!("{}", bootrom::BOOT_ROM_INSTRUCTIONS);
    // parse_rom(bootrom::BOOT_ROM_INSTRUCTIONS);

    // let i = instruction::Instruction::new(0x06, 8, 1, "LD B, n");
    // println!("{}", i._str);

    let mut cpu = cpu::CPU::new();
    println!("{}", cpu.to_string());
    (cpu.instruction_set.get(&0x06).unwrap().execute)(&mut cpu, vec!["08".to_string()]);
    println!("{}", cpu.to_string());


    
    // Iterate over everything.
    for (op_code, instruction) in cpu.instruction_set {
        println!("{op_code}: \"{}\"", instruction._str);
    }
}