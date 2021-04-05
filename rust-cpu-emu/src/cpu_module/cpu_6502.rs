use std::fmt;

struct Registers {
    a: u8,
    x: u8,
    y: u8,
}

pub struct CPU6502 {
    pc: u8,               // program counter
    sp: u16,              //stack pointer
    registers: Registers, //registers
    cpu_arch: String,
}

impl fmt::Display for CPU6502 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(CPU Type: {})", self.cpu_arch)
    }
}

impl CPU6502 {
    pub fn new() -> Self {
        let registers = Registers { a: 0, x: 0, y: 0 };
        Self {
            pc: 0,
            sp: 0,
            registers,
            cpu_arch: "6502".to_string(),
        }
    }

    pub fn get_register_value(&self, register: char) -> u8 {
        match register {
            'a' => return self.registers.a,
            'x' => return self.registers.x,
            'y' => return self.registers.y,
            _ => panic!("Invalid register requested!"),
        }
    }

    pub fn get_stack_pointer(&self) -> u16 {
        return self.sp;
    }

    pub fn get_program_counter(&self) -> u8 {
        return self.pc;
    }

    pub fn set_register_value(&mut self, register: char, value: u8) {
        match register {
            'a' => self.registers.a = value,
            'x' => self.registers.x = value,
            'y' => self.registers.y = value,
            _ => panic!("Invalid register requested!"),
        }
    }

    pub fn set_stack_pointer(&mut self, value: u16) {
        self.sp = value;
    }

    pub fn set_program_counter(&mut self, value: u8) {
        self.pc = value;
    }
}
