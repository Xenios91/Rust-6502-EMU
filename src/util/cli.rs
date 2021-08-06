use std::{
    env,
    fs::{self, File, Metadata},
    io::Read,
    process::exit,
};

pub fn run_cli_args() -> (String, bool) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments supplied... Exiting...");
        exit(0);
    }

    if args[1] == "-c" || args[1] == "--compile" {
        if run_compiler(&args[2]) {
            print!("{} successfully compiled!", args[2]);
        }
        (String::from(""), false)
    } else {
        let file_name = args[1].to_owned();
        (file_name, true)
    }
}

pub fn get_program(file_name: &str) -> Vec<u8> {
    let mut file: File = File::open(&file_name).expect("file not found!");
    let metadata: Metadata = fs::metadata(&file_name).expect("unable to read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer)
        .expect("An error has occured reading the file");
    buffer
}

fn run_compiler(source_file: &str) -> bool {
    print!("Compiling {}...", source_file);
    println!("Scuessfully compiled {}", source_file);
    true //test for now
}
