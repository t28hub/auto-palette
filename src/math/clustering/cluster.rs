use crate::math::point::Point;
use std::collections::HashSet;

/// Cluster represents a cluster of points.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
#[derive(Debug, Clone)]
pub struct Cluster<const N: usize> {
    members: HashSet<usize>,
    centroid: Point<N>,
}

impl<const N: usize> Cluster<N> {
    /// Creates a new `Cluster` instance.
    ///
    /// # Returns
    /// A new `Cluster` instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            members: HashSet::new(),
            centroid: [0.0; N],
        }
    }

    /// Returns the number of points in this cluster.
    ///
    /// # Returns
    /// The number of points in this cluster.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns whether this cluster is empty.
    ///
    /// # Returns
    /// `true` if this cluster is empty; `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Returns an iterator over the members of this cluster.
    ///
    /// # Returns
    /// An iterator over the members of this cluster.
    pub fn members(&self) -> impl Iterator<Item = &usize> {
        self.members.iter()
    }

    /// Returns the centroid of this cluster.
    ///
    /// # Returns
    /// The centroid of this cluster.
    #[must_use]
    pub fn centroid(&self) -> &Point<N> {
        &self.centroid
    }

    /// Adds a member point to this cluster.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `point` - The point to add.
    ///
    /// # Returns
    /// `true` if the point is added; `false` otherwise.
    pub fn add_member(&mut self, index: usize, point: &Point<N>) -> bool {
        if !self.members.insert(index) {
            return false;
        }

        let size = self.members.len() as f32;
        for (i, value) in point.iter().enumerate() {
            self.centroid[i] *= (size - 1.0) / size;
            self.centroid[i] += value / size;
        }
        true
    }

    /// Clears this cluster and resets the centroid.
    pub fn clear(&mut self) {
        self.centroid = [0.0; N];
        self.members.clear();
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
        assert!(cluster.is_empty());
        assert_eq!(cluster.len(), 0);
        assert_eq!(cluster.members().copied().collect::<Vec<_>>(), vec![]);
        assert_eq!(cluster.centroid(), &[0.0, 0.0]);
    }

    #[test]
    fn test_add_member() {
        // Arrange
        let mut cluster = Cluster::<2>::new();

        // Act & Assert
        let point = [1.0, 2.0];
        assert!(cluster.add_member(0, &point));
        assert!(!cluster.is_empty());
        assert_eq!(cluster.len(), 1);
        assert_eq!(
            cluster.members().copied().collect::<HashSet<_>>(),
            HashSet::from([0])
        );
        assert_eq!(cluster.centroid(), &[1.0, 2.0]);

        let point = [2.0, 4.0];
        assert!(cluster.add_member(1, &point));
        assert_eq!(cluster.len(), 2);
        assert_eq!(
            cluster.members().copied().collect::<HashSet<_>>(),
            HashSet::from([0, 1])
        );
        assert_eq!(cluster.centroid(), &[1.5, 3.0]);

        let point = [3.0, 6.0];
        assert!(cluster.add_member(2, &point));
        assert_eq!(cluster.len(), 3);
        assert_eq!(
            cluster.members().copied().collect::<HashSet<_>>(),
            HashSet::from([0, 1, 2])
        );
        assert_eq!(cluster.centroid(), &[2.0, 4.0]);

        assert!(!cluster.add_member(2, &point));
        assert_eq!(cluster.len(), 3);
        assert_eq!(
            cluster.members().copied().collect::<HashSet<_>>(),
            HashSet::from([0, 1, 2])
        );
        assert_eq!(cluster.centroid(), &[2.0, 4.0]);
    }

    #[test]
    fn test_clear() {
        // Arrange
        let mut cluster = Cluster::<2>::new();
        cluster.add_member(0, &[1.0, 2.0]);
        cluster.add_member(1, &[2.0, 4.0]);
        cluster.add_member(2, &[3.0, 6.0]);

        assert!(!cluster.is_empty());
        assert_eq!(cluster.len(), 3);
        assert_eq!(
            cluster.members().copied().collect::<HashSet<_>>(),
            HashSet::from([0, 1, 2])
        );
        assert_eq!(cluster.centroid(), &[2.0, 4.0]);

        // Act
        cluster.clear();

        // Assert
        assert!(cluster.is_empty());
        assert_eq!(cluster.len(), 0);
        assert_eq!(
            cluster.members().copied().collect::<HashSet<_>>(),
            HashSet::new()
        );
        assert_eq!(cluster.centroid(), &[0.0, 0.0]);
    }
}
