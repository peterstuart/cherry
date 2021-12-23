use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DataStruct, DeriveInput, Fields, Meta};

#[proc_macro_derive(Builder, attributes(omit))]
pub fn derive_builder(token_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token_stream as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Builder functions can only be derived for structs with named fields"),
    };

    let mut functions = quote!();

    for field in fields {
        let name = &field.ident;
        let ty = &field.ty;

        let omit = field
            .attrs
            .iter()
            .any(|attribute| match attribute.parse_meta().unwrap() {
                Meta::Path(path) => {
                    path.get_ident().map(|ident| ident.to_string()) == Some("omit".to_string())
                }
                _ => false,
            });

        if !omit {
            functions.extend(quote! {
                pub fn #name(mut self, value: #ty) -> Self {
                    self.#name = value;
                    self
                }
            });
        }
    }

    quote! {
        impl #generics #name #generics #where_clause {
            #functions
        }
    }
    .into()
}
