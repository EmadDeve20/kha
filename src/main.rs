use std::{env, process};
use kha::{ProgramFile, line_reader};



fn main() {
    let file_path = env::args().nth(1).unwrap();

    let mut program_file = ProgramFile::new(&file_path).unwrap_or_else(|err| {
        eprintln!("file error: {}", err);
        process::exit(1);
    });

    if let Err(e) = line_reader(&mut program_file) {
        eprintln!("{}", e.get_format_message());
        process::exit(1);
    }
}
