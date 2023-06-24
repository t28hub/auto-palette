/// Struct representing a node of a k-d tree.
#[derive(Debug)]
pub struct KDNode {
    pub index: usize,
    pub axis: usize,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

impl KDNode {
    /// Creates a new `KDNode` instance.
    ///
    /// # Arguments
    /// * `index` - The index of the node.
    /// * `axis` - The axis of the node.
    /// * `left` - The left child node.
    /// * `right` - The right child node.
    ///
    /// # Returns
    /// A new `KDNode` instance.
    #[inline]
    #[must_use]
    pub fn new(index: usize, axis: usize, left: Option<KDNode>, right: Option<KDNode>) -> Self {
        Self {
            index,
            axis,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Returns the left child node.
    ///
    /// # Returns
    /// The left child node.
    #[inline]
    #[must_use]
    pub fn left(&self) -> &Option<Box<KDNode>> {
        &self.left
    }

    /// Returns the right child node.
    ///
    /// # Returns
    /// The right child node.
    #[inline]
    #[must_use]
    pub fn right(&self) -> &Option<Box<KDNode>> {
        &self.right
    }

    /// Checks if the node is a leaf node.
    ///
    /// # Returns
    /// `true` if the node is a leaf node, otherwise `false`.
    #[inline]
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = KDNode::new(0, 1, None, None);
        assert_eq!(node.index, 0);
        assert_eq!(node.axis, 1);
        assert!(node.left().is_none());
        assert!(node.right().is_none());
    }

    #[test]
    fn test_left() {
        let node = KDNode::new(0, 1, None, None);
        assert!(node.left().is_none());

        let left = KDNode::new(1, 2, None, None);
        let node = KDNode::new(0, 1, Some(left), None);
        assert!(node.left().is_some());
    }

    #[test]
    fn test_right() {
        let node = KDNode::new(0, 1, None, None);
        assert!(node.left().is_none());

        let right = KDNode::new(1, 2, None, None);
        let node = KDNode::new(0, 1, None, Some(right));
        assert!(node.right().is_some());
    }

    #[test]
    fn test_is_leaf() {
        let node = KDNode::new(0, 1, None, None);
        assert!(node.is_leaf());

        let node = KDNode::new(0, 1, Some(KDNode::new(1, 2, None, None)), None);
        assert!(!node.is_leaf());

        let node = KDNode::new(0, 1, None, Some(KDNode::new(1, 2, None, None)));
        assert!(!node.is_leaf());

        let node = KDNode::new(
            0,
            1,
            Some(KDNode::new(1, 2, None, None)),
            Some(KDNode::new(1, 2, None, None)),
        );
        assert!(!node.is_leaf());
    }
}
