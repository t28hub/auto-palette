use crate::math::point::Point;
use crate::number::Float;

/// Struct representing a node in a ball tree.
///
/// # Type Parameters
/// * `F` - The float type.
/// * `P` - The point type.
#[derive(Debug, PartialEq)]
pub struct Node<F: Float, P: Point<F>> {
    point: P,
    radius: F,
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
    /// * `point` - The point of this node.
    /// * `radius` - The radius of this node.
    /// * `left` - The left child node.
    /// * `right` - The right child node.
    ///
    /// # Returns
    /// A new `Node` instance.
    #[must_use]
    pub fn new(point: P, radius: F, left: Option<Node<F, P>>, right: Option<Node<F, P>>) -> Self {
        Self {
            point,
            radius,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::point::Point3;
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::new(Point3(1.0, 2.0, 3.0), 1.0, None, None);
        assert_eq!(node.point, Point3(1.0, 2.0, 3.0));
        assert_eq!(node.radius, 1.0);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
    }
}
