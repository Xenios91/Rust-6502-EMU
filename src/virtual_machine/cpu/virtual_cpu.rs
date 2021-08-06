pub mod cpu {
    use super::registers::*;

    pub struct CPU {
        pub program_counter: u16,
        pub stack_pointer: u8,
        pub registers: Registers,
        pub status_flags: StatusFlags,
        pub cpu_arch: String,
    }

    impl std::fmt::Display for CPU {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "(CPU Type: {})", self.cpu_arch)
        }
    }

    impl CPU {
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
}

mod registers {
    pub struct Registers {
        pub a: u8,
        pub x: u8,
        pub y: u8,
    }

    impl Registers {
        pub fn new() -> Self {
            Registers { a: 0, x: 0, y: 0 }
        }

        pub fn set_register_value(&mut self, register: char, value: u8) {
            match register {
                'a' => self.a = value,
                'x' => self.x = value,
                'y' => self.y = value,
                _ => panic!("Invalid register requested!"),
            }
        }

        pub fn set_many_registers_value(&mut self, registers: &[char], register_value: u8) {
            for register in registers {
                self.set_register_value(*register, register_value);
            }
        }

        pub fn get_register_value(&self, register: char) -> u8 {
            match register {
                'a' => self.a,
                'x' => self.x,
                'y' => self.y,
                _ => panic!("Invalid register requested!"),
            }
        }
    }

    pub struct StatusFlag {
        pub name: char,
        pub placeholder: u8,
        pub bitfield: u8,
    }

    impl StatusFlag {
        pub fn new(name: char, placeholder: u8) -> Self {
            Self {
                name,
                placeholder,
                bitfield: 0b00000000,
            }
        }

        pub fn set_status_flag(&mut self, setflag: bool) {
            if setflag {
                self.bitfield = self.placeholder;
            } else {
                self.bitfield ^= self.placeholder;
            }
        }

        pub fn is_zeroed(&self) -> bool {
            self.bitfield == 0
        }
    }

    pub struct StatusFlags {
        pub c: StatusFlag,
        pub z: StatusFlag,
        pub i: StatusFlag,
        pub d: StatusFlag,
        pub b: StatusFlag,
        pub v: StatusFlag,
        pub n: StatusFlag,
    }

    impl StatusFlags {
        pub fn new() -> Self {
            StatusFlags {
                c: StatusFlag::new('c', 1),
                z: StatusFlag::new('z', 2),
                i: StatusFlag::new('i', 4),
                d: StatusFlag::new('d', 8),
                b: StatusFlag::new('b', 16),
                v: StatusFlag::new('v', 64),
                n: StatusFlag::new('n', 128),
            }
        }

        pub fn set_status_flag_value(&mut self, status_flag: char, setflag: bool) {
            match status_flag {
                'c' => self.c.set_status_flag(setflag),
                'z' => self.z.set_status_flag(setflag),
                'i' => self.i.set_status_flag(setflag),
                'd' => self.d.set_status_flag(setflag),
                'b' => self.b.set_status_flag(setflag),
                'v' => self.v.set_status_flag(setflag),
                'n' => self.n.set_status_flag(setflag),
                _ => panic!("Invalid status flag requested!"),
            }
        }

        pub fn clear_status_flags(&mut self, status_flags: &[char]) {
            for status_flag in status_flags {
                self.set_status_flag_value(*status_flag, false);
            }
        }

        pub fn set_status_flags_value(&mut self, status_flags: &[char], setflag: bool) {
            for status_flag in status_flags {
                self.set_status_flag_value(*status_flag, setflag);
            }
        }

        pub fn get_status_flag_value(&self, status_flag: char) -> bool {
            match status_flag {
                'c' => !self.c.is_zeroed(),
                'z' => !self.z.is_zeroed(),
                'i' => !self.i.is_zeroed(),
                'd' => !self.d.is_zeroed(),
                'b' => !self.b.is_zeroed(),
                'v' => !self.v.is_zeroed(),
                'n' => !self.n.is_zeroed(),
                _ => panic!("Invalid status flag requested!"),
            }
        }
    }
}
