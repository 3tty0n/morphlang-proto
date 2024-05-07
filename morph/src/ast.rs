use std::fmt::{Debug, Formatter, Error};

#[derive(Debug)]
pub enum Bool {
    True,
    False,
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Star,
    Slash,
    LT,
    GT,
    LE,
    GE,
}

#[derive(Debug)]
pub enum Stmt {
    // for x = 0 to n do <expr> done;
    For(String, Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Unit,
    Number(i32),
    FNumber(f64),
    Variable(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    LetStmt(String, Box<Stmt>, Box<Expr>),
    FunApp(String, Vec<Box<Expr>>),
    ModFunApp(String, String, Vec<Box<Expr>>),
}

#[derive(Debug)]
pub enum Module {
    Module(String)
}

#[derive(Debug)]
pub enum Function {
    Fun(String, Vec<Box<Expr>>, Box<Expr>),
    Router(String, Vec<Box<Expr>>, Box<Expr>),
    Coordinator(String, Vec<Box<Expr>>, Box<Expr>),
}

#[derive(Debug)]
pub enum Program {
    Program(Vec<Box<Module>>, Vec<Box<Function>>)
}
