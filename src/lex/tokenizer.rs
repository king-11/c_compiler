use regex::Regex;

use super::model::Token;

pub fn string_tokenizer(value: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut end_idx = 0;
  let byte_array = value.as_bytes();
  while end_idx < byte_array.len() {
    let literal_regex = Regex::new("^\\w+").unwrap();

    if let Some(val) = literal_regex.find(&value[end_idx..]) {
        let number_regex = Regex::new("^\\d+").unwrap();
        if let Some(token_val) = Token::keywords(val.as_str()) {
            tokens.push(token_val);
        }
        else if let Some(val) = number_regex.find(&value[end_idx..]) {
            let result = val.as_str().parse().unwrap();
            tokens.push(Token::Integer(result));
        }
        else {
            tokens.push(Token::Identifier(val.as_str().to_string()));
        }

        end_idx += val.end();
        continue;
    };

    let val = &byte_array[end_idx];
    let simple_token = match *val as char {
        '{' => Token::OpenBrace,
        '}' => Token::CloseBrace,
        '(' => Token::OpenParenthesis,
        ')' => Token::CloseParenthesis,
        ';' => Token::SemiColon,
        '-' => Token::Negation,
        '~' => Token::BitwiseComplement,
        '!' => Token::LogicalNegation,
        val => if val.is_whitespace() { end_idx += 1; continue } else { break }
    };
    end_idx += 1;
    tokens.push(simple_token);
  }

  tokens
}
