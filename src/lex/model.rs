use std::{fmt::Display, rc::Rc};

use crate::utility::SyntaxError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
  Negation,
  BitwiseComplement,
  LogicalNegation,
}

impl TryFrom<Token> for UnaryOperator {
  type Error = SyntaxError;

  fn try_from(value: Token) -> Result<Self, Self::Error> {
    match value {
      Token::Negation => Ok(Self::Negation),
      Token::BitwiseComplement => Ok(Self::BitwiseComplement),
      Token::LogicalNegation => Ok(Self::LogicalNegation),
      _ => Err(SyntaxError::new_lex_error(
        "Can only convert unary operators".to_string(),
      )),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
  Addition,
  Minus,
  Multiplication,
  Division,
  And,
  Or,
  Equal,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
}

impl TryFrom<Token> for BinaryOperator {
  type Error = SyntaxError;

  fn try_from(value: Token) -> Result<Self, Self::Error> {
    match value {
      Token::Addition => Ok(Self::Addition),
      Token::Negation => Ok(Self::Minus),
      Token::Multiplication => Ok(Self::Multiplication),
      Token::Division => Ok(Self::Division),
      Token::And => Ok(Self::And),
      Token::Or => Ok(Self::Or),
      Token::Equal => Ok(Self::Equal),
      Token::NotEqual => Ok(Self::NotEqual),
      Token::LessThan => Ok(Self::LessThan),
      Token::LessThanOrEqual => Ok(Self::LessThanOrEqual),
      Token::GreaterThan => Ok(Self::GreaterThan),
      Token::GreaterThanOrEqual => Ok(Self::GreaterThanOrEqual),
      _ => Err(SyntaxError::new_lex_error(
        "Can only convert binary operators".to_string(),
      )),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  OpenBrace,
  CloseBrace,
  OpenParenthesis,
  CloseParenthesis,
  SemiColon,
  Int,
  Return,
  Identifier(Rc<String>),
  Integer(i32),
  Negation,
  BitwiseComplement,
  LogicalNegation,
  Addition,
  Multiplication,
  Division,
  And,
  Or,
  Equal,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
  Assignment,
}

impl Token {
  pub fn keywords(value: &str) -> Option<Self> {
    match value {
      "int" => Some(Self::Int),
      "return" => Some(Self::Return),
      _ => None,
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Token::Integer(val) => f.write_fmt(format_args!("Token: Integer{{{}}}", val)),
      Token::Identifier(val) => f.write_fmt(format_args!("Token: Identifier{{{}}}", val)),
      val => f.write_fmt(format_args!("Token: {:?}", val)),
    }
  }
}
