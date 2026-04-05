mod view_writer_for_loop;
mod view_writer_if;

pub use view_writer_for_loop::*;
pub use view_writer_if::*;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, Pat};

#[derive(Default)]
pub(crate) struct ViewWriter {
    pub(self) tokens: TokenStream,
    static_segment: String,
    static_len: usize,
}

impl ViewWriter {
    pub fn new() -> Self {
        Self::default()
    }

    fn flush(&mut self) {
        if !self.static_segment.is_empty() {
            let static_segment = &self.static_segment;
            quote! { writer.push_fragment(#static_segment); }.to_tokens(&mut self.tokens);
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
        quote! { writer.push_fragment(#expr); }.to_tokens(&mut self.tokens);
    }

    pub fn begin_if<'a>(&'a mut self, cond: &'a Expr) -> ViewWriterIf<'a> {
        ViewWriterIf::new(self, cond)
    }

    pub fn begin_for_loop<'a>(&'a mut self, pat: &'a Pat, expr: &'a Expr) -> ViewWriterForLoop<'a> {
        ViewWriterForLoop::new(self, pat, expr)
    }

    pub(self) fn merge_into(&mut self, parent: &mut Self) {
        parent.flush();
        self.flush();
        parent.static_len += self.static_len;
        self.tokens.to_tokens(&mut parent.tokens);
    }
}

impl ToTokens for ViewWriter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let static_segment = &self.static_segment;

        // Optimized path: The view has no dynamic content. We can construct it as a &'static str.
        if self.tokens.is_empty() {
            quote! { ::topcoat::view::View::new(#static_segment) }.to_tokens(tokens);
            return;
        }

        let buffer = &self.tokens;
        let static_len = self.static_len + static_segment.len();
        let final_segment = (!static_segment.is_empty()).then(|| {
            quote! { writer.push_fragment(#static_segment); }
        });
        quote! {{
            let mut writer = ::topcoat::view::ViewWriter::with_capacity(#static_len);
            #buffer
            #final_segment
            writer.finish()
        }}
        .to_tokens(tokens);
    }
}
