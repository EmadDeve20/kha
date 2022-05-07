use std::{env, error::Error, fs, thread, time::Duration, vec};

mod commands;

#[derive(Debug)]
pub enum ErroType {
    CommondNotFound,
}
#[derive(Debug)]
pub struct KhaInterpreterErro {
    type_of_erro: ErroType,
    line: usize,
    text_of_line: String,
}

impl KhaInterpreterErro {
    pub fn new(type_of_erro: ErroType, line: usize, text_of_line: String) -> KhaInterpreterErro {
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

    pub fn new_line(&mut self, new_text: String) -> &Self {
        self.syntax.push(new_text);
        self.line_total += 1;

        self
    }
}

pub fn new_empy_programing_config() -> ProgramFileConfig {
    ProgramFileConfig {
        delay: 0,
        curent_line: 0,
        line_total: 1_000_000,
        syntax: vec![],
    }
}

fn get_list_of_lines(text: &String) -> Vec<String> {
    let mut list_of_lines: Vec<String> = Vec::new();
    // I pushed an empty string in the first index because we start at one line and we have no zero line in plain text
    list_of_lines.push("".to_string());

    for line in text.lines() {
        list_of_lines.push(line.trim_start().trim_end().to_string());
    }

    // println!("{:#?}", list_of_lines);
    list_of_lines
}

pub fn interpreter(code: &mut ProgramFileConfig) -> Result<(), KhaInterpreterErro> {
    let lex = lexer(code.syntax.clone());
    let parse = parser(lex);
    loop {
        if code.curent_line >= code.line_total {
            return Ok(());
        }

        let parse = &parse[code.curent_line];

        evaluation(
            parse.to_vec(),
            &mut code.curent_line,
            &code.line_total,
            &code.delay,
        )?;
    }
}

pub fn online_interpreter(code: &mut ProgramFileConfig) -> Result<(), KhaInterpreterErro> {
    let lex = lexer(code.syntax.clone());
    let parse = parser(lex);
    let parse = &parse[code.curent_line];

    evaluation(
        parse.to_vec(),
        &mut code.curent_line,
        &code.line_total,
        &code.delay,
    )?;

    Ok(())
}

// this function is special string splitter for kha
fn kha_splitter(text: String) -> Vec<String> {
    let text_as_bytes = text.as_bytes();

    let mut result: Vec<String> = Vec::new();

    for (index, &item) in text_as_bytes.into_iter().enumerate() {
        // this character is a comment so we return an empty string
        if item == b'#' {
            return vec![String::from("comment")];
        }
        if item == b' ' {
            result.push(text[..index].trim().to_string());
            result.push(text[index + 1..].trim().to_string());
            return result;
        } else if item == b'=' {
            result.push(text[..index].trim().to_string());
            result.push(String::from("="));
            result.push(text[index + 1..].trim().to_string());
            return result;
        }
    }

    vec![text]
}

fn lexer(syntax: Vec<String>) -> Vec<Vec<String>> {
    let mut lex = Vec::new();

    for t in syntax {
        lex.push(kha_splitter(t));
    }

    lex
}

//TODO: We must change this name of function to 'parser'
fn parser(lex: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut parse: Vec<Vec<String>> = Vec::new();

    for list in lex {
        let mut p: Vec<String> = Vec::new();

        if list.len() == 3 {
            if list[1] == "=" {
                p.push(String::from("var"));
                p.push(list[0].to_string());
                p.push(list[2].to_string());
            }
        }
        if list.len() == 2 {
            p.push(String::from("command"));
            p.push(list[0].to_string());
            p.push(list[1].to_string());
        }
        if list.len() == 1 && list[0] != "comment".to_string() {
            p.push(String::from("command"));
            p.push(list[0].to_string());
        }
        if list.len() == 1 && (list[0] == "comment".to_string() || list[0] == "") {
            p.push("comment".to_string());
        }

        parse.push(p);
    }

    parse
}

fn evaluation(
    parse: Vec<String>,
    curent_line: &mut usize,
    total_line: &usize,
    delay: &u64,
) -> Result<(), KhaInterpreterErro> {
    if parse[0] == "comment".to_string() {
        *curent_line += 1;
        return Ok(());
    }

    thread::sleep(Duration::from_secs(*delay));

    if parse[0] == "command".to_string() {
        if parse[1] == "print".to_string() {
            commands::print_commond(&parse[2]);
            *curent_line += 1;
            return Ok(());
        }
        if parse[1] == "go".to_string() {
            commands::go_command(curent_line, &parse[2].to_string(), &total_line);
            return Ok(());
        }
        if parse[1] == "exit".to_string() || parse[1] == "exit\n".to_string() {
            commands::exit_command();
            *curent_line += 1;
            return Ok(());
        }
        if parse[1] == "clear".to_string() || parse[1] == "clear\n".to_string() {
            commands::clear_command();
            *curent_line += 1;
            return Ok(());
        }
    }
    // println!("{:#?}", vec);
    let text_in_line = &parse[1];
    Err(KhaInterpreterErro::new(
        ErroType::CommondNotFound,
        *curent_line,
        text_in_line.to_string(),
    ))
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
