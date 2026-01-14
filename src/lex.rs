use regex::Regex;

static TOK_REGEXP: &str = r"^(?<atom>\d+)|(?<op>(\+|-|\*|/))|(?<ignore>(\s|\n)+)";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Atom(&'a str),
    Op(&'a str),
    Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
}

fn make_token<'a>(name: &str, value: &'a str) -> Token<'a> {
    match name {
        "atom" => Token::Atom(value),
        "op" => Token::Op(value),
        _ => panic!("unexpected token name"),
    }
}

fn parse<'a>(text: &'a String) -> Vec<Token<'a>> {
    let re = Regex::new(TOK_REGEXP).unwrap();

    fn next_token<'a>(re: &Regex, remaining: &'a str) -> (Option<Token<'a>>, usize) {
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

    tokens.reverse();
    tokens
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a String) -> Self {
        Lexer {
            tokens: parse(text),
        }
    }

    pub fn next(&mut self) -> Token<'a> {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token<'a> {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}
