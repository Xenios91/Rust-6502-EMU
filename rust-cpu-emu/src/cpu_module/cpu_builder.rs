use crate::cpu_module::cpu_6502;

pub fn build_new_cpu() -> cpu_6502::CPU {
    let cpu = cpu_6502::create_new_cpu();
    return cpu;
}
