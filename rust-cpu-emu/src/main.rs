mod cpu_module;

fn main() {
    let mut cpu = cpu_module::cpu_builder::build_new_cpu();
    println!("CPU: {}", &cpu);
    println!("REGISTER a: {}", &cpu.get_register_value('a'));
    println!("SP: {}", &cpu.get_stack_pointer());
    cpu.set_register_value('a', 10);
    println!("REGISTER a: {}", &cpu.get_register_value('a'));
    cpu.set_stack_pointer(20);
    println!("SP: {}", &cpu.get_stack_pointer());
    cpu.set_status_flag_value('c', true);
    println!("SFC: {}", &cpu.get_status_flag_value('c'));
}
