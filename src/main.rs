use std::{io::{Error, BufReader, BufRead, Write}, fs::File, env, path::Path, process::Command};

use codegen::generate;

mod lex;
mod ast;
mod codegen;

fn lex(path: &str) -> Result<Vec<lex::Token>, Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut token_vector = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut tokens = lex::recurse_tokens(&line);
        token_vector.append(&mut tokens);
    };

    Ok(token_vector)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "./data/multi_digit.c".to_string();
    let path = Path::new(args.get(1).unwrap_or(&default_filename));

    let path_value = path.to_str().unwrap();
    match lex(path_value) {
        Ok(tokens) => {
            match ast::parse_program(&mut tokens.iter()) {
                Ok(program) => {
                    let assembly = generate(&program);
                    let filename = path.file_stem().unwrap().to_str().unwrap();
                    let mut file = File::create(path.with_file_name(format!("{}.s", filename))).unwrap();
                    file.write_all(assembly.as_bytes()).unwrap();

                    Command::new("gcc")
                        .arg(format!("{}.s", filename))
                        .arg("-o")
                        .arg(path.with_file_name(format!("{}", filename)))
                        .output()
                        .expect("assembly to elf failed");

                },
                Err(e) => {
                    println!("error occured: {}", e);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
