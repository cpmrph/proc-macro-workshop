use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);
    let fields = match input.data {
        Data::Struct(st) => {
            st.fields
        },
        _ => unimplemented!()
    };

    let mut builder_fields = Vec::new();
    let mut fields_inits = Vec::new();
    let mut setters = Vec::new();
    for field in fields {
        let ident = field.ident;
        let ty = field.ty;

        builder_fields.push(quote!{
            #ident: Option<#ty>,
        });
        fields_inits.push(quote!{
            #ident: None,
        });
        setters.push(quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        });
    }

    let expanded = quote! {
        pub struct #builder_name {
            #( #builder_fields )*
        }

        impl #builder_name {
            #( #setters )*
        }

        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #( #fields_inits )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
