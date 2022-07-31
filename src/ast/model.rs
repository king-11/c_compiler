use std::{fmt, error::Error};

#[derive(Debug)]
pub enum Expression {
  Const(i32)
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        Expression::Const(val) => write!(f, "Int<{}>", val)
      }
  }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Function {
  pub name: String,
  pub body: Statement
}

#[derive(Debug)]
pub struct Program {
  pub func: Function
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "FUN INT {}:\n  params: ()\n  body:\n    {}", self.func.name, self.func.body)
  }
}

#[derive(Debug)]
pub struct MyError(pub &'static str);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occured: {}", self.0)
    }
}

impl Error for MyError {}
