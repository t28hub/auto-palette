use crate::math::number::Float;

/// Struct representing a node in a hierarchical clustering tree.
///
/// # Type Parameters
/// * `F` - The type of the weight.
#[derive(Debug, PartialEq)]
pub struct HierarchicalNode<F: Float> {
    /// The index of the left child node.
    pub left: usize,
    /// The index of the right child node.
    pub right: usize,
    /// The weight of the edge between the left and right child nodes.
    pub weight: F,
    /// The number of nodes in the subtree rooted at this node.
    pub size: usize,
}
