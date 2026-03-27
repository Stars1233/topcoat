use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    LitStr,
    parse::{Parse, ParseStream},
};

use crate::parse::{Element, ParseOption};

pub enum Node {
    Text(LitStr),
    Element(Element),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if Element::peek(input) {
            Ok(Self::Element(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected view node"))
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Text(text) => quote! { ::topcoat::view::Node::Text(#text.into()) },
            Self::Element(element) => quote! { ::topcoat::view::Node::Element(#element) },
        }
        .to_tokens(tokens);
    }
}
