use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => derive_struct(&input, fields),
        _ => Err(Error::new(
            Span::call_site(),
            "Only structs with named fields are supported",
        )),
    }
}

pub fn derive_struct(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream> {
    let attrs = &input.attrs;
    let vis = &input.vis;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // let dummy = Ident::new(&format!("_IMPL_APIERROR_FOR_{}", ident), Span::call_site());
    
    let field_name = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let field_ty = fields.named.iter().map(|f| &f.ty);
    // let field_str = fields
    //     .named
    //     .iter()
    //     .map(attr::name_of_field)
    //     .collect::<Result<Vec<_>>>()?;

    Ok(quote!{
        #(#attrs)*
        #vis struct #ident #ty_generics #where_clause {
            #(pub #field_name: #field_ty)*
        }

        
    })
}
