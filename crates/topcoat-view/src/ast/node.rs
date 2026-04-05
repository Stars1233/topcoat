use syn::{
    LitStr,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::{Element, NodeExpr, NodeIf, ParseOption},
    view_writer::ViewWriter,
};

pub enum Node {
    Text(LitStr),
    Element(Element),
    ViewExpr(NodeExpr),
    NodeIf(NodeIf),
}

impl Node {
    pub fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Text(inner) => writer.push_escaped(&inner.value()),
            Self::Element(inner) => inner.write(writer),
            Self::ViewExpr(inner) => inner.write(writer),
            Self::NodeIf(inner) => inner.write(writer),
        }
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if Element::peek(input) {
            Ok(Self::Element(input.parse()?))
        } else if NodeExpr::peek(input) {
            Ok(Self::ViewExpr(input.parse()?))
        } else if NodeIf::peek(input) {
            Ok(Self::NodeIf(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected view node"))
        }
    }
}
