extern crate morph;

use proc_macro::TokenStream;
use syn::{parse, parse_macro_input, LitStr, DeriveInput};

use morph::grammar;
use morph::codegen;

pub fn generate(input: TokenStream) -> TokenStream {
    let prog = input.to_string();
    println!("Target : {}", &prog);

    let parser = grammar::ProgramParser::new();
    let prog = parser.parse(&prog).unwrap();
    println!("Generated AST: {}", format!("{:?}", prog));

    let ts = codegen::gen_prog(prog);
    println!("\nGenerated TokenStream: {}", format!("{:?}", &ts));
    ts.into()
}
