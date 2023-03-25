use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod api_error;
mod error_if;

#[proc_macro_derive(ErrorIf)]
pub fn derive_error_if(input: TokenStream) -> TokenStream {
    error_if::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ApiError, attributes(api_error))]
pub fn api_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    api_error::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
