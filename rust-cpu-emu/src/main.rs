mod cpu_module;

fn main() {
    let mut cpu = cpu_module::cpu_builder::build_new_cpu();
    println!("{}", cpu);
    println!("{}", cpu.get_register_value('a'));
    println!("{}", cpu.get_stack_pointer());
    cpu.set_register_value('a', 10);
    println!("{}", cpu.get_register_value('a'));
    cpu.set_stack_pointer(20);
    println!("{}", cpu.get_stack_pointer());
}
