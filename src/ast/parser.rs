use crate::{lex::{Token, UnaryOperator, BinaryOperator}, utility::SyntaxError};
use super::model::*;

fn parse_factor(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let token = tokens.pop("token not found")?;
  match *token {
    Token::OpenParenthesis => {
      let inner_exp = parse_expression(tokens)?;
      tokens.take(Token::CloseParenthesis, "parenthesis not balanced")?;
      return Ok(inner_exp);
    },
    Token::Negation | Token::BitwiseComplement | Token::LogicalNegation => {
      let op = UnaryOperator::try_from(token.clone())?;
      let inner_exp = parse_factor(tokens)?;
      return Ok(Expression::Unary { op: op, exp: Box::new(inner_exp) })
    },
    Token::Integer(val) => Ok(Expression::Const(val)),
    _ => Err(SyntaxError::new_parse_error("invalid tokens".to_string()))
  }
}

fn parse_term(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut factor = parse_factor(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if bin_op == BinaryOperator::Multiplication || bin_op == BinaryOperator::Division {
        tokens.pop("not expecting error here").unwrap();
        let next_factor = parse_factor(tokens)?;
        factor = Expression::Binary { exp1: Box::new(factor), op: bin_op, exp2: Box::new(next_factor) }
      }
      else {
        break;
      }
    }
    else {
      break;
    }
  }

  Ok(factor)
}

fn parse_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut term = parse_term(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if bin_op == BinaryOperator::Addition || bin_op == BinaryOperator::Minus {
        tokens.pop("not expection error").unwrap();
        let next_term = parse_term(tokens)?;
        term = Expression::Binary { exp1: Box::new(term), op: bin_op, exp2: Box::new(next_term) }
      }
      else {
        break;
      }
    }
    else {
      break;
    }
  }

  Ok(term)
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
