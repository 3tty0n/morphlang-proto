extern crate morph;
extern crate proc_macro;
mod codegen;

use proc_macro::TokenStream;
use codegen::generate;

#[proc_macro]
pub fn morph(input: TokenStream) -> TokenStream {
    generate(input)
}
