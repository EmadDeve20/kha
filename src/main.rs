use std::{env,fs, process, error::Error , collections::HashMap};


#[derive(Debug)]
enum Erros{
    CommondNotFound
}

struct ProgramFile {
    syntaxt: String
}

impl ProgramFile {
    fn new(file_path: &String) -> Result<ProgramFile, Box<dyn Error>> {
        let file = fs::read_to_string(file_path)?;

        Ok ( ProgramFile { syntaxt: file } )
    }
}

fn main() {
    
    let file_path = env::args().nth(1).unwrap();
    
    let program_file = ProgramFile::new(&file_path).unwrap_or_else(|err|{
        eprintln!("file error: {}", err);
        process::exit(1);
    });
    

    if let Err(e) = parser(&program_file){
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }

}


fn parser(line: &ProgramFile) -> Result<(), Erros> {

    let line_as_byte = line.syntaxt.as_bytes();
    let syntax = &line.syntaxt.clone();

    for (i, &item) in line_as_byte.iter().enumerate(){
        if item == b' ' {
            if &syntax[..i] == "print" {
                print_commond(&syntax[i+1..]);
                return Ok(())
            }
        }    
    }
    
    Err(Erros::CommondNotFound)
}

fn print_commond(command: &str) {
    println!("{}", command);
}
