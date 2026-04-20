use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
};

use crate::{ast::parse_option::ParseOption, output::ViewWriter};

pub struct NodeExpr {
    pub paren: syn::token::Paren,
    pub expr: syn::Expr,
}

impl NodeExpr {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let expr = &self.expr;
        writer.write_expr(expr.to_token_stream());
    }
}

impl Parse for NodeExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}

impl ParseOption for NodeExpr {
    fn peek(input: ParseStream) -> bool {
        input.peek(syn::token::Paren)
    }
}

#[cfg(feature = "pretty")]
impl crate::pretty::PrettyPrint for NodeExpr {
    fn pretty_print(&self, printer: &mut crate::pretty::Printer<'_>) {
        use crate::pretty::{BreakMode, Delim};

        self.paren
            .pretty_print(printer, Some(BreakMode::Inconsistent), |printer| {
                self.expr.pretty_print(printer);
            });
    }
}
