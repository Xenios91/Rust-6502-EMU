pub mod memory {

    pub struct Memory {
        max_memory: usize,
        data: [usize; 1024 * 64],
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

        pub fn get_total_memory(&self) -> usize {
            self.max_memory
        }
    }
}
