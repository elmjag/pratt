use crate::ast::{Assign, Exp, Program};
use crate::lexer::Token;
use crate::lexer::{Lexer, Op};

fn infix_binding_power(op: Op) -> (u8, u8) {
    match op {
        Op::Add | Op::Sub => (1, 2),
        Op::Mul | Op::Div => (3, 4),
    }
}

fn expr(lexer: &mut Lexer, min_bp: u8) -> Exp {
    let mut lhs = match lexer.current() {
        Token::Atom(val) => Exp::Atom(val),
        Token::Name(name) => Exp::Var(name),
        t => panic!("unexpected token: {:?}", t),
    };
    lexer.eat(1);

    loop {
        let op = match lexer.current() {
            Token::Eol | Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("unexpected token: {:?}", t),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }

        lexer.eat(1);
        let rhs = expr(lexer, r_bp);

        lhs = Exp::Op(op, Box::new(lhs), Box::new(rhs));
    }

    lhs
}

fn assignment(lexer: &mut Lexer) -> Assign {
    assert!(lexer.next() == Token::Assign);

    let Token::Name(name) = lexer.current() else {
        panic!("unxpected token")
    };
    lexer.eat(2);

    let expr = expr(lexer, 0);

    Assign { name, expr }
}

fn program(lexer: &mut Lexer) -> Program {
    let mut assignments = vec![];

    while lexer.next() == Token::Assign {
        assignments.push(assignment(lexer));
        assert!(lexer.current() == Token::Eol);
        lexer.eat(1);
    }

    let result = expr(lexer, 0);
    Program {
        assignments,
        result,
    }
}

pub fn parse(lexer: &mut Lexer) -> Program {
    program(lexer)
}
