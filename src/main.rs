mod virtual_machine;
use virtual_machine::virtual_machine::VirtualMachine;
fn main() {
    let mut system: VirtualMachine = VirtualMachine::new();

    system.load_program();
    system.execute();
}
