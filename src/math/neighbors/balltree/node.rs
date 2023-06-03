use crate::math::point::Point;
use crate::number::Float;

/// Struct representing a node in a ball tree.
///
/// # Type Parameters
/// * `F` - The float type.
/// * `P` - The point type.
#[derive(Debug, PartialEq)]
pub struct Node<F: Float, P: Point<F>> {
    center: P,
    radius: F,
    indices: Vec<usize>,
    left: Option<Box<Node<F, P>>>,
    right: Option<Box<Node<F, P>>>,
}

impl<F, P> Node<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Creates a new `Node` instance.
    ///
    /// # Arguments
    /// * `center` - The center point of this node.
    /// * `radius` - The radius of this node.
    /// * `indices` - The indices of the points in the dataset.
    /// * `left` - The left child of this node.
    /// * `right` - The right child of this node.
    ///
    /// # Returns
    /// A new `Node` instance.
    #[must_use]
    pub fn new(
        center: P,
        radius: F,
        indices: Vec<usize>,
        left: Option<Node<F, P>>,
        right: Option<Node<F, P>>,
    ) -> Self {
        Self {
            center,
            radius,
            indices,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Returns a reference to the center point of this node.
    ///
    /// # Returns
    /// A reference to the center point of this node.
    #[inline]
    #[must_use]
    pub fn center(&self) -> &P {
        &self.center
    }

    /// Returns the radius of this node.
    ///
    /// # Returns
    /// The radius of this node.
    #[inline]
    #[must_use]
    pub fn radius(&self) -> F {
        self.radius
    }

    /// Returns a reference to the indices of the points in the dataset.
    ///
    /// # Returns
    /// A reference to the indices of the points in the dataset.
    #[inline]
    #[must_use]
    pub fn indices(&self) -> &Vec<usize> {
        &self.indices
    }

    /// Returns a reference to the left child node.
    ///
    /// # Returns
    /// A reference to the left child node.
    #[inline]
    #[must_use]
    pub fn left(&self) -> &Option<Box<Node<F, P>>> {
        &self.left
    }

    /// Returns a reference to the right child node.
    ///
    /// # Returns
    /// A reference to the right child node.
    #[inline]
    #[must_use]
    pub fn right(&self) -> &Option<Box<Node<F, P>>> {
        &self.right
    }

    /// Checks whether this node is a leaf node.
    ///
    /// # Returns
    /// `true` if this node is a leaf node, otherwise `false`.
    #[inline]
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point3;

    #[test]
    fn test_node() {
        let indices = vec![0, 1, 2];
        let node = Node::new(Point3(1.0, 2.0, 3.0), 1.0, indices, None, None);
        assert_eq!(node.center, Point3(1.0, 2.0, 3.0));
        assert_eq!(node.radius, 1.0);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
    }
}
