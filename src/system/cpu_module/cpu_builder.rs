use super::super::cpu_module::cpu_6502;

pub fn build_new_cpu() -> cpu_6502::CPU6502 {
    cpu_6502::CPU6502::new()
}
