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
pub(crate) use syn::{Body, DeriveInput, Field, Ident, VariantData};

mod resources;

#[proc_macro_derive(Resources, attributes(path))]
pub fn derive_resources(input: TokenStream) -> TokenStream {
    resources::derive(input)
}

mod settings;

#[proc_macro_derive(Settings, attributes(setting))]
pub fn derive_settings(input: TokenStream) -> TokenStream {
    settings::derive(input)
}

mod shader_defines;

#[proc_macro_derive(ShaderDefines)]
pub fn derive_shader_defines(input: TokenStream) -> TokenStream {
    shader_defines::derive(input)
}

fn simple_derive(
    input: TokenStream,
    typ: syn::Path,
    expand: fn(&DeriveInput) -> Tokens,
) -> TokenStream {
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
    result
        .parse()
        .expect("Expanded output was no correct Rust code")
}
