use std::{fmt, rc::Rc, slice::Iter};

use itertools::MultiPeek;

use crate::{
  lex::{BinaryOperator, Token, UnaryOperator},
  utility::SyntaxError,
};

pub struct Scanner<'a> {
  tokens: MultiPeek<Iter<'a, Token>>,
}

impl<'a> Scanner<'a> {
  pub fn new(tokens: MultiPeek<Iter<'a, Token>>) -> Self {
    Self { tokens: tokens }
  }

  pub fn peek(&mut self) -> Option<&Token> {
    match self.tokens.peek() {
      Some(val) => Some(*val),
      None => None,
    }
  }

  pub fn pop(&mut self, error_message: &str) -> Result<&'a Token, SyntaxError> {
    match self.tokens.next() {
      Some(val) => Ok(val),
      None => Err(SyntaxError::new_parse_error(error_message.to_string())),
    }
  }

  pub fn take(&mut self, token_type: Token, error_message: &str) -> Result<(), SyntaxError> {
    let token = self.pop(error_message)?;
    if *token == token_type {
      return Ok(());
    }
    Err(SyntaxError::new_parse_error(error_message.to_string()))
  }

  pub fn reset_peek(&mut self) {
    self.tokens.reset_peek();
  }
}

pub enum Expression {
  Const(i32),
  Unary {
    op: UnaryOperator,
    exp: Box<Expression>,
  },
  Binary {
    exp1: Box<Expression>,
    op: BinaryOperator,
    exp2: Box<Expression>,
  },
  Assign {
    name: Rc<String>,
    exp: Box<Expression>,
  },
  Var {
    name: Rc<String>,
  },
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expression::Const(val) => write!(f, "Int({})", val),
      Expression::Unary { op, exp } => write!(f, "{:?}[{}]", op, exp),
      Expression::Binary { exp1, op, exp2 } => write!(f, "{{{}}}{:?}{{{}}}", exp1, op, exp2),
      Expression::Assign { name, exp } => write!(f, "({} = [{}])", name, exp),
      Expression::Var { name } => write!(f, "({})", name),
    }
  }
}

pub enum Statement {
  Return(Expression),
  Exp(Expression),
  Declare {
    name: Rc<String>,
    exp: Option<Expression>,
  },
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Return(val) => write!(f, "RETURN {}", val),
      Statement::Declare { name, exp } => {
        if let Some(val) = exp {
          return write!(f, "INT {} = {}", name, val);
        }

        write!(f, "INT {}", name)
      }
      Self::Exp(val) => write!(f, "{}", val),
    }
  }
}

pub struct Function {
  pub name: Rc<String>,
  pub body: Vec<Statement>,
}

impl fmt::Display for Function {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "FUN INT {}:\n  params: ()\n  body:\n", self.name)?;
    for x in self.body.iter() {
      write!(f, "    {}\n", x)?;
    }
    Ok(())
  }
}

pub struct Program {
  pub func: Function,
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.func)
  }
}
