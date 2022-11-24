use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);

    let expanded = quote! {
        pub struct #builder_name {
        }

        impl Command {
            pub fn builder() -> #builder_name {
                #builder_name {
                }
            }
        }
    };

    TokenStream::from(expanded)
}
