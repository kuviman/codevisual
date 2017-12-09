#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate prelude;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

pub(crate) use prelude::*;
pub(crate) use proc_macro::TokenStream;
pub(crate) use quote::Tokens;
pub(crate) use syn::{Body, Ident, Field, VariantData, DeriveInput};

mod uniforms;

#[proc_macro_derive(Uniforms)]
pub fn derive_uniforms(input: TokenStream) -> TokenStream {
    uniforms::derive(input)
}

mod vertex;

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    vertex::derive(input)
}

fn simple_derive(input: TokenStream, typ: syn::Path, expand: fn(&DeriveInput) -> Tokens) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impl_body = expand(&ast);
    let result = quote! {
        impl#impl_generics #typ for #input_type#ty_generics #where_clause {
            #impl_body
        }
    };
    result.parse().expect("Expanded output was no correct Rust code")
}