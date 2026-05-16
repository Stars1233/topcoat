use syn::{
    Expr, ExprBreak, ExprContinue, Pat, Token,
    parse::{Parse, ParseStream},
};

use crate::ast::{
    ParseOption,
    view::{ViewWriter, WriteView},
};

/// A `for pat in expr { ... }` loop in view-body position. The body is
/// rendered once per iteration.
pub struct TemplateForLoop<B> {
    pub for_token: Token![for],
    pub pat: Box<Pat>,
    pub in_token: Token![in],
    pub expr: Box<Expr>,
    pub body: B,
}

impl<B: WriteView> WriteView for TemplateForLoop<B> {
    fn write(&self, writer: &mut ViewWriter) {
        writer.for_loop(&self.pat, &self.expr, |writer| {
            self.body.write(writer);
        });
    }
}

impl<B: Parse> Parse for TemplateForLoop<B> {
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

impl<B: Parse> ParseOption for TemplateForLoop<B> {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![for])
    }
}

#[cfg(feature = "pretty")]
impl<B: topcoat_pretty::PrettyPrint> topcoat_pretty::PrettyPrint for TemplateForLoop<B> {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        self.for_token.pretty_print(printer);
        " ".pretty_print(printer);
        self.pat.pretty_print(printer);
        " ".pretty_print(printer);
        self.in_token.pretty_print(printer);
        " ".pretty_print(printer);
        self.expr.pretty_print(printer);
        " ".pretty_print(printer);
        self.body.pretty_print(printer);
    }
}

/// A `continue;` statement. Parsed for completeness but currently rejected.
pub struct TemplateContinue {
    pub expr_continue: ExprContinue,
    pub semi_token: Token![;],
}

impl WriteView for TemplateContinue {
    fn write(&self, _writer: &mut ViewWriter) {
        todo!();
    }
}

impl Parse for TemplateContinue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr_continue: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ParseOption for TemplateContinue {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![continue])
    }
}

#[cfg(feature = "pretty")]
impl topcoat_pretty::PrettyPrint for TemplateContinue {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        self.semi_token.pretty_print(printer);
        todo!();
    }
}

/// A `break;` statement. Parsed for completeness but currently rejected.
pub struct TemplateBreak {
    pub expr_break: ExprBreak,
    pub semi_token: Token![;],
}

impl WriteView for TemplateBreak {
    fn write(&self, _writer: &mut ViewWriter) {
        todo!();
    }
}

impl Parse for TemplateBreak {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr_break: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ParseOption for TemplateBreak {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![break])
    }
}

#[cfg(feature = "pretty")]
impl topcoat_pretty::PrettyPrint for TemplateBreak {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        self.semi_token.pretty_print(printer);
        todo!();
    }
}
