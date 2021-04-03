use std::fmt;

struct Registers {
    a: u8,
    x: u8,
    y: u8,
}

pub struct CPU {
    pc: u8,               // program counter
    sp: u16,              //stack pointer
    registers: Registers, //registers
    cpu_arch: String,
}

fn create_new_registers() -> Registers {
    let registers = Registers { a: 0, x: 0, y: 0 };
    return registers;
}

pub fn create_new_cpu() -> CPU {
    let registers = create_new_registers();
    let cpu = CPU {
        pc: 0,
        sp: 0,
        registers,
        cpu_arch: "6502".to_string(),
    };
    return cpu;
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(CPU Type: {})", self.cpu_arch)
    }
}
