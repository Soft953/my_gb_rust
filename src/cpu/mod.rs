mod instruction;
mod registers;

use std::collections::HashMap;

use self::instruction::Instruction;
// use self::registers::Registers;
use crate::memory::Memory;

#[derive(Clone)]
pub struct CPU {
    pub instruction_set: HashMap<u8, instruction::Instruction>,
    pub memory: Memory,
    // 8 bits registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    // 16 bits registers -> combine 8 bits
    pub sp: usize, // Stack Pointer register points to the current stack position
    pub pc: usize, // Program Counter register point to the next instruction to be executed in GB memory
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
        cpu.store_instructions(0x06, 8, "LD B, n", |cpu| cpu.nn_n("b"));
        cpu.store_instructions(0x0E, 8, "LD C, n", |cpu| cpu.nn_n("c"));
        cpu.store_instructions(0x16, 8, "LD D, n", |cpu| cpu.nn_n("d"));
        cpu.store_instructions(0x1E, 8, "LD E, n", |cpu| cpu.nn_n("e"));
        cpu.store_instructions(0x26, 8, "LD H, n", |cpu| cpu.nn_n("h"));
        cpu.store_instructions(0x2E, 8, "LD L, n", |cpu| cpu.nn_n("l"));
        cpu.store_instructions(0x36, 12, "LD (HL), n", |cpu| {
            let n = cpu.read_next_opcode();
            cpu.memory
                .write_bytes(cpu.get_register_value_u16("hl") as usize, n);
        });

        // LD r1,r2
        // A
        cpu.store_instructions(0x7F, 4, "LD A, A", |cpu| cpu.r2_to_r1("a", "a"));
        cpu.store_instructions(0x78, 4, "LD A, B", |cpu| cpu.r2_to_r1("a", "b"));
        cpu.store_instructions(0x79, 4, "LD A, C", |cpu| cpu.r2_to_r1("a", "c"));
        cpu.store_instructions(0x7A, 4, "LD A, D", |cpu| cpu.r2_to_r1("a", "d"));
        cpu.store_instructions(0x7B, 4, "LD A, E", |cpu| cpu.r2_to_r1("a", "e"));
        cpu.store_instructions(0x7C, 4, "LD A, H", |cpu| cpu.r2_to_r1("a", "h"));
        cpu.store_instructions(0x7D, 4, "LD A, L", |cpu| cpu.r2_to_r1("a", "l"));
        cpu.store_instructions(0x7E, 8, "LD A, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("a", value);
        });
        // B
        cpu.store_instructions(0x40, 4, "LD B, B", |cpu| cpu.r2_to_r1("b", "b"));
        cpu.store_instructions(0x41, 4, "LD B, C", |cpu| cpu.r2_to_r1("b", "c"));
        cpu.store_instructions(0x42, 4, "LD B, D", |cpu| cpu.r2_to_r1("b", "d"));
        cpu.store_instructions(0x43, 4, "LD B, E", |cpu| cpu.r2_to_r1("b", "e"));
        cpu.store_instructions(0x44, 4, "LD B, H", |cpu| cpu.r2_to_r1("b", "h"));
        cpu.store_instructions(0x45, 4, "LD B, L", |cpu| cpu.r2_to_r1("b", "l"));
        cpu.store_instructions(0x46, 8, "LD B, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("b", value);
        });
        // C
        cpu.store_instructions(0x48, 4, "LD C, B", |cpu| cpu.r2_to_r1("c", "b"));
        cpu.store_instructions(0x49, 4, "LD C, C", |cpu| cpu.r2_to_r1("c", "c"));
        cpu.store_instructions(0x4A, 4, "LD C, D", |cpu| cpu.r2_to_r1("c", "d"));
        cpu.store_instructions(0x4B, 4, "LD C, E", |cpu| cpu.r2_to_r1("c", "e"));
        cpu.store_instructions(0x4C, 4, "LD C, H", |cpu| cpu.r2_to_r1("c", "h"));
        cpu.store_instructions(0x4D, 4, "LD C, L", |cpu| cpu.r2_to_r1("c", "l"));
        cpu.store_instructions(0x4E, 8, "LD C, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("c", value);
        });
        // D
        cpu.store_instructions(0x50, 4, "LD D, B", |cpu| cpu.r2_to_r1("d", "b"));
        cpu.store_instructions(0x51, 4, "LD D, C", |cpu| cpu.r2_to_r1("d", "c"));
        cpu.store_instructions(0x52, 4, "LD D, D", |cpu| cpu.r2_to_r1("d", "d"));
        cpu.store_instructions(0x53, 4, "LD D, E", |cpu| cpu.r2_to_r1("d", "e"));
        cpu.store_instructions(0x54, 4, "LD D, H", |cpu| cpu.r2_to_r1("d", "h"));
        cpu.store_instructions(0x55, 4, "LD D, L", |cpu| cpu.r2_to_r1("d", "l"));
        cpu.store_instructions(0x56, 8, "LD D, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("d", value);
        });
        // E
        cpu.store_instructions(0x58, 4, "LD E, B", |cpu| cpu.r2_to_r1("e", "b"));
        cpu.store_instructions(0x59, 4, "LD E, C", |cpu| cpu.r2_to_r1("e", "c"));
        cpu.store_instructions(0x5A, 4, "LD E, D", |cpu| cpu.r2_to_r1("e", "d"));
        cpu.store_instructions(0x5B, 4, "LD E, E", |cpu| cpu.r2_to_r1("e", "e"));
        cpu.store_instructions(0x5C, 4, "LD E, H", |cpu| cpu.r2_to_r1("e", "h"));
        cpu.store_instructions(0x5D, 4, "LD E, L", |cpu| cpu.r2_to_r1("e", "l"));
        cpu.store_instructions(0x5E, 8, "LD E, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("e", value);
        });
        // H
        cpu.store_instructions(0x60, 4, "LD H, B", |cpu| cpu.r2_to_r1("h", "b"));
        cpu.store_instructions(0x61, 4, "LD H, C", |cpu| cpu.r2_to_r1("h", "c"));
        cpu.store_instructions(0x62, 4, "LD H, D", |cpu| cpu.r2_to_r1("h", "d"));
        cpu.store_instructions(0x63, 4, "LD H, E", |cpu| cpu.r2_to_r1("h", "e"));
        cpu.store_instructions(0x64, 4, "LD H, H", |cpu| cpu.r2_to_r1("h", "h"));
        cpu.store_instructions(0x65, 4, "LD H, L", |cpu| cpu.r2_to_r1("h", "l"));
        cpu.store_instructions(0x66, 8, "LD H, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("h", value);
        });
        // L
        cpu.store_instructions(0x68, 4, "LD L, B", |cpu| cpu.r2_to_r1("l", "b"));
        cpu.store_instructions(0x69, 4, "LD L, C", |cpu| cpu.r2_to_r1("l", "c"));
        cpu.store_instructions(0x6A, 4, "LD L, D", |cpu| cpu.r2_to_r1("l", "d"));
        cpu.store_instructions(0x6B, 4, "LD L, E", |cpu| cpu.r2_to_r1("l", "e"));
        cpu.store_instructions(0x6C, 4, "LD L, H", |cpu| cpu.r2_to_r1("l", "h"));
        cpu.store_instructions(0x6D, 4, "LD L, L", |cpu| cpu.r2_to_r1("l", "l"));
        cpu.store_instructions(0x6E, 8, "LD L, (HL)", |cpu| {
            let hl: u16 = cpu.get_register_value_u16("hl");
            let value = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("l", value);
        });
        // HL
        cpu.store_instructions(0x70, 8, "LD (HL), B", |cpu| {
            let pointer = cpu.get_register_value_u16("hl");
            let b = cpu
                .memory
                .write_bytes(pointer as usize, cpu.get_register_value_u8("b"));
        });
        cpu.store_instructions(0x71, 8, "LD (HL), C", |cpu| {
            let pointer = cpu.get_register_value_u16("hl");
            cpu.memory
                .write_bytes(pointer as usize, cpu.get_register_value_u8("c"));
        });
        cpu.store_instructions(0x72, 8, "LD (HL), D", |cpu| {
            let pointer = cpu.get_register_value_u16("hl");
            cpu.memory
                .write_bytes(pointer as usize, cpu.get_register_value_u8("d"));
        });
        cpu.store_instructions(0x73, 8, "LD (HL), E", |cpu| {
            let pointer = cpu.get_register_value_u16("hl");
            cpu.memory
                .write_bytes(pointer as usize, cpu.get_register_value_u8("e"));
        });
        cpu.store_instructions(0x74, 8, "LD (HL), H", |cpu| {
            let pointer = cpu.get_register_value_u16("hl");
            cpu.memory
                .write_bytes(pointer as usize, cpu.get_register_value_u8("h"));
        });
        cpu.store_instructions(0x75, 8, "LD (HL), L", |cpu| {
            let pointer = cpu.get_register_value_u16("hl");
            cpu.memory
                .write_bytes(pointer as usize, cpu.get_register_value_u8("l"));
        });

        //LD A, n // FIXME do
        cpu.store_instructions(0x0A, 8, "LD A, (BC)", |cpu| cpu.r2_to_r1("a", "bc"));
        cpu.store_instructions(0x1A, 8, "LD A, (DE)", |cpu| cpu.r2_to_r1("a", "de"));
        cpu.store_instructions(0xFA, 16, "LD A, (nn)", |cpu| {
            let first_byte = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            let value = cpu
                .memory
                .read_bytes(u16::from_le_bytes([first_byte, second_byte]) as usize);
            cpu.set_register_value_u8("a", value);
        });
        cpu.store_instructions(0x3E, 8, "LD A, #", |cpu| cpu.r2_to_r1("a", "bc"));

        // LD n, A
        cpu.store_instructions(0x78, 4, "LD B, A", |cpu| cpu.r2_to_r1("b", "a"));
        cpu.store_instructions(0x79, 4, "LD C, A", |cpu| cpu.r2_to_r1("c", "a"));
        cpu.store_instructions(0x7A, 4, "LD D, A", |cpu| cpu.r2_to_r1("d", "a"));
        cpu.store_instructions(0x7B, 4, "LD E, A", |cpu| cpu.r2_to_r1("e", "a"));
        cpu.store_instructions(0x7C, 4, "LD H, A", |cpu| cpu.r2_to_r1("h", "a"));
        cpu.store_instructions(0x7D, 4, "LD L, A", |cpu| cpu.r2_to_r1("l", "a"));
        cpu.store_instructions(0x02, 8, "LD (BC), A", |cpu| cpu.r2_to_r1("bc", "a"));
        cpu.store_instructions(0x12, 8, "LD (DE), A", |cpu| cpu.r2_to_r1("de", "a"));
        cpu.store_instructions(0x77, 8, "LD (HL), A", |cpu| cpu.r2_to_r1("hl", "a"));
        cpu.store_instructions(0xEA, 16, "LD (nn), A", |cpu| {
            let first_byte = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            cpu.memory.write_bytes(
                u16::from_le_bytes([first_byte, second_byte]) as usize,
                cpu.get_register_value_u8("a"),
            );
        });

        // LD A, (C)
        cpu.store_instructions(0xF2, 8, "LD A, (C)", |cpu| cpu.a_c());

        // LD A, (C)
        cpu.store_instructions(0xE2, 8, "LD (C), A", |cpu| {
            let c = cpu.get_register_value_u8("c");
            let a = cpu.get_register_value_u8("a");
            cpu.memory.write_bytes(0xFF00 + c as usize, a);
        });

        // LDD A, (HL)
        cpu.store_instructions(0x3A, 8, "LDD A, (HL)", |cpu| {
            let hl: u8 = cpu.get_register_value_u8("hl");
            let a = cpu.get_register_value_u8("a");
            cpu.memory.write_bytes(hl as usize, a);
            cpu.set_register_value_u8("hl", hl - 1);
        });

        // LDD (HL), A
        cpu.store_instructions(0x32, 8, "LDD (HL), A", |cpu| {
            let hl = cpu.get_register_value_u8("hl");
            let value_at_hl = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("a", value_at_hl);
            cpu.set_register_value_u8("hl", hl - 1);
        });

        // LDI A, (HL)
        cpu.store_instructions(0x2A, 8, "LDI A, (HL)", |cpu| {
            let hl: u8 = cpu.get_register_value_u8("hl");
            let a = cpu.get_register_value_u8("a");
            cpu.memory.write_bytes(hl as usize, a);
            cpu.set_register_value_u8("hl", hl + 1);
        });

        // LDI (HL), A
        cpu.store_instructions(0x22, 8, "LDI (HL), A", |cpu| {
            let hl = cpu.get_register_value_u8("hl");
            let value_at_hl = cpu.memory.read_bytes(hl as usize);
            cpu.set_register_value_u8("a", value_at_hl);
            cpu.set_register_value_u8("hl", hl + 1);
        });

        // LDH (n), A
        cpu.store_instructions(0xE0, 12, "LDH (n), A", |cpu| {
            let a = cpu.get_register_value_u8("a");
            let n = cpu.read_next_opcode();
            cpu.memory.write_bytes(0xFF00 + (n as usize), a);
        });

        // LDH A, (n)
        cpu.store_instructions(0xF0, 12, "LDH A, (n)", |cpu| {
            let n = cpu.read_next_opcode();
            let value = cpu.memory.read_bytes(0xFF00 + (n as usize));
            cpu.set_register_value_u8("a", value);
        });

        // LD n, nn
        cpu.store_instructions(0x01, 12, "LD BC, nn", |cpu| {
            let first_byte = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            cpu.set_register_value_u16("bc", u16::from_le_bytes([first_byte, second_byte]));
        });
        cpu.store_instructions(0x11, 12, "LD DE, nn", |cpu| {
            let first_byte = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            cpu.set_register_value_u16("de", u16::from_le_bytes([first_byte, second_byte]));
        });
        cpu.store_instructions(0x21, 12, "LD HL, nn", |cpu| {
            let first_byte = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            cpu.set_register_value_u16("hl", u16::from_le_bytes([first_byte, second_byte]));
        });
        cpu.store_instructions(0x31, 12, "LD SP, nn", |cpu| {
            let first_byte: u8 = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            cpu.set_register_value_u16("sp", u16::from_le_bytes([first_byte, second_byte]));
        });

        // LD SP, HL
        cpu.store_instructions(0xF9, 8, "LD SP, HL", |cpu| {
            let hl = cpu.get_register_value_u16("hl");
            cpu.set_register_value_u16("sp", hl);
        });

        // LDHL SP, n
        cpu.store_instructions(0xF8, 12, "LDHL SP, n", |cpu| {
            let sn = i16::from_le_bytes([cpu.read_next_opcode(), 0x00]); //signed value
            cpu.set_register_value_u16("hl", (cpu.sp as i16).wrapping_add(sn) as u16);
            // try to avoid any overflow
            unimplemented!(); // FIXME add flag reset
                              // #define CARRY_S(n, n1, n2)     ((((n) < (n1)) | ((n) < (n2))) << 4)
                              // let n1 = cpu.get_register_value_u16("hl").to_le_bytes()[1];
                              // let n2 = cpu.get_register_value_u16("sp").to_le_bytes()[1];
                              // let carry_s: <bool as Shl<i32>>::Output = ((((n1) < (n2)) | ((n1) < (sn))) << 4);
        });

        // LD (nn), SP
        cpu.store_instructions(0x08, 20, "LD (nn), SP", |cpu| {
            let first_byte = cpu.read_next_opcode();
            let second_byte = cpu.read_next_opcode();
            cpu.memory.write_bytes(
                u16::from_le_bytes([first_byte, second_byte]) as usize,
                cpu.get_register_value_u16("sp").to_le_bytes()[1],
            );
        });

        // PUSH nn
        cpu.store_instructions(0xF5, 16, "PUSH AF", |cpu| cpu.push_stack("af"));
        cpu.store_instructions(0xC5, 16, "PUSH BC", |cpu| cpu.push_stack("bc"));
        cpu.store_instructions(0xD5, 16, "PUSH DE", |cpu| cpu.push_stack("de"));
        cpu.store_instructions(0xE5, 16, "PUSH HL", |cpu| cpu.push_stack("hl"));

        // POP nn
        cpu.store_instructions(0xF1, 12, "POP AF", |cpu| cpu.pop_stack("af"));
        cpu.store_instructions(0xC1, 12, "POP BC", |cpu| cpu.pop_stack("bc"));
        cpu.store_instructions(0xD1, 12, "POP DE", |cpu| cpu.pop_stack("de"));
        cpu.store_instructions(0xE1, 12, "POP HL", |cpu| cpu.pop_stack("hl"));

        // ADD A, n
        // cpu.store_instructions(0x87, 4, "ADD A, A", |cpu| cpu.add())

        return cpu;
    }

    pub fn store_instructions(
        &mut self,
        op_code: u8,
        cycles: u8,
        _str: &str,
        execute: fn(&mut CPU),
    ) {
        self.instruction_set
            .insert(op_code, Instruction::new(op_code, cycles, _str, execute));
    }

    pub fn execute_instruction(&mut self) {
        let op_code = self.memory.read_bytes(self.pc);
        println!("read_operation -> {:#04X?} at {}", op_code, self.pc);
        let inst = self.instruction_set.get(&op_code).unwrap().clone(); // FIXME try to avoid the clone here
        (inst.execute)(self);
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

    pub fn get_register_value_u8(&self, register_name: &str) -> u8 {
        match register_name {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            "e" => self.e,
            "f" => self.f,
            "h" => self.h,
            "l" => self.l,
            _ => panic!("unknown cpu u8 register name"),
        }
    }

    pub fn get_register_value_u16(&self, register_name: &str) -> u16 {
        match register_name {
            "af" => u16::from_le_bytes([
                self.get_register_value_u8("a"),
                self.get_register_value_u8("f"),
            ]),
            "bc" => u16::from_le_bytes([
                self.get_register_value_u8("b"),
                self.get_register_value_u8("c"),
            ]),
            "de" => u16::from_le_bytes([
                self.get_register_value_u8("d"),
                self.get_register_value_u8("e"),
            ]),
            "hl" => u16::from_le_bytes([
                self.get_register_value_u8("h"),
                self.get_register_value_u8("l"),
            ]),
            _ => panic!("unknown  cpu u16 register name"),
        }
    }

    pub fn set_register_value_u8(&mut self, register_name: &str, value: u8) {
        match register_name {
            "a" => self.a = value,
            "b" => self.b = value,
            "c" => self.c = value,
            "d" => self.d = value,
            "e" => self.e = value,
            "f" => self.f = value,
            "h" => self.h = value,
            "l" => self.l = value,
            _ => panic!("unknown cpu u8 register name"),
        }
    }

    pub fn set_register_value_u16(&mut self, register_name: &str, value: u16) {
        let bytes: [u8; 2] = u16::to_le_bytes(value);
        match register_name {
            "af" => {
                self.set_register_value_u8("a", bytes[0]);
                self.set_register_value_u8("f", bytes[1]);
            }
            "bc" => {
                self.set_register_value_u8("b", bytes[0]);
                self.set_register_value_u8("c", bytes[1]);
            }
            "de" => {
                self.set_register_value_u8("d", bytes[0]);
                self.set_register_value_u8("e", bytes[1]);
            }
            "hl" => {
                self.set_register_value_u8("h", bytes[0]);
                self.set_register_value_u8("l", bytes[1]);
            }
            "sp" => self.sp = value as usize,
            _ => panic!("unknown cpu u16 register name"),
        }
    }

    pub fn nn_n(&mut self, register_name: &str) {
        let value = self.read_next_opcode();
        self.set_register_value_u8(register_name, value);
    }

    pub fn a_c(&mut self) {
        let c = self.get_register_value_u8("c");
        self.set_register_value_u8("a", self.memory.read_bytes(0xFF00 + c as usize));
    }

    pub fn r2_to_r1(&mut self, r1: &str, r2: &str) {
        let r2_value = self.get_register_value_u8(r2);
        self.set_register_value_u8(r1, r2_value);
    }

    pub fn push_stack(&mut self, register_name: &str) {
        let value = self.get_register_value_u16(register_name);
        self.sp -= 1;
        self.memory.write_bytes(self.sp, value.to_le_bytes()[0]);
        self.sp -= 1;
        self.memory.write_bytes(self.sp, value.to_le_bytes()[1]);
    }

    pub fn pop_stack(&mut self, register_name: &str) {
        let value = self.get_register_value_u16(register_name);
        self.sp += 1;
        self.memory.write_bytes(self.sp, value.to_le_bytes()[0]);
        self.sp += 1;
        self.memory.write_bytes(self.sp, value.to_le_bytes()[1]);
    }

    pub fn registers_to_string(&self) -> String {
        return format!("Registers state:\n(a: {}) (b: {}) c: {}) (d: {})\n(e: {}) (f: {}) (h: {}) (l: {})\n(sp: {}) (pc: {})",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.sp, self.pc);
    }

    pub fn to_string(&self) -> String {
        return format!("CPU | {}", self.registers_to_string());
    }
}
