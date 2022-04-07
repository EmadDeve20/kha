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
    syntaxt: String,
}

impl ProgramFile {
    pub fn new(file_path: &String) -> Result<ProgramFile, Box<dyn Error>> {
        let file = fs::read_to_string(file_path)?;

        Ok(ProgramFile { syntaxt: file })
    }
}



pub fn line_reader(code: &ProgramFile) -> Result<(), KhaInterpreterErro> {
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
                commands::print_commond(&syntax[i + 1..]);
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
