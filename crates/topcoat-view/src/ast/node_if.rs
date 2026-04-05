use syn::{
    Token,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::{NodeBlock, parse_option::ParseOption},
    view_writer::{ViewWriter, ViewWriterIf},
};

pub struct NodeIf {
    if_token: Token![if],
    cond: syn::Expr,
    then_branch: NodeBlock,
    else_branch: Option<NodeElse>,
}

impl NodeIf {
    pub fn write(&self, writer: &mut ViewWriter) {
        let mut writer = writer.begin_if(&self.cond);
        self.then_branch.write(&mut writer);
        if let Some(else_branch) = self.else_branch.as_ref() {
            else_branch.write(writer);
        }
    }
}

impl Parse for NodeIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.parse()?,
            then_branch: input.parse()?,
            else_branch: input.call(NodeElse::parse_option)?,
        })
    }
}

impl ParseOption for NodeIf {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![if])
    }
}

pub enum NodeElse {
    ElseIf {
        else_token: Token![else],
        node_if: Box<NodeIf>,
    },
    Else {
        else_token: Token![else],
        then_branch: NodeBlock,
    },
}

impl NodeElse {
    fn write(&self, writer: ViewWriterIf<'_>) {
        let mut writer = writer.begin_else();
        match self {
            Self::ElseIf { node_if, .. } => node_if.write(&mut writer),
            Self::Else { then_branch, .. } => then_branch.write(&mut writer),
        }
    }
}

impl Parse for NodeElse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let else_token: Token![else] = input.parse()?;
        if input.peek(Token![if]) {
            Ok(Self::ElseIf {
                else_token,
                node_if: input.parse()?,
            })
        } else {
            Ok(Self::Else {
                else_token,
                then_branch: input.parse()?,
            })
        }
    }
}

impl ParseOption for NodeElse {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![else])
    }
}
