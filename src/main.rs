use crate::ast::{Stmt, Expr};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_number() {
    assert!(grammar::TermParser::new().parse("12").is_ok());
    assert!(grammar::TermParser::new().parse(".34").is_ok());
    assert!(grammar::TermParser::new().parse("1.23").is_ok());
    assert!(grammar::TermParser::new().parse("1234.5").is_ok());
}

#[test]
fn test_stmt() {
    let parser = grammar::StmtParser::new();
    let expr1 = parser.parse("let x = 1;").unwrap();
    assert_eq!("Assign(\"x\", Number(1))", format!("{:?}", expr1));
}

#[test]
fn test_function() {
    assert!(grammar::FunctionParser::new().parse("function f(x) = 1;").is_ok());
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
