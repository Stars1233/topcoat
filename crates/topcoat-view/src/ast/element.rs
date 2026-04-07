use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::{Attributes, Node, ParseOption},
    output::ViewWriter,
};

pub enum Element {
    Normal {
        opening_tag: OpeningTag,
        children: Vec<Node>,
        closing_tag: ClosingTag,
    },
    SelfClosing {
        tag: SelfClosingTag,
    },
    Void {
        tag: OpeningTag,
    },
}

impl Element {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Normal { opening_tag, .. } => &opening_tag.name,
            Self::SelfClosing { tag } => &tag.name,
            Self::Void { tag } => &tag.name,
        }
    }

    pub fn attributes(&self) -> &Attributes {
        match self {
            Self::Normal { opening_tag, .. } => &opening_tag.attributes,
            Self::SelfClosing { tag } => &tag.attributes,
            Self::Void { tag } => &tag.attributes,
        }
    }

    pub fn children(&self) -> &[Node] {
        match self {
            Self::Normal { children, .. } => children,
            Self::SelfClosing { .. } | Self::Void { .. } => &[],
        }
    }

    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Normal {
                opening_tag,
                children,
                closing_tag,
            } => {
                writer.push_str("<");
                writer.push_str(&opening_tag.name.to_string());
                opening_tag.attributes.write(writer);
                writer.push_str(">");

                for child in children {
                    child.write(writer);
                }

                writer.push_str("</");
                writer.push_str(&closing_tag.name.to_string());
                writer.push_str(">");
            }
            Self::SelfClosing { tag } => {
                writer.push_str("<");
                writer.push_str(&tag.name.to_string());
                tag.attributes.write(writer);
                writer.push_str("/>");
            }
            Self::Void { tag } => {
                writer.push_str("<");
                writer.push_str(&tag.name.to_string());
                tag.attributes.write(writer);
                writer.push_str(">");
            }
        }
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lt: Token![<] = input.parse()?;
        let name: Ident = input.parse()?;
        let attributes: Attributes = input.parse()?;

        if input.peek(Token![/]) {
            return Ok(Self::SelfClosing {
                tag: SelfClosingTag {
                    lt,
                    name,
                    attributes,
                    slash: input.parse()?,
                    gt: input.parse()?,
                },
            });
        }

        let opening_tag = OpeningTag {
            lt,
            name,
            attributes,
            gt: input.parse()?,
        };

        if opening_tag.is_void_element() {
            return Ok(Self::Void { tag: opening_tag });
        }

        let mut children = Vec::new();
        while !input.is_empty() && !ClosingTag::peek(input) {
            children.push(input.parse()?);
        }

        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                format!(
                    "missing closing tag for opening tag `{}`",
                    &opening_tag.name
                ),
            ));
        }
        let closing_tag: ClosingTag = input.parse()?;
        if closing_tag.name != opening_tag.name {
            return Err(syn::Error::new(
                closing_tag.name.span(),
                format!(
                    "closing tag `{}` does not match opening tag `{}`",
                    closing_tag.name, opening_tag.name
                ),
            ));
        }
        Ok(Self::Normal {
            opening_tag,
            children,
            closing_tag,
        })
    }
}

impl ParseOption for Element {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![<])
    }
}

pub struct OpeningTag {
    pub lt: Token![<],
    pub name: Ident,
    pub attributes: Attributes,
    pub gt: Token![>],
}

impl OpeningTag {
    const VOID_ELEMENTS: &[&str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
        "track", "wbr",
    ];

    fn is_void_element(&self) -> bool {
        let name = self.name.to_string();
        Self::VOID_ELEMENTS.iter().any(|v| *v == name)
    }
}

impl Parse for OpeningTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lt: input.parse()?,
            name: input.parse()?,
            attributes: input.parse()?,
            gt: input.parse()?,
        })
    }
}

pub struct SelfClosingTag {
    pub lt: Token![<],
    pub name: Ident,
    pub attributes: Attributes,
    pub slash: Token![/],
    pub gt: Token![>],
}

impl Parse for SelfClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lt: input.parse()?,
            name: input.parse()?,
            attributes: input.parse()?,
            slash: input.parse()?,
            gt: input.parse()?,
        })
    }
}

pub struct ClosingTag {
    pub lt: Token![<],
    pub slash: Token![/],
    pub name: Ident,
    pub gt: Token![>],
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lt: input.parse()?,
            slash: input.parse()?,
            name: input.parse()?,
            gt: input.parse()?,
        })
    }
}

impl ParseOption for ClosingTag {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![<]) && input.peek2(Token![/])
    }
}
