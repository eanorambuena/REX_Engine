use std::process;

fn error (message: String) {
    println!("{}", message);
    process::exit(-1);
}

pub fn argument_error () { error("ArgumentError: Invalid number of arguments".to_string()) }

pub fn file_error () { error("FileError: No file or argument specified".to_string()) }

pub fn assembly_error () { error("AssemblyError: Assembly failed".to_string()) }

pub fn assert_arguments (argc: usize, number: u8) {
    if argc - 1 < (number).into() {
        file_error();
    }
}
