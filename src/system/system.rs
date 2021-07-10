use crate::system::cpu_module::cpu_6502::CPU6502;
use crate::system::cpu_module::instructions;
use crate::system::memory::memory::Memory;

pub struct System {
    pub cpu: CPU6502,
    pub memory: Memory,
}

impl System {
    pub fn new() -> Self {
        let mut system: System = System {
            cpu: CPU6502::new(),
            memory: Memory::new(),
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
                instructions::fetch_byte(&mut self.cpu, &self.memory, &mut cycles);
            instructions::run_instruction(
                &mut self.cpu,
                &self.memory,
                byte_instruction,
                &mut cycles,
            );
        }
    }
}
