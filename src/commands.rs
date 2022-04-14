use std::process;

pub fn print_commond(command: &str) {
    println!("{}", command);
}

pub fn exit_command() {
    process::exit(0);
}

pub fn go_command(current_line: &mut usize, go_line: &String, line_total: &usize) {
    *current_line = go_line.parse::<usize>().unwrap_or_else(|_| line_total + 1);
    if *current_line == 0 {
        *current_line = line_total + 1
    }
}
