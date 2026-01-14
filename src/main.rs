mod lex;
use lex::Lexer;
use std::env;
use std::fs;

fn process_file(file_name: &String) {
    let text = match fs::read_to_string(file_name) {
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
        Ok(text) => text,
    };

    let mut lex = Lexer::new(&text);
    loop {
        let next = lex.next();
        let peek = lex.peek();

        println!("peek {peek:?} next {next:?}");
        if next == lex::Token::Eof {
            break;
        }
    }
}

fn main() {
    for argument in env::args().skip(1) {
        process_file(&argument);
    }
}
