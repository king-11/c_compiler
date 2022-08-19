use std::{fmt, error::Error, sync::atomic::Ordering};

use std::sync::atomic::AtomicU64;

static mut CLAUSE_COUNT: AtomicU64 = AtomicU64::new(0);
static mut END_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn generate_clause() -> String {
    unsafe {
        let clause = format!("_clause{}", CLAUSE_COUNT.load(Ordering::Relaxed));
        CLAUSE_COUNT.fetch_add(1u64, Ordering::Relaxed);
        clause
    }
}

pub fn generate_end() -> String {
    unsafe {
        let clause = format!("_end{}", END_COUNT.load(Ordering::Relaxed));
        END_COUNT.fetch_add(1u64, Ordering::Relaxed);
        clause
    }
}

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
    pub fn new_codegen_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "CodeGen".to_string()
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

impl Error for SyntaxError {}
