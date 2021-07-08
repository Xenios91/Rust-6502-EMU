use crate::cpu_module::registers::registers::Registers;
use crate::cpu_module::registers::registers::StatusFlag;
use crate::cpu_module::registers::registers::StatusFlags;

pub struct CPU6502 {
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub registers: Registers,
    pub status_flags: StatusFlags,
    pub cpu_arch: String,
}

impl std::fmt::Display for CPU6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(CPU Type: {})", self.cpu_arch)
    }
}

impl CPU6502 {
    pub fn new() -> Self {
        let registers = Registers { a: 0, x: 0, y: 0 };
        let status_flags = StatusFlags {
            c: StatusFlag::new('c', 1),
            z: StatusFlag::new('z', 2),
            i: StatusFlag::new('i', 4),
            d: StatusFlag::new('d', 8),
            b: StatusFlag::new('b', 16),
            v: StatusFlag::new('v', 32),
            n: StatusFlag::new('n', 64),
        };
        Self {
            program_counter: 0,
            stack_pointer: 0,
            registers: registers,
            status_flags: status_flags,
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

    pub fn reset(&mut self) {
        self.program_counter = 0xfffc;
        self.stack_pointer = 0x100;
        let registers_to_zero: [char; 3] = ['a', 'x', 'y'];
        self.registers
            .set_many_registers_value(&registers_to_zero, 0);
        let flags_to_clear: [char; 7] = ['c', 'z', 'i', 'd', 'b', 'v', 'n'];
        self.status_flags.clear_status_flags(&flags_to_clear);
    }
}
