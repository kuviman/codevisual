use ::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let input = syn::parse_derive_input(&s).unwrap();
    let input_type = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
        _ => panic!("codevisual::Settings can only be derived by structs")
    };
    let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
    fn find_attr(field: &Field, name: &str, parse: bool) -> Option<Tokens> {
        let name_to_find = name;
        for attr in &field.attrs {
            if let syn::MetaItem::List(ref name, ref list) = attr.value {
                if name == "setting" {
                    for attr in list {
                        if let syn::NestedMetaItem::MetaItem(ref attr) = *attr {
                            if let syn::MetaItem::NameValue(ref name, ref value) = *attr {
                                if name == name_to_find {
                                    if parse {
                                        if let syn::Lit::Str(ref value, _) = *value {
                                            let value = syn::parse_expr(value).unwrap();
                                            return Some(quote!(#value));
                                        }
                                    } else {
                                        return Some(quote!(#value));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
    let field_defaults = fields.iter().map(|field| find_attr(field, "default", true).expect("Default value is not provided"));
    let field_settings = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let name = syn::Lit::from(field_name.as_ref());
        let mut name: Tokens = quote!(#name);
        if let Some(name_override) = find_attr(field, "name", false) {
            name = name_override;
        }
        let default = find_attr(field, "default", true).expect("Default value is not provided");
        if let Some(range) = find_attr(field, "range", true) {
            quote! {{
                let settings = settings.clone();
                ::codevisual::Setting::create_range(
                    #name, #default, #range, move |value| {
                        settings.borrow_mut().#field_name = value;
                    }
                )
            }}
        } else {
            quote! {{
                let settings = settings.clone();
                ::codevisual::Setting::Bool {
                    name: String::from(#name),
                    default: #default,
                    setter: Box::new(move |value| {
                        settings.borrow_mut().#field_name = value;
                    }),
                }
            }}
        }
    });
    let result = quote! {
        impl#impl_generics ::codevisual::Settings for #input_type#ty_generics #where_clause {
            fn register(app: &::codevisual::Application) -> ::std::rc::Rc<::std::cell::RefCell<Self>> {
                let settings = ::std::rc::Rc::new(::std::cell::RefCell::new(
                    Self {
                        #(#field_names: #field_defaults,)*
                    }));
                #(app.add_setting(#field_settings);)*
                settings
            }
        }
    };
    result.parse().expect("Expanded output was no correct Rust code")
}