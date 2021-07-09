mod system;
use system::system::system::System;
fn main() {
    let mut system: System = System::new();

    println!("Program Counter: {}", system.cpu.program_counter);
    system.cpu.program_counter = 1;
    println!("Program Counter: {}", system.cpu.program_counter);
    system.cpu.registers.set_register_value('a', 0xa);
    println!("Register a: {}", system.cpu.registers.a);
    println!("Resetting registers...");
    system.cpu.reset();
    println!("Program Counter: {}", system.cpu.program_counter);
    println!("Register a: {}", system.cpu.registers.a);
    println!("Resetting system...");
    system.reset();
    println!("Program Counter: {}", system.cpu.program_counter);
    println!("Register a: {}", system.cpu.registers.a);
    println!("Total memory: {}", system.memory.get_total_memory());
    system.execute(2);
}
