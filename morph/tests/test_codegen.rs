use morph::ast::{BinOp, Expr, Function, Argument};
use morph::codegen;

#[test]
fn test_gen_expr() {
    let input = Box::new(Expr::BinOp(
        Box::new(Expr::Number(1)), BinOp::Plus, Box::new(Expr::Number(2))));
    parse_exp_and_print(input);

    let input = Box::new(
        Expr::IfElse(
            Box::new(
                Expr::BinOp(
                    Box::new(Expr::Number(1)),
                    BinOp::LT,
                    Box::new(Expr::Number(2))
                )
            ),
            Box::new(Expr::Variable(String::from("x"))),
            Box::new(Expr::Variable(String::from("y"))))
    );
    parse_exp_and_print(input);

    let input = Box::new(
        Expr::Let(
            String::from("x"),
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        )
    );
    parse_exp_and_print(input)
}

#[test]
fn test_gen_function() {
    let add_1_to_2 = Box::new(Expr::BinOp(
        Box::new(Expr::Number(1)), BinOp::Plus, Box::new(Expr::Number(2))));
    let input = Box::new(
        Function::Fun(
            String::from("f"),
            vec![Argument::Argument(String::from("x"), String::from("i32"))],
            add_1_to_2,
            String::from("i32")
        )
    );
    let ts = codegen::gen_function(input);
    println!("{}", format!("{:?}", &ts));
}


fn parse_exp_and_print(input: Box<Expr>) {
    let ts = codegen::gen_expr(input);
    println!("{}", format!("{:?}", &ts));
}
