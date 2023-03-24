use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, FieldsUnnamed, LitStr,
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
    let mut template_arms = TokenStream2::new();
    let mut arg_arms = TokenStream2::new();

    for variant in data.variants.iter() {
        let variant_name = &variant.ident;

        match variant.fields {
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

                arg_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name => 0,
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

                arg_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name(..) => #fields,
                });
            }
            _ => {
                return derive_error!(
                    "enum2str is only implemented for unit and unnamed-field enums"
                )
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
            pub fn template(&self) -> String {
                match self {
                    #template_arms
                }
            }

            pub fn number_of_args(&self) -> usize {
                match self {
                    #arg_arms
                }
            }
        }
    };

    TokenStream::from(expanded)
}
