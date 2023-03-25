use api_error::ApiError;
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

#[proc_macro_attribute]
pub fn api_error(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    api_error::derive(parse_macro_input!(input as ApiError))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
