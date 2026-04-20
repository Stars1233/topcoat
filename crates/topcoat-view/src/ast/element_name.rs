use std::fmt::Display;

use proc_macro2::Span;
use quote::quote;
use syn::{
    Expr, Ident, LitStr, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Paren,
};

use crate::output::ViewWriter;

#[derive(PartialEq, Eq)]
pub enum ElementName {
    Ident(Ident),
    LitStr(LitStr),
    Expr { paren: Paren, expr: Expr },
}

impl ElementName {
    pub fn string_name(&self) -> Option<String> {
        match self {
            Self::Ident(inner) => Some(inner.to_string()),
            Self::LitStr(inner) => Some(inner.value()),
            Self::Expr { .. } => None,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Ident(inner) => inner.span(),
            Self::LitStr(inner) => inner.span(),
            Self::Expr { paren, .. } => paren.span.span(),
        }
    }

    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Ident(inner) => writer.write_str_unescaped(&inner.to_string()),
            Self::LitStr(inner) => writer.write_str_unescaped(&inner.value()),
            Self::Expr { expr, .. } => writer.write_expr(quote! { #expr }),
        }
    }

    pub fn is_void_element(&self) -> bool {
        const VOID_ELEMENTS: &[&str] = &[
            "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
            "track", "wbr",
        ];

        match self {
            Self::Ident(inner) => {
                let name = inner.to_string();
                VOID_ELEMENTS.iter().any(|v| *v == name)
            }
            _ => false,
        }
    }

    pub fn expr(&self) -> Option<&Expr> {
        match self {
            Self::Expr { expr, .. } => Some(expr),
            _ => None,
        }
    }
}

impl Display for ElementName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(inner) => inner.fmt(f),
            Self::LitStr(inner) => inner.value().fmt(f),
            Self::Expr { .. } => f.write_str("<expr>"),
        }
    }
}

impl Parse for ElementName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if lookahead.peek(LitStr) {
            Ok(Self::LitStr(input.parse()?))
        } else if lookahead.peek(Paren) {
            let content;
            Ok(Self::Expr {
                paren: parenthesized!(content in input),
                expr: content.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(feature = "pretty")]
impl crate::pretty::PrettyPrint for ElementName {
    fn pretty_print(&self, printer: &mut crate::pretty::Printer<'_>) {
        match self {
            Self::Ident(inner) => inner.pretty_print(printer),
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
