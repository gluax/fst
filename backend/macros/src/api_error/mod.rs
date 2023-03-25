mod kw;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, token::Struct, Attribute, FieldsNamed, Generics, Ident, Visibility};

pub struct ApiError {
    pub attrs: Vec<Attribute>,
    pub api_error: String,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}

impl Parse for ApiError {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        input.parse::<Struct>()?;
        let ident = input.parse()?;
        let generics = input.parse::<Generics>()?;
        let fields = input.parse::<FieldsNamed>()?;

        Ok(Self {
            attrs,
            api_error: "".to_string(),
            vis,
            ident,
            generics,
            fields,
        })
    }
}

pub fn derive(input: ApiError) -> syn::Result<TokenStream> {
    let attrs = &input.attrs;
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let fields = &input.fields.named.to_token_stream();

    Ok(quote! {
      #(#attrs)*
      #vis struct #ident #generics {
         #fields
      }
    })
}
