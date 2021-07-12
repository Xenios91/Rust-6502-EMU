pub mod bus_read {
    use crate::system::cpu::cpu_6502::CPU6502;
    use crate::system::memory::memory::Memory;

    pub fn read_byte(memory: &Memory, cycles: &mut u32, address: usize) -> u8 {
        let byte: u8 = memory.retrieve_memory(address);
        *cycles -= 1;
        byte
    }

    pub fn fetch_byte(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) -> u8 {
        let byte: u8 = memory.retrieve_memory(cpu.program_counter as usize);
        cpu.program_counter += 1;
        *cycles -= 1;
        byte
    }
}

pub mod bus_write {
    use crate::system::memory::memory::Memory;

    fn write_bytes(memory: &mut Memory, cycles: &mut u32, address: usize, memory_value: &[u8]) {
        for value in memory_value {
            memory.set_memory_value(address, *value);
            *cycles -= 1;
        }
    }
}
