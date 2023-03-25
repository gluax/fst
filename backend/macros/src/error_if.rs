use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    Ok(quote! {
      impl ErrorIf for #ident {
        fn error_or_ok(self) -> Result<()> {
            if self == Self::default() {
                Ok(())
            } else {
                Err(Error::Auth(Auth::Login(self)))
            }
        }
      }
    })
}
