use crate::system::cpu::cpu_6502::CPU6502;
use crate::system::instructions::opcode_categories::lda_instructions::*;
use crate::system::memory::memory::Memory;

pub fn execute_instruction(cpu: &mut CPU6502, memory: &Memory, instruction: u8, cycles: &mut u32) {
    match instruction {
        lda_opcodes::LDA_IMMEDIATE => lda_immediate(cpu, memory, cycles),
        lda_opcodes::LDA_ZERO_PAGE => lda_zero_page(cpu, memory, cycles),
        lda_opcodes::LDA_ZERO_PAGE_X => lda_zero_page_x(cpu, memory, cycles),
        _ => panic!("Unknown instruction {}\n Exiting emulator!", instruction),
    }
}
