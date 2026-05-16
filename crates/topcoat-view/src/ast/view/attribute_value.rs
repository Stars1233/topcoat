use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    LitStr,
    parse::{Parse, ParseStream},
    token::Paren,
};

use crate::ast::view::{
    TemplateExpr,
    view_writer::{ViewWriter, WriteView},
};

pub enum AttributeValue {
    Expr(Box<TemplateExpr>),
    LitStr(LitStr),
}

impl WriteView for AttributeValue {
    fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Expr(inner) => writer.write_expr(inner.expr.to_token_stream()),
            Self::LitStr(inner) => writer.write_str(&inner.value()),
        }
    }
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Paren) {
            Ok(Self::Expr(input.parse()?))
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
            Self::Expr(inner) => inner.to_tokens(tokens),
            Self::LitStr(inner) => inner.to_tokens(tokens),
        }
    }
}

#[cfg(feature = "pretty")]
impl topcoat_pretty::PrettyPrint for AttributeValue {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        match self {
            Self::Expr(inner) => inner.pretty_print(printer),
            Self::LitStr(inner) => inner.pretty_print(printer),
        }
    }
}
