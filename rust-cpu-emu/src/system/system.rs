pub mod system {
    use super::super::cpu_module::cpu_6502::CPU6502;
    use super::super::cpu_module::cpu_builder::build_new_cpu;
    use super::super::memory::memory::memory::Memory;
    use super::super::memory::memory_builder::memory_builder::build_new_memory;

    pub struct System {
        pub cpu: CPU6502,
        pub memory: Memory,
    }

    impl System {
        pub fn new() -> Self {
            System {
                cpu: build_new_cpu(),
                memory: build_new_memory(),
            }
        }

        pub fn reset(&mut self) {
            self.cpu.reset();
            self.memory.reset();
        }
    }
}
