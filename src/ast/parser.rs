use std::rc::Rc;

use super::model::*;
use crate::{
  lex::{BinaryOperator, Token, UnaryOperator},
  utility::SyntaxError,
};

fn parse_factor(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let token = tokens.pop("token not found for factor")?;
  match token {
    Token::OpenParenthesis => {
      let inner_exp = parse_expression(tokens)?;
      tokens.take(Token::CloseParenthesis, "parenthesis not balanced")?;
      return Ok(inner_exp);
    }
    Token::Negation | Token::BitwiseComplement | Token::LogicalNegation => {
      let op = UnaryOperator::try_from(token.clone())?;
      let inner_exp = parse_factor(tokens)?;
      return Ok(Expression::Unary {
        op: op,
        exp: Box::new(inner_exp),
      });
    }
    Token::Identifier(val) => Ok(Expression::Var {
      name: Rc::clone(val),
    }),
    Token::Integer(val) => Ok(Expression::Const(*val)),
    _ => Err(SyntaxError::new_parse_error("invalid tokens for factor".to_string())),
  }
}

fn parse_sub_function(
  tokens: &mut Scanner,
  sub_exp: fn(&mut Scanner) -> Result<Expression, SyntaxError>,
  operators: &[BinaryOperator],
) -> Result<Expression, SyntaxError> {
  let mut exp = sub_exp(tokens)?;

  while let Some(val) = tokens.peek() {
    if let Ok(bin_op) = BinaryOperator::try_from(val.clone()) {
      if operators.contains(&bin_op) {
        // not expecting error here
        tokens.pop("").unwrap();
        let next_exp = sub_exp(tokens)?;
        exp = Expression::Binary {
          exp1: Box::new(exp),
          op: bin_op,
          exp2: Box::new(next_exp),
        }
      } else {
        break;
      }
    } else {
      break;
    }
  }
  tokens.reset_peek();

  Ok(exp)
}

fn parse_term(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  parse_sub_function(
    tokens,
    parse_factor,
    &[BinaryOperator::Multiplication, BinaryOperator::Division],
  )
}

fn parse_additive_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  parse_sub_function(
    tokens,
    parse_term,
    &[BinaryOperator::Addition, BinaryOperator::Minus],
  )
}

fn parse_relational_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  parse_sub_function(
    tokens,
    parse_additive_expression,
    &[
      BinaryOperator::LessThan,
      BinaryOperator::LessThanOrEqual,
      BinaryOperator::GreaterThan,
      BinaryOperator::GreaterThanOrEqual,
    ],
  )
}

fn parse_equality_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  parse_sub_function(
    tokens,
    parse_relational_expression,
    &[BinaryOperator::Equal, BinaryOperator::NotEqual],
  )
}

fn parse_logical_and_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  parse_sub_function(tokens, parse_equality_expression, &[BinaryOperator::And])
}

fn parse_expression(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  if let Some(val) = tokens.peek() {
    if let Token::Identifier(id) = val {
      let identifier = Rc::clone(id);

      if let Some(Token::Assignment) = tokens.peek() {
        // only pop identifier if assignment token found
        tokens.pop("")?;
        tokens.take(
          Token::Assignment,
          "expecting an assignment operator over here",
        )?;

        let inner_exp = parse_expression(tokens)?;
        return Ok(Expression::Assign {
          name: identifier,
          exp: Box::new(inner_exp),
        });
      }
    }

    tokens.reset_peek();
    return parse_sub_function(tokens, parse_logical_and_expression, &[BinaryOperator::Or]);
  }

  Err(SyntaxError::new_parse_error("expected tokens for expression".to_string()))
}

fn parse_return_statement(tokens: &mut Scanner) -> Result<Statement, SyntaxError> {
  tokens.take(Token::Return, "expected return token")?;
  let expression = parse_expression(tokens)?;
  tokens.take(Token::SemiColon, "invalid token, type should be SemiColon")?;
  Ok(Statement::Return(expression))
}

fn parse_int_declaration_statement(tokens: &mut Scanner) -> Result<Statement, SyntaxError> {
  tokens.take(Token::Int, "expected int token")?;
  let token = tokens.pop("token not found")?;

  if let Token::Identifier(identifier_name) = token {
    let exp = if let Some(Token::Assignment) = tokens.peek() {
      tokens.take(Token::Assignment, "")?;
      Some(parse_expression(tokens)?)
    } else {
      None
    };

    tokens.take(
      Token::SemiColon,
      "expecting a semicolon at end of declaration",
    )?;

    return Ok(Statement::Declare {
      name: Rc::clone(identifier_name),
      exp: exp,
    });
  }

  Err(SyntaxError::new_parse_error(
    "expected a identifier".to_string(),
  ))
}

fn parse_statement(tokens: &mut Scanner) -> Result<Statement, SyntaxError> {
  if let Some(val) = tokens.peek() {
    return match val {
      Token::Return => parse_return_statement(tokens),
      Token::Int => parse_int_declaration_statement(tokens),
      _ => {
        tokens.reset_peek();
        let exp = parse_expression(tokens)?;
        tokens.take(Token::SemiColon, "expecting a semi colon")?;
        Ok(Statement::Exp(exp))
      }
    }
  }

  Err(SyntaxError::new_parse_error(
    "expected tokens for statement".to_string(),
  ))
}

fn parse_function(tokens: &mut Scanner) -> Result<Function, SyntaxError> {
  // int
  tokens.take(Token::Int, "invalid token, type should be Int")?;

  // identifier
  let token = tokens.pop("token not found")?;
  let func_name = if let Token::Identifier(val) = token {
    Rc::clone(val)
  } else {
    return Err(SyntaxError::new_parse_error(
    "invalid token, type should be Identifier".to_string(),
    ))
  };

  // open parenthesis
  tokens.take(
    Token::OpenParenthesis,
    "invalid token, type should be OpenParenthesis",
  )?;

  // close parenthesis
  tokens.take(
    Token::CloseParenthesis,
    "invalid token, type should be CloseParenthesis",
  )?;

  // open braces
  tokens.take(Token::OpenBrace, "invalid token, type should be OpenBrace")?;

  let mut statements: Vec<Statement> = vec![];
  while let Some(val) = tokens.peek() {
    if *val != Token::CloseBrace {
      tokens.reset_peek();
      let func_body = parse_statement(tokens)?;
      statements.push(func_body);
    }
  }

  // close braces
  tokens.take(
    Token::CloseBrace,
    "invalid token, type should be CloseBrace",
  )?;

  Ok(Function {
    name: func_name,
    body: statements,
  })
}

pub fn parse_program(tokens: &mut Scanner) -> Result<Program, SyntaxError> {
  let function = parse_function(tokens)?;

  Ok(Program { func: function })
}
