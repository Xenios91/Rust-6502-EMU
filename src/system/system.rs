pub mod system {
    use crate::system::cpu_module::cpu_6502::CPU6502;
    use crate::system::cpu_module::cpu_builder::build_new_cpu;
    use crate::system::cpu_module::instructions::cpu_6502_instructions;
    use crate::system::memory::memory::memory::Memory;
    use crate::system::memory::memory_builder::memory_builder::build_new_memory;

    pub struct System {
        pub cpu: CPU6502,
        pub memory: Memory,
    }

    impl System {
        pub fn new() -> Self {
            let mut system: System = System {
                cpu: build_new_cpu(),
                memory: build_new_memory(),
            };
            system.reset();
            system
        }

        pub fn reset(&mut self) {
            self.cpu.reset();
            self.memory.reset();
        }

        pub fn execute(&mut self, cpu_cycles: u32) {
            let mut cycles = cpu_cycles;
            while cycles > 0 {
                let byte_instruction: u8 =
                    cpu_6502_instructions::fetch_byte(&mut self.cpu, &self.memory, &mut cycles);
                cpu_6502_instructions::run_instruction(
                    &mut self.cpu,
                    &self.memory,
                    byte_instruction,
                    &mut cycles,
                );
            }
        }
    }
}
