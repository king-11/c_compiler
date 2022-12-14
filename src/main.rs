use std::{io::{Error, BufReader, BufRead, Write}, fs::File, env, path::Path, process::Command};

mod lex;

mod ast;
use ast::Scanner;
use itertools::multipeek;

mod codegen;
use codegen::CodeGenerator;

mod utility;

fn lex(path: &str) -> Result<Vec<lex::Token>, Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut token_vector = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut tokens = lex::string_tokenizer(&line);
        token_vector.append(&mut tokens);
    };

    Ok(token_vector)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "./data/stage_5/valid/exp_return_val.c".to_string();
    let path = Path::new(args.get(1).unwrap_or(&default_filename));

    let path_value = path.to_str().unwrap();
    match lex(path_value) {
        Ok(tokens) => {
            let mut scanner = Scanner::new(multipeek(tokens.iter()));
            match ast::parse_program(&mut scanner) {
                Ok(program) => {
                    // println!("{}", program);
                    let mut codegenerator = CodeGenerator::new();
                    match codegenerator.generate(&program) {
                        Ok(assembly) => {
                            let filename = path.file_stem().unwrap().to_str().unwrap();
                            let mut file = File::create(path.with_file_name(format!("{}.s", filename))).unwrap();
                            file.write_all(assembly.as_bytes()).unwrap();

                            Command::new("gcc")
                                .arg(path.with_file_name(format!("{}.s", filename)))
                                .arg("-o")
                                .arg(path.with_file_name(format!("{}", filename)))
                                .output()
                                .expect("assembly to elf failed");
                        },
                        Err(e) => println!("{}", e)
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
