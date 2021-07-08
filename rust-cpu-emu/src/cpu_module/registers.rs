pub mod registers {

    pub struct Registers {
        pub a: u8,
        pub x: u8,
        pub y: u8,
    }

    impl Registers {
        pub fn set_register_value(&mut self, register: char, value: u8) {
            match register {
                'a' => self.a = value,
                'x' => self.x = value,
                'y' => self.y = value,
                _ => panic!("Invalid register requested!"),
            }
        }

        pub fn set_many_registers_value(&mut self, registers: &[char], value: u8) {
            for register in registers {
                self.set_register_value(*register, value);
            }
        }
    }

    mod bitfield {
        use std::fmt;

        pub struct Bitfield {
            pub field: u8,
        }
        impl fmt::Display for Bitfield {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "(Bitfield: {})", self.field)
            }
        }
        impl Bitfield {
            pub fn new() -> Self {
                Self { field: 0b00000000 }
            }
            pub fn flip_bit(&mut self, setflag: bool, placeholder: u8) {
                if setflag {
                    self.field = placeholder;
                } else {
                    self.field = self.field ^ self.field;
                }
            }
            pub fn is_zeroed(&self) -> bool {
                self.field == 0
            }
        }
    }

    pub struct StatusFlag {
        pub name: char,
        pub placeholder: u8,
        pub bitfield: bitfield::Bitfield,
    }

    impl StatusFlag {
        pub fn new(name: char, placeholder: u8) -> Self {
            Self {
                name: name,
                placeholder: placeholder,
                bitfield: bitfield::Bitfield::new(),
            }
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
        pub fn set_status_flag_value(&mut self, flag: char, setflag: bool) {
            match flag {
                'c' => self.c.bitfield.flip_bit(setflag, self.c.placeholder),
                'z' => self.z.bitfield.flip_bit(setflag, self.z.placeholder),
                'i' => self.i.bitfield.flip_bit(setflag, self.i.placeholder),
                'd' => self.d.bitfield.flip_bit(setflag, self.d.placeholder),
                'b' => self.b.bitfield.flip_bit(setflag, self.b.placeholder),
                'v' => self.v.bitfield.flip_bit(setflag, self.v.placeholder),
                'n' => self.n.bitfield.flip_bit(setflag, self.n.placeholder),
                _ => panic!("Invalid status flag requested!"),
            }
        }

        pub fn clear_status_flags(&mut self, flags: &[char]) {
            for flag in flags {
                self.set_status_flag_value(*flag, false);
            }
        }

        pub fn set_many_status_flags(&mut self, flags: &[char], setflag: bool) {
            for flag in flags {
                self.set_status_flag_value(*flag, setflag);
            }
        }

        pub fn get_status_flag_value(&self, register: char) -> bool {
            match register {
                'c' => !self.c.bitfield.is_zeroed(),
                'z' => !self.z.bitfield.is_zeroed(),
                'i' => !self.i.bitfield.is_zeroed(),
                'd' => !self.d.bitfield.is_zeroed(),
                'b' => !self.b.bitfield.is_zeroed(),
                'v' => !self.v.bitfield.is_zeroed(),
                'n' => !self.n.bitfield.is_zeroed(),
                _ => panic!("Invalid status flag requested!"),
            }
        }
    }
}
