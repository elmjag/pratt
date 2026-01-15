use crate::ast::Exp;
use crate::lexer::Op;

fn bin_op(op: Op, l: Box<Exp>, r: Box<Exp>) -> i64 {
    let lval = expression(*l);
    let rval = expression(*r);

    match op {
        Op::Add => lval + rval,
        Op::Sub => lval - rval,
        Op::Mul => lval * rval,
        Op::Div => lval / rval,
    }
}

fn expression(exp: Exp) -> i64 {
    match exp {
        Exp::Atom(val) => val,
        Exp::Op(op, l, r) => bin_op(op, l, r),
    }
}

pub fn eval(exp: Exp) -> i64 {
    expression(exp)
}
