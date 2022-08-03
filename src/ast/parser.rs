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
      if let BinaryOperator::Multiplication | BinaryOperator::Division = bin_op {
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

fn parse_additive_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut term = parse_term(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if let BinaryOperator::Addition | BinaryOperator::Minus = bin_op {
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

fn parse_relational_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut additive_exp = parse_additive_expression(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if let BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual | BinaryOperator::GreaterThan | BinaryOperator::GreaterThanOrEqual = bin_op {
        tokens.pop("not expection error").unwrap();
        let next_additive_exp = parse_additive_expression(tokens)?;
        additive_exp = Expression::Binary { exp1: Box::new(additive_exp), op: bin_op, exp2: Box::new(next_additive_exp) }
      }
      else {
        break;
      }
    }
    else {
      break;
    }
  }

  Ok(additive_exp)
}

fn parse_equality_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut relational_exp = parse_relational_expression(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if let BinaryOperator::Equal | BinaryOperator::NotEqual = bin_op {
        tokens.pop("not expection error").unwrap();
        let next_relational_exp = parse_relational_expression(tokens)?;
        relational_exp = Expression::Binary { exp1: Box::new(relational_exp), op: bin_op, exp2: Box::new(next_relational_exp) }
      }
      else {
        break;
      }
    }
    else {
      break;
    }
  }

  Ok(relational_exp)
}

fn parse_logical_and_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut equality_exp = parse_equality_expression(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if BinaryOperator::And == bin_op {
        tokens.pop("not expection error").unwrap();
        let next_equality_exp = parse_equality_expression(tokens)?;
        equality_exp = Expression::Binary { exp1: Box::new(equality_exp), op: bin_op, exp2: Box::new(next_equality_exp) }
      }
      else {
        break;
      }
    }
    else {
      break;
    }
  }

  Ok(equality_exp)
}

fn parse_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let mut logical_and_exp = parse_logical_and_expression(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if BinaryOperator::Or == bin_op {
        tokens.pop("not expection error").unwrap();
        let next_logical_and_exp = parse_logical_and_expression(tokens)?;
        logical_and_exp = Expression::Binary { exp1: Box::new(logical_and_exp), op: bin_op, exp2: Box::new(next_logical_and_exp) }
      }
      else {
        break;
      }
    }
    else {
      break;
    }
  }

  Ok(logical_and_exp)
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
