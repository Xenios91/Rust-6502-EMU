use crate::system::cpu_module::cpu_6502::CPU6502;
use crate::system::memory::memory::Memory;

mod lda_opcodes {
    pub const LDA_IMMEDIATE: u8 = 0xa9;
}

fn run_lda_immediate(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) {
    cpu.registers.a = fetch_byte(cpu, memory, cycles);
    cpu.status_flags
        .set_status_flag_value('z', cpu.registers.a == 0);
    cpu.status_flags
        .set_status_flag_value('n', cpu.registers.a & 0b10000000 > 0);
}

pub fn run_instruction(cpu: &mut CPU6502, memory: &Memory, instruction: u8, cycles: &mut u32) {
    match instruction {
        lda_opcodes::LDA_IMMEDIATE => run_lda_immediate(cpu, memory, cycles),
        _ => panic!("Unknown instruction {}", instruction),
    }
}

pub fn fetch_byte(cpu: &mut CPU6502, memory: &Memory, cycles: &mut u32) -> u8 {
    let byte: u8 = memory.retrieve_memory(cpu.program_counter as usize);
    cpu.program_counter += 1;
    *cycles -= 1;
    byte
}
