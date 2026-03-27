use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, braced,
    parse::{Parse, ParseStream},
};

use crate::parse::{Attributes, Node, ParseOption};

pub struct Element {
    name: Ident,
    attributes: Attributes,
    body: ElementBody,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            attributes: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl ParseOption for Element {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident)
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name.to_string();
        let attributes = &self.attributes;
        let body = &self.body;
        quote! {
            ::topcoat::view::Element::new(#name.into(), #attributes, #body)
        }
        .to_tokens(tokens);
    }
}

pub struct ElementBody {
    _brace: syn::token::Brace,
    children: Vec<Node>,
}

impl Parse for ElementBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brace: braced!(content in input),
            children: {
                let mut children = Vec::new();
                while !content.is_empty() {
                    children.push(content.parse()?)
                }
                children
            },
        })
    }
}

impl ToTokens for ElementBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let children = &self.children;
        quote! {
            vec![#(#children),*]
        }
        .to_tokens(tokens);
    }
}
