use ::*;

pub fn derive(input: TokenStream) -> TokenStream {
    simple_derive(input, syn::parse_path("::ugli::Vertex").unwrap(), expand)
}

pub fn expand(input: &DeriveInput) -> Tokens {
    match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => {
            let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
            let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
            quote! {
                fn walk_attributes<C>(&self, mut visitor: C)
                    where C: ::ugli::VertexAttributeVisitor {
                    #(visitor.visit(stringify!(#field_names_copy), &self.#field_names));*
                }
            }
        }
        _ => panic!("ugli::Vertex can only be derived by structs"),
    }
}
