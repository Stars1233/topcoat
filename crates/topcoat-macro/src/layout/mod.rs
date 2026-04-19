use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    ItemFn, LitStr,
    parse::{Parse, ParseStream},
};

pub struct LayoutAttr {
    path: Option<LitStr>,
}

impl Parse for LayoutAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.peek(LitStr).then(|| input.parse()).transpose()?,
        })
    }
}

pub struct LayoutItem {
    item: ItemFn,
}

impl Parse for LayoutItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub struct Layout(LayoutAttr, LayoutItem);

impl Layout {
    pub fn new(attr: LayoutAttr, item: LayoutItem) -> Self {
        Self(attr, item)
    }
}

impl ToTokens for Layout {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attr = &self.0;
        let item = &self.1.item;
        let ident = &item.sig.ident;

        let render = quote! {
            |slot| {
                #item
                Box::pin(#ident(slot))
            }
        };

        match attr.path.as_ref() {
            Some(path) => quote! {
                #[allow(non_upper_case_globals)]
                const #ident: ::topcoat::router::Layout = ::topcoat::router::Layout::new(#path, #render);
            },
            None => quote! {
                #[allow(non_upper_case_globals)]
                const #ident: ::topcoat::router::FileLayout = ::topcoat::router::FileLayout::new(file!(), #render);
            }
        }.to_tokens(tokens);

        if cfg!(feature = "discover") {
            quote! { ::topcoat::inventory::submit! { #ident } }.to_tokens(tokens);
        }
    }
}
