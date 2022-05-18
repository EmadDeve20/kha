use std::process::exit;
use std::{thread, time::Duration};

pub fn print_commond(command: &str) {
    println!("{}", command);
}

pub fn exit_command() {
    exit(0);
}

pub fn go_command(current_line: &mut usize, go_line: &String, line_total: &usize) {
    *current_line = go_line.parse::<usize>().unwrap_or_else(|_| line_total + 1);
    if *current_line == 0 {
        *current_line = line_total + 1
    }
}

pub fn clear_command(){
    // print ASCII character to clear the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn sleep_command(time_to_second: &String){
    let time_to_secoud:u64 = time_to_second.trim().parse().unwrap();
    thread::sleep(Duration::from_secs(time_to_secoud));
}
