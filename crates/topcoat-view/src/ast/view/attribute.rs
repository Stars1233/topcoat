use syn::{
    Ident, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::ast::{
    ParseOption,
    view::{AttributeValue, ViewWriter, WriteView},
};

/// A single `name=value` attribute on an [`Element`](super::Element) or
/// [`Component`](super::Component).
pub struct Attribute {
    pub name: Ident,
    pub eq: Token![=],
    pub value: AttributeValue,
}

impl WriteView for Attribute {
    fn write(&self, writer: &mut ViewWriter) {
        let name = self.name.to_string();
        writer.write_str_unescaped(" ");
        writer.write_str_unescaped(&name);
        writer.write_str_unescaped("=\"");
        self.value.write(writer);
        writer.write_str_unescaped("\"");
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            // Accept Rust keywords as attribute names.
            name: Ident::parse_any(input)?,
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ParseOption for Attribute {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident::peek_any) && input.peek2(Token![=])
    }
}

#[cfg(feature = "pretty")]
impl topcoat_pretty::PrettyPrint for Attribute {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        self.name.pretty_print(printer);
        self.eq.pretty_print(printer);
        self.value.pretty_print(printer);
    }
}
