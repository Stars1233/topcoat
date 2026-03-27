use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

use crate::parse::Node;

pub struct View {
    nodes: Vec<Node>,
}

impl Parse for View {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            nodes: {
                let mut children = Vec::new();
                while !input.is_empty() {
                    children.push(input.parse()?)
                }
                children
            },
        })
    }
}

impl ToTokens for View {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let nodes = &self.nodes;
        quote! {
            ::topcoat::view::View::new(vec![#(#nodes),*])
        }
        .to_tokens(tokens);
    }
}
