use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Lit;

pub struct ExprLit {
    inner: Lit,
}

impl ExprLit {
    pub fn new(inner: Lit) -> Self {
        Self { inner }
    }
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.inner;
        quote! { ::topcoat::runtime::IntoExpr::into_expr(#inner) }.to_tokens(tokens);
    }
}
