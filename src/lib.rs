use proc_macro::TokenStream;

use quote::quote;
use syn::{LitInt, parse_macro_input};

#[proc_macro]
pub fn minus_one(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let literal: LitInt = parse_macro_input!(input);

    // Convert the literal to i64, subtract 1, and create a new literal
    let value = literal.base10_parse::<i64>().unwrap() - 1;
    let new_literal = LitInt::new(&value.to_string(), literal.span());

    // Generate the final token stream
    let expanded = quote! {
        #new_literal
    };

    // Return the generated tokens
    TokenStream::from(expanded)
}