use std::{env, error::Error, fs, process};

#[derive(Debug)]
enum ErroType {
    CommondNotFound,
}
#[derive(Debug)]
struct KhaInterpreterErro {
    type_of_erro: ErroType,
    line: String,
    text_of_line: String,
}

impl KhaInterpreterErro {
    fn new(type_of_erro: ErroType, line: String, text_of_line: String) -> KhaInterpreterErro {
        KhaInterpreterErro {
            type_of_erro,
            line,
            text_of_line,
        }
    }

    fn get_format_message(&self) -> String {
        format!(
            "{:?}: {} in {} line",
            self.type_of_erro, self.text_of_line, self.line
        )
    }
}

struct ProgramFile {
    syntaxt: String,
}

impl ProgramFile {
    fn new(file_path: &String) -> Result<ProgramFile, Box<dyn Error>> {
        let file = fs::read_to_string(file_path)?;

        Ok(ProgramFile { syntaxt: file })
    }
}

fn main() {
    let file_path = env::args().nth(1).unwrap();

    let program_file = ProgramFile::new(&file_path).unwrap_or_else(|err| {
        eprintln!("file error: {}", err);
        process::exit(1);
    });

    if let Err(e) = line_reader(&program_file) {
        eprintln!("{}", e.get_format_message());
        process::exit(1);
    }
}

fn line_reader(code: &ProgramFile) -> Result<(), KhaInterpreterErro> {
    let mut curent_line: u32 = 1;
    for line in code.syntaxt.lines() {
        parser(line, &curent_line)?;
        curent_line += 1;
    }

    Ok(())
}

fn parser(text: &str, line: &u32) -> Result<(), KhaInterpreterErro> {
    let line_as_byte = text.clone().as_bytes();
    let syntax = &text.clone();

    for (i, &item) in line_as_byte.iter().enumerate() {
        if item == b'#' {
            return Ok(());
        }
        if item == b' ' {
            if &syntax[..i] == "print" {
                print_commond(&syntax[i + 1..]);
                return Ok(());
            }
        }
    }
    Err(KhaInterpreterErro::new(
        ErroType::CommondNotFound,
        line.to_string(),
        text.to_string(),
    ))
}

fn print_commond(command: &str) {
    println!("{}", command);
}
