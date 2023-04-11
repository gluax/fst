use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(validate), forward_attrs(allow, doc, cfg))]
struct ValidateOptions {
    data: ast::Data<util::Ignored, ValidateFieldOptions>,
}

#[derive(FromField)]
#[darling(attributes(validate), forward_attrs(allow, doc, cfg))]
struct ValidateFieldOptions {
    ident: Option<Ident>,
    with: Ident,
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let opts = ValidateOptions::from_derive_input(&input)?;

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = opts
        .data
        .take_struct()
        // TODO error
        .expect("Should never be enum")
        .fields;

    let error_fields = fields
        .into_iter()
        .map(|f| {
            let ident = f.ident.unwrap();
            let with = f.with;

            quote! {
                #with(&self.#ident)?;
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
      impl #impl_generics crate::utils::Validate for #ident #ty_generics #where_clause {

        fn validate(self) -> ::salvo::Result<()> {
            #(#error_fields)*

            Ok(())
        }
      }
    })
}
