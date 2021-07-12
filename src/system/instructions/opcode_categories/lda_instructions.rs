use crate::system::bus::bus_operations::bus_read;
use crate::system::cpu::cpu_6502::CPU6502;
use crate::system::memory::memory::Memory;

pub mod lda_opcodes {
    pub const LDA_IMMEDIATE: u8 = 0xA9;
    pub const LDA_ZERO_PAGE: u8 = 0xA5;
    pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
}

pub fn lda_immediate(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
    cpu.registers.a = bus_read::fetch_byte(cpu, memory, cycles);
    cpu.lda_set_status_flags();
}
pub fn lda_zero_page(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
    let zero_page_address: u8 = bus_read::fetch_byte(cpu, memory, cycles);
    cpu.registers.a = bus_read::read_byte(memory, cycles, zero_page_address as usize);
    cpu.lda_set_status_flags();
}
pub fn lda_zero_page_x(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
    let mut zero_page_address: u8 = bus_read::fetch_byte(cpu, memory, cycles);
    zero_page_address += cpu.registers.x;
    *cycles -= 1;
    cpu.registers.a = bus_read::read_byte(memory, cycles, zero_page_address as usize);
    cpu.lda_set_status_flags();
    *cycles -= 1;
}
