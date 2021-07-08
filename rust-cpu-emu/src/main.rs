mod cpu_module;

fn main() {
    let mut cpu = cpu_module::cpu_builder::build_new_cpu();
    println!("CPU: {}", &cpu);
    println!("REGISTER a: {}", &cpu.get_register_value('a'));
    println!("SP: {}", &cpu.stack_pointer);
    cpu.registers.set_register_value('a', 10);
    println!("REGISTER a: {}", &cpu.get_register_value('a'));
    cpu.stack_pointer = 20;
    println!("SP: {}", &cpu.stack_pointer);
    cpu.status_flags.set_status_flag_value('c', true);
    println!("SFC: {}", &cpu.status_flags.get_status_flag_value('c'));
    cpu.reset();
    println!("SFC: {}", &cpu.status_flags.get_status_flag_value('c'));
}
