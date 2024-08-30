use std::{
  env,
  fs::File,
  io::{BufRead, BufReader, Write},
  path::Path,
  process::Command,
};

mod lex;

mod ast;
use ast::Scanner;
use itertools::{multipeek, Itertools};

mod codegen;
use codegen::CodeGenerator;
use tracing::{debug, info};
use utility::SyntaxError;

mod utility;

fn lex(path: &str) -> Result<Vec<lex::Token>, SyntaxError> {
  let f = File::open(path).expect("unable to open file");
  let f = BufReader::new(f);
  let mut token_vector = Vec::new();
  for line in f.lines() {
    let line = line.unwrap();
    debug!("lexing line {}", line);
    let mut tokens = lex::string_tokenizer(&line)?;
    token_vector.append(&mut tokens);
  }

  Ok(token_vector)
}

fn main() -> Result<(), SyntaxError> {
  tracing_subscriber::fmt::fmt().init();

  let default_filename = "./data/stage_5/valid/exp_return_val.c".to_string();
  info!("default file path {}", default_filename);

  let path_value = env::args().get(1..2).next().unwrap_or(default_filename);
  let path = Path::new(&path_value);
  info!("running compiler for file {}", path_value);

  let tokens = lex(&path_value)?;
  let mut scanner = Scanner::new(multipeek(tokens.iter()));
  let program = ast::parse_program(&mut scanner)?;
  let mut codegenerator = CodeGenerator::new();
  let assembly = codegenerator.generate(&program)?;
  let filename = path.file_stem().unwrap().to_str().unwrap();
  let mut file = File::create(path.with_file_name(format!("{}.s", filename))).unwrap();
  file.write_all(assembly.as_bytes()).unwrap();

  Command::new("gcc")
    .arg(path.with_file_name(format!("{}.s", filename)))
    .arg("-o")
    .arg(path.with_file_name(format!("{}", filename)))
    .output()
    .expect("assembly to elf failed");

  Ok(())
}
