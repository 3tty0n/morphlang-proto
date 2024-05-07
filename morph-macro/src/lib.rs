extern crate morph;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, LitStr};
use quote::quote;

use morph::grammar;

#[proc_macro]
pub fn morph(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let input_str_val = input_str.value();

    let parser = grammar::ProgramParser::new();
    let prog = parser.parse(&input_str_val);

    // TODO: convert prog into rust function
    quote! {
        let x = 1;
    }.into()
}
