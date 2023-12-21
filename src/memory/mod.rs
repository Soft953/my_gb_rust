use std::cmp::min;

const MEMORY_SIZE: usize = 0xFFFF;
const ROM: usize = 0x0000;
const VRAM: usize = 0x8000;
const EXTERNAL_RAM: usize = 0xA000;
const RAM: usize = 0xC000;
const OAM_RAM: usize = 0xFE00;
const IO: usize = 0xFF00;
const HRAM: usize = 0xFF80;

#[derive(Clone)]
pub struct Memory {
    pub mem_space: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {

        let mem_space = [0; MEMORY_SIZE];

        return Memory {
            mem_space: mem_space,
        }
    }

    pub fn read_bytes(&self, pointer: usize) -> u8 {
        return self.mem_space[pointer];
    }

    pub fn load_rom(&mut self, bytes: &Vec<u8>) {
        if bytes.len() > VRAM {
            println!("Weird, ROM does not fit in memory, size > 8Kb")
        }

        for i in 0..min(bytes.len(), VRAM) {
            self.mem_space[i] = bytes[i];
        }
    }

    pub fn to_string(&self) -> String {
        return format!("Memory | {:x?}", self.mem_space);
    }
}