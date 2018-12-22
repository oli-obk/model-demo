extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::*;
use std::collections::HashSet;
use std::str::FromStr;

#[proc_macro]
pub fn state_machine(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut lines = input.split(',');
    let type_name = Ident::new(lines.next().expect("first line must be the name of the type").trim());

    let mut names = Vec::new();
    let mut sources = Vec::new();
    let mut targets = Vec::new();
    let mut states = HashSet::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let mut split = line.split(':');
        names.push(Ident::new(split.next().unwrap().trim()));
        let mut split2 = split.next().unwrap().split("->");
        assert!(split.next().is_none(), "only one colon is allowed per line");
        let source = split2.next().unwrap().trim();
        sources.push(Ident::new(format!("{}::{}", type_name, source)));
        let target = split2.next().unwrap().trim();
        targets.push(Ident::new(format!("{}::{}", type_name, target)));
        states.insert(target);
        states.insert(source);
        assert!(split2.next().is_none(), "multiple `->` found in one line")
    }

    let states: Vec<_> = states.into_iter().map(Ident::new).collect();

    let gen = quote! {
        enum #type_name {
            #(#states),*
        }
        impl #type_name {
            #(
                fn #names(&mut self) {
                    match self {
                        #sources => *self = #targets,
                        _ => panic!("invalid transition"),
                    }
                }
            )*
        }
    };
    TokenStream::from_str(&gen.to_string()).unwrap()
}
