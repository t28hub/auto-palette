use crate::math::clustering::hierarchical::node::Node;
use crate::number::Float;

#[derive(Debug)]
pub struct Dendrogram<F>
where
    F: Float,
{
    nodes: Vec<Node<F>>,
}

impl<F> Dendrogram<F>
where
    F: Float,
{
    #[must_use]
    pub fn new(initial_nodes: Vec<Node<F>>) -> Self {
        Self {
            nodes: initial_nodes,
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn nodes(&self) -> &[Node<F>] {
        &self.nodes
    }

    pub fn add(&mut self, node: Node<F>) {
        self.nodes.push(node);
    }
}
