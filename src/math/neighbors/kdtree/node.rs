/// Struct representing a node of kd-tree.
#[derive(Debug)]
pub struct KDNode {
    /// The index of a point in the points.
    pub index: usize,

    /// The axis of the split.
    pub axis: usize,

    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

impl KDNode {
    /// Creates a new `KDNode` instance.
    ///
    /// # Arguments
    /// * `index` - The index of a point.
    /// * `axis` - The axis of the split.
    /// * `left` - The left child node.
    /// * `right` - The right child node.
    ///
    /// # Returns
    /// A new `KDNode` instance.
    #[must_use]
    pub fn new(index: usize, axis: usize, left: Option<KDNode>, right: Option<KDNode>) -> Self {
        Self {
            index,
            axis,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Returns a reference to the left child node.
    ///
    /// # Returns
    /// A reference to the left child node.
    #[must_use]
    pub fn left(&self) -> &Option<Box<KDNode>> {
        &self.left
    }

    /// Returns a reference to the right child node.
    ///
    /// # Returns
    /// A reference to the right child node.
    #[must_use]
    pub fn right(&self) -> &Option<Box<KDNode>> {
        &self.right
    }

    /// Checks whether this node is a leaf node.
    ///
    /// # Returns
    /// `true` if this node is a leaf node, otherwise `false`.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_kdnode() {
        let left = Some(KDNode::new(5, 1, None, None));
        let kdnode = KDNode::new(2, 0, left, None);
        assert_eq!(kdnode.index, 2);
        assert_eq!(kdnode.axis, 0);
        assert!(kdnode.left().is_some());
        assert!(kdnode.right().is_none());
    }

    #[test]
    fn is_leaf_should_return_whether_node_is_leaf_or_not() {
        let left = Some(KDNode::new(5, 1, None, None));
        let right = Some(KDNode::new(6, 1, None, None));
        let kdnode = KDNode::new(2, 0, left, right);
        assert_eq!(kdnode.is_leaf(), false);

        let left = Some(KDNode::new(5, 1, None, None));
        let kdnode = KDNode::new(2, 0, left, None);
        assert_eq!(kdnode.is_leaf(), false);

        let right = Some(KDNode::new(6, 1, None, None));
        let kdnode = KDNode::new(2, 0, None, right);
        assert_eq!(kdnode.is_leaf(), false);

        let kdnode = KDNode::new(2, 0, None, None);
        assert_eq!(kdnode.is_leaf(), true);
    }
}
