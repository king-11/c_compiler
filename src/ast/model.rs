use std::{fmt, iter::Peekable, slice::Iter};

use crate::lex::{UnaryOperator, Token, SyntaxError};

pub struct Scanner<'a> {
  tokens: Peekable<Iter<'a, Token>>
}

impl<'a> Scanner<'a> {
  pub fn new(tokens: Peekable<Iter<'a, Token>>) -> Self {
    Self {
      tokens: tokens
    }
  }
  pub fn pop(&mut self, error_message: &'static str) -> Result<&'a Token, SyntaxError> {
    match self.tokens.next() {
      Some(val) => Ok(val),
      None => Err(SyntaxError::new_parse_error(error_message.to_string()))
    }
  }
  pub fn take(&mut self, token_type: Token, error_message: &'static str) -> Result<(), SyntaxError> {
    let token = self.pop(error_message)?;
    if *token == token_type {
      return Ok(());
    }
    Err(SyntaxError::new_parse_error(error_message.to_string()))
  }
}


pub enum Expression {
  Const(i32),
  Unary { op: UnaryOperator, exp: Box<Expression> }
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        Expression::Const(val) => write!(f, "Int<{}>", val),
        Expression::Unary { op, exp } => write!(f, "{:?}<{}>", op, exp)
      }
  }
}

pub enum Statement {
  Return(Expression)
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        Statement::Return(val) => write!(f, "RETURN {}", val)
      }
  }
}

pub struct Function {
  pub name: String,
  pub body: Statement
}

pub struct Program {
  pub func: Function
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "FUN INT {}:\n  params: ()\n  body:\n    {}", self.func.name, self.func.body)
  }
}
