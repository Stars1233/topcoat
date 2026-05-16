use syn::parse::{Parse, ParseStream};

use crate::ast::{
    ParseOption,
    view::{AttributeNode, ViewWriter, WriteView},
};

/// The full list of attributes attached to a single tag.
pub struct Attributes {
    pub items: Vec<AttributeNode>,
}

impl Attributes {
    /// Returns `true` if `self` has no attributes.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl WriteView for Attributes {
    fn write(&self, writer: &mut ViewWriter) {
        for item in &self.items {
            item.write(writer);
        }
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while let Some(item) = input.call(AttributeNode::parse_option)? {
            items.push(item);
        }
        Ok(Self { items })
    }
}

#[cfg(feature = "pretty")]
impl topcoat_pretty::PrettyPrint for Attributes {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
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
