use crate::cpu_module::cpu_6502;

pub fn build_new_cpu() -> cpu_6502::CPU6502 {
    let cpu = cpu_6502::CPU6502::new();
    return cpu;
}
