mod instruction;
mod registers;

use std::collections::HashMap;

use self::instruction::Instruction;
// use self::registers::Registers;
use crate::memory::Memory;

#[derive(Clone)]
pub struct CPU {
    pub instruction_set: HashMap<u8, instruction::Instruction>,
    // pub registers: Registers,
    pub memory: Memory,
    // 8 bytes registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    // 16 bytes registers
    pub sp: usize, // Stack Pointer register points to the current stack position
    pub pc: usize, // Program Counter register point to the next instruction to be executed in GB memory

    // TODO handle flags

    // cycles
    cycles: u32,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu: CPU = CPU {
            instruction_set: HashMap::new(),
            // registers: Registers::new(),
            memory: Memory::new(),
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cycles: 0,
        };

        // LD nn,n
        cpu.store_instructions(0x06, 8,1,"LD B, n",|cpu: &mut CPU, _args| cpu.nn_n("b"));
        cpu.store_instructions(0x0E, 8, 1, "LD C, n", |cpu, _args| cpu.nn_n("c"));
        cpu.store_instructions(0x16, 8, 1, "LD D, n", |cpu, _args| cpu.nn_n("d"));
        cpu.store_instructions(0x1E, 8, 1, "LD E, n", |cpu: &mut CPU, _args| cpu.nn_n("e"));
        cpu.store_instructions(0x26, 8, 1, "LD H, n", |cpu, _args| cpu.nn_n("h"));
        cpu.store_instructions(0x2E, 8, 1, "LD L, n", |cpu, _args| cpu.nn_n("l"));
        cpu.store_instructions(0x36, 12, 1, "LD (HL), n", |cpu, _args| cpu.nn_n("hl")); // FIXME implement two bytes operation

        // LD r1,r2
        // A
        cpu.store_instructions(0x7F, 4, 0, "LD A, A", |cpu, _args| cpu.r2_to_r1("a", "a"));
        cpu.store_instructions(0x78, 4, 0, "LD A, B", |cpu, _args| cpu.r2_to_r1("a", "b"));
        cpu.store_instructions(0x79, 4, 0, "LD A, C", |cpu, _args| cpu.r2_to_r1("a", "c"));
        cpu.store_instructions(0x7A, 4, 0, "LD A, D", |cpu, _args| cpu.r2_to_r1("a", "d"));
        cpu.store_instructions(0x7B, 4, 0, "LD A, E", |cpu, _args| cpu.r2_to_r1("a", "e"));
        cpu.store_instructions(0x7C, 4, 0, "LD A, H", |cpu, _args| cpu.r2_to_r1("a", "h"));
        cpu.store_instructions(0x7D, 4, 0, "LD A, L", |cpu, _args| cpu.r2_to_r1("a", "l"));
        cpu.store_instructions(0x7E, 8, 0, "LD A, (HL)", |cpu, _args| cpu.r2_to_r1("a", "hl"));
        // B
        cpu.store_instructions(0x40, 4, 0, "LD B, B", |cpu, _args| cpu.r2_to_r1("b", "b"));
        cpu.store_instructions(0x41, 4, 0, "LD B, C", |cpu, _args| cpu.r2_to_r1("b", "c"));
        cpu.store_instructions(0x42, 4, 0, "LD B, D", |cpu, _args| cpu.r2_to_r1("b", "d"));
        cpu.store_instructions(0x43, 4, 0, "LD B, E", |cpu, _args| cpu.r2_to_r1("b", "e"));
        cpu.store_instructions(0x44, 4, 0, "LD B, H", |cpu, _args| cpu.r2_to_r1("b", "h"));
        cpu.store_instructions(0x45, 4, 0, "LD B, L", |cpu, _args| cpu.r2_to_r1("b", "l"));
        cpu.store_instructions(0x46, 8, 0, "LD B, (HL)", |cpu, _args| cpu.r2_to_r1("b", "hl"));
        // C
        cpu.store_instructions(0x48, 4, 0, "LD C, B", |cpu, _args| cpu.r2_to_r1("c", "b"));
        cpu.store_instructions(0x49, 4, 0, "LD C, C", |cpu, _args| cpu.r2_to_r1("c", "c"));
        cpu.store_instructions(0x4A, 4, 0, "LD C, D", |cpu, _args| cpu.r2_to_r1("c", "d"));
        cpu.store_instructions(0x4B, 4, 0, "LD C, E", |cpu, _args| cpu.r2_to_r1("c", "e"));
        cpu.store_instructions(0x4C, 4, 0, "LD C, H", |cpu, _args| cpu.r2_to_r1("c", "h"));
        cpu.store_instructions(0x4D, 4, 0, "LD C, L", |cpu, _args| cpu.r2_to_r1("c", "l"));
        cpu.store_instructions(0x4E, 8, 0, "LD C, (HL)", |cpu, _args| cpu.r2_to_r1("c", "hl"));
        // D
        cpu.store_instructions(0x50, 4, 0, "LD D, B", |cpu, _args| cpu.r2_to_r1("d", "b"));
        cpu.store_instructions(0x51, 4, 0, "LD D, C", |cpu, _args| cpu.r2_to_r1("d", "c"));
        cpu.store_instructions(0x52, 4, 0, "LD D, D", |cpu, _args| cpu.r2_to_r1("d", "d"));
        cpu.store_instructions(0x53, 4, 0, "LD D, E", |cpu, _args| cpu.r2_to_r1("d", "e"));
        cpu.store_instructions(0x54, 4, 0, "LD D, H", |cpu, _args| cpu.r2_to_r1("d", "h"));
        cpu.store_instructions(0x55, 4, 0, "LD D, L", |cpu, _args| cpu.r2_to_r1("d", "l"));
        cpu.store_instructions(0x56, 8, 0, "LD D, (HL)", |cpu, _args| cpu.r2_to_r1("d", "hl"));
        // E
        cpu.store_instructions(0x58, 4, 0, "LD E, B", |cpu, _args| cpu.r2_to_r1("e", "b"));
        cpu.store_instructions(0x59, 4, 0, "LD E, C", |cpu, _args| cpu.r2_to_r1("e", "c"));
        cpu.store_instructions(0x5A, 4, 0, "LD E, D", |cpu, _args| cpu.r2_to_r1("e", "d"));
        cpu.store_instructions(0x5B, 4, 0, "LD E, E", |cpu, _args| cpu.r2_to_r1("e", "e"));
        cpu.store_instructions(0x5C, 4, 0, "LD E, H", |cpu, _args| cpu.r2_to_r1("e", "h"));
        cpu.store_instructions(0x5D, 4, 0, "LD E, L", |cpu, _args| cpu.r2_to_r1("e", "l"));
        cpu.store_instructions(0x5E, 8, 0, "LD E, (HL)", |cpu, _args| cpu.r2_to_r1("e", "hl"));
        // H
        cpu.store_instructions(0x60, 4, 0, "LD H, B", |cpu, _args| cpu.r2_to_r1("h", "b"));
        cpu.store_instructions(0x61, 4, 0, "LD H, C", |cpu, _args| cpu.r2_to_r1("h", "c"));
        cpu.store_instructions(0x62, 4, 0, "LD H, D", |cpu, _args| cpu.r2_to_r1("h", "d"));
        cpu.store_instructions(0x63, 4, 0, "LD H, E", |cpu, _args| cpu.r2_to_r1("h", "e"));
        cpu.store_instructions(0x64, 4, 0, "LD H, H", |cpu, _args| cpu.r2_to_r1("h", "h"));
        cpu.store_instructions(0x65, 4, 0, "LD H, L", |cpu, _args| cpu.r2_to_r1("h", "l"));
        cpu.store_instructions(0x66, 8, 0, "LD H, (HL)", |cpu, _args| cpu.r2_to_r1("h", "hl"));
        // L
        cpu.store_instructions(0x68, 4, 0, "LD L, B", |cpu, _args| cpu.r2_to_r1("l", "b"));
        cpu.store_instructions(0x69, 4, 0, "LD L, C", |cpu, _args| cpu.r2_to_r1("l", "c"));
        cpu.store_instructions(0x6A, 4, 0, "LD L, D", |cpu, _args| cpu.r2_to_r1("l", "d"));
        cpu.store_instructions(0x6B, 4, 0, "LD L, E", |cpu, _args| cpu.r2_to_r1("l", "e"));
        cpu.store_instructions(0x6C, 4, 0, "LD L, H", |cpu, _args| cpu.r2_to_r1("l", "h"));
        cpu.store_instructions(0x6D, 4, 0, "LD L, L", |cpu, _args| cpu.r2_to_r1("l", "l"));
        cpu.store_instructions(0x6E, 8, 0, "LD L, (HL)", |cpu, _args| cpu.r2_to_r1("l", "hl"));
        // HL
        cpu.store_instructions(0x70, 8, 0, "LD (HL), B", |cpu, _args| cpu.r2_to_r1("hl", "b"));
        cpu.store_instructions(0x71, 8, 0, "LD (HL), C", |cpu, _args| cpu.r2_to_r1("hl", "c"));
        cpu.store_instructions(0x72, 8, 0, "LD (HL), D", |cpu, _args| cpu.r2_to_r1("hl", "d"));
        cpu.store_instructions(0x73, 8, 0, "LD (HL), E", |cpu, _args| cpu.r2_to_r1("hl", "e"));
        cpu.store_instructions(0x74, 8, 0, "LD (HL), H", |cpu, _args| cpu.r2_to_r1("hl", "h"));
        cpu.store_instructions(0x75, 8, 0, "LD (HL), L", |cpu, _args| cpu.r2_to_r1("hl", "l"));

        //LD A, n
        cpu.store_instructions(0x0A, 8, 1, "LD A, (BC)", |cpu, _args| cpu.r2_to_r1("a", "bc"));
        cpu.store_instructions(0x1A, 8, 1, "LD A, (DE)", |cpu, _args| cpu.r2_to_r1("a", "de"));
        cpu.store_instructions(0xFA, 16, 1, "LD A, (nn)", |cpu, _args| cpu.a_nn());
        cpu.store_instructions(0x3E, 8, 1, "LD A, #", |cpu, _args| cpu.r2_to_r1("a", "bc"));

        return cpu;
    }

    pub fn store_instructions(&mut self, op_code: u8, cycles: u8, n_args: u8, _str: &str, execute: fn(&mut CPU, &Vec<u8>)) {
        self.instruction_set.insert(op_code, Instruction::new(
            op_code,
            cycles,
            n_args,
            _str,
            execute,
        ));
    }

    pub fn execute_instruction(&mut self) {
        let op_code = self.memory.read_bytes(self.pc);
        println!("read_operation -> {:#04X?} at {}", op_code, self.pc);
        let inst = self.instruction_set.get(&op_code).unwrap().clone(); // FIXME try to avoid the clone here
        (inst.execute)(self, &[].to_vec());
        // Increment cpu counter here
        // TODO possibly wrong
        self.pc += 1;
        // increment cycles count
        self.cycles += inst.cycles as u32;
    }

    pub fn read_next_opcode(&mut self) -> u8 {
        self.pc += 1;
        let op_code = self.memory.mem_space[self.pc];
        return op_code;
    }

    pub fn get_register_value(&mut self, register_name: &str) -> u8 {
        match register_name {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            "e" => self.e,
            "f" => self.f,
            "h" => self.h,
            "l" => self.l,
            _ => panic!("unknown cpu register name"),
        }
    }

    pub fn set_register_value(&mut self, register_name: &str, value: u8) {
        match register_name {
            "a" => self.a = value,
            "b" => self.b = value,
            "c" => self.c = value,
            "d" => self.d = value,
            "e" => self.e = value,
            "f" => self.f = value,
            "h" => self.h = value,
            "l" => self.l = value,
            _ => panic!("unknown cpu register name"),
        }
    }

    pub fn nn_n(&mut self, register_name: &str) {
        let value = self.read_next_opcode();
        self.set_register_value(register_name, value);
    }

    pub fn a_nn(&mut self) {
        // let value: u = self.read_next_opcode() << 8 | self.read_next_opcode();
        // self.set_register_value("a", value);
        unimplemented!();
    }

    pub fn r2_to_r1(&mut self, r1: &str, r2: &str) {
        let r2_value = self.get_register_value(r2);
        self.set_register_value(r1, r2_value);
    }

    pub fn registers_to_string(&self) -> String {
        return format!("Registers state:\n(a: {}) (b: {}) c: {}) (d: {})\n(e: {}) (f: {}) (h: {}) (l: {})\n(sp: {}) (pc: {})",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.sp, self.pc);
    }

    pub fn to_string(&self) -> String {
        return format!("CPU | {}", self.registers_to_string());
    }
}
