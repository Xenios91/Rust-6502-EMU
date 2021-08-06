use crate::virtual_system::bus::bus_operations::bus_output;
use crate::virtual_system::cpu::opcodes::vm_instructions;
use crate::virtual_system::cpu::virtual_cpu::cpu::CPU;
use crate::virtual_system::memory::virtual_memory::Memory;

pub struct VirtualMachine {
    cpu: CPU,
    memory: Memory,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut system: VirtualMachine = VirtualMachine {
            cpu: CPU::new(),
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
                let opcode: u8 = bus_output::fetch_byte(&mut self.cpu, &self.memory);
                let instructions: (fn(&mut CPU, &Memory), u8) =
                    vm_instructions::get_instructions(opcode);
                cycles = instructions.1;
                instructions.0(&mut self.cpu, &self.memory);
            }
            cycles -= 1;

            if cycles == 0 {
                self.exit_vm();
            }
        }
    }

    fn exit_vm(&self) {
        println!("Register a: {}", self.cpu.registers.a); //print register A's value to test
        println!("TEST COMPLETE!... EXITING PROGRAM!");
        std::process::exit(0);
    }

    //test program
    pub fn load_program(&mut self, program_bytes: Vec<u8>) {
        self.memory.set_memory_value(0xfffc, 0xa5);
        self.memory.set_memory_value(0xfffd, 0x42);
        self.memory.set_memory_value(0x42, 84);
    }
}
