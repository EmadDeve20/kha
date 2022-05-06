use std::{env, error::Error,fs, thread, time::Duration, vec};

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

pub struct ProgramFileConfig {
    syntax: Vec<String>,
    curent_line: usize,
    line_total: usize,
    delay: u64,
}

impl ProgramFileConfig {
    pub fn new(mut args: env::Args) -> Result<ProgramFileConfig, Box<dyn Error>> {
        args.next();
        let file_path = args.nth(0).unwrap();
        let text_file = fs::read_to_string(file_path)?;

        let delay: u64 = match args.next() {
            Some(arg) => arg.trim().parse::<u64>().unwrap_or_else(|_| 1),
            None => 0,
        };
        let syntax = get_list_of_lines(&text_file);
        let line_total = syntax.len();
        Ok(ProgramFileConfig {
            syntax,
            curent_line: 1,
            line_total,
            delay,
        })
    }
}

fn get_list_of_lines(text: &String) -> Vec<String> {
    let mut list_of_lines: Vec<String> = Vec::new();
    list_of_lines.push("".to_string());

    for line in text.lines() {
        list_of_lines.push(line.trim_start().trim_end().to_string());
    }

    // println!("{:#?}", list_of_lines);
    list_of_lines
}

pub fn line_reader(code: &mut ProgramFileConfig) -> Result<(), KhaInterpreterErro> {
    loop {
        if code.curent_line >= code.line_total {
            return Ok(());
        }
        parser(
            &code.syntax[code.curent_line],
            &mut code.curent_line,
            &code.line_total,
        )?;
        thread::sleep(Duration::from_secs(code.delay));
    }
}

fn parser(text: &String, line: &mut usize, max_line: &usize) -> Result<(), KhaInterpreterErro> {
    if text.trim() == "exit" {
        commands::exit_command();
        return Ok(());
    }
    let line_as_byte = text.as_bytes();
    let syntax = &text.clone();

    for (i, &item) in line_as_byte.iter().enumerate() {
        if item == b'#' {
            *line += 1;
            return Ok(());
        }
        if item == b' ' {
            if &syntax[..i] == "print" {
                commands::print_commond(&syntax[i + 1..]);
                *line += 1;
                return Ok(());
            }
            if &syntax[..i] == "go" {
                commands::go_command(line, &syntax[i + 1..].to_string(), &max_line);
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

// this function is special string splitter for kha
fn kha_splitter<'a>(text: &'a str) -> Vec<&'a str> {
    let text_as_bytes = text.as_bytes();

    // this character is a comment so we return an empty string
    if text_as_bytes[0] == b'#' {
        return vec![""];
    }

    let mut result: Vec<&str> = Vec::new();

    for (index, &item) in text_as_bytes.into_iter().enumerate() {
        if item == b' ' {
            result.push(&text[..index].trim());
            result.push(&text[index + 1..].trim());
            return result;
        } else if item == b'=' {
            result.push(&text[..index].trim());
            result.push("=");
            result.push(&text[index + 1..].trim());
            return result;
        }
    }

    vec![&text]
}

fn lexer<'a>(text: &'a str) -> Vec<Vec<&'a str>> {
    let mut lex = Vec::new();

    for t in text.lines() {
        lex.push(kha_splitter(t));
    }

    lex
}

//TODO: We must change this name of function to 'parser'
fn parseer<'a>(lex: Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut parse: Vec<Vec<&'a str>> = Vec::new();

    for list in lex {
        let mut p = Vec::new();

        if list.len() == 3 {
            if list[1] == "=" {
                p.push("var");
                p.push(list[0]);
                p.push(list[2]);
            }
        }
        if list.len() == 2 {
            p.push("command");
            p.push(list[0]);
            p.push(list[1]);
        } else if list.len() == 1 {
            p.push("command");
            p.push(list[0]);
        }

        parse.push(p);
    }

    parse
}

fn evaluation<'a>(parse: Vec<Vec<&'a str>>) {
    for (_, vec) in parse.iter().enumerate() {
        if vec.len() == 3 {
            if vec[1] == "print" {
                commands::print_commond(vec[2]);
            }
        }
        if vec.len() == 2 {
            if vec[1] == "exit" {
                commands::exit_command();
            }
        }

        // println!("{:#?}", vec);
    }
}

pub fn get_plain_text(plain_text: String) {
    evaluation(parseer(lexer(&plain_text[..])))
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::lexer;

    #[test]
    fn lexter_test() {
        let test1 = vec!["a", "=", "test"];
        let test2 = vec!["b", "=", "1"];
        let test3 = vec!["print", "hello world"];
        let test4 = vec!["exit"];
        let test5 = vec!["go", "1"];
        todo!();
        //we must create and impl PartialEq for Vec<&'a str>
        // assert_eq!(lexer("a=   test"), test1);
    }
}
