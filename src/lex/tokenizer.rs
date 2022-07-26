use regex::Regex;

use super::model::Token;

pub fn recurse_tokens(value: &str) -> Vec<Token> {
  if value.is_empty() {
      return vec![];
  }

  let mut tokens = Vec::new();
  let mut end_idx = 0;
  let byte_array = value.as_bytes();
  while end_idx < byte_array.len() {
      if value.get(end_idx..end_idx+3) == Some("int") {
          tokens.push(Token::Int);
          end_idx += 3;
          continue;
      }

      if value.get(end_idx..end_idx+6) == Some("return") {
          tokens.push(Token::Return);
          end_idx += 6;
          continue;
      }

      let number_regex = Regex::new("^\\d+").unwrap();
      if let Some(val) = number_regex.find(&value[end_idx..]) {
          let result = val.as_str().parse().unwrap();
          tokens.push(Token::Integer(result));
          end_idx += val.end();
          continue;
      }

      let identifier_regex = Regex::new("^[a-zA-Z]\\w*").unwrap();
      if let Some(val) = identifier_regex.find(&value[end_idx..]) {
          tokens.push(Token::Identifier(val.as_str().to_string()));
          end_idx += val.end();
          continue;
      }

      let val = &byte_array[end_idx];
      let simple_token = match *val as char {
          '{' => Token::OpenBrace,
          '}' => Token::CloseBrace,
          '(' => Token::OpenParenthesis,
          ')' => Token::CloseParenthesis,
          ';' => Token::SemiColon,
          val => if val.is_whitespace() { end_idx += 1; continue } else { break }
      };
      end_idx += 1;
      tokens.push(simple_token);
  }

  tokens
}
