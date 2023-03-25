use darling::FromDeriveInput;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(api_error), forward_attrs(allow, doc, cfg))]
struct ApiErrorOptions {
    error: String,
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let opts = ApiErrorOptions::from_derive_input(&input)?;
    let status_error = Ident::new(&opts.error.to_ascii_lowercase(), Span::call_site());
    let status_code = Ident::new(&opts.error.to_ascii_uppercase(), Span::call_site());

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::salvo::Piece for #ident #ty_generics #where_clause {
            fn render(self, res: &mut ::salvo::Response) {
                res.set_status_error(::salvo::http::StatusError::#status_error());
                let resp = ::salvo::writer::Json(AppResponse::<(), _>::from_error(
                    ::salvo::http::StatusCode::#status_code.as_u16(),
                    self,
                ));
                res.render(resp);
            }
        }
    })
}
