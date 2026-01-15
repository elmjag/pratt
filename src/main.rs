mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::eval;
use lexer::Lexer;
use parser::parse;
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
    let exp = parse(&mut lex);
    println!("ast: {exp:?}");
    let res = eval(exp);
    println!("Result: {res}");
}

fn main() {
    for argument in env::args().skip(1) {
        process_file(&argument);
    }
}
