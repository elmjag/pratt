mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::eval;
use lexer::Lexer;
use parser::parse;
use std::env;
use std::fs;

struct Args {
    tokens: bool,
    ast: bool,
    files: Vec<String>,
}

fn parse_args() -> Args {
    let mut tokens = false;
    let mut ast = false;
    let mut files = Vec::new();

    for argument in env::args().skip(1) {
        match argument.as_str() {
            "--ast" => ast = true,
            "--tokens" => tokens = true,
            file => files.push(String::from(file)),
        }
    }

    Args { tokens, ast, files }
}

fn process_file(print_tokens: bool, print_ast: bool, file_name: &String) {
    let text = match fs::read_to_string(file_name) {
        Err(e) => {
            eprintln!("Error reading '{file_name}': {e}");
            return;
        }
        Ok(text) => text,
    };

    println!("\n--- {file_name} ---\n");

    let mut lex = Lexer::new(print_tokens, &text);

    let exp = parse(&mut lex);
    if print_ast {
        println!("AST: {exp:?}\n");
    }

    let res = eval(exp);
    println!("Result: {res}");
}

fn main() {
    let args = parse_args();
    for file in args.files {
        process_file(args.tokens, args.ast, &file);
    }
}
