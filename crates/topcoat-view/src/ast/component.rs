use quote::quote;
use syn::{
    Path, Token, bracketed,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Bracket,
};

use crate::{
    ast::{Attributes, Node, ParseOption},
    output::ViewWriter,
};

pub enum Component {
    Normal {
        opening_tag: ComponentOpeningTag,
        children: Vec<Node>,
        closing_tag: ComponentClosingTag,
    },
    SelfClosing {
        tag: ComponentSelfClosingTag,
    },
}

impl Component {
    pub fn path(&self) -> &Path {
        match self {
            Self::Normal { opening_tag, .. } => &opening_tag.path,
            Self::SelfClosing { tag } => &tag.path,
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
        let name = self.path();
        let fields = self.attributes().items.iter().map(|item| {
            let name = &item.name;
            let value = &item.value;
            quote! { #name: #value }
        });
        let mut child_writer = ViewWriter::new();
        for child in self.children() {
            child.write(&mut child_writer);
        }

        writer.push_expr(quote! {
            <#name as ::topcoat::component::Component>::render(#name {
                child: #child_writer,
                #(#fields),*
            }).await
        });
    }
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let path: Path = content.parse()?;
        let attributes: Attributes = content.parse()?;

        if content.peek(Token![/]) {
            return Ok(Self::SelfClosing {
                tag: ComponentSelfClosingTag {
                    bracket_token,
                    path,
                    attributes,
                    slash: content.parse()?,
                },
            });
        }

        let opening_tag = ComponentOpeningTag {
            bracket_token,
            path,
            attributes,
        };

        let mut children = Vec::new();
        while !input.is_empty() && !ComponentClosingTag::peek(input) {
            children.push(input.parse()?);
        }

        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                format!(
                    "missing closing tag for opening tag `{}`",
                    &opening_tag.path.segments.last().unwrap().ident
                ),
            ));
        }
        let closing_tag: ComponentClosingTag = input.parse()?;
        if closing_tag.path != opening_tag.path {
            return Err(syn::Error::new(
                closing_tag.path.span(),
                format!(
                    "closing tag `{}` does not match opening tag `{}`",
                    &closing_tag.path.segments.last().unwrap().ident,
                    &opening_tag.path.segments.last().unwrap().ident
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

pub struct ComponentOpeningTag {
    pub bracket_token: Bracket,
    pub path: Path,
    pub attributes: Attributes,
}

impl Parse for ComponentOpeningTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            path: content.parse()?,
            attributes: content.parse()?,
        })
    }
}

pub struct ComponentSelfClosingTag {
    pub bracket_token: Bracket,
    pub path: Path,
    pub attributes: Attributes,
    pub slash: Token![/],
}

impl Parse for ComponentSelfClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            path: content.parse()?,
            attributes: content.parse()?,
            slash: content.parse()?,
        })
    }
}

pub struct ComponentClosingTag {
    pub bracket_token: Bracket,
    pub slash: Token![/],
    pub path: Path,
}

impl Parse for ComponentClosingTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            slash: content.parse()?,
            path: content.parse()?,
        })
    }
}

impl ParseOption for ComponentClosingTag {
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
