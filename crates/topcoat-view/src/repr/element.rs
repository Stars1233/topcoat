use std::borrow::Cow;

use crate::repr::{Attributes, Node};

pub struct Element {
    name: Cow<'static, str>,
    attributes: Attributes,
    children: Vec<Node>,
}

impl Element {
    pub fn new(name: Cow<'static, str>, attributes: Attributes, children: Vec<Node>) -> Self {
        Self {
            name,
            attributes,
            children,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}
