#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(trace_macros)]

mod parser;
mod sexp;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let expr = parser::sexp::parse_expr(&input.to_string()).unwrap().1;
    
    let exp = quote! {
        #expr
    };
    
    TokenStream::from(exp)
}
