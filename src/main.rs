mod ast;
use ast::{Stmt, Expr};

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(grammar);


#[test]
fn test_number() {
    assert!(grammar::TermParser::new().parse("12").is_ok());
    assert!(grammar::TermParser::new().parse(".34").is_ok());
    assert!(grammar::TermParser::new().parse("1.23").is_ok());
    assert!(grammar::TermParser::new().parse("1234.5").is_ok());
}

#[test]
fn test_expr() {
    let parser = grammar::ExprParser::new();
    let expr1 = parser.parse("f(1, 2, 3)");
    assert!(expr1.is_ok());
}

#[test]
fn test_stmt() {
    let parser = grammar::StmtParser::new();
    let stmt1 = parser.parse("let x = 1;").unwrap();
    let stmt2 = parser.parse("let x = 1 + 2;").unwrap();
    assert_eq!("Assign(\"x\", Number(1))", format!("{:?}", stmt1));
    // println!("{}", format!("{:?}", expr2));
    assert_eq!("Assign(\"x\", BinOp(Number(1), Plus, Number(2)))", format!("{:?}", stmt2));
}

#[test]
fn test_function() {
    assert!(grammar::FunctionParser::new().parse("function f(x) = return 1; ;;").is_ok());
    assert!(grammar::FunctionParser::new().parse("function g(x, y) = return x * y; ;;").is_ok());
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
fn test_program() {
    let parser = grammar::ProgramParser::new();
    let str1 = r###"
function f(x) =
  return x - 1;
;;

function g(x, y) =
  let z = f(1);
  return z + y;
;;
"###;
    let prog1 = parser.parse(str1);
    assert!(prog1.is_ok());
    // println!("{}", format!("{:?}", prog1.unwrap()));
}


// #[proc_macro]
// pub fn make_answer(_item: TokenStream) -> TokenStream {
//     "pub fn answer() -> u32 { 42 }".parse().unwrap()
// }

// make_answer!();

fn main() {
    println!("Hello, World!");
}
