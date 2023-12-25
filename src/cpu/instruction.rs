use crate::cpu::CPU;

// pub type ExecuteFn = dyn Fn(Instruction, CPU, Vec<String>) -> i32;

#[derive(Clone)]
pub struct Instruction {
    pub op_code: u8,
    pub cycles: u8,
    pub _str: String,
    pub execute: fn(&mut CPU),
}

impl Instruction {
    pub fn new(op_code: u8, cycles: u8, _str: &str, execute: fn(&mut CPU)) -> Instruction {
        Instruction {
            op_code: op_code,
            cycles: cycles,
            _str: _str.to_string(),
            execute: execute,
        }
    }
}

