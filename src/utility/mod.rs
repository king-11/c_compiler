use std::{fmt, error::Error};

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    level: String,
}

impl SyntaxError {
    pub fn new_lex_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Lex".to_string(),
        }
    }
    pub fn new_parse_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Parse".to_string(),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

impl Error for SyntaxError {}
