extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Vertex)]
pub fn vertex(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_vertex(&ast);
    gen.parse().unwrap()
}

fn impl_vertex(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(ref data) = ast.body {
        let field_name = data.fields().into_iter().map(|field| {
            field.ident.as_ref().unwrap()
        });
        let field_name2 = data.fields().into_iter().map(|field| {
            field.ident.as_ref().unwrap()
        });
        quote! {
            impl ::codevisual::ugli::VertexData for #name {
                fn walk_attributes<C>(&self, mut consumer: C) where C: ::codevisual::ugli::VertexAttributeConsumer {
                    #(consumer.consume(stringify!(#field_name2), &self.#field_name));*
                }
            }
        }
    } else {
        panic!("Can only be implemented for structs");
    }
}

#[proc_macro_derive(Uniforms)]
pub fn uniforms(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_uniforms(&ast);
    gen.parse().unwrap()
}

fn impl_uniforms(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(ref data) = ast.body {
        let field_name = data.fields().into_iter().map(|field| {
            field.ident.as_ref().unwrap()
        });
        let field_name2 = data.fields().into_iter().map(|field| {
            field.ident.as_ref().unwrap()
        });
        quote! {
            impl ::codevisual::ugli::UniformStorage for #name {
                fn walk_uniforms<C>(&self, consumer: &mut C) where C: ::codevisual::ugli::UniformConsumer {
                    #(consumer.consume(stringify!(#field_name2), &self.#field_name));*
                }
            }
        }
    } else {
        panic!("Can only be implemented for structs");
    }
}

#[proc_macro_derive(Defines)]
pub fn defines(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_defines(&ast);
    gen.parse().unwrap()
}

fn impl_defines(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(ref data) = ast.body {
        let field_name = data.fields().into_iter().map(|field| {
            field.ident.as_ref().unwrap()
        });
        let field_name2 = data.fields().into_iter().map(|field| {
            field.ident.as_ref().unwrap()
        });
        quote! {
            impl ::codevisual::ShaderDefineStorage for #name {
                fn as_glsl(&self, sources: &mut Vec<String>) {
                    #(sources.push(format!(concat!("#define ", stringify!(#field_name2), " {}"),
                        <::codevisual::ShaderDefine>::as_glsl(&self.#field_name))));*
                }
            }
        }
    } else {
        panic!("Can only be implemented for structs");
    }
}
