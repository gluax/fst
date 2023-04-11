use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod validate;

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    validate::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
