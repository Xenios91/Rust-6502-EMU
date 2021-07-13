pub mod bus_read {
    use crate::system::cpu::cpu_6502::CPU6502;
    use crate::system::memory::memory::Memory;

    pub fn read_byte(memory: &Memory, address: usize) -> u8 {
        let byte: u8 = memory.retrieve_memory(address);
        byte
    }

    pub fn fetch_byte(cpu: &mut CPU6502, memory: &Memory) -> u8 {
        let byte: u8 = memory.retrieve_memory(cpu.program_counter as usize);
        cpu.program_counter += 1;
        byte
    }
}

pub mod bus_write {
    use crate::system::memory::memory::Memory;

    fn write_bytes(memory: &mut Memory, address: usize, memory_value: &[u8]) {
        for value in memory_value {
            memory.set_memory_value(address, *value);
        }
    }
}
