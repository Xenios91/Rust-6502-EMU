pub mod registers {
    use crate::cpu_module::bitfield::bitfield::Bitfield;

    pub struct Registers {
        pub a: u8,
        pub x: u8,
        pub y: u8,
    }

    pub struct StatusFlag {
        pub name: char,
        pub placeholder: u8,
        pub bitfield: Bitfield,
    }

    impl StatusFlag {
        pub fn new(name: char, placeholder: u8) -> Self {
            Self {
                name: name,
                placeholder: placeholder,
                bitfield: Bitfield::new(),
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
}
