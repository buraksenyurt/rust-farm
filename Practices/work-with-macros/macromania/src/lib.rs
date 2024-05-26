extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Snoopy)]
pub fn snoopy_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let new_token_stream = quote! {
        impl #name {
            pub fn do_something() {
                println!("This is a '{}' data structure!", stringify!(#name));
            }
        }
    };

    TokenStream::from(new_token_stream)
}
