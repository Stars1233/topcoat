use crate::repr::Node;

pub struct View {
    nodes: Vec<Node>,
}

impl View {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}
