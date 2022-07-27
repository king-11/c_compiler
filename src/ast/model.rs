use std::{fmt, error::Error};

#[derive(Debug)]
pub enum Expression {
  Const(i32)
}

#[derive(Debug)]
pub enum Statement {
  Return(Expression)
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

#[derive(Debug)]
pub struct MyError(pub &'static str);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occured: {}", self.0)
    }
}

impl Error for MyError {}
