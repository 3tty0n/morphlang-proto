#![allow(unused_imports, unused_variables, dead_code)]
use std::env;
use std::fs;

pub mod ast;
pub mod codegen;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let arg = args[1].clone();
        let content = fs::read_to_string(&arg).unwrap();

        let parser = grammar::ProgramParser::new();
        let program : Vec<String> = codegen::gen_program(parser.parse(&content).unwrap());
        for p in program {
            println!("{}", p);
        }
    }
}
