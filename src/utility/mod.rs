use std::{error::Error, fmt};

#[derive(Debug)]
enum CompilerStage {
  LEXER,
  PARSER,
  CODEGENERATOR,
}

#[derive(Debug)]
pub struct SyntaxError {
  message: String,
  level: CompilerStage,
}

impl SyntaxError {
  pub fn new_lex_error(message: String) -> Self {
    SyntaxError {
      message,
      level: CompilerStage::LEXER,
    }
  }
  pub fn new_parse_error(message: String) -> Self {
    SyntaxError {
      message,
      level: CompilerStage::PARSER,
    }
  }
  pub fn new_codegen_error(message: String) -> Self {
    SyntaxError {
      message,
      level: CompilerStage::CODEGENERATOR,
    }
  }
}

impl fmt::Display for SyntaxError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} Error {}", self.level, self.message)
  }
}

impl Error for SyntaxError {}
