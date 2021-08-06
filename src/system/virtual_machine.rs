use crate::system::bus::bus_operations::bus_read;
use crate::system::cpu::cpu_6502::CPU6502;
use crate::system::cpu::instructions::vm_instructions;
use crate::system::memory::virtual_memory::Memory;

pub struct VirtualMachine {
    pub cpu: CPU6502,
    pub memory: Memory,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut system: VirtualMachine = VirtualMachine {
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
        let mut cycles: u8 = 0;
        loop {
            if cycles == 0 {
                let opcode: u8 = bus_read::fetch_byte(&mut self.cpu, &self.memory);
                let instructions: (fn(&mut CPU6502, &Memory), u8) =
                    vm_instructions::get_instructions(opcode);
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
        println!("Register a: {}", self.cpu.registers.a); //print register A's value to test
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
