use regex::Regex;

static TOK_REGEXP: &str = concat!(
    r"^(",
    r"(?<name>[a-z]+)|",
    r"(?<assign><-)|",
    r"(?<atom>\d+)|",
    r"(?<op>(\+|-|\*|/))|",
    r"(?<eol>\n)|",
    r"(?<ignore>( )+)",
    r")"
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Name(String),
    Atom(i64),
    Assign,
    Op(Op),
    Eol,
    Eof,
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}

fn str_to_op(text: &str) -> Op {
    match text {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        _ => panic!("unexpected operation '{}'", text),
    }
}

fn make_token(name: &str, value: &str) -> Token {
    match name {
        "name" => Token::Name(String::from(value)),
        "atom" => Token::Atom(value.parse().unwrap()),
        "assign" => Token::Assign,
        "op" => Token::Op(str_to_op(value)),
        "eol" => Token::Eol,
        _ => panic!("unexpected token name '{name}'"),
    }
}

fn parse(print_tokens: bool, text: &String) -> Vec<Token> {
    let re = Regex::new(TOK_REGEXP).unwrap();

    fn next_token(re: &Regex, remaining: &str) -> (Option<Token>, usize) {
        let Some(caps) = re.captures(remaining) else {
            panic!("parse error");
        };

        // figure out which named group group was matched
        for name in re.capture_names() {
            if None == name {
                continue;
            }
            let name = name.unwrap();

            if let Some(matched) = caps.name(name) {
                if name == "ignore" {
                    return (None, matched.len());
                }

                let token = make_token(name, matched.as_str());
                return (Some(token), matched.len());
            }
        }
        panic!("no named regexp groups matched")
    }

    let mut tokens = Vec::new();
    let mut unparsed = 0;

    loop {
        let (token, length) = next_token(&re, &text[unparsed..]);
        if let Some(t) = token {
            tokens.push(t);
        }
        unparsed += length;

        if unparsed >= text.len() {
            /* we have parsed the whole text */
            break;
        }
    }

    if print_tokens {
        print_toks(&tokens);
    }

    tokens.reverse();
    tokens
}

fn print_toks(tokens: &Vec<Token>) {
    print!("Tokens: ");
    for tok in tokens {
        print!("{:?} ", tok);
    }
    println!("{:?}", Token::Eof);
}

impl Lexer {
    pub fn new(print_tokens: bool, text: &String) -> Self {
        Lexer {
            tokens: parse(print_tokens, text),
        }
    }

    fn get_nth(&self, position: usize) -> Token {
        if position >= self.tokens.len() {
            return Token::Eof;
        }

        self.tokens
            .get(self.tokens.len() - position - 1)
            .cloned()
            .unwrap()
    }

    pub fn current(&self) -> Token {
        self.get_nth(0)
    }

    pub fn next(&self) -> Token {
        self.get_nth(1)
    }

    pub fn eat(&mut self, num_tokens: usize) {
        for _ in 0..num_tokens {
            self.tokens.pop();
        }
    }
}
