use quote::quote;
use syn::{
    Expr, ExprBreak, ExprContinue, Pat, Token,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::{NodeBlock, parse_option::ParseOption},
    output::ViewWriter,
};

pub struct NodeForLoop {
    pub for_token: Token![for],
    pub pat: Box<Pat>,
    pub in_token: Token![in],
    pub expr: Box<Expr>,
    pub body: NodeBlock,
}

impl NodeForLoop {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let mut writer = writer.begin_for_loop(&self.pat, &self.expr);
        self.body.write(&mut writer);
    }
}

impl Parse for NodeForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            pat: Box::new(input.call(Pat::parse_single)?),
            in_token: input.parse()?,
            expr: Box::new(input.call(Expr::parse_without_eager_brace)?),
            body: input.parse()?,
        })
    }
}

impl ParseOption for NodeForLoop {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![for])
    }
}

pub struct NodeContinue {
    pub expr_continue: ExprContinue,
    pub semi_token: Token![;],
}

impl NodeContinue {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let expr_continue = &self.expr_continue;
        let semi_token = &self.semi_token;
        writer.push_raw(quote! { #expr_continue #semi_token });
    }
}

impl Parse for NodeContinue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr_continue: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ParseOption for NodeContinue {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![continue])
    }
}

pub struct NodeBreak {
    pub expr_break: ExprBreak,
    pub semi_token: Token![;],
}

impl NodeBreak {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let expr_break = &self.expr_break;
        let semi_token = &self.semi_token;
        writer.push_raw(quote! { #expr_break #semi_token });
    }
}

impl Parse for NodeBreak {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr_break: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ParseOption for NodeBreak {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![break])
    }
}
