use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Token, parse_macro_input};

struct EnumInput {
    enum_name: Ident,
    variant_prefix: Ident,
    constant_name: Ident,
    count: LitInt,
}

impl Parse for EnumInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let variant_prefix: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let constant_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let count: LitInt = input.parse()?;

        Ok(EnumInput {
            enum_name,
            variant_prefix,
            constant_name,
            count,
        })
    }
}

#[proc_macro]
pub fn generate_enum(input: TokenStream) -> TokenStream {
    let EnumInput {
        enum_name,
        variant_prefix,
        constant_name,
        count,
    } = parse_macro_input!(input as EnumInput);
    let count_value: usize = count.base10_parse().expect("Invalid number");

    // Create the enum variants using the prefix and numbers in a range
    let variants = (1..=count_value).map(|i| {
        let variant_name = format_ident!("{}{}", variant_prefix, i);
        if i == 1 {
            quote! { #[default] #variant_name }
        } else {
            quote! { #variant_name }
        }
    });

    // Generate From<usize> for Enum
    let from_usize_matches = (0..count_value).map(|i| {
        let variant_name = format_ident!("{}{}", variant_prefix, i + 1);
        quote! { #i => #enum_name::#variant_name }
    });

    // Generate From<Enum> for usize
    let from_enum_matches = (0..count_value).map(|i| {
        let variant_name = format_ident!("{}{}", variant_prefix, i + 1);
        quote! { #enum_name::#variant_name => #i }
    });

    // Generate the name of the max num constant
    let enum_max_const = format_ident!("{}", constant_name.to_string().to_uppercase());

    // Generate a std::fmt::Display implementation
    let display_matches = (1..=count_value).map(|i| {
        let variant_name = format_ident!("{}{}", variant_prefix, i);
        quote! {
            #enum_name::#variant_name => write!(f, stringify!(#variant_name))
        }
    });

    // Turn all those generated texts into code to be pasted
    let expanded = quote! {
        #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
        pub enum #enum_name {
            #(#variants),*
        }

        pub const #enum_max_const: usize = #count_value;

        impl From<usize> for #enum_name {
            fn from(value: usize) -> Self {
                match value {
                    #(#from_usize_matches,)*
                    _ => panic!("Invalid usize for enum {}: {}", stringify!(#enum_name), value),
                }
            }
        }

        impl From<#enum_name> for usize {
            fn from(value: #enum_name) -> Self {
                match value {
                    #(#from_enum_matches,)*
                }
            }
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_matches),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
