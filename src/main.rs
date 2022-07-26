use std::{io::{Error, BufReader, BufRead}, fs::File, env};

mod lex;

fn lex(filename: &str) -> Result<Vec<lex::Token>, Error> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);
    let mut token_vector = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut tokens = lex::recurse_tokens(&line);
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
