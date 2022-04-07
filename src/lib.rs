use std::{error::Error, fs};

mod commands;

#[derive(Debug)]
pub enum ErroType {
    CommondNotFound,
}
#[derive(Debug)]
pub struct KhaInterpreterErro {
    type_of_erro: ErroType,
    line: String,
    text_of_line: String,
}

impl KhaInterpreterErro {
    pub fn new(type_of_erro: ErroType, line: String, text_of_line: String) -> KhaInterpreterErro {
        KhaInterpreterErro {
            type_of_erro,
            line,
            text_of_line,
        }
    }

    pub fn get_format_message(&self) -> String {
        format!(
            "{:?}: {} in {} line",
            self.type_of_erro, self.text_of_line, self.line
        )
    }
}

pub struct ProgramFile {
    syntax: Vec<String>,
    curent_line: usize,
    line_total: usize,
}

impl ProgramFile {
    pub fn new(file_path: &String) -> Result<ProgramFile, Box<dyn Error>> {
        let text_file = fs::read_to_string(file_path)?;
        let syntax = get_list_of_lines(&text_file);
        let line_total = syntax.len();
        Ok(ProgramFile {
            syntax,
            curent_line: 0,
            line_total,
        })
    }
}

fn get_list_of_lines(text: &String) -> Vec<String> {
    let mut list_of_lines: Vec<String> = Vec::new();
    for line in text.lines() {
        list_of_lines.push(line.trim_start().trim_end().to_string());
    }

    // println!("{:#?}", list_of_lines);
    list_of_lines
}

pub fn line_reader(code: &mut ProgramFile) -> Result<(), KhaInterpreterErro> {
    loop {
        parser(&code.syntax[code.curent_line], &mut code.curent_line)?;

        code.curent_line += 1;

        if code.curent_line >= code.line_total {
            return Ok(());
        }
    }
}

fn parser(text: &String, line: &mut usize) -> Result<(), KhaInterpreterErro> {
    if text.trim() == "exit" {
        commands::exit_command();
        return Ok(());
    }
    let line_as_byte = text.as_bytes();
    let syntax = &text.clone();

    for (i, &item) in line_as_byte.iter().enumerate() {
        if item == b'#' {
            return Ok(());
        }
        if item == b' ' {
            if &syntax[..i] == "print" {
                commands::print_commond(&syntax[i + 1..]);
                return Ok(());
            }
            // this is a standard command for kha
            // this is better to save it here or command.rs file?
            if &syntax[..i] == "go" {
                *line = syntax[i+1 ..].parse::<usize>().unwrap() - 1;
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
