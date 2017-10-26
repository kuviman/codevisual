use ::*;

pub fn derive(input: TokenStream) -> TokenStream {
    simple_derive(input, syn::parse_path("::codevisual::ugli::Uniforms").unwrap(), expand)
}

fn expand(input: &DeriveInput) -> Tokens {
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