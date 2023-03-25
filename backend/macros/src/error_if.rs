use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
      impl #impl_generics crate::utils::ErrorIf for #ident #ty_generics #where_clause {
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
