mod instruction;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Registers {
    // 8 bytes registers
    pub a: i8,
    pub b: i8,
    pub c: i8,
    pub d: i8,
    pub e: i8,
    pub f: i8,
    pub h: i8,
    // 16 bytes registers
    pub sp: i16, // Stack Pointer register points to the current stack position
    pub pc: i16, // Program Counter register point to the next instruction to be executed in GB memory
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("Registers state:\n(a: {}) (b: {}) c: {}) (d: {})\n(e: {}) (f: {}) (h: {})\n(sp: {}) (pc: {})",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.sp, self.pc);
    }
}

#[derive(Clone)]
pub struct CPU {
    pub instruction_set: HashMap<u8, instruction::Instruction>,
    pub registers: Registers,
    // TODO handle flags
}

impl CPU {
    pub fn new() -> CPU {

        let mut i_set = HashMap::new();
        i_set.insert(
            0x06,
            instruction::Instruction::new(
                0x06,
                8,
                1,
                "LD B, n",
                |cpu, args| cpu.set_register_value("b", args[0].parse::<i8>().unwrap()),
            ),
        );

        CPU {
            instruction_set: i_set,
            registers: Registers::new(),
        }
    }

    pub fn set_register_value(&mut self, register_name: &str, value: i8) {
        println!("Im modifying registers....");
        match register_name {
            "a" => self.registers.a = value,
            "b" => self.registers.b = value,
            _ => panic!("unknown cpu register name"),
        }
        println!("new value of b -> {}", self.registers.b);
    }

    pub fn to_string(&self) -> String {
        return format!("CPU | {}", self.registers.to_string());
    }

}
