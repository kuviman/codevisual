use ::*;

pub fn derive(input: TokenStream) -> TokenStream {
    simple_derive(input, syn::parse_path("::codevisual::ShaderDefineStorage").unwrap(), expand)
}

fn expand(input: &DeriveInput) -> Tokens {
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
