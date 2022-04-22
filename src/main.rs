use kha::{line_reader, ProgramFileConfig};
use std::{env, process};

fn main() {
    let mut program_file = ProgramFileConfig::new(env::args()).unwrap_or_else(|err| {
        eprintln!("file error: {}", err);
        process::exit(1);
    });

    if let Err(e) = line_reader(&mut program_file) {
        eprintln!("{}", e.get_format_message());
        process::exit(1);
    }
}
