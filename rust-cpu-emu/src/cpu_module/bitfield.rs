pub mod bitfield {
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
