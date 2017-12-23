use ::*;

pub fn derive(input: TokenStream) -> TokenStream {
    simple_derive(input, syn::parse_path("::ugli::Uniforms").unwrap(), expand)
}

fn expand(input: &DeriveInput) -> Tokens {
    match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => {
            let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
            let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
            quote! {
                    fn walk_uniforms<C>(&self, visitor: &mut C) where C: ::ugli::UniformVisitor {
                        #(visitor.visit(stringify!(#field_names_copy), &self.#field_names));*
                    }
                }
        }
        _ => panic!("ugli::Uniforms can only be derived by structs")
    }
}