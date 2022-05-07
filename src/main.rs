use kha::{interpreter, ProgramFileConfig, new_empy_programing_config, online_interpreter};
use std::{env, io, process};

fn main() {
    if env::args().len() == 1 {
        run_online_interpreter();
    } else {
        let mut program_file = ProgramFileConfig::new(env::args()).unwrap_or_else(|err| {
            eprintln!("file error: {}", err);
            process::exit(1);
        });

        if let Err(e) = interpreter(&mut program_file) {
            eprintln!("{}", e.get_format_message());
            process::exit(1);
        }
    }
}

fn run_online_interpreter() {
    let mut code = new_empy_programing_config();
    loop {
        // TODO: is why this [In] can not print first? we must fix it
        // println!("[In]: ");
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("sonething wrong!");
        code.new_line(text);
        if let Err(e) = online_interpreter(&mut code) {
            eprintln!("{}", e.get_format_message());
            continue;
        }
        // print!("[OutPut]: ");
        // get_plain_text(text);
    }
}
