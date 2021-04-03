mod cpu_module;

fn main() {
    let cpu = cpu_module::cpu_builder::build_new_cpu();
    println!("{}", cpu);
}
