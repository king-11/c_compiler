use std::rc::Rc;

use regex::Regex;
use tracing::trace;

use crate::utility::SyntaxError;

use super::model::Token;

pub fn parse_literal_token(value: &str) -> (Option<Token>, usize) {
    let literal_regex = Regex::new("^\\w+").unwrap();
    let number_regex = Regex::new("^\\d+").unwrap();

    match literal_regex.find(value) {
        Some(literal_match) => {
            let token = if let Some(keyword) = Token::keywords(literal_match.as_str()) {
                keyword
            } else if let Some(number_match) = number_regex.find(literal_match.as_str()) {
                Token::Integer(number_match.as_str().parse().expect("integer parse"))
            } else {
                Token::Identifier(Rc::new(literal_match.as_str().to_owned()))
            };

            (Some(token), literal_match.end())
        }
        None => (None, 0),
    }
}

pub fn parse_compound_token(value: Option<&str>) -> (Option<Token>, usize) {
    return match value {
        Some(compound_value) => {
            let compound_token = match compound_value {
                "&&" => Token::And,
                "||" => Token::Or,
                "==" => Token::Equal,
                "!=" => Token::NotEqual,
                "<=" => Token::LessThanOrEqual,
                ">=" => Token::GreaterThanOrEqual,
                _ => return (None, 0),
            };

            (Some(compound_token), 2)
        }
        None => (None, 0),
    };
}

pub fn try_parse_simple_token(value: char) -> Result<(Option<Token>, usize), SyntaxError> {
    if value.is_whitespace() {
        return Ok((None, 1));
    }

    let token = match value {
        '{' => Token::OpenBrace,
        '}' => Token::CloseBrace,
        '(' => Token::OpenParenthesis,
        ')' => Token::CloseParenthesis,
        ';' => Token::SemiColon,
        '-' => Token::Negation,
        '~' => Token::BitwiseComplement,
        '!' => Token::LogicalNegation,
        '+' => Token::Addition,
        '*' => Token::Multiplication,
        '/' => Token::Division,
        '<' => Token::LessThan,
        '>' => Token::GreaterThan,
        '=' => Token::Assignment,
        val => {
            return Err(SyntaxError::new_lex_error(format!(
                "unidentifiable character {}",
                val
            )))
        }
    };

    return Ok((Some(token), 1));
}

pub fn string_tokenizer(value: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::new();
    let mut end_idx = 0;
    let byte_array = value.as_bytes();
    while end_idx < value.len() {
        if let (Some(literal_value), increment) = parse_literal_token(&value[end_idx..]) {
            trace!(
                "literal match for token {} end index is now {}",
                literal_value,
                end_idx + increment
            );
            tokens.push(literal_value);
            end_idx += increment;
        } else if let (Some(compound_value), increment) =
            parse_compound_token(value.get(end_idx..end_idx + 2))
        {
            trace!(
                "compound match for token {} end index is now {}",
                compound_value,
                end_idx
            );
            tokens.push(compound_value);
            end_idx += increment;
        } else {
            let (simple_token, increment) = try_parse_simple_token(byte_array[end_idx] as char)?;
            trace!(
                "simple match for token {:?} end index is now {}",
                simple_token,
                end_idx
            );
            if let Some(simple_value) = simple_token {
                tokens.push(simple_value);
            }
            end_idx += increment;
        }
    }

    Ok(tokens)
}
