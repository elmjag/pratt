use crate::{interpreter, lexer::Lexer, parser};

fn eval_string(source: &str) -> i64 {
    let mut lexer = Lexer::new(false, &String::from(source));
    let ast = parser::parse(&mut lexer);
    interpreter::eval(ast)
}

fn assert_result(source: &str, result: i64) {
    assert_eq!(eval_string(source), result);
}

#[test]
fn expressions() {
    assert_result("5\n", 5);
    assert_result("11 + 10\n", 11 + 10);
    assert_result("100 - 42\n", 100 - 42);
    assert_result("4 * 2 + 10 / 5\n", 4 * 2 + 10 / 5);

    // no trailing \n
    assert_result("5", 5);
    assert_result("4 * 2 + 10 / 5", 4 * 2 + 10 / 5);
}

#[test]
fn assignments() {
    assert_result("x <- 10\nx\n", 10);
    assert_result("a <- 42\nb <- 2\na + b\n", 44);
    assert_result("foo <- 10 / 2\nbar <- 2 * 5\nfoo + bar\n", 15);

    // variable white spaces between tokens
    assert_result("foo<-10/2\nbar       <-2*5\nfoo   +bar\n", 15);

    // no trailing \n
    assert_result("x <- 10\nx", 10);
}
