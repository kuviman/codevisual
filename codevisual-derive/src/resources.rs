use ::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let input = syn::parse_derive_input(&s).unwrap();
    let input_type = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
        _ => panic!("codevisual::Resources can only be derived by structs"),
    };
    let field_names: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_names = &field_names;
    let field_names_copy: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
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
    let future = Ident::new(String::from(input_type.as_ref()) + "Future");
    let result = quote! {
        pub struct #future {
            #(#field_names: <#field_types as ::codevisual::Resource>::Future,)*
        }
        impl ::codevisual::ResourceFuture<#input_type> for #future {
            fn unwrap(self) -> #input_type {
                use ::codevisual::ResourceFuture;
                #input_type {
                    #(#field_names: self.#field_names_copy.unwrap(),)*
                }
            }
        }
        impl#impl_generics ::codevisual::Resource for #input_type#ty_generics #where_clause {
            type Future = #future;
        }
        impl#impl_generics ::codevisual::ResourceContainer
            for #input_type#ty_generics #where_clause {
            fn load(loader: &::std::rc::Rc<::codevisual::ResourceLoader>) -> Self::Future {
                Self::Future {
                    #(#field_loaders,)*
                }
            }
        }
    };
    result
        .parse()
        .expect("Expanded output was no correct Rust code")
}
