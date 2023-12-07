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
    let expr2 = parser.parse("let x = 1 + 2;").unwrap();
    assert_eq!("Assign(\"x\", Number(1))", format!("{:?}", expr1));
    // println!("{}", format!("{:?}", expr2));
    assert_eq!("Assign(\"x\", BinOp(Number(1), Plus, Number(2)))", format!("{:?}", expr2));
}

#[test]
fn test_function() {
    assert!(grammar::FunctionParser::new().parse("function f(x) { return 1; }").is_ok());
}

#[test]
fn test_program() {
    let parser = grammar::ProgramParser::new();
    let str1 = r###"
function f(x) {
  return x - 1;
}

function g(x, y) {
  return x + y;
}
"###;
    let prog1 = parser.parse(str1);
    assert!(prog1.is_ok());
    println!("{}", format!("{:?}", prog1.unwrap()));
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
