use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, GenericArgument, Meta,
    Path, PathArguments, Type,
};

const OMIT_ATTRIBUTE: &str = "omit";

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
        if should_omit(&field) {
            continue;
        }

        let name = &field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let function = match option_inner_type(ty) {
            Some(option_type) => quote! {
                pub fn #name(mut self, value: #option_type) -> Self {
                    self.#name = Some(value);
                    self
                }
            },
            None => quote! {
                pub fn #name(mut self, value: #ty) -> Self {
                    self.#name = value;
                    self
                }
            },
        };

        functions.extend(function);
    }

    quote! {
        impl #generics #name #generics #where_clause {
            #functions
        }
    }
    .into()
}

fn should_omit(field: &Field) -> bool {
    has_attribute(OMIT_ATTRIBUTE, field)
}

fn has_attribute(name: &str, field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attribute| match attribute.parse_meta().unwrap() {
            Meta::Path(path) => {
                path.get_ident().map(|ident| ident.to_string()) == Some(name.to_string())
            }
            _ => false,
        })
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
            let type_params = &typepath.path.segments.first().unwrap().arguments;
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => return None,
            };
            match generic_arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None,
            }
        }
        _ => None,
    }
}

fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Option"
}
