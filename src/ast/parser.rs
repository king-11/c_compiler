use std::rc::Rc;

use super::model::*;
use crate::{
  lex::{BinaryOperator, Token, UnaryOperator},
  utility::SyntaxError,
};

fn parse_factor(tokens: &mut Scanner) -> Result<Expression, SyntaxError> {
  let token = tokens.pop("token not found")?;
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
    _ => Err(SyntaxError::new_parse_error("invalid tokens".to_string())),
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
    match val {
      Token::Identifier(id) => {
        let identifier = Rc::clone(id);

        if let Some(assign) = tokens.peek() {
          match assign {
            Token::Assignment => {
              // not expecting an error here
              tokens.pop("")?;

              // not expecting an error here
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
            _ => (),
          }
        }
      }
      _ => (),
    }
  }
  tokens.reset_peek();
  parse_sub_function(tokens, parse_logical_and_expression, &[BinaryOperator::Or])
}

fn parse_statement(tokens: &mut Scanner) -> Result<Statement, SyntaxError> {
  if let Some(val) = tokens.peek() {
    match val {
      Token::Return => {
        // not expecting error here
        tokens.take(Token::Return, "")?;
        let expression = parse_expression(tokens)?;

        tokens.take(Token::SemiColon, "invalid token, type should be SemiColon")?;

        return Ok(Statement::Return(expression));
      }
      Token::Int => {
        // not expecting error here
        tokens.take(Token::Int, "")?;

        let identifier = tokens.pop("indetifier not found")?;
        match identifier {
          Token::Identifier(identifier_name) => {
            let mut exp: Option<Expression> = None;
            if let Some(assign) = tokens.peek() {
              match assign {
                Token::Assignment => {
                  // not expecting error here
                  tokens.take(Token::Assignment, "")?;
                  exp = Some(parse_expression(tokens)?);
                }
                _ => (),
              }
            }
            tokens.reset_peek();
            tokens.take(
              Token::SemiColon,
              "expecting a semicolon at end of declaration",
            )?;
            return Ok(Statement::Declare {
              name: Rc::clone(identifier_name),
              exp: exp,
            });
          }
          _ => {
            return Err(SyntaxError::new_parse_error(
              "expected a identifier".to_string(),
            ))
          }
        }
      }
      _ => (),
    }
  }
  tokens.reset_peek();
  let exp = parse_expression(tokens)?;
  tokens.take(Token::SemiColon, "expecting a semi colon")?;
  Ok(Statement::Exp(exp))
}

fn parse_function(tokens: &mut Scanner) -> Result<Function, SyntaxError> {
  // int
  tokens.take(Token::Int, "invalid token, type should be Int")?;

  let func_name;
  // identifier
  let token = tokens.pop("token not found")?;
  match token {
    Token::Identifier(val) => func_name = Rc::clone(val),
    _ => {
      return Err(SyntaxError::new_parse_error(
        "invalid token, type should be Identifier".to_string(),
      ))
    }
  }

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
