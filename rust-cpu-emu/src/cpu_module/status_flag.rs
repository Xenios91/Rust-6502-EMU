pub mod status_flag {
    use crate::cpu_module::bitfield::bitfield::Bitfield;

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
}
