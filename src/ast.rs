use crate::lexer;

#[derive(Debug)]
pub struct Program {
    pub assignments: Vec<Assign>,
    pub result: Exp,
}

#[derive(Debug)]
pub struct Assign {
    pub name: String,
    pub expr: Exp,
}

#[derive(Debug)]
pub enum Exp {
    Atom(i64),
    Var(String),
    Op(lexer::Op, Box<Exp>, Box<Exp>),
}
