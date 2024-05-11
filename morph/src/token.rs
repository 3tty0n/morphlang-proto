extern crate syn;
extern crate proc_macro2;
extern crate litrs;

use proc_macro2::TokenTree;
use proc_macro2::TokenTree::{Group, Ident, Punct, Literal};
use quote::ToTokens;
use litrs::Literal as Ltrl;

pub enum MorphToken {
    Var(String),
    Num(i32),
    FNum(f64),
    String(String),
    Function,
    If,
    Then,
    Else,
    For,
    To,
    Do,
    Done,
    True,
    False,
    Let,
    In,
    Eq,
    Lt,
    Gt,
    Plus,
    Minus,
    Slash,
    Star,
    Lparen,
    Rparen,
    SemiColon,
    SemiSemi,
    RustTokenTree(TokenTree)
}

impl MorphToken {
    pub fn from_rust_token(tt: TokenTree) -> MorphToken {
        match tt {
            Group(group) => unimplemented!(),
            Ident(ref id) => {
                let id_str: &str = &id.to_string();
                match id_str {
                    "if" => MorphToken::If,
                    "then" =>  MorphToken::Then,
                    "else" => MorphToken::Else,
                    "for" => MorphToken::For,
                    "to" =>  MorphToken::To,
                    "do" =>  MorphToken::Do,
                    "done" =>  MorphToken::Done,
                    "true" =>  MorphToken::True,
                    "false" =>  MorphToken::False,
                    "let" =>  MorphToken::Let,
                    "in" =>  MorphToken::In,
                    "Eq" =>  MorphToken::Eq,
                    "Lt" =>  MorphToken::Lt,
                    "Gt" =>  MorphToken::Gt,
                    "(" =>   MorphToken::Lparen,
                    ")" =>   MorphToken::Rparen,
                    _ => MorphToken::Var(id.to_string())
                }
            },
            Punct(punct) => {
                match punct.as_char() {
                    '+' => MorphToken::Plus,
                    '-' => MorphToken::Minus,
                    '/' => MorphToken::Slash,
                    '*' => MorphToken::Star,
                    ';' => MorphToken::SemiColon,
                    _ => unreachable!("unreachable punct {}", punct.as_char())
                }
            },
            Literal(ref lit) => {
                match Ltrl::try_from(tt.clone()) {
                    Err(e) => panic!("Invalid token: {}", format!("{}", tt.clone())),
                    Ok(Ltrl::Integer(i)) => {
                        let num = i.into_raw_input().parse::<i32>();
                        match num {
                            Ok(i) => MorphToken::Num(i),
                            Err(e) => panic!("Parse int error: {}", format!("{}", tt)),
                        }
                    },
                    Ok(Ltrl::Float(f)) => {
                        let num = f.into_raw_input().parse::<f64>();
                        match num {
                            Ok(f) => MorphToken::FNum(f),
                            Err(e) => panic!("Parse float error: {}", format!("{}", tt)),
                        }
                    },
                    Ok(Ltrl::Bool(b)) => {
                        if b.value() {
                            MorphToken::True
                        } else {
                            MorphToken::False
                        }
                    },
                    Ok(Ltrl::Byte(b)) => {
                        MorphToken::String(b.to_string())
                    },
                    Ok(Ltrl::ByteString(b)) => {
                        MorphToken::String(b.to_string())
                    },
                    Ok(Ltrl::String(b)) => {
                        MorphToken::String(b.to_string())
                    },
                    Ok(Ltrl::Char(c)) => {
                        MorphToken::String(c.to_string())
                    },
                }
            }
        }
    }
}
