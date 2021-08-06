pub mod lda_opcodes {
    pub const LDA_IMMEDIATE: u8 = 0xA9;
    pub const LDA_ZERO_PAGE: u8 = 0xA5;
    pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
}

pub mod lda_opcode_cycles {
    pub const LDA_IMMEDIATE: u8 = 2;
    pub const LDA_ZERO_PAGE: u8 = 3;
    pub const LDA_ZERO_PAGE_X: u8 = 4;
}

pub mod lda_instructions {
    use crate::virtual_system::bus::bus_operations::bus_output;
    use crate::virtual_system::cpu::virtual_cpu::cpu::CPU;
    use crate::virtual_system::memory::virtual_memory::Memory;

    pub fn lda_immediate(cpu: &mut CPU, memory: &Memory) {
        cpu.registers.a = bus_output::fetch_byte(cpu, memory);
        cpu.lda_set_status_flags();
    }
    pub fn lda_zero_page(cpu: &mut CPU, memory: &Memory) {
        let zero_page_address: u8 = bus_output::fetch_byte(cpu, memory);
        cpu.registers.a = bus_output::read_byte(memory, zero_page_address as usize);
        cpu.lda_set_status_flags();
    }
    pub fn lda_zero_page_x(cpu: &mut CPU, memory: &Memory) {
        let mut zero_page_address: u8 = bus_output::fetch_byte(cpu, memory);
        zero_page_address += cpu.registers.x;
        cpu.registers.a = bus_output::read_byte(memory, zero_page_address as usize);
        cpu.lda_set_status_flags();
    }
}
