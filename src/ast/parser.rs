use std::error::Error;
use std::slice::Iter;

use crate::lex::Token;
use super::model::*;

fn get_token_or_error<'a>(tokens: &'a mut Iter<Token>, error_message: &'static str) -> Result<&'a Token, Box<dyn Error>> {
  match tokens.next() {
    Some(val) => Ok(val),
    None => Err(Box::new(MyError(error_message)))
  }
}

fn match_token_or_error(token_value: &Token, token_type: Token, error_message: &'static str) -> Result<(), Box<dyn Error>> {
  if std::mem::discriminant(token_value) == std::mem::discriminant(&token_type) { Ok(()) } else { Err(Box::new(MyError(error_message))) }
}

fn parse_expression(tokens: &mut Iter<Token>) -> Result<Expression, Box<dyn Error>> {
  let token = get_token_or_error(tokens, "token not found")?;

  match token {
    Token::Integer(val) => return Ok(Expression::Const(*val)),
    _ => return Err(Box::new(MyError("invalid token, type should be Integer")))
  }
}

fn parse_statement(tokens: &mut Iter<Token>) -> Result<Statement, Box<dyn Error>> {
  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(token, Token::Return, "invalid token, type should be Return")?;

  let expression = parse_expression(tokens)?;

  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(token, Token::SemiColon, "invalid token, type should be SemiColon")?;

  Ok(Statement::Return(expression))
}

fn parse_function(tokens: &mut Iter<Token>) -> Result<Function, Box<dyn Error>> {
  // int
  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(&token, Token::Int, "invalid token, type should be Int")?;

  let func_name;
  // identifier
  let token = get_token_or_error(tokens, "token not found")?;
  match token {
    Token::Identifier(val) => func_name = val.clone(),
    _ => return Err(Box::new(MyError("invalid token, type should be Identifier")))
  }

  // open parenthesis
  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(&token, Token::OpenParenthesis, "invalid token, type should be OpenParenthesis")?;

  // close parenthesis
  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(&token, Token::CloseParenthesis, "invalid token, type should be CloseParenthesis")?;

  // open braces
  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(&token, Token::OpenBrace, "invalid token, type should be OpenBrace")?;

  let func_body = parse_statement(tokens)?;

  // close braces
  let token = get_token_or_error(tokens, "token not found")?;
  match_token_or_error(&token, Token::CloseBrace, "invalid token, type should be CloseBrace")?;

  Ok(Function{name: func_name, body: func_body})
}

pub fn parse_program(tokens: &mut Iter<Token>) -> Result<Program, Box<dyn Error>> {
  let function = parse_function(tokens)?;

  Ok(Program{func:function})
}
