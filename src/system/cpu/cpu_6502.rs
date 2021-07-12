use crate::system::cpu::registers::Registers;
use crate::system::cpu::registers::StatusFlags;

pub struct CPU6502 {
    pub program_counter: u16,
    pub stack_pointer: u8,
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
        Self {
            program_counter: 0,
            stack_pointer: 0,
            registers: Registers::new(),
            status_flags: StatusFlags::new(),
            cpu_arch: String::from("6502"),
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = 0xfffc;
        self.stack_pointer = 0x10;
        let registers_to_zero: [char; 3] = ['a', 'x', 'y'];
        self.registers
            .set_many_registers_value(&registers_to_zero, 0);
        let flags_to_clear: [char; 7] = ['c', 'z', 'i', 'd', 'b', 'v', 'n'];
        self.status_flags.clear_status_flags(&flags_to_clear);
    }

    pub fn lda_set_status_flags(&mut self) {
        self.status_flags
            .set_status_flag_value('z', self.registers.a == 0);
        self.status_flags
            .set_status_flag_value('n', self.registers.a & 0b10000000 > 0);
    }
}
