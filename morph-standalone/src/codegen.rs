use crate::ast::{BinOp, Expr, Stmt, Match, Pattern, Function, Argument, Program};

pub fn gen_expr(expr: Box<Expr>) -> String {
    match *expr {
        Expr::Unit => "".to_string(),
        Expr::Number(i) => i.to_string(),
        Expr::FNumber(f) => f.to_string(),
        Expr::Variable(v) => v.to_string(),
        Expr::BinOp(lhs, op, rhs) => {
            let lhs = gen_expr(lhs);
            let rhs = gen_expr(rhs);
            match op {
                BinOp::Plus => format!("{lhs} + {rhs}"),
                BinOp::Minus => format!("{lhs} - {rhs}"),
                BinOp::Star => format!("{lhs} * {rhs}"),
                BinOp::Slash => format!("{lhs} / {rhs}"),
                BinOp::LT => format!("{lhs} < {rhs}"),
                BinOp::GT => format!("{lhs} < {rhs}"),
                BinOp::EQ => format!("{lhs} == {rhs}"),
                BinOp::LE => format!("{lhs} <= {rhs}"),
                BinOp::GE => format!("{lhs} >= {rhs}"),
            }
        },
        Expr::ModFunApp(modid, funid, args) => {
            let module = alternate_modid(modid);
            let function = alternate_funid(funid);
            format!(r#"{module}.{function}(..);
"#)
        },
        _ => todo!(),
    }

}

fn alternate_modid(modid: String) -> String {
    // TODO: tell using module(s) to compiler
    if modid == "Display" {
        "myTFT"
    } else {
        panic!("Only Display is supported.")
    }.to_string()
}

fn alternate_funid(funid: String) -> String {
    if funid == "rotate" {
        "setRotation"
    } else if funid == "fill_screen" {
        "filLScreen"
    } else {
        panic!("Only rotate and fill_screen is supported")
    }.to_string()
}

pub fn gen_stmt(stmt: Box<Stmt>) -> String {
    match *stmt {
        Stmt::Return(e) => {
            match *e {
                Expr::FunApp(..) => format!("{}", gen_expr(e)),
                Expr::ModFunApp(..) => format!("{}", gen_expr(e)),
                _ => format!("return {};", gen_expr(e))
            }
        },
        Stmt::Let(var, body, cont) => {
            let body_exp = gen_expr(body);
            let cont = gen_stmt(cont);
            let ts_var = format!("{}", &var);
            format!("int {} = {};\n{}", &ts_var, &body_exp, &cont)
        },
        Stmt::IfElse(cond_exp, then_exp, else_exp) => {
            let cond_exp = gen_expr(cond_exp);
            let then_exp = gen_stmt(then_exp);
            let else_exp = gen_stmt(else_exp);
            format!(r#"if ({cond_exp}) {{
  {then_exp}
}} else {{
  {else_exp}
}}
"#)
        },
        _ => todo!(),
    }

}

pub fn gen_pattern(pattern: Box<Pattern>) -> String {
    match *pattern {
        Pattern::Send(e, stmt) => gen_stmt(stmt),
        Pattern::Receive(e, stmt) => gen_stmt(stmt),
    }
}

pub fn gen_function(function: Box<Function>) -> String {
    match *function {
        Function::Router(id, args, mtch, rettype) => {
            match *mtch {
                Match::Match(e, mut patterns) => {
                    // TODO
                    // index 0: Send
                    // index 1: Receive
                    let send = patterns.pop().unwrap();
                    let receive = patterns.pop().unwrap();
                    let send_str = gen_pattern(send);
                    let receive_str = gen_pattern(receive);
                    format!(r#"#if !COMM_ROOT_NODE
    // send action
    {send_str}

    // receive action
    {receive_str}
#endif
"#)
                },
            }
        },
        Function::Coordinator(id, args, mtch, rettype) => {
            match *mtch {
                Match::Match(e, mut patterns) => {
                    // TODO
                    // index 0: Send
                    // index 1: Receive
                    let send = patterns.pop().unwrap();
                    let receive = patterns.pop().unwrap();
                    let send_str = gen_pattern(send);
                    let receive_str = gen_pattern(receive);
                    format!(r#"
#if COMM_ROOT_NODE
    // send action
    {send_str}

    // receive action
    {receive_str}
#endif
"#)
                },
            }
        },
        _ => todo!(),
    }
}

pub fn gen_program(p: Box<Program>) -> Vec<String> {
    match *p {
        Program::Program(_, functions) => {
            let mut result = Vec::new();
            for function in functions {
                result.push(gen_function(function))
            }
            result
        }
    }

}
