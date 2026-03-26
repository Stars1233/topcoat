use bumpalo::Bump;

use crate::parse::{Node, NodeKind, Token};

pub struct Parser<'a> {
    bump: &'a Bump,
    tokens: &'a [Token<'a>],
    position: usize,
}

impl<'a> Parser<'a> {
    fn parse_root(&mut self) -> Node<'a> {
        Node::Parent {
            kind: NodeKind::Root,
            children: (),
        }
    }
}
