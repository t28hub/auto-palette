use super::cluster::Cluster;
use crate::next::math::Vector;

/// A node of the merge hierarchy, with what the linkage measures.
#[derive(Debug)]
pub(super) struct Node {
    /// The indices of the points.
    members: Vec<usize>,
    /// The coordinates of each distinct color of the points.
    colors: Vec<Vector<3>>,
    /// The sum of the coordinates of the points, each multiplied by its weight.
    sum: Vector<3>,
    /// The sum of the weights of the points.
    weight: f64,
}

impl Node {
    /// Creates the node of one point.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `coordinates` - The coordinates of the color of the point.
    /// * `weight` - The weight of the point, positive.
    ///
    /// # Returns
    /// The node of the point alone.
    pub(super) fn of_point(index: usize, coordinates: Vector<3>, weight: f64) -> Self {
        Self {
            members: vec![index],
            colors: vec![coordinates],
            sum: coordinates * weight,
            weight,
        }
    }

    /// Adds a point of the same color as the first point.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `weight` - The weight of the point, positive.
    pub(super) fn add_point(&mut self, index: usize, weight: f64) {
        self.members.push(index);
        self.sum += self.colors[0] * weight;
        self.weight += weight;
    }

    /// Absorbs the points of another node.
    ///
    /// # Arguments
    /// * `other` - The node to absorb.
    pub(super) fn absorb(&mut self, other: Self) {
        self.members.extend(other.members);
        self.colors.extend(other.colors);
        self.sum += other.sum;
        self.weight += other.weight;
    }

    /// Returns the coordinates of each distinct color of the points.
    ///
    /// # Returns
    /// The coordinates of the colors, the color of the first point first.
    #[inline]
    pub(super) fn colors(&self) -> &[Vector<3>] {
        &self.colors
    }

    /// Returns the weighted mean of the coordinates of the points.
    ///
    /// # Returns
    /// The mean coordinates, weighted by the weights of the points.
    #[inline]
    pub(super) fn mean(&self) -> Vector<3> {
        self.sum / self.weight
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

    #[test]
    fn test_add_point_adds_weight_at_first_color() {
        // Arrange
        let mut node = Node::of_point(0, Vector([0.2, 0.0, 0.0]), 1.0);
        node.absorb(Node::of_point(1, Vector([0.4, 0.0, 0.0]), 1.0));

        // Act
        node.add_point(2, 2.0);

        // Assert
        assert_approx_eq!(node.mean().0[0], 0.25);
    }

    #[test]
    fn test_mean_weighs_coordinates() {
        // Arrange
        let mut node = Node::of_point(0, Vector([0.2, 0.0, 0.0]), 0.25);
        node.absorb(Node::of_point(1, Vector([0.6, 0.4, 0.0]), 0.75));

        // Act
        let actual = node.mean();

        // Assert
        assert_approx_eq!(actual.0[0], 0.5);
        assert_approx_eq!(actual.0[1], 0.3);
    }

    #[test]
    fn test_add_point_keeps_colors_distinct() {
        // Arrange
        let mut node = Node::of_point(0, Vector([0.2, 0.0, 0.0]), 1.0);

        // Act
        node.add_point(1, 1.0);

        // Assert
        assert_eq!(node.colors(), [Vector([0.2, 0.0, 0.0])]);
    }
}
