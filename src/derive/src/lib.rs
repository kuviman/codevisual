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
        let field_name = data.fields()
            .into_iter()
            .map(|field| field.ident.as_ref().unwrap());
        let field_name2 = data.fields()
            .into_iter()
            .map(|field| field.ident.as_ref().unwrap());
        quote!{
            impl ::codevisual::draw::vertex::Data for #name {
                fn walk_attributes<F>(&self, f: &mut F) where F: ::codevisual::draw::vertex::AttributeConsumer {
                    #(f.consume(stringify!(#field_name), &self.#field_name));*
                }
            }
        }
    } else {
        panic!("Can only be implemented for structs");
    }
}