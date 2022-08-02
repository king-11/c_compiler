use crate::{lex::{Token, UnaryOperator}, utility::SyntaxError};
use super::model::*;

fn parse_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let token = tokens.pop("token not found")?;
  match *token {
    Token::Integer(val) => return Ok(Expression::Const(val)),
    Token::Negation | Token::BitwiseComplement | Token::LogicalNegation => {
      let op = UnaryOperator::try_from(token.clone())?;
      let inner_exp = parse_expression(tokens)?;
      return Ok(Expression::Unary { op: op, exp: Box::new(inner_exp) })
    },
    _ => return Err(SyntaxError::new_parse_error("invalid token, type should be UnaryOperator | Integer".to_string()))
  }
}

fn parse_statement(tokens: &mut Scanner) -> Result<Statement, SyntaxError> {
  tokens.take(Token::Return, "invalid token, type should be Return")?;

  let expression = parse_expression(tokens)?;

  tokens.take(Token::SemiColon, "invalid token, type should be SemiColon")?;

  Ok(Statement::Return(expression))
}

fn parse_function(tokens: &mut Scanner) -> Result<Function, SyntaxError> {
  // int
  tokens.take(Token::Int, "invalid token, type should be Int")?;

  let func_name;
  // identifier
  let token = tokens.pop("token not found")?;
  match token {
    Token::Identifier(val) => func_name = val.clone(),
    _ => return Err(SyntaxError::new_parse_error("invalid token, type should be Identifier".to_string()))
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

pub fn parse_program(tokens: &mut Scanner) -> Result<Program, SyntaxError> {
  let function = parse_function(tokens)?;

  Ok(Program{func:function})
}
