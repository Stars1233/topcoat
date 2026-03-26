use bumpalo::Bump;

use crate::parse::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Root,
    Element,
    AttributeList,
    Attribute,
}

#[derive(Debug, Clone)]
pub enum Data<'a> {
    Token(Token<'a>),
    Node(&'a Node<a>),
}

#[derive(Debug, Clone)]
pub struct Child<'a> {
    data: Data<'a>,
    next_sibling: Option<&'a Child<'a>>,
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    first_child: Child<'a>,
}

struct Frame<'a> {
    kind: NodeKind,
    children: Vec<&'a Node<'a>>,
}

pub struct NodeBuilder<'a> {
    bump: &'a Bump,
    stack: Vec<Frame<'a>>,
}

impl<'a> NodeBuilder<'a> {
    pub fn new(bump: &'a Bump) -> Self {
        Self {
            bump,
            stack: Vec::new(),
        }
    }

    pub fn begin(&mut self, kind: NodeKind) {
        self.stack.push(Frame {
            kind,
            children: Vec::new(),
        });
    }

    pub fn end(&mut self) -> &'a Node<'a> {}
}
