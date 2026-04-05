use std::ops::{Deref, DerefMut};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Expr;

#[derive(Default)]
pub(crate) struct ViewWriter {
    tokens: TokenStream,
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
            quote! { writer.push_str(#static_segment); }.to_tokens(&mut self.tokens);
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
        quote! { writer.push_str(#expr); }.to_tokens(&mut self.tokens);
    }

    pub fn begin_if<'a>(&'a mut self, cond: &'a Expr) -> ViewWriterIf<'a> {
        ViewWriterIf::new(self, cond)
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
            quote! { writer.push_str(#static_segment); }
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

macro_rules! impl_deref {
    ($ident:ident) => {
        impl Deref for $ident<'_> {
            type Target = ViewWriter;

            fn deref(&self) -> &Self::Target {
                self.writer
            }
        }

        impl DerefMut for $ident<'_> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.writer
            }
        }
    };
}

pub(crate) struct ViewWriterIf<'a> {
    writer: Option<&'a mut ViewWriter>,
    cond: &'a Expr,
    tokens: TokenStream,
}

impl<'a> ViewWriterIf<'a> {
    fn new(writer: &'a mut ViewWriter, cond: &'a Expr) -> Self {
        Self {
            writer: Some(writer),
            cond,
            tokens: TokenStream::new(),
        }
    }

    pub fn begin_else(mut self) -> ViewWriterElse<'a> {
        let writer = self.flush();
        ViewWriterElse::new(writer)
    }

    fn flush(&mut self) -> &'a mut ViewWriter {
        let writer = self.writer.take().expect("was already flushed");
        writer.flush();
        let cond = self.cond;
        let body = &self.tokens;
        quote! { if #cond { #body } }.to_tokens(&mut writer.tokens);
        writer
    }
}

impl Deref for ViewWriterIf<'_> {
    type Target = ViewWriter;

    fn deref(&self) -> &Self::Target {
        self.writer.as_ref().unwrap()
    }
}

impl DerefMut for ViewWriterIf<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.writer.as_mut().unwrap()
    }
}

impl Drop for ViewWriterIf<'_> {
    fn drop(&mut self) {
        if self.writer.is_some() {
            self.flush();
        }
    }
}

pub(crate) struct ViewWriterElse<'a> {
    writer: &'a mut ViewWriter,
    tokens: TokenStream,
}

impl<'a> ViewWriterElse<'a> {
    fn new(writer: &'a mut ViewWriter) -> Self {
        Self {
            writer,
            tokens: TokenStream::new(),
        }
    }
}

impl_deref!(ViewWriterElse);

impl Drop for ViewWriterElse<'_> {
    fn drop(&mut self) {
        let body = &self.tokens;
        quote! { else { #body } }.to_tokens(&mut self.writer.tokens);
    }
}
