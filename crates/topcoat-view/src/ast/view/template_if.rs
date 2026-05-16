use syn::{
    Token,
    parse::{Parse, ParseStream},
};

use crate::ast::{
    ParseOption,
    view::{ViewWriter, WriteView},
};

/// An `if cond { ... } else { ... }` chain in view-body position.
pub struct TemplateIf<B> {
    pub if_token: Token![if],
    pub cond: syn::Expr,
    pub then_branch: B,
    pub else_branch: Option<TemplateElse<B>>,
}

impl<B: WriteView> WriteView for TemplateIf<B> {
    fn write(&self, writer: &mut ViewWriter) {
        writer.if_else(&self.cond, |then_writer, else_writer| {
            self.then_branch.write(then_writer);
            if let Some(else_branch) = self.else_branch.as_ref() {
                else_branch.write(else_writer);
            }
        });
    }
}

impl<B: Parse> Parse for TemplateIf<B> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.call(syn::Expr::parse_without_eager_brace)?,
            then_branch: input.parse()?,
            else_branch: input.call(TemplateElse::parse_option)?,
        })
    }
}

impl<B: Parse> ParseOption for TemplateIf<B> {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![if])
    }
}

#[cfg(feature = "pretty")]
impl<B: topcoat_pretty::PrettyPrint> topcoat_pretty::PrettyPrint for TemplateIf<B> {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        self.if_token.pretty_print(printer);
        " ".pretty_print(printer);
        self.cond.pretty_print(printer);
        " ".pretty_print(printer);
        self.then_branch.pretty_print(printer);
        self.else_branch.pretty_print(printer);
    }
}

/// The trailing `else if ...` or `else { ... }` of a [`TemplateIf`].
pub enum TemplateElse<B> {
    ElseIf {
        else_token: Token![else],
        template_if: Box<TemplateIf<B>>,
    },
    Else {
        else_token: Token![else],
        then_branch: B,
    },
}

impl<B: WriteView> WriteView for TemplateElse<B> {
    fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::ElseIf { template_if, .. } => template_if.write(writer),
            Self::Else { then_branch, .. } => then_branch.write(writer),
        }
    }
}

impl<B: Parse> Parse for TemplateElse<B> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let else_token: Token![else] = input.parse()?;
        if input.peek(Token![if]) {
            Ok(Self::ElseIf {
                else_token,
                template_if: input.parse()?,
            })
        } else {
            Ok(Self::Else {
                else_token,
                then_branch: input.parse()?,
            })
        }
    }
}

impl<B: Parse> ParseOption for TemplateElse<B> {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![else])
    }
}

#[cfg(feature = "pretty")]
impl<B: topcoat_pretty::PrettyPrint> topcoat_pretty::PrettyPrint for TemplateElse<B> {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        match self {
            Self::ElseIf {
                else_token,
                template_if,
            } => {
                " ".pretty_print(printer);
                else_token.pretty_print(printer);
                " ".pretty_print(printer);
                template_if.pretty_print(printer);
            }
            Self::Else {
                else_token,
                then_branch,
            } => {
                " ".pretty_print(printer);
                else_token.pretty_print(printer);
                " ".pretty_print(printer);
                then_branch.pretty_print(printer);
            }
        }
    }
}
