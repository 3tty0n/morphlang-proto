#[derive(Debug)]
pub enum Expr {
    Literal(f64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, one_of},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

fn parse_literal(input: &str) -> IResult<&str, Expr> {
    map(digit1, |s: &str| {
        Expr::Literal(s.parse::<f64>().unwrap())
    })(input)
}

fn parse_variable(input: &str) -> IResult<&str, Expr> {
    map(alpha1, |s: &str| {
        Expr::Variable(s.to_string())
    })(input)
}


fn parse_parenthesized(input: &str) -> IResult<&str, Expr> {
    delimited(
        preceded(multispace0, tag("(")),
        parse_expr,
        preceded(multispace0, tag(")")),
    )(input)
}

fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    map(
        pair(
            alpha1,
            delimited(
                preceded(multispace0, tag("(")),
                separated_list0(preceded(multispace0, tag(",")), parse_expr),
                preceded(multispace0, tag(")")),
            ),
        ),
        |(name, args)| Expr::FunctionCall(name.to_string(), args)
    )(input)
}

fn parse_atom(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_literal,
        parse_variable,
        parse_parenthesized,
        parse_function_call,
    ))(input)
}

fn parse_operator(input: &str) -> IResult<&str, char> {
    preceded(multispace0, one_of("+-*/"))(input)
}


pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_atom(input)?;
    fold_expression(init, input)
}

pub fn fold_expression(init: Expr, input: &str) -> IResult<&str, Expr> {
    let (input, operations) = nom::multi::many0( pair(parse_operator, parse_atom))(input)?;
    Ok((input, operations.into_iter().fold(init, |acc, (op, atom)| {
        match op {
            '+' => Expr::Add(Box::new(acc), Box::new(atom)),
            '-' => Expr::Sub(Box::new(acc), Box::new(atom)),
            '*' => Expr::Mul(Box::new(acc), Box::new(atom)),
            '/' => Expr::Div(Box::new(acc), Box::new(atom)),
            _ => unreachable!()
        }
    })))
}

use std::collections::HashMap;

pub type Function = fn(&[f64]) -> f64;

pub struct Evaluator<'a> {
    variables: &'a HashMap<String, f64>,
    functions: &'a HashMap<String, Function>,
}

impl<'a> Evaluator<'a> {
    pub fn new(
        variables: &'a HashMap<String, f64>,
        functions: &'a HashMap<String, Function>,
    ) -> Self {
        Evaluator { variables, functions }
    }

    pub fn eval(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Literal(value) => *value,
            Expr::Variable(name) => *self.variables.get(name).unwrap(),
            Expr::Add(left, right) => self.eval(left) + self.eval(right),
            Expr::Sub(left, right) => self.eval(left) - self.eval(right),
            Expr::Mul(left, right) => self.eval(left) * self.eval(right),
            Expr::Div(left, right) => self.eval(left) / self.eval(right),
            Expr::FunctionCall(name, args) => {
                let function = self.functions.get(name).unwrap();
                let arg_values: Vec<f64> = args.iter().map(|arg| self.eval(arg)).collect();
                function(&arg_values)
            }
        }
    }
}
