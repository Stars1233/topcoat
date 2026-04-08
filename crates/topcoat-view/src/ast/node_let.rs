use quote::quote;
use syn::{
    ExprLet, Token,
    parse::{Parse, ParseStream},
};

use crate::{ast::parse_option::ParseOption, output::ViewWriter};

pub struct NodeLet {
    pub expr_let: ExprLet,
    pub semi_token: Token![;],
}

impl NodeLet {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let expr_let = &self.expr_let;
        writer.push_raw(quote! { #expr_let; });
    }
}

impl Parse for NodeLet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr_let: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ParseOption for NodeLet {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![let])
    }
}
