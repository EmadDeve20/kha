use std::process;

pub fn print_commond(command: &str) {
    println!("{}", command);
}

pub fn exit_command() {
    process::exit(0);
}