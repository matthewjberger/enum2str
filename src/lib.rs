use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, FieldsNamed,
    FieldsUnnamed, LitStr,
};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(EnumStr, attributes(enum2str))]
pub fn derive_enum2str(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let data = match input.data {
        Data::Enum(data) => data,
        _ => return derive_error!("enum2str only supports enums"),
    };

    let mut match_arms = TokenStream2::new();
    let mut variant_names = TokenStream2::new();
    let mut template_arms = TokenStream2::new();
    let mut arg_arms = TokenStream2::new();

    for variant in data.variants.iter() {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                let mut display_ident = variant_name.to_string().to_token_stream();

                for attr in &variant.attrs {
                    if attr.path.is_ident("enum2str") && attr.path.segments.first().is_some() {
                        match attr.parse_args::<syn::LitStr>() {
                            Ok(literal) => {
                                display_ident = literal.to_token_stream();
                            }
                            Err(_) => {
                                return derive_error!(
                                    r#"The 'enum2str' attribute is missing a String argument. Example: #[enum2str("Listening on: {} {}")] "#
                                );
                            }
                        }
                    }
                }

                match_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name =>  write!(f, "{}", #display_ident),
                });

                template_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name => #display_ident.to_string(),
                });

                variant_names.extend(quote_spanned! {
                    variant.span() =>
                        stringify!(#variant_name).to_string(),
                });

                arg_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name => vec![],
                });
            }
            Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
                let mut format_ident = "{}".to_string().to_token_stream();

                for attr in &variant.attrs {
                    if attr.path.is_ident("enum2str") && attr.path.segments.first().is_some() {
                        match attr.parse_args::<LitStr>() {
                            Ok(literal) => format_ident = literal.to_token_stream(),
                            Err(_) => {
                                return derive_error!(
                                    r#"The 'enum2str' attribute is missing a String argument. Example: #[enum2str("Listening on: {} {}")] "#
                                );
                            }
                        }
                    }
                }

                if format_ident.to_string().contains("{}") {
                    let fields = unnamed.iter().len();
                    let args = ('a'..='z')
                        .take(fields)
                        .map(|letter| Ident::new(&letter.to_string(), variant.span()))
                        .collect::<Vec<_>>();
                    match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name(#(#args),*) => write!(f, #format_ident, #(#args),*),
                    });

                    template_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name(..) => #format_ident.to_string(),
                    });

                    variant_names.extend(quote_spanned! {
                        variant.span() =>
                            stringify!(#variant_name).to_string(),
                    });

                    arg_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name(#(#args),*) => vec![#(#args.to_string()),*],
                    });
                } else {
                    match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name(..) => write!(f, #format_ident),
                    });

                    variant_names.extend(quote_spanned! {
                        variant.span() =>
                            stringify!(#variant_name).to_string(),
                    });

                    template_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name(..) => #format_ident.to_string(),
                    });

                    arg_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name(..) => vec![],
                    });
                }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let mut format_ident = "{}".to_string().to_token_stream();
                let mut field_idents = Vec::new();

                let mut has_attribute = false;
                for attr in &variant.attrs {
                    if attr.path.is_ident("enum2str") {
                        has_attribute = true;
                        match attr.parse_args::<LitStr>() {
                            Ok(literal) => {
                                format_ident = literal.clone().to_token_stream();
                                let literal_str = literal.value().clone();
                                let mut start_indices =
                                    literal_str.match_indices('{').map(|(i, _)| i);
                                let mut end_indices =
                                    literal_str.match_indices('}').map(|(i, _)| i);

                                while let (Some(start), Some(end)) =
                                    (start_indices.next(), end_indices.next())
                                {
                                    let field_name = &literal_str[(start + 1)..end];
                                    field_idents.push(Ident::new(field_name, Span::call_site()));
                                }
                            }
                            Err(_) => {
                                return derive_error!(
                                    r#"The 'enum2str' attribute is missing a String argument. Example: #[enum2str("Listening on: {} {}")] "#
                                );
                            }
                        }
                    }
                }

                let field_names = named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                if !field_idents.is_empty() {
                    let arg_pattern = field_idents
                        .iter()
                        .map(|ident| quote!(#ident = #ident))
                        .collect::<Vec<_>>();

                    match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { #(#field_names),* } => write!(f, #format_ident, #(#arg_pattern),*),
                    });

                    template_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => #format_ident.to_string(),
                    });

                    variant_names.extend(quote_spanned! {
                        variant.span() =>
                            stringify!(#variant_name).to_string(),
                    });

                    arg_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { #(#field_names),* } => vec![#(#field_idents.to_string()),*],
                    });
                } else {
                    if has_attribute {
                        match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => write!(f, "{}", #format_ident.to_string()),
                    });
                    } else {
                        match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => write!(f, "{}", stringify!(#variant_name)),
                    });
                    }

                    if has_attribute {
                        template_arms.extend(quote_spanned! {
                            variant.span() =>
                                #name::#variant_name { .. } => #format_ident.to_string(),
                        });
                    } else {
                        template_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => stringify!(#variant_name).to_string(),
                        });
                    }

                    arg_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => vec![],
                    });

                    variant_names.extend(quote_spanned! {
                        variant.span() =>
                            stringify!(#variant_name).to_string(),
                    });
                }
            }
        };
    }

    let expanded = quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    #match_arms
                }
            }
        }

        impl #name {
            pub fn variant_names() -> Vec<String> {
                vec![
                    #variant_names
                ]
            }

            pub fn template(&self) -> String {
                match self {
                    #template_arms
                }
            }

            pub fn arguments(&self) -> Vec<String> {
                match self {
                    #arg_arms
                }
            }
        }
    };

    TokenStream::from(expanded)
}
