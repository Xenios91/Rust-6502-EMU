mod system;
use system::virtual_machine::VirtualMachine;
fn main() {
    let mut system: VirtualMachine = VirtualMachine::new();

    system.load_program();
    system.execute();
}
