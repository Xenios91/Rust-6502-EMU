use crate::system::cpu_module::cpu_6502::CPU6502;
use crate::system::memory::memory::Memory;

mod lda_opcodes {
    pub const LDA_IMMEDIATE: u8 = 0xA9;
    pub const LDA_ZERO_PAGE: u8 = 0xA5;
    pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
}

mod lda_instructions {
    use crate::system::cpu_module::cpu_6502::CPU6502;
    use crate::system::memory::memory::Memory;

    pub fn run_lda_immediate(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
        cpu.registers.a = super::fetch_byte(cpu, memory, cycles);
        cpu.lda_set_status_flags();
    }

    pub fn run_lda_zero_page(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
        let zero_page_address: u8 = super::fetch_byte(cpu, memory, cycles);
        cpu.registers.a = super::read_byte(memory, cycles, zero_page_address as usize);
        cpu.lda_set_status_flags();
    }

    pub fn run_lda_zero_page_x(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
        let mut zero_page_address: u8 = super::fetch_byte(cpu, memory, cycles);
        zero_page_address += cpu.registers.x;
        *cycles -= 1;
        cpu.registers.a = super::read_byte(memory, cycles, zero_page_address as usize);
        cpu.lda_set_status_flags();
        *cycles -= 1;
    }
}

pub fn run_instruction(cpu: &mut CPU6502, memory: &Memory, instruction: u8, cycles: &mut u32) {
    match instruction {
        lda_opcodes::LDA_IMMEDIATE => lda_instructions::run_lda_immediate(cpu, memory, cycles),
        lda_opcodes::LDA_ZERO_PAGE => lda_instructions::run_lda_zero_page(cpu, memory, cycles),
        lda_opcodes::LDA_ZERO_PAGE_X => lda_instructions::run_lda_zero_page_x(cpu, memory, cycles),
        _ => panic!("Unknown instruction {}", instruction),
    }
}

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
