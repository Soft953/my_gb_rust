mod bootrom;
// mod instruction;
mod cpu;
mod memory;

use std::fs;


// will parse the rom file to a list of cpu operation to execute
fn parse_rom(rom: &str) -> Vec<String> {
    let v: Vec<String> = rom.lines().map(str::to_string).collect();
    for line in &v{
        println!("{}", line);
    }

    return v;
}


// fn read_rom_file(file_path: &str) -> &[u8] {
//     let contents = fs::read_to_string(file_path)
//         .expect("Could not read rom file");

//     return contents.as_bytes();
// }

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
    // println!("{}", cpu.to_string());
    // (cpu.instruction_set.get(&0x06).unwrap().execute)(&mut cpu, &vec![0x08]);
    // println!("{}", cpu.to_string());


    
    // Iterate over everything.
    // for (op_code, instruction) in cpu.instruction_set {
    //     println!("{op_code}: \"{}\"", instruction._str);
    // }

    let bytes = fs::read("/home/bourgh_s/my_gb_rust/tests/assets/dmg_boot.bin")
        .expect("Could not read rom file");

    cpu.memory.load_rom(&bytes);

    // println!("{}", cpu.memory.to_string());

    // let bytes = contents.as_bytes();

    // let mut counter = 0;
    loop {
        // if counter > 10 {
        //     break;
        // }
        // println!("{}", bytes[counter]);
        cpu.execute_instruction();
        // &bytes[counter], &[].to_vec());
        println!("{}", cpu.to_string());
        // counter+=1;
    }

}