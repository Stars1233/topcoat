mod component;
mod layout;
mod memoize;
mod page;
mod quote_option;
mod route;
mod segment;

use proc_macro::TokenStream;
use quote::quote;
use topcoat_view::ast::View;

#[proc_macro]
pub fn view(tokens: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(tokens as View);
    quote! { #parsed }.into()
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attr = syn::parse_macro_input!(attr as component::ComponentAttr);
    let item = syn::parse_macro_input!(item as component::ComponentItem);
    quote! { #item }.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attr = syn::parse_macro_input!(attr as route::RouteAttr);
    let item = syn::parse_macro_input!(item as route::RouteItem);
    quote! { #item }.into()
}

#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as page::PageAttr);
    let item = syn::parse_macro_input!(item as page::PageItem);
    let page = page::Page::new(attr, item);
    quote! { #page }.into()
}

#[proc_macro_attribute]
pub fn layout(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as layout::LayoutAttr);
    let item = syn::parse_macro_input!(item as layout::LayoutItem);
    let layout = layout::Layout::new(attr, item);
    quote! { #layout }.into()
}

#[proc_macro]
pub fn segment(tokens: TokenStream) -> TokenStream {
    let segment = syn::parse_macro_input!(tokens as segment::Segment);
    quote! { #segment }.into()
}

#[proc_macro_attribute]
pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as memoize::MemoizeAttr);
    let item = syn::parse_macro_input!(item as memoize::MemoizeItem);
    let memoize = memoize::Memoize::new(attr, item);
    quote! { #memoize }.into()
}
