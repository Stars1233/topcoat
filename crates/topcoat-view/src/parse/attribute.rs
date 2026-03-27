use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

use crate::parse::ParseOption;

pub struct Attribute {
    name: Ident,
    _eq: Token![=],
    value: LitStr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ParseOption for Attribute {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident) && input.peek2(Token![=])
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.to_string();
        let value = &self.value;
        quote! {
            ::topcoat::view::Attribute::new(#name.into(), #value.into())
        }
        .to_tokens(tokens)
    }
}

pub struct Attributes {
    items: Vec<Attribute>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while let Some(attribute) = input.call(Attribute::parse_option)? {
            items.push(attribute);
        }
        Ok(Self { items })
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let items = &self.items;
        quote! {
            ::topcoat::view::Attributes::new(vec![#(#items),*])
        }
        .to_tokens(tokens);
    }
}
