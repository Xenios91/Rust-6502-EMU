pub struct Memory {
    max_memory: u64,
    program_instructions: Vec<u8>,
    stack_size: usize,
    stack_space: Vec<usize>,
    heap_size: usize,
    heap_space: Vec<usize>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            max_memory: 65535,
            program_instructions: Vec::<u8>::with_capacity(256),
            stack_size: 12000,
            stack_space: Vec::<usize>::with_capacity(12000),
            heap_size: 32000,
            heap_space: Vec::<usize>::with_capacity(32000),
        }
    }

    fn initialize(&mut self) {
        self.max_memory = 65535;
    }

    pub fn reset(&mut self) {
        self.initialize();
    }

    pub fn get_total_memory(&self) -> u64 {
        self.max_memory
    }

    pub fn increase_heap(&mut self) {
        let increase_size: usize = (self.heap_size as f64 * 0.05) as usize;
        self.heap_size += increase_size;
        self.heap_space.resize_with(increase_size, Default::default);
    }

    pub fn decrease_heap(&mut self) {
        let decrease_size: usize = (self.heap_size as f64 * 0.05) as usize;
        self.heap_size -= decrease_size;
        self.heap_space.resize_with(decrease_size, Default::default);
    }

    pub fn retrieve_memory(&self, address: usize) -> u8 {
        self.program_instructions[address]
    }

    pub fn set_memory_value(&mut self, address: usize, memory_value: u8) {
        self.program_instructions[address] = memory_value;
    }
}
