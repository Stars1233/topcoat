use syn::{
    Ident, Token, bracketed,
    parse::{Parse, ParseStream},
    token::Bracket,
};

use crate::{
    ast::{Attributes, Node, ParseOption},
    output::ViewWriter,
};

pub enum Component {
    Normal {
        opening_tag: OpeningTag,
        children: Vec<Node>,
        closing_tag: ClosingTag,
    },
    SelfClosing {
        tag: SelfClosingTag,
    },
}

impl Component {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Normal { opening_tag, .. } => &opening_tag.name,
            Self::SelfClosing { tag } => &tag.name,
        }
    }

    pub fn attributes(&self) -> &Attributes {
        match self {
            Self::Normal { opening_tag, .. } => &opening_tag.attributes,
            Self::SelfClosing { tag } => &tag.attributes,
        }
    }

    pub fn children(&self) -> &[Node] {
        match self {
            Self::Normal { children, .. } => children,
            Self::SelfClosing { .. } => &[],
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
        }
    }
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let name: Ident = content.parse()?;
        let attributes: Attributes = content.parse()?;

        if content.peek(Token![/]) {
            return Ok(Self::SelfClosing {
                tag: SelfClosingTag {
                    bracket_token,
                    name,
                    attributes,
                    slash: input.parse()?,
                },
            });
        }

        let opening_tag = OpeningTag {
            bracket_token,
            name,
            attributes,
        };

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

impl ParseOption for Component {
    fn peek(input: ParseStream) -> bool {
        input.peek(Bracket)
    }
}

pub struct OpeningTag {
    pub bracket_token: Bracket,
    pub name: Ident,
    pub attributes: Attributes,
}

impl Parse for OpeningTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            name: content.parse()?,
            attributes: content.parse()?,
        })
    }
}

pub struct SelfClosingTag {
    pub bracket_token: Bracket,
    pub name: Ident,
    pub attributes: Attributes,
    pub slash: Token![/],
}

impl Parse for SelfClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            name: content.parse()?,
            attributes: content.parse()?,
            slash: content.parse()?,
        })
    }
}

pub struct ClosingTag {
    pub bracket_token: Bracket,
    pub slash: Token![/],
    pub name: Ident,
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            slash: content.parse()?,
            name: content.parse()?,
        })
    }
}

impl ParseOption for ClosingTag {
    fn peek(input: ParseStream) -> bool {
        fn inner(input: ParseStream) -> syn::Result<()> {
            let content;
            let _ = bracketed!(content in input.fork());
            let _: Token![/] = content.parse()?;
            Ok(())
        }

        inner(input).is_ok()
    }
}
