use crate::cpu_module::status_flag::status_flag::StatusFlag;
use std::fmt;

struct Registers {
    a: u8,
    x: u8,
    y: u8,
}

struct StatusFlags {
    c: StatusFlag,
    z: StatusFlag,
    i: StatusFlag,
    d: StatusFlag,
    b: StatusFlag,
    v: StatusFlag,
    n: StatusFlag,
}

pub struct CPU6502 {
    pc: u8,                    // program counter
    sp: u16,                   // stack pointer
    registers: Registers,      // registers
    status_flags: StatusFlags, // cpu status flags
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
            pc: 0,
            sp: 0,
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

    pub fn get_status_flag_value(&self, register: char) -> bool {
        match register {
            'c' => !self.status_flags.c.bitfield.is_zeroed(),
            'z' => !self.status_flags.z.bitfield.is_zeroed(),
            'i' => !self.status_flags.i.bitfield.is_zeroed(),
            'd' => !self.status_flags.d.bitfield.is_zeroed(),
            'b' => !self.status_flags.b.bitfield.is_zeroed(),
            'v' => !self.status_flags.v.bitfield.is_zeroed(),
            'n' => !self.status_flags.n.bitfield.is_zeroed(),
            _ => panic!("Invalid status flag requested!"),
        }
    }

    pub fn get_stack_pointer(&self) -> u16 {
        self.sp
    }

    pub fn get_program_counter(&self) -> u8 {
        self.pc
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

    pub fn set_status_flag_value(&mut self, register: char, setflag: bool) {
        match register {
            'c' => self
                .status_flags
                .c
                .bitfield
                .flip_bit(setflag, self.status_flags.c.placeholder),
            'z' => self
                .status_flags
                .z
                .bitfield
                .flip_bit(setflag, self.status_flags.z.placeholder),
            'i' => self
                .status_flags
                .i
                .bitfield
                .flip_bit(setflag, self.status_flags.i.placeholder),
            'd' => self
                .status_flags
                .d
                .bitfield
                .flip_bit(setflag, self.status_flags.d.placeholder),
            'b' => self
                .status_flags
                .b
                .bitfield
                .flip_bit(setflag, self.status_flags.b.placeholder),
            'v' => self
                .status_flags
                .v
                .bitfield
                .flip_bit(setflag, self.status_flags.v.placeholder),
            'n' => self
                .status_flags
                .n
                .bitfield
                .flip_bit(setflag, self.status_flags.n.placeholder),
            _ => panic!("Invalid status flag requested!"),
        }
    }
}
