use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

use crate::ast::node::Node;

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
        let mut writer = ViewWriter::new();
        for node in &self.nodes {
            node.write(&mut writer);
        }
        writer.to_tokens(tokens);
    }
}

#[derive(Default)]
pub(crate) struct ViewWriter {
    exprs: Vec<TokenStream>,
    static_segment: String,
    static_len: usize,
}

impl ViewWriter {
    pub fn new() -> Self {
        Self::default()
    }

    fn flush(&mut self) {
        if !self.static_segment.is_empty() {
            self.exprs.push(self.static_segment.to_token_stream());
            self.static_len += self.static_segment.len();
            self.static_segment.clear();
        }
    }

    pub fn push(&mut self, ch: char) {
        self.static_segment.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.static_segment.push_str(string);
    }

    pub fn push_escaped(&mut self, string: &str) {
        for c in string.chars() {
            match c {
                '&' => self.push_str("&amp;"),
                '<' => self.push_str("&lt;"),
                '>' => self.push_str("&gt;"),
                '"' => self.push_str("&quot;"),
                '\'' => self.push_str("&#x27;"),
                _ => self.push(c),
            }
        }
    }

    pub fn push_expr(&mut self, expr: TokenStream) {
        self.flush();
        self.exprs.push(expr);
    }
}

impl ToTokens for ViewWriter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let static_segment = &self.static_segment;

        // Optimized path: The view has no dynamic content. We can construct it as a &'static str.
        if self.exprs.is_empty() {
            quote! { ::topcoat::view::View::new(#static_segment) }.to_tokens(tokens);
            return;
        }

        let static_segment = &self.static_segment;
        let static_len = self.static_len + static_segment.len();

        let mut push_ops = Vec::new();
        for expr in &self.exprs {
            push_ops.push(quote! { writer.push_str(#expr); });
        }

        if !static_segment.is_empty() {
            push_ops.push(quote! { writer.push_str(#static_segment); });
        }

        quote! {{
            let mut writer = ::topcoat::view::ViewWriter::with_capacity(#static_len);
            #(#push_ops)*
            writer.finish()
        }}
        .to_tokens(tokens);
    }
}
