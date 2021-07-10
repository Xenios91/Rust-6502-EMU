mod system;
use system::system::System;
fn main() {
    let mut system: System = System::new();

    system.memory.set_memory_value(0xfffc, 0xa5);
    system.memory.set_memory_value(0xfffd, 0x42);
    system.memory.set_memory_value(0x42, 84);
    system.execute(3);
    println!("register a value: {}", system.cpu.registers.a);
}
