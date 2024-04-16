mod ast;
use ast::{Stmt, Expr};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammar);

#[test]
fn test_number() {
    let parser = grammar::ExprParser::new();
    assert!(parser.parse("12").is_ok());
    assert!(parser.parse(".34").is_ok());
    assert!(parser.parse("1.23").is_ok());
    assert!(parser.parse("1234.5").is_ok());
}

#[test]
fn test_expr() {
    let parser = grammar::ExprParser::new();
    let expr1 = parser.parse("f(1, 2, 3)");
    assert!(expr1.is_ok());
    let expr2 = parser.parse("let x = 1 in let y = 2 in 3");
    assert!(expr2.is_ok());
    let expr3 = parser.parse("let x = 1 in let y = 2 in f(x, y)");
    assert!(expr3.is_ok());
    // println!("{}", format!("{:?}", expr2.unwrap()));
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

// #[proc_macro]
// pub fn make_answer(_item: TokenStream) -> TokenStream {
//     "pub fn answer() -> u32 { 42 }".parse().unwrap()
// }

// make_answer!();

fn main() {
    println!("Hello, World!");
}
