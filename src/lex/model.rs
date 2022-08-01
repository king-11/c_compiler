use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Negation,
    BitwiseComplement,
    LogicalNegation,
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
    Identifier(String),
    Integer(i32),
    UnaryOperator(UnaryOperator),
}

impl Token {
    pub fn keywords(value: &str) -> Option<Self> {
        match value {
            "int" => Some(Self::Int),
            "return" => Some(Self::Return),
            _ => None
        }
    }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          Token::Integer(val) => f.write_fmt(format_args!("Token: Integer{{{}}}", val)),
          Token::Identifier(val) => f.write_fmt(format_args!("Token: Identifier{{{}}}", val)),
          val => f.write_fmt(format_args!("Token: {:?}", val))
      }
  }
}
