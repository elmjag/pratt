use crate::ast::Exp;
use crate::lexer::Token;
use crate::lexer::{Lexer, Op};

fn infix_binding_power(op: Op) -> (u8, u8) {
    match op {
        Op::Add | Op::Sub => (1, 2),
        Op::Mul | Op::Div => (3, 4),
    }
}

fn expr(lexer: &mut Lexer, min_bp: u8) -> Exp {
    let mut lhs = match lexer.next() {
        Token::Atom(val) => Exp::Atom(val),
        t => panic!("unexpected token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("unexpected token: {:?}", t),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }

        lexer.next();
        let rhs = expr(lexer, r_bp);

        lhs = Exp::Op(op, Box::new(lhs), Box::new(rhs));
    }

    lhs
}

pub fn parse(lexer: &mut Lexer) -> Exp {
    expr(lexer, 0)
}
