use crate::system::bus::bus_operations::bus_read;
use crate::system::cpu::cpu_6502::CPU6502;
use crate::system::cpu::instructions::instructions;
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

    pub fn execute(&mut self) {
        let mut cycles = 0;
        loop {
            if cycles == 0 {
                let opcode: u8 = bus_read::fetch_byte(&mut self.cpu, &self.memory);
                let instructions: (fn(&mut CPU6502, &Memory), u8) =
                    instructions::get_instructions(opcode);
                cycles = instructions.1;
                instructions.0(&mut self.cpu, &self.memory);
            }
            cycles -= 1;

            if cycles == 0 {
                self.exit_program();
            }
        }
    }

    fn exit_program(&self) {
        println!("Register a: {}", self.cpu.registers.a);
        println!("TEST COMPLETE!... EXITING PROGRAM!");
        std::process::exit(0);
    }

    //test program
    pub fn load_program(&mut self) {
        self.memory.set_memory_value(0xfffc, 0xa5);
        self.memory.set_memory_value(0xfffd, 0x42);
        self.memory.set_memory_value(0x42, 84);
    }
}
