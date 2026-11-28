use super::cluster::Cluster;
use crate::next::{
    extraction::{centroid::Centroid, point::Point},
    math::Vector,
};

/// A node of the merge hierarchy, with what the linkage measures.
#[derive(Debug)]
pub(super) struct Node<const N: usize> {
    /// The indices of the points.
    members: Vec<usize>,
    /// Each distinct vector of the points.
    vectors: Vec<Vector<N>>,
    /// The weighted mean of the points.
    centroid: Centroid<N>,
}

impl<const N: usize> Node<N> {
    /// Creates the node of one point.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `point` - The point, with weight.
    ///
    /// # Returns
    /// The node of the point alone.
    pub(super) fn of_point(index: usize, point: Point<N>) -> Self {
        let mut centroid = Centroid::default();
        centroid.add(point);
        Self {
            members: vec![index],
            vectors: vec![point.vector()],
            centroid,
        }
    }

    /// Adds a point at the same vector as the first point.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `point` - The point, with weight.
    pub(super) fn add_point(&mut self, index: usize, point: Point<N>) {
        self.members.push(index);
        self.centroid.add(point);
    }

    /// Absorbs the points of another node.
    ///
    /// # Arguments
    /// * `other` - The node to absorb.
    pub(super) fn absorb(&mut self, other: Self) {
        self.members.extend(other.members);
        self.vectors.extend(other.vectors);
        self.centroid.merge(other.centroid);
    }

    /// Returns each distinct vector of the points.
    ///
    /// # Returns
    /// The vectors, the vector of the first point first.
    #[inline]
    pub(super) fn vectors(&self) -> &[Vector<N>] {
        &self.vectors
    }

    /// Returns the weighted mean of the points.
    ///
    /// # Returns
    /// The mean vector, weighted by the weights of the points.
    #[inline]
    pub(super) fn mean(&self) -> Vector<N> {
        let mean = self.centroid.point().expect("a node has weight");
        mean.vector()
    }

    /// Converts the node into a cluster, sorting its points.
    ///
    /// # Returns
    /// The cluster of the same points.
    pub(super) fn into_cluster(mut self) -> Cluster {
        self.members.sort_unstable();
        Cluster::new(self.members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    /// Creates a point on the first axis.
    fn point(x: f64, weight: f64) -> Point<3> {
        Point::of_vector(Vector([x, 0.0, 0.0]), weight)
    }

    #[test]
    fn test_add_point_adds_weight() {
        // Arrange
        let mut node = Node::of_point(0, point(0.2, 1.0));
        node.absorb(Node::of_point(1, point(0.4, 1.0)));

        // Act
        node.add_point(2, point(0.2, 2.0));

        // Assert
        let [x, ..] = node.mean().0;
        assert_approx_eq!(x, 0.25);
    }

    #[test]
    fn test_add_point_keeps_vectors_distinct() {
        // Arrange
        let mut node = Node::of_point(0, point(0.2, 1.0));

        // Act
        node.add_point(1, point(0.2, 1.0));

        // Assert
        assert_eq!(node.vectors(), [Vector([0.2, 0.0, 0.0])]);
    }

    #[test]
    fn test_mean_weighs_points() {
        // Arrange
        let mut node = Node::of_point(0, point(0.2, 0.25));
        node.absorb(Node::of_point(
            1,
            Point::of_vector(Vector([0.6, 0.4, 0.0]), 0.75),
        ));

        // Act
        let actual = node.mean();

        // Assert
        let [x, y, _] = actual.0;
        assert_approx_eq!(x, 0.5);
        assert_approx_eq!(y, 0.3);
    }
}
