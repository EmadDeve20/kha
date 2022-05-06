use kha::{get_plain_text, line_reader, ProgramFileConfig};
use std::{env, io, process};

fn main() {
    if env::args().len() == 1 {
        run_online_interpreter();
    } else {
        let mut program_file = ProgramFileConfig::new(env::args()).unwrap_or_else(|err| {
            eprintln!("file error: {}", err);
            process::exit(1);
        });

        if let Err(e) = line_reader(&mut program_file) {
            eprintln!("{}", e.get_format_message());
            process::exit(1);
        }
    }
}

fn run_online_interpreter() {
    loop {
        // TODO: is why this [In] can not print first? we must fix it
        // println!("[In]: ");
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("sonething wrong!");
        print!("[OutPut]: ");
        get_plain_text(text);
    }
}
