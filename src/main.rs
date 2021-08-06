mod util;
mod virtual_system;
use std::process::exit;
use util::cli::get_program;
use virtual_system::virtual_machine::VirtualMachine;

fn main() {
    let cli_results: (String, bool) = util::cli::run_cli_args();
    if !cli_results.1 {
        exit(0);
    }

    let file_name: String = cli_results.0;
    let mut system: VirtualMachine = VirtualMachine::new();

    let program_bytes: Vec<u8> = get_program(&file_name);
    system.load_program(program_bytes);
    system.execute();
}
