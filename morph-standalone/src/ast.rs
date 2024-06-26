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
    EQ,
}

#[derive(Debug)]
pub enum Stmt {
    // for x = 0 to n do <expr> done;
    For(String, Box<Expr>, Box<Expr>, Box<Expr>),
    // if cond then e1 else e2
    IfElse(Box<Expr>, Box<Stmt>, Box<Stmt>),
    // let x = 1; cont
    Let(String, Box<Expr>, Box<Stmt>),
    Return(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Unit,
    Number(i32),
    FNumber(f64),
    Variable(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    FunApp(String, Vec<Box<Expr>>),
    ModFunApp(String, String, Vec<Box<Expr>>),
}

#[derive(Debug)]
pub enum Match {
  Match(Box<Expr>, Vec<Box<Pattern>>) // Send, Receive, etc.
}

#[derive(Debug)]
pub enum Pattern {
    Send(Box<Expr>, Box<Stmt>),
    Receive(Box<Expr>, Box<Stmt>)
}

#[derive(Debug)]
pub enum Module {
    Module(String)
}

#[derive(Debug)]
pub enum Function {
    Fun(String, Vec<Argument>, Box<Expr>, String),
    Router(String, Vec<Argument>, Box<Match>, String),
    Coordinator(String, Vec<Argument>, Box<Match>, String),
}

#[derive(Debug)]
pub enum Argument {
    Argument(String, String)
}

#[derive(Debug)]
pub enum Program {
    Program(Vec<Box<Module>>, Vec<Box<Function>>)
}
