use std::error::Error;

use crate::lex::Token;
use super::model::*;

fn parse_expression(tokens: &mut Scanner) -> Result<Expression, Box<dyn Error>> {
  let token = tokens.pop("token not found")?;

  match *token {
    Token::Integer(val) => return Ok(Expression::Const(val)),
    Token::UnaryOperator(op) => {
      let inner_exp = parse_expression(tokens)?;
      return Ok(Expression::UnaryOperator { op: op, exp: Box::new(inner_exp) })
    },
    _ => return Err(Box::new(MyError("invalid token, type should be UnaryOperator | Integer")))
  }
}

fn parse_statement(tokens: &mut Scanner) -> Result<Statement, Box<dyn Error>> {
  tokens.take(Token::Return, "invalid token, type should be Return")?;

  let expression = parse_expression(tokens)?;

  tokens.take(Token::SemiColon, "invalid token, type should be SemiColon")?;

  Ok(Statement::Return(expression))
}

fn parse_function(tokens: &mut Scanner) -> Result<Function, Box<dyn Error>> {
  // int
  tokens.take(Token::Int, "invalid token, type should be Int")?;

  let func_name;
  // identifier
  let token = tokens.pop("token not found")?;
  match token {
    Token::Identifier(val) => func_name = val.clone(),
    _ => return Err(Box::new(MyError("invalid token, type should be Identifier")))
  }

  // open parenthesis
  tokens.take(Token::OpenParenthesis, "invalid token, type should be OpenParenthesis")?;

  // close parenthesis
  tokens.take(Token::CloseParenthesis, "invalid token, type should be CloseParenthesis")?;

  // open braces
  tokens.take(Token::OpenBrace, "invalid token, type should be OpenBrace")?;

  let func_body = parse_statement(tokens)?;

  // close braces
  tokens.take(Token::CloseBrace, "invalid token, type should be CloseBrace")?;

  Ok(Function{name: func_name, body: func_body})
}

pub fn parse_program(tokens: &mut Scanner) -> Result<Program, Box<dyn Error>> {
  let function = parse_function(tokens)?;

  Ok(Program{func:function})
}
