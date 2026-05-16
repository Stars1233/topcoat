use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::ast::{
    ParseOption,
    view::{ViewWriter, WriteView},
};

/// A brace-delimited group of template nodes: `{ ...nodes... }`. Used as the
/// body of `if`, `for` and `match` arms, generic over the kind of node it
/// contains.
pub struct TemplateBlock<T> {
    pub brace: Brace,
    pub children: Vec<T>,
}

impl<T: WriteView> WriteView for TemplateBlock<T> {
    fn write(&self, writer: &mut ViewWriter) {
        for child in &self.children {
            child.write(writer);
        }
    }
}

impl<T: Parse> Parse for TemplateBlock<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace: braced!(content in input),
            children: {
                let mut children = Vec::new();
                while !content.is_empty() {
                    children.push(content.parse()?)
                }
                children
            },
        })
    }
}

impl<T: Parse> ParseOption for TemplateBlock<T> {
    fn peek(input: ParseStream) -> bool {
        input.peek(Brace)
    }
}

#[cfg(feature = "pretty")]
impl<T: topcoat_pretty::PrettyPrint> topcoat_pretty::PrettyPrint for TemplateBlock<T> {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        use topcoat_pretty::Delim;

        printer.move_cursor(self.brace.span().open().start());
        "{".pretty_print(printer);
        printer.move_cursor(self.brace.span().open().end());

        printer.scan_indent(1);
        printer.scan_break();

        printer.scan_trivia(false, true);

        for (index, node) in self.children.iter().enumerate() {
            node.pretty_print(printer);
            if index < self.children.len() - 1 {
                printer.scan_same_line_trivia();
                printer.scan_force_break();
                " ".pretty_print(printer);
                printer.scan_trivia(true, true);
            }
        }

        printer.move_cursor(self.brace.span().close().start());
        printer.scan_trivia(true, false);

        printer.scan_indent(-1);
        printer.scan_force_break();
        printer.scan_break();

        "}".pretty_print(printer);
        printer.move_cursor(self.brace.span().close().end());
    }
}
