#![allow(unused_imports)]

extern crate prelude;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

pub ( crate ) use prelude::*;
pub ( crate ) use proc_macro::TokenStream;
pub ( crate ) use quote::Tokens;
pub ( crate ) use syn::{Body, Ident, Field, VariantData, DeriveInput};

macro_rules! create_derive {
    ($fn_name:ident = $derive_name:ident: $typ:ty) => {
        #[proc_macro_derive($derive_name)]
        #[doc(hidden)]
        pub fn $fn_name(input: TokenStream) -> TokenStream {
            let s = input.to_string();
            let ast = syn::parse_derive_input(&s).unwrap();
            let input_type = &ast.ident;
            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
            let impl_body = $fn_name::expand(&ast);
            let result = quote! {
                impl#impl_generics $typ for #input_type#ty_generics #where_clause {
                    #impl_body
                }
            };
            result.parse().expect("Expanded output was no correct Rust code")
        }
    }
}

create_derive!(vertex = Vertex: ::codevisual::ugli::Vertex);
mod vertex {
    use ::*;

    pub fn expand(input: &DeriveInput) -> Tokens {
        match input.body {
            Body::Struct(VariantData::Struct(ref fields)) => {
                let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
                quote! {
                    fn walk_attributes<C>(&self, mut consumer: C) where C: ::codevisual::ugli::VertexAttributeConsumer {
                        #(consumer.consume(stringify!(#field_names_copy), &self.#field_names));*
                    }
                }
            }
            _ => panic!("ugli::Vertex can only be derived by structs")
        }
    }
}

create_derive!(uniforms = Uniforms: ::codevisual::ugli::Uniforms);
mod uniforms {
    use ::*;

    pub fn expand(input: &DeriveInput) -> Tokens {
        match input.body {
            Body::Struct(VariantData::Struct(ref fields)) => {
                let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
                quote! {
                    fn walk_uniforms<C>(&self, consumer: &mut C) where C: ::codevisual::ugli::UniformConsumer {
                        #(consumer.consume(stringify!(#field_names_copy), &self.#field_names));*
                    }
                }
            }
            _ => panic!("ugli::Uniforms can only be derived by structs")
        }
    }
}

create_derive!(shader_defines = ShaderDefines: ::codevisual::ShaderDefineStorage);
mod shader_defines {
    use ::*;

    pub fn expand(input: &DeriveInput) -> Tokens {
        match input.body {
            Body::Struct(VariantData::Struct(ref fields)) => {
                let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
                quote! {
                    fn as_glsl(&self, sources: &mut Vec<String>) {
                        #(sources.push(format!(concat!("#define ", stringify!(#field_names_copy), " {}"),
                            <::codevisual::ShaderDefine>::as_glsl(&self.#field_names))));*
                    }
                }
            }
            _ => panic!("codevisual::ShaderDefines can only be derived by structs")
        }
    }
}

#[proc_macro_derive(Resources, attributes(path))]
pub fn derive_resources(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let input = syn::parse_derive_input(&s).unwrap();
    let input_type = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
        _ => panic!("codevisual::Resources can only be derived by structs")
    };
    let field_names: Vec<_> = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect();
    let field_names = &field_names;
    let field_names_copy: Vec<_> = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let field_types = &field_types;
    let field_loaders = fields.iter().map(|field| {
        let mut path = None;
        for attr in &field.attrs {
            if let syn::MetaItem::NameValue(ref ident, ref lit) = attr.value {
                if ident == "path" {
                    assert!(path.is_none(), "Multiple paths for an asset");
                    path = Some(lit);
                }
            }
        }
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if let Some(path) = path {
            quote! {
                #field_name: <#field_type as ::codevisual::Asset>::load(loader, #path)
            }
        } else {
            quote! {
                #field_name: <#field_type as ::codevisual::ResourceContainer>::load(loader)
            }
        }
    });
    let result = quote! {
        pub struct Future {
            #(#field_names: <#field_types as ::codevisual::Resource>::Future,)*
        }
        impl ::codevisual::ResourceFuture<#input_type> for Future {
            fn unwrap(self) -> #input_type {
                #input_type {
                    #(#field_names: <#field_types as ::codevisual::Resource>::Future::unwrap(self.#field_names_copy),)*
                }
            }
        }
        impl#impl_generics ::codevisual::Resource for #input_type#ty_generics #where_clause {
            type Future = Future;
        }
        impl#impl_generics ::codevisual::ResourceContainer for #input_type#ty_generics #where_clause {
            fn load(loader: &::std::rc::Rc<::codevisual::ResourceLoader>) -> Self::Future {
                Future {
                    #(#field_loaders,)*
                }
            }
        }
    };
    result.parse().expect("Expanded output was no correct Rust code")
}