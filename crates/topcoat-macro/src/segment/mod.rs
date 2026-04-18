use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Segment {
    attrs: Punctuated<SegmentAttr, Token![,]>,
}

impl Parse for Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.parse_terminated(SegmentAttr::parse, Token![,])?,
        })
    }
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if cfg!(feature = "discover") {
            quote! {
                ::topcoat::inventory::submit! {
                    ::topcoat::router::Segment::new(
                        file!(),
                        None,
                        None,
                    )
                }
            }
            .to_tokens(tokens);
        }
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kind);
    custom_keyword!(rename);
}

pub enum SegmentAttr {
    Kind {
        kind_kw: kw::kind,
        eq_token: Token![=],
        value: Ident,
    },
    Rename {
        rename_kw: kw::rename,
        eq_token: Token![=],
        value: LitStr,
    },
}

impl Parse for SegmentAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::kind) {
            Ok(Self::Kind {
                kind_kw: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::rename) {
            Ok(Self::Rename {
                rename_kw: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
