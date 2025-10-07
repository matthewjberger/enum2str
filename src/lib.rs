//! enum2str is a rust derive macro that creates Display and FromStr impls for enums.
//! This is useful for strongly typing composable sets of strings.
//! The crate is `no_std` compatible and uses `alloc` by default.
//!
//! ## Features
//!
//! - `try_from_string` (optional): Enables `TryFrom<String>` implementation for enums with only unit variants.
//!   This feature is not enabled by default and requires `std`. To enable it, use:
//!   ```toml
//!   enum2str = { version = "0.1.16", features = ["try_from_string"] }
//!   ```
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! enum2str = "0.1.16"
//! ```
//!
//! For `no_std` environments, make sure you have `alloc` available:
//!
//! ```rust
//! extern crate alloc;
//! ```

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Data, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, LitStr, parse_macro_input,
    spanned::Spanned,
};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[cfg(feature = "try_from_string")]
fn has_only_unit_variants(data: &syn::DataEnum) -> bool {
    data.variants
        .iter()
        .all(|variant| matches!(variant.fields, Fields::Unit))
}

#[cfg(feature = "try_from_string")]
fn find_duplicate_strings(data: &syn::DataEnum) -> Vec<(String, Vec<String>)> {
    let mut string_to_variants = std::collections::HashMap::new();

    for variant in data.variants.iter() {
        if let Fields::Unit = variant.fields {
            let mut string = variant.ident.to_string();
            let variant_name = variant.ident.to_string();

            // Check for enum2str attribute
            for attr in &variant.attrs {
                if attr.path.is_ident("enum2str") {
                    if let Ok(literal) = attr.parse_args::<syn::LitStr>() {
                        string = literal.value();
                    }
                }
            }

            string_to_variants
                .entry(string)
                .or_insert_with(Vec::new)
                .push(variant_name);
        }
    }

    string_to_variants
        .into_iter()
        .filter(|(_, variants)| variants.len() > 1)
        .collect()
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
    let mut from_str_arms = TokenStream2::new();

    for variant in data.variants.iter() {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                let mut display_ident = variant_name.to_string().to_token_stream();
                let mut from_str_pattern = variant_name.to_string();

                for attr in &variant.attrs {
                    if attr.path.is_ident("enum2str") && attr.path.segments.first().is_some() {
                        match attr.parse_args::<syn::LitStr>() {
                            Ok(literal) => {
                                display_ident = literal.to_token_stream();
                                from_str_pattern = literal.value();
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

                from_str_arms.extend(quote_spanned! {
                    variant.span() =>
                        s if s == #from_str_pattern => Ok(#name::#variant_name),
                });
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
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
                let mut format_ident = variant_name.to_string().to_token_stream();
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

                let field_names: Vec<_> = named.iter().map(|f| f.ident.as_ref().unwrap()).collect();

                if !field_idents.is_empty() {
                    // Use named arguments in format string
                    let arg_pattern = field_idents
                        .iter()
                        .map(|ident| quote!(#ident = #ident))
                        .collect::<Vec<_>>();

                    match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { #(#field_names),* } => write!(f, #format_ident, #(#arg_pattern),*),
                    });

                    arg_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { #(#field_names),* } => vec![#(#field_names.to_string()),*],
                    });
                } else {
                    // Just use variant name or custom string
                    match_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => write!(f, "{}", if #has_attribute { #format_ident.to_string() } else { stringify!(#variant_name).to_string() }),
                    });

                    arg_arms.extend(quote_spanned! {
                        variant.span() =>
                            #name::#variant_name { .. } => vec![],
                    });
                }

                template_arms.extend(quote_spanned! {
                    variant.span() =>
                        #name::#variant_name { .. } => #format_ident.to_string(),
                });

                variant_names.extend(quote_spanned! {
                    variant.span() =>
                        stringify!(#variant_name).to_string(),
                });

                if field_names.is_empty() && has_attribute {
                    let display_str = format_ident.to_string();
                    from_str_arms.extend(quote_spanned! {
                        variant.span() =>
                            s if s == #display_str => Ok(#name::#variant_name { }),
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

        impl ::core::str::FromStr for #name {
            type Err = ::alloc::string::String;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    #from_str_arms
                    _ => ::core::result::Result::Err(::alloc::format!("Invalid {} variant: {}", stringify!(#name), s))
                }
            }
        }

        impl #name {
            /// Get the names of this enum's variants
            pub fn variant_names() -> ::alloc::vec::Vec<::alloc::string::String> {
                ::alloc::vec![
                    #variant_names
                ]
            }

            /// Get the format specifier used to display a variant
            pub fn template(&self) -> ::alloc::string::String {
                match self {
                    #template_arms
                }
            }

            /// Gets the names of a variant's arguments
            pub fn arguments(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
                match self {
                    #arg_arms
                }
            }
        }
    };

    #[allow(unused_mut)]
    let mut expanded = TokenStream::from(expanded);

    #[cfg(feature = "try_from_string")]
    if has_only_unit_variants(&data) {
        let duplicates = find_duplicate_strings(&data);
        let has_error_variant = data.variants.iter().any(|v| v.ident == "Error");

        let try_from_impl = if has_error_variant {
            if duplicates.is_empty() {
                quote! {
                    impl std::convert::TryFrom<std::string::String> for #name {
                        type Error = std::string::String;

                        fn try_from(value: std::string::String) -> Result<Self, std::string::String> {
                            use core::str::FromStr;
                            FromStr::from_str(&value)
                        }
                    }
                }
            } else {
                let error_msg = format!(
                    "Ambiguous string representation. The following strings are used by multiple variants: {}",
                    duplicates
                        .iter()
                        .map(|(s, v)| format!("'{}' (used by {})", s, v.join(", ")))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                let duplicate_strings: Vec<_> = duplicates.iter().map(|(s, _)| s).collect();

                quote! {
                    impl std::convert::TryFrom<std::string::String> for #name {
                        type Error = std::string::String;

                        fn try_from(value: std::string::String) -> Result<Self, std::string::String> {
                            use core::str::FromStr;
                            if [#(#duplicate_strings),*].contains(&value.as_str()) {
                                return Err(#error_msg.to_string());
                            }
                            FromStr::from_str(&value)
                        }
                    }
                }
            }
        } else {
            if duplicates.is_empty() {
                quote! {
                    impl std::convert::TryFrom<std::string::String> for #name {
                        type Error = std::string::String;

                        fn try_from(value: std::string::String) -> Result<Self, Self::Error> {
                            use core::str::FromStr;
                            Self::from_str(&value)
                        }
                    }
                }
            } else {
                let error_msg = format!(
                    "Ambiguous string representation. The following strings are used by multiple variants: {}",
                    duplicates
                        .iter()
                        .map(|(s, v)| format!("'{}' (used by {})", s, v.join(", ")))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                let duplicate_strings: Vec<_> = duplicates.iter().map(|(s, _)| s).collect();

                quote! {
                    impl std::convert::TryFrom<std::string::String> for #name {
                        type Error = std::string::String;

                        fn try_from(value: std::string::String) -> Result<Self, Self::Error> {
                            use core::str::FromStr;
                            if [#(#duplicate_strings),*].contains(&value.as_str()) {
                                return Err(#error_msg.to_string());
                            }
                            Self::from_str(&value)
                        }
                    }
                }
            }
        };

        expanded.extend(TokenStream::from(try_from_impl));
    }

    expanded
}
