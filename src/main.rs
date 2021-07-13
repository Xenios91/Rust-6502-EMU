mod system;
use system::system::System;
fn main() {
    let mut system: System = System::new();

    system.load_program();
    system.execute();
}
