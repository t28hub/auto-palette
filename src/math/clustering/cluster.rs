use crate::math::point::Point;
use std::collections::HashSet;

/// Cluster represents a cluster of points.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
#[derive(Debug, Clone)]
pub struct Cluster<const N: usize> {
    centroid: Point<N>,
    membership: HashSet<usize>,
}

impl<const N: usize> Cluster<N> {
    /// Creates a new `Cluster` instance.
    ///
    /// # Returns
    /// A new `Cluster` instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            centroid: [0.0; N],
            membership: HashSet::new(),
        }
    }

    /// Returns the centroid of this cluster.
    ///
    /// # Returns
    /// The centroid of this cluster.
    #[must_use]
    pub fn centroid(&self) -> &Point<N> {
        &self.centroid
    }

    /// Returns the number of points in this cluster.
    ///
    /// # Returns
    /// The number of points in this cluster.
    #[must_use]
    pub fn len(&self) -> usize {
        self.membership.len()
    }

    /// Adds a point to this cluster and updates the centroid.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `point` - The point to add.
    ///
    /// # Returns
    /// `true` if the point is added; `false` otherwise.
    pub fn add_point(&mut self, index: usize, point: &Point<N>) -> bool {
        if !self.membership.insert(index) {
            return false;
        }

        let size = self.membership.len() as f32;
        for (i, value) in point.iter().enumerate() {
            self.centroid[i] *= size - 1.0;
            self.centroid[i] += value;
            self.centroid[i] /= size;
        }
        true
    }

    /// Clears this cluster and resets the centroid.
    pub fn clear(&mut self) {
        self.centroid = [0.0; N];
        self.membership.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cluster() {
        // Act
        let cluster = Cluster::<2>::new();

        // Assert
        assert_eq!(cluster.centroid(), &[0.0, 0.0]);
        assert_eq!(cluster.len(), 0);
    }

    #[test]
    fn test_add_point() {
        // Arrange
        let mut cluster = Cluster::<2>::new();

        // Act & Assert
        let point = [1.0, 2.0];
        assert!(cluster.add_point(0, &point));
        assert_eq!(cluster.centroid(), &[1.0, 2.0]);
        assert_eq!(cluster.len(), 1);

        let point = [2.0, 4.0];
        assert!(cluster.add_point(1, &point));
        assert_eq!(cluster.centroid(), &[1.5, 3.0]);
        assert_eq!(cluster.len(), 2);

        let point = [3.0, 6.0];
        assert!(cluster.add_point(2, &point));
        assert_eq!(cluster.centroid(), &[2.0, 4.0]);
        assert_eq!(cluster.len(), 3);

        assert!(!cluster.add_point(2, &point));
        assert_eq!(cluster.centroid(), &[2.0, 4.0]);
        assert_eq!(cluster.len(), 3);
    }

    #[test]
    fn test_clear() {
        // Arrange
        let mut cluster = Cluster::<2>::new();
        let point = [1.0, 2.0];
        assert!(cluster.add_point(0, &point));
        assert_eq!(cluster.centroid(), &[1.0, 2.0]);
        assert_eq!(cluster.len(), 1);

        // Act
        cluster.clear();

        // Assert
        assert_eq!(cluster.centroid(), &[0.0, 0.0]);
        assert_eq!(cluster.len(), 0);
    }
}
