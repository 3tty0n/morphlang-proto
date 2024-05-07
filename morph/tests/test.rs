use morph::ast::{Stmt, Expr};
use morph::grammar;

#[test]
fn test_number() {
    let parser = grammar::ExprParser::new();
    assert!(parser.parse("12").is_ok());
    assert!(parser.parse(".34").is_ok());
    assert!(parser.parse("1.23").is_ok());
    assert!(parser.parse("1234.5").is_ok());
}

#[test]
fn test_let() {
    let parser = grammar::ExprParser::new();
    let expr1 = parser.parse("let x = 1 in let y = 2 in 3");
    assert!(expr1.is_ok());
    let expr2 = parser.parse("let x = 1 in let y = 2 in f(x, y)");
    assert!(expr2.is_ok());
    // println!("{}", format!("{:?}", expr2.unwrap()));
}

#[test]
fn test_if() {
    let parser = grammar::ExprParser::new();
    let expr1 = parser.parse("if x < 2 then 1 else 2");
    assert!(expr1.is_ok());
    let expr1 = parser.parse("if 3 > y then y else 32");
    assert!(expr1.is_ok());
}

#[test]
fn test_function() {
    assert!(grammar::FunctionParser::new().parse("function f(x) = let y = 1 in x + y;;").is_ok());
    assert!(grammar::FunctionParser::new().parse("function g(x, y) = x / y;;").is_ok());
}

#[test]
fn test_funapp() {
    let parser = grammar::ExprParser::new();
    let expr1 = parser.parse("f(1, 2, 3)").unwrap();
    assert_eq!("FunApp(\"f\", [Number(1), Number(2), Number(3)])", format!("{:?}", expr1));
    let expr2 = parser.parse("f(g(1))").unwrap();
    assert_eq!("FunApp(\"f\", [FunApp(\"g\", [Number(1)])])", format!("{:?}", expr2));
}

#[test]
fn test_stmt() {
    let parser = grammar::ExprParser::new();
    let expr1 = parser.parse("let n = 1 in for i = 0 to n do i + 1 done; 10");
    assert!(expr1.is_ok());
    println!("{}", format!("{:?}", expr1.unwrap()));
}

#[test]
fn test_module() {
    let parser = grammar::ModuleParser::new();
    let module1 = parser.parse("open Module1").unwrap();
    assert_eq!("Module(\"Module1\")", format!("{:?}", module1));
}

#[test]
fn test_program() {
    let parser = grammar::ProgramParser::new();
    let str1 = r###"
open Module1
open Module2

function f(x) =
  let y = x + 1 in
  y
;;

function g(y) =
  let z = 42 in
  let w = z + y in
  f(w)
;;
"###;
    let prog1 = parser.parse(str1);
    assert!(prog1.is_ok());
    println!("{}", format!("{:?}", prog1.unwrap()));
}
