use std::{io::{Error, BufReader, BufRead}, fs::File, fmt::{Display}, env};
use regex::Regex;

#[derive(Debug)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    SemiColon,
    Int,
    Return,
    Identifier(String),
    Integer(i32)
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(val) => f.write_fmt(format_args!("Token: Integer{{{}}}", val)),
            Token::Identifier(val) => f.write_fmt(format_args!("Token: Identifier{{{}}}", val)),
            val => f.write_fmt(format_args!("Token: {:?}", val))
        }
    }
}

fn recurse_tokens(value: &str) -> Vec<Token> {
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

fn lex(filename: &str) -> Result<Vec<Token>, Error> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);
    let mut token_vector = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut tokens = recurse_tokens(&line);
        token_vector.append(&mut tokens);
    };

    Ok(token_vector)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "./data/multi_digit.c".to_string();
    let filename = args.get(1).unwrap_or(&default_filename);

    match lex(filename) {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token);
            }
        },
        Err(e) => {
            println!("error occured: {}", e);
        }
    }
}
