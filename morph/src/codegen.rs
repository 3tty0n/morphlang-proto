extern crate proc_macro2;
extern crate quote;

use std::sync::Condvar;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, IdentFragment, ToTokens};

use crate::ast::{BinOp, Expr, Function, Argument, Program};


pub fn gen_expr(expr: Box<Expr>) -> TokenStream {
    match *expr {
        Expr::Unit => quote!(),
        Expr::Number(i) => quote!(#i),
        Expr::FNumber(f) => quote!(#f),
        Expr::Variable(id) => {
            let ts_id = Ident::new(&id, Span::call_site());
            quote!(#ts_id)
        },
        Expr::BinOp(lhs, op, rhs) => {
            let lhs = gen_expr(lhs);
            let rhs = gen_expr(rhs);
            match op {
                BinOp::Plus => quote!(#lhs + #rhs),
                BinOp::Minus => quote!(#lhs - #rhs),
                BinOp::Star => quote!(#lhs * #rhs),
                BinOp::Slash => quote!(#lhs / #rhs),
                BinOp::LT => quote!(#lhs < #rhs),
                BinOp::GT => quote!(#lhs > #rhs),
                BinOp::EQ => quote!(#lhs == #rhs),
                BinOp::LE => quote!(#lhs <= #rhs),
                BinOp::GE => quote!(#lhs <= #rhs),
            }
        },
        Expr::IfElse(cond_exp, then_exp, else_exp) => {
            let cond_exp = gen_expr(cond_exp);
            let then_exp = gen_expr(then_exp);
            let else_exp = gen_expr(else_exp);
            quote! {
                if #cond_exp {
                    #then_exp
                } else {
                    #else_exp
                }
            }
        },
        Expr::Let(var, body, cont) => {
            let body_exp = gen_expr(body);
            let cont = gen_expr(cont);
            let ts_var = format_ident!("{}", &var);
            quote!(let #ts_var = #body_exp; #cont)
        },
        Expr::FunApp(var, arguments) => {
            let ident = Ident::new(&var, Span::call_site());
            let tt_arguments = gen_real_arguments(arguments);
            quote! {
                #ident(#tt_arguments)
            }
        },
        _ => unimplemented!("{} is unimplemented", format!("{:?}", expr))
    }
}

fn gen_real_arguments(arguments: Vec<Box<Expr>>) -> TokenStream {
    let mut ts = TokenStream::new();
    for argument in arguments.into_iter() {
        let tt_arg = gen_expr(argument);
        ts.extend(quote!(#tt_arg,))
    }
    ts
}

pub fn gen_function(function: Box<Function>) -> TokenStream {
    match *function {
        Function::Fun(var, arguments, body, rettype) => {
            let ts_idents = gen_arguments(arguments);
            let ts_rettype = Ident::new(&rettype, Span::call_site());
            let tt_body = gen_expr(body);
            let ident_funname = Ident::new(&var, Span::call_site());
            quote! {
                pub fn #ident_funname ( #ts_idents ) -> #ts_rettype {
                    #tt_body
                }
            }
        },
        _ => todo!()
    }
}

fn gen_arguments(arguments: Vec<Argument>) -> TokenStream {
    let mut items = TokenStream::new();
    for argument in arguments.iter() {
        match argument {
            Argument::Argument(var, typ) => {
                let ident_param = Ident::new(&var, Span::call_site());
                let ident_type = Ident::new(&typ, Span::call_site());
                if !items.is_empty() {
                    items.extend(quote!(, ).to_token_stream())
                }
                items.extend(quote!(#ident_param: #ident_type).to_token_stream())

            }
        }
    }
    items
}

pub fn gen_prog(prog: Box<Program>) -> TokenStream {
    match *prog {
        Program::Program(_, functions) => {
            println!("{}", format!("{:?}", functions));
            let mut ts_functions = TokenStream::new();
            for function in functions.into_iter() {
                let ts = gen_function(function);
                ts_functions.extend(ts)
            }
            ts_functions
        }
    }
}
