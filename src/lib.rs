extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn hello_world(_input: TokenStream) -> TokenStream {
    let gen = quote! {
        fn hello() {
            println!("Hello World!");
        }
    };
    gen.into()
}
