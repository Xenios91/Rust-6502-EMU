mod system;
use system::system::system::System;
fn main() {
    let mut system: System = System::new();

    system.memory.set_memory_value(0xfffc, 0xa9);
    system.memory.set_memory_value(0xfffd, 0xb);
    system.execute(2);
    println!("register a value: {}", system.cpu.registers.a);
}
