use std::collections::HashMap;

use crate::ast::{Exp, Program};
use crate::lexer::Op;

fn bin_op(op: Op, l: Box<Exp>, r: Box<Exp>, variables: &HashMap<String, i64>) -> i64 {
    let lval = expression(*l, variables);
    let rval = expression(*r, variables);

    match op {
        Op::Add => lval + rval,
        Op::Sub => lval - rval,
        Op::Mul => lval * rval,
        Op::Div => lval / rval,
    }
}

fn variable_value(name: String, variables: &HashMap<String, i64>) -> i64 {
    match variables.get(&name) {
        Some(v) => *v,
        None => panic!("unknown variable: {name}"),
    }
}

fn expression(exp: Exp, variables: &HashMap<String, i64>) -> i64 {
    match exp {
        Exp::Atom(val) => val,
        Exp::Var(name) => variable_value(name, variables),
        Exp::Op(op, l, r) => bin_op(op, l, r, variables),
    }
}

fn program(ast: Program) -> i64 {
    let mut variables = HashMap::new();

    for assignment in ast.assignments {
        let res = expression(assignment.expr, &variables);
        variables.insert(assignment.name, res);
    }

    expression(ast.result, &variables)
}

pub fn eval(ast: Program) -> i64 {
    program(ast)
}
