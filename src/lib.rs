use std::{env, error::Error, fs, thread, time::Duration, vec};
use crate::kha_error::{ErroType, KhaInterpreterErro};
use regex::Regex;

mod commands;
mod kha_error;

pub struct ProgramFileConfig {
    syntax_tree: Vec<Vec<String>>,
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

        let syntax_tree = parser(lexer(push_empty_line(text_file)));
        let line_total = syntax_tree.len();
        Ok(ProgramFileConfig {
            syntax_tree,
            curent_line: 1,
            line_total,
            delay,
        })
    }

    //
    // pub fn new_line(&mut self, new_text: String) -> &Self {
    //     self.syntax_tree.push(parser(lexer(new_text)));
    //     self.line_total += 1;

    //     self
    // }
}

pub fn new_empy_programing_config() -> ProgramFileConfig {
    ProgramFileConfig {
        delay: 0,
        curent_line: 0,
        line_total: 1_000_000,
        syntax_tree: vec![vec!["".to_string()]],
    }
}

fn push_empty_line(text: String) -> Vec<String> {
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
    loop {
        if code.curent_line >= code.line_total {
            return Ok(());
        }

        let parse = &code.syntax_tree[code.curent_line];

        evaluation(
            parse.to_vec(),
            &mut code.curent_line,
            &code.line_total,
            &code.delay,
        )?;
    }
}

pub fn online_interpreter(code: &mut ProgramFileConfig) -> Result<(), KhaInterpreterErro> {
    let parse = &code.syntax_tree[code.curent_line];

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

    // this '#' character is a comment
    // if this line is empty or is started with the comment character this function returns a vector with the comment value
    if text_as_bytes.len() == 0 || text_as_bytes[0] == b'#' {
        return vec![String::from("comment")];
    }

    let mut result: Vec<String> = Vec::new();

    for (index, &item) in text_as_bytes.into_iter().enumerate() {
        
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

fn parser(lex: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut parse: Vec<Vec<String>> = Vec::new();

    for list in lex {
        let mut p: Vec<String> = Vec::new();

        if list.len() == 3 {
            if list[1] == "=" {             
                let re = Regex::new("(^\"|')|[[:alpha:]]+").unwrap();
                
                if re.is_match(&list[2]) {
                    p.push(String::from("text_var"));
                } else {
                    p.push(String::from("num_var"));
                }                
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
        if list.len() == 1 && list[0] == "comment".to_string() {
            p.push("comment".to_string());
        }

        parse.push(p);
    }

    // println!("{:#?}", parse);
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

    if parse[0] == "comment".to_string() {
        return Ok(())
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
        if parse[1] == "sleep".to_string() {
            commands::sleep_command(&parse[2]);
            *curent_line += 1;
            return Ok(());
        }
            let text_in_line = &parse[1];
            return Err(KhaInterpreterErro::new(
            ErroType::CommondNotFound,
            *curent_line,
            text_in_line.to_string(),
        ))
    }
   
    Ok(())
    
}

#[cfg(test)]
mod tests {

    use crate::{kha_splitter, lexer, parser};


    #[test]
    fn standard_length_index_of_lexer() {
        let comment = "# this is comment! right?".to_string();
        let print_command = "print hi emad :D".to_string();
        let exit_command = "exit".to_string();
        let empty_line = "".to_string();
        let text_value = "txt= 1+1abc".to_string();
        let number_value = "num= 1+1".to_string();


        assert_eq!(kha_splitter(comment).len(), 1);
        assert_eq!(kha_splitter(print_command).len(), 2);
        assert_eq!(kha_splitter(exit_command).len(), 1);
        assert_eq!(kha_splitter(empty_line).len(), 1);
        assert_eq!(kha_splitter(text_value).len(), 3);
        assert_eq!(kha_splitter(number_value).len(), 3);
    }

    #[test]
    fn standard_length_of_parser() {

        let comment = vec!["# this is comment! right?".to_string()];
        let print_command = vec!["print hi emad :D".to_string()];
        let exit_command = vec!["exit".to_string()];
        let empty_line = vec!["".to_string()];
        let text_value = vec!["txt= 1+1abc".to_string()];
        let number_value = vec!["num= 1+1".to_string()];

        // let source_code = vec![comment, print_command, exit_command, empty_line, text_value, number_value];

        assert_eq!(parser(lexer(comment))[0].len(), 1);
        assert_eq!(parser(lexer(print_command))[0].len(), 3);
        assert_eq!(parser(lexer(exit_command))[0].len(), 2);
        assert_eq!(parser(lexer(empty_line))[0].len(), 1);
        assert_eq!(parser(lexer(text_value))[0].len(), 3);
        assert_eq!(parser(lexer(number_value))[0].len(), 3);



    }

}
    