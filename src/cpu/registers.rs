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
    pub l: i8,
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
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_register_value(&self, register_name: &str) -> i8 {
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

    pub fn set_register_value(&mut self, register_name: &str, value: i8) {
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

    pub fn r2_to_r1(&mut self, r1: &str, r2: &str) {
        let r2_value = self.get_register_value(r2);
        self.set_register_value(r1, r2_value);
    }

    pub fn to_string(&self) -> String {
        return format!("Registers state:\n(a: {}) (b: {}) c: {}) (d: {})\n(e: {}) (f: {}) (h: {}) (l: {})\n(sp: {}) (pc: {})",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.sp, self.pc);
    }
}
