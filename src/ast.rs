use std::fmt::{Debug, Formatter, Error};

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Box<Expr>),
    Return(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    FNumber(f64),
    Variable(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
    FunApp(String, Vec<Box<Expr>>),
}

#[derive(Debug)]
pub enum Function {
    Fun(String, Vec<Box<Expr>>, Box<Stmt>),
}

pub type Program = Vec<Box<Function>>;
