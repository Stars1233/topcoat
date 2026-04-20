use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Ident, LitStr, Token, parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
};

use crate::{ast::ParseOption, output::ViewWriter};

pub struct Attribute {
    pub name: Ident,
    pub eq: Token![=],
    pub value: AttributeValue,
}

impl Attribute {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let name = self.name.to_string();
        writer.write_str_unescaped(&name);
        writer.write_str_unescaped("=\"");
        self.value.write(writer);
        writer.write_str_unescaped("\"");
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ParseOption for Attribute {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident) && input.peek2(Token![=])
    }
}

#[cfg(feature = "pretty")]
impl crate::pretty::PrettyPrint for Attribute {
    fn pretty_print(&self, printer: &mut crate::pretty::Printer<'_>) {
        self.name.pretty_print(printer);
        self.eq.pretty_print(printer);
        self.value.pretty_print(printer);
    }
}

pub enum AttributeValue {
    Expr { paren: Paren, expr: Expr },
    LitStr(LitStr),
}

impl AttributeValue {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Expr { expr, .. } => writer.write_expr(expr.to_token_stream()),
            Self::LitStr(inner) => writer.write_str(&inner.value()),
        }
    }
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Paren) {
            let content;
            Ok(Self::Expr {
                paren: parenthesized!(content in input),
                expr: content.parse()?,
            })
        } else if lookahead.peek(LitStr) {
            Ok(Self::LitStr(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for AttributeValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Expr { expr, .. } => quote! {{ #expr }}.to_tokens(tokens),
            Self::LitStr(inner) => inner.to_tokens(tokens),
        }
    }
}

#[cfg(feature = "pretty")]
impl crate::pretty::PrettyPrint for AttributeValue {
    fn pretty_print(&self, printer: &mut crate::pretty::Printer<'_>) {
        match self {
            Self::LitStr(inner) => inner.pretty_print(printer),
            Self::Expr { paren, expr } => {
                use crate::pretty::{BreakMode, Delim};
                paren.pretty_print(printer, Some(BreakMode::Inconsistent), |printer| {
                    expr.pretty_print(printer);
                });
            }
        }
    }
}

pub struct Attributes {
    pub items: Vec<Attribute>,
}

impl Attributes {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        for item in &self.items {
            writer.write_str_unescaped(" ");
            item.write(writer);
        }
    }

    /// Returns `true` if `self` has no attributes.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
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

#[cfg(feature = "pretty")]
impl crate::pretty::PrettyPrint for Attributes {
    fn pretty_print(&self, printer: &mut crate::pretty::Printer<'_>) {
        if self.items.is_empty() {
            return;
        }
        for item in &self.items {
            printer.scan_break();
            " ".pretty_print(printer);
            item.pretty_print(printer);
        }
    }
}
