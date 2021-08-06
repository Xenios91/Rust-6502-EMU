pub struct Memory {
    max_memory: u16,
    data: [u8; 65535],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            max_memory: 65535,
            data: [0; 65535],
        }
    }

    fn initialize(&mut self) {
        self.max_memory = 65535;
        self.data = [0; 65535];
    }

    pub fn reset(&mut self) {
        self.initialize();
    }

    pub fn get_total_memory(&self) -> u16 {
        self.max_memory
    }

    pub fn retrieve_memory(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn set_memory_value(&mut self, address: usize, memory_value: u8) {
        self.data[address] = memory_value;
    }
}
