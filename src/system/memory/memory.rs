pub struct Memory {
    max_memory: u32,
    data: [u8; 1024 * 64],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            max_memory: 1024 * 64,
            data: [0; 1024 * 64],
        }
    }

    fn initialize(&mut self) {
        self.max_memory = 1024 * 64;
        self.data = [0; 1024 * 64];
    }

    pub fn reset(&mut self) {
        self.initialize();
    }

    pub fn get_total_memory(&self) -> u32 {
        self.max_memory
    }

    pub fn retrieve_memory(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn set_memory_value(&mut self, address: usize, memory_value: u8) {
        self.data[address] = memory_value;
    }
}
