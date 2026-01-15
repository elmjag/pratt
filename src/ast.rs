use crate::lexer;

#[derive(Debug)]
pub enum Exp {
    Atom(i64),
    Op(lexer::Op, Box<Exp>, Box<Exp>),
}
