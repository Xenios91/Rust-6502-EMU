use crate::system::cpu::cpu_6502::CPU6502;
use crate::system::cpu::instructions::opcode_categories::lda::*;
use crate::system::memory::virtual_memory::Memory;

pub fn get_instructions(opcode: u8) -> (fn(&mut CPU6502, &Memory), u8) {
    match opcode {
        lda_opcodes::LDA_IMMEDIATE => (
            lda_instructions::lda_immediate,
            lda_opcode_cycles::LDA_IMMEDIATE,
        ),
        lda_opcodes::LDA_ZERO_PAGE => (
            lda_instructions::lda_zero_page,
            lda_opcode_cycles::LDA_ZERO_PAGE,
        ),
        lda_opcodes::LDA_ZERO_PAGE_X => (
            lda_instructions::lda_zero_page_x,
            lda_opcode_cycles::LDA_ZERO_PAGE_X,
        ),
        _ => panic!("Unknown instruction {}\n Exiting emulator!", opcode),
    }
}
